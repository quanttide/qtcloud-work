# 重构待办

正式版 CLI 还停在 v1 的模型（`task` 带 `log` / `gates` / `artifacts`，工作流不带凭证，位置记进任务文件），而规格已换成「工作流 / 工作步骤 / 工单 / 工作记录」四篇；实验室 v2（`examples/default/apps/qtcloud-work-lab-v2`）按新规格跑通了真流程，把坑踩过一遍。

这一份是把那些经验落到正式版上的动手清单。每条写清**动哪儿**；踩过坑的附一句**为什么**（实验室里的原话）。

## 一、模型对表（规格为准）

- [ ] **命名先说**：规格里这一对叫「工单」（WorkOrder）与「工作记录」（WorkRecord），现目录与命令叫 `task`。名不副实，改成什么由你定；定下来与下面模型改造一并做。
- [ ] **`Task` 拆成工单与工作记录**：`Task`（`name` / `workflow` / `start` / `log` / `gates` / `artifacts`）→ `WorkOrder`（`id` / `name` / `description` / `workflow_id` / `created_at` / `records`）+ `WorkRecord`（`id` / `seq` / `created_at` / `order_id` / `step` / `step_id` / `description` / `is_succeeded`）。动 `src/task/model.rs`、`src/task/journal.rs`、`src/fields.rs`。
  为什么：规格已无 `task` 篇，代码注释还引 `specification/process/task.md`——那篇已被 `work-order.md` / `work-record.md` 换掉。
- [ ] **删 `gates` 字段**：闸门就是定义里 `human` 判据的所在站，由定义加流水推导，不落字段。动 `src/task/model.rs`、`src/task/state.rs`、`src/task/report.rs`。
- [ ] **凭证分工**：定义（工作流、步骤）**不写 `id`**，按「工作区 id + 名字」现算（uuid5，命名空间钉死，各平台算得一样）；工单与工作记录的凭证由程序发。新增 `src/ids.rs`；`src/fields.rs` 的字段表不动——旧定义照旧读得进。
  为什么：实验室里把 v1 的定义翻成 v2，第一件事是给每条工作流、每个步骤凭空编 UUID，纯手写负担；改成按名派生后，`data/profile/iGuo/workflows/*.yaml` 一个字没改就读进来、check 得过、起得了单。
- [ ] **流水只增不改**：追加幂等（同 `id` 即拒）、`seq` 自 1 起严格递增不跳号、`step_id` 按 `step` 查填、时间倒序即拒。新增 `src/task/record.rs`。
- [ ] **封面落笔即封**：`workflow_id` 落笔即封、请求带了即拒；有账不销——流水非空的工单不可删，白纸（没动工的）可删。动 `src/task/state.rs`，补一个删白纸的动作。
- [ ] **进度与完结只推导**：`src/workspace/progress.rs` 现在按 `·审` / `·判` 后缀投票、重走从头算——改成拿 `records` 的 `step` 名序列对着定义的 `steps` 逐站对账；带 `human` 判据的闸门站，通过须出自人。
- [ ] **工作区身份**：新增 `workspace.yaml`（`id` / `name` / `title` / `description` / `created_at` / `updated_at`），落账本仓；`src/workspace/model.rs` 补身份，`id` 供派生用。

## 二、位置（实验室 v2 的经验）

- [ ] **账本归 CLI**：`--data` 缺省改成 `$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/`，键由工作区根路径派生（可读名 + 短码）；`--data` 指到仓库就等于把账本入版控。动 `src/cli.rs`、`src/workspace/mod.rs`。
- [ ] **产物不进账本**：新增 `--artifacts`（缺省 `<工作区根>/artifacts`）；`src/workspace/place.rs` 的落点从「相对工作区根」改成「相对产物落点」。动 `src/workspace/place.rs`、`src/cli.rs`。
  为什么：账本是这台机器上的账，报告与日志是内容。混在一处，程序每跑一趟就往仓库写账本；账本挪走、产物留下，两边各得其所。
- [ ] **位置不进模型**：删掉工单文件里的 `root` / `data` / `workflows` 三个字段（`src/task/state.rs` 的 `context()`），位置一律由启动参数装载。动 `src/task/state.rs`、`src/task/journal.rs`。
  为什么：v1 把位置记进任务文件，换机器、挪仓库就断；规格已定位置由平台装载、不进模型。

