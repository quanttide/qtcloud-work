//! 领域事件落盘：追加 JSONL，一条一行，只增不改。
//!
//! 事件名与负载形状**由各聚合自己定**——工作流在 `crate::workflow::events`、工单在
//! `crate::order::events`、工作区在 `crate::workspace::events`。这里只管三件事：补上
//! 三个公共字段（`event` / `at` / `workspace_id`）、拼成一行、追加到账本的事件文件。
//! 下游按 `id` 幂等去重。

use crate::locate::LocalWorkspace;
use serde_json::{Map as JsonMap, Value as Json};

/// 追加一条事件：公共三样在前，聚合给的字段随后——顺序即写出的顺序。
pub(crate) fn append(
    locate: &LocalWorkspace,
    event: &str,
    fields: Vec<(&str, Json)>,
) -> Result<(), String> {
    let mut payload = JsonMap::new();
    payload.insert("event".into(), Json::String(event.into()));
    payload.insert("at".into(), Json::String(crate::clock::now()));
    payload.insert("workspace_id".into(), Json::String(locate.workspace_id()?));
    for (key, value) in fields {
        payload.insert(key.to_string(), value);
    }

    let path = locate.events_file();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{} 建不了：{e}", parent.display()))?;
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
