//! 目录层：按资产表清点工作区里实际有什么，建成名字索引。
//!
//! 目录是快照——仓库变了要重扫；名字索引同时收文件名与篇内标题，因为命名规则规定
//! 英文文件名与中文标题不互译。

use serde_json::{Value as Json, json};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const CONTAINERS: [&str; 5] = ["data", "docs", "packages", "apps", "examples"];
const SKIP: [&str; 7] = [
    ".git",
    "node_modules",
    ".venv",
    "build",
    "dist",
    ".dart_tool",
    "__pycache__",
];
const FACADE: [&str; 3] = ["README.md", "CHANGELOG.md", "LICENSE"];

#[derive(Debug, Clone)]
pub struct Entry {
    pub kind: String,
    pub path: PathBuf,
    pub names: BTreeSet<String>,
}

pub struct Catalog {
    pub entries: Vec<Entry>,
}

impl Catalog {
    pub fn add(&mut self, kind: &str, path: PathBuf, names: BTreeSet<String>) {
        self.entries.push(Entry {
            kind: kind.to_string(),
            path,
            names,
        });
    }

    /// 目录有而契约无：未登记在资产表里的顶层子目录。
    pub fn unregistered(&self, root: &Path) -> Vec<PathBuf> {
        let known: BTreeSet<PathBuf> = crate::artifact::assets()
            .iter()
            .flat_map(|asset| crate::artifact::locate(root, asset))
            .collect();
        let mut found: Vec<PathBuf> = Vec::new();
        for container in CONTAINERS {
            let parent = root.join(container);
            if !parent.is_dir() {
                continue;
            }
            if let Ok(entries) = std::fs::read_dir(&parent) {
                for child in entries.flatten().map(|e| e.path()) {
                    let name = child
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    if child.is_dir() && !name.starts_with('.') && !known.contains(&child) {
                        found.push(child);
                    }
                }
            }
        }
        found.sort();
        found
    }
}

/// 目录的自带格式：JSON——每条含种类、路径与全部名字。
pub fn payload(root: &Path, catalog: &Catalog) -> Json {
    json!({
        "root": root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
        "count": catalog.entries.len(),
        "entries": catalog.entries.iter().map(|entry| json!({
            "kind": entry.kind,
            "path": short(root, &entry.path),
            "names": entry.names.iter().cloned().collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    })
}

pub fn short(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(rest) => rest.to_string_lossy().to_string(),
        Err(_) => path.to_string_lossy().to_string(),
    }
}

pub fn write_json(target: &Path, payload: &Json) {
    if let Some(parent) = target.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let text = serde_json::to_string_pretty(payload).unwrap_or_default();
    let _ = std::fs::write(target, format!("{text}\n"));
}

/// 篇内标题：正文第一个一级标题。
fn title_of(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    text.lines()
        .find(|line| line.starts_with("# "))
        .map(|line| line[2..].trim().to_string())
}

/// 仓库的中文名：README 里「量潮知识工作X」或首行的量潮品牌名。
fn cn_name(path: &Path) -> Option<String> {
    let readme = path.join("README.md");
    if !readme.is_file() {
        return None;
    }
    let text = std::fs::read_to_string(&readme).ok()?;
    for line in text.lines() {
        let Some(rest) = line.strip_prefix("# 量潮") else {
            continue;
        };
        let value = format!("量潮{rest}").trim().to_string();
        if !value.is_empty() {
            return Some(value);
        }
    }
    for line in text.lines() {
        if let Some(at) = line.find("量潮知识工作") {
            let tail = &line[at + "量潮知识工作".len()..];
            if let Some(end) = tail.find("——") {
                let value: String = tail[..end].chars().take(8).collect();
                if !value.is_empty() {
                    return Some(format!("量潮知识工作{value}"));
                }
            }
        }
    }
    None
}

/// 资产下的文档：Markdown，跳过构建目录与门面文件。
fn documents(path: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    for child in entries.flatten().map(|e| e.path()) {
        let name = child
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        if child.is_dir() {
            if SKIP.contains(&name.as_str()) {
                continue;
            }
            documents(&child, out);
        } else if name.ends_with(".md") && !FACADE.contains(&name.as_str()) {
            out.push(child);
        }
    }
}

pub fn build(root: &Path) -> Catalog {
    let mut catalog = Catalog {
        entries: Vec::new(),
    };
    for asset in crate::artifact::assets() {
        for path in crate::artifact::locate(root, &asset) {
            let mut names: BTreeSet<String> = BTreeSet::new();
            names.insert(asset.kind.clone());
            names.insert(asset.name.clone());
            if let Some(name) = path.file_name() {
                names.insert(name.to_string_lossy().to_string());
            }
            if let Some(alias) = cn_name(&path) {
                names.insert(alias);
            }
            catalog.add(&asset.kind, path.clone(), names);
            let mut docs = Vec::new();
            documents(&path, &mut docs);
            docs.sort();
            for md in docs {
                let mut doc_names: BTreeSet<String> = BTreeSet::new();
                if let Some(stem) = md.file_stem() {
                    doc_names.insert(stem.to_string_lossy().to_string());
                }
                if let Some(title) = title_of(&md) {
                    doc_names.insert(title);
                }
                catalog.add(&asset.kind, md, doc_names);
            }
        }
    }
    catalog
}

// ---- 动作（看目录）----

use quanttide_work::outcome::Outcome;

pub fn catalog(root: &Path) -> Outcome {
    let found = build(root);
    let mut result = Outcome {
        ok: true,
        ..Default::default()
    };
    result.columns = vec!["种类".to_string(), "路径".to_string()];
    for entry in &found.entries {
        let rel = short(root, &entry.path);
        result.lines.push(format!("[{}] {rel}", entry.kind));
        result.rows.push(vec![entry.kind.clone(), rel]);
    }
    result.data = Some(payload(root, &found));
    result
}
