//! 位置装载：根、账本、工作流目录、产物落点——三处位置由启动参数定。
//!
//! 工作区是建模单位，不是目录：领域那一侧只认内容（定义与工单），不认位置；
//! 物理位置由平台在装载时给（规格 `docs/specification/place/workspace.md`）。
//! 装载不属任何聚合——聚合只认内容，碰盘的事都在这里。
//!
//! 装载分三处：
//!
//! - **根（root）**：判据路径的基准、`run` 判据的工作目录、工作区级动作扫描的面；
//!   装载顺序 命令行 > 环境变量 `QTCLOUD_WORK_ROOT` > 从当前目录往上找到含
//!   `data/journal` 的第二大脑。
//! - **账本（data）**：CLI 自己维护的东西——工作区身份、工单、事件落在这里；
//!   缺省 `$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/`，指到仓库就等于入版控。
//! - **产物（artifacts）**：报告与日志是内容，不跟账本走；缺省落工作区根下的
//!   `artifacts/`，由 `--artifacts` 另指。
//!
//! 工作区身份缺则首跑生成（[`Locate::ensure`]）：`id` / `name` / `title` /
//! `description` / `created_at` / `updated_at`；`id` 供凭证派生用。位置不进模型：
//! 这些都不写进工单文件。

use crate::order::WorkOrder;
use crate::sha1::sha1_hex;
use serde_yaml::Value;
use std::path::{Path, PathBuf};

/// 工作区身份文件名。
pub const IDENTITY: &str = "workspace.yaml";
/// 事件文件名：领域事件落 JSONL，一条一行。
pub const EVENTS: &str = "events.jsonl";

/// 一次装载：根、账本与产物落点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locate {
    /// 工作区根：判据路径的基准。
    pub root: PathBuf,
    /// 账本：工作区身份、工单、事件。
    pub data: PathBuf,
    /// 工作流目录；缺省跟在账本里。
    pub workflows: Option<PathBuf>,
    /// 产物落点；缺省 `<根>/artifacts`。
    pub artifacts: PathBuf,
}

impl Locate {
    /// 装载：给了就用给的，没给按缺省的规矩找。
    pub fn resolve(
        root: Option<&Path>,
        data: Option<&Path>,
        workflows: Option<&Path>,
        artifacts: Option<&Path>,
    ) -> Locate {
        let root = match root {
            Some(root) => root.to_path_buf(),
            None => self::root(None),
        };
        let data = match data {
            Some(data) => data.to_path_buf(),
            None => account(&root),
        };
        let artifacts = match artifacts {
            Some(path) => path.to_path_buf(),
            None => root.join("artifacts"),
        };
        Locate {
            root,
            data,
            workflows: workflows.map(|p| p.to_path_buf()),
            artifacts,
        }
    }

    /// 工作流目录：给了 `--workflows` 用给的，否则跟在账本里。
    pub fn workflows_dir(&self) -> PathBuf {
        self.workflows
            .clone()
            .unwrap_or_else(|| self.data.join("workflows"))
    }

    /// 工单账本：`<账本>/workorders/`。
    pub fn workorders_dir(&self) -> PathBuf {
        self.data.join("workorders")
    }

    /// 工作区身份：`<账本>/workspace.yaml`。
    pub fn identity_file(&self) -> PathBuf {
        self.data.join(IDENTITY)
    }

    /// 事件文件：`<账本>/events.jsonl`。
    pub fn events_file(&self) -> PathBuf {
        self.data.join(EVENTS)
    }

    /// 工单文件：`<账本>/workorders/<名字>.yaml`。
    pub fn order_file(&self, order: &WorkOrder) -> PathBuf {
        self.workorders_dir().join(format!("{}.yaml", order.name))
    }

    /// 产物落点：`<产物落点>/<类别>/<工单名>.md`。
    pub fn artifact_path(&self, category: &str, order_name: &str) -> PathBuf {
        self.artifacts.join(crate::artifact::place::place(
            &crate::artifact::Artifact::named(category),
            order_name,
        ))
    }

    /// 写动作前把账本开出来：身份缺则首跑生成。只读动作不调它。
    pub fn ensure(&self) -> Result<(), String> {
        std::fs::create_dir_all(self.workorders_dir())
            .map_err(|e| format!("账本开不了：{}（{e}）", self.workorders_dir().display()))?;
        if self.workflows.is_none() {
            std::fs::create_dir_all(self.workflows_dir())
                .map_err(|e| format!("工作流目录开不了：{e}"))?;
        }
        if !self.identity_file().is_file() {
            let payload = crate::workspace::model::identity(&root_name(&self.root));
            write_yaml(&self.identity_file(), &Value::Mapping(payload))?;
        }
        Ok(())
    }

    /// 工作区 id：身份缺则先开账本——凭证派生认它，宁可落盘不可空算。
    pub fn workspace_id(&self) -> Result<String, String> {
        self.ensure()?;
        let text = std::fs::read_to_string(self.identity_file())
            .map_err(|e| format!("{} 读不了：{e}", self.identity_file().display()))?;
        let payload: Value = serde_yaml::from_str(&text)
            .map_err(|e| format!("{} 不是合法的 YAML：{e}", IDENTITY))?;
        Ok(crate::fields::text_of(&payload, "id"))
    }
}

/// 工作区根：装载顺序 命令行 > 环境变量 `QTCLOUD_WORK_ROOT` > 向上搜索——
/// 跨工作区干活不必每条命令带 `--root`，也不用站在根里。
pub fn root(cli_root: Option<&Path>) -> PathBuf {
    if let Some(root) = cli_root {
        return root.to_path_buf();
    }
    if let Some(root) = std::env::var_os("QTCLOUD_WORK_ROOT")
        .map(PathBuf::from)
        .filter(|root| !root.as_os_str().is_empty())
    {
        return root;
    }
    repo_root()
}

/// 工作区键：由工作区根的路径派生——可读名加短码。账本是「这台机器上的这个工作区」的账。
pub fn workspace_key(root: &Path) -> String {
    let resolved = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    format!(
        "{}-{}",
        resolved
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default(),
        &sha1_hex(&resolved.to_string_lossy())[..8]
    )
}

/// 账本缺省位置：`$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/`。
pub fn account(root: &Path) -> PathBuf {
    let home = std::env::var("XDG_DATA_HOME")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
        });
    home.join("qtcloud-work")
        .join("workspaces")
        .join(workspace_key(root))
}

/// 工作区根缺省：从起点往上找，直到看见数据层（含 `data/journal` 的目录）。
pub fn repo_root() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    loop {
        if dir.join("data").join("journal").is_dir() {
            return dir;
        }
        if !dir.pop() {
            return std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        }
    }
}

/// 路径怎么显示给人看：相对工作区根写短一点，不在根底下就原样。
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

fn root_name(root: &Path) -> String {
    root.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| root.to_string_lossy().to_string())
}

pub(crate) fn write_yaml(path: &Path, payload: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{} 建不了：{e}", parent.display()))?;
    }
    std::fs::write(path, serde_yaml::to_string(payload).unwrap_or_default())
        .map_err(|e| format!("{} 写不了：{e}", path.display()))
}
