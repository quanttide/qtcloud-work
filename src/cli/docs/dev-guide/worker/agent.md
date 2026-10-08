# AgentWorker：交给智能体走一步的执行器

`AgentWorker` 把工单的一步交给智能体——凑现场、起 `pi`、读回结论、判定、记账，落成一次走一步的结果。落点 `src/worker/agent.rs`。

## 它持有什么

只持工作区（`LocalWorkspace`，三处路径）。不持工单：`execute` 每次把 `Order` 传进来。不持判据：判据从传进来的 `Step` 现取。

## 它做什么

四步，一步一法：

- 组装现场 `build_context`：从工单与这一步凑出话术要的数据（工单名、描述、全部步骤、这一步做什么、两样产物路径、前几笔流水），并把这一步的判据占位展开；
- 调 AI `invoke_ai`：把话交给 `pi` 非交互跑一趟，收回原文与一句话；
- 判 `judge`：让智能体按 agent 判准逐条审，认回「说明 / 结论 / 理由」；
- 记 `record`：核 rule 判据、记账（走 `crate::order::execute`），成结果信封。

动作出口 `order_next`：开单、认下一步；轮到人做的站不抢着做，提示用 `order done`；其余交给 `execute`。

## 它不做什么

- 不直接改工单字段——记账经 `crate::order::execute`，由聚合自己的方法落；
- 不发领域事件——`WorkRecorded` 归聚合自己发；
- 不定判据怎么判——判据的模型与翻译在 `crate::criterion`，真去跑在 `crate::order::rules`；
- 不写话术——给智能体的两段话在 `crate::prompts`。

最要紧的一条不依赖：不碰 `crate::order::events`。事件归聚合，执行器只交结果。

## 测试

`tests/order_next.rs`——`pi` 跑成记账、跑不成不记、审查 ✗ 照记、重走追加；`tests/criteria_matrix.rs`——闸门站等人、agent 判不过、人的步骤。
