# 路线图

本文记录本轮重构的动机、阶段划分与验收标准；剩余待办逐条见 [TODO.md](TODO.md)。

## 背景

规格已修订为四篇：工作流（`docs/specification/process/workflow.md`）、工作步骤（`work-step.md`）、工单（`work-order.md`）、工作记录（`work-record.md`）。实验室 v2（`examples/default/apps/qtcloud-work-lab-v2`）按新规格跑通了真流程；正式版 CLI 原停在 v1 模型——`task` 携带 `log` / `gates` / `artifacts` 字段，工作流定义不含凭证，执行位置记进任务文件。本轮重构把实验室验证所得落实到正式版。

问题的共同根源是：应当由推导得出的结论被落成了字段，应当由外部装载的位置被记进了模型。`Task` 兼做工单封面与工作记录账本，规格里对应两篇；`gates` 可由定义中 `human` 判据的所在步骤推导得出，却作为字段存储；工单文件记着 `root` / `data` / `workflows` 三处位置，换机器、挪仓库即失效；账本与产物同仓，程序每跑一趟就往仓库写账本。实验室踩过的坑（给每条工作流手工编 UUID、流水不幂等、人的放行只落在流水里导致后续步骤重复执行已裁决的内容、定义抄录会变化的分类目录）不再逐条展开，见提交历史与实验室记录。

## 目标

1. 模型对表：`Task` 拆成工单与工作记录；`gates` 不落字段，由定义加流水推导；封面落笔即封、流水只增不改；进度与完结只由推导得出。
2. 凭证分层：定义不写 `id`，按「工作区 id + 名字」派生（uuid5，命名空间钉死，各平台计算结果一致）；工单与工作记录的凭证由程序生成。
3. 位置归位：账本归 CLI（缺省 `$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/`），产物归工作区（缺省 `<工作区根>/artifacts`），位置全部由启动参数装载、不进模型。
4. 命令面对齐规格端点表：`workflow …` / `order …` 动词式；三类领域事件写入 JSONL，下游按 `id` 幂等去重。
5. 文档与测试同步：用例、接口参考、开发指南、数据迁移与变更记录全部对齐新模型。

## 阶段规划

| 阶段 | 内容 | 验收标准 |
| :-- | :-- | :-- |
| 一 · 定名与模型对表 | 先确定 `task` 的新名称，再拆分工单与工作记录、删除 `gates` 字段、建立凭证与流水规则、补充工作区身份 | 模型中不存在 `log` / `gates` / `start`；`records` 只增不改；进度由 `records` 对照定义推导 |
| 二 · 位置 | 账本缺省位置改为 XDG 工作区键、新增 `--artifacts`、工作区根可由 `QTCLOUD_WORK_ROOT` 选定、删除工单中的三处位置 | 缺省 `--root` 时仍可创建工单；只读动作不在任何根上创建文件；装载顺序为命令行 > 环境变量 > 向上搜索 |
| 三 · 定义 | 落点引用按产物落点展开；定义不抄录其他来源的事实；`check` 补两条规则 | 现有定义不做修改即可读取并通过核对 |
| 四 · 执行 | 命令面改为动词式、提示词附带流水、规格引用改指新篇 | 命令面与规格端点表一一对应 |
| 五 · 事件 | `WorkflowCreated` / `WorkOrderCreated` / `WorkRecorded` 写入 JSONL | 负载带全工作区与工单、记录的身份字段；事件文件只增不改，去重由下游按 `id` 做 |
| 六 · 文档与测试 | 用例、接口参考、开发指南、数据迁移与门禁 | 两侧用例集合一致；单文件不超过 250 行；与 studio 对表四项一致 |

阶段一、二是后续各段的基础；阶段五、六待字段确定后进行，其后可并行。

每段完成后执行以下命令，全部通过方视为完成：

```bash
cargo fmt --check && cargo clippy --all-targets --locked -- -D warnings && cargo test --locked
sh scripts/validate-usecases.sh && sh scripts/validate-line-count.sh
```

发布前另行执行 `scripts/validate-changelog.sh` 与 `scripts/validate-version.sh`；命令面变更属于破坏性变更，随版本号发布。

## 当前状态

阶段一至五与数据迁移已在工作区实施完毕（尚未提交）：`src/order/` 取代 `src/task/`（名称定为 `order`），凭证派生（`src/ids.rs`）、事件落账（`src/events.rs`）、账本与产物的缺省位置、动词式命令面、`data/workorders/` 新格式均已落地。剩余集中在阶段六——测试与文档还没跟上：`tests/common` 夹具未改齐，七个测试文件编译不过；旧测试与新测试并存；`docs/` 仍是 task 时代的样子；门禁 fmt 与 test 当前不过。阶段二有一处补漏排进收尾清单：工作区选定缺一层接口，定为环境变量 `QTCLOUD_WORK_ROOT`（装载顺序：命令行 > 环境变量 > 向上搜索），见 [TODO.md](TODO.md) 一。

提交按段原子进行，每段勾销 [TODO.md](TODO.md) 对应条目并更新本节；全部勾清后本路线图归基线。
