# workspace：工作区

工作区是一次工作的边界：把一套工作流定义与上下文内的工件绑在一起，只认内容、不认位置（规范 `docs/specification/place/workspace.md`）。件从哪来、落在哪，由端侧装载时定。

`src/workspace/` 装本模块：领域模型（`Workspace`）与三件操作（落点 / 核对 / 流水判定），加上端侧这一步——把命令行的位置落成实际路径，并决定路径怎么写给人看。

## 这一层做什么

- `root`——工作区根：给了 `--root` 就用它，没给用当前目录；
- `data_dir`——数据仓：给了 `--data` 就用它，没给用当前目录下的 `data/`，并把用的是哪个印到标准错误；
- `short`——路径显示：相对工作区根写短，不在根底下就原样。

工作流目录的默认值不在这一层，在 [`workflow`](workflow.md) 的 `workflows_dir`（跟在数据仓里）。

## 模型与三件操作

工作区的模型（`Workspace`）与三件操作在 `model` / `place` / `check` / `progress`：

| 操作 | 做什么 |
| :-- | :-- |
| 落点 `place` | 产物落在哪 |
| 核对 `check` | 判据引用的路径在不在、描述提到的小节有没有覆盖 |
| 流水判定 `done_steps` / `next_step` / `state_line` | 步骤走没走过 |

这三件都要拿整个工作区才做得了，所以归这一层；「在不在」由端侧判断——`workflow::check` 把 `exists` 传进去，这一层不读文件系统。

落点与核对不看工作区里装了什么，调用时用空工作区（`Workspace::default()`）即可；流水判定要在装着定义的工作区上算，用 `Task::workspace()`。

## 测试

缺省矩阵在 `tests/defaults.rs`——三个可省位置（`--root` / `--data` / `--workflows`）各缺一次的行为。
