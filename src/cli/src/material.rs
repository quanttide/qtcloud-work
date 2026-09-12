//! 材料：还没做成成品的输入——类型、内容、来源、时间。
//!
//! 四字段都能自动填出：类型取扩展名、内容取正文、来源取上级目录与文件名、
//! 时间取文件自己那个仓库里首次提交的日期（退回文件名里的日期）。
//! 原始与材料之分不占字段，由资产位置承担：`journal` 是原始，其余是材料。

use std::path::{Path, PathBuf};
use std::process::Command;

const PROSE: [&str; 3] = ["md", "txt", "rst"];

#[derive(Debug, Clone)]
pub struct Material {
    pub r#type: String,
    pub content: String,
    pub source: String,
    pub created_at: String,
    pub stage: String,
}

impl Material {
    pub fn missing(&self) -> Vec<&'static str> {
        let fields = [
            ("类型", &self.r#type),
            ("内容", &self.content),
            ("来源", &self.source),
            ("时间", &self.created_at),
        ];
        fields
            .iter()
            .filter(|(_, value)| value.is_empty())
            .map(|(name, _)| *name)
            .collect()
    }
}

/// 时间：优先取文件所在仓库里首次提交的日期，其次取文件名里的日期。
fn first_seen(path: &Path) -> String {
    let mut dir = path.parent().map(|p| p.to_path_buf());
    while let Some(current) = dir {
        if current.join(".git").exists() {
            if let Ok(relative) = path.strip_prefix(&current) {
                let out = Command::new("git")
                    .args(["log", "--diff-filter=A", "--format=%as", "--"])
                    .arg(relative)
                    .current_dir(&current)
                    .output();
                if let Ok(out) = out {
                    let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    if let Some(last) = text.lines().last()
                        && !last.is_empty()
                    {
                        return last.to_string();
                    }
                }
            }
            break;
        }
        dir = current.parent().map(|p| p.to_path_buf());
    }
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let digits: Vec<&str> = stem
        .split('-')
        .filter(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
        .collect();
    if digits.len() >= 3 {
        digits[..3].join("-")
    } else {
        String::new()
    }
}

/// 阶段：由资产位置承担——日志是原始，其余是材料。
fn stage_of(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(rest) => {
            if rest.components().any(|c| c.as_os_str() == "journal") {
                "原始"
            } else {
                "材料"
            }
        }
        Err(_) => "材料",
    }
    .to_string()
}

pub fn as_material(root: &Path, path: &Path) -> Material {
    let extension = path
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();
    let text = if PROSE.contains(&extension.as_str()) {
        std::fs::read_to_string(path).unwrap_or_default()
    } else {
        String::new()
    };
    let body = if let Some(rest) = text.strip_prefix("# ") {
        rest.split_once('\n')
            .map(|(_, body)| body)
            .unwrap_or("")
            .trim()
            .to_string()
    } else {
        text.trim().to_string()
    };
    let snippet: String = body.chars().take(40).collect();
    let content = if body.chars().count() > 40 {
        format!("{}…", snippet.replace('\n', " "))
    } else {
        snippet.replace('\n', " ")
    };
    Material {
        r#type: extension,
        content,
        source: format!(
            "{}/{}",
            path.parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default(),
            path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default()
        ),
        created_at: first_seen(path),
        stage: stage_of(root, path),
    }
}

/// 逐条读材料：给了路径就看那些，没给就看日志与档案里的文档。
pub fn materials(root: &Path, rels: Option<&[String]>) -> Vec<(String, Material)> {
    if let Some(rels) = rels {
        return rels
            .iter()
            .map(|rel| {
                let path = if rel.starts_with('/') {
                    PathBuf::from(rel)
                } else {
                    root.join(rel)
                };
                (rel.clone(), as_material(root, &path))
            })
            .collect();
    }
    let mut found = Vec::new();
    for kind in ["journal", "profile"] {
        let base = root.join("data").join(kind);
        if !base.is_dir() {
            continue;
        }
        let mut docs = Vec::new();
        collect_markdown(&base, &mut docs);
        docs.sort();
        for path in docs {
            let rel = path
                .strip_prefix(root)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| path.to_string_lossy().to_string());
            let material = as_material(root, &path);
            found.push((rel, material));
        }
    }
    found
}

fn collect_markdown(base: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(base) else {
        return;
    };
    for child in entries.flatten().map(|e| e.path()) {
        if child.is_dir() {
            collect_markdown(&child, out);
        } else if child.extension().map(|e| e == "md").unwrap_or(false) {
            out.push(child);
        }
    }
}

use quanttide_work::outcome::Outcome;
use serde_json::json;
// ---- 动作 ----

pub fn material(root: &Path, paths: Option<&[String]>) -> Outcome {
    let found = materials(root, paths);
    let mut result = Outcome::new(true);
    result.columns = vec![
        "材料".to_string(),
        "类型".to_string(),
        "阶段".to_string(),
        "时间".to_string(),
        "来源".to_string(),
    ];
    for (rel, mat) in &found {
        let created = if mat.created_at.is_empty() {
            "（缺）".to_string()
        } else {
            mat.created_at.clone()
        };
        result.rows.push(vec![
            rel.clone(),
            mat.r#type.clone(),
            mat.stage.clone(),
            created.clone(),
            mat.source.clone(),
        ]);
        result.lines.push(format!(
            "{rel:52} {:5} {:5} {created:11} {}",
            mat.r#type, mat.stage, mat.source
        ));
        if !mat.missing().is_empty() {
            result.ok = false;
            result
                .lines
                .push(format!("缺字段：{rel}——{}", mat.missing().join("、")));
        }
    }
    if result.ok {
        result
            .lines
            .push("阶段由资产位置承担：日志是原始，其余是材料。".to_string());
    }
    result.data = Some(json!({
        "count": found.len(),
        "materials": found.iter().map(|(rel, mat)| json!({
            "path": rel,
            "type": mat.r#type,
            "content": mat.content,
            "source": mat.source,
            "created_at": mat.created_at,
            "stage": mat.stage,
        })).collect::<Vec<_>>(),
    }));
    result
}

// ---- 工作流 ----
