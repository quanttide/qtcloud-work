# TODO：cli 待办

可直接动手的事项，逐条列、做完就勾。

做完任一条的通用判据（缺一不算完）：

```bash
cd apps/qtcloud-work/src/cli
cargo fmt --check && cargo clippy --all-targets --locked -- -D warnings && cargo test --locked
sh scripts/validate-usecases.sh
sh scripts/validate-line-count.sh
```

## 收回工具箱（Rust）

取消对 crates.io `quanttide-work` 的依赖，把 toolkit 抽走的 Rust 代码并回本仓，本仓自持。

- [x] **并聚合**：`packages/quanttide-work-toolkit/packages/rust/src` 的每个聚合按同名并入 `src/`
  - `workflow/{model,read}` → `src/workflow/`
  - `task/{model,journal}` → `src/task/`
  - `workspace/{model,place,progress,check}` → `src/workspace/`
  - `artifact/model` → `src/artifact/model.rs`（产物实例：名字 + 规格；`mod.rs` 仍是资产表，两者分文件）
  - `task/model` → `src/task/model.rs`——toolkit 的 `Task`（领域模型）与命令行的 `Task`（带位置的句柄）同名不同物：只 `mod model;`，不 `pub use model::*`，全仓用 `crate::task::model::Task` 限定
- [x] **并横切件**：`criterion/` → `src/criterion/`；`outcome.rs`、`error.rs`、`executor.rs`、`paths.rs`、`fields.rs` → `src/` 根下同名文件
- [x] **减法**：toolkit 是 lib，`pub` 项不算死；并进 bin 后没人用的公开项（`Workflow::to_yaml` / `from_value`、`Step::to_yaml` / `from_value`、`Criterion::to_yaml`、`RuleKind::as_str` 等）会算 `dead_code`，按减法删或用掉，clippy `-D warnings` 不许放行
- [x] **改引用**：`use quanttide_work::…` 全改 `use crate::…`（含 `workflow/mod.rs` 里的 `pub use`）
- [x] **去依赖**：`Cargo.toml` 删 `quanttide-work`；跟 `Cargo.lock`；`README.md`「依赖与许可」删那一行、正文提工具箱的说辞改掉
- [x] **判据**：`grep -rn "quanttide_work\|quanttide-work" src/` 为空；通用判据全绿
