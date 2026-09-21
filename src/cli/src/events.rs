//! 领域事件：落 JSONL，一条一行，只增不改。
//!
//! 规格：工作流已创建（`WorkflowCreated`）、工单已创建（`WorkOrderCreated`）、
//! 工作记录已追加（`WorkRecorded`）。负载至少带工作区 `id`；工单事件带工单
//! `id` 与 `name`、`workflow_id` 与声明全文；记录事件带记录 `id` / `seq` /
//! `step_id` 与记录全文。下游按 `id` 幂等去重。事件文件落账本仓。

use crate::locate::Locate;
use crate::order::{WorkOrder, WorkRecord};
use crate::workflow::Workflow;
use serde_json::{Map as JsonMap, Value as Json};

pub const WORKFLOW_CREATED: &str = "WorkflowCreated";
pub const WORKORDER_CREATED: &str = "WorkOrderCreated";
pub const WORK_RECORDED: &str = "WorkRecorded";

/// 工作流已创建：带声明全文与派生出的凭证。
pub fn workflow_created(locate: &Locate, workflow: &Workflow) -> Result<(), String> {
    emit(
        locate,
        [
            ("event", Json::String(WORKFLOW_CREATED.into())),
            ("at", Json::String(crate::clock::now())),
            ("workspace_id", Json::String(locate.workspace_id()?)),
            ("workflow_id", Json::String(workflow.id.clone())),
            ("name", Json::String(workflow.name.clone())),
            ("workflow", workflow_to_json(workflow)),
        ],
    )
}

/// 工单已创建：带封面全文。
pub fn workorder_created(locate: &Locate, order: &WorkOrder) -> Result<(), String> {
    emit(
        locate,
        [
            ("event", Json::String(WORKORDER_CREATED.into())),
            ("at", Json::String(crate::clock::now())),
            ("workspace_id", Json::String(locate.workspace_id()?)),
            ("order_id", Json::String(order.id.clone())),
            ("name", Json::String(order.name.clone())),
            ("workflow_id", Json::String(order.workflow_id.clone())),
            ("order", crate::events::yaml_to_json(&order.to_yaml())),
        ],
    )
}

/// 工作记录已追加：带记录全文。
pub fn work_recorded(
    locate: &Locate,
    order: &WorkOrder,
    record: &WorkRecord,
) -> Result<(), String> {
    emit(
        locate,
        [
            ("event", Json::String(WORK_RECORDED.into())),
            ("at", Json::String(crate::clock::now())),
            ("workspace_id", Json::String(locate.workspace_id()?)),
            ("order_id", Json::String(order.id.clone())),
            ("order_name", Json::String(order.name.clone())),
            ("record_id", Json::String(record.id.clone())),
            ("seq", Json::Number((record.seq as u64).into())),
            ("step_id", Json::String(record.step_id.clone())),
            ("record", crate::events::yaml_to_json(&record.to_yaml())),
        ],
    )
}

fn emit<const N: usize>(locate: &Locate, fields: [(&str, Json); N]) -> Result<(), String> {
    let path = locate.events_file();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{} 建不了：{e}", parent.display()))?;
    }
    let mut payload = JsonMap::new();
    for (key, value) in fields {
        payload.insert(key.to_string(), value);
    }
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("{} 开不了：{e}", path.display()))?;
    writeln!(file, "{}", Json::Object(payload))
        .map_err(|e| format!("{} 写不了：{e}", path.display()))
}

/// 领域模型装成 JSON：判据线上形状照定义里写法。
pub fn workflow_to_json(workflow: &Workflow) -> Json {
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

/// YAML 值装成 JSON 值（事件负载里的原文）。
pub fn yaml_to_json(value: &serde_yaml::Value) -> Json {
    match value {
        serde_yaml::Value::Null => Json::Null,
        serde_yaml::Value::Bool(value) => Json::Bool(*value),
        serde_yaml::Value::Number(number) => {
            if let Some(value) = number.as_i64() {
                Json::Number(value.into())
            } else if let Some(value) = number.as_u64() {
                Json::Number(value.into())
            } else {
                serde_json::json!(number.as_f64().unwrap_or_default())
            }
        }
        serde_yaml::Value::String(value) => Json::String(value.clone()),
        serde_yaml::Value::Sequence(items) => Json::Array(items.iter().map(yaml_to_json).collect()),
        serde_yaml::Value::Mapping(mapping) => {
            let mut out = JsonMap::new();
            for (key, value) in mapping {
                let key = match key {
                    serde_yaml::Value::String(text) => text.clone(),
                    other => serde_yaml::to_string(other)
                        .unwrap_or_default()
                        .trim()
                        .to_string(),
                };
                out.insert(key, yaml_to_json(value));
            }
            Json::Object(out)
        }
        serde_yaml::Value::Tagged(tagged) => yaml_to_json(&tagged.value),
    }
}
