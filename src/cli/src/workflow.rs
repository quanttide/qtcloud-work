//! 工作流：串联的工作步骤——过程的编排定义，用 YAML 存。
//!
//! 定义要有**固定的意义**，所以是 YAML 而不是散文：字段名、字段取值、判据种类都由 schema
//! 定死，不认识的字段直接报错。
//!
//! `<工作流目录>/<名字>.yaml`（默认 `<数据仓>/workflows/`，可用 `--workflows` 另指）。

use serde_yaml::{Mapping, Value};
use std::fmt;
use std::path::{Path, PathBuf};

pub const AGENT: &str = "agent";
pub const HUMAN: &str = "human";
pub const RULE: &str = "rule";
pub const EXECUTORS: [&str; 2] = [AGENT, HUMAN];
pub const TYPES: [&str; 3] = [RULE, AGENT, HUMAN];
pub const TOP_FIELDS: [&str; 3] = ["name", "description", "steps"];
pub const STEP_FIELDS: [&str; 4] = ["name", "description", "executor", "criteria"];
pub const CRITERION_FIELDS: [&str; 7] = [
    "executor",
    "description",
    "path",
    "absent",
    "file",
    "contains",
    "run",
];

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

fn unknown_fields(mapping: &Mapping, allowed: &[&str]) -> Vec<String> {
    mapping
        .keys()
        .filter_map(|k| k.as_str())
        .filter(|k| !allowed.contains(k))
        .map(|k| k.to_string())
        .collect()
}

/// 读一份定义：不是映射、缺字段、取值不对，当场报错。
pub fn load(path: &Path) -> std::result::Result<Value, WorkflowError> {
    let file = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let text =
        std::fs::read_to_string(path).map_err(|e| WorkflowError(format!("{file} 读不了：{e}")))?;
    let payload: Value = serde_yaml::from_str(&text)
        .map_err(|e| WorkflowError(format!("{file} 不是合法的 YAML：{e}")))?;
    let top = payload
        .as_mapping()
        .ok_or_else(|| WorkflowError(format!("{file} 的顶层不是映射（name / steps）")))?;
    if text_of(&payload, "name").is_empty() {
        return Err(WorkflowError(format!("{file} 少了 name")));
    }
    let steps = payload
        .get("steps")
        .and_then(|v| v.as_sequence())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| WorkflowError(format!("{file} 少了 steps（至少一个步骤）")))?;
    let unknown = unknown_fields(top, &TOP_FIELDS);
    if !unknown.is_empty() {
        return Err(WorkflowError(format!(
            "{file} 顶层有不认识的字段：{}（只认 {}）",
            unknown.join("、"),
            TOP_FIELDS.join("、")
        )));
    }
    for (index, step) in steps.iter().enumerate() {
        let index = index + 1;
        let step_map = step
            .as_mapping()
            .ok_or_else(|| WorkflowError(format!("{file} 第 {index} 个步骤少了 name")))?;
        if text_of(step, "name").is_empty() {
            return Err(WorkflowError(format!("{file} 第 {index} 个步骤少了 name")));
        }
        let extra = unknown_fields(step_map, &STEP_FIELDS);
        if !extra.is_empty() {
            return Err(WorkflowError(format!(
                "{file} 第 {index} 个步骤有不认识的字段：{}（只认 {}）",
                extra.join("、"),
                STEP_FIELDS.join("、")
            )));
        }
        let executor = text_of(step, "executor");
        let executor = if executor.is_empty() {
            AGENT.to_string()
        } else {
            executor
        };
        if !EXECUTORS.contains(&executor.as_str()) {
            return Err(WorkflowError(format!(
                "{file} 第 {index} 个步骤的 executor 只能是 {}，实得 {executor}",
                EXECUTORS.join(" 或 ")
            )));
        }
        let criteria = match step.get("criteria") {
            None | Some(Value::Null) => &[][..],
            Some(Value::Sequence(items)) => items.as_slice(),
            Some(_) => {
                return Err(WorkflowError(format!(
                    "{file} 第 {index} 个步骤的 criteria 应当是列表"
                )));
            }
        };
        for (order, criterion) in criteria.iter().enumerate() {
            let order = order + 1;
            let where_ = format!("第 {index} 个步骤第 {order} 条判据");
            let kind = text_of(criterion, "executor");
            if !TYPES.contains(&kind.as_str()) {
                return Err(WorkflowError(format!(
                    "{file} {where_}的 executor 只能是 {}（谁判：规则引擎 / 智能体 / 人）",
                    TYPES.join(" / ")
                )));
            }
            let criterion_map = criterion
                .as_mapping()
                .ok_or_else(|| WorkflowError(format!("{file} {where_}不是映射")))?;
            let odd = unknown_fields(criterion_map, &CRITERION_FIELDS);
            if !odd.is_empty() {
                return Err(WorkflowError(format!(
                    "{file} {where_}有不认识的字段：{}（只认 {}）",
                    odd.join("、"),
                    CRITERION_FIELDS.join("、")
                )));
            }
            let given: Vec<&str> = ["path", "absent", "file", "contains", "run"]
                .into_iter()
                .filter(|name| criterion.get(*name).is_some())
                .collect();
            if kind == RULE {
                if given.is_empty() {
                    return Err(WorkflowError(format!(
                        "{file} {where_}是 rule，得写一条判法（path / absent / file+contains / run）"
                    )));
                }
                if given.contains(&"contains") && !given.contains(&"file") {
                    return Err(WorkflowError(format!(
                        "{file} {where_}写了 contains，还得写 file"
                    )));
                }
                if given.contains(&"file") && !given.contains(&"contains") {
                    return Err(WorkflowError(format!(
                        "{file} {where_}写了 file，还得写 contains"
                    )));
                }
                let others: Vec<&str> = given
                    .iter()
                    .copied()
                    .filter(|n| *n != "file" && *n != "contains")
                    .collect();
                if others.len() > 1 || (!others.is_empty() && given.contains(&"file")) {
                    return Err(WorkflowError(format!(
                        "{file} {where_}的判法只能一种：path / absent / file+contains / run"
                    )));
                }
            } else {
                if text_of(criterion, "description").is_empty() {
                    return Err(WorkflowError(format!(
                        "{file} {where_}是 {kind}，必须写 description（判准 / 要人拍板的事）"
                    )));
                }
                if !given.is_empty() {
                    return Err(WorkflowError(format!(
                        "{file} {where_}是 {kind}，不该带 {}（那是 rule 的字段）",
                        given.join("、")
                    )));
                }
            }
        }
    }
    Ok(payload)
}

