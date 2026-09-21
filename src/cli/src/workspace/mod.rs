//! 工作区聚合：一次工作的边界。
//!
//! 工作区只认内容（材料、产物、工作流、工单都归属于它），不认位置——位置由平台装载
//! （[`crate::locate::Locate`]）。出处：`docs/specification/place/workspace.md`。
