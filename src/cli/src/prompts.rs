//! 给智能体的两段话：走一步要它做什么、审一遍要它按什么判。
//!
//! 这两段话是产品的一部分——说什么、不说什么是定死的，只是命令行这一侧的说法。
//! 执行者与复查者都带上流水（前几笔，含人的放行与裁决）：放行只落在流水里，
//! 报告与定义都看不到——不带流水，前面已被裁决的内容会被重新跑一遍。

use crate::criterion::Criterion;

/// 这一步的判据清单：每条一行「谁判：说明」。
pub fn criteria_text(criteria: &[Criterion]) -> String {
    let lines: Vec<String> = criteria
        .iter()
        .map(|criterion| format!("- {}：{}", criterion.executor(), criterion.text()))
        .collect();
    if lines.is_empty() {
        "（这一步没有判据）".to_string()
    } else {
        lines.join("\n")
    }
}

/// 走一步那件事的现场：工单与这一步的已知事实（路径由命令行这边算好递进来）。
pub struct Facts {
    pub root: String,
    pub name: String,
    pub description: String,
    pub workflow_name: String,
    pub steps: String,
    pub step: String,
    pub what: String,
    pub report: String,
    pub journal: String,
    /// 前几笔流水（含人的放行与裁决）。
    pub records: String,
}

/// 交给 AI 的那一段话。
pub fn prompt_for(facts: &Facts, criteria: &[Criterion]) -> String {
    format!(
        "你在按一条工作流走一步。只做这一步，做完就停。\n\n\
         工作区：{root}\n\
         工单：{name}（{description}）\n\
         工作流：{flow}\n\
         步骤：{steps}\n\
         这一步：{step}\n\
         做什么：\n{what}\n\n\
         判据（程序随后自己核对，你不能改判据、也不许改判据文件）：\n{criteria}\n\n\
         流水（前几笔，含人的放行与裁决——照它办）：\n{records}\n\n\
         本单的两样产物：报告 {report}、日志 {journal}（工作流里用 {{{{report}}}} / {{{{journal}}}} 指它们；\n\
         程序不碰产物内容，谁写谁定）。工作内容写进报告，别动程序写的节。\n\
         规矩：只在工作区里动「做什么」点名的东西。最后用一句话说明你做了什么。\n",
        root = facts.root,
        name = facts.name,
        description = if facts.description.is_empty() {
            "（没写描述）".to_string()
        } else {
            facts.description.clone()
        },
        flow = facts.workflow_name,
        steps = facts.steps,
        step = facts.step,
        what = facts.what,
        criteria = criteria_text(criteria),
        records = facts.records,
        report = facts.report,
        journal = facts.journal,
    )
}

/// 交给智能体审的那一段话：产物 + 判准 + 流水，逐条回答。
pub fn judge_prompt(facts: &Facts, criteria: &[Criterion]) -> String {
    let listed = criteria
        .iter()
        .enumerate()
        .map(|(index, criterion)| format!("{}. {}", index + 1, criterion.text()))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "你是审查者，不是执行者。别改产物、别改判据文件。\n\n\
         工作区：{root}\n\
         要审的东西：这一步的产物与相关文件都在工作区内\n\
         这一步做什么：{what}\n\n\
         流水（前几笔，含人的放行与裁决——照它判）：\n{records}\n\n\
         判准（逐条判）：\n{listed}\n\n\
         对每条输出一行，格式只能是「序号. 通过 — 一句话理由」或「序号. 不通过 — 一句话理由」，最后不要写别的。\n",
        root = facts.root,
        what = facts.what,
        records = facts.records,
        listed = listed,
    )
}

/// 前几笔流水：执行者与复查者得看得见前面发生了什么。
pub fn previous_records(records: &[crate::order::WorkRecord], limit: usize) -> String {
    let recent: Vec<&crate::order::WorkRecord> = records.iter().rev().take(limit).collect();
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
