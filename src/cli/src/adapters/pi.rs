//! 适配 / 执行器 / 调 `pi`：起进程、读回一段话、从回答里认结论。

use std::path::Path;
use std::process::Command;

/// 把话交给 `pi` 跑：非交互。
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

/// 一段输出收成一行。
pub fn one_line(text: &str, limit: usize) -> String {
    text.lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
        .trim()
        .chars()
        .take(limit)
        .collect()
}

/// 从智能体的回答里读一条结论：先认「序号. …」那行，没有就整段兜底。
pub fn verdict_of(out: &str, index: usize) -> (String, String) {
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
