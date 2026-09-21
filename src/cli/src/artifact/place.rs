//! 资产表聚合 / 落点：产物落在哪。
//!
//! 落点只给**相对产物落点的路径**：`<类别>/<工单名>.md`（规格 `piece/artifact.md`·落点；
//! 拼上产物落点目录是平台的事——账本归 CLI、产物归工作区，两边各得其所）。

use super::Artifact;

/// 这件产物落哪：相对产物落点（`--artifacts`）的路径。
pub fn place(artifact: &Artifact, order_name: &str) -> String {
    format!("{}/{}.md", artifact.name, order_name)
}
