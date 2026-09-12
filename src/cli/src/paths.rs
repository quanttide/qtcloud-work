//! 路径怎么显示给人看：相对工作区根写短一点，不在根底下就原样。
//!
//! 这是各自的平台的事（结果本身在工具箱里，路径怎么显示不在）。

use std::path::Path;

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
