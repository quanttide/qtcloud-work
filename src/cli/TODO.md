# TODO：cli 待办

可直接动手的事项，逐条列、做完就勾。

每一条做完的通用判据（缺一不算完）：

```bash
cd apps/qtcloud-work/src/cli
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
sh scripts/validate-usecases.sh
sh scripts/validate-line-count.sh
```

搬文件、改路径一律**行为不变**：命令面、输出信封（`ok` / `lines` / `columns` / `rows` / `data`）与报错文字都不许改。命令面与信封既没动，studio 侧无需对表——附带一笔：studio 的 `scripts/parity.sh` 仍在用改名前的老命令 `task` 与老写法 `workflow --list`，对表本来就没跑通，那件事归 studio 的 TODO。

## 拆 workspace 枢纽环

方案与理由见 [docs/dev-guide/workspace.md](docs/dev-guide/workspace.md)·结构；现状数字见 [STATUS.md](STATUS.md) 整改优先级第 3 条。目标：`workspace ↔ order`、`workspace ↔ workflow` 两个聚合环消失——办法是把「碰盘的那半」从聚合里搬出去，逻辑一行不改。

- [x] **一 · 建装载模块，搬到新家**：新建 `src/locate/`，收下 `Locate`（连 `ensure` / `workspace_id` / `workorders_dir` / `order_file` / `artifact_path` 等方法）、`write_yaml`、`repo_root()`，以及 `root()` / `workspace_key()` / `account()`，外加 `IDENTITY` / `EVENTS` 两个常量。
  - 判据：`Locate` 只在 `src/locate/` 定义；`grep -rn "std::fs\|serde_yaml" src/workspace/` 为空。（两条均验过）
- [x] **二 · 调用方改路径**：`Locate` 十二件——`order/`（`mod` / `journal` / `inspect` / `actions` / `ai`）、`workflow/`（`mod` / `actions`）、`events.rs`、`search/mod.rs`、`audit/mod.rs`、`cli/handlers/{command,workspace}.rs`；`order/mod.rs` 里 `crate::workspace::locate::write_yaml` 一句跟改，`root()` 的四处调用点在 `cli/handlers/workspace.rs`。
  - 判据：`grep -rn "crate::workspace::Locate\|workspace::root\|workspace::locate" src tests` 为空；通用判据全绿。（均验过）
- [x] **三 · workspace 收成纯领域**：`mod.rs` 只剩三个子模块与 `check` 出口，目录里是 `model` / `place` / `progress` / `check` 四件。
  - 判据：`ls src/workspace/` 只有这四件加壳；`grep -rn "crate::locate" src/workspace/` 为空。（均验过）
- [x] **四 · `short` 挪出聚合**：落在 `locate/`，**不是 `cli`**——`search` / `audit` / `order` / `workflow` 都是它的调用方，挪进入口层会撞 `tests/contract.rs` 的「动作层不依赖入口层」；六处引用（`search/mod.rs` / `audit/mod.rs` / `workflow/actions.rs` / `order/{inspect,actions}.rs` / `cli/handlers/command.rs`）跟改。
  - 判据：`grep -rn "workspace::short" src tests` 为空。（验过）
- [ ] **五 · 可选，单独一轮：`order_file` 去聚合化**：签名从 `(&self, order: &WorkOrder)` 改成收 `&str`，调用方传 `order.name`（`artifact_path` 已是收字符串，不动）。改完 `locate` 只依赖 `ids` / `fields` / `clock` 这类小件，连 `locate ⇄ order` 这条互认也消掉。
  - 判据：`grep -n "crate::order" src/locate/` 为空。

## 连带文档（随上面同批改）

