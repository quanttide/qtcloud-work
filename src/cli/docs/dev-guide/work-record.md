# work-record：工作记录

工作记录是账上的一笔：哪一站、谁判的、过没过。流水内嵌在工单的 `records` 里，只增不改。

## 落点

- `order/record.rs`——流水纪律：读出与校验；
- `order/journal.rs`——产物落点常量（`REPORT` / `JOURNAL`）与人写的日志叙事；
- `events.rs`——领域事件落 JSONL；
- `order/ai.rs`——交给 AI 的两段话与智能体审查。

## 记账纪律

一笔八项：`id` / `seq` / `created_at` / `order_id` / `step` / `step_id` / `description` / `is_succeeded`。四条纪律，犯任何一条整本账拒读：

- **凭证不重**：同 `id` 即拒——复制的账；
- **页码自 1 起不跳号**：`seq` 严格递增——被抽走的账；
- **时刻不倒流**：`created_at` 倒序即拒——倒流的账不可信；
- **两处指认对得上**：`order_id` 认账，`step_id` 按 `step` 查填。

只增不改：旧记录原样在账上，以最新一条为准是推导，不是销毁。谁记账分三类——`rule` 机械比对、`agent` 智能体审查、`human` 闸门放行（出自 `order done`，程序不替人记）。AI 没跑成不记账：流水里是事实，不是失败记录。

## 领域事件

三件事各落一行 JSONL 到账本仓的 `events.jsonl`：`WorkflowCreated` / `WorkOrderCreated` / `WorkRecorded`。负载带工作区 `id`；工单事件带工单 `id` / `name` / `workflow_id` 与封面全文；记录事件带记录 `id` / `seq` / `step_id` 与全文。事件文件只增不改，去重是下游按 `id` 做的事。

## 测试

流水纪律在 `tests/records.rs`（同 id、跳号、倒流、旧账原样在）；事件在 `tests/events.rs`；凭证在 `tests/credentials.rs`；场景在 `tests/order_flow.rs` / `tests/material_intake.rs`。
