//! 工作区聚合 / 落点：产物落在哪。
//!
//! 落点只给**相对工作区根的路径**：声明了按声明的（绝对路径原样），没声明落
//! `artifacts/<产物的名字>/<任务名>.md`。拼上目录是平台的事（规范 `process/task.md`·语法）。
//! 占位展开在平台侧（`crate::task::execute`）——那里才知道产物目录接在哪儿。

use super::model::Workspace;
use crate::artifact::Artifact;
use crate::task::model::Task;

impl Workspace {
    /// 这件产物落哪（规范「任务 / 语法」里的落点）。
    ///
    /// 给的是相对工作区根的路径；平台拿自己的目录接上。
    pub fn place(&self, task: &Task, artifact: &Artifact) -> String {
        if let Some(written) = task.declared(&artifact.name) {
            return written;
        }
        format!("artifacts/{}/{}.md", artifact.name, task.name)
    }
}
