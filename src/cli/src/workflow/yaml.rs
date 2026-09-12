//! 工作流聚合 / YAML 读写与 schema 校验。

use quanttide_work::workflow as shared;
use serde_yaml::Value;
use std::fmt;
use std::path::Path;

/// 这份文件不像一份工作流。
#[derive(Debug)]
pub struct WorkflowError(pub String);

impl fmt::Display for WorkflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for WorkflowError {}

pub fn dump(value: &Value) -> String {
    serde_yaml::to_string(value).unwrap_or_default()
}

/// 读一份定义：不是映射、缺字段、取值不对，当场报错。
///
/// 校验的规矩在工具箱里（`quanttide_work::workflow::validate`）——两侧共用一份，
/// 报错文字也一字不差。
pub fn load(path: &Path) -> std::result::Result<Value, WorkflowError> {
    let file = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let text =
        std::fs::read_to_string(path).map_err(|e| WorkflowError(format!("{file} 读不了：{e}")))?;
    let payload: Value = serde_yaml::from_str(&text)
        .map_err(|e| WorkflowError(format!("{file} 不是合法的 YAML：{e}")))?;
    if let Err(error) = shared::validate(&payload) {
        return Err(WorkflowError(error.message(&file)));
    }
    Ok(payload)
}

/// 取一条字符串字段（缺了、不是字符串都当空；两头空白去掉）。
pub fn text_of(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|item| item.as_str())
        .unwrap_or_default()
        .trim()
        .to_string()
}
