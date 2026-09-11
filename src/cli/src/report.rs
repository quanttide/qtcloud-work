//! 动作的结果：命令行与图形界面共用的一层。
//!
//! 每个动作返回一个 `Result`——`ok` 通不通、`lines` 命令行要打印的话、
//! `columns` / `rows` 界面要画的同一份表格。界面只管画，命令行只管印，算法只写一遍。

use crate::assets;
use crate::catalog;
use crate::material;
use crate::task::{self, Task};
use crate::workflow;
use serde_json::{Value as Json, json};
use std::path::Path;

#[derive(Default)]
pub struct Result {
    pub ok: bool,
    pub lines: Vec<String>,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub payload: Option<Json>,
}

impl Result {
    pub fn new(ok: bool) -> Self {
        Result {
            ok,
            ..Default::default()
        }
    }

    pub fn lines(ok: bool, lines: Vec<String>) -> Self {
        Result {
            ok,
            lines,
            ..Default::default()
        }
    }

    pub fn with_first(mut self, line: String) -> Self {
        self.lines.insert(0, line);
        self
    }

    pub fn to_json(&self) -> Json {
        if let Some(payload) = &self.payload {
            return payload.clone();
        }
        json!({
            "ok": self.ok,
            "lines": self.lines,
            "columns": self.columns,
            "rows": self.rows,
        })
    }
}

fn short(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(rest) => rest.to_string_lossy().to_string(),
        Err(_) => path.to_string_lossy().to_string(),
    }
}

// ---- 工作区 ----

pub fn catalog(root: &Path) -> Result {
    let found = catalog::build(root);
    let mut result = Result {
        ok: true,
        ..Default::default()
    };
    result.columns = vec!["种类".to_string(), "路径".to_string()];
    for entry in &found.entries {
        let rel = short(root, &entry.path);
        result.lines.push(format!("[{}] {rel}", entry.kind));
        result.rows.push(vec![entry.kind.clone(), rel]);
    }
    result.payload = Some(catalog::payload(root, &found));
    result
}

pub fn audit(root: &Path, make: bool) -> Result {
    let made = if make {
        assets::make(root, None)
    } else {
        Vec::new()
    };
    let missing = assets::missing(root);
    let unregistered = catalog::build(root).unregistered(root);
    let ok = missing.is_empty() && unregistered.is_empty();
    let mut result = Result {
        ok,
        ..Default::default()
    };
    result.columns = vec!["问题".to_string(), "说明".to_string()];
    for asset in &missing {
        result.rows.push(vec![
            "缺资产".to_string(),
            format!("{}（{}）", asset.kind, asset.name),
        ]);
    }
    for path in &unregistered {
        result
            .rows
            .push(vec!["未登记".to_string(), short(root, path)]);
    }
    result.lines = made
        .iter()
        .map(|path| format!("补建：{}", short(root, path)))
        .collect();
    result.lines.extend(
        result
            .rows
            .iter()
            .map(|row| format!("{}：{}", row[0], row[1])),
    );
    if ok {
        result
            .lines
            .push("审计通过：二十格齐备，无未登记目录。".to_string());
    } else if !unregistered.is_empty() && missing.is_empty() {
        result
            .lines
            .push("未登记的目录要么属于某一格（改资产表），要么不该在这儿。".to_string());
    }
    result.payload = Some(json!({
        "root": root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
        "result": if ok { "通过" } else { "有问题" },
        "missing": missing.iter().map(|a| json!({"kind": a.kind, "name": a.name})).collect::<Vec<_>>(),
        "unregistered": unregistered.iter().map(|p| short(root, p)).collect::<Vec<_>>(),
    }));
    result
}

