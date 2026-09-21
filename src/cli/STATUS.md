# 状态：cli 依赖结构体检

体检日 2026-09-21。对象为 src/cli（约 4 700 行、20 个模块），对照 [CONTRIBUTING](CONTRIBUTING.md) 的分层与依赖规则。

## 规模

| 项 | 数 |
|---|---|
| 模块数 | 20 |
| 总行数 | 约 4 700 |
| 扇入 Top | workspace 15、outcome 12、workflow 9、criterion 8、executor 7 |
| 贴 250 行红线的文件 | material 240（全库最大），另有三件 mod.rs |

## 结构契约

| 条款 | 现状 | 判定 |
|---|---|---|
| 聚合不得依赖领域服务 | `order → audit`（`order/execute.rs` 两处 import），证据确凿，且无任何手段拦截 | ✗ 违规 |
| events 不反向引用聚合 | events 载荷引用 order、workflow 聚合类型，形成「聚合 → infra → 聚合」的层间虚环 | ✗ 违规 |
| 聚合间单向依赖 | workspace ↔ order、workspace ↔ workflow 两个聚合环，以 workspace 为枢纽——15 个 dependents 为全库最高扇入，workspace 已成事实上的公共类型集散地 | ✗ 成环 |
| 单文件行数越界即触发转聚合 | 全库达标；但贴线四件中三件是 mod.rs，正变成重编汇集点，material/mod.rs 预计第一个破线 | ✓ 贴线 |

## 治理现状

| 条款 | 现状 | 判定 |
|---|---|---|
| 架构规则测试化 | tests/contract.rs 只钉住入口层方向——唯一的违规恰好落在唯一没有测试覆盖的方向上；再写一条违规同样无人发现。违规是症状，「无测试钉住」才是漏洞：规则只存在于人的记忆里 | ✗ 缺 |
| 规则文档与现实一致 | CONTRIBUTING 还在说 task/、agent_step、task_start，现实已是 order/、order_start。违规无测试拦截加规则文档过期，架构约束实际失守 | ✗ 漂移 |

## 结构观察

- 扇入热点健康：outcome、criterion 作为跨切面被广泛依赖，属正常；
- material 孤立：仅 1 个 dependents（cli）却占 240 行、全库最大——要么被低估（应被更多模块复用），要么被高估（臃肿而无人用），值得单独复审。

## 整改优先级

1. 架构规则测试化：扫描 `use crate::` 做断言（grep 方法论已有），把「聚合不得依赖服务」「events 不反向引用聚合」变成 CI 里会红的断言；
2. 更新 CONTRIBUTING——命名对齐 order/ 现实；
3. 拆 workspace 枢纽环：共享值对象抽到 infra 或独立小模块，order 与 workflow 只单向依赖 workspace；events 载荷只携带 ID 与原始值，聚合回环整组随之消掉；
4. 复审 material。

一句话总结：数据结构是健康的（分层成立、单文件纪律良好），真正的风险不在代码而在治理。
