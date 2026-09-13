//! 任务聚合 / 日志：流水事件、时间戳、字段读取、日志叙事。

use serde_yaml::Value as Yaml;
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

pub fn text_of(value: &serde_yaml::Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string()
}

/// 日志收叙事：一段一段往下写。
pub fn narrate(task: &super::Task, words: &str) {
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

/// 流水里的一条：什么时候、哪一步、一句话、过没过。
///
/// 流水只增不改（规范 `process/task.md`·语法）；「走过哪几步」由工作区按
/// 事件里的步骤名判（`crate::workspace::progress`）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEvent {
    pub at: String,
    pub step: String,
    pub detail: String,
    pub ok: bool,
}

impl JournalEvent {
    pub fn of(value: &Yaml) -> JournalEvent {
        JournalEvent {
            at: text_of(value, "at"),
            step: text_of(value, "step"),
            detail: text_of(value, "detail"),
            ok: value.get("ok").and_then(|v| v.as_bool()).unwrap_or(false),
        }
    }
}
