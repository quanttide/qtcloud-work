# Changelog

本文件仅记录 **cli（量潮知识工作云 CLI，Rust）** 的版本变更。

## [Unreleased]

### Changed

- 结果改用工具箱 `quanttide_work::outcome::Outcome`（规范「过程 / 结果」那一节结成的模型）；本地的 `outcome.rs` 只留「路径怎么显示给人看」，改成 `paths.rs`
- `--json` 一律是这个结果：`ok` / `lines` / `columns` / `rows` 四样，原文托在 `data` 里（原先 `audit` / `catalog` / `material` 吐的是裸原文）；旧键按「只加不改」在顶层再留一轮，下一轮删
- `task <名字>` 与 `workflow <名字>` 的结果补上 `data`：任务原文与三样产物的落点、定义原文与其位置——窗口从这一栏装领域对象
- `--out` 落的是原文那一栏（`data` 的内容），形状不变
- 声明表正名为 `artifacts`（原先叫 `products`）：任务文件里的键、`--json` 那一栏的键、`--new` 写下的键都跟着改；落点算法收进工具箱的 `Task::artifact`，本地只剩「路径怎么显示」与「建空骨架」

## [0.1.0-beta.1] - 2026-09-11

从 alpha 进 beta：功能齐了，等反馈。

- 新增 `help` 导览（`src/help.rs`）：按用途列出命令；`help <话题>` 说那一条要点；`-h/--help` 说明统一成中文。
- 每条命令的帮助补齐例子、标志配套与文档指引，并用测试钉住「帮助里必须有例子」。
- 文档：接口参考以 API 为单位重排（一个 API 一篇：find / catalog / audit / material / workflow / task / help / health，另有入口与工作区）；使用指南与开发指南各拆成几篇；删去过时的用例篇。
- 测试：新增定义核对、导览与帮助两类契约测试。

## [0.1.0-alpha.4] - 2026-09-11

- CI（`release-cli`）：crates.io 预检带上 User-Agent（不带会被 403 挡回，与 token 无关）；token 同时认 `CRATES_API_TOKEN` 与 `CARGO_REGISTRY_TOKEN` 两个名字。

## [0.1.0-alpha.3] - 2026-09-11

- CI（`release-cli`）：预发布也推 crates.io（撤掉 alpha 跳过那一条）——`0.1.0-alpha.N` 是合法的 crates.io 版本，够格就推。

## [0.1.0-alpha.2] - 2026-09-11

- 修：机器判据不过时这一步不算走过——机器判据的结果单记一笔（`<步骤>·判`），「走过了」认所有后缀判定；重走一次通过即算过，上一笔失败不再压着它。
- 修：定义核对别把版本号写法（`## [X.Y.Z]`）当报告小节——小节名得是中文短词。
- CI（`release-cli`）：预发布（tag 带 `-`）不推 crates.io；版本从 tag 现取（`cli/v0.1.0` → `0.1.0`）修正原先取空值的问题；tag 带 `-` 时自动把 Release 标成预发布。

## [0.1.0-alpha.1] - 2026-09-11

首个预发布。把实验室 `kg` 那套本地知识工作做法扶正为平台侧命令行，provider 探活 `health` 保留。

### 新增

- 工作流与任务两族动作：`workflow --list / --new / --export / --import / --check`，`task --list / --new / --next / --done / --journal`；定义写成 YAML，任务是一次执行实例，落在数据仓
- `--next` 按执行者分派：`agent` 交给 `pi` 跑完自动核判据；`human` 不代做，用 `--done` 记一笔
- 判据三类：`rule`（存在 / 不存在 / 含文 / 跑命令四种判法）、`agent`（照说明审）、`human`（进闸门）；一步算过＝rule 全过且 agent 全通过
- 工作区四个只读动作：`find` / `catalog` / `audit` / `material`
- 全局上下文 `--root` / `--data` / `--workflows` 随任务走；`--json` 与 `--out` 分家；写入型动作支持 `--dry-run`；退出码 0 / 1
- 三部分文档（使用指南、开发指南、接口参考）与场景测试、用例对账脚本
- provider 探活 `health`，仍带全局 `--server` / `--json`

### 变更

- 源码按「动作与对象同住」分模块：`cli` / `outcome` / `artifact` / `audit` / `catalog` / `material` / `workflow` / `task`
- 任务与产物分家：程序不写产物，闸门项记在任务文件，产物落点由任务声明
- 数据仓默认取当前目录下的 `data/`（开发环境，不进版本库），用哪个印到标准错误

### 修复

- 人的步骤挂 agent 判据时不再被挡住：没跑智能体算待判，进闸门项
- 审查判 ✗ 时这一步不算走过；判定只看最近一次尝试
