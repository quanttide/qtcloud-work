//! 适配 / 执行器：把工单的一步交给智能体跑完、收回判定。
//!
//! 意图声明：
//!
//! - 核心目标：把一步交给智能体执行、收回判定，落成一次走一步的结果；
//! - 边界：不持有 `Order`（每次传入）、不直接改工单字段（只经聚合自己的方法记账）、
//!   不发领域事件（事件归聚合自己发）；
//! - 依赖：`crate::prompts`（话术）、外部命令 `pi`（AI 调用）、`crate::order::execute`（记账）；
//! - 不依赖：工单内部字段、`crate::order::events`。
//!
//! 给智能体的两段话在 `crate::prompts`，走一步的判据与记账在 `crate::order::execute`。

use crate::adapters::pi::{one_line, run_ai, verdict_of};
use crate::criterion::Criterion;
use crate::order::execute::{AiRun, Judging, expanded_criteria};
use crate::order::{Order, WorkRecord};
use crate::outcome::Outcome;
use crate::prompts::Facts;
use crate::workflow::Step;
use std::path::{Path, PathBuf};

/// 走一步的执行器：持工作区根（路径），不持工单。
pub struct AgentWorker {
    root: PathBuf,
}

/// 这一步的现场：话术要的数据 + 全部判据（占位已展开）。
struct Plan {
    facts: Facts,
    criteria: Vec<Criterion>,
}

/// 一次 AI 调用收回的东西：跑没跑成、原文、一句话。
struct Output {
    ran: bool,
    text: String,
    one: String,
}

impl AgentWorker {
    pub fn new(root: &Path) -> Self {
        AgentWorker {
            root: root.to_path_buf(),
        }
    }

    /// 走一步：组装现场、调 AI、判、记。开单与认下一步在调用方。
    pub fn run(&self, order: &mut Order, step: &Step, note: &str) -> Outcome {
        let plan = self.plan(order, step);
        let output = self.execute(&crate::prompts::prompt_for(&plan.facts, &plan.criteria));
        let verdicts = self.evaluate(&plan, &output);
        self.record(order, step, &output, &verdicts, note)
    }

    /// 组装现场：工单、步骤、产物路径、前几笔流水，外加这一步全部判据（占位已展开）。
    fn plan(&self, order: &Order, step: &Step) -> Plan {
        Plan {
            facts: facts_of(&self.root, order, step),
            criteria: expanded_criteria(order, &step.criteria()),
        }
    }

    /// 把话交给 `pi` 跑一趟。
    fn execute(&self, prompt: &str) -> Output {
        let (ran, text) = run_ai(prompt, &self.root);
        let one = if text.is_empty() {
            "（没输出）".to_string()
        } else {
            one_line(&text, 80)
        };
        Output { ran, text, one }
    }

    /// 让智能体按判准审一遍；返回（说明，结论，理由）。
    fn evaluate(&self, plan: &Plan, output: &Output) -> Vec<Judging> {
        let agents: Vec<Criterion> = plan
            .criteria
            .iter()
            .filter(|criterion| criterion.executor() == crate::executor::AGENT)
            .cloned()
            .collect();
        if !output.ran || agents.is_empty() {
            return Vec::new();
        }
        let verdicts = self.execute(&crate::prompts::judge_prompt(&plan.facts, &agents));
        let mut rows = Vec::new();
        for (index, criterion) in agents.iter().enumerate() {
            let note = criterion.text();
            if !verdicts.ran {
                rows.push((
                    note,
                    "待判".to_string(),
                    format!("智能体没跑成：{}", one_line(&verdicts.text, 80)),
                ));
                continue;
            }
            let (verdict, reason) = verdict_of(&verdicts.text, index + 1);
            rows.push((note, verdict, reason));
        }
        rows
    }

    /// 记一笔：核 rule 判据、记账，成一次走一步的结果。
    fn record(
        &self,
        order: &mut Order,
        step: &Step,
        output: &Output,
        judgments: &[Judging],
        note: &str,
    ) -> Outcome {
        let (ok, lines, rows) = order.record_step_result(
            step,
            note,
            AiRun {
                ran: output.ran,
                one: output.one.clone(),
                judged: judgments.to_vec(),
            },
        );
        let mut result = Outcome::new(ok);
        result.lines = lines;
        result.columns = vec!["核对".to_string(), "结论".to_string(), "说明".to_string()];
        result.rows = rows.into_iter().map(|(a, b, c)| vec![a, b, c]).collect();
        result.lines.push(order.state_line());
        result
    }
}

/// 这一步的现场：路径由命令行这边算好递进去。
fn facts_of(root: &Path, order: &Order, step: &Step) -> Facts {
    Facts {
        root: root.display().to_string(),
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
        report: crate::workspace::short(root, &order.artifact_path(crate::order::journal::REPORT)),
        journal: crate::workspace::short(
            root,
            &order.artifact_path(crate::order::journal::JOURNAL),
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
