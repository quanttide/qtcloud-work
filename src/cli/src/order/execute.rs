//! 工单聚合 / 走一步：展开占位、核判据、记一笔工作记录。
//!
//! 工作记录的 `is_succeeded` 来源与判据的 `executor` 一一对应——`rule` 机械比对、
//! `agent` 智能体审查、`human` 闸门放行（出处：`docs/specification/process/work-record.md`）。
//! 闸门不落封面字段：带 `human` 判据的站，程序核完 rule 判据后等人放行（`order done`）。
//!
//! 记一笔与随之而来的 `WorkRecorded` 事件都在本件——事件归聚合自己发。
//! 跑判据真去碰文件系统、起进程在 [`super::rules`]；把一步交给 AI 在 [`crate::workers::agent`]。

use super::Order;
use super::rules::run;
use crate::criterion::{Criterion, items_of};
use crate::order::WorkRecord;
use crate::workflow::Step;

/// 一条核对：说明、结论、理由（给人看的表格行）。
pub type Judging = (String, String, String);

/// 交给 AI 那半的结果：跑了没有、一句话、智能体逐条审出的结论。
pub struct AiRun {
    pub ran: bool,
    pub one: String,
    pub judged: Vec<Judging>,
}

/// 机器路径（`order next`）：程序核 rule 判据，记一笔。
///
/// 返回（过没过，话，核对行，记下的那笔）。有闸门时这一笔不记——等人放行；
/// 智能体没跑成时也不记——修好再来。
pub fn walk(
    order: &mut Order,
    step: &Step,
    note: &str,
    ai: AiRun,
) -> (bool, Vec<String>, Vec<Judging>, Option<WorkRecord>) {
    let mut lines: Vec<String> = Vec::new();
    let rules: Vec<Criterion> = step.rules();
    let gates: Vec<Criterion> = step.gates();

    lines.push(format!("{}：交给 AI（{}）跑", step.name, step.executor));
    lines.push(format!(
        "  AI {}：{}",
        if ai.ran { "跑完了" } else { "跑不动" },
        ai.one
    ));
    if !ai.ran {
        lines.push("  （AI 没跑成，这一步不算过；修好再来）".to_string());
        return (false, lines, Vec::new(), None);
    }

    let rule_items = items_of(&expanded_criteria(order, &rules));
    let (results, _) = run(&order.locate.root, &rule_items);
    let judged: Vec<Judging> = ai.judged;
    let rules_pass = results.iter().all(|(_, passed, _)| *passed);
    let agents_pass = judged.iter().all(|(_, verdict, _)| verdict == "✓");
    let ok = rules_pass && agents_pass;
    let detail = if !note.trim().is_empty() {
        note.trim().to_string()
    } else if !results.is_empty() {
        results
            .iter()
            .map(|(item, _, _)| item.description.clone())
            .collect::<Vec<_>>()
            .join("；")
    } else {
        "做完".to_string()
    };

    lines.push(format!(
        "{} {}：{}",
        if ok { "✓" } else { "✗" },
        step.name,
        detail
    ));
    lines.extend(results.iter().map(|(item, passed, spec)| {
        format!(
            "  {} {}（{spec}）",
            if *passed { "✓" } else { "✗" },
            item.description
        )
    }));
    lines.extend(
        judged
            .iter()
            .map(|(note, verdict, reason)| format!("  {verdict} {note}（{reason}）")),
    );
    lines.extend(
        gates
            .iter()
            .map(|criterion| format!("  ⧗ {}（留给人）", criterion.text())),
    );

    let mut rows: Vec<Judging> = results
        .iter()
        .map(|(item, passed, spec)| {
            (
                item.description.clone(),
                if *passed { "✓" } else { "✗" }.to_string(),
                spec.clone(),
            )
        })
        .collect();
    rows.extend(judged.clone());
    rows.extend(gates.iter().map(|criterion| {
        (
            criterion.text(),
            "闸门".to_string(),
            "留给人拍板".to_string(),
        )
    }));

    // 有闸门：程序核过的这半算数，放行那半等人——不记这一笔。
    if ok && !gates.is_empty() {
        lines.push(format!(
            "  这一步有闸门，等人放行：qtcloud-work order done {} {}",
            order.payload.name, step.name
        ));
        return (ok, lines, rows, None);
    }
    let record = match order.append(&crate::ids::new_id(), &step.name, &detail, ok) {
        Ok(record) => record,
        Err(error) => {
            lines.push(format!("  （这笔记不下：{error}）"));
            return (false, lines, rows, None);
        }
    };
    if let Err(error) = super::events::recorded(&order.locate, &order.payload, &record) {
        lines.push(format!("  （这笔记不下：{error}）"));
        return (false, lines, rows, None);
    }
    (ok, lines, rows, Some(record))
}

/// 人的路径（`order done`）：闸门放行，或人自己做完记一笔；程序仍核 rule 判据。
pub fn record_by_human(
    order: &mut Order,
    step: &Step,
    note: &str,
) -> Result<(bool, Vec<Judging>, WorkRecord), String> {
    let rules: Vec<Criterion> = step.rules();
    let rule_items = items_of(&expanded_criteria(order, &rules));
    let (results, _) = run(&order.locate.root, &rule_items);
    let ok = results.iter().all(|(_, passed, _)| *passed);
    let detail = if !note.trim().is_empty() {
        note.trim().to_string()
    } else if !results.is_empty() {
        results
            .iter()
            .map(|(item, _, _)| item.description.clone())
            .collect::<Vec<_>>()
            .join("；")
    } else {
        "人记一笔".to_string()
    };
    let rows: Vec<Judging> = results
        .iter()
        .map(|(item, passed, spec)| {
            (
                item.description.clone(),
                if *passed { "✓" } else { "✗" }.to_string(),
                spec.clone(),
            )
        })
        .collect();
    let record = order.append(&crate::ids::new_id(), &step.name, &detail, ok)?;
    super::events::recorded(&order.locate, &order.payload, &record)?;
    Ok((ok, rows, record))
}

/// 判据里的落点引用先换成本单的真路径，再去跑。
pub fn expanded_criteria(order: &Order, criteria: &[Criterion]) -> Vec<Criterion> {
    criteria
        .iter()
        .map(|criterion| criterion.expanded(|name| place_of(order, name)))
        .collect()
}

/// 一个占位换成哪条路径：产物按「产物落点 + 工单名」算，在区内就按区内相对写。
fn place_of(order: &Order, name: &str) -> Option<String> {
    let locate = &order.locate;
    let path = match name {
        // 产物目录不是产物，另有落点
        "artifacts" => locate.artifacts.clone(),
        named if crate::paths::PLACEHOLDER_NAMES.contains(&named) => {
            locate.artifact_path(named, &order.payload.name)
        }
        _ => return None,
    };
    Some(crate::workspace::short(&locate.root, &path))
}
