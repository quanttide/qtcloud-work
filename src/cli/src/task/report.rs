//! 任务聚合 / 动作：起任务、看状态、列任务、走一步、记日志。

use super::{Task, execute, journal, progress, state};
use crate::outcome::Outcome;
use crate::workflow;
use crate::workspace::short;
use serde_json::json;
use std::path::Path;

pub fn task_new(
    root: &Path,
    data: &Path,
    name: &str,
    workflow_name: &str,
    workflows: Option<&Path>,
) -> Outcome {
    if name.trim().is_empty() {
        return Outcome::lines(false, vec!["请先给这件任务起个名字".to_string()]);
    }
    let flow = workflow::open_workflow(data, workflow_name, workflows);
    if !flow.exists() {
        return Outcome::lines(
            false,
            vec![format!(
                "没有这条工作流：{}（qtcloud-work workflow --list 看有哪些）",
                short(data, &flow.file())
            )],
        );
    }
    let existing = state::reopen(data, name.trim(), Some(root), workflows);
    if existing.exists() {
        return Outcome::lines(
            false,
            vec![format!(
                "已经有这件任务：{}（换个名字，不覆盖）",
                short(data, &existing.file())
            )],
        );
    }
    let task = state::create(root, data, name.trim(), workflow_name.trim(), workflows);
    task_status(Some(root), data, name.trim(), workflows)
        .with_first(format!("起了：{}", short(data, &task.file())))
}

pub fn task_status(
    root: Option<&Path>,
    data: &Path,
    name: &str,
    workflows: Option<&Path>,
) -> Outcome {
    if name.trim().is_empty() {
        return Outcome::lines(
            false,
            vec!["请先选一件任务（qtcloud-work task --list 看有哪些）".to_string()],
        );
    }
    let task = state::reopen(data, name, root, workflows);
    if !task.exists() {
        return Outcome::lines(
            false,
            vec![format!("没有这件任务：{}", short(data, &task.file()))],
        );
    }
    let done = state::done(&task);
    let mut result = Outcome::new(true);
    result.columns = vec!["步骤".to_string(), "状态".to_string()];
    result.lines.push(format!("任务：{}", task.name));
    result.lines.push(format!(
        "  开工：{}",
        if task.start().is_empty() {
            "（没记）".to_string()
        } else {
            task.start()
        }
    ));
    result.lines.push(format!(
        "  工作流：{}——{}",
        task.workflow_name(),
        task.workflow().description()
    ));
    result.lines.push(format!(
        "  进度：{}",
        progress::bar(done.len(), task.steps().len())
    ));
    for step in task.steps() {
        let state = if done.contains(&step.name()) {
            "✓"
        } else {
            "—"
        };
        result.rows.push(vec![step.name(), state.to_string()]);
        result.lines.push(format!("  {state} {}", step.name()));
    }
    result.lines.push(state::state_line(&task));
    result
        .lines
        .push(format!("指令：{}", short(data, &task.file())));
    result.lines.push(format!(
        "产物：{}、{}　流水：{}",
        short(data, &task.artifact(journal::REPORT)),
        short(data, &task.artifact(journal::JOURNAL)),
        short(data, &task.artifact(journal::LOG))
    ));
    // 给窗口的那一栏：任务原文（装成领域对象要用）+ 三样产物的落点。
    result.data = Some(json!({
        "payload": serde_json::to_value(task.payload()).unwrap_or(serde_json::Value::Null),
        "artifacts": {
            journal::REPORT: short(data, &task.artifact(journal::REPORT)),
            journal::JOURNAL: short(data, &task.artifact(journal::JOURNAL)),
            journal::LOG: short(data, &task.artifact(journal::LOG)),
        },
    }));
    let events = task.events();
    if !events.is_empty() {
        result.lines.push("流水（最近五条）：".to_string());
        for event in events
            .iter()
            .rev()
            .take(5)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            let at = event.get("at").and_then(|v| v.as_str()).unwrap_or("");
            let step = event.get("step").and_then(|v| v.as_str()).unwrap_or("");
            let detail = event.get("detail").and_then(|v| v.as_str()).unwrap_or("");
            result.lines.push(format!("  {at}　{step}　{detail}"));
        }
    }
    result
}

pub fn task_list(root: Option<&Path>, data: &Path, workflows: Option<&Path>) -> Outcome {
    let found = state::listing(root, data, workflows);
    let mut result = Outcome::new(true);
    result.columns = vec![
        "任务".to_string(),
        "工作流".to_string(),
        "下一步".to_string(),
    ];
    for task in &found {
        let next = state::next_step(task)
            .map(|s| s.name())
            .unwrap_or_else(|| "走完".to_string());
        let progress = progress::bar(state::done(task).len(), task.steps().len());
        result
            .rows
            .push(vec![task.name.clone(), task.workflow_name(), next.clone()]);
        result.lines.push(format!(
            "{:24} 工作流 {}　{progress}　下一步：{next}",
            task.name,
            task.workflow_name()
        ));
    }
    if found.is_empty() {
        result.lines =
            vec!["还没有任务：qtcloud-work task --new <名字> --workflow <工作流>".to_string()];
    }
    result
}

/// 走一步：能让 AI 跑的交给 AI（auto），然后跑判据、记账。
pub fn task_step(
    root: Option<&Path>,
    data: &Path,
    name: &str,
    step: &str,
    note: &str,
    auto: bool,
    workflows: Option<&Path>,
) -> Outcome {
    let task: Task = state::reopen(data, name, root, workflows);
    if !task.exists() {
        return Outcome::lines(
            false,
            vec![format!("没有这件任务：{}", short(data, &task.file()))],
        );
    }
    let mut chosen = step.trim().to_string();
    if chosen.is_empty() {
        if !auto {
            return Outcome::lines(
                false,
                vec!["请给步骤名（qtcloud-work task <名字> 看有哪些步骤）".to_string()],
            );
        }
        match state::next_step(&task) {
            None => return Outcome::lines(true, vec!["所有步骤都走过了".to_string()]),
            Some(next) => chosen = next.name(),
        }
    }
    let (ok, lines, rows) = execute::execute(&task, &task.root, &chosen, note, auto);
    let mut result = Outcome {
        ok,
        lines,
        ..Default::default()
    };
    result.columns = vec!["核对".to_string(), "结论".to_string(), "说明".to_string()];
    result.rows = rows.into_iter().map(|(a, b, c)| vec![a, b, c]).collect();
    result.lines.push(format!(
        "进度：{}",
        progress::bar(state::done(&task).len(), task.steps().len())
    ));
    result.lines.push(state::state_line(&task));
    result
}

pub fn task_journal(
    root: Option<&Path>,
    data: &Path,
    name: &str,
    words: &str,
    workflows: Option<&Path>,
) -> Outcome {
    let task = state::reopen(data, name, root, workflows);
    if !task.exists() {
        return Outcome::lines(
            false,
            vec![format!("没有这件任务：{}", short(data, &task.file()))],
        );
    }
    if words.trim().is_empty() {
        return Outcome::lines(
            false,
            vec![format!(
                "日志要人来写：{}",
                short(data, &task.artifact(journal::JOURNAL))
            )],
        );
    }
    journal::narrate(&task, words);
    Outcome::lines(
        true,
        vec![
            format!(
                "日志记下一段：{}",
                short(data, &task.artifact(journal::JOURNAL))
            ),
            state::state_line(&task),
        ],
    )
}
