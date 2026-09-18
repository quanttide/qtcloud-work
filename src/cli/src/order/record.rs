//! 工单聚合 / 流水纪律：只增不改。
//!
//! 出处：`docs/specification/process/work-record.md`·约束——凭证不重、页码自 1 起
//! 不跳号、时刻不倒流、两处指认（`order_id` / `step_id`）对得上；工单封面
//! `workflow_id` 落笔即封。校验只认写下的字段：字段表在 `model`，字段之外一律拒绝。

use super::model::{FIELDS, WorkOrder, WorkRecord};
use crate::ids;
use serde_yaml::Value as Yaml;

/// 从工单文件里读出并校验：字段之外一律拒绝，流水纪律逐条过。
pub fn validate_yaml(payload: &Yaml, where_: &str) -> Result<WorkOrder, String> {
    let mapping = payload
        .as_mapping()
        .ok_or_else(|| format!("{where_}的顶层不是映射"))?;
    let unknown: Vec<String> = mapping
        .keys()
        .filter_map(|key| key.as_str())
        .filter(|key| !FIELDS.contains(key))
        .map(|key| key.to_string())
        .collect();
    if !unknown.is_empty() {
        return Err(format!(
            "{where_}有不认识的字段：{}（只认 {}）",
            unknown.join("、"),
            FIELDS.join("、")
        ));
    }
    let order = WorkOrder::of(payload);
    validate(&order, where_)?;
    Ok(order)
}

/// 校验一本账：封面必填，流水纪律逐条过。
pub fn validate(order: &WorkOrder, where_: &str) -> Result<(), String> {
    if !ids::is_id(&order.id) {
        return Err(format!("{where_}少了 id，或 id 不是 UUID"));
    }
    if order.name.is_empty() {
        return Err(format!("{where_}少了 name"));
    }
    if !ids::is_id(&order.workflow_id) {
        return Err(format!(
            "{where_}少了 workflow_id，或 workflow_id 不是 UUID"
        ));
    }
    if order.created_at.is_empty() {
        return Err(format!("{where_}少了 created_at"));
    }
    validate_records(&order.records, &order.id, where_)?;
    Ok(())
}

/// 流水只增不改：凭证不重、页码自 1 起不跳号、时刻不倒流、两处指认对得上。
pub fn validate_records(
    records: &[WorkRecord],
    order_id: &str,
    where_: &str,
) -> Result<(), String> {
    let mut seen: Vec<String> = Vec::new();
    let mut last = String::new();
    for (index, record) in records.iter().enumerate() {
        let spot = format!("{where_}第 {} 笔", index + 1);
        if !ids::is_id(&record.id) {
            return Err(format!("{spot}少了 id，或 id 不是 UUID"));
        }
        if seen.contains(&record.id) {
            return Err(format!("{spot}的 id 撞号：{}（账被复制）", record.id));
        }
        seen.push(record.id.clone());
        if record.seq != index + 1 {
            return Err(format!(
                "{spot}的 seq 应当是从 1 起不跳号的页码，实得 {}",
                record.seq
            ));
        }
        if record.created_at.is_empty() {
            return Err(format!("{spot}少了 created_at"));
        }
        if !last.is_empty() && record.created_at < last {
            return Err(format!("{spot}的时刻早于上一笔（账本时间倒流）"));
        }
        last = record.created_at.clone();
        if record.order_id != order_id {
            return Err(format!("{spot}的 order_id 与工单对不上"));
        }
        if record.step.is_empty() {
            return Err(format!("{spot}少了 step（按名指认的一站）"));
        }
        if !ids::is_id(&record.step_id) {
            return Err(format!("{spot}少了 step_id，或 step_id 不是 UUID"));
        }
    }
    Ok(())
}

/// 追加前的单笔检查：id 是追加方给的幂等键，账上不能已有这一枚。
pub fn append_check(order: &WorkOrder, record_id: &str) -> Result<(), String> {
    if !ids::is_id(record_id) {
        return Err("记录的 id 得是 UUID（幂等键）".to_string());
    }
    if order.records.iter().any(|record| record.id == record_id) {
        return Err(format!("这条记录已经在账上了：{record_id}（视作重放）"));
    }
    Ok(())
}
