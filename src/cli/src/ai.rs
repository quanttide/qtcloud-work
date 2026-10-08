//! 适配：交给智能体的那一趟——造话术、非交互调 `pi`、读回结论。
//!
//! 话术本身在 [`crate::prompts`]，只收纯数据；把工单现场凑成那份数据、起进程、
//! 认结论都在这里。聚合不碰这一层：走一步的动作在这儿组装，工单聚合只提供判据与记账。

use crate::criterion::Criterion;
use crate::order::execute::{AiRun, expanded_criteria, walk};
use crate::order::{Order, WorkRecord, open};
use crate::outcome::Outcome;
use crate::prompts::Facts;
use crate::workflow::Step;
use crate::workspace::LocalWorkspace;
use std::path::Path;
use std::process::Command;

/// 走下一步：能让 AI 跑的交给 AI，然后跑判据、记一笔。
pub fn order_next(locate: &LocalWorkspace, name: &str, note: &str) -> Outcome {
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
    let (ran, out) = run_ai(&prompt_for(&order, &step), &order.locate.root);
    let one = if out.is_empty() {
        "（没输出）".to_string()
    } else {
        one_line(&out, 80)
    };
    let agents = step.agents();
    let judged = if ran && !agents.is_empty() {
        judge_by_ai(&order, &step, &expanded_criteria(&order, &agents))
    } else {
        Vec::new()
    };
    let (ok, lines, rows, recorded) = walk(&mut order, &step, note, AiRun { ran, one, judged });
    if let Some(record) = &recorded
        && let Err(error) = crate::order::events::recorded(locate, &order.payload, record)
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

/// 交给 AI 的那一段话：说什么、不说什么是定死的，话本身在 `crate::prompts`。
fn prompt_for(order: &Order, step: &Step) -> String {
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
        report: crate::workspace::short(
            &locate.root,
            &locate.artifact_path(crate::order::journal::REPORT, &order.payload.name),
        ),
        journal: crate::workspace::short(
            &locate.root,
            &locate.artifact_path(crate::order::journal::JOURNAL, &order.payload.name),
        ),
        records: previous_records(&order.payload.records, 8),
    }
}

/// 前几笔流水：执行者与复查者得看得见前面发生了什么。
fn previous_records(records: &[WorkRecord], limit: usize) -> String {
    let recent: Vec<&WorkRecord> = records.iter().rev().take(limit).collect();
    if recent.is_empty() {
        return "（还没有流水）".to_string();
    }
    recent
        .iter()
        .rev()
        .map(|record| {
            format!(
                "- 第 {} 笔（{}）{}：{}——{}",
                record.seq,
                record.created_at,
                record.step,
                if record.is_succeeded { "过" } else { "没过" },
                record.description
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// 把这一步交给 AI 跑：非交互调 `pi`。
fn run_ai(prompt: &str, root: &Path) -> (bool, String) {
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

fn one_line(text: &str, limit: usize) -> String {
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
fn judge_by_ai(
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
fn judge_prompt(order: &Order, step: &Step, criteria: &[Criterion]) -> String {
    crate::prompts::judge_prompt(&facts_of(order, step), &expanded_criteria(order, criteria))
}
