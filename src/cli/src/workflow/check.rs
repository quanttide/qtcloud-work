//! 工作流聚合 / 定义核对：声明与判据对不对得上。

use super::WorkflowFile;
use quanttide_work::workflow::Finding;
use std::path::Path;

/// 核对一条工作流：判据里的路径在不在；描述里提到的报告小节有没有判据覆盖。
///
/// 规矩在工具箱里；这里只把「路径在不在」用工作区根包一层。
pub fn check(flow: &WorkflowFile, root: &Path, data: &Path) -> Vec<Finding> {
    flow.shared().check(&data.to_string_lossy(), |written| {
        let path = Path::new(written);
        let target = if path.is_absolute() {
            path.to_path_buf()
        } else {
            root.join(path)
        };
        target.exists()
    })
}

/// 核对结果写成人读的一段。
pub fn describe(found: &[Finding]) -> Vec<String> {
    let mut lines = vec![format!("核对 {} 件事", found.len())];
    for item in found {
        lines.push(format!(
            "  {} {}——{}",
            if item.ok { "✓" } else { "✗" },
            item.where_,
            item.what
        ));
    }
    if found.is_empty() {
        lines.push("  （这条定义里没有可核对的路径与小节）".to_string());
    }
    lines
}

pub fn all_ok(found: &[Finding]) -> bool {
    found.iter().all(|item| item.ok)
}
