//! 工单聚合 / 看与列：读全貌与列清单（只读动作，不写账）。

use super::{journal, listing, open};
use crate::locate::{Locate, short};
use crate::outcome::Outcome;
use crate::workspace::progress;
use serde_json::json;

/// 进度条：十格。
fn bar(done: usize, total: usize) -> String {
    let filled = done
        .checked_mul(10)
        .and_then(|value| value.checked_div(total))
        .unwrap_or(0);
    let body: String = "█".repeat(filled) + &"░".repeat(10 - filled);
    format!("[{body}] {done}/{total}")
}

/// 读全貌：封面加全量流水，进度照流水推导。
pub fn order_show(locate: &Locate, name: &str) -> Outcome {
    let order = match open(locate, name) {
        Ok(order) => order,
        Err(error) => return Outcome::lines(false, vec![error]),
    };
    let done = order.done_steps();
    let mut result = Outcome::new(true);
    result.columns = vec!["步骤".to_string(), "状态".to_string()];
    result.lines.push(format!(
        "工单：{}（{}）",
        order.payload.name,
        short(&locate.root, &order.file())
    ));
    result.lines.push(format!("  id：{}", order.payload.id));
    result.lines.push(format!(
        "  工作流：{}（{}）",
        order.workflow.name, order.payload.workflow_id
    ));
    result
        .lines
        .push(format!("  开工：{}", order.payload.created_at));
    if !order.payload.description.is_empty() {
        result
            .lines
            .push(format!("  描述：{}", order.payload.description));
    }
    for step in &order.workflow.steps {
        let state = if done.contains(&step.name) {
            "✓"
        } else {
            "—"
        };
        result.rows.push(vec![step.name.clone(), state.to_string()]);
        result.lines.push(format!("  {state} {}", step.name));
    }
    result.lines.push(order.state_line());
    for note in progress::pending_gates(&order.payload, &order.workflow) {
        result.lines.push(format!("  闸门：{note}"));
    }
    for record in &order.payload.records {
        result.lines.push(format!(
            "  {:>3}  {}  {} {}　{}",
            record.seq,
            record.created_at,
            if record.is_succeeded { "✓" } else { "✗" },
            record.step,
            record.description
        ));
    }
    result.lines.push(format!(
        "产物：{}",
        short(&locate.root, &locate.artifact_path(journal::REPORT, name))
    ));
    // 给窗口的那一栏：工单原文 + 产物落点 + 所引工作流（名字与凭证分开）。
    result.data = Some(json!({
        "payload": crate::events::yaml_to_json(&order.payload.to_yaml()),
        "workflow": order.workflow.name,
        "workflow_id": order.payload.workflow_id,
        "artifacts": {
            journal::REPORT: short(&locate.root, &locate.artifact_path(journal::REPORT, name)),
            journal::JOURNAL: short(&locate.root, &locate.artifact_path(journal::JOURNAL, name)),
        },
    }));
    result
}

pub fn order_list(locate: &Locate, workflow: &str) -> Outcome {
    let found = match listing(locate, workflow) {
        Ok(found) => found,
        Err(error) => return Outcome::lines(false, vec![error]),
    };
    let mut result = Outcome::new(true);
    result.columns = vec![
        "工单".to_string(),
        "工作流".to_string(),
        "进度".to_string(),
        "下一步".to_string(),
    ];
    let mut payload: Vec<serde_json::Value> = Vec::new();
    for order in &found {
        let total = order.workflow.steps.len();
        let done = order.done_steps().len();
        let next = order
            .next_step()
            .map(|step| step.name.clone())
            .unwrap_or_else(|| "走完".to_string());
        result.rows.push(vec![
            order.payload.name.clone(),
            order.workflow.name.clone(),
            format!("{done}/{total}"),
            next.clone(),
        ]);
        result.lines.push(format!(
            "{:24} 工作流 {}　{}　下一步：{next}",
            order.payload.name,
            order.workflow.name,
            bar(done, total)
        ));
        payload.push(json!({
            "name": order.payload.name,
            "workflow": order.workflow.name,
            "workflow_id": order.payload.workflow_id,
            "progress": format!("{done}/{total}"),
            "finished": progress::finished(&order.payload, &order.workflow),
        }));
    }
    if found.is_empty() {
        result.lines =
            vec!["还没有工单：qtcloud-work order create <名字> --workflow <工作流>".to_string()];
    }
    result.data = Some(json!(payload));
    result
}
