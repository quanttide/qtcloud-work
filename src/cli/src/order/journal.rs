//! 工单聚合 / 日志叙事：一段一段往下写。
//!
//! 日志是产物（内容），流水是账——写日志不记流水，账不替货作证
//! （出处：`docs/specification/piece/artifact.md`）。

/// 产物类别：报告。
pub const REPORT: &str = "report";
/// 产物类别：日志。
pub const JOURNAL: &str = "journal";

/// 日志收叙事：别的节原样保留，只把占位的临时说明滤掉。
pub fn narrate(
    locate: &crate::workspace::LocalWorkspace,
    order_name: &str,
    words: &str,
) -> std::path::PathBuf {
    let path = locate.artifact_path(JOURNAL, order_name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| format!("# 日志：{order_name}\n"));
    let text = text
        .lines()
        .filter(|line| {
            let stripped = line.trim();
            !(stripped.starts_with('（') && stripped.ends_with('）'))
        })
        .collect::<Vec<_>>()
        .join("\n");
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&path, format!("{}\n\n{}\n", text.trim_end(), words.trim()));
    path
}
