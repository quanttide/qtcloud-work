//! 子命令分派：把 clap 解析出的参数翻成聚合与服务的调用。
//!
//! 入口层只管「定位 + 发射」，业务调用都落在这里；业务本身在各自的聚合与服务里。

mod command;
mod workspace;

pub(crate) use command::{TaskArgs, WorkflowArgs, task, workflow};
pub(crate) use workspace::{audit, catalog, health, help, material, search};
