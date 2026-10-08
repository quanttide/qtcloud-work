# TODO：cli 待办

已办（2026-10-08）。待人类评审后，本文件与 [ROADMAP.md](ROADMAP.md) 一并删除。

可直接动手的事项，逐条列、做完就勾。依据 [ROADMAP.md](ROADMAP.md) 与 [data/report/plan/qtcloud-work-cli.md](../../../../data/report/plan/qtcloud-work-cli.md)（2026-10-08）。

每一条做完的通用判据（缺一不算完）：

```bash
cd apps/qtcloud-work/src/cli
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
sh scripts/validate-usecases.sh
sh scripts/validate-line-count.sh
```

搬文件、改路径一律行为不变：命令面、输出信封（`ok` / `lines` / `columns` / `rows` / `data`）与报错文字都不许改。

## 站一 order ↔ prompts

拆法：`previous_records` 移出 `prompts`，造话术、调 pi、审从 `order/ai.rs` 移到适配层；`order` 只留走一步的判据与记账。适配层落在 `src/ai.rs`。

- [x] 一 · `previous_records` 移出 `prompts`：`prompts.rs` 只收纯数据（`Facts` 与 `&[Criterion]`），前几笔流水的拼装移到 `ai.rs`。
  - 判据：`grep -n "crate::order" src/prompts.rs` 为空。
- [x] 二 · 造话术、调 pi、审移出 `order/ai.rs`：搬到 `src/ai.rs`，`order` 不再引 `prompts`。
  - 判据：`grep -rn "crate::prompts" src/order/` 为空。
- [x] 三 · 收口：确认 `src/prompts.rs` 与 `src/order/` 之间两个方向都不再出现 `crate::` 互引，通用判据全绿。

## 站二 order → audit

拆法：把 `audit::check` / `run` / `items_of` 搬进 `order`，`audit` 只留资产审计。

- [x] 一 · 搬规则执行：`check` / `run` 从 `audit` 进 `order/execute.rs`，`items_of` 直接用 `crate::criterion`，`audit` 只做资产表与工作区对账。
  - 判据：`grep -rn "crate::audit" src/order/` 为空。
- [x] 二 · 对表：`docs/dev-guide/audit.md` 的「两件相关的事」跟着改。

## 站三 locate 文档（随站一）

已不存在的 `locate/` 改成 `workspace` 容器事实，涉及 `CONTRIBUTING.md`、`docs/dev-guide/index.md`、`docs/dev-guide/workspace.md`、`docs/dev-guide/work-order.md`、`STATUS.md` 五份。`docs/user-guide/usecases.md` 里的 `locate` 是 `compare-course-profile` 的一站（步骤名），不是模块，保留。

- [x] 改这五份。
  - 判据：五份里不再出现 `locate/`。

## 清理项

见重构方案·模块拆分五至七。

- [x] 两处 `short` 合一：留 `src/workspace/local.rs` 那份，`src/catalog/mod.rs` 改引它。
- [x] 工作流事件负载带全文：`src/workflow/events.rs` 的判据带上 `path` / `absent` / `file` / `contains` / `run`，与「带声明全文」的说法对齐。
- [x] 删失效注释 `src/criterion/model.rs:96`（指向已无的 `crate::task`）。
- [x] 报告 `src/criterion/mod.rs:1` 自称「判据聚合」与 `CONTRIBUTING.md` 的中立模型分类不符；命名由用户定，先报告不改名。
- [x] 契约测试改扫目录：`tests/contract.rs` 现手维护清单且重复三次，改成扫 `src/` 自动收集。
