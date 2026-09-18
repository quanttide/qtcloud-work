# 接口参考

签名、字段与落点的**接口参考**，一个命令一篇。怎么用、怎么走一遍看[使用指南](../user-guide/index.md)；程序分层看[开发指南](../dev-guide/index.md)。

## API 索引

| API | 做什么 |
| :-- | :-- |
| [search](search.md) | 按名找文档 |
| [catalog](catalog.md) | 按资产类别列条目 |
| [audit](audit.md) | 审计资产表与工作区 |
| [material](material.md) | 材料的四字段与阶段 |
| [workflow](workflow.md) | 工作流的定义与六个动作 |
| [order](order.md) | 工单的一次行程与七个动作 |
| [help](help.md) | 导览：按用途列出命令 |
| [health](health.md) | provider 探活 |

工作区的布局与资产表见[使用指南·工作区](../user-guide/workspace.md)。

## 全局选项

四个全局选项决定动作在哪跑，装载顺序写死，不许靠猜：命令行 > 环境变量 `QTCLOUD_WORK_ROOT` > 缺省规矩。位置不进模型，全部由启动参数装载。

- `--root <路径>` 工作区根，判据路径的基准。不写先看 `QTCLOUD_WORK_ROOT`，再从当前目录往上找到含 `data/journal` 的第二大脑，找不到就用当前目录，并把用的是哪个根印出来。
- `--data <路径>` 账本：工作区身份、工单、事件。不写落 `$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/`，键由工作区根派生（可读名加短码）——账本是这台机器上这本账，指到仓库就等于入版控。
- `--workflows <路径>` 工作流目录，缺省跟在账本里。定义常是固定资产，用这一项另指一处（如 `data/profile/iGuo/workflows/`）。
- `--artifacts <路径>` 产物落点（报告与日志），缺省 `<工作区根>/artifacts/`。产物是内容，不跟账本走。

## 输出与退出码

标准输出只放结果与 `--json` 的 JSON，管道里不混进别的东西；提示、进度与错误一律走标准错误。每个动作都支持 `--json`：`--json` 把结果以 JSON 输出到标准输出，`--out <文件>` 另存一份到文件，两个可以一起用。

成功退出码 0，失败非 0（`ok=False` 为 1）；失败时标准错误的最后一行给一句「下一步敲什么」。结果为 `ok=False`，`lines` 照常打印。

写入型动作（`workflow create` / `import`、`order create` / `next` / `done` / `journal` / `delete`、`audit --make`）都支持 `--dry-run`：把要写什么、写去哪打印出来，不落盘。`order next` 走一步时先打印一句「这一步交给谁」，交给 AI 的超时 900 秒。

## 契约与版本

`--json` 的字段与退出码是脚本依赖的契约，只加不改：要改先加新字段、后废旧的，旧字段先留一轮。`--version` 打印版本号，与 `CHANGELOG.md` 头一行、`scripts/validate-version.sh` 的校验一致。

## 结果与 JSON

每个动作算出一个结果，四样：`ok` 通不通、`lines` 命令行要打印的话、`columns` 与 `rows` 给窗口画的同一份表格、`data` 给窗口与脚本的结构化那一栏（可省）。命令行的 `--json` 与窗口都从这层取，算法只写一遍；`--out` 落的是 `data` 里的原文。