pub fn find(root: &Path, name: &str, show: bool) -> Result {
    if name.trim().is_empty() {
        return Result::lines(false, vec!["请填要找的名字".to_string()]);
    }
    let matches = catalog::build(root).find(name);
    if matches.is_empty() {
        return Result::lines(false, vec![format!("未找到：{name}")]);
    }
    let mut result = Result::new(true);
    for entry in matches {
        let rel = short(root, &entry.path);
        result.lines.push(format!("[{}] {rel}", entry.kind));
        result.rows.push(vec![entry.kind.clone(), rel]);
        if show {
            if entry.path.is_dir() {
                let listed: Vec<String> = std::fs::read_dir(&entry.path)
                    .map(|entries| {
                        entries
                            .flatten()
                            .map(|e| e.file_name().to_string_lossy().to_string())
                            .filter(|n| !n.starts_with('.'))
                            .collect()
                    })
                    .unwrap_or_default();
                result
                    .lines
                    .push(format!("  （目录）{}", listed.join("、")));
            } else {
                result.lines.push(
                    std::fs::read_to_string(&entry.path)
                        .unwrap_or_default()
                        .trim_end()
                        .to_string(),
                );
            }
        }
    }
    result.columns = vec!["种类".to_string(), "路径".to_string()];
    result
}

