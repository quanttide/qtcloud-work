//! 资产层：第二大脑应该有什么、叫什么、落在哪。
//!
//! 依据量潮第二大脑章程第九条（程序型）与第十三条（陈述型）；资产的中文用名是
//! 本领域的命名决定，改动即改规范。
//!
//! 产物实例（[`Artifact`]：名字 + 规格，一件产出物）在 `model`——与上面的资产表
//! 分文件：资产表是**类型**清单，`Artifact` 是**实例**。

mod model;

pub use model::*;

use std::path::{Path, PathBuf};

/// 陈述型九宫格与不占格资产。
pub const STATED: [(&str, &str); 11] = [
    ("报告", "report"),
    ("参考", "library"),
    ("历史", "history"),
    ("日志", "journal"),
    ("档案", "profile"),
    ("宣传册", "brochure"),
    ("路线图", "roadmap"),
    ("洞察", "insight"),
    ("意图", "intention"),
    ("语境", "context"),
    ("归档", "archive"),
];

/// 程序型九宫格。
pub const PROCEDURAL: [(&str, &str); 9] = [
    ("章程", "bylaw"),
    ("规格", "specification"),
    ("工具箱", "toolkit"),
    ("手册", "handbook"),
    ("案例", "gallery"),
    ("平台", "platform"),
    ("教程", "tutorial"),
    ("札记", "essay"),
    ("实验室", "example"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    pub category: String,
    pub name: String,
}

pub fn assets() -> Vec<Asset> {
    STATED
        .iter()
        .chain(PROCEDURAL.iter())
        .map(|(category, name)| Asset {
            category: category.to_string(),
            name: name.to_string(),
        })
        .collect()
}

/// 非同名目录的三格，按命名规则找独立仓库。
fn locate_pattern(name: &str) -> Option<(&'static str, &'static str)> {
    match name {
        "toolkit" => Some(("packages", "-toolkit")),
        "platform" => Some(("apps", "")),
        "example" => Some(("examples", "")),
        _ => None,
    }
}

/// 落点：文档类入 `data/` 或 `docs/` 的同名目录，独立仓库按命名规则找。
pub fn locate(root: &Path, asset: &Asset) -> Vec<PathBuf> {
    for candidate in [
        root.join("data").join(&asset.name),
        root.join("docs").join(&asset.name),
    ] {
        if candidate.is_dir() {
            return vec![candidate];
        }
    }
    if let Some((container, suffix)) = locate_pattern(&asset.name) {
        let base = root.join(container);
        if let Ok(entries) = std::fs::read_dir(&base) {
            let mut found: Vec<PathBuf> = entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.is_dir())
                .filter(|p| {
                    let name = p
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    if suffix.is_empty() {
                        !name.starts_with('.')
                    } else {
                        name.ends_with(suffix)
                    }
                })
                .collect();
            found.sort();
            return found;
        }
    }
    Vec::new()
}

/// 资产表有而工作区无的格子。
pub fn missing(root: &Path) -> Vec<Asset> {
    assets()
        .into_iter()
        .filter(|asset| locate(root, asset).is_empty())
        .collect()
}

/// 补建缺的格子：文档类建 `data/` 或 `docs/` 下的同名目录，各带一份 README。
/// 独立仓库那三格不凭空建——它们是另外的仓库。
pub fn make(root: &Path, wanted: Option<&[Asset]>) -> Vec<PathBuf> {
    let stated: Vec<&str> = STATED.iter().map(|(_, name)| *name).collect();
    let list = match wanted {
        Some(list) => list.to_vec(),
        None => missing(root),
    };
    let mut created = Vec::new();
    for asset in list {
        if locate_pattern(&asset.name).is_some() {
            continue;
        }
        let container = if stated.contains(&asset.name.as_str()) {
            "data"
        } else {
            "docs"
        };
        let path = root.join(container).join(&asset.name);
        let _ = std::fs::create_dir_all(&path);
        let readme = path.join("README.md");
        if !readme.is_file() {
            let _ = std::fs::write(&readme, format!("# 量潮知识工作{}\n", asset.category));
        }
        created.push(path);
    }
    created
}
