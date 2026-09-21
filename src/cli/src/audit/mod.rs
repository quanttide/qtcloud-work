//! 判据：跑定义里写下的机械核对。
//!
//! 规则引擎的判据是**字段**，不是一行小语法：
//!
//! ```yaml
//! - executor: rule
//!   path: docs/index.md            # 路径存在
//! - executor: rule
//!   absent: docs/old.md            # 路径不存在
//! - executor: rule
//!   file: docs/index.md            # 文件含这段文字（file + contains 成对）
//!   contains: 第二大脑
//! - executor: rule
//!   run: test -f docs/index.md     # 命令在工作区根跑，退出码为零
//! ```
//!
//! 路径相对工作区根；写绝对路径则按绝对路径（跨仓库核对用）。

use std::path::Path;
use std::process::Command;

// 判据的翻译（说明怎么写、四种判法怎么认）在工具箱里；这里只剩「真去跑」。
pub use crate::criterion::{RuleItem as Item, RuleKind as Kind, items_of};

/// 跑一条判据，返回（是否通过，说明）。
pub fn check(root: &Path, item: &Item) -> (bool, String) {
    match item.kind {
        Some(Kind::PathExists) => {
            let target = &item.args[0];
            (root.join(target).exists(), target.clone())
        }
        Some(Kind::PathAbsent) => {
            let target = &item.args[0];
            (!root.join(target).exists(), target.clone())
        }
        Some(Kind::FileContains) => {
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
        Some(Kind::CommandRun) => {
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
pub fn run(root: &Path, items: &[Item]) -> (Vec<(Item, bool, String)>, Vec<Item>) {
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

use serde_json::json;
// ---- 动作 ----

use crate::catalog;
use crate::outcome::Outcome;
use crate::workspace::short;

pub fn audit(root: &Path, make: bool) -> Outcome {
    let made = if make {
        crate::artifact::make(root, None)
    } else {
        Vec::new()
    };
    let missing = crate::artifact::missing(root);
    let unregistered = catalog::build(root).unregistered(root);
    let ok = missing.is_empty() && unregistered.is_empty();
    let mut result = Outcome {
        ok,
        ..Default::default()
    };
    result.columns = vec!["问题".to_string(), "说明".to_string()];
    for asset in &missing {
        result.rows.push(vec![
            "缺资产".to_string(),
            format!("{}（{}）", asset.category, asset.name),
        ]);
    }
    for path in &unregistered {
        result
            .rows
            .push(vec!["未登记".to_string(), short(root, path)]);
    }
    result.lines = made
        .iter()
        .map(|path| format!("补建：{}", short(root, path)))
        .collect();
    result.lines.extend(
        result
            .rows
            .iter()
            .map(|row| format!("{}：{}", row[0], row[1])),
    );
    if ok {
        result
            .lines
            .push("审计通过：二十格齐备，无未登记目录。".to_string());
    } else if !unregistered.is_empty() && missing.is_empty() {
        result
            .lines
            .push("未登记的目录要么属于某一格（改资产表），要么不该在这儿。".to_string());
    }
    result.data = Some(json!({
        "root": root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
        "result": if ok { "通过" } else { "有问题" },
        "missing": missing.iter().map(|a| json!({"category": a.category, "name": a.name})).collect::<Vec<_>>(),
        "unregistered": unregistered.iter().map(|p| short(root, p)).collect::<Vec<_>>(),
    }));
    result
}
