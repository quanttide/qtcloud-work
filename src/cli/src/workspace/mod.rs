//! 工作区聚合：一次工作的边界。
//!
//! 工作区只认内容（材料、产物、工作流、工单都归属于它），不认位置——位置由平台装载
//! （[`crate::workspace::LocalWorkspace`]）。出处：`docs/specification/place/workspace.md`。
//!
//! 子件：模型在 `model`（类型 [`Workspace`]），本聚合的领域事件在 `events`；
//! 本地工作区在 [`local`]——落点解析与账本生命周期，不属于聚合但只服务它。
//! 模型的规矩是造内容不落盘——落哪儿是装载的事。

pub mod events;
pub mod local;
pub mod model;

pub use local::{LocalWorkspace, short};
