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

搬文件、改路径一律**行为不变**：命令面、输出信封（`ok` / `lines` / `columns` / `rows` / `data`）与报错文字都不许改。命令面与信封既没动，studio 侧无需对表。

## 拆 workspace 枢纽环

方案与理由见 [docs/dev-guide/workspace.md](docs/dev-guide/workspace.md)·拆环；现状数字见 [STATUS.md](STATUS.md) 整改优先级第 3 条。目标：`workspace ↔ order`、`workspace ↔ workflow` 两个聚合环消失——办法是把「碰盘的那半」从聚合里搬出去，逻辑一行不改。

- [ ] **一 · 建装载模块，搬到新家**：新建 `src/locate/`（模块名暂用，`platform/` 之类亦可），把碰盘的那半从 `workspace` 搬过去——`Locate` 结构体（`src/workspace/locate.rs`，连 `ensure` / `workspace_id` / `workorders_dir` / `order_file` / `artifact_path` 等方法）、同一件的 `write_yaml`、`repo_root()`，以及 `src/workspace/mod.rs` 里的 `root()` / `workspace_key()` / `account()`。
  - 留在原地的：`model` / `place` / `progress` / `check`——调用方写的是 `crate::workspace::{check, progress}`，路径不动。
  - 判据：`Locate` 只在 `src/locate/` 定义；`grep -rn "std::fs\|serde_yaml" src/workspace/` 为空。
- [ ] **二 · 调用方改路径**：`Locate` 八件——`order/`（`mod` / `journal` / `inspect` / `actions` / `ai`）、`workflow/`（`mod` / `actions`）、`events.rs`；另 `order/mod.rs` 里 `crate::workspace::locate::write_yaml` 一句跟改，`root()` 的调用点在 `cli/handlers/workspace.rs`（四处）。
  - 判据：`grep -rn "crate::workspace::Locate\|workspace::root\|workspace::locate" src tests` 为空；通用判据全绿。
  - 注：本步与第一步同批落地——旧路径一删，调用方不改就编译不过。要分两次提交，先在 `workspace/mod.rs` 留一行 `pub use crate::locate::Locate;` 转发，改完再删。
- [ ] **三 · workspace 收成纯领域**：`mod.rs` 只留出口（或直接并进 `model.rs`），目录里剩 `model` / `place` / `progress` / `check` 四件。
  - 判据：`ls src/workspace/` 只有这四件（加壳）；`grep -rn "crate::locate" src/workspace/` 为空。
- [ ] **四 · `short` 挪 cli**：`workspace/mod.rs` 的 `short()` 是给人看的路径显示，归适配，不属工作区聚合——挪去 `cli`（落哪一件落地时定），改六处引用：`search/mod.rs` / `audit/mod.rs` / `workflow/actions.rs` / `order/{inspect,actions}.rs` / `cli/handlers/command.rs`。
  - 判据：`grep -rn "workspace::short" src tests` 为空；通用判据全绿。
- [ ] **五 · 可选，单独一轮：`order_file` 去聚合化**：签名从 `(&self, order: &WorkOrder)` 改成收 `&str`，调用方传 `order.name`（`artifact_path` 已是收字符串，不动）。改完 `locate` 只依赖 `ids` / `fields` / `clock` 这类小件，连 `locate ⇄ order` 这条互引也消掉。
  - 判据：`grep -n "crate::order" src/locate/` 为空。

## 连带文档（随上面同批改）

- [ ] **`CONTRIBUTING.md` 三类归类**：`workspace/` 不再挂 `locate.rs`；装载模块补进三类表——按判据「有没有自己的定义」它属**适配**（碰盘、不入聚合），落位一并写定。
- [ ] **`docs/dev-guide/index.md` 落点图**：`workspace/` 那行从「装载与身份 locate / 落点 place / 核对 check / 流水判定 progress」改为纯领域四件，并列装载模块一行（根、账本、路径与 `ensure`）。
- [ ] **`STATUS.md` 复检**：本批完成后重体检，两个聚合环应消掉；`order → audit` 与 events 虚环两条不在本次范围，仍在账上。
