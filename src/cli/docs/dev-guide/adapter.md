# adapter：适配

`src/` 顶层的适配件与入口，边界外的东西都落在这里。

- `main.rs`——入口：一行，把命令行交给 `cli`；
- `cli.rs` 与 `cli/`——clap 定义、定位（工作区 / 数据仓 / 工作流目录）、子命令分派（`handlers/`）、发射（`emit.rs`）；
- `help.rs`——导览：按用途分组列命令、给话题要点；
- `prompts.rs`——给智能体的两段话（走一步要它做什么、审一遍按什么判）；
- `health.rs`——provider 探活：`GET /health`，基地址优先级 `--server` > 环境变量 `QTCLOUD_WORK_API_BASE_URL` > 默认网关。

## 入口的边界

入口不写算法，只解析参数、定位路径、把动作层的结果印出来。发射一处：`--out` 落 `data` 那一栏，`--json` 印信封（`ok` / `lines` / `columns` / `rows` / `data`），人看的走 `lines`。

聚合与服务不得依赖入口层（`crate::cli`），有测试钉住。
