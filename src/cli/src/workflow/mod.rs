//! 工作流聚合：串联的工作步骤——过程的编排定义，用 YAML 存。
//!
//! 定义要有**固定的意义**，所以是 YAML 而不是散文：字段名、字段取值、判据种类都由 schema
//! 定死，不认识的字段直接报错。
//!
//! 定义这一类**领域模型**（字段表、校验、步骤与判据的视图、定义核对）都在工具箱
//! `quanttide-work` 里——两侧共用一份规矩。这一层只剩命令行自己的两件事：
//! 文件读写（`<工作流目录>/<名字>.yaml`）与把动作写成信封。
//!
//! 子件按事分：YAML 读写与 schema 校验在 `yaml`、定义核对在 `check`、五个动作在 `actions`。

mod actions;
mod check;
mod yaml;

pub use actions::{
    workflow_check, workflow_export, workflow_import, workflow_list, workflow_new, workflow_show,
};
pub use check::{all_ok, check, describe};
pub use yaml::{WorkflowError, load, text_of};

// 字段表与取值：一处定义、两侧共用（工具箱 `quanttide-work`）。
pub use quanttide_work::executor::{AGENT, HUMAN, RULE};
pub use quanttide_work::workflow::Step;

use quanttide_work::workflow::Workflow as SharedWorkflow;
use serde_yaml::{Mapping, Value};
use std::path::{Path, PathBuf};

/// 工作流目录：默认跟在数据仓里，可另指一处固定资产目录。
pub fn workflows_dir(data: &Path, workflows: Option<&Path>) -> PathBuf {
    match workflows {
        Some(path) => path.to_path_buf(),
        None => data.join("workflows"),
    }
}

/// 一条定义**连同它的文件位置**（工具箱那份只管内容，不管文件）。
pub struct WorkflowFile {
    pub name: String,
    pub payload: Value,
    pub workflows: PathBuf,
}

impl WorkflowFile {
    pub fn new(data: &Path, name: &str, payload: Value, workflows: Option<&Path>) -> Self {
        let data = data.to_path_buf();
        let workflows = workflows_dir(&data, workflows);
        WorkflowFile {
            name: name.to_string(),
            payload,
            workflows,
        }
    }

    pub fn file(&self) -> PathBuf {
        self.workflows.join(format!("{}.yaml", self.name))
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

    /// 内容那一层交给工具箱（工作流名取自定义里的 `name` 字段）。
    pub fn shared(&self) -> SharedWorkflow {
        SharedWorkflow::of(&self.payload)
    }

    pub fn description(&self) -> String {
        self.shared().description()
    }

    /// 步骤：按定义里的顺序——这就是「串联」。
    pub fn steps(&self) -> Vec<Step> {
        self.shared().steps()
    }

    pub fn step(&self, name: &str) -> Option<Step> {
        self.shared().step(name)
    }

    pub fn to_yaml(&self) -> String {
        yaml::dump(&self.payload)
    }
}

/// 写一条工作流：步骤串联，每步给一份判据骨架（执行者默认 AI）。
pub fn create(
    data: &Path,
    name: &str,
    steps: &[String],
    note: &str,
    workflows: Option<&Path>,
) -> WorkflowFile {
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
    let flow = WorkflowFile::new(data, name, Value::Mapping(payload), workflows);
    if let Some(parent) = flow.file().parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(flow.file(), flow.to_yaml());
    flow
}

pub fn open_workflow(data: &Path, name: &str, workflows: Option<&Path>) -> WorkflowFile {
    WorkflowFile::new(data, name, Value::Mapping(Mapping::new()), workflows).reload()
}

/// 把一条工作流存成一份可带走的文件（原样，不改内容）。
pub fn export(flow: &WorkflowFile, target: &Path) -> PathBuf {
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

/// 把一份工作流导进来：先照 schema 验一遍，再起个名字落进 workflows/。
pub fn import_workflow(
    data: &Path,
    source: &Path,
    name: &str,
    workflows: Option<&Path>,
) -> std::result::Result<WorkflowFile, WorkflowError> {
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
    let flow = WorkflowFile::new(data, &chosen, payload.clone(), workflows);
    if flow.exists() {
        return Err(WorkflowError(format!(
            "已经有一条工作流叫「{chosen}」：{}（换名字用 --as）",
            flow.file().display()
        )));
    }
    if let Some(mapping) = payload.as_mapping_mut() {
        mapping.insert(Value::String("name".into()), Value::String(chosen.clone()));
    }
    let flow = WorkflowFile::new(data, &chosen, payload, workflows);
    if let Some(parent) = flow.file().parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(flow.file(), flow.to_yaml());
    Ok(flow)
}

pub fn listing(data: &Path, workflows: Option<&Path>) -> Vec<WorkflowFile> {
    let base = workflows_dir(data, workflows);
    if !base.is_dir() {
        return Vec::new();
    }
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&base)
        .map(|entries| entries.flatten().map(|e| e.path()).collect())
        .unwrap_or_default();
    paths.retain(|p| p.extension().map(|e| e == "yaml").unwrap_or(false));
    paths.sort();
    paths
        .into_iter()
        .filter_map(|path| path.file_stem().map(|s| s.to_string_lossy().to_string()))
        .map(|name| open_workflow(data, &name, workflows))
        .collect()
}
