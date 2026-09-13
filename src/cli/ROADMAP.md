# 路线图

细则与判据见 [TODO.md](TODO.md)，体检数据见 [STATUS.md](STATUS.md)。

## 基线

以下已达成，不进阶段：

- 测试补缺：`tests/task_start.rs` 断流水落点等于任务文件本身；
- 文档与实现同构：`docs/dev-guide/layers.md` 对齐 `src/`；`docs/api-references/` 分 `commands/` 与 `concepts/`；用例对账非空且两边用例号一致；
- 依赖与门禁：许可清单在 `docs/dev-guide/dependencies.md`；`scripts/validate-line-count.sh` 超 250 行即红，已进 CI。

## 待决

跨仓库的决策见 [TODO.md](TODO.md) 的「待决事项」。

provider 的接口层等就位后另起一轮。
