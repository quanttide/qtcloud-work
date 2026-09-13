//! 任务聚合 / 模型：工作流的一次执行实例。
//!
//! 指令是跑哪条工作流（`workflow_name`，**按名字**引用）与从哪开工（`start`）；
//! 状态是流水（只增不改）、闸门项、产物声明。任务是运行数据，不是产物——
//! 程序只维护它，不往产物里写字。任务不带位置：它在哪、产物落哪，由工作区算。
//!
//! 「走过哪几步」与落点在 [`crate::workspace`]；占位展开在中立的 `crate::paths`。
//! 平台的 `crate::task::Task` 另持位置并自己落盘，本模型只读内容。
//! 出处：`docs/specification/process/task.md`·语法。

use super::journal::JournalEvent;
use crate::fields::text_of;
use serde_yaml::Value as Yaml;
use std::collections::BTreeMap;

/// 任务聚合。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub name: String,
    pub workflow_name: String,
    pub start: String,
    pub journal: Vec<JournalEvent>,
    /// 等人拍板的事项。
    pub gates: Vec<String>,
    /// 这次执行往哪写产物（声明写成什么就是什么，相对平台给的目录基准）。
    pub artifacts: BTreeMap<String, String>,
}

impl Task {
    /// 从任务文件里的字段读出（不校验）；任务名取自 `name` 字段。
    pub fn of(payload: &Yaml) -> Task {
        let journal = payload
            .get("log")
            .and_then(|v| v.as_sequence())
            .map(|items| items.iter().map(JournalEvent::of).collect())
            .unwrap_or_default();
        let gates = payload
            .get("gates")
            .and_then(|v| v.as_sequence())
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| item.as_str().map(|text| text.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let artifacts = payload
            .get("artifacts")
            .and_then(|v| v.as_mapping())
            .map(|mapping| {
                mapping
                    .iter()
                    .filter_map(|(key, value)| {
                        Some((key.as_str()?.to_string(), value.as_str()?.to_string()))
                    })
                    .collect()
            })
            .unwrap_or_default();
        Task {
            name: text_of(payload, "name"),
            workflow_name: text_of(payload, "workflow"),
            start: text_of(payload, "start"),
            journal,
            gates,
            artifacts,
        }
    }

    /// 这种产物声明了往哪写；没声明给 `None`。
    pub fn declared(&self, kind: &str) -> Option<String> {
        let written = self.artifacts.get(kind)?.trim();
        if written.is_empty() {
            None
        } else {
            Some(written.to_string())
        }
    }
}