## 三、定义

- [ ] **落点引用**：判据里的 `{{report}}` / `{{journal}}` 保留，展开按「产物落点 + 工单名」算；`log` 这个占位随流水字段一起删。动 `src/paths.rs`、`src/workspace/place.rs`。
- [ ] **定义里不抄别处拥有的事实**：工作流描述里不枚举会变的表（分类目录、落点、名字），一律指过去。写进 `docs/dev-guide/workflow.md` 当规矩，并排查 `data/workflows/*.yaml`。
  为什么：实验室那次 `classify` 里抄了八类分类目录，材料库后来长到十一类；执行者照旧表归类，落不进去的只能报「拿不准」——把人的闸门派去裁一个不是判断问题的东西。
- [ ] **定义核对补齐**：`src/workflow/check.rs` 补两条，只看写下的位置、不访问文件系统：判据里 `path` / `file` 的路径须在工作区内；描述里点到的小节须有 `contains` 判据覆盖（小节只认干净的名字，引号里的长句当叙述）。动 `src/workflow/check.rs`、`src/workspace/check.rs`。

## 四、执行

- [ ] **提示词带流水**：`src/prompts.rs` 给执行者与复查者都带上「流水（前几笔）」，含人的放行与裁决。动 `src/prompts.rs`、`src/task/ai.rs`。
  为什么：实验室里放行只落在流水里，报告和定义都看不到，下一步的执行者与复查者就不认那 6 条已被裁决——机械判据过、复查不过，白跑一趟智能体。
- [ ] **命令面改动词式**：`workflow create|show|list|check|export|import`、`order create|show|list|next|done|journal|delete`，与规格端点表一一对应（端点表里没有的操作，命令行里也没有）。动 `src/cli.rs`、`src/cli/handlers/`。
- [ ] **规格引用改指新篇**：全库 doc comment 与文档里的 `specification/process/task.md` 换成 `work-order.md` / `work-record.md`。动 `src/**/*.rs`、`docs/`。

## 五、事件

- [ ] **领域事件落 JSONL**：`WorkflowCreated` / `WorkOrderCreated` / `WorkRecorded`；负载至少带工作区 `id`、工单 `id` 与 `name`、`workflow_id`，记录事件带记录 `id` / `seq` / `step_id` 与全文；下游按 `id` 幂等去重。新增 `src/events.rs`，事件文件落账本仓。

## 六、文档与测试

### 测试

现状：`tests/` 12 件、约 1000 行，按用例切（`agent_step` / `task_start` / `state_machine` / `criteria` / `human_step` / `defaults` / `run_context` …），每条测试上方一行 `// 用例：<号>`。

- [ ] **改名与改意的**：`tests/task_start.rs` → `order_start.rs`（起步即封面落笔即封，`id` / `workflow_id` / `created_at` 账本方查填）；`tests/agent_step.rs` / `tests/human_step.rs` → `order_next.rs` / `order_done.rs`（机器路径、人的路径含闸门放行）；`tests/state_machine.rs` 的「流水序列 → 下一步」真值表改成「`records` 的 `step` 序列 → 下一步」，不再按 `·审` / `·判` 后缀投票。
- [ ] **重写的**：`tests/run_context.rs` 整篇换——现在测的是「运行上下文随任务记着」，位置不进模型后这几件事不存在了；改成测装载：不给 `--root` 往上找 `data/journal`、账本缺省落 `$XDG_DATA_HOME/qtcloud-work/workspaces/<键>`、`--data` / `--artifacts` 指到哪就落哪，并加一条「只读动作不在任何根上建文件」。`tests/defaults.rs` 的缺省矩阵从三项（`--root` / `--data` / `--workflows`）变四项（加 `--artifacts`），断言换成新缺省。
- [ ] **新增的**：
  - `tests/records.rs`：追加幂等（同 `id` 即拒）、`seq` 自 1 起不跳号、`step_id` 按 `step` 查填、时间倒序即拒、只增不改（旧记录原样在账上，以最新一条为准）；
  - `tests/credentials.rs`：定义落盘文件里没有 `id`；读时按「工作区 id + 名字」派生，两次读一样；同名跨工作区凭证不同；工单与工作记录照旧带凭证；
  - `tests/events.rs`：三件事各落一行 JSONL，负载带工作区 `id`、工单 `id` / `name` / `workflow_id`、记录 `id` / `seq` / `step_id` 与全文；重放同 `id` 不二次落账。
