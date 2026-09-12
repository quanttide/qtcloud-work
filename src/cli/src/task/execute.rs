//! 任务聚合 / 走一步：展开占位、跑判据、记流水、写闸门。

use super::{Task, ai, journal};
use quanttide_work::criterion::Criterion;
use std::path::Path;

/// 把 `{{report}}` / `{{journal}}` / `{{log}}` / `{{artifacts}}` 换成本次任务的产物路径。
pub fn expand(task: &Task, value: &str) -> String {
    let mut out = String::new();
    let mut rest = value;
    while let Some(at) = rest.find("{{") {
        out.push_str(&rest[..at]);
        let after = &rest[at + 2..];
        if let Some(end) = after.find("}}") {
            let kind = &after[..end];
            let path = match kind {
                "report" => Some(task.artifact(journal::REPORT)),
                "journal" => Some(task.artifact(journal::JOURNAL)),
                "log" => Some(task.artifact(journal::LOG)),
                "artifacts" => Some(task.artifacts_dir()),
                _ => None,
            };
            match path {
                Some(path) => out.push_str(&relative_to_root(task, &path)),
                None => {
                    out.push_str("{{");
                    out.push_str(kind);
                    out.push_str("}}");
                }
            }
            rest = &after[end + 2..];
        } else {
            out.push_str("{{");
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// 判据按工作区根解析，占位也给工作区根视角的路径。
fn relative_to_root(task: &Task, path: &Path) -> String {
    match path.strip_prefix(&task.root) {
        Ok(rest) => rest.to_string_lossy().to_string(),
        Err(_) => path.to_string_lossy().to_string(),
    }
}

/// 判据里的占位先换成本次任务的真实路径，再去跑。
pub fn expanded_criteria(task: &Task, criteria: &[Criterion]) -> Vec<Criterion> {
    criteria
        .iter()
        .map(|criterion| criterion.expanded(|text| expand(task, text)))
        .collect()
}

/// 走一步：能让 AI 跑的交给 AI，然后跑判据、记账、写报告。
pub fn execute(
    task: &Task,
    root: &Path,
    step: &str,
    note: &str,
    auto: bool,
) -> (bool, Vec<String>, Vec<(String, String, String)>) {
    let found = match task.workflow().step(step) {
        Some(found) => found,
        None => {
            return (
                false,
                vec![format!("工作流里没有这一步：{step}")],
                Vec::new(),
            );
        }
    };
    let mut lines: Vec<String> = Vec::new();
    if auto && !found.human() {
        lines.push(format!(
            "{}：交给 AI（{}）跑",
            found.name(),
            found.executor()
        ));
        let (ran, out) = ai::run_ai(&ai::prompt_for(task, &found), root);
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
        task.record(&found.name(), &format!("AI 执行：{one}"), ran);
        if !ran {
            write_gates(task, &[]);
            lines.push("  （AI 没跑成，这一步不算过；修好再来）".to_string());
            return (false, lines, Vec::new());
        }
    } else if found.human() && auto {
        lines.push(format!(
            "{}：这一步的执行者是人（{}）——轮到你，做完用 qtcloud-work task <名字> --done {}",
            found.name(),
            found.executor(),
            found.name()
        ));
        return (true, lines, Vec::new());
    }

    let rule_items = crate::audit::items_of(&expanded_criteria(task, &found.rules()));
    let (results, _) = crate::audit::run(root, &rule_items);
    let agents = found.agents();
    let judged: Vec<(String, String, String)> = if auto && !agents.is_empty() {
        ai::judge_by_ai(task, &found, &expanded_criteria(task, &agents))
    } else {
        agents
            .iter()
            .map(|criterion| {
                (
                    criterion.text(),
                    "待判".to_string(),
                    "没跑智能体（人为地记一步）".to_string(),
                )
            })
            .collect()
    };
    let gates: Vec<String> = found
        .gates()
        .iter()
        .map(|criterion| criterion.text())
        .collect();
    let rules_pass = results.iter().all(|(_, passed, _)| *passed);
    // 待判（人为地记一步、没跑智能体）不挡这一步，原样进闸门项。
    let judged_pass = judged
        .iter()
        .all(|(_, verdict, _)| verdict == "✓" || verdict == "待判");
    let ok = rules_pass && judged_pass;
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
    if !(auto && !found.human()) {
        task.record(step, &detail, ok);
    } else if !found.rules().is_empty() {
        // 交给 AI 跑的步骤：机器判据这一半单独记一笔（审查那条只知道 agent 判据）。
        task.record(&format!("{}·判", found.name()), &detail, rules_pass);
    }
    let mut gate_lines = gates.clone();
    gate_lines.extend(
        judged
            .iter()
            .filter(|(_, verdict, _)| verdict != "✓")
            .map(|(note, _, _)| note.clone()),
    );
    write_gates(task, &gate_lines);
    lines.push(format!(
        "{} {}：{}",
        if ok { "✓" } else { "✗" },
        step,
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
    lines.extend(gates.iter().map(|note| format!("  ⧗ {note}（留给人）")));
    let mut rows: Vec<(String, String, String)> = results
        .iter()
        .map(|(item, passed, spec)| {
            (
                item.description.clone(),
                if *passed {
                    "✓".to_string()
                } else {
                    "✗".to_string()
                },
                spec.clone(),
            )
        })
        .collect();
    rows.extend(judged.clone());
    rows.extend(
        gates
            .into_iter()
            .map(|note| (note, "闸门".to_string(), "留给人拍板".to_string())),
    );
    (ok, lines, rows)
}

/// 闸门项是任务的状态，记进任务文件；产物一个字都不碰。
pub fn write_gates(task: &Task, gates: &[String]) {
    let mut notes = task.gates();
    for note in gates {
        if !notes.iter().any(|existing| existing == note) {
            notes.push(note.clone());
        }
    }
    task.set_gates(&notes);
}
