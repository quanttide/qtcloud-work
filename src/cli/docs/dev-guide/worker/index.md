# worker：交给智能体的执行器

`src/worker/` 装的是把工单的一步交给智能体跑完的那件事：凑现场、起 `pi`、读回结论、判定、记账。它是适配层的一件，外边界是外部命令 `pi`。

## 落点

- `worker/agent.rs`——执行器 [`AgentWorker`](agent.md)：走一步的四个动作（组装现场、调 AI、判、记），外加动作出口 `order_next`；
- `worker/pi.rs`——起 `pi` 与读回：非交互跑一趟、把输出收成一行、从回答里认「序号. 通过 / 不通过」。

话术不在这一层：给智能体的两段话在 `prompts.rs`，执行器只把工单现场凑成它要的数据。

## 边界

`AgentWorker` 持工作区（路径），不持工单——`Order` 每次传入。它不直接改工单字段，记账走聚合自己的方法；也不发领域事件，`WorkRecorded` 归工单聚合自己发（见 [work-record](../work-record.md)·领域事件）。

依赖三样：`crate::prompts`（话术）、外部命令 `pi`、`crate::order::execute`（记账）。不依赖 `crate::order::events`，也不碰工单内部字段。

## 与 order 的关系

工单聚合不认这一层：`order/` 里既没有 `prompts` 也没有 `worker`，它只提供判据与记账（见 [work-order](../work-order.md)）。一步怎么走由 `AgentWorker` 编排，走完把结果交回聚合落账。

## 测试

走一步的场景在 `tests/order_next.rs` 与 `tests/criteria_matrix.rs`——AI 跑成 / 跑不成、审查过 / 不过、闸门站等人。
