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

- [ ] **接口参考与开发指南对表**：`docs/api-references/task.md` → `order.md`（动作改动词式）、`workflow.md` 同改；`docs/dev-guide/task.md` → `work-order.md`，工作记录另起一篇；`docs/api-references/index.md` 的 API 索引与全局选项跟改（含 `--artifacts`）。
- [ ] **用例号对账**：`tests/` 按用例切文件、上方一行 `// 用例：<号>`，与 `docs/user-guide/*.md` 的 `## 用例 <号>、…` 相等；跑 `scripts/validate-usecases.sh` 核。
- [ ] **数据迁移**：`data/tasks/*.yaml`（v1 格式，带 `log` / `gates` / `root`）翻成工单格式；`data/workflows/*.yaml` 不带凭证，直接读得进，不用动。
- [ ] **与 studio 对表**：新增与改动的动作，`ok` / `columns` / `rows` / `data` 四样与 studio 对得上（不比 `lines`）。
