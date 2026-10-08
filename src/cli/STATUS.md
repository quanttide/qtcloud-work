# 状态：cli 依赖结构体检

体检日 2026-10-08。对象为 src/cli（约 5 100 行、24 个模块），对照 [CONTRIBUTING](CONTRIBUTING.md) 的分层与依赖规则。上一次体检是 2026-09-21，那时装载还在 `locate/`；现在装载归 `workspace/local.rs`，`locate/` 已不存在。

## 规模

| 项 | 数 |
|---|---|
| 模块数 | 24（顶层目录 10 + 顶层件 14，不计 `main.rs`） |
| 总行数 | 约 5 100 |
| 扇入 Top | workspace 29、workflow 19、order 19、outcome 13、ids 10、events 10、criterion 9、artifact 9 |
| 贴 250 行红线的文件 | order/execute.rs 249（全库最大）、workspace/local.rs 241、material 240、workflow/mod.rs 238 |

## 结构契约

| 条款 | 现状 | 判定 |
|---|---|---|
| 聚合不得依赖领域服务 | `order → audit` 已拆：规则执行搬进 `order/execute.rs`，`audit` 只做资产表与工作区对账 | ✓ 已消 |
| 聚合与适配不得互引 | `order ⇄ prompts` 已拆：话术在 `prompts.rs`、造话术与调 `pi`、审在 `ai.rs`，聚合两件都不碰 | ✓ 已消 |
| events 不反向引用聚合 | 事件落盘只认 `LocalWorkspace` 与 JSON，事件名与负载形状各归各聚合（`order/events.rs`、`workflow/events.rs`、`workspace/events.rs`） | ✓ |
| 容器与内件的互认不算环 | `workspace` 是 `workorder` 的上级容器：装载认内件类型（`order_file` 收 `&WorkOrder`），内件落盘调装载（`order → workspace` 单向） | ✓ 不成立 |
| 单文件行数越界即触发转聚合 | 全库 ≤250；`order/execute.rs` 249 贴线 | ✓ 贴线 |

## 治理现状

| 条款 | 现状 | 判定 |
|---|---|---|
| 架构规则测试化 | `tests/contract.rs` 的「不依赖入口层」改成扫 `src/` 自动收集，新增文件即入检；聚合依赖服务、聚合与适配互引两条仍只靠人看 | ✗ 缺（范围收窄） |
| 规则文档与现实一致 | CONTRIBUTING 与 dev-guide 里的 `locate/` 已改成 `workspace` 容器事实，`order/ai.rs` 已改指 `ai.rs` | ✓ |

## 结构观察

- 扇入热点健康：workspace 29 是装载被各动作共用，是它的本分；outcome、criterion 作为跨切面被广泛依赖；
- `order/execute.rs` 249 行贴线，是全库最大件，再添一样东西先按事拆；
- material 孤立：仅 1 个 dependents（cli）却占 240 行、全库第三——值得单独复审。

## 整改优先级

1. 架构规则测试化：把「聚合不得依赖服务」「聚合与适配不得互引」也变成会红的断言；
2. `order/execute.rs` 贴线：再添东西前先拆；
3. 复审 material。

一句话总结：这一轮把两处依赖拆了（`order → audit`、`order ⇄ prompts`）、`locate/` 那套说法从文档撤了、契约测试改成扫目录；结构风险只剩治理一条——那两条规则还没测试钉住，`order/execute.rs` 贴线。