fn text_of(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string()
}

/// 工作流目录：默认跟在数据仓里，可另指一处固定资产目录。
pub fn workflows_dir(data: &Path, workflows: Option<&Path>) -> PathBuf {
    match workflows {
        Some(path) => path.to_path_buf(),
        None => data.join("workflows"),
    }
}

/// 一个工作步骤：叫什么、做什么、谁执行、怎么算完。
#[derive(Clone)]
pub struct Step {
    payload: Value,
}

impl Step {
    pub fn name(&self) -> String {
        text_of(&self.payload, "name")
    }

    pub fn description(&self) -> String {
        text_of(&self.payload, "description")
    }

    pub fn executor(&self) -> String {
        let value = text_of(&self.payload, "executor");
        if value.is_empty() {
            AGENT.to_string()
        } else {
            value
        }
    }

    pub fn human(&self) -> bool {
        self.executor() == HUMAN
    }

    pub fn criteria(&self) -> Vec<Value> {
        self.payload
            .get("criteria")
            .and_then(|v| v.as_sequence())
            .cloned()
            .unwrap_or_default()
    }

    pub fn of(&self, kind: &str) -> Vec<Value> {
        self.criteria()
            .into_iter()
            .filter(|c| c.get("executor").and_then(|v| v.as_str()) == Some(kind))
            .collect()
    }

    pub fn rules(&self) -> Vec<Value> {
        self.of(RULE)
    }
    pub fn agents(&self) -> Vec<Value> {
        self.of(AGENT)
    }
    pub fn gates(&self) -> Vec<Value> {
        self.of(HUMAN)
    }
}

/// 过程的编排定义：一串步骤。
pub struct Workflow {
    pub name: String,
    pub payload: Value,
    pub workflows: PathBuf,
}

