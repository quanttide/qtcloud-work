# work-order：工单

工单是工作流的一次行程：封面写着走哪条工作流，内页是流水（见 [work-record](work-record.md)）。落账本仓 `<账本>/workorders/<名字>.yaml`，一件一个文件，文件名即工单名。

## 落点

- `order/model.rs`——领域模型：`WorkOrder` 与 `WorkRecord`，字段表 `FIELDS`；
- `order/mod.rs`——账本本身：文件读写、开单、销白纸、按凭证找工作流；
- `order/actions.rs`——动作：开单、走一步、人记一笔、日志、删白纸；
- `order/inspect.rs`——看与列：封面、步骤状态、待拍板闸门、状态行；
- `workspace/progress.rs`——进度与完结的推导（见下）；
- `locate/mod.rs`——账本与产物的装载、工作区身份与工单落盘。

## 封面

封面六字段：`id` / `name` / `description` / `workflow_id` / `created_at` / `records`。三条封条：

- **凭证由程序发**：`id` 落笔后不变；`workflow_id` 是开单时按「工作区 id + 名字」派生的那枚——封面落笔即封，此后工作流改名也不改这枚凭证；
- **位置不进封面**：没有 `root` / `data` / `workflows`，三处位置由启动参数装载（见 [workspace](workspace.md)）；
- **进度不进封面**：走过哪几步、下一步、完结没完结，由流水对照定义推导，不另存字段。

## 开单与销户

`create` 按名找工作流、现算凭证、查填封面，重名即拒——不覆盖。`delete` 只销白纸：流水非空即拒，有账不销。

## 推导

推导在 `workspace/progress.rs`，只拿流水与定义这两样：

- **走过**：这一站在流水里有一条 `is_succeeded` 为真的记录；
- **下一步**：定义里第一个没走过的站；
- **完结**：每站都走过——结论随时可重算，完成不是动作，是事实；
- **闸门**：带 `human` 判据的站，通过须出自人（`order done`），程序核完自己的半程就等人。

真值表在 `tests/state_machine.rs`：审查 ✗、重走通过、乱序、名字不在定义里这些边界。
