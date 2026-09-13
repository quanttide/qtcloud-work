# 路线图

这一份是给人看的：为什么做、做到哪、怎么算做完。细节改哪些文件见 [TODO.md](TODO.md)，逐条体检数据见 [STATUS.md](STATUS.md)。

## 为什么做

cli 已经能用也能发：命令面四组齐（工作区 / 工作流 / 任务 / 导览探活）、三轴文档齐、**用例对账**（`validate-usecases.sh` 把文档用例号与测试出处两边对平）与**发布纪律**（版本与 CHANGELOG 校验）都在——这两件比 studio 那一侧做得干净。

但按软件工程契约量一遍，`src/` 是**平的**：11 个文件一层摊开，聚合件（`task` `workflow` `material` `catalog`）、服务件（`catalog` 里的 search、`audit`）、适配件（`cli` `help` `prompts` `paths`，还有住在 `cli.rs` 里的 `health` 与工作区落点）**同层并列**。后果直接可见：**`task.rs` 涨到 945 行**、`cli.rs` 541 行、`workflow.rs` 459 行——没有边界的地方，谁都往里加。

契约的终态是分聚合——**每个聚合就是一个可独立验证的小软件**：结构给验证单元、用例给判据、人审一个聚合而不是审整个工具。平铺一层做不到这件事。

## 目标

1. **cli 成为契约下的可独立验证单元**：按聚合分目录、服务与适配各归其位、单文件 ≤250 行
2. **对外对齐**：依赖许可清单、两侧工具库版本同步、IaC 归属
3. **文档与实现同构**：`docs/dev-guide/layers.md` 重写成三类的落点图

## 交付哪些模块

**聚合**（有自己的定义：身份、生命周期、字段规矩）

| 目录 | 管什么 | 现状 |
|---|---|---|
| `task/` | 任务：状态推导、走一步、流水、报告、日志 | 已有（`task.rs`，945 行待拆） |
| `workflow/` | 工作流：YAML 读写、严格 schema、五个动作 | 已有（`workflow.rs` 459 行待拆） |
| `catalog/` | 目录：扫描成名字索引（条目与快照语义是它自己的定义） | 已有（`catalog.rs` 276 行） |
| `artifact/` | 资产表：**标准产物定义**（`journal`、`report`、`profile` … 各叫什么、落在哪、依哪一条）——显式定义的领域模型，定义即规范 | 已有（`artifact.rs` 147 行） |
| `material/` | 材料：类型 / 内容 / 来源 / 时间，阶段由位置承担 | 已有（`material.rs` 240 行） |
| `workspace/` | 工作区：三处位置与落点 | **未立**——`workspace_root` / `data_dir` 现在散在 `cli.rs`，`paths.rs` 只管路径显示 |

**领域服务**（没有自己的定义，跨聚合只做一件事）

| 目录 | 管什么 | 现状 |
|---|---|---|
| `search/` | 按名找文档（用 `catalog` 的名字索引；先精确、不中再模糊） | 现名 `find`，住在 `catalog.rs` 里，待分出 |
| `audit/` | 审计资产表与工作区 | 已有（`audit.rs` 147 行） |

**适配**（边界外的东西与入口）：`cli.rs`（入口与发射）、`help.rs`、`prompts.rs`、`paths.rs`、`health`（现住 `cli.rs`，待分出）。

`artifact` 是**定义型聚合**：它不描述某个对象长什么样，它定的是「有哪些标准产物」（`journal`、`report`、`profile` …）与各自的落点——**改它即改规范**（文档面同源：work 域 handbook 的产物规范、gallery 的案例、`.quanttide/*/contract.yaml`）。

## 五段怎么走

| 段 | 做什么 | 怎么算完 |
|---|---|---|
| 一 · 正名与归类 | `find` → `search`（与 studio 同批）；把三类归类写下来 | 命令面与文档里只剩 `search`；归类的判据成文 |
| 二 · 立目录 | 聚合 / 服务 / 适配分家；`workspace` 立聚合，`search`、`health` 各自分出 | 聚合件不再与适配件同层；`search → catalog` 单向 |
| 三 · 拆长文件 | `task.rs` 945、`cli.rs` 541、`workflow.rs` 459 拆开；约定集中一处 | 无一件 >250 行；**行为不变**（测试与用例对账全绿） |
| 四 · 契约对齐 | 两侧工具库版本同步、依赖许可清单、IaC 归属 | 每条各有判据，见 TODO 段四 |
| 五 · 文档对齐 | `layers.md` 重写、api-references 分命令与概念、`prompts` 两侧说法一致 | 文档结构与 `src/` 一一对得上 |