impl Workflow {
    pub fn new(data: &Path, name: &str, payload: Value, workflows: Option<&Path>) -> Self {
        let data = data.to_path_buf();
        let workflows = workflows_dir(&data, workflows);
        Workflow {
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

    pub fn description(&self) -> String {
        text_of(&self.payload, "description")
    }

    /// 步骤：按定义里的顺序——这就是「串联」。
    pub fn steps(&self) -> Vec<Step> {
        self.payload
            .get("steps")
            .and_then(|v| v.as_sequence())
            .map(|items| {
                items
                    .iter()
                    .cloned()
                    .map(|payload| Step { payload })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn step(&self, name: &str) -> Option<Step> {
        self.steps().into_iter().find(|step| step.name() == name)
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
) -> Workflow {
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
    let flow = Workflow::new(data, name, Value::Mapping(payload), workflows);
    if let Some(parent) = flow.file().parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(flow.file(), flow.to_yaml());
    flow
}

pub fn open_workflow(data: &Path, name: &str, workflows: Option<&Path>) -> Workflow {
    Workflow::new(data, name, Value::Mapping(Mapping::new()), workflows).reload()
}

/// 把一条工作流存成一份可带走的文件（原样，不改内容）。
pub fn export(flow: &Workflow, target: &Path) -> PathBuf {
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
) -> std::result::Result<Workflow, WorkflowError> {
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
    let flow = Workflow::new(data, &chosen, payload.clone(), workflows);
    if flow.exists() {
        return Err(WorkflowError(format!(
            "已经有一条工作流叫「{chosen}」：{}（换名字用 --as）",
            flow.file().display()
        )));
    }
    if let Some(mapping) = payload.as_mapping_mut() {
        mapping.insert(Value::String("name".into()), Value::String(chosen.clone()));
    }
    let flow = Workflow::new(data, &chosen, payload, workflows);
    if let Some(parent) = flow.file().parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(flow.file(), flow.to_yaml());
    Ok(flow)
}

pub fn listing(data: &Path, workflows: Option<&Path>) -> Vec<Workflow> {
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

// ---- 任务 ----

// ---- 定义核对：声明与判据对不对得上 ----

/// 一条定义核对出来的一件事。
pub struct Finding {
    pub where_: String,
    pub what: String,
    pub ok: bool,
}

/// 核对一条工作流：
///
/// 一、判据里写到的路径（`path` / `file`，`{{…}}` 先按数据仓展开）在不在；
/// 二、description 里提到的报告小节（`## 名字`）有没有判据覆盖（至少一条 `contains` 写它）。
///
/// 目的是把「约定」变成当场能红的核对：路径一搬家、小节一漏，立刻看得见。
pub fn check(flow: &Workflow, root: &Path, data: &Path) -> Vec<Finding> {
    let mut found: Vec<Finding> = Vec::new();
    for step in flow.steps() {
        for criterion in step.rules() {
            let literal = criterion
                .get("path")
                .or_else(|| criterion.get("file"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if literal.is_empty() {
                continue;
            }
            // 按任务落点的占位（{{report}} 一类）在定义这一层核不了，跳过。
            if literal.contains("{{report}}")
                || literal.contains("{{journal}}")
                || literal.contains("{{log}}")
            {
                continue;
            }
            let written = expand_placeholders(literal, data);
            let target = if Path::new(&written).is_absolute() {
                PathBuf::from(&written)
            } else {
                root.join(&written)
            };
            found.push(Finding {
                where_: format!("{}·{}", step.name(), literal),
                what: format!("判据里的路径在不在：{written}"),
                ok: target.exists(),
            });
        }
    }

    let covered: Vec<String> = flow
        .steps()
        .into_iter()
        .flat_map(|step| step.rules())
        .filter_map(|criterion| {
            criterion
                .get("contains")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .collect();
    let mut mentioned: Vec<String> = Vec::new();
    for step in flow.steps() {
        let text: String = step.description();
        // 两种写法都认：`## 名字` 与 「名字」一节 / 「名字」节
        for piece in text.split("## ").skip(1) {
            let name = piece
                .split(|ch: char| ch.is_whitespace() || ch == '`' || ch == '」')
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            if !name.is_empty() && !mentioned.contains(&name) {
                mentioned.push(name);
            }
        }
        let mut rest: &str = text.as_str();
        while let Some(at) = rest.find('「') {
            let after = &rest[at + '「'.len_utf8()..];
            let Some(end) = after.find('」') else { break };
            let name = after[..end].trim().to_string();
            let tail = after[end + '」'.len_utf8()..].trim_start();
            let is_section =
                tail.starts_with("一节") || tail.starts_with("节") || tail.starts_with("两节");
            if is_section && !name.is_empty() && !name.contains(' ') && !mentioned.contains(&name) {
                mentioned.push(name);
            }
            rest = &after[end + '」'.len_utf8()..];
        }
    }
    for name in mentioned {
        found.push(Finding {
            where_: "description".to_string(),
            what: format!("description 提到的报告小节有没有判据覆盖：{name}"),
            ok: covered.iter().any(|value| value.contains(&name)),
        });
    }
    found
}

/// 判据里的占位先按数据仓展开（够核对用：`{{report}}` 一类指到本仓的产物路径）。
fn expand_placeholders(value: &str, data: &Path) -> String {
    value
        .replace("{{artifacts}}", &data.join("artifacts").to_string_lossy())
        .replace(
            "{{report}}",
            &data.join("artifacts/report").to_string_lossy(),
        )
        .replace(
            "{{journal}}",
            &data.join("artifacts/journal").to_string_lossy(),
        )
        .replace("{{log}}", &data.join("tasks").to_string_lossy())
}

/// 核对结果写成人读的一段。
pub fn describe(found: &[Finding]) -> Vec<String> {
    let mut lines = vec![format!("核对 {} 件事", found.len())];
    for item in found {
        let mark = if item.ok { "✓" } else { "✗" };
        lines.push(format!("  {mark} {}——{}", item.where_, item.what));
    }
    if found.is_empty() {
        lines.push("  （这条定义里没有可核对的路径与小节）".to_string());
    }
    lines
}

pub fn all_ok(found: &[Finding]) -> bool {
    found.iter().all(|item| item.ok)
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
