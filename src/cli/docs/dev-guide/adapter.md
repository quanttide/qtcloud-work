# adapter：适配

`src/` 顶层的适配件与入口，边界外的东西都落在这里。

- `main.rs`——入口：一行，把命令行交给 `cli`；
- `cli.rs` 与 `cli/`——clap 定义、定位（根 / 账本 / 工作流目录 / 产物落点）、命令树、子命令分派（`handlers/`）、发射（`emit.rs`）；
- `help.rs`——导览：按用途分组列命令、给话题要点；
- `prompts.rs`——给智能体的两段话（走一步要它做什么、审一遍按什么判）；
- `health.rs`——provider 探活：`GET /health`，基地址优先级 `--server` > 环境变量 `QTCLOUD_WORK_API_BASE_URL` > 默认网关。

## 入口的边界

入口不写算法，只解析参数、定位路径、把动作层的结果印出来。发射一处：`--out` 落 `data` 那一栏，`--json` 印信封（`ok` / `lines` / `columns` / `rows` / `data`），人看的走 `lines`。

聚合与服务不得依赖入口层（`crate::cli`）。

## 扩展

- 加动作：在对应聚合写一个函数返回 `Outcome`，在这里注册子命令与选项。算法只落一次。
- 接 provider：另一条轴，落在入口与 `health`，不动本地的定义、工单与判据。
- 开窗口：窗口与命令行共用动作算出的 `Outcome`，窗口只负责画，不重写算法。

## 测试

契约快照在 `tests/contract.rs`——子命令清单、`--json` 字段、依赖方向。

## 待定

`src/prompts.rs` 与 studio 的 `prompts.dart` 各有一份给智能体的两段话，头注释互相矛盾。合一还是各留一份，待定。
