# workspace：工作区

工作区是一次工作的边界：一套工作流定义与上下文内的工件绑在一起。领域那一侧只认内容（定义与工单），不认位置；物理位置由平台在装载时给（规范 `docs/specification/place/workspace.md`）。

## 装载

`locate.rs` 的 `Locate` 把启动参数落成三处位置，装载顺序写死：命令行 > 环境变量 `QTCLOUD_WORK_ROOT`（只管根）> 缺省规矩。

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

## 拆环

### 背景

`workspace/` 一个目录下住了两种东西：只算不碰盘的领域逻辑（`model` / `place` / `progress` / `check`），和专门碰盘的装载逻辑（`locate.rs` 的 `Locate`，以及 `mod.rs` 里的 `root` / `workspace_key` / `account` / `short`）。

`order` 与 `workflow` 要的其实是后者——落盘、找目录；但引用时只能写 `use crate::workspace::Locate`，读起来就成了「聚合引用聚合」。而 `workspace` 的 `progress` / `model` / `check` 又反过来读工单与工作流。两条边互为反向，`workspace ↔ order`、`workspace ↔ workflow` 两个聚合环都以此为枢纽（扇入全库最高，见 [STATUS](../../STATUS.md)）。

### 选项

**前三步：把装载那半搬出去。**

1. 新建一个装载模块（下面暂称 `locate/`），从 `workspace` 搬进：`Locate` 结构体（连同 `ensure` / `workspace_id` / `workorders_dir` / `order_file` / `artifact_path` 等方法）、`root()`、`account()`、`workspace_key()`、`repo_root()`；
2. `workspace/` 删掉这些，只剩纯领域 `model` / `place` / `progress` / `check`——`mod.rs` 只留出口，或直接并进 `model`；
3. 调用方改路径：`use crate::workspace::Locate` → `use crate::locate::Locate`（`order/` 六件、`workflow/` 两件是主要调用方）。纯替换，逻辑一行不改。

**可选第四步：连 `Locate` 的签名也去聚合化。** `order_file(&self, order: &WorkOrder)` 现在认聚合类型，可改成收 `&str`（调用方传 `order.name`）——`artifact_path` 已经是收字符串，不用动。改完 `locate` 只依赖 `ids` / `fields` / `clock` 这类小件，成为纯粹的平台装载层。

拆前 / 拆后：

```text
拆前  order ─use Locate─→ workspace ─done(order)─→ order        两个环
     workflow ─use Locate─→ workspace ─model/check─→ workflow

拆后  order ──→ locate ←── workflow          装载模块，单向
     workspace ──→ order / workflow          纯领域，单向
     locate ⇄ order                          装载层与聚合互认类型，不算聚合环
```

### 影响

- 两个聚合环消失：`order → locate`、`workflow → locate` 单向；`workspace → order` / `workspace → workflow` 保留——工作区持有工单、推导进度本来就是设计意图，这条边不该断；
- `ensure()` / `repo_root` / `account` 从聚合归位装载层（`ensure` 本就是应用服务的行为），`short` 挪去 `cli`；
- 剩下 `locate ⇄ order` 仍是互引：`order_file` 要 `WorkOrder` 类型，order 落盘要 `Locate`。这是「平台装载层 ⇄ 聚合」，不是聚合之间互引，分层上允许；要连这条也消，只有第四步。

工作量：搬文件加一处全局替换，逻辑零改动。

### 建议

先做前三步——两个聚合环当场消失，不碰签名、风险最小；第四步等 `locate` 稳定后另起一轮。
