# audit：审计资产表与工作区

`src/audit/` 只做一件事：比资产表与工作区——资产表有而工作区无（`artifact::missing`）、工作区有而未登记（`catalog::unregistered`）。`--make` 补建缺的文档格，独立仓库那三格不凭空建。

判据怎么跑不在这里：工单走一步时展开占位、核 rule 判据，编排在 `crate::order::execute`，真去跑（文件系统、起进程）在 `order/rules.rs`。判据的模型与翻译（四种判法怎么认）在 `crate::criterion`。

## 测试

判据矩阵在 `tests/criteria_matrix.rs`——谁执行（agent / human）× 怎么走（`--next` / `--done`）× 判据三类。
