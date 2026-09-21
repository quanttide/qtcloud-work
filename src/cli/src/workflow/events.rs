//! 工作流聚合 / 事件：定义在此，落盘在 [`crate::events`]。
//!
//! 出处：`docs/specification/process/workflow.md`·领域事件。

use crate::locate::Locate;
use crate::workflow::Workflow;
use serde_json::{Map as JsonMap, Value as Json};

/// 工作流已创建。
pub const CREATED: &str = "WorkflowCreated";

/// 工作流已创建：带声明全文与派生出的凭证。
pub fn created(locate: &Locate, workflow: &Workflow) -> Result<(), String> {
    crate::events::append(
        locate,
        CREATED,
        vec![
            ("workflow_id", Json::String(workflow.id.clone())),
            ("name", Json::String(workflow.name.clone())),
            ("workflow", to_json(workflow)),
        ],
    )
}

/// 定义装成 JSON：判据线上形状照定义里写法。
fn to_json(workflow: &Workflow) -> Json {
    let steps: Vec<Json> = workflow
        .steps
        .iter()
        .map(|step| {
            let criteria: Vec<Json> = step
                .criteria
                .iter()
                .map(|criterion| {
                    let mut item = JsonMap::new();
                    item.insert("executor".into(), Json::String(criterion.executor().into()));
                    item.insert("description".into(), Json::String(criterion.text()));
                    Json::Object(item)
                })
                .collect();
            let mut item = JsonMap::new();
            item.insert("id".into(), Json::String(step.id.clone()));
            item.insert("name".into(), Json::String(step.name.clone()));
            item.insert("description".into(), Json::String(step.description.clone()));
            item.insert("executor".into(), Json::String(step.executor.clone()));
            item.insert("criteria".into(), Json::Array(criteria));
            Json::Object(item)
        })
        .collect();
    let mut payload = JsonMap::new();
    payload.insert("id".into(), Json::String(workflow.id.clone()));
    payload.insert("name".into(), Json::String(workflow.name.clone()));
    payload.insert(
        "description".into(),
        Json::String(workflow.description.clone()),
    );
    payload.insert("steps".into(), Json::Array(steps));
    Json::Object(payload)
}