pub fn material(root: &Path, paths: Option<&[String]>) -> Result {
    let found = material::materials(root, paths);
    let mut result = Result::new(true);
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
    result.payload = Some(json!({
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
    let flow = workflow::create(data, name.trim(), steps, note, workflows);
    workflow_show(data, name.trim(), workflows)
        .with_first(format!("写下工作流：{}", short(data, &flow.file())))
}

pub fn workflow_show(data: &Path, name: &str, workflows: Option<&Path>) -> Result {
    let flow = workflow::open_workflow(data, name, workflows);
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
    let flow = workflow::open_workflow(data, name, workflows);
    if !flow.exists() {
        return Result::lines(
            false,
            vec![format!("没有这条工作流：{}", short(data, &flow.file()))],
        );
    }
    let saved = workflow::export(&flow, target);
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
    match workflow::import_workflow(data, source, name, workflows) {
        Err(error) => Result::lines(false, vec![error.0]),
        Ok(flow) => workflow_show(data, &flow.name, workflows).with_first(format!(
            "已导入：{}（步骤 {} 个）",
            short(data, &flow.file()),
            flow.steps().len()
        )),
    }
}

pub fn workflow_list(data: &Path, workflows: Option<&Path>) -> Result {
    let found = workflow::listing(data, workflows);
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

// ---- 任务 ----

pub fn task_new(
    root: &Path,
    data: &Path,
    name: &str,
    workflow_name: &str,
    workflows: Option<&Path>,
) -> Result {
    if name.trim().is_empty() {
        return Result::lines(false, vec!["请先给这件任务起个名字".to_string()]);
    }
    let flow = workflow::open_workflow(data, workflow_name, workflows);
    if !flow.exists() {
        return Result::lines(
            false,
            vec![format!(
                "没有这条工作流：{}（qtcloud-work workflow --list 看有哪些）",
                short(data, &flow.file())
            )],
        );
    }
    let existing = task::reopen(data, name.trim(), Some(root), workflows);
    if existing.exists() {
        return Result::lines(
            false,
            vec![format!(
                "已经有这件任务：{}（换个名字，不覆盖）",
                short(data, &existing.file())
            )],
        );
    }
    let task = task::create(root, data, name.trim(), workflow_name.trim(), workflows);
    task_status(Some(root), data, name.trim(), workflows)
        .with_first(format!("起了：{}", short(data, &task.file())))
}

pub fn task_status(
    root: Option<&Path>,
    data: &Path,
    name: &str,
    workflows: Option<&Path>,
) -> Result {
    if name.trim().is_empty() {
        return Result::lines(
            false,
            vec!["请先选一件任务（qtcloud-work task --list 看有哪些）".to_string()],
        );
    }
    let task = task::reopen(data, name, root, workflows);
    if !task.exists() {
        return Result::lines(
            false,
            vec![format!("没有这件任务：{}", short(data, &task.file()))],
        );
    }
    let done = task.done();
    let mut result = Result::new(true);
    result.columns = vec!["步骤".to_string(), "状态".to_string()];
    result.lines.push(format!("任务：{}", task.name));
    result.lines.push(format!(
        "  开工：{}",
        if task.start().is_empty() {
            "（没记）".to_string()
        } else {
            task.start()
        }
    ));
    result.lines.push(format!(
        "  工作流：{}——{}",
        task.workflow_name(),
        task.workflow().description()
    ));
    result
        .lines
        .push(format!("  步骤：{} 个", task.steps().len()));
    for step in task.steps() {
        let state = if done.contains(&step.name()) {
            "✓"
        } else {
            "—"
        };
        result.rows.push(vec![step.name(), state.to_string()]);
        result.lines.push(format!("  {state} {}", step.name()));
    }
    result.lines.push(task::state_line(&task));
    result
        .lines
        .push(format!("指令：{}", short(data, &task.file())));
    result.lines.push(format!(
        "产物：{}、{}　流水：{}",
        short(data, &task.artifact(task::REPORT)),
        short(data, &task.artifact(task::JOURNAL)),
        short(data, &task.artifact(task::LOG))
    ));
    let events = task.events();
    if !events.is_empty() {
        result.lines.push("流水（最近五条）：".to_string());
        for event in events
            .iter()
            .rev()
            .take(5)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            let at = event.get("at").and_then(|v| v.as_str()).unwrap_or("");
            let step = event.get("step").and_then(|v| v.as_str()).unwrap_or("");
            let detail = event.get("detail").and_then(|v| v.as_str()).unwrap_or("");
            result.lines.push(format!("  {at}　{step}　{detail}"));
        }
    }
    result
}

pub fn task_list(root: Option<&Path>, data: &Path, workflows: Option<&Path>) -> Result {
    let found = task::listing(root, data, workflows);
    let mut result = Result::new(true);
    result.columns = vec![
        "任务".to_string(),
        "工作流".to_string(),
        "下一步".to_string(),
    ];
    for task in &found {
        let next = task
            .next_step()
            .map(|s| s.name())
            .unwrap_or_else(|| "走完".to_string());
        result
            .rows
            .push(vec![task.name.clone(), task.workflow_name(), next.clone()]);
        result.lines.push(format!(
            "{:24} 工作流 {}　下一步：{next}",
            task.name,
            task.workflow_name()
        ));
    }
    if found.is_empty() {
        result.lines =
            vec!["还没有任务：qtcloud-work task --new <名字> --workflow <工作流>".to_string()];
    }
    result
}

/// 走一步：能让 AI 跑的交给 AI（auto），然后跑判据、记账。
pub fn task_step(
    root: Option<&Path>,
    data: &Path,
    name: &str,
    step: &str,
    note: &str,
    auto: bool,
    workflows: Option<&Path>,
) -> Result {
    let task: Task = task::reopen(data, name, root, workflows);
    if !task.exists() {
        return Result::lines(
            false,
            vec![format!("没有这件任务：{}", short(data, &task.file()))],
        );
    }
    let mut chosen = step.trim().to_string();
    if chosen.is_empty() {
        if !auto {
            return Result::lines(
                false,
                vec!["请给步骤名（qtcloud-work task <名字> 看有哪些步骤）".to_string()],
            );
        }
        match task.next_step() {
            None => return Result::lines(true, vec!["所有步骤都走过了".to_string()]),
            Some(next) => chosen = next.name(),
        }
    }
    let (ok, lines, rows) = task::execute(&task, &task.root, &chosen, note, auto);
    let mut result = Result {
        ok,
        lines,
        ..Default::default()
    };
    result.columns = vec!["核对".to_string(), "结论".to_string(), "说明".to_string()];
    result.rows = rows.into_iter().map(|(a, b, c)| vec![a, b, c]).collect();
    result.lines.push(task::state_line(&task));
    result
}

pub fn task_journal(
    root: Option<&Path>,
    data: &Path,
    name: &str,
    words: &str,
    workflows: Option<&Path>,
) -> Result {
    let task = task::reopen(data, name, root, workflows);
    if !task.exists() {
        return Result::lines(
            false,
            vec![format!("没有这件任务：{}", short(data, &task.file()))],
        );
    }
    if words.trim().is_empty() {
        return Result::lines(
            false,
            vec![format!(
                "日志要人来写：{}",
                short(data, &task.artifact(task::JOURNAL))
            )],
        );
    }
    task::narrate(&task, words);
    Result::lines(
        true,
        vec![
            format!(
                "日志记下一段：{}",
                short(data, &task.artifact(task::JOURNAL))
            ),
            task::state_line(&task),
        ],
    )
}
