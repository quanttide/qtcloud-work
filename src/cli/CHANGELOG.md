# Changelog

本文件仅记录 **cli（量潮知识工作云 CLI，Rust）** 的版本变更。

## [Unreleased]

### Added

- 任务进度条：`task <名字>`、`task --list` 与走一步（`--next` / `--done`）的结果印一条十格进度条（如 `[███░░░░░░░] 1/3`）。只落在人看的那一栏（`lines`），`columns` / `rows` / `data` 一个字不动，与 studio 对表不受影响。

### Changed

- 文档与门禁补齐：
  - 接口参考一个命令一篇（扁平）；工作区的布局与资产表并入使用指南的「工作区」一篇。
  - 使用指南恢复「用例」一篇（五件真事），`validate-usecases.sh` 的对账不再空转。
  - 开发指南按 `src/` 的领域重排（聚合 / 领域服务 / 适配各一篇）；依赖与许可列进 README。
  - 门禁补到五条并与 CI 一致：clippy 改 `--all-targets -- -D warnings`，新增 `validate-line-count.sh`（`src/**/*.rs` 超 250 行即红），用例对账与行数门禁进 CI。
- 产物类别的字段与 JSON 键由 `kind` 改名 `category`（`catalog --json` 每条、`audit --json` 的 `missing` 每条）；中文标签「种类」改「类别」；studio 侧同批改。`--json` 键名属破坏性变更，随版本号走。
- 工具箱跟到 `quanttide-work 0.1.0-beta.6`（两侧同号），两次破坏性变更改到位：
  - **落点**改走 `quanttide_work::workspace::Workspace::place`——工具箱只给**相对工作区根的路径**，接哪一处目录由本仓定：任务里声明过的按工作区根接（能指到正式仓），没声明的按数据仓接（草稿区）；流水不是产物，仍是任务文件本身
  - **流水判定**改走 `Workspace::done_steps` / `next_step` / `state_line`（按名字从工作区里取定义），定义核对改走 `Workspace::check`（`Finding.ok` 成了三态：过 / 不过 / 未核，退回了「○ 未核」的写法）
  - **判据里的占位**展开改用工具箱那一套（`Criterion::expanded` 收「名字 → 路径」的解析函数）：本仓不再自己扫 `{{…}}`，只答「这个名字换成哪条路径」，且给的是工作区根视角（`run` 里的命令直接可用）
  - 工作流读写对齐工具箱：`Workflow::of(值)` 的工作流名取自 `name` 字段（不再是文件名），语法校验收工具箱的结构化报错（`DefinitionError::message(file)`）；`text_of` 本仓自备
- 删掉 `RunContext` 相关的转发（工具箱不再有那个类型）：任务里记的三处位置（`root` / `data` / `workflows`）仍是本仓自己的运行数据，读写不变

- 命令 `find` 改名 `search`（按名找文档）：命令面、导览、README 与接口参考（`docs/api-references/search.md`）同步；命令面已发布，属破坏性变更，随版本号走。
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
