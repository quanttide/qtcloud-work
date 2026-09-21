//! 工作流聚合 / 定义核对：声明与判据对不对得上。
//!
//! 只看写下的位置、不访问文件系统（出处：`docs/specification/process/workflow.md`·约束）：
//!
//! - 判据里 `path` / `file` 的路径须在工作区内；
//! - 描述里点到的小节须有 `contains` 判据覆盖——小节只认干净的名字
//!   （`##` 起的标题或引号里的短名），引号里的长句当叙述。

use crate::criterion::Criterion;
use crate::paths::placeholders_in;
use crate::workflow::Workflow;
use std::path::Path;

/// 定义核对出来的一件事：在哪里、核的是什么、过没过。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub where_: String,
    pub what: String,
    pub ok: bool,
}

/// 核对一条定义：判据路径须在区内，描述点到的小节须有 `contains` 判据覆盖。
pub fn check(workflow: &Workflow, root: &Path) -> Vec<Finding> {
    let mut found: Vec<Finding> = Vec::new();
    for step in &workflow.steps {
        for criterion in step.rules() {
            let literal = match &criterion {
                Criterion::PathExists { path, .. } => path.clone(),
                Criterion::FileContains { file, .. } => file.clone(),
                _ => continue,
            };
            // 占位是运行时按产物落点展开的，不是写下的位置，不核。
            if !placeholders_in(&literal).is_empty() {
                continue;
            }
            let inside = inside(root, &literal);
            found.push(Finding {
                where_: format!("{}·{}", step.name, literal),
                what: format!("判据里的路径在工作区内：{literal}"),
                ok: inside,
            });
        }
        let covered: Vec<String> = step
            .rules()
            .iter()
            .filter_map(|criterion| match criterion {
                Criterion::FileContains { contains, .. } => Some(contains.clone()),
                _ => None,
            })
            .collect();
        for name in sections(&step.description) {
            found.push(Finding {
                where_: format!("{}·description", step.name),
                what: format!("描述点到的小节有 contains 判据覆盖：{name}"),
                ok: covered.iter().any(|value| value.contains(&name)),
            });
        }
    }
    found
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

/// 像不像小节名：干净的短名——全是字词、不超过十个字；版本号、标点、路径都不算。
fn looks_like_section(name: &str) -> bool {
    !name.is_empty() && name.chars().count() <= 10 && name.chars().all(|ch| ch.is_alphanumeric())
}

/// 路径落在工作区内（不访问文件系统，只看写下的位置）。
fn inside(root: &Path, written: &str) -> bool {
    let written = written.trim();
    if written.is_empty() || written.starts_with('<') {
        return true;
    }
    let path = Path::new(written);
    if path.is_absolute() {
        return path.starts_with(root);
    }
    !path
        .components()
        .any(|part| part == std::path::Component::ParentDir)
}

/// 描述里点到的小节：`##` 起的标题，或引号里的短名。只认干净的名字——
/// 长句当叙述，带标点、路径、占位的不算。小节按出现次序：`##` 与「」两种写法混排时，
/// 谁先出现谁在前。
fn sections(text: &str) -> Vec<String> {
    let mut hits: Vec<(usize, String)> = Vec::new();

    let mut rest = text;
    let mut offset = 0;
    while let Some(at) = rest.find("##") {
        let after = &rest[at + 2..];
        let name: String = after
            .trim_start()
            .chars()
            .take_while(|ch| !ch.is_whitespace())
            .collect();
        hits.push((offset + at, name));
        offset += at + 2;
        rest = after;
    }

    let mut rest = text;
    let mut offset = 0;
    while let Some(at) = rest.find('「') {
        let after = &rest[at + '「'.len_utf8()..];
        let Some(end) = after.find('」') else { break };
        hits.push((offset + at, after[..end].to_string()));
        offset += at + '「'.len_utf8() + end + '」'.len_utf8();
        rest = &after[end + '」'.len_utf8()..];
    }

    hits.sort_by_key(|(at, _)| *at);
    let mut found: Vec<String> = Vec::new();
    for (_, name) in hits {
        let name = name.trim();
        if looks_like_section(name) && !found.iter().any(|seen| seen == name) {
            found.push(name.to_string());
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sections_only_takes_clean_names() {
        assert_eq!(sections("看「报告」与##方法 两节"), vec!["报告", "方法"]);
        assert!(sections("「这句话是很长的叙述，不当小节名」").is_empty());
        assert!(sections("版本号 ## [0.1.0] 不算").is_empty());
    }

    #[test]
    fn inside_rejects_escaping_paths() {
        let root = Path::new("/repo");
        assert!(inside(root, "docs/index.md"));
        assert!(inside(root, "<相对工作区的路径>"));
        assert!(!inside(root, "../outside.md"));
        assert!(inside(root, "/repo/docs/index.md"));
        assert!(!inside(root, "/elsewhere/index.md"));
    }
}
