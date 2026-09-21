# 状态：cli 依赖结构体检

体检日 2026-09-21。当日两次复检：先记拆 workspace 枢纽环（装载搬进 `locate/`），再记归位（别家的事各回各家）。下面为第二次复检后的数——对象为 src/cli（约 5 000 行、24 个模块），对照 [CONTRIBUTING](CONTRIBUTING.md) 的分层与依赖规则。

## 规模

| 项 | 数 |
|---|---|
| 模块数 | 24（顶层目录 11 + 顶层件 13，不计 `main.rs`） |
| 总行数 | 约 5 000 |
| 扇入 Top | locate 15、outcome 11、workflow 9、criterion 8、executor 7——workspace 从 15 一路落到 1 |
| 贴 250 行红线的文件 | material 240（全库最大）、workflow 236、locate 220、order 216 |

## 结构契约

| 条款 | 现状 | 判定 |
|---|---|---|
| 聚合不得依赖领域服务 | `order → audit`（`order/execute.rs` 两处 import），证据确凿，且无任何手段拦截 | ✗ 违规 |
| events 不反向引用聚合 | infra 只认 `Locate` 与 JSON，事件名与负载形状各归各聚合（`order/events.rs`、`workflow/events.rs`、`workspace/events.rs`）——原先的「聚合 → infra → 聚合」虚环消掉 | ✓ 已消 |
| 聚合间单向依赖 | 两个环不再存在：装载归 `locate/`、别家的事归各家，`workspace` 一件也不引 `order` / `workflow`（扇入只剩 1 处：装载调它造身份） | ✓ 不成立 |
| 单文件行数越界即触发转聚合 | 全库达标；贴线四件里三件是 mod.rs，仍在往重编汇集点走，material/mod.rs 预计第一个破线 | ✓ 贴线 |

## 治理现状

| 条款 | 现状 | 判定 |
|---|---|---|
| 架构规则测试化 | `tests/contract.rs` 的清单已扩到全部非入口件（42 件），但仍只钉「不依赖入口层」这一个方向；上表那两条真违规（聚合依赖服务、规则只活在人的记忆里）照样无断言 | ✗ 缺（范围收窄） |
| 规则文档与现实一致 | CONTRIBUTING 的 `task/` 已对齐 `order/`（归类表、领域模型段）；测试组织形式那一节里仍写着 `agent_step` / `task_start` | ✗ 漂移（剩一处） |

## 结构观察

- 扇入热点健康：outcome、criterion 作为跨切面被广泛依赖；locate 15 是装载层被各动作共用，是它的本分；
- workspace 扇入 1：只被装载调用一次（造身份），不再被任何动作直接引用——「边界聚合」该有的形态；工作区级动作走的是装载；
- material 孤立：仅 1 个 dependents（cli）却占 240 行、全库最大——要么被低估（应被更多模块复用），要么被高估（臃肿而无人用），值得单独复审；
- `workspace/model.rs` 死件**已处置**：原先没有任何 `mod` 声明（不参与编译）、内容还引着改名前就删掉的 `crate::task::model::Task`；已按现模型重写（身份六字段）。

## 整改优先级

1. 架构规则测试化：除了入口方向，把「聚合不得依赖服务」与「infra 不反向引用聚合」也变成 CI 里会红的断言（grep `use crate::` 的路子已有）；
2. 更新 CONTRIBUTING——测试组织形式那一节的 `agent_step` / `task_start` 对齐现实；
3. 拆 workspace 枢纽环——**已办**：两次搬动（装载归 `locate/`、三件别家的事归各家）把两个环从根上去掉；剩下的 `locate ⇄ order` 是装载层与聚合的互认，不算聚合环；
4. 复审 material。

一句话总结：结构风险只剩治理一条——分层与环都清了（装载独立、别家的事各归各家、infra 不认聚合），剩下的是那两条规则没有测试钉住、文档还有一处旧名。
