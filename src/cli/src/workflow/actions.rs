//! 工作流聚合 / 动作：写、看、列、核、带走、导进来。

use super::check;
use super::{create as create_file, export as export_file, import as import_file, listing, open};
use crate::locate::{Locate, short};
use crate::outcome::Outcome;
use serde_json::json;
use std::path::Path;

/// 写一条工作流：写完发事件，再看一遍。
pub fn workflow_create(locate: &Locate, name: &str, steps: &[String], note: &str) -> Outcome {
    if name.trim().is_empty() {
        return Outcome::lines(false, vec!["请先给工作流起个名字".to_string()]);
    }
    if steps.iter().all(|step| step.trim().is_empty()) {
        return Outcome::lines(false, vec!["至少给一个步骤：--steps 甲,乙,丙".to_string()]);
    }
    let flow = match create_file(locate, name.trim(), steps, note) {
        Ok(flow) => flow,
        Err(error) => return Outcome::lines(false, vec![error.0]),
    };
    if let Ok(payload) = flow.credentialed() {
        let _ = crate::workflow::events::created(locate, &payload);
    }
    workflow_show(locate, &flow.name)
        .with_first(format!("写下工作流：{}", short(&locate.root, &flow.file())))
}

/// 看一条工作流：步骤、谁执行、几条判据，外加凭证。
pub fn workflow_show(locate: &Locate, name: &str) -> Outcome {
    let flow = open(locate, name);
    if !flow.exists() {
        return Outcome::lines(
            false,
            vec![format!(
                "没有这条工作流：{}",
                short(&locate.root, &flow.file())
            )],
        );
    }
    let credentialed = match flow.credentialed() {
        Ok(flow) => flow,
        Err(error) => return Outcome::lines(false, vec![error]),
    };
    let mut result = Outcome::new(true);
    result.columns = vec!["步骤".to_string(), "谁做".to_string(), "判据".to_string()];
    result.lines.push(format!(
        "工作流：{}（{}）",
        flow.name,
        short(&locate.root, &flow.file())
    ));
    result.lines.push(format!("  id：{}", credentialed.id));
    if !flow.description().is_empty() {
        result.lines.push(format!("  描述：{}", flow.description()));
    }
    for step in &credentialed.steps {
        let counts = format!(
            "{} rule / {} agent / {} human",
            step.rules().len(),
            step.agents().len(),
            step.gates().len()
        );
        result
            .rows
            .push(vec![step.name.clone(), step.executor.clone(), counts]);
        result.lines.push(format!(
            "  {}（{}）：{}",
            step.name, step.executor, step.description
        ));
        for criterion in &step.criteria {
            result.lines.push(format!(
                "      {}：{}",
                criterion.executor(),
                criterion.text()
            ));
        }
    }
    // 给窗口的那一栏：定义原文 + 凭证（工作流与工作步骤各一枚）。
    result.data = Some(json!({
        "payload": serde_json::to_value(&flow.payload).unwrap_or(serde_json::Value::Null),
        "workflow_id": credentialed.id,
        "step_ids": credentialed
            .steps
            .iter()
            .map(|step| json!({"name": step.name, "id": step.id}))
            .collect::<Vec<_>>(),
    }));
    result
}

pub fn workflow_list(locate: &Locate) -> Outcome {
    let found = listing(locate);
    let mut result = Outcome::new(true);
    result.columns = vec!["工作流".to_string(), "步骤".to_string(), "位置".to_string()];
    for flow in &found {
        let names = flow
            .steps()
            .into_iter()
            .map(|step| step.name)
            .collect::<Vec<_>>()
            .join("、");
        result.rows.push(vec![
            flow.name.clone(),
            names.clone(),
            short(&locate.root, &flow.file()),
        ]);
        result.lines.push(format!("{:24} 步骤：{names}", flow.name));
    }
    if found.is_empty() {
        result.lines =
            vec!["还没有工作流：qtcloud-work workflow create <名字> --steps 甲,乙".to_string()];
    }
    result
}

/// 定义核对：只看写下的位置、不访问文件系统——路径须在区内、小节须有判据覆盖。
pub fn workflow_check(locate: &Locate, name: &str) -> Outcome {
    let flow = open(locate, name);
    if !flow.exists() {
        return Outcome::lines(
            false,
            vec![format!(
                "没有这条工作流：{}",
                short(&locate.root, &flow.file())
            )],
        );
    }
    let found = check::check(&flow.shared(), &locate.root);
    let ok = check::all_ok(&found);
    let mut result = Outcome::new(ok);
    result.columns = vec!["核对".to_string(), "结论".to_string()];
    result.lines = vec![format!(
        "定义核对：{}——{}",
        flow.name,
        if ok {
            "判据路径都在区内，描述点到的小节都有判据覆盖。"
        } else {
            "有要改的地方。"
        }
    )];
    result.lines.extend(check::describe(&found));
    result.rows = found
        .iter()
        .map(|item| {
            vec![
                item.where_.clone(),
                format!("{} {}", if item.ok { "✓" } else { "✗" }, item.what),
            ]
        })
        .collect();
    result.data = Some(json!({
        "workflow": flow.name,
        "findings": found
            .iter()
            .map(|item| json!({"where": item.where_, "what": item.what, "ok": item.ok}))
            .collect::<Vec<_>>(),
    }));
    result
}

pub fn workflow_export(locate: &Locate, name: &str, target: &Path) -> Outcome {
    let flow = open(locate, name);
    if !flow.exists() {
        return Outcome::lines(
            false,
            vec![format!(
                "没有这条工作流：{}",
                short(&locate.root, &flow.file())
            )],
        );
    }
    let saved = export_file(&flow, target);
    Outcome::lines(
        true,
        vec![format!("已导出：{}（原样带走）", saved.display())],
    )
}

pub fn workflow_import(locate: &Locate, source: &Path, as_name: &str) -> Outcome {
    let flow = match import_file(locate, source, as_name) {
        Ok(flow) => flow,
        Err(error) => return Outcome::lines(false, vec![error.0]),
    };
    if let Ok(payload) = flow.credentialed() {
        let _ = crate::workflow::events::created(locate, &payload);
    }
    Outcome::lines(
        true,
        vec![format!(
            "已导入：{}（步骤 {} 个）",
            short(&locate.root, &flow.file()),
            flow.steps().len()
        )],
    )
}
