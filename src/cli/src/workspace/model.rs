//! 工作区聚合 / 模型：身份六字段。
//!
//! 出处：`docs/specification/place/workspace.md`·领域属性——`id`（只读，UUID）、
//! `name`（必选）、`title`（必选）、`description`（推荐，默认为空）、`created_at` /
//! `updated_at`（只读）。
//!
//! 模型管「是什么」：新身份怎么造（[`Workspace::new`]）、盘上读回的怎么核
//! （[`Workspace::parse`]）。落在哪、怎么落是 [`crate::workspace::local`] 的事；名字与标题
//! 缺省取自工作区根的名字。

use serde_yaml::{Mapping, Value};

/// 工作区身份六字段：`id` / `created_at` / `updated_at` 只读，`name` / `title` 必选。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    /// 全球凭证：凭证派生认它。
    pub id: String,
    /// 工作区名，在其所属工作区内唯一。
    pub name: String,
    /// 标题，给人看的名字。
    pub title: String,
    /// 一句话说清这个工作区干什么，默认为空。
    pub description: String,
    /// 创建时刻，只读。
    pub created_at: String,
    /// 最近改动时刻，只读。
    pub updated_at: String,
}

impl Workspace {
    /// 新开一份身份：名字与标题取工作区根的名字，描述留空，时刻由本侧落定。
    pub fn new(root_name: &str) -> Workspace {
        let stamp = crate::clock::now();
        Workspace {
            id: crate::ids::new_id(),
            name: root_name.to_string(),
            title: root_name.to_string(),
            description: String::new(),
            created_at: stamp.clone(),
            updated_at: stamp,
        }
    }

    /// 从盘上读回：YAML 要合法，`id` / `name` / `title` 必选——身份坏了当场报，
    /// 不给凭证派生喂空串。
    pub fn parse(text: &str) -> Result<Workspace, String> {
        let payload: Value =
            serde_yaml::from_str(text).map_err(|e| format!("不是合法的 YAML：{e}"))?;
        let field = |key: &str| {
            payload
                .get(key)
                .and_then(Value::as_str)
                .unwrap_or_default()
                .trim()
                .to_string()
        };
        let missing: Vec<&str> = ["id", "name", "title"]
            .into_iter()
            .filter(|key| field(key).is_empty())
            .collect();
        if !missing.is_empty() {
            return Err(format!(
                "缺必选字段：{}——身份文件坏了，修好它，或删掉它重首跑",
                missing.join("、")
            ));
        }
        Ok(Workspace {
            id: field("id"),
            name: field("name"),
            title: field("title"),
            description: field("description"),
            created_at: field("created_at"),
            updated_at: field("updated_at"),
        })
    }

    /// 落盘形状：六字段按规范顺序排。
    pub fn to_mapping(&self) -> Mapping {
        let mut payload = Mapping::new();
        for (key, value) in [
            ("id", &self.id),
            ("name", &self.name),
            ("title", &self.title),
            ("description", &self.description),
            ("created_at", &self.created_at),
            ("updated_at", &self.updated_at),
        ] {
            payload.insert(Value::String(key.into()), Value::String(value.clone()));
        }
        payload
    }
}
