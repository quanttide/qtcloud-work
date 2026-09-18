//! 工作区聚合 / 流水判定：走过哪几步、下一步是哪、完结没完结。
//!
//! 进度与完结只推导，不落字段：拿流水（`records`）的 `step` 名对着定义的
//! `steps` 逐站对账——每站有一条 `is_succeeded` 为真的记录即走过；带 `human`
//! 判据的闸门步骤，通过须出自人（闸门放行走 `order done`，程序不替人记）。
//! 出处：`docs/specification/process/work-order.md`·关联、`work-record.md`·关联。

use crate::order::model::WorkOrder;
use crate::workflow::Workflow;

/// 这件工单走过哪几步（按定义里的顺序）。
pub fn done_steps(order: &WorkOrder, workflow: &Workflow) -> Vec<String> {
    workflow
        .step_names()
        .into_iter()
        .filter(|name| done(order, name))
        .collect()
}

/// 第一个没走到的步骤；都走过了给 `None`。
pub fn next_step<'a>(
    order: &WorkOrder,
    workflow: &'a Workflow,
) -> Option<&'a crate::workflow::Step> {
    workflow.steps.iter().find(|step| !done(order, &step.name))
}

/// 走完没走完：每个步骤都有成功的记录——结论随时可重算，完成不是动作，是事实。
pub fn finished(order: &WorkOrder, workflow: &Workflow) -> bool {
    let names = workflow.step_names();
    !names.is_empty() && names.iter().all(|name| done(order, name))
}

/// 待拍板清单：带 `human` 判据的所在站，还没过的那一站。闸门不落字段，由定义加流水推导。
pub fn pending_gates(order: &WorkOrder, workflow: &Workflow) -> Vec<String> {
    let mut notes = Vec::new();
    for step in &workflow.steps {
        if done(order, &step.name) {
            continue;
        }
        for criterion in step.gates() {
            let text = criterion.description().trim();
            if !text.is_empty() {
                notes.push(format!("{}：{}", step.name, text));
            }
        }
    }
    notes
}

/// 状态行：进度与下一步。
pub fn state_line(order: &WorkOrder, workflow: &Workflow) -> String {
    if workflow.steps.is_empty() {
        return format!("这条工作流没有步骤：{}", workflow.name);
    }
    match next_step(order, workflow) {
        Some(step) => format!(
            "进度：{}/{}　下一步：{}",
            done_steps(order, workflow).len(),
            workflow.steps.len(),
            step.name
        ),
        None => format!("走完了：{} 个步骤都过了", workflow.steps.len()),
    }
}

/// 这一站有没有走过：有一条成功的记录即算；闸门站的放行只能出自人记的那笔。
fn done(order: &WorkOrder, step: &str) -> bool {
    order
        .records
        .iter()
        .any(|record| record.step == step && record.is_succeeded)
}
