# workspace：工作区

工作区是一次工作的边界：一套工作流定义与上下文内的工件绑在一起。领域那一侧只认内容（定义与工单），不认位置；物理位置由平台在装载时给（规范 `docs/specification/place/workspace.md`）。

## 装载

`crate::locate` 的 `Locate` 把启动参数落成三处位置，装载顺序写死：命令行 > 环境变量 `QTCLOUD_WORK_ROOT`（只管根）> 缺省规矩。

- **根（root）**：判据路径的基准、`run` 判据的工作目录、工作区级动作扫描的面。缺省从当前目录往上找含 `data/journal` 的第二大脑，找不到就用当前目录；
- **账本（data）**：工作区身份、工单、事件。缺省 `$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/`，键由根派生（可读名加短码）——账本是「这台机器上的这个工作区」的账；
- **产物（artifacts）**：缺省 `<根>/artifacts/`；
- **工作流目录（workflows）**：缺省跟在账本里，定义常是固定资产，用 `--workflows` 另指。

**位置不进模型**：这些都不写进工单文件，换机器、挪仓库，重新装载即可。

## 身份

工作区身份落账本仓的 `workspace.yaml`：`id` / `name` / `title` / `description` / `created_at` / `updated_at`。写动作前把账本开出来，身份缺则首跑生成；`id` 供凭证派生用——宁可落盘不可空算。只读动作不开账本，不在任何根上建文件。

## 三处位置的分工

**账本归 CLI、产物归工作区。** 账本是这台机器上的账，程序每跑一趟都写——落 XDG，不入版控；产物（报告与日志）是内容，跟着工作区走——落 `<根>/artifacts/`。混在一处，程序每跑一趟就往仓库写账本；分开放，两边各得其所。

## 模型与三件操作

| 操作 | 做什么 |
| :-- | :-- |
| 落点 `place` | 产物落在产物落点的哪一格 |
| 核对 `check` | 判据写下的路径在不在区内、描述点到的小节有没有 `contains` 覆盖（只看写下的位置，不访问文件系统；占位运行时展开，不核） |
| 流水判定 `done_steps` / `next_step` / `finished` | 拿流水对照定义逐站对账（见 [work-order](work-order.md)·推导） |

路径怎么显示给人看：`short` 相对工作区根写短一点，不在根底下就原样。

## 测试

装载与只读纪律在 `tests/run_context.rs`（向上搜索、环境变量、指哪落哪、只读不落盘）；缺省矩阵在 `tests/defaults.rs`——四个可省位置各缺一次的行为。

## 结构

装载与领域分住两处：碰盘的那半归 `locate/`（根、账本、产物落点、身份与工单落盘、路径的短显示），聚合里只剩纯领域——`model` / `place` / `progress` / `check`。

为什么这么切：`order` 与 `workflow` 要落盘，只能引 `workspace::Locate`；`workspace` 又要读工单与工作流来推导进度——两条反向边让 `workspace ↔ order`、`workspace ↔ workflow` 两个聚合环以本聚名为枢纽（体检见 [STATUS](../../STATUS.md)）。装载搬出去之后，`order → locate`、`workflow → locate` 单向，而 `workspace → order` / `workspace → workflow` 保留——工作区持有工单、推导进度本来就是设计意图，这条边不该断。

剩下 `locate ⇄ order` 是一对互认（`order_file` 收 `&WorkOrder`，order 落盘收 `Locate`）：属装载层与聚合互认类型，不算聚合环。要连它一起消，得让 `order_file` 改收 `&str`——记在 [TODO](../../TODO.md) 第五步（可选，单独一轮）。
