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

工作区身份落账本仓的 `workspace.yaml`，六个字段（`id` / `name` / `title` / `description` / `created_at` / `updated_at`）由 `model.rs` 的 `identity()` 拼出来——**模型造内容、装载写盘**。写动作前把账本开出来，身份缺则首跑生成（并落一条 `WorkspaceCreated`）；`id` 供凭证派生用——宁可落盘不可空算。只读动作不开账本，不在任何根上建文件。

## 三处位置的分工

**账本归 CLI、产物归工作区。** 账本是这台机器上的账，程序每跑一趟都写——落 XDG，不入版控；产物（报告与日志）是内容，跟着工作区走——落 `<根>/artifacts/`。混在一处，程序每跑一趟就往仓库写账本；分开放，两边各得其所。

## 路径显示与只读纪律

`short` 相对工作区根写短一点，不在根底下就原样；只读动作不开账本、不在任何根上建文件。装载与只读纪律的测试在 `tests/run_context.rs`；缺省矩阵（四个可省位置各缺一次）在 `tests/defaults.rs`。

## 结构

工作区聚合只装自己的东西：身份与字段在 `model`、领域事件在 `events`。碰盘的全在 `locate/`，别家的推导与核对各回各家（见下节）。

原先 `order` 与 `workflow` 要落盘，只能引 `workspace::Locate`；`workspace` 又要读工单与工作流来推导进度、核对定义——两条反向边让 `workspace ↔ order`、`workspace ↔ workflow` 两个聚合环以本聚名为枢纽。两次搬动把环从根上去掉：装载归 `locate/`，三件别家的事归各家。现在 `workspace` 一件也不引 `order` / `workflow`——环不必解，它不成立。

剩下 `locate ⇄ order` 是一对互认（`order_file` 收 `&WorkOrder`，order 落盘收 `Locate`），属装载层与聚合互认类型，不算聚合环。

## 归属：只装自己的事

判据一句话：**这件说的是谁的事？** 按这条尺子，三件别家的事曾寄住在 `workspace/`、两件本家的事曾落在装载层，都已归位：

| 件 | 说的是谁的事 | 现在住哪 | 出处 |
| :-- | :-- | :-- | :-- |
| 定义核对 `check` | 工作流定义写得对不对 | `workflow/check.rs` | `process/workflow.md`·约束：「定义须可对照工作区核对……核对由端侧执行」 |
| 流水判定 `progress` | 工单走过哪几步、走完没有 | `order/progress.rs` | `process/work-order.md`·约束：「工单没有状态字段：走到哪一步、走完没走完，由流水对照定义推导」 |
| 落点 `place` | 产物落在哪一格 | `artifact/place.rs` | `piece/artifact.md`·落点：「算式归产物，位置由工作区给」 |
| 身份生成 | 工作区自己的字段 | `workspace/model.rs` | `place/workspace.md`·领域属性 |
| 领域事件 | 工作区自己的事件 | `workspace/events.rs`（`WorkspaceCreated` 随首跑生成发出；`WorkspaceUpdated` 等改工作区信息的动作为止） | `place/workspace.md`·领域事件 |

新增东西先过这把尺子：说的是哪家的事，就住哪家。
