//! 工单聚合 / 走一步：展开占位、跑判据、记一笔工作记录。
//!
//! 工作记录的 `is_succeeded` 来源与判据的 `executor` 一一对应——`rule` 机械比对、
//! `agent` 智能体审查、`human` 闸门放行（出处：`docs/specification/process/work-record.md`）。
//! 闸门不落封面字段：带 `human` 判据的站，程序核完 rule 判据后等人放行（`order done`）。

use super::{Order, ai};
use crate::criterion::Criterion;
use crate::order::WorkRecord;
use crate::workflow::Step;

/// 一条核对：说明、结论、理由（给人看的表格行）。
pub type Judging = (String, String, String);

/// 机器路径（`order next`）：智能体执行，程序核 rule 判据，记一笔。
///
/// 返回（过没过，话，核对行，记下的那笔）。有闸门时这一笔不记——等人放行；
/// 智能体没跑成时也不记——修好再来。
pub fn walk(
    order: &mut Order,
    step: &Step,
    note: &str,
) -> (bool, Vec<String>, Vec<Judging>, Option<WorkRecord>) {
    let mut lines: Vec<String> = Vec::new();
    let _criteria = step.criteria();
    let rules: Vec<Criterion> = step.rules();
    let agents: Vec<Criterion> = step.agents();
    let gates: Vec<Criterion> = step.gates();

    lines.push(format!("{}：交给 AI（{}）跑", step.name, step.executor));
    let (ran, out) = ai::run_ai(&ai::prompt_for(order, step), &order.locate.root);
    let one = if out.is_empty() {
        "（没输出）".to_string()
    } else {
        ai::one_line(&out, 80)
    };
    lines.push(format!(
        "  AI {}：{}",
        if ran { "跑完了" } else { "跑不动" },
        one
    ));
    if !ran {
        lines.push("  （AI 没跑成，这一步不算过；修好再来）".to_string());
        return (false, lines, Vec::new(), None);
    }

    let rule_items = crate::audit::items_of(&ai::expanded_criteria(order, &rules));
    let (results, _) = crate::audit::run(&order.locate.root, &rule_items);
    let judged: Vec<Judging> = if agents.is_empty() {
        Vec::new()
    } else {
        ai::judge_by_ai(order, step, &ai::expanded_criteria(order, &agents))
    };
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
    match order.append(&crate::ids::new_id(), &step.name, &detail, ok) {
        Ok(record) => (ok, lines, rows, Some(record)),
        Err(error) => {
            lines.push(format!("  （这笔记不下：{error}）"));
            (false, lines, rows, None)
        }
    }
}

/// 人的路径（`order done`）：闸门放行，或人自己做完记一笔；程序仍核 rule 判据。
pub fn record_by_human(
    order: &mut Order,
    step: &Step,
    note: &str,
) -> Result<(bool, Vec<Judging>, WorkRecord), String> {
    let rules: Vec<Criterion> = step.rules();
    let rule_items = crate::audit::items_of(&ai::expanded_criteria(order, &rules));
    let (results, _) = crate::audit::run(&order.locate.root, &rule_items);
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
    Ok((ok, rows, record))
}
