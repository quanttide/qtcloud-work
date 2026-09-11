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

use serde_yaml::Value;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Path,
    Absent,
    Contains,
    Run,
}

/// 一条要跑的判据：说明 + 怎么判（`kind` 为空即不跑，交给智能体或人）。
#[derive(Debug, Clone)]
pub struct Item {
    pub description: String,
    pub kind: Option<Kind>,
    pub args: Vec<String>,
}

impl Item {
    pub fn machine(&self) -> bool {
        self.kind.is_some()
    }
}

fn text(criterion: &Value, key: &str) -> Option<String> {
    criterion
        .get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// 说明：写了就用写的，没写按字段拼一句。
pub fn description_of(criterion: &Value) -> String {
    if let Some(written) = text(criterion, "description")
        && !written.trim().is_empty()
    {
        return written.trim().to_string();
    }
    if let Some(path) = text(criterion, "path") {
        return format!("存在：{path}");
    }
    if let Some(path) = text(criterion, "absent") {
        return format!("不存在：{path}");
    }
    if let Some(file) = text(criterion, "file") {
        let needle = text(criterion, "contains").unwrap_or_default();
        return format!("含「{needle}」：{file}");
    }
    if let Some(run) = text(criterion, "run") {
        return format!("跑通：{run}");
    }
    String::new()
}

/// 把定义里的判据翻成要跑的东西：rule 的跑，agent / human 的不跑。
pub fn items_of(criteria: &[Value]) -> Vec<Item> {
    let mut items = Vec::new();
    for criterion in criteria {
        let description = description_of(criterion);
        let is_rule = criterion.get("executor").and_then(|v| v.as_str()) == Some("rule");
        if !is_rule {
            items.push(Item {
                description,
                kind: None,
                args: Vec::new(),
            });
            continue;
        }
        if let Some(path) = text(criterion, "path") {
            items.push(Item {
                description,
                kind: Some(Kind::Path),
                args: vec![path],
            });
        } else if let Some(path) = text(criterion, "absent") {
            items.push(Item {
                description,
                kind: Some(Kind::Absent),
                args: vec![path],
            });
        } else if let Some(file) = text(criterion, "file") {
            let needle = text(criterion, "contains").unwrap_or_default();
            items.push(Item {
                description,
                kind: Some(Kind::Contains),
                args: vec![file, needle],
            });
        } else if let Some(run) = text(criterion, "run") {
            items.push(Item {
                description,
                kind: Some(Kind::Run),
                args: vec![run],
            });
        }
    }
    items
}

/// 跑一条判据，返回（是否通过，说明）。
pub fn check(root: &Path, item: &Item) -> (bool, String) {
    match item.kind {
        Some(Kind::Path) => {
            let target = &item.args[0];
            (root.join(target).exists(), target.clone())
        }
        Some(Kind::Absent) => {
            let target = &item.args[0];
            (!root.join(target).exists(), target.clone())
        }
        Some(Kind::Contains) => {
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
        Some(Kind::Run) => {
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
use crate::outcome::{Result, short};

pub fn audit(root: &Path, make: bool) -> Result {
    let made = if make {
        crate::artifact::make(root, None)
    } else {
        Vec::new()
    };
    let missing = crate::artifact::missing(root);
    let unregistered = catalog::build(root).unregistered(root);
    let ok = missing.is_empty() && unregistered.is_empty();
    let mut result = Result {
        ok,
        ..Default::default()
    };
    result.columns = vec!["问题".to_string(), "说明".to_string()];
    for asset in &missing {
        result.rows.push(vec![
            "缺资产".to_string(),
            format!("{}（{}）", asset.kind, asset.name),
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
    result.payload = Some(json!({
        "root": root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
        "result": if ok { "通过" } else { "有问题" },
        "missing": missing.iter().map(|a| json!({"kind": a.kind, "name": a.name})).collect::<Vec<_>>(),
        "unregistered": unregistered.iter().map(|p| short(root, p)).collect::<Vec<_>>(),
    }));
    result
}