- [ ] **跟改的**：`tests/contract.rs`（契约快照，新动作与 `data` 字段变更要覆盖，「动作层不依赖入口层」那条要包住新动作）；`tests/criteria.rs` / `tests/criteria_matrix.rs`（落点引用按 `--artifacts` 展开）；`tests/definition_check.rs`（补两条：判据路径须在区内、描述里点到的小节须有 `contains` 覆盖）；`tests/common/mod.rs` 的起任务夹具改成起工单（备 `records` 与 `workspace.yaml`）。
- [ ] **用例号对账**：`docs/user-guide/*.md` 的 `## 用例 <号>、…` 与 `tests/*.rs` 的 `// 用例：<号>` 两边集合必须相等，`scripts/validate-usecases.sh` 核。现有五条（起一件任务并走一步 / 三类判据各判各的 / 比对两份课程档案 / 把语境条目收进材料 / 人做的步骤人记一笔）改标题（任务 → 工单、人记一笔 → 闸门放行），并按需增开：流水只增不改、凭证按名派生、产物落点。

### 文档

- [ ] **接口参考·动作面**：`docs/api-references/task.md` → `order.md`，动作改动词式（`create` / `show` / `list` / `next` / `done` / `journal` / `delete`），每个动作写清参数、落盘与拒绝条件；`workflow.md` 的动作表同改，导出仍是原样文件、导入撞名即拒这两条保留。
- [ ] **接口参考·输出契约**：`order` 的 `--json` 里 `data` 不得再有 `log` / `gates` / `start`；`workflow`（名字）与新增 `workflow_id` 分开；新动作 `delete` 的 `data` 与退出码写进契约（`--json` 字段只加不改、要改先加新留旧）。
- [ ] **接口参考·全局选项**：`docs/api-references/index.md` 的 API 索引表（task → order）与全局选项跟改：加 `--artifacts`，`--data` 缺省改成 `$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>`，并写明「位置不进模型，全部由启动参数装载」。
- [ ] **开发指南**：`docs/dev-guide/task.md` 拆成 `work-order.md` 与 `work-record.md`（前者封面与推导，后者记账与只增不改）；`workspace.md` 补工作区身份与三处位置（根 / 账本 / 产物）；`artifact.md` 的落点规矩从「相对工作区根」改成「相对产物落点」；`docs/dev-guide/index.md` 的落点图跟着改。
- [ ] **规矩落座**：把两条规矩写进开发指南——「定义里不抄别处拥有的事实（分类目录、落点、名字一律指过去）」写 `workflow.md`；「账本归 CLI、产物归工作区」写 `workspace.md`。
- [ ] **使用指南**：`docs/user-guide/{index,task,workflow,workspace,usecases}.md` 里的命令示例、全局选项说明与用例标题跟着改。

### 数据与发布

- [ ] **数据迁移**：`data/tasks/review-ui-shot.yaml`、`data/tasks/review-ui-shot-v2.yaml` 两张 v1 工单翻成新格式——补 `id` / `workflow_id` / `created_at`，`log` 逐条变 `records`（按序补 `seq` 与 `step_id`），删 `gates` 与 `root` / `data` / `workflows` 三个上下文字段。`data/workflows/*.yaml`（`optimize-workflow.yaml` / `review-ui-shot.yaml`）不带凭证，不用动；`data/artifacts/` 与 `data/materials/` 是内容，也不动。
- [ ] **门禁与版本**：命令面变了要记 `CHANGELOG.md`（`scripts/validate-changelog.sh` 核）、版本与 CHANGELOG 头一行一致（`scripts/validate-version.sh` 核）；拆 `task/` 时盯住单文件 ≤250 行（`scripts/validate-line-count.sh` 核，`task/mod.rs` 197 行、`task/report.rs` 242 行已经贴线，加 `records` 与 `events` 很容易超）。
- [ ] **与 studio 对表**：新增动作（`order delete`）与改名字段（`data` 里 `log` → `records`、`start` / `gates` 去掉）要在 studio 侧同步；两侧只比 `ok` / `columns` / `rows` / `data` 四样，不比 `lines`。
