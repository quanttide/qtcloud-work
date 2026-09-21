//! 工作区聚合：一次工作的边界——落点、流水判定与定义核对。
//!
//! 领域那一侧只认内容（定义与工单），不认位置；位置由平台装载（[`crate::locate::Locate`]）。
//! 子件按事分：产物落点在 `place`、流水判定（进度与完结只推导）在 `progress`、
//! 定义核对在 `check`。出处：`docs/specification/place/workspace.md`。

pub mod check;
pub mod place;
pub mod progress;

pub use check::check;
