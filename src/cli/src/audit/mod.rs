//! 领域服务：审计资产表与工作区。
//!
//! 比资产表与工作区：资产表有而工作区无（`artifact::missing`）、工作区有而未登记
//! （`catalog::unregistered`）。`--make` 补建缺的文档格，独立仓库那三格不凭空建。
//!
//! 判据怎么跑不在这里——那是工单走一步的事，在 `crate::order::execute`；
//! 出处见 `docs/dev-guide/audit.md`。

use std::path::Path;

use crate::catalog;
use crate::outcome::Outcome;
use crate::workspace::short;

pub fn audit(root: &Path, make: bool) -> Outcome {
    let made = if make {
        crate::artifact::make(root, None)
    } else {
        Vec::new()
    };
    let missing = crate::artifact::missing(root);
    let unregistered = catalog::build(root).unregistered(root);
    let ok = missing.is_empty() && unregistered.is_empty();
    let mut result = Outcome {
        ok,
        ..Default::default()
    };
    result.columns = vec!["问题".to_string(), "说明".to_string()];
    for asset in &missing {
        result.rows.push(vec![
            "缺资产".to_string(),
            format!("{}（{}）", asset.category, asset.name),
        ]);
    }
    for path in &unregistered {
        result
            .rows
            .push(vec!["未登记".to_string(), short(root, path)]);
    }
    result.lines = made
        .iter()
        .map(|path| format!("补建：{}", short(root, path)))
        .collect();
    result.lines.extend(
        result
            .rows
            .iter()
            .map(|row| format!("{}：{}", row[0], row[1])),
    );
    if ok {
        result
            .lines
            .push("审计通过：二十格齐备，无未登记目录。".to_string());
    } else if !unregistered.is_empty() && missing.is_empty() {
        result
            .lines
            .push("未登记的目录要么属于某一格（改资产表），要么不该在这儿。".to_string());
    }
    result.data = Some(serde_json::json!({
        "root": root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
        "result": if ok { "通过" } else { "有问题" },
        "missing": missing.iter().map(|a| serde_json::json!({"category": a.category, "name": a.name})).collect::<Vec<_>>(),
        "unregistered": unregistered.iter().map(|p| short(root, p)).collect::<Vec<_>>(),
    }));
    result
}
