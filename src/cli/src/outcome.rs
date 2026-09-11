//! 动作结果：命令行与窗口共用的一份算出来的东西。
//!
//! `ok` 定退出码，`lines` 给命令行印，`columns` 与 `rows` 给窗口画；
//! 动作之间不互相打印，都只交出这一层。

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

pub fn short(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(rest) => rest.to_string_lossy().to_string(),
        Err(_) => path.to_string_lossy().to_string(),
    }
}

// ---- 工作区 ----
