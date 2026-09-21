# 代码约定

横切约束集中在这一处。目录与文件怎么分、名字怎么取、依赖朝哪边走、单文件多长、测试怎么组织，都看这一份。

## 三类归类

`src/` 顶层按三类落位，判据是**有没有自己的定义**：

| 类 | 判据 | 落位 |
| :-- | :-- | :-- |
| **聚合** | 有自己的定义：身份、生命周期、字段规矩 | 一个聚合一个目录：`order/`、`workflow/`、`catalog/`、`artifact/`、`material/`、`workspace/` |
| **领域服务** | 没有自己的定义，跨聚合只做一件事 | `search/`、`audit/` |
| **适配** | 边界外的东西与入口 | `cli.rs`、`cli/`（子命令分派与发射）、`help.rs`、`prompts.rs`、`health.rs`、`locate/`（位置装载：根、账本、产物落点、身份与工单落盘）、`events.rs`（事件落盘） |

举例：`catalog` 有自己的定义（条目与快照语义），所以是聚合；`search` 只用 `catalog` 建的名字索引做一件事，所以是服务。`artifact` 是**定义型聚合**——它定的是有哪些标准产物、各落在哪，改它即改规范。

聚合件不与适配件同层：`src/` 顶层只剩目录、少量适配件、领域模型与 `main.rs`。

领域模型随聚合并回本仓：每个聚合目录里，模型在 `model.rs`（`order/`、`workflow/`、`workspace/`、`artifact/`），
读法与校验在 `read.rs`（`workflow/`），自己的领域事件在 `events.rs`（`order/`、`workflow/`、`workspace/` 各一件），
本聚合的规则与推导用独立文件（`order/` 的 `progress.rs`、`workflow/` 的 `check.rs`、`artifact/` 的 `place.rs`）。跨聚合的中立件在根下：`criterion/`（判据）与
`outcome.rs`（结果）、`error.rs`（定义错误）、`executor.rs`（执行者取值）、`paths.rs`（占位）、
`fields.rs`（字段表）。

## 命名

用能力名，不造 `-er`：`search` 而不是 `searcher`，`audit` 而不是 `auditor`。目录名用代码实际结构，不用抽象概念；有实体才建目录，但预留目录也是有效的架构声明。

## 依赖单向

服务可依赖聚合，聚合不得依赖服务；聚合与服务都不得依赖入口层（`crate::cli`）。`search → catalog` 是允许的，`catalog → search` 是零。这条有测试钉住（`tests/contract.rs` 的「动作层不依赖入口层」）。

装载层（`locate/`）另算：它不吃聚合的定义，聚合可以调它落盘——`order → locate`、`workflow → locate` 单向。反过来它认聚合类型（`order_file` 收 `&WorkOrder`），于是 `locate ⇄ order` 是一对互认，不是聚合环。聚合之间的环已经不存在：`workspace ↔ order` 与 `workspace ↔ workflow` 两条都从根上去掉了——装载归 `locate/`、别家的事归各家，`workspace` 一件也不引。

新增一件东西先问一句：**它说的是谁的事？** 谁的事住谁家；判据与逐件去处见 [docs/dev-guide/workspace.md](docs/dev-guide/workspace.md)·归属。

## 单文件 ≤250 行

任一件超阈即触发转聚合：把长文件按**事**拆成同目录多件，`mod.rs` 只留类型与出口。拆的时候行为不变——命令面、输出信封（`ok` / `lines` / `columns` / `rows` / `data`）、报错文字都不许改。

## 与 studio 对表

cli 与 studio 是同一件事的两侧，算出的结果要能对得上。对表只比 `ok` / `columns` / `rows` / `data` 四样，不比 `lines`——`lines` 是给人看的那一栏，两侧措辞与排版可以不同。

## 测试按用例组织

测试不是按模块组织的，是按**用例**组织的：`tests/` 下按用例切文件（`agent_step` / `state_machine` / `task_start` …），共用夹具在 `tests/common/`。每个测试上面一行 `// 用例：<号>`，使用指南 `docs/user-guide/*.md` 里标题形如 `## 用例 <号>、…`；两边的用例号集合必须相等，由 `scripts/validate-usecases.sh` 对账。

这与「`src` 与 `tests` 不同构」是故意的——用例即切口，不是违规，别按模块去「修正」它。
