# audit：判据与审计

`src/audit/` 装两件相关的事：跑机械判据，与审计资产表与工作区。

## 判据怎么跑

判据是字段，不是一行小语法。四种判法：`path` 存在、`absent` 不存在、`file` 加 `contains` 文件含这段文字、`run` 命令退出码为零。路径相对工作区根，写绝对路径则按绝对路径（跨仓库核对用）。

翻译（说明怎么写、四种判法怎么认）在本仓 `crate::criterion`；本侧只剩「真去跑」，`run` 拿 `sh -c` 在工作区根跑。

## 审计什么

`audit` 比资产表与工作区：资产表有而工作区无（`artifact::missing`）、工作区有而未登记（`catalog::unregistered`）。`--make` 补建缺的文档格，独立仓库那三格不凭空建。

## 测试

判据矩阵在 `tests/criteria_matrix.rs`——谁执行（agent / human）× 怎么走（`--next` / `--done`）× 判据三类。
