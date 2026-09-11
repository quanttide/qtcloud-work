# 测试与门禁

五类测试与四条门禁命令（与 CI 同参数）。

| 类 | 文件 | 挡什么 |
| :-- | :-- | :-- |
| 场景 | `tests/task_start.rs` 等六个 | 用例里的真事：一个场景一个文件，出处 `// 用例：N` 与使用指南对账 |
| 状态真值表 | `tests/state_machine.rs` | 状态从流水推出来的那些边界：审查 ✗、重走通过、乱序、名字不在定义里 |
| 判据矩阵 | `tests/criteria_matrix.rs` | 谁执行（agent / human）× 怎么走（`--next` / `--done`）× 判据三类 |
| 缺省矩阵 | `tests/defaults.rs` | 三个可省位置（`--root` / `--data` / `--workflows`）各缺一次的行为 |
| 契约快照 | `tests/contract.rs` | 子命令清单、`--json` 字段、依赖方向（动作层不依赖入口层） |

定义本身也有核对：`qtcloud-work workflow <名字> --check` 查判据里的路径在不在、描述提到的小节有没有判据覆盖，`tests/definition_check.rs` 守着它。

约定只有写成脚本或测试才会红：写进注释、写进 description 的，迟早会漂。加测试的时机是「刚被咬过一次」——今天每一次翻车都补了一条。
