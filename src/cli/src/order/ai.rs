//! 工单聚合 / 交给 AI 的两段话与智能体审查。

use super::Order;
use crate::criterion::Criterion;
use crate::prompts::Facts;
use crate::workflow::Step;
use std::path::Path;
use std::process::Command;

/// 交给 AI 的那一段话：说什么、不说什么是定死的，话本身在 `crate::prompts`。
pub fn prompt_for(order: &Order, step: &Step) -> String {
    crate::prompts::prompt_for(
        &facts_of(order, step),
        &expanded_criteria(order, &step.criteria()),
    )
}

/// 这一步的现场：路径由命令行这边算好递进去。
fn facts_of(order: &Order, step: &Step) -> Facts {
    let locate = &order.locate;
    Facts {
        root: locate.root.display().to_string(),
        name: order.payload.name.clone(),
        description: order.payload.description.clone(),
        workflow_name: order.workflow.name.clone(),
        steps: order
            .workflow
            .steps
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<_>>()
            .join("、"),
        step: step.name.clone(),
        what: step.description.clone(),
        report: relative_to_root(
            locate,
            &locate.artifact_path(super::journal::REPORT, &order.payload.name),
        ),
        journal: relative_to_root(
            locate,
            &locate.artifact_path(super::journal::JOURNAL, &order.payload.name),
        ),
        records: crate::prompts::previous_records(&order.payload.records, 8),
    }
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
    Some(relative_to_root(locate, &path))
}

fn relative_to_root(locate: &crate::workspace::Locate, path: &Path) -> String {
    match path.strip_prefix(&locate.root) {
        Ok(rest) => rest.to_string_lossy().to_string(),
        Err(_) => path.to_string_lossy().to_string(),
    }
}

/// 判据里的落点引用先换成本单的真路径，再去跑。
pub fn expanded_criteria(order: &Order, criteria: &[Criterion]) -> Vec<Criterion> {
    criteria
        .iter()
        .map(|criterion| criterion.expanded(|name| place_of(order, name)))
        .collect()
}

/// 把这一步交给 AI 跑：非交互调 `pi`。
pub fn run_ai(prompt: &str, root: &Path) -> (bool, String) {
    let done = Command::new("pi")
        .args(["-p", "--no-session", prompt])
        .current_dir(root)
        .output();
    match done {
        Err(e) => (false, format!("没找到 pi：{e}")),
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
            let text = if stdout.is_empty() { stderr } else { stdout };
            (out.status.success(), text)
        }
    }
}

pub(super) fn one_line(text: &str, limit: usize) -> String {
    text.lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
        .trim()
        .chars()
        .take(limit)
        .collect()
}

/// 从智能体的回答里读一条结论：先认「序号. …」那行，没有就整段兜底。
fn verdict_of(out: &str, index: usize) -> (String, String) {
    let prefix = format!("{index}.");
    for line in out.lines() {
        let stripped = line.trim();
        if let Some(rest) = stripped.strip_prefix(&prefix) {
            let tail = rest.trim();
            let verdict = if tail.starts_with("不通过") {
                "✗"
            } else if tail.starts_with("通过") {
                "✓"
            } else {
                "待判"
            };
            return (verdict.to_string(), tail.to_string());
        }
    }
    let flat = out.replace(' ', "");
    if flat.contains("不通过") {
        ("✗".to_string(), one_line(out, 80))
    } else if flat.contains("通过") {
        ("✓".to_string(), one_line(out, 80))
    } else {
        ("待判".to_string(), one_line(out, 80))
    }
}

/// 让智能体按判准审一遍；返回（说明，结论，理由）。
pub fn judge_by_ai(
    order: &Order,
    step: &Step,
    criteria: &[Criterion],
) -> Vec<(String, String, String)> {
    let (ran, out) = run_ai(&judge_prompt(order, step, criteria), &order.locate.root);
    let mut rows = Vec::new();
    for (index, criterion) in criteria.iter().enumerate() {
        let note = criterion.text();
        if !ran {
            rows.push((
                note,
                "待判".to_string(),
                format!("智能体没跑成：{}", one_line(&out, 80)),
            ));
            continue;
        }
        let (verdict, reason) = verdict_of(&out, index + 1);
        rows.push((note, verdict, reason));
    }
    rows
}

/// 交给智能体审的那一段话：产物 + 判准 + 流水，逐条回答。
pub fn judge_prompt(order: &Order, step: &Step, criteria: &[Criterion]) -> String {
    crate::prompts::judge_prompt(&facts_of(order, step), &expanded_criteria(order, criteria))
}
