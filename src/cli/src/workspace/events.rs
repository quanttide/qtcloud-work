//! 工作区聚合 / 事件：定义在此，落盘在 [`crate::events`]。
//!
//! 出处：`docs/specification/place/workspace.md`·领域事件。

use crate::locate::LocalWorkspace;

/// 工作区已创建。
pub const CREATED: &str = "WorkspaceCreated";

/// 工作区已创建：负载只带公共三样（`event` / `at` / `workspace_id`）——规范只要求
/// 至少携带工作区 `id`，且本事件先于其内一切事件（见规范·事件约束）。
pub fn created(locate: &LocalWorkspace) -> Result<(), String> {
    crate::events::append(locate, CREATED, Vec::new())
}
