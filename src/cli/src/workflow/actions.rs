//! 工作流聚合 / 五个动作：写、看、导出、导入、列。

use super::{all_ok, check, create, describe, export, import_workflow, listing, open_workflow};
use crate::outcome::Outcome;
use crate::workspace::short;
use serde_json::{Value as Json, json};
use serde_yaml::Value;
use std::path::Path;

pub fn workflow_new(
    data: &Path,
    name: &str,
    steps: &[String],
    note: &str,
    workflows: Option<&Path>,
) -> Outcome {
    if name.trim().is_empty() {
        return Outcome::lines(false, vec!["请先给工作流起个名字".to_string()]);
    }
    if steps.is_empty() {
        return Outcome::lines(false, vec!["至少给一个步骤：--steps 甲,乙,丙".to_string()]);
    }
    let flow = create(data, name.trim(), steps, note, workflows);
    workflow_show(data, name.trim(), workflows)
        .with_first(format!("写下工作流：{}", short(data, &flow.file())))
}

pub fn workflow_show(data: &Path, name: &str, workflows: Option<&Path>) -> Outcome {
    let flow = open_workflow(data, name, workflows);
    if !flow.exists() {
        return Outcome::lines(
            false,
            vec![format!("没有这条工作流：{}", short(data, &flow.file()))],
        );
    }
    let mut result = Outcome::new(true);
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
    // 给窗口的那一栏：定义原文 + 它在哪 + 文件原文（定义态看的就是它）。
    result.data = Some(json!({
        "payload": to_json(&flow.payload),
        "path": short(data, &flow.file()),
        "yaml": std::fs::read_to_string(flow.file()).unwrap_or_default(),
    }));
    result
}

/// 定义原文转成 JSON（给窗口那一栏用）；转不动按 null。
fn to_json(value: &Value) -> Json {
    serde_json::to_value(value).unwrap_or(Json::Null)
}

pub fn workflow_export(
    data: &Path,
    name: &str,
    target: &Path,
    workflows: Option<&Path>,
) -> Outcome {
    let flow = open_workflow(data, name, workflows);
    if !flow.exists() {
        return Outcome::lines(
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

pub fn workflow_import(
    data: &Path,
    source: &Path,
    name: &str,
    workflows: Option<&Path>,
) -> Outcome {
    if !source.is_file() {
        return Outcome::lines(false, vec![format!("没有这份文件：{}", source.display())]);
    }
    match import_workflow(data, source, name, workflows) {
        Err(error) => Outcome::lines(false, vec![error.0]),
        Ok(flow) => workflow_show(data, &flow.name, workflows).with_first(format!(
            "已导入：{}（步骤 {} 个）",
            short(data, &flow.file()),
            flow.steps().len()
        )),
    }
}

pub fn workflow_list(data: &Path, workflows: Option<&Path>) -> Outcome {
    let found = listing(data, workflows);
    let mut result = Outcome::new(true);
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

/// 核对一条工作流的声明与判据对不对得上，结果印给人。
pub fn workflow_check(data: &Path, name: &str, root: &Path, workflows: Option<&Path>) -> Outcome {
    let flow = open_workflow(data, name, workflows);
    if !flow.exists() {
        return Outcome::lines(
            false,
            vec![format!("没有这条工作流：{}", short(data, &flow.file()))],
        );
    }
    let found = check(&flow, root);
    let ok = all_ok(&found);
    let mut result = Outcome::new(ok);
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
