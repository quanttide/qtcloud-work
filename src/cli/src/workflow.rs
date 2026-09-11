//! 工作流：串联的工作步骤——过程的编排定义，用 YAML 存。
//!
//! 定义要有**固定的意义**，所以是 YAML 而不是散文：字段名、字段取值、判据种类都由 schema
//! 定死，不认识的字段直接报错。
//!
//! 定义这一类**领域模型**（字段表、校验、步骤与判据的视图、定义核对）都在工具箱
//! `quanttide-work` 里——两侧共用一份规矩。这一层只剩命令行自己的两件事：
//! 文件读写（`<工作流目录>/<名字>.yaml`）与把动作写成信封。

use quanttide_work::definition::{self as shared, Finding, Workflow as SharedWorkflow};
use serde_yaml::{Mapping, Value};
use std::fmt;
use std::path::{Path, PathBuf};

// 字段表与取值：一处定义、两侧共用（工具箱 `quanttide-work`）。
pub use quanttide_work::definition::{AGENT, HUMAN, RULE, Step};

/// 这份文件不像一份工作流。
#[derive(Debug)]
pub struct WorkflowError(pub String);

impl fmt::Display for WorkflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for WorkflowError {}

fn dump(value: &Value) -> String {
    serde_yaml::to_string(value).unwrap_or_default()
}

/// 读一份定义：不是映射、缺字段、取值不对，当场报错。
///
/// 校验的规矩在工具箱里（`quanttide_work::definition::validate`）——两侧共用一份，
/// 报错文字也一字不差。
pub fn load(path: &Path) -> std::result::Result<Value, WorkflowError> {
    let file = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let text =
        std::fs::read_to_string(path).map_err(|e| WorkflowError(format!("{file} 读不了：{e}")))?;
    let payload: Value = serde_yaml::from_str(&text)
        .map_err(|e| WorkflowError(format!("{file} 不是合法的 YAML：{e}")))?;
    if let Err(error) = shared::validate(&payload, &file) {
        return Err(WorkflowError(error.0));
    }
    Ok(payload)
}

pub fn text_of(value: &Value, key: &str) -> String {
    shared::text_of(value, key)
}

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

    /// 内容那一层交给工具箱。
    pub fn shared(&self) -> SharedWorkflow {
        SharedWorkflow::new(&self.name, self.payload.clone())
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
        dump(&self.payload)
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

// ---- 动作 ----

use crate::outcome::{Result, short};

pub fn workflow_new(
    data: &Path,
    name: &str,
    steps: &[String],
    note: &str,
    workflows: Option<&Path>,
) -> Result {
    if name.trim().is_empty() {
        return Result::lines(false, vec!["请先给工作流起个名字".to_string()]);
    }
    if steps.is_empty() {
        return Result::lines(false, vec!["至少给一个步骤：--steps 甲,乙,丙".to_string()]);
    }
    let flow = create(data, name.trim(), steps, note, workflows);
    workflow_show(data, name.trim(), workflows)
        .with_first(format!("写下工作流：{}", short(data, &flow.file())))
}

pub fn workflow_show(data: &Path, name: &str, workflows: Option<&Path>) -> Result {
    let flow = open_workflow(data, name, workflows);
    if !flow.exists() {
        return Result::lines(
            false,
            vec![format!("没有这条工作流：{}", short(data, &flow.file()))],
        );
    }
    let mut result = Result::new(true);
    result.columns = vec![
        "步骤".to_string(),
        "谁执行".to_string(),
        "怎么算完".to_string(),
    ];
    result.lines.push(format!(
        "工作流：{}（{}）",
        flow.name,
        short(data, &flow.file())
    ));
    for step in flow.steps() {
        let counts = format!(
            "{} rule / {} agent / {} human",
            step.rules().len(),
            step.agents().len(),
            step.gates().len()
        );
        result
            .rows
            .push(vec![step.name(), step.executor(), counts.clone()]);
        result
            .lines
            .push(format!("  {}：{}　{counts}", step.name(), step.executor()));
    }
    result
}

pub fn workflow_export(data: &Path, name: &str, target: &Path, workflows: Option<&Path>) -> Result {
    let flow = open_workflow(data, name, workflows);
    if !flow.exists() {
        return Result::lines(
            false,
            vec![format!("没有这条工作流：{}", short(data, &flow.file()))],
        );
    }
    let saved = export(&flow, target);
    workflow_show(data, name, workflows).with_first(format!(
        "已导出：{}（步骤 {} 个，原样带走）",
        saved.display(),
        flow.steps().len()
    ))
}

pub fn workflow_import(data: &Path, source: &Path, name: &str, workflows: Option<&Path>) -> Result {
    if !source.is_file() {
        return Result::lines(false, vec![format!("没有这份文件：{}", source.display())]);
    }
    match import_workflow(data, source, name, workflows) {
        Err(error) => Result::lines(false, vec![error.0]),
        Ok(flow) => workflow_show(data, &flow.name, workflows).with_first(format!(
            "已导入：{}（步骤 {} 个）",
            short(data, &flow.file()),
            flow.steps().len()
        )),
    }
}

pub fn workflow_list(data: &Path, workflows: Option<&Path>) -> Result {
    let found = listing(data, workflows);
    let mut result = Result::new(true);
    result.columns = vec!["工作流".to_string(), "步骤".to_string(), "位置".to_string()];
    for flow in &found {
        let steps = flow
            .steps()
            .into_iter()
            .map(|s| s.name())
            .collect::<Vec<_>>()
            .join("、");
        result.rows.push(vec![
            flow.name.clone(),
            steps.clone(),
            short(data, &flow.file()),
        ]);
        result.lines.push(format!("{:24} 步骤：{steps}", flow.name));
    }
    if found.is_empty() {
        result.lines =
            vec!["还没有工作流：qtcloud-work workflow --new <名字> --steps 甲,乙".to_string()];
    }
    result
}

// ---- 定义核对：声明与判据对不对得上 ----

/// 核对一条工作流：判据里的路径在不在；描述里提到的报告小节有没有判据覆盖。
///
/// 规矩在工具箱里；这里只把「路径在不在」用工作区根包一层。
pub fn check(flow: &WorkflowFile, root: &Path, data: &Path) -> Vec<Finding> {
    shared::check(&flow.shared(), &data.to_string_lossy(), |written| {
        let path = Path::new(written);
        let target = if path.is_absolute() {
            path.to_path_buf()
        } else {
            root.join(path)
        };
        target.exists()
    })
}

/// 核对结果写成人读的一段。
pub fn describe(found: &[Finding]) -> Vec<String> {
    shared::describe(found)
}

pub fn all_ok(found: &[Finding]) -> bool {
    shared::all_ok(found)
}

/// 核对一条工作流的声明与判据对不对得上，结果印给人。
pub fn workflow_check(data: &Path, name: &str, root: &Path, workflows: Option<&Path>) -> Result {
    let flow = open_workflow(data, name, workflows);
    if !flow.exists() {
        return Result::lines(
            false,
            vec![format!("没有这条工作流：{}", short(data, &flow.file()))],
        );
    }
    let found = check(&flow, root, data);
    let ok = all_ok(&found);
    let mut result = Result::new(ok);
    result.lines = vec![format!(
        "工作流：{}",
        flow.file()
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
    )];
    result.lines.extend(describe(&found));
    result
}