- [x] **`CONTRIBUTING.md`**：归类表把 `locate/` 记入适配；依赖单向补一段装载层——聚合可调它落盘（`order → locate`、`workflow → locate` 单向），`locate ⇄ order` 是互认而非聚合环。
- [x] **`docs/dev-guide/index.md` 落点图**：`workspace/` 一行改为纯领域，并列 `locate/` 一行；适配一节点出装载的所在。
- [x] **`docs/dev-guide/workspace.md`**：装载一节改指 `crate::locate`；原「拆环」提案一节改为「结构」——现状、为什么这么切、剩什么。
- [x] **`docs/dev-guide/work-order.md`**：落点清单里的 `workspace/locate.rs` 改 `locate/mod.rs`。
- [x] **`STATUS.md` 复检**：两个聚合环标为已消，扇入与贴线数刷新，新增 `workspace/model.rs` 死件一条。`order → audit` 一条不在本次范围，仍在账上。

## 归位：把别家的事还给别家

判断与依据见 [docs/dev-guide/workspace.md](docs/dev-guide/workspace.md)·归属，判据一句话：**这件说的是谁的事？** 三件别家的事曾寄住在 `workspace/`，两件本家的事曾落在装载层——都归了位。

- [x] **一 · `check` 归 `workflow/`**：搬到 `workflow/check.rs`；调用点只有一处（`workflow/actions.rs` 的 `workflow check`），改 `use super::check`。
  - 判据：`grep -rn "workspace::check" src tests` 为空；`workflow check` 输出不变，`tests/definition_check.rs` 全绿。（验过）
- [x] **二 · `progress` 归 `order/`**：搬到 `order/progress.rs`；调用点两处（`order/mod.rs` 改为同模块引用、`order/inspect.rs` 改 `use super::progress`）。
  - 判据：`grep -rn "workspace::progress" src tests` 为空；工单进度、下一步、状态行不变，`tests/state_machine.rs` 全绿。（验过）
- [x] **三 · `place` 归 `artifact/`**：规范先补一条——`piece/artifact.md` 新增「落点」节（算式 `<类别>/<工单名>.md`）并把口径改成「算式归产物，位置由工作区给」；实现搬到 `artifact/place.rs`，`Locate::artifact_path` 只拼目录。
  - 判据：规范里有这条算式；`grep -rn "workspace::place" src tests` 为空；产物落点不变，`tests/criteria.rs` 与 `tests/order_done.rs` 全绿。（验过）
- [x] **四 · 身份生成收回本家**：`workspace/model.rs` 按现模型重写（原内容是未参与编译的死件），`identity()` 造内容；`locate::ensure()` 只留写盘。判据：`workspace/` 里有身份生成、`locate/` 不再拼字段；首跑生成的身份逐字段相同。（验过，并顺带处置了那条死件）
- [x] **五 · 补工作区事件，事件定义随聚合**：根 `events.rs` 退回纯落盘（`append` 补公共三样 + `yaml_to_json`），事件名与负载回各家（`order/events.rs`、`workflow/events.rs`、`workspace/events.rs`）；补 `WorkspaceCreated`——首跑生成身份时发，规范要求它先于其内一切事件。`WorkspaceUpdated` 等改工作区信息的动作落地再定义（`workspace/events.rs` 注释写明）。
  - 判据：三个聚合各自带事件定义；`events.rs` 里不出现任何聚合类型（`grep -c "WorkOrder\|WorkRecord\|Workflow"` 为 0，只在注释里指路）；`tests/events.rs` 从三件事改钉四件事（含事件带的工作区 id 与身份里那枚一致）。（验过）
- [x] **收尾对表**：`docs/dev-guide/index.md` 落点图与适配节点、`work-record.md`·领域事件（三件→四件 + 定义/落盘分家）、`workspace.md`（结构 + 归属改为现状）、`workflow.md` / `work-order.md` / `artifact.md` 的待归位标注撤掉、`CONTRIBUTING.md`（归类表、领域模型段、装载层与归属判据）、`user-guide/workspace.md` 的账本树、`tests/contract.rs` 层清单扩到 42 件、`STATUS.md` 第二次复检。（均已改）

