//! 任务聚合 / 交给 AI 的两段话与智能体审查。

use super::Task;
use crate::workflow::Step;
use quanttide_work::criterion::Criterion;
use std::path::Path;
use std::process::Command;

/// 交给 AI 的那一段话：说什么、不说什么是定死的，话本身在 `crate::prompts`。
pub fn prompt_for(task: &Task, step: &Step) -> String {
    crate::prompts::prompt_for(&facts_of(task, step), &step.criteria())
}

/// 这一步的现场：路径由命令行这边算好递进去。
fn facts_of(task: &Task, step: &Step) -> crate::prompts::Facts {
    crate::prompts::Facts {
        root: task.root.display().to_string(),
        data: task.data.display().to_string(),
        name: task.name.clone(),
        start: task.start(),
        workflow_name: task.workflow_name(),
        workflow_description: task.workflow().description(),
        steps: task
            .steps()
            .into_iter()
            .map(|s| s.name())
            .collect::<Vec<_>>()
            .join("、"),
        step: step.name(),
        what: step.description(),
        report: task.relative(&task.artifact(crate::task::journal::REPORT)),
        journal: task.relative(&task.artifact(crate::task::journal::JOURNAL)),
        log: task.relative(&task.artifact(crate::task::journal::LOG)),
        artifacts: task.artifacts_dir().display().to_string(),
    }
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

/// 交给智能体审的那一段话：产物 + 判准，逐条回答。
pub fn judge_prompt(task: &Task, step: &Step, criteria: &[Criterion]) -> String {
    crate::prompts::judge_prompt(&facts_of(task, step), criteria)
}

pub(super) fn one_line(text: &str, limit: usize) -> String {
    let line = text
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .unwrap_or("")
        .trim();
    line.chars().take(limit).collect()
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
    task: &Task,
    step: &Step,
    criteria: &[Criterion],
) -> Vec<(String, String, String)> {
    let (ran, out) = run_ai(&judge_prompt(task, step, criteria), &task.root);
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
    let all_pass = rows.iter().all(|(_, verdict, _)| verdict == "✓");
    let detail = format!(
        "AI 审查（同一模型）：{}",
        rows.iter()
            .map(|(note, verdict, _)| format!("{note}→{verdict}"))
            .collect::<Vec<_>>()
            .join("；")
    );
    task.record(&format!("{}·审", step.name()), &detail, all_pass);
    rows
}
