//! 工作流聚合：一串有序的步骤。
//!
//! 本文件只装模型——从定义里读与校验在 [`super::read`]；
//! 定义核对在本聚合，见 `crate::workflow::check`。
//! 规矩的出处是 `docs/specification/process/workflow.md`·语法。
//!
//! 模型不可变:[`Workflow::of`] 读已经校验过的定义。YAML 怎么读写是平台的事。

use crate::criterion::Criterion;
use crate::executor::{AGENT, HUMAN, RULE};

/// 一个工作步骤：叫什么、做什么、谁执行、怎么算完。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    /// 全球凭证：定义里不写，读时按「工作流凭证 + 名字」派生（`crate::ids`）。
    pub id: String,
    pub name: String,
    pub description: String,
    pub executor: String,
    pub criteria: Vec<Criterion>,
}

impl Step {
    pub fn criteria(&self) -> Vec<Criterion> {
        self.criteria.clone()
    }

    pub fn rules(&self) -> Vec<Criterion> {
        self.of_kind(RULE)
    }

    pub fn agents(&self) -> Vec<Criterion> {
        self.of_kind(AGENT)
    }

    pub fn gates(&self) -> Vec<Criterion> {
        self.of_kind(HUMAN)
    }

    fn of_kind(&self, kind: &str) -> Vec<Criterion> {
        self.criteria
            .iter()
            .filter(|item| item.executor() == kind)
            .cloned()
            .collect()
    }
}

/// 工作流聚合：一串步骤（不含文件位置——那是各自包的事）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow {
    /// 全球凭证：定义里不写，读时按「工作区 id + 名字」派生（`crate::ids`）。
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<Step>,
}

impl Workflow {
    /// 凭证现算：工作流按「工作区 id + 名字」、步骤按「工作流凭证 + 名字」派生。
    /// 定义文件不带凭证；在人写的那份上不添字，凭证只在读进内存时补上。
    pub fn credentials(mut self, workspace_id: &str) -> Workflow {
        if self.id.is_empty() {
            self.id = crate::ids::derive("workflow", &[workspace_id, &self.name]);
        }
        for step in &mut self.steps {
            if step.id.is_empty() {
                step.id = crate::ids::derive("step", &[&self.id, &step.name]);
            }
        }
        self
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn steps(&self) -> Vec<Step> {
        self.steps.clone()
    }

    /// 步骤名，按定义顺序。
    pub fn step_names(&self) -> Vec<String> {
        self.steps.iter().map(|step| step.name.clone()).collect()
    }

    pub fn step(&self, name: &str) -> Option<Step> {
        self.steps.iter().find(|step| step.name == name).cloned()
    }
}
