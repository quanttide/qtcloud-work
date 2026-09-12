//! 任务：工作流的一次执行实例。
//!
//! `<数据仓>/tasks/<任务>.yaml` 是一次执行：跑哪条工作流 + 自带的运行上下文
//! （`root` / `data` / `workflows`）+ 流水；产物落在 `artifacts/` 下。
//!
//! 走一步：执行者是 AI 的交给 `pi` 跑，然后程序自己判机械判据、把闸门项记进任务文件，
//! 事实记进流水与报告。

use crate::workflow::{self, Step, WorkflowFile};
use quanttide_work::criterion::Criterion;
use serde_json::{Value as Json, json};
use serde_yaml::{Mapping, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

pub const LOG: &str = "log";
pub const REPORT: &str = "report";
pub const JOURNAL: &str = "journal";

pub fn now() -> String {
    // 不引 chrono：直接用系统命令取本地时间，格式与实验室一致。
    let out = Command::new("date").arg("+%Y-%m-%d %H:%M").output();
    match out {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        Err(_) => String::new(),
    }
}

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
    pub fn products(&self) -> std::collections::BTreeMap<String, String> {
        let mut found = std::collections::BTreeMap::new();
        if let Some(mapping) = self.payload().get("products").and_then(|v| v.as_mapping()) {
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

    /// 流水就在任务文件里（`{{log}}` 指它）；产物路径先看声明，没声明就落草稿区。
    pub fn artifact(&self, kind: &str) -> PathBuf {
        if kind == LOG {
            return self.file();
        }
        if let Some(written) = self.products().get(kind)
            && !written.trim().is_empty()
        {
            let path = PathBuf::from(written.trim());
            return if path.is_absolute() {
                path
            } else {
                self.root.join(path)
            };
        }
        self.artifacts_dir()
            .join(kind)
            .join(format!("{}.md", self.name))
    }

    pub fn exists(&self) -> bool {
        self.file().is_file()
    }

    /// 任务自己的属性，不是流水里的一步。
    pub fn start(&self) -> String {
        text_of(&self.payload(), "start")
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
        text_of(&self.payload(), "workflow")
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
        quanttide_work::task::Task::of(&self.name, &self.payload())
    }

    /// 哪些步骤走过了：算法在工具箱的任务聚合里——附加判定投票、重新执行从头算。
    pub fn done(&self) -> Vec<String> {
        self.shared().done_steps(&self.workflow().shared())
    }

    pub fn next_step(&self) -> Option<Step> {
        let next = self.shared().next_step(&self.workflow().shared())?;
        self.steps().into_iter().find(|step| step.name() == next)
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
            event.insert(Value::String("at".into()), Value::String(now()));
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

/// 任务原文转成 JSON（给窗口那一栏用）；转不动按 null。
fn to_json(value: &Value) -> Json {
    serde_json::to_value(value).unwrap_or(Json::Null)
}

fn text_of(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string()
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
        payload.insert(Value::String("start".into()), Value::String(now()));
        payload.insert(
            Value::String("workflow".into()),
            Value::String(workflow_name.to_string()),
        );
        payload.insert("log".into(), Value::Sequence(Vec::new()));
        payload.insert("gates".into(), Value::Sequence(Vec::new()));
        payload.insert("products".into(), Value::Mapping(Mapping::new()));
        for (key, value) in context(root, data, workflows) {
            payload.insert(Value::String(key), Value::String(value));
        }
        payload.insert(Value::String("log".into()), Value::Sequence(Vec::new()));
        let _ = std::fs::write(
            task.file(),
            serde_yaml::to_string(&Value::Mapping(payload)).unwrap_or_default(),
        );
    }
    for kind in [REPORT, JOURNAL] {
        let declared = task
            .products()
            .get(kind)
            .is_some_and(|v| !v.trim().is_empty());
        if declared && !task.artifact(kind).is_file() {
            if let Some(parent) = task.artifact(kind).parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(task.artifact(kind), format!("# {kind}：{name}\n"));
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

/// 交给 AI 的那一段话：说什么、不说什么是定死的，话本身在 `crate::prompts`。
pub fn prompt_for(task: &Task, step: &Step) -> String {
    crate::prompts::prompt_for(&facts_of(task, step), &step.criteria())
}

/// 这一步的现场：路径由命令行这边算好递进去。
fn facts_of(task: &Task, step: &Step) -> crate::prompts::Facts {
    crate::prompts::Facts {
        root: task.root.display().to_string(),
        data: task.data.display().to_string(),
        name: task.name.clone(),
        start: task.start(),
        workflow_name: task.workflow_name(),
        workflow_description: task.workflow().description(),
        steps: task
            .steps()
            .into_iter()
            .map(|s| s.name())
            .collect::<Vec<_>>()
            .join("、"),
        step: step.name(),
        what: step.description(),
        report: task.relative(&task.artifact(REPORT)),
        journal: task.relative(&task.artifact(JOURNAL)),
        log: task.relative(&task.artifact(LOG)),
        artifacts: task.artifacts_dir().display().to_string(),
    }
}

/// 把这一步交给 AI 跑：非交互调 `pi`。
pub fn run_ai(prompt: &str, root: &Path) -> (bool, String) {
    let done = Command::new("pi")
        .args(["-p", "--no-session", prompt])
        .current_dir(root)
        .output();
    match done {
        Err(e) => (false, format!("没找到 pi：{e}")),
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
            let text = if stdout.is_empty() { stderr } else { stdout };
            (out.status.success(), text)
        }
    }
}

/// 交给智能体审的那一段话：产物 + 判准，逐条回答。
pub fn judge_prompt(task: &Task, step: &Step, criteria: &[Criterion]) -> String {
    crate::prompts::judge_prompt(&facts_of(task, step), criteria)
}

fn one_line(text: &str, limit: usize) -> String {
    let line = text
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .unwrap_or("")
        .trim();
    line.chars().take(limit).collect()
}

/// 从智能体的回答里读一条结论：先认「序号. …」那行，没有就整段兜底。
fn verdict_of(out: &str, index: usize) -> (String, String) {
    let prefix = format!("{index}.");
    for line in out.lines() {
        let stripped = line.trim();
        if let Some(rest) = stripped.strip_prefix(&prefix) {
            let tail = rest.trim();
            let verdict = if tail.starts_with("不通过") {
                "✗"
            } else if tail.starts_with("通过") {
                "✓"
            } else {
                "待判"
            };
            return (verdict.to_string(), tail.to_string());
        }
    }
    let flat = out.replace(' ', "");
    if flat.contains("不通过") {
        ("✗".to_string(), one_line(out, 80))
    } else if flat.contains("通过") {
        ("✓".to_string(), one_line(out, 80))
    } else {
        ("待判".to_string(), one_line(out, 80))
    }
}

/// 让智能体按判准审一遍；返回（说明，结论，理由）。
pub fn judge_by_ai(
    task: &Task,
    step: &Step,
    criteria: &[Criterion],
) -> Vec<(String, String, String)> {
    let (ran, out) = run_ai(&judge_prompt(task, step, criteria), &task.root);
    let mut rows = Vec::new();
    for (index, criterion) in criteria.iter().enumerate() {
        let note = criterion.text();
        if !ran {
            rows.push((
                note,
                "待判".to_string(),
                format!("智能体没跑成：{}", one_line(&out, 80)),
            ));
            continue;
        }
        let (verdict, reason) = verdict_of(&out, index + 1);
        rows.push((note, verdict, reason));
    }
    let all_pass = rows.iter().all(|(_, verdict, _)| verdict == "✓");
    let detail = format!(
        "AI 审查（同一模型）：{}",
        rows.iter()
            .map(|(note, verdict, _)| format!("{note}→{verdict}"))
            .collect::<Vec<_>>()
            .join("；")
    );
    task.record(&format!("{}·审", step.name()), &detail, all_pass);
    rows
}

/// 把 `{{report}}` / `{{journal}}` / `{{log}}` / `{{artifacts}}` 换成本次任务的产物路径。
pub fn expand(task: &Task, value: &str) -> String {
    let mut out = String::new();
    let mut rest = value;
    while let Some(at) = rest.find("{{") {
        out.push_str(&rest[..at]);
        let after = &rest[at + 2..];
        if let Some(end) = after.find("}}") {
            let kind = &after[..end];
            let path = match kind {
                "report" => Some(task.artifact(REPORT)),
                "journal" => Some(task.artifact(JOURNAL)),
                "log" => Some(task.artifact(LOG)),
                "artifacts" => Some(task.artifacts_dir()),
                _ => None,
            };
            match path {
                Some(path) => out.push_str(&relative_to_root(task, &path)),
                None => {
                    out.push_str("{{");
                    out.push_str(kind);
                    out.push_str("}}");
                }
            }
            rest = &after[end + 2..];
        } else {
            out.push_str("{{");
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// 判据按工作区根解析，占位也给工作区根视角的路径。
fn relative_to_root(task: &Task, path: &Path) -> String {
    match path.strip_prefix(&task.root) {
        Ok(rest) => rest.to_string_lossy().to_string(),
        Err(_) => path.to_string_lossy().to_string(),
    }
}

/// 判据里的占位先换成本次任务的真实路径，再去跑。
pub fn expanded_criteria(task: &Task, criteria: &[Criterion]) -> Vec<Criterion> {
    criteria
        .iter()
        .map(|criterion| criterion.expanded(|text| expand(task, text)))
        .collect()
}

/// 走一步：能让 AI 跑的交给 AI，然后跑判据、记账、写报告。
pub fn execute(
    task: &Task,
    root: &Path,
    step: &str,
    note: &str,
    auto: bool,
) -> (bool, Vec<String>, Vec<(String, String, String)>) {
    let found = match task.workflow().step(step) {
        Some(found) => found,
        None => {
            return (
                false,
                vec![format!("工作流里没有这一步：{step}")],
                Vec::new(),
            );
        }
    };
    let mut lines: Vec<String> = Vec::new();
    if auto && !found.human() {
        lines.push(format!(
            "{}：交给 AI（{}）跑",
            found.name(),
            found.executor()
        ));
        let (ran, out) = run_ai(&prompt_for(task, &found), root);
        let one = if out.is_empty() {
            "（没输出）".to_string()
        } else {
            one_line(&out, 80)
        };
        lines.push(format!(
            "  AI {}：{}",
            if ran { "跑完了" } else { "跑不动" },
            one
        ));
        task.record(&found.name(), &format!("AI 执行：{one}"), ran);
        if !ran {
            write_gates(task, &[]);
            lines.push("  （AI 没跑成，这一步不算过；修好再来）".to_string());
            return (false, lines, Vec::new());
        }
    } else if found.human() && auto {
        lines.push(format!(
            "{}：这一步的执行者是人（{}）——轮到你，做完用 qtcloud-work task <名字> --done {}",
            found.name(),
            found.executor(),
            found.name()
        ));
        return (true, lines, Vec::new());
    }

    let rule_items = crate::audit::items_of(&expanded_criteria(task, &found.rules()));
    let (results, _) = crate::audit::run(root, &rule_items);
    let agents = found.agents();
    let judged: Vec<(String, String, String)> = if auto && !agents.is_empty() {
        judge_by_ai(task, &found, &expanded_criteria(task, &agents))
    } else {
        agents
            .iter()
            .map(|criterion| {
                (
                    criterion.text(),
                    "待判".to_string(),
                    "没跑智能体（人为地记一步）".to_string(),
                )
            })
            .collect()
    };
    let gates: Vec<String> = found
        .gates()
        .iter()
        .map(|criterion| criterion.text())
        .collect();
    let rules_pass = results.iter().all(|(_, passed, _)| *passed);
    // 待判（人为地记一步、没跑智能体）不挡这一步，原样进闸门项。
    let judged_pass = judged
        .iter()
        .all(|(_, verdict, _)| verdict == "✓" || verdict == "待判");
    let ok = rules_pass && judged_pass;
    let detail = if !note.trim().is_empty() {
        note.trim().to_string()
    } else if !results.is_empty() {
        results
            .iter()
            .map(|(item, _, _)| item.description.clone())
            .collect::<Vec<_>>()
            .join("；")
    } else {
        "做完".to_string()
    };
    if !(auto && !found.human()) {
        task.record(step, &detail, ok);
    } else if !found.rules().is_empty() {
        // 交给 AI 跑的步骤：机器判据这一半单独记一笔（审查那条只知道 agent 判据）。
        task.record(&format!("{}·判", found.name()), &detail, rules_pass);
    }
    let mut gate_lines = gates.clone();
    gate_lines.extend(
        judged
            .iter()
            .filter(|(_, verdict, _)| verdict != "✓")
            .map(|(note, _, _)| note.clone()),
    );
    write_gates(task, &gate_lines);
    lines.push(format!(
        "{} {}：{}",
        if ok { "✓" } else { "✗" },
        step,
        detail
    ));
    lines.extend(results.iter().map(|(item, passed, spec)| {
        format!(
            "  {} {}（{spec}）",
            if *passed { "✓" } else { "✗" },
            item.description
        )
    }));
    lines.extend(
        judged
            .iter()
            .map(|(note, verdict, reason)| format!("  {verdict} {note}（{reason}）")),
    );
    lines.extend(gates.iter().map(|note| format!("  ⧗ {note}（留给人）")));
    let mut rows: Vec<(String, String, String)> = results
        .iter()
        .map(|(item, passed, spec)| {
            (
                item.description.clone(),
                if *passed {
                    "✓".to_string()
                } else {
                    "✗".to_string()
                },
                spec.clone(),
            )
        })
        .collect();
    rows.extend(judged.clone());
    rows.extend(
        gates
            .into_iter()
            .map(|note| (note, "闸门".to_string(), "留给人拍板".to_string())),
    );
    (ok, lines, rows)
}

/// 闸门项是任务的状态，记进任务文件；产物一个字都不碰。
pub fn write_gates(task: &Task, gates: &[String]) {
    let mut notes = task.gates();
    for note in gates {
        if !notes.iter().any(|existing| existing == note) {
            notes.push(note.clone());
        }
    }
    task.set_gates(&notes);
}

/// 日志收叙事：一段一段往下写。
pub fn narrate(task: &Task, words: &str) {
    let path = task.artifact(JOURNAL);
    let mut text =
        std::fs::read_to_string(&path).unwrap_or_else(|_| format!("# 日志：{}\n", task.name));
    text = text
        .lines()
        .filter(|line| {
            let stripped = line.trim();
            !(stripped.starts_with('（') && stripped.ends_with('）'))
        })
        .collect::<Vec<_>>()
        .join("\n");
    text = text.trim_end().to_string();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&path, format!("{text}\n\n{}\n", words.trim()));
    task.record(
        "历史",
        &words.trim().chars().take(40).collect::<String>(),
        true,
    );
}

pub fn state_line(task: &Task) -> String {
    task.shared().state_line(&task.workflow().shared())
}

// ---- 动作 ----

use crate::paths::short;
use quanttide_work::outcome::Outcome;

pub fn task_new(
    root: &Path,
    data: &Path,
    name: &str,
    workflow_name: &str,
    workflows: Option<&Path>,
) -> Outcome {
    if name.trim().is_empty() {
        return Outcome::lines(false, vec!["请先给这件任务起个名字".to_string()]);
    }
    let flow = workflow::open_workflow(data, workflow_name, workflows);
    if !flow.exists() {
        return Outcome::lines(
            false,
            vec![format!(
                "没有这条工作流：{}（qtcloud-work workflow --list 看有哪些）",
                short(data, &flow.file())
            )],
        );
    }
    let existing = reopen(data, name.trim(), Some(root), workflows);
    if existing.exists() {
        return Outcome::lines(
            false,
            vec![format!(
                "已经有这件任务：{}（换个名字，不覆盖）",
                short(data, &existing.file())
            )],
        );
    }
    let task = create(root, data, name.trim(), workflow_name.trim(), workflows);
    task_status(Some(root), data, name.trim(), workflows)
        .with_first(format!("起了：{}", short(data, &task.file())))
}

pub fn task_status(
    root: Option<&Path>,
    data: &Path,
    name: &str,
    workflows: Option<&Path>,
) -> Outcome {
    if name.trim().is_empty() {
        return Outcome::lines(
            false,
            vec!["请先选一件任务（qtcloud-work task --list 看有哪些）".to_string()],
        );
    }
    let task = reopen(data, name, root, workflows);
    if !task.exists() {
        return Outcome::lines(
            false,
            vec![format!("没有这件任务：{}", short(data, &task.file()))],
        );
    }
    let done = task.done();
    let mut result = Outcome::new(true);
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
    result.lines.push(state_line(&task));
    result
        .lines
        .push(format!("指令：{}", short(data, &task.file())));
    result.lines.push(format!(
        "产物：{}、{}　流水：{}",
        short(data, &task.artifact(REPORT)),
        short(data, &task.artifact(JOURNAL)),
        short(data, &task.artifact(LOG))
    ));
    // 给窗口的那一栏：任务原文（装成领域对象要用）+ 三样产物的落点。
    result.data = Some(json!({
        "payload": to_json(&task.payload()),
        "products": {
            REPORT: short(data, &task.artifact(REPORT)),
            JOURNAL: short(data, &task.artifact(JOURNAL)),
            LOG: short(data, &task.artifact(LOG)),
        },
    }));
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

pub fn task_list(root: Option<&Path>, data: &Path, workflows: Option<&Path>) -> Outcome {
    let found = listing(root, data, workflows);
    let mut result = Outcome::new(true);
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
) -> Outcome {
    let task: Task = reopen(data, name, root, workflows);
    if !task.exists() {
        return Outcome::lines(
            false,
            vec![format!("没有这件任务：{}", short(data, &task.file()))],
        );
    }
    let mut chosen = step.trim().to_string();
    if chosen.is_empty() {
        if !auto {
            return Outcome::lines(
                false,
                vec!["请给步骤名（qtcloud-work task <名字> 看有哪些步骤）".to_string()],
            );
        }
        match task.next_step() {
            None => return Outcome::lines(true, vec!["所有步骤都走过了".to_string()]),
            Some(next) => chosen = next.name(),
        }
    }
    let (ok, lines, rows) = execute(&task, &task.root, &chosen, note, auto);
    let mut result = Outcome {
        ok,
        lines,
        ..Default::default()
    };
    result.columns = vec!["核对".to_string(), "结论".to_string(), "说明".to_string()];
    result.rows = rows.into_iter().map(|(a, b, c)| vec![a, b, c]).collect();
    result.lines.push(state_line(&task));
    result
}

pub fn task_journal(
    root: Option<&Path>,
    data: &Path,
    name: &str,
    words: &str,
    workflows: Option<&Path>,
) -> Outcome {
    let task = reopen(data, name, root, workflows);
    if !task.exists() {
        return Outcome::lines(
            false,
            vec![format!("没有这件任务：{}", short(data, &task.file()))],
        );
    }
    if words.trim().is_empty() {
        return Outcome::lines(
            false,
            vec![format!(
                "日志要人来写：{}",
                short(data, &task.artifact(JOURNAL))
            )],
        );
    }
    narrate(&task, words);
    Outcome::lines(
        true,
        vec![
            format!("日志记下一段：{}", short(data, &task.artifact(JOURNAL))),
            state_line(&task),
        ],
    )
}

// ---- 记录的段位与骨架：报告两节与日志模板 ----
