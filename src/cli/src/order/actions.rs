//! 工单聚合 / 动作：开、看、列、走下一步、人记一笔、日志、销白纸。

use super::{create, delete, execute, inspect::order_show, journal, open};
use crate::locate::{Locate, short};
use crate::outcome::Outcome;
use serde_json::json;

/// 走下一步：能让 AI 跑的交给 AI，然后跑判据、记一笔。
/// 开工单：写完发事件，再看一遍。
pub fn order_create(locate: &Locate, name: &str, workflow: &str, description: &str) -> Outcome {
    let flow = crate::workflow::open(locate, workflow.trim());
    if !flow.exists() {
        return Outcome::lines(
            false,
            vec![format!(
                "没有这条工作流：{}（qtcloud-work workflow list 看有哪些）",
                short(&locate.root, &flow.file())
            )],
        );
    }
    let order = match create(locate, name, workflow, description) {
        Ok(order) => order,
        Err(error) => return Outcome::lines(false, vec![error]),
    };
    if let Err(error) = crate::events::workorder_created(locate, &order.payload) {
        return Outcome::lines(false, vec![error]);
    }
    order_show(locate, &order.payload.name)
        .with_first(format!("开了工单：{}", short(&locate.root, &order.file())))
}

pub fn order_next(locate: &Locate, name: &str, note: &str) -> Outcome {
    let mut order = match open(locate, name) {
        Ok(order) => order,
        Err(error) => return Outcome::lines(false, vec![error]),
    };
    let Some(step) = order.next_step().cloned() else {
        return Outcome::lines(true, vec!["所有步骤都走过了".to_string()]);
    };
    // 人做的步骤程序不抢着做：轮到人，做完用 `order done` 记一笔。
    if step.executor == crate::executor::HUMAN {
        return Outcome::lines(
            true,
            vec![
                format!("{}：这一步轮到你（人做的不替你做）", step.name),
                format!("做完记一笔：qtcloud-work order done {name} {}", step.name),
            ],
        );
    }
    let (ok, lines, rows, recorded) = execute::walk(&mut order, &step, note);
    if let Some(record) = &recorded
        && let Err(error) = crate::events::work_recorded(locate, &order.payload, record)
    {
        return Outcome::lines(false, vec![error]);
    }
    let mut result = Outcome::new(ok);
    result.lines = lines;
    result.columns = vec!["核对".to_string(), "结论".to_string(), "说明".to_string()];
    result.rows = rows.into_iter().map(|(a, b, c)| vec![a, b, c]).collect();
    if let Ok(fresh) = open(locate, name) {
        result.lines.push(fresh.state_line());
    }
    result
}

/// 人记一笔：闸门放行，或人自己做完记一笔；程序仍核 rule 判据。
pub fn order_done(locate: &Locate, name: &str, step: &str, note: &str) -> Outcome {
    if step.trim().is_empty() {
        return Outcome::lines(
            false,
            vec![format!(
                "请给步骤名（qtcloud-work order show {name} 看有哪些）"
            )],
        );
    }
    let mut order = match open(locate, name) {
        Ok(order) => order,
        Err(error) => return Outcome::lines(false, vec![error]),
    };
    let Some(found) = order.workflow.step(step.trim()) else {
        return Outcome::lines(
            false,
            vec![format!("所引工作流里没有这一步：{}", step.trim())],
        );
    };
    let (ok, rows, recorded) = match execute::record_by_human(&mut order, &found, note) {
        Ok(result) => result,
        Err(error) => return Outcome::lines(false, vec![error]),
    };
    if let Err(error) = crate::events::work_recorded(locate, &order.payload, &recorded) {
        return Outcome::lines(false, vec![error]);
    }
    let mut result = Outcome::new(ok);
    result.lines = vec![format!(
        "{} {}：{}",
        if ok { "✓" } else { "✗" },
        found.name,
        recorded.description
    )];
    result.columns = vec!["核对".to_string(), "结论".to_string(), "说明".to_string()];
    result.rows = rows.into_iter().map(|(a, b, c)| vec![a, b, c]).collect();
    if let Ok(fresh) = open(locate, name) {
        result.lines.push(fresh.state_line());
    }
    result
}

/// 日志：叙事落产物，不记流水。
pub fn order_journal(locate: &Locate, name: &str, words: &str) -> Outcome {
    let _order = match open(locate, name) {
        Ok(order) => order,
        Err(error) => return Outcome::lines(false, vec![error]),
    };
    if words.trim().is_empty() {
        return Outcome::lines(
            false,
            vec![format!(
                "日志要人来写：{}",
                short(&locate.root, &locate.artifact_path(journal::JOURNAL, name))
            )],
        );
    }
    let path = journal::narrate(locate, name, words);
    let fresh = open(locate, name).ok();
    Outcome::lines(
        true,
        vec![
            format!("日志记下一段：{}", short(&locate.root, &path)),
            fresh.map(|order| order.state_line()).unwrap_or_default(),
        ],
    )
}

/// 删一张白纸：流水非空即拒——账本不销户。
pub fn order_delete(locate: &Locate, name: &str) -> Outcome {
    match delete(locate, name) {
        Ok(path) => {
            let mut result = Outcome::lines(
                true,
                vec![format!("删了白纸：{}", short(&locate.root, &path))],
            );
            result.data = Some(json!({"name": name}));
            result
        }
        Err(error) => Outcome::lines(false, vec![error]),
    }
}
