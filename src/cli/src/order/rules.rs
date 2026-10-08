//! 工单聚合 / 跑判据：文件系统、起进程。
//!
//! 判据是**字段**，不是一行小语法：`path` 存在、`absent` 不存在、
//! `file` 加 `contains` 含这段文字、`run` 这条命令退出码为零。
//! 路径相对工作区根；写绝对路径则按绝对路径（跨仓库核对用）。
//! 翻译（说明怎么写、四种判法怎么认）在 `crate::criterion`；本件只剩「真去跑」。

use crate::criterion::{RuleItem, RuleKind};
use std::path::Path;
use std::process::Command;

/// 跑一条判据，返回（是否通过，说明）。
fn check(root: &Path, item: &RuleItem) -> (bool, String) {
    match item.kind {
        Some(RuleKind::PathExists) => {
            let target = &item.args[0];
            (root.join(target).exists(), target.clone())
        }
        Some(RuleKind::PathAbsent) => {
            let target = &item.args[0];
            (!root.join(target).exists(), target.clone())
        }
        Some(RuleKind::FileContains) => {
            let target = &item.args[0];
            let needle = &item.args[1];
            let path = root.join(target);
            if !path.is_file() {
                return (false, format!("{target} 不存在"));
            }
            let body = std::fs::read_to_string(&path).unwrap_or_default();
            (
                body.contains(needle.as_str()),
                format!("{target} 含「{needle}」"),
            )
        }
        Some(RuleKind::CommandRun) => {
            let command = &item.args[0];
            let done = Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(root)
                .output();
            match done {
                Ok(out) if out.status.success() => (true, command.clone()),
                Ok(out) => {
                    let text = String::from_utf8_lossy(&out.stderr).trim().to_string();
                    let text = if text.is_empty() {
                        String::from_utf8_lossy(&out.stdout).trim().to_string()
                    } else {
                        text
                    };
                    let tail = text.lines().last().unwrap_or("无输出").trim().to_string();
                    (false, format!("{command}——{tail}"))
                }
                Err(e) => (false, format!("{command}——{e}")),
            }
        }
        None => (false, "不认得的判据".to_string()),
    }
}

/// 跑全部要跑的判据，返回（逐条结果，不跑的——留给智能体或人）。
pub fn run(root: &Path, items: &[RuleItem]) -> (Vec<(RuleItem, bool, String)>, Vec<RuleItem>) {
    let results = items
        .iter()
        .filter(|i| i.machine())
        .map(|i| {
            let (passed, spec) = check(root, i);
            (i.clone(), passed, spec)
        })
        .collect();
    let pending = items.iter().filter(|i| !i.machine()).cloned().collect();
    (results, pending)
}
