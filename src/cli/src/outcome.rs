//! 动作结果：命令行与窗口共用的一份算出来的东西。
//!
//! `ok` 定退出码，`lines` 给命令行印，`columns` 与 `rows` 给窗口画；
//! 动作之间不互相打印，都只交出这一层。原先抽在工具箱里两边共用，
//! 后来决定它不是领域模型（是各平台自己的说法），就搬回这里。

use serde_json::{Value as Json, json};
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct Outcome {
    pub ok: bool,
    pub lines: Vec<String>,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    /// 有些动作要交出结构化的东西，而不是行列。
    pub payload: Option<Json>,
}

/// 这一层就是「动作的结果」——命令行里叫 `Result`。
pub type Result = Outcome;

impl Outcome {
    pub fn new(ok: bool) -> Self {
        Outcome {
            ok,
            ..Default::default()
        }
    }

    pub fn lines(ok: bool, lines: Vec<String>) -> Self {
        Outcome {
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

/// 路径相对根写短一点；不在根底下就原样。
pub fn short(root: &Path, path: &Path) -> String {
    let root = root.to_string_lossy();
    let path = path.to_string_lossy();
    let prefix = if root.ends_with('/') {
        root.to_string()
    } else {
        format!("{root}/")
    };
    match path.strip_prefix(&prefix) {
        Some(rest) => rest.to_string(),
        None => path.to_string(),
    }
}
