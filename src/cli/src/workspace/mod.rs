//! 工作区：三处位置与落点。
//!
//! 「工作区根 / 数据仓」的默认值在这里定一处；结果本身在工具箱里，
//! 路径怎么显示给人看（相对工作区根写短一点）也归这里。

use std::path::{Path, PathBuf};

/// 工作区根：给了就用给的，没给就用当前目录。
pub fn root(cli_root: Option<&Path>) -> std::result::Result<PathBuf, String> {
    match cli_root {
        Some(root) => Ok(root.to_path_buf()),
        None => std::env::current_dir().map_err(|e| e.to_string()),
    }
}

/// 数据仓：任务与产物草稿落在这里。
///
/// 不给 `--data` 就用当前目录下的 `data/`——开发环境的默认位置，不进版本库；
/// 用哪个数据仓印到标准错误，免得结果落在哪里靠猜。
pub fn data_dir(cli_data: Option<&Path>) -> std::result::Result<PathBuf, String> {
    if let Some(data) = cli_data {
        return Ok(data.to_path_buf());
    }
    let data = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .join("data");
    eprintln!("用的是数据仓：{}（没给 --data）", data.display());
    Ok(data)
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
