# 路线图

三站已办（2026-10-08）。待人类评审后，本文件与 [TODO.md](TODO.md) 一并删除。

这一份是给人看的：为什么做、做到哪、怎么算做完。逐条待办见 [TODO.md](TODO.md)，逐条体检数据见 [STATUS.md](STATUS.md)。

把 CLI 里该消掉的依赖逐站拆掉。依据 `data/report/evaluation/qtcloud-work-cli.md` 与 `data/report/plan/qtcloud-work-cli.md`（2026-10-08）。

`workspace` 是 `workorder` 的上级容器，容器与内件互认不算环，不列站；文档没跟上单列站三。

每站行为不变：命令面、输出信封（`ok` / `lines` / `columns` / `rows` / `data`）与报错文字都不改；做完跑 `cargo fmt --check`、`cargo clippy --all-targets --locked -- -D warnings`、`cargo test --locked` 三绿。

## 站点

1. `order ↔ prompts`——互引的环，先拆
2. `order → audit`——单向越层
3. `locate/` 文档——随站一改

## 站一 order ↔ prompts

- 现状：`order/ai.rs` 引 `crate::prompts`，`prompts.rs` 又引 `crate::order`
- 证据：`src/order/ai.rs:5`、`:12`、`:43`、`:157`；`src/prompts.rs:94`
- 来路：`2cc2e41` 接上前向边，`e689df7` 接上反向边；此前 `prompts.rs` 只引 `criterion`
- 拆法：`previous_records` 移出 `prompts`；造话术、调 pi、审从 `order/ai.rs` 移到适配层；`order` 只留走一步的判据与记账
- 判据：`src/prompts.rs` 与 `src/order/` 之间两个方向都不再出现 `crate::` 互引
- 已成：话术在 `prompts.rs`、造话术与调 pi、审在 `src/ai.rs`；`order/ai.rs` 已撤

## 站二 order → audit

- 现状：`order/execute.rs` 调 `crate::audit`
- 证据：`src/order/execute.rs:47`、`:48`、`:136`、`:137`
- 来路：`2cc2e41` 把机械判据的运行放进 `audit` 服务，`e689df7` 拆工单时 `order` 开始调它
- 拆法：把 `audit::check` / `run` / `items_of` 搬进 `order`，`audit` 只留资产审计
- 判据：`src/order/` 不再出现 `crate::audit`
- 已成：规则执行在 `src/order/execute.rs`；`audit` 只做资产表与工作区对账

## 站三 locate 文档

- 现状：`CONTRIBUTING.md`、`docs/dev-guide/index.md`、`docs/dev-guide/workspace.md`、`docs/dev-guide/work-order.md`、`STATUS.md` 仍写已不存在的 `locate/`
- 拆法：改成 `workspace` 容器事实
- 判据：五份文档里不再出现 `locate/`
- 时机：随站一改
- 已成：五份已改；`docs/user-guide/usecases.md` 里的 `locate` 是步骤名，不动

## 顺序

先站一，再站二，站三随站一。清理项（两处 `short`、事件负载、失效注释、`criterion` 名与类、契约测试清单）见[重构方案·模块拆分五至七](../../../../data/report/plan/qtcloud-work-cli.md)，逐条见 [TODO.md](TODO.md)。
