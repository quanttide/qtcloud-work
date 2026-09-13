# task：任务

任务（Task）是工作流的一次执行实例，落在 `<数据仓>/tasks/<名字>.yaml`。

## 落点

- `task/mod.rs`——类型与出入口：任务文件读写、产物落点（`artifact`）、闸门项、流水、运行上下文；
- `task/state.rs`——状态推导：走过哪些步、下一步、状态行；
- `task/execute.rs`——走一步：展开占位、跑判据、记流水、写闸门；
- `task/ai.rs`——交给 AI 的两段话与智能体审查；
- `task/journal.rs`——时间戳、日志叙事，以及 `log` / `report` / `journal` 三个常量；
- `task/report.rs`——动作：起任务、看状态、列任务、走一步、记日志；
- `task/progress.rs`——十格进度条。

## 数据

任务文件记 `start`、`workflow`、运行上下文（`root` / `data` / `workflows`）、`log`、`gates`、`artifacts`。

- 运行上下文随任务走：后续命令不写 `--root` / `--workflows` 就用记着的。
- 流水只增不改；状态从流水推出来，不另存字段。
- 闸门项（`gates`）是任务的状态，记在任务文件里。
- 产物落点由任务的 `artifacts` 声明，没声明就落草稿区；流水不是产物，`Task::artifact("log")` 是任务文件本身。
- 产物内容程序不写：报告与日志由人或智能体写，程序只备空骨架。

## 状态与走一步

一条流水算走过，要 `ok` 为真且步骤名在定义里；第一个没走过的步骤是下一步。推导算法在工具箱的工作区聚合（`Workspace::done_steps` / `next_step` / `state_line`），本侧只调，不各算各的。

```text
工作流定义 ──> 任务引用它 ──> 走一步
                              ├── agent：拼提示交 pi，回来核判据
                              ├── human：提示轮到你，等人 --done
                              ├── 跑 rule 判据、交 agent 审、列 human 闸门
                              └── 追加一条流水、写闸门
```

一步算过 = 这一步所有 `rule` 通过、所有 `agent` 判为通过；`human` 只列闸门，不影响过不过。交给 AI 跑的步骤，AI 没跑成即不过。人为记一步（`--done`）时 `agent` 判据算「待判」，不挡这一步。定义里的判据先换占位再跑，所以工作流不写死任务名。

## 测试

状态真值表在 `tests/state_machine.rs`——审查 ✗、重走通过、乱序、名字不在定义里这些边界；场景在 `tests/task_start.rs` / `tests/agent_step.rs` / `tests/human_step.rs`。
