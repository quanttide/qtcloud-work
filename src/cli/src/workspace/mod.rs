//! 工作区聚合：一次工作的边界——落点与流水判定。
//!
//! 领域那一侧只认内容（定义与工单），不认位置；位置由平台装载（[`crate::locate::Locate`]）。
//! 子件按事分：产物落点在 `place`、流水判定（进度与完结只推导）在 `progress`。
//! 出处：`docs/specification/place/workspace.md`。

pub mod place;
pub mod progress;
