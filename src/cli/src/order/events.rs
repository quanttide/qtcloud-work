//! 工单聚合 / 事件：定义在此，落盘在 [`crate::events`]。
//!
//! 出处：`docs/specification/process/work-order.md`·领域事件、
//! `process/work-record.md`·领域事件。

use crate::locate::Locate;
use crate::order::model::{WorkOrder, WorkRecord};
use serde_json::Value as Json;

/// 工单已创建。
pub const CREATED: &str = "WorkOrderCreated";
/// 工作记录已追加。
pub const RECORDED: &str = "WorkRecorded";

/// 工单已创建：带封面全文。
pub fn created(locate: &Locate, order: &WorkOrder) -> Result<(), String> {
    crate::events::append(
        locate,
        CREATED,
        vec![
            ("order_id", Json::String(order.id.clone())),
            ("name", Json::String(order.name.clone())),
            ("workflow_id", Json::String(order.workflow_id.clone())),
            ("order", crate::events::yaml_to_json(&order.to_yaml())),
        ],
    )
}

/// 工作记录已追加：带记录全文。
pub fn recorded(locate: &Locate, order: &WorkOrder, record: &WorkRecord) -> Result<(), String> {
    crate::events::append(
        locate,
        RECORDED,
        vec![
            ("order_id", Json::String(order.id.clone())),
            ("order_name", Json::String(order.name.clone())),
            ("record_id", Json::String(record.id.clone())),
            ("seq", Json::Number((record.seq as u64).into())),
            ("step_id", Json::String(record.step_id.clone())),
            ("record", crate::events::yaml_to_json(&record.to_yaml())),
        ],
    )
}