段一是零风险的整理，先做；段二、三是一件事的两半（分家必拆文件），同一批做完。

**每段做完都跑这几条**（缺一不算完）：

```bash
cargo fmt --check && cargo clippy --locked && cargo test --locked
sh scripts/validate-usecases.sh
```

## 现在在哪

**段一（正名与归类）、段二（立目录）、段三（拆长文件）已完成**（2026-09-12，pi 执行 + 复核）：

- `src/` 顶层已是三类：聚合 `task/ workflow/ catalog/ artifact/ material/ workspace/`、服务 `search/ audit/`、适配 `cli.rs` `cli/` `help.rs` `prompts.rs` `health.rs`
- 最长文件 240 行（原 `task.rs` 945 行 → `task/` 内 `ai / execute / journal / report / state`）
- `find` → `search` 已在 cli 与 studio 两侧同批改完；约定落进 `src/CONVENTIONS.md`
- 四条门禁全过（`cargo fmt --check`、`cargo clippy`、`cargo test --locked`、`validate-usecases.sh`）；与 studio 的全量对表 **24/24 一致**

**剩下**：段四（依赖许可清单、IaC 归属；4.1 版本同步已做）、段五（`layers.md` 重写、api-references 分两类、`prompts` 两侧说法一致、用例对账去空转）。

**端侧适配已做**（2026-09-12）：工具箱两侧同号发到 `0.1.0-beta.6`，cli 与 studio 同批引这一号，并把两次破坏性变更改到位（`RunContext` 退出、落点/流水判定/核对归工具箱的工作区聚合、`Outcome` 不可变、判据占位展开收成一套）。四条门禁全过，与 studio 的对表 **10/10 一致**。

**三处已知欠账**（不阻塞，但要记着）：

- `docs/dev-guide/layers.md` 仍是旧说法（还提 `outcome.rs`、还写"拟建"）——段五 5.1
- `validate-usecases.sh` 目前空转（文档侧没有用例号，脚本靠跳过通过）——段五 5.5
- **类别当成了名字**：规格新立「产物类别」（`report` / `journal` 是**类别**，一件产物的**名字**如《量潮知识工作报告》），工具箱的 `Artifact` 只认名字+规格；本侧 `Task::artifact(kind)` 仍把类别当产物名传给 `Artifact::named`（只拿来算落点）。平台按新定义重理「类别 / 名字」两轴时一并改。

**端侧适配留下的三处**（2026-09-12 跟到工具箱 `0.1.0-beta.6` 时发现，不阻塞）：

- **流水落点没有测试**：`Task::artifact("log")` 应当等于任务文件本身。这轮第一版按 `Workspace::place` 算成了 `artifacts/log/<任务>.md`——**是 studio 那条测试逮住的，本侧没吭声**；补一条断在这。
- **借一个空工作区**：工具箱的 `place` / `expanded` / `check` 不吃 `self`，这一侧只能 `Workspace::default()` 现造一个才调得动。等工具箱把这三件收成关联函数（`Workspace::place(任务, 产物)`），删掉这一行。
- **`name` 与文件名不一致时无人挡**：`Workflow::of(值)` 的名字只取 `name` 字段，工具箱的 `validate` 拿不到文件名——定义里两者不一致时，工作区按名字取定义会**静默取不到**（流水判定返回空）。要么工具箱的 `validate` 收文件名核一致，要么本侧 `load` 里挡；这是规范层面的取舍，得先定规矩。

## 与 studio 的关系

两条命令面是一件事的两侧，**改名与版本必须一起走**：

- `find` → `search`：两侧同批改（cli 侧还有 `docs/api-references/find.md`）
- `quanttide-work` 版本：两侧已引同一个号（`0.1.0-beta.6`）——新工具箱再有破坏性变更时同批改，引完跑一次 `parity.sh`
- `prompts`：两侧各有一份，头注释互相打脸，要合一或写明差异
