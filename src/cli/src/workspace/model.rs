//! 工作区聚合 / 模型：身份六字段。
//!
//! 出处：`docs/specification/place/workspace.md`·领域属性——`id`（只读，UUID）、
//! `name`（必选）、`title`（必选）、`description`（推荐，默认为空）、`created_at` /
//! `updated_at`（只读）。
//!
//! 模型造内容、装载写盘：这里只管把一份**新身份**拼出来（`id` 走 `ids`、时刻走 `clock`），
//! 落在哪、怎么落是 `crate::locate` 的事；名字与标题缺省取自工作区根的名字。

use serde_yaml::{Mapping, Value};

/// 新开一份身份：名字与标题取工作区根的名字，描述留空，时刻由本侧落定。
pub fn identity(root_name: &str) -> Mapping {
    let stamp = crate::clock::now();
    let mut payload = Mapping::new();
    for (key, value) in [
        ("id", Value::String(crate::ids::new_id())),
        ("name", Value::String(root_name.to_string())),
        ("title", Value::String(root_name.to_string())),
        ("description", Value::String(String::new())),
        ("created_at", Value::String(stamp.clone())),
        ("updated_at", Value::String(stamp)),
    ] {
        payload.insert(Value::String(key.into()), value);
    }
    payload
}
