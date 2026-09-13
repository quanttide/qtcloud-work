//! 任务聚合：工作流的一次执行实例。
//!
//! `<数据仓>/tasks/<任务>.yaml` 是一次执行：跑哪条工作流 + 自带的运行上下文
//! （`root` / `data` / `workflows`）+ 流水；产物落在 `artifacts/` 下。
//!
//! 这一件装任务本身：类型、文件读写、上下文与流水。子件按事分：
//! 状态推导在 `state`、走一步在 `execute`、交给 AI 的两段话在 `ai`、
//! 动作在 `report`、日志在 `journal`。

pub mod ai;
pub mod execute;
pub mod journal;
pub mod progress;
pub mod report;
pub mod state;

pub use report::{task_journal, task_list, task_new, task_status, task_step};

use crate::workflow::{self, Step, WorkflowFile};
use quanttide_work::artifact::Artifact;
use quanttide_work::workspace::Workspace;
use serde_yaml::{Mapping, Value};
use std::path::{Path, PathBuf};

/// 一次执行：跑某条工作流，有自己的流水与产物。
pub struct Task {
    pub root: PathBuf,
    pub data: PathBuf,
    pub name: String,
    pub workflows: Option<PathBuf>,
}

impl Task {
    pub fn file(&self) -> PathBuf {
        self.data.join("tasks").join(format!("{}.yaml", self.name))
    }

    pub fn artifacts_dir(&self) -> PathBuf {
        self.data.join("artifacts")
    }

    /// 这次执行往哪写产物（任务是运行数据，产物与它没有从属关系）。
    pub fn artifacts(&self) -> std::collections::BTreeMap<String, String> {
        let mut found = std::collections::BTreeMap::new();
        if let Some(mapping) = self.payload().get("artifacts").and_then(|v| v.as_mapping()) {
            for (key, value) in mapping {
                if let (Some(key), Some(value)) = (key.as_str(), value.as_str()) {
                    found.insert(key.to_string(), value.to_string());
                }
            }
        }
        found
    }

    /// 闸门项：等人拍板的事项，记在任务文件里。
    pub fn gates(&self) -> Vec<String> {
        self.payload()
            .get("gates")
            .and_then(|v| v.as_sequence())
            .map(|items| {
                items
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn set_gates(&self, notes: &[String]) {
        let mut payload = self.payload();
        if let Value::Mapping(ref mut mapping) = payload {
            mapping.insert(
                Value::String("gates".into()),
                Value::Sequence(notes.iter().map(|n| Value::String(n.clone())).collect()),
            );
        }
        let text = serde_yaml::to_string(&payload).unwrap_or_default();
        let _ = std::fs::write(self.file(), text);
    }

    /// 这次执行往哪写这种产物——落点按名字在工具箱里算（规范「任务 / 语法」·落点）。
    ///
    /// 工具箱给的是**相对工作区根的路径**；接上哪一处目录是命令行的事：
    /// 任务里声明过的按工作区根接（能指到正式仓），没声明的按数据仓接（草稿区）。
    pub fn artifact(&self, kind: &str) -> PathBuf {
        // 流水不是产物——它就是任务文件（规范 `piece/artifact.md`）
        if kind == crate::task::journal::LOG {
            return self.file();
        }
        let task = self.shared();
        // 落点只按名字算，不看工作区里装了什么——借一个空的把规矩走工具箱那一份。
        let place = Workspace::default().place(&task, &Artifact::named(kind));
        let base = if task.declared(kind).is_some() {
            &self.root
        } else {
            &self.data
        };
        base.join(place)
    }

    /// 这次工作的边界：把这条任务与它跑的定义装在一起（工具箱按名字取工作流算流水）。
    pub fn workspace(&self) -> Workspace {
        Workspace::of(vec![self.workflow().shared()], vec![self.shared()])
    }

    pub fn exists(&self) -> bool {
        self.file().is_file()
    }

    /// 任务自己的属性，不是流水里的一步。
    pub fn start(&self) -> String {
        crate::task::journal::text_of(&self.payload(), "start")
    }

    pub fn payload(&self) -> Value {
        if !self.file().is_file() {
            return Value::Mapping(Mapping::new());
        }
        match std::fs::read_to_string(self.file())
            .ok()
            .and_then(|t| serde_yaml::from_str::<Value>(&t).ok())
        {
            Some(value @ Value::Mapping(_)) => value,
            _ => Value::Mapping(Mapping::new()),
        }
    }

    pub fn workflow_name(&self) -> String {
        crate::task::journal::text_of(&self.payload(), "workflow")
    }

    pub fn workflow(&self) -> WorkflowFile {
        workflow::open_workflow(&self.data, &self.workflow_name(), self.workflows.as_deref())
    }

    pub fn steps(&self) -> Vec<Step> {
        self.workflow().steps()
    }

    /// 流水：任务文件里的 `log` 一节，一条一条按发生顺序。
    pub fn events(&self) -> Vec<Value> {
        self.payload()
            .get("log")
            .and_then(|v| v.as_sequence())
            .map(|items| items.iter().filter(|e| e.is_mapping()).cloned().collect())
            .unwrap_or_default()
    }

    /// 这件任务在工具箱里的样子（内容那一层交给工具箱）。
    pub fn shared(&self) -> quanttide_work::task::Task {
        quanttide_work::task::Task::of(&self.payload())
    }

    /// 记一笔流水：读任务文件、追加一条、写回去（流水只增不改）。
    pub fn record(&self, step: &str, detail: &str, ok: bool) {
        let mut payload = self.payload();
        let mapping = payload.as_mapping_mut().expect("任务顶层是映射");
        if !mapping.contains_key("log") {
            mapping.insert(Value::String("log".into()), Value::Sequence(Vec::new()));
        }
        if let Some(Value::Sequence(seq)) = mapping.get_mut("log") {
            let mut event = Mapping::new();
            event.insert(
                Value::String("at".into()),
                Value::String(crate::task::journal::now()),
            );
            event.insert(
                Value::String("step".into()),
                Value::String(step.to_string()),
            );
            event.insert(
                Value::String("detail".into()),
                Value::String(detail.to_string()),
            );
            event.insert(Value::String("ok".into()), Value::Bool(ok));
            seq.push(Value::Mapping(event));
        }
        if let Some(parent) = self.file().parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(
            self.file(),
            serde_yaml::to_string(&payload).unwrap_or_default(),
        );
    }

    pub fn relative(&self, path: &Path) -> String {
        match path.strip_prefix(&self.data) {
            Ok(rest) => rest.to_string_lossy().to_string(),
            Err(_) => path.to_string_lossy().to_string(),
        }
    }
}
