# 重构待办

六段重构的主体已在工作区实施（尚未提交），来龙去脉见 [ROADMAP.md](ROADMAP.md)；本篇只剩收尾。每条写清**动哪儿**；提交按段原子进行，做完一段勾一段。

## 一 · 测试与门禁（先绿再提交）

- [x] **工作区选定接口**：装载顺序定为 命令行 > `QTCLOUD_WORK_ROOT` > 向上搜索——跨工作区干活或从外部操作时不必每条命令带 `--root`；动 `src/workspace/mod.rs`（`Locate::resolve` 的缺省链）与 `src/cli.rs`，`defaults.rs` / `run_context.rs` 的装载矩阵补「环境变量指哪就落哪」一行。
- [x] **补夹具**：`tests/common/mod.rs` 的 `Fixture` 补 `run_in`（指定目录与环境变量跑，装载矩阵靠它）、`run_ledger` 带上定义目录；另缺省摆一枚「不该被调到」的 `pi` 失败桩——测试忘摆桩就当场炸，绝不落到机器上的真 `pi`（曾致测试暗调真模型二十秒）。
- [x] **删旧测试**：`tests/task_start.rs` / `agent_step.rs` / `human_step.rs`——`order_start` / `order_next` / `order_done` 已接替，删除即消掉三处编译错。
- [x] **`run_context.rs` 整篇换**：位置不进模型后，「运行上下文随任务记着」不存在了，改成测装载——不给 `--root` 往上找 `data/journal`、账本缺省落 `$XDG_DATA_HOME/qtcloud-work/workspaces/<键>`、`--data` / `--artifacts` 指到哪就落哪；加一条「只读动作不在任何根上建文件」。
- [x] **`defaults.rs`**：缺省矩阵从三项变四项（加 `--artifacts`），断言换新缺省。
- [x] **`material_intake.rs` 跟改**：随夹具改名。
- [x] **`contract.rs` 跟改**：新动作与 `data` 字段变更要覆盖；「动作层不依赖入口层」那条包住新动作。
- [x] **新增 `tests/events.rs`**：三件事各落一行 JSONL，负载带工作区 `id`、工单 `id` / `name` / `workflow_id`、记录 `id` / `seq` / `step_id` 与全文；事件文件只增不改，去重是下游的事。
- [x] **用例增开三条**：流水只增不改（六）、凭证按名派生（七）、产物落点（八）——`docs/user-guide/usecases.md` 的 `## 用例` 与 `tests/*.rs` 的 `// 用例：<号>` 两边同增；现有五条改标题（任务 → 工单、人记一笔 → 闸门放行）；用例三（比对两份课程档案）另立 `order_flow.rs` 安家。`scripts/validate-usecases.sh` 核。
- [x] **门禁全绿**：`cargo fmt --check`（当前 `src/cli/commands.rs` 不过）→ `cargo clippy --all-targets --locked -- -D warnings` → `cargo test --locked` → `validate-usecases.sh` / `validate-line-count.sh`。

## 二 · 文档

- [x] **接口参考·动作面**：`docs/api-references/task.md` → `order.md`，动词式（`create` / `show` / `list` / `next` / `done` / `journal` / `delete`），每个动作写清参数、落盘与拒绝条件；`workflow.md` 的动作表同改，导出原样文件、导入撞名即拒两条保留。
- [x] **接口参考·输出契约**：`order` 的 `--json` 里 `data` 不得再有 `log` / `gates` / `start`；`workflow`（名字）与 `workflow_id` 分开；新动作 `delete` 的 `data` 与退出码写进契约（`--json` 字段只加不改、要改先加新留旧）。
- [x] **接口参考·全局选项**：`docs/api-references/index.md` 的 API 索引表（task → order）与全局选项跟改：加 `--artifacts`，`--data` 缺省改成 `$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>`，写明「位置不进模型，全部由启动参数装载」与环境变量的装载顺序（命令行 > `QTCLOUD_WORK_ROOT` > 向上搜索）。
- [x] **开发指南**：`docs/dev-guide/task.md` 拆成 `work-order.md`（封面与推导）与 `work-record.md`（记账与只增不改）；`workspace.md` 补工作区身份与三处位置（根 / 账本 / 产物）；`artifact.md` 的落点从「相对工作区根」改成「相对产物落点」；`index.md` 的落点图跟着改。
- [x] **规矩落座**：「定义里不抄别处拥有的事实（分类目录、落点、名字一律指过去）」写进 dev-guide 的 `workflow.md`；「账本归 CLI、产物归工作区」写进 `workspace.md`。
- [x] **使用指南**：`docs/user-guide/{index,task,workflow,workspace,usecases}.md` 的命令示例、全局选项说明与用例标题跟着改。

## 三 · 发布

- [x] **行数门禁**：`src/order/mod.rs` 214 行，已贴 250 行的线；加东西先看 `scripts/validate-line-count.sh`。
- [x] **CHANGELOG 与版本**：命令面 `task` → `order`、`data` 字段变更、缺省位置变更都是破坏性变更，记 `CHANGELOG.md`（`scripts/validate-changelog.sh` 核），版本与头一行一致（`scripts/validate-version.sh` 核）。
- [ ] **与 studio 对表**：新增动作（`order delete`）与改名字段（`data` 里 `log` → `records`、`start` / `gates` 去掉）在 studio 侧同步；两侧只比 `ok` / `columns` / `rows` / `data` 四样，不比 `lines`。
