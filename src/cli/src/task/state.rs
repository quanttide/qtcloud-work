//! 任务聚合 / 状态推导：走过了哪些步骤、下一步是哪一步。

use super::Task;
use super::journal::text_of;
use crate::workflow::Step;
use serde_yaml::{Mapping, Value};
use std::path::{Path, PathBuf};

/// 哪些步骤走过了：算法在工具箱的工作区聚合里——附加判定投票、重新执行从头算。
pub fn done(task: &Task) -> Vec<String> {
    task.workspace().done_steps(&task.shared())
}

pub fn next_step(task: &Task) -> Option<Step> {
    let next = task.workspace().next_step(&task.shared())?;
    task.steps().into_iter().find(|step| step.name() == next)
}

pub fn state_line(task: &Task) -> String {
    task.workspace().state_line(&task.shared())
}

/// 起一件任务：写下指令（跑哪条工作流、要什么），备好产物三家。
pub fn create(
    root: &Path,
    data: &Path,
    name: &str,
    workflow_name: &str,
    workflows: Option<&Path>,
) -> Task {
    let task = Task {
        root: root.to_path_buf(),
        data: data.to_path_buf(),
        name: name.to_string(),
        workflows: workflows.map(|p| p.to_path_buf()),
    };
    if let Some(parent) = task.file().parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if !task.file().is_file() {
        let mut payload = Mapping::new();
        payload.insert(
            Value::String("name".into()),
            Value::String(name.to_string()),
        );
        payload.insert(
            Value::String("start".into()),
            Value::String(crate::task::journal::now()),
        );
        payload.insert(
            Value::String("workflow".into()),
            Value::String(workflow_name.to_string()),
        );
        payload.insert("log".into(), Value::Sequence(Vec::new()));
        payload.insert("gates".into(), Value::Sequence(Vec::new()));
        payload.insert("artifacts".into(), Value::Mapping(Mapping::new()));
        for (key, value) in context(root, data, workflows) {
            payload.insert(Value::String(key), Value::String(value));
        }
        payload.insert(Value::String("log".into()), Value::Sequence(Vec::new()));
        let _ = std::fs::write(
            task.file(),
            serde_yaml::to_string(&Value::Mapping(payload)).unwrap_or_default(),
        );
    }
    for category in [crate::task::journal::REPORT, crate::task::journal::JOURNAL] {
        let declared = task
            .artifacts()
            .get(category)
            .is_some_and(|v| !v.trim().is_empty());
        if declared && !task.artifact(category).is_file() {
            if let Some(parent) = task.artifact(category).parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(task.artifact(category), format!("# {category}：{name}\n"));
        }
    }
    task
}

/// 这次执行自带的运行上下文：工作区按绝对记，草稿仓与工作流目录能相对就相对。
pub fn context(root: &Path, data: &Path, workflows: Option<&Path>) -> Vec<(String, String)> {
    fn as_written(root: &Path, path: Option<&Path>) -> String {
        match path {
            None => String::new(),
            Some(path) => match path.strip_prefix(root) {
                Ok(rest) => rest.to_string_lossy().to_string(),
                Err(_) => path.to_string_lossy().to_string(),
            },
        }
    }
    vec![
        ("root".to_string(), root.to_string_lossy().to_string()),
        ("data".to_string(), as_written(root, Some(data))),
        ("workflows".to_string(), as_written(root, workflows)),
    ]
}

/// 开一件任务：命令行给了就用命令行的，没给就用任务里记的。
pub fn reopen(data: &Path, name: &str, root: Option<&Path>, workflows: Option<&Path>) -> Task {
    let raw = Task {
        root: PathBuf::from("."),
        data: data.to_path_buf(),
        name: name.to_string(),
        workflows: None,
    }
    .payload();
    let recorded_root = text_of(&raw, "root");
    let resolved_root = match root {
        Some(root) => root.to_path_buf(),
        None => {
            if recorded_root.is_empty() {
                crate::artifact::repo_root().unwrap_or_else(|_| PathBuf::from("."))
            } else {
                PathBuf::from(shellexpand_home(&recorded_root))
            }
        }
    };
    let resolved_flows = match workflows {
        Some(path) => Some(path.to_path_buf()),
        None => {
            let recorded = text_of(&raw, "workflows");
            if recorded.is_empty() {
                None
            } else {
                let candidate = PathBuf::from(&recorded);
                Some(if candidate.is_absolute() {
                    candidate
                } else {
                    resolved_root.join(candidate)
                })
            }
        }
    };
    Task {
        root: resolved_root,
        data: data.to_path_buf(),
        name: name.to_string(),
        workflows: resolved_flows,
    }
}

fn shellexpand_home(value: &str) -> String {
    if let Some(rest) = value.strip_prefix("~/")
        && let Ok(home) = std::env::var("HOME")
    {
        return format!("{home}/{rest}");
    }
    value.to_string()
}

pub fn listing(root: Option<&Path>, data: &Path, workflows: Option<&Path>) -> Vec<Task> {
    let base = data.join("tasks");
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
        .map(|name| reopen(data, &name, root, workflows))
        .collect()
}
