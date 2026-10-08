# pi：命令行 AI

`pi` 是这个程序要调用的那个命令行 AI。`adapters/pi.rs` 只干三件小事，跟工单、判据都无关——谁要调 AI 都能用它。

## 它做三件事

1. `run_ai` 起进程：把一段话交给 `pi -p --no-session` 跑，在工作区根下执行，收回标准输出（空则收回标准错误），以及成功与否。
2. `one_line` 收一行：把一段多行输出压成一行（取最后一行非空），再截到指定长度——给屏幕显示用。
3. `verdict_of` 认结论：从 `pi` 的回答里读第 `index` 条的结论。先找「序号. …」那一行，看它是不是以「不通过」或「通过」开头；找不到就在整段里兜底搜。都认不出就记「待判」。

## 谁在用

`workers` 的 `execute` 用 `run_ai` 执行、`one_line` 取摘要；`evaluate` 用 `run_ai` 审、`verdict_of` 认回每条结论（见 [agent](../workers/agent.md)）。

## 测试

`tests/order_next.rs` 里的 `pi` 是测试桩（`fix.pi(...)`），能造出「跑成」「跑不成」「答不通过」三种；`verdict_of` 的认法由 `tests/criteria_matrix.rs` 的 agent 判据覆盖。
