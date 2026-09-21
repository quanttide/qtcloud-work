# 状态：cli 依赖结构体检

体检日 2026-09-21；同日在拆 workspace 枢纽环落地后复检一次，下面为复检后的数。对象为 src/cli（约 5 000 行、21 个模块），对照 [CONTRIBUTING](CONTRIBUTING.md) 的分层与依赖规则。

## 规模

| 项 | 数 |
|---|---|
| 模块数 | 21（较上次 +1：装载从 workspace 分出，独立成 `locate/`） |
| 总行数 | 约 5 000 |
| 扇入 Top | locate 12、outcome 11、workflow 9、criterion 8、executor 7——workspace 从 15 落到 4 |
| 贴 250 行红线的文件 | material 240（全库最大）、workflow 234、locate 229、order 214 |

## 结构契约

| 条款 | 现状 | 判定 |
|---|---|---|
| 聚合不得依赖领域服务 | `order → audit`（`order/execute.rs` 两处 import），证据确凿，且无任何手段拦截 | ✗ 违规 |
| events 不反向引用聚合 | events 载荷引用 order、workflow 聚合类型，形成「聚合 → infra → 聚合」的层间虚环 | ✗ 违规 |
| 聚合间单向依赖 | 两个聚合环已消：装载那半搬进 `locate/` 后，`order → locate`、`workflow → locate` 单向，`workspace → order` / `workspace → workflow` 保留（设计意图）；枢纽随之解除，workspace 扇入 15 → 4（见 [docs/dev-guide/workspace.md](docs/dev-guide/workspace.md)·结构） | ✓ 已消 |
| 单文件行数越界即触发转聚合 | 全库达标；但贴线四件中三件是 mod.rs，正变成重编汇集点，material/mod.rs 预计第一个破线 | ✓ 贴线 |

## 治理现状

| 条款 | 现状 | 判定 |
|---|---|---|
| 架构规则测试化 | tests/contract.rs 只钉住入口层方向——唯一的违规恰好落在唯一没有测试覆盖的方向上；再写一条违规同样无人发现。违规是症状，「无测试钉住」才是漏洞：规则只存在于人的记忆里 | ✗ 缺 |
| 规则文档与现实一致 | CONTRIBUTING 的归类表已跟改（`locate/` 入适配、装载层依赖写清），但仍说 `task/`、`agent_step`、`task_start`，现实已是 `order/`、`order_start`。规则文档半新半旧 | ✗ 漂移 |

## 结构观察

- 扇入热点健康：outcome、criterion 作为跨切面被广泛依赖，属正常；locate 12 是新热点，同属这一类——装载层被各动作共用，是它的本分；
- material 孤立：仅 1 个 dependents（cli）却占 240 行、全库最大——要么被低估（应被更多模块复用），要么被高估（臃肿而无人用），值得单独复审；
- `workspace/model.rs` 是死件：没有任何 `mod` 声明（不参与编译），内容还引着改名前就删掉的 `crate::task::model::Task`。留还是删待定，不在本次范围。

## 整改优先级

1. 架构规则测试化：扫描 `use crate::` 做断言（grep 方法论已有），把「聚合不得依赖服务」「events 不反向引用聚合」变成 CI 里会红的断言；
2. 更新 CONTRIBUTING——命名对齐 order/ 现实（归类表已跟改，剩下 `task/` / `agent_step` / `task_start` 三处）；
3. 拆 workspace 枢纽环——**已办**：装载那半搬进 `locate/`，两个聚合环消掉；剩下 `locate ⇄ order` 的互认留在 [TODO.md](TODO.md) 第五步（可选）；
4. 复审 material。

一句话总结：结构风险从「成环」降到治理——数据层已健康（分层成立、环消掉、单文件纪律良好），剩下的漏洞是规则没有测试钉住、文档命名半新半旧。
