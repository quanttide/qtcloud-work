# 分层

`src/` 顶层按三类落位：聚合 / 领域服务 / 适配。判据与命名规矩在 [`src/CONVENTIONS.md`](../../src/CONVENTIONS.md)；这一篇是落点图，图里每一项都对应 `src/` 里的实际落点。

```text
src/
├── main.rs        入口：一行，把命令行交给 cli
├── cli.rs         适配：clap 定义、定位、发射
├── cli/           适配：子命令分派（handlers/）与发射（emit.rs）
├── help.rs        适配：导览
├── prompts.rs     适配：给智能体的话术
├── health.rs      适配：provider 探活
├── task/          聚合：任务——类型、文件读写、上下文与流水
├── workflow/      聚合：工作流——YAML 读写、schema、定义核对、五个动作
├── catalog/       聚合：目录——扫描成名字索引
├── artifact/      聚合：产物——标准产物定义
├── material/      聚合：材料——类型 / 内容 / 来源 / 时间
├── workspace/     聚合：工作区——三处位置与落点
├── search/        领域服务：按名找文档
└── audit/         领域服务：审计产物表与工作区
```

`task/` 内部再按事分件：类型与出入口在 `mod.rs`，状态推导在 `state`、走一步在 `execute`、交给 AI 的两段话在 `ai`、动作在 `report`、日志在 `journal`、进度条在 `progress`。`workflow/` 同样是 `mod.rs` 留类型与出口，读写、核对、动作各一件。

各层分工：

- 入口不写算法，只解析参数、定位路径、把动作层的结果印出来；工作区与数据仓的默认值在 `workspace/` 一处定。
- 动作各归其主：动作与它操作的对象住同一处（工作流的动作在 `workflow/`、任务的在 `task/`、材料的在 `material/`），每个动作返回 `quanttide_work::outcome::Outcome`——`lines` 给命令行印，`columns` 与 `rows` 给窗口画，`ok` 定退出码。动作之间不互相打印。
- 定义层（`workflow/`）与执行层（`task/`）分家：工作流是数据不是代码，加流程不改程序；任务引用工作流名，状态从流水读出来。
- 结果信封在工具箱 `quanttide_work::outcome`（本仓不再有 `outcome.rs`）；本仓只留「路径怎么显示给人看」在 `workspace::short`。
- 判据、报告段位、产物定义、材料字段各只写一处，用的人从那一处取。
