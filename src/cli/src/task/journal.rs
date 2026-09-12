//! 任务聚合 / 日志：时间戳、字段读取、日志叙事。

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
