//! 工作流聚合：串联的工作步骤——过程的编排定义，用 YAML 存。
//!
//! 定义要有**固定的意义**，所以是 YAML 而不是散文：字段名、字段取值、判据种类都由 schema
//! 定死，不认识的字段直接报错。
//!
//! 定义这一类**领域模型**（模型、读法与语法校验）在 `model` / `read` 里，本仓自持。
//! 这一层另管命令行自己的两件事：文件读写（`<工作流目录>/<名字>.yaml`，文件名即工作流名）
//! 与把动作写成信封。
//!
//! 定义不带凭证：工作流按「工作区 id + 名字」、步骤按「工作流凭证 + 名字」现算
//! （`crate::ids`、[`Workflow::credentials`]）——指到哪个工作区都能直接跑，
//! 程序不必先导入、也不往人写的文件里添字。
//!
//! 子件按事分：模型在 `model`、读法与语法校验在 `read`、YAML 读写与装载在 `yaml`、
//! 五个动作在 `actions`。

mod actions;
mod model;
mod read;
mod yaml;

pub use actions::{
    workflow_check, workflow_create, workflow_export, workflow_import, workflow_list, workflow_show,
};
pub use model::{Step, Workflow};
pub use read::validate;
pub use yaml::{WorkflowError, load, text_of};

// 执行者取值：一处定义，本仓自持。
pub use crate::executor::{AGENT, HUMAN, RULE};

use crate::locate::Locate;
use serde_yaml::{Mapping, Value};

/// 一条定义**连同它的文件位置**（[`Workflow`] 那份只管内容，不管文件）。
pub struct WorkflowFile {
    pub name: String,
    pub payload: Value,
    pub locate: Locate,
}

impl WorkflowFile {
    pub fn new(locate: &Locate, name: &str, payload: Value) -> Self {
        WorkflowFile {
            name: name.to_string(),
            payload,
            locate: locate.clone(),
        }
    }

    pub fn file(&self) -> std::path::PathBuf {
        self.locate
            .workflows_dir()
            .join(format!("{}.yaml", self.name))
    }

    pub fn exists(&self) -> bool {
        self.file().is_file()
    }

    pub fn reload(mut self) -> Self {
        if self.exists()
            && let Ok(payload) = load(&self.file())
        {
            self.payload = payload;
        }
        self
    }

    /// 内容那一层：模型（不带凭证）。
    pub fn shared(&self) -> Workflow {
        Workflow::of(&self.payload)
    }

    /// 内容那一层，凭证已按「工作区 id + 名字」补上。
    pub fn credentialed(&self) -> Result<Workflow, String> {
        let workspace_id = self.locate.workspace_id()?;
        Ok(self.shared().credentials(&workspace_id))
    }

    pub fn description(&self) -> String {
        self.shared().description()
    }

    /// 步骤：按定义里的顺序——这就是「串联」。
    pub fn steps(&self) -> Vec<Step> {
        self.shared().steps()
    }

    pub fn to_yaml(&self) -> String {
        yaml::dump(&self.payload)
    }
}

/// 写一条工作流：步骤串联，每步给一份判据骨架（执行者默认 AI）。定义不带凭证。
pub fn create(
    locate: &Locate,
    name: &str,
    steps: &[String],
    note: &str,
) -> Result<WorkflowFile, WorkflowError> {
    let mut payload = Mapping::new();
    payload.insert(
        Value::String("name".into()),
        Value::String(name.to_string()),
    );
    let description = if note.trim().is_empty() {
        "步骤串联：写清每步做什么、谁执行、怎么判。"
    } else {
        note.trim()
    };
    payload.insert(
        Value::String("description".into()),
        Value::String(description.to_string()),
    );
    let steps_value: Vec<Value> = steps
        .iter()
        .map(|step| {
            let mut criterion = Mapping::new();
            criterion.insert(Value::String("executor".into()), Value::String(RULE.into()));
            criterion.insert(
                Value::String("path".into()),
                Value::String("data/journal/README.md".into()),
            );
            let mut gate = Mapping::new();
            gate.insert(
                Value::String("executor".into()),
                Value::String(HUMAN.into()),
            );
            gate.insert(
                Value::String("description".into()),
                Value::String("<只能人拍板的>".into()),
            );
            let mut item = Mapping::new();
            item.insert(Value::String("name".into()), Value::String(step.clone()));
            item.insert(
                Value::String("description".into()),
                Value::String(format!("<{step}这一步做什么>")),
            );
            item.insert(
                Value::String("executor".into()),
                Value::String(AGENT.into()),
            );
            item.insert(
                Value::String("criteria".into()),
                Value::Sequence(vec![Value::Mapping(criterion), Value::Mapping(gate)]),
            );
            Value::Mapping(item)
        })
        .collect();
    payload.insert(Value::String("steps".into()), Value::Sequence(steps_value));
    let flow = WorkflowFile::new(locate, name, Value::Mapping(payload));
    write(&flow).map_err(WorkflowError)?;
    Ok(flow)
}

/// 落盘：建父目录、写 YAML。
pub(crate) fn write(flow: &WorkflowFile) -> Result<(), String> {
    if let Some(parent) = flow.file().parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{} 建不了：{e}", parent.display()))?;
    }
    std::fs::write(flow.file(), flow.to_yaml())
        .map_err(|e| format!("{} 写不了：{e}", flow.file().display()))
}

pub fn open(locate: &Locate, name: &str) -> WorkflowFile {
    WorkflowFile::new(locate, name, Value::Mapping(Mapping::new())).reload()
}

/// 把一条工作流存成一份可带走的文件（原样，不改内容）。
pub fn export(flow: &WorkflowFile, target: &std::path::Path) -> std::path::PathBuf {
    let target = if target.is_dir() {
        target.join(format!("{}.yaml", flow.name))
    } else {
        target.to_path_buf()
    };
    if let Some(parent) = target.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&target, flow.to_yaml());
    target
}

/// 把一份工作流导进来：先照 schema 验一遍，再起个名字落进工作流目录。
pub fn import(
    locate: &Locate,
    source: &std::path::Path,
    name: &str,
) -> Result<WorkflowFile, WorkflowError> {
    let mut payload = load(source)?;
    let chosen = if !name.trim().is_empty() {
        name.trim().to_string()
    } else {
        let from_payload = text_of(&payload, "name");
        if from_payload.is_empty() {
            source
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default()
        } else {
            from_payload
        }
    };
    let flow = WorkflowFile::new(locate, &chosen, payload.clone());
    if flow.exists() {
        return Err(WorkflowError(format!(
            "已经有一条工作流叫「{chosen}」：{}（换名字用 --as）",
            flow.file().display()
        )));
    }
    if let Some(mapping) = payload.as_mapping_mut() {
        mapping.insert(Value::String("name".into()), Value::String(chosen.clone()));
    }
    let flow = WorkflowFile::new(locate, &chosen, payload);
    write(&flow).map_err(WorkflowError)?;
    Ok(flow)
}

pub fn listing(locate: &Locate) -> Vec<WorkflowFile> {
    let base = locate.workflows_dir();
    if !base.is_dir() {
        return Vec::new();
    }
    let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir(&base)
        .map(|entries| entries.flatten().map(|e| e.path()).collect())
        .unwrap_or_default();
    paths.retain(|p| p.extension().map(|e| e == "yaml").unwrap_or(false));
    paths.sort();
    paths
        .into_iter()
        .filter_map(|path| path.file_stem().map(|s| s.to_string_lossy().to_string()))
        .map(|name| open(locate, &name))
        .collect()
}
