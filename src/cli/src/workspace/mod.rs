//! 工作区聚合：一次工作的边界——身份、落点、流水判定与定义核对。
//!
//! 领域那一侧只认内容（定义与工单），不认位置；位置由平台装载（[`Locate`]）。
//! 子件按事分：身份与位置装载在 `locate`、产物落点在 `place`、
//! 流水判定（进度与完结只推导）在 `progress`、定义核对在 `check`。
//! 出处：`docs/specification/place/workspace.md`。

pub mod check;
pub mod locate;
pub mod place;
pub mod progress;

pub use check::check;
pub use locate::Locate;

use crate::sha1::sha1_hex;
use std::path::{Path, PathBuf};

/// 工作区根：装载顺序 命令行 > 环境变量 `QTCLOUD_WORK_ROOT` > 向上搜索——
/// 跨工作区干活不必每条命令带 `--root`，也不用站在根里。
pub fn root(cli_root: Option<&Path>) -> PathBuf {
    if let Some(root) = cli_root {
        return root.to_path_buf();
    }
    if let Some(root) = std::env::var_os("QTCLOUD_WORK_ROOT")
        .map(PathBuf::from)
        .filter(|root| !root.as_os_str().is_empty())
    {
        return root;
    }
    locate::repo_root()
}

/// 工作区键：由工作区根的路径派生——可读名加短码。账本是「这台机器上的这个工作区」的账。
pub fn workspace_key(root: &Path) -> String {
    let resolved = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    format!(
        "{}-{}",
        resolved
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default(),
        &sha1_hex(&resolved.to_string_lossy())[..8]
    )
}

/// 账本缺省位置：`$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/`。
pub fn account(root: &Path) -> PathBuf {
    let home = std::env::var("XDG_DATA_HOME")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
        });
    home.join("qtcloud-work")
        .join("workspaces")
        .join(workspace_key(root))
}

/// 路径怎么显示给人看：相对工作区根写短一点，不在根底下就原样。
pub fn short(root: &Path, path: &Path) -> String {
    let root = root.to_string_lossy();
    let path = path.to_string_lossy();
    let prefix = if root.ends_with('/') {
        root.to_string()
    } else {
        format!("{root}/")
    };
    match path.strip_prefix(&prefix) {
        Some(rest) => rest.to_string(),
        None => path.to_string(),
    }
}
