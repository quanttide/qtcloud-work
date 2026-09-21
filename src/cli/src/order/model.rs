//! 工单与工作记录：一次行程的账本（领域模型——内容那一层）。
//!
//! 规格出处：`docs/specification/process/work-order.md` 与 `work-record.md`。
//! 工单是行程的封面——封皮上写着走哪条工作流，内页是流水；工作记录是账上一笔。
//! 位置不进模型：工单在哪、产物落哪，由平台装载（`crate::locate::Locate`）。

use serde_yaml::Value as Yaml;

/// 工单认得的字段。
pub const FIELDS: [&str; 6] = [
    "id",
    "name",
    "description",
    "workflow_id",
    "created_at",
    "records",
];

/// 工单：一次行程的封面（流水内嵌在 `records`，不另落盘）。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorkOrder {
    /// 凭证号，系统生成，落笔后不变。
    pub id: String,
    /// 工单名，工作区内唯一；人读身份。
    pub name: String,
    /// 一句话任务，给执行的人（含智能体）看。
    pub description: String,
    /// 所引工作流的全球凭证，账本方查填；落笔即封。
    pub workflow_id: String,
    /// 开单时刻，账本方生成。
    pub created_at: String,
    /// 流水：只增不改。
    pub records: Vec<WorkRecord>,
}

impl WorkOrder {
    /// 从工单文件里读出（不校验；校验在 `crate::order::record`）。
    pub fn of(payload: &Yaml) -> WorkOrder {
        WorkOrder {
            id: text_of(payload, "id"),
            name: text_of(payload, "name"),
            description: text_of(payload, "description"),
            workflow_id: text_of(payload, "workflow_id"),
            created_at: text_of(payload, "created_at"),
            records: payload
                .get("records")
                .and_then(|v| v.as_sequence())
                .map(|items| items.iter().map(WorkRecord::of).collect())
                .unwrap_or_default(),
        }
    }

    /// 装成落盘的 YAML。
    pub fn to_yaml(&self) -> Yaml {
        let mut mapping = serde_yaml::Mapping::new();
        let mut insert = |key: &str, value: Yaml| {
            mapping.insert(Yaml::String(key.into()), value);
        };
        insert("id", Yaml::String(self.id.clone()));
        insert("name", Yaml::String(self.name.clone()));
        insert("description", Yaml::String(self.description.clone()));
        insert("workflow_id", Yaml::String(self.workflow_id.clone()));
        insert("created_at", Yaml::String(self.created_at.clone()));
        insert(
            "records",
            Yaml::Sequence(self.records.iter().map(|r| r.to_yaml()).collect()),
        );
        Yaml::Mapping(mapping)
    }
}

/// 工作记录：流水里的一条——什么时候、哪一站、一句话、过没过。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorkRecord {
    /// 凭证号，追加方生成（幂等键），工单内唯一。
    pub id: String,
    /// 页码，账本方分配，自 1 起严格递增不跳号。
    pub seq: usize,
    /// 什么时候。取步骤发生的时刻，不取落笔时刻。
    pub created_at: String,
    /// 所属工单的 `id`。
    pub order_id: String,
    /// 这一站（按名）。
    pub step: String,
    /// 这一站的全球凭证，账本对账查填。
    pub step_id: String,
    /// 一句话证词，记这一步已发生之事。
    pub description: String,
    /// 过没过，缺省 `false`。
    pub is_succeeded: bool,
}

impl WorkRecord {
    pub fn of(value: &Yaml) -> WorkRecord {
        WorkRecord {
            id: text_of(value, "id"),
            seq: value
                .get("seq")
                .and_then(|v| v.as_u64())
                .map(|v| v as usize)
                .unwrap_or(0),
            created_at: text_of(value, "created_at"),
            order_id: text_of(value, "order_id"),
            step: text_of(value, "step"),
            step_id: text_of(value, "step_id"),
            description: text_of(value, "description"),
            is_succeeded: value
                .get("is_succeeded")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        }
    }

    pub fn to_yaml(&self) -> Yaml {
        let mut mapping = serde_yaml::Mapping::new();
        let mut insert = |key: &str, value: Yaml| {
            mapping.insert(Yaml::String(key.into()), value);
        };
        insert("id", Yaml::String(self.id.clone()));
        insert("seq", Yaml::Number((self.seq as u64).into()));
        insert("created_at", Yaml::String(self.created_at.clone()));
        insert("order_id", Yaml::String(self.order_id.clone()));
        insert("step", Yaml::String(self.step.clone()));
        insert("step_id", Yaml::String(self.step_id.clone()));
        insert("description", Yaml::String(self.description.clone()));
        insert("is_succeeded", Yaml::Bool(self.is_succeeded));
        Yaml::Mapping(mapping)
    }
}

fn text_of(value: &Yaml, key: &str) -> String {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string()
}
