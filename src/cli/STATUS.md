# 状态：cli 对照软件工程契约

体检日 2026-09-12。条款来自 [契约原型](../../../../code/data/insight/code-agent/contract.md)（结构契约、依赖契约、阶段条款）与 code 手册的平台契约。

## 规模

| 项 | 数 |
|---|---|
| `src/` 文件 | 11（2 971 行）——**平铺**，无目录层 |
| 最长四个文件 | `task.rs` **945**、`cli.rs` **541**、`workflow.rs` 459、`catalog.rs` 276 |
| `tests/` | 11 个文件，**按用例组织**（agent_step / criteria / definition_check / state_machine / task_start …），含 `common/` 夹具 |
| `docs/` | 三轴齐全：api-references 9 篇、dev-guide 5 篇、user-guide 3 篇 |
| `scripts/` | 3 条：`validate-version.sh`、`validate-changelog.sh`、**`validate-usecases.sh`（文档用例号 ↔ 测试出处，两边集合必须相等）** |

## 结构契约

| 条款 | 现状 | 判定 |
|---|---|---|
| 终态按聚合、聚合与领域服务并列、适配一处 | `src/*.rs` 平铺一层：聚合件（`task` `workflow` `material` `catalog`）、服务件（`catalog.rs` 里的 search、`audit`）、适配件（`cli` `help` `prompts` `paths` `health` 在 `cli.rs` 里）**全部同层** | ✗ 该转 |
| 单文件行数越界即触发转聚合 | 3 个文件超阈，最长 945 行 | ✗ 已触发 |
| 同一目录不得混用层名与聚合名 | 无层名，但**聚合件与适配件同层**——同一病的另一种形态 | ✗ 违规 |
| 角色唯一 | `workspace` 没有自己的文件：`workspace_root` / `data_dir` 散在 `cli.rs`，`paths.rs` 只管路径显示；`health` 也住在 `cli.rs` | ✗ 待立 |
| 服务与聚合分明 | `find`（将改名 `search`）住在 `catalog.rs` 里（"含按名找文档的动作"）——服务寄生在聚合上 | ✗ 待分 |
| 依赖单向（端侧引用工具箱、不反向定义） | `quanttide-work` 是依赖；`artifact.rs`／`paths.rs` 的规矩各自只有一处 | ✓ |
| 不留第二份模型 | 领域模型在工具箱；`outcome.rs` 已撤，只留路径显示 | ✓ |
| 组装与实现分离 | `main.rs` 只一行，交给 `cli.rs` | ✓（但 `cli.rs` 541 行把入口、解析、发射、业务动作的调用都揽了） |
| 横切约束集中一处 | 无此物；约定散在 `docs/dev-guide/` 与文件头注释 | ✗ 缺 |
| 测试的分布 | 测试按**用例**组织（不是按模块）——这不是违规，是「用例即切口」；但 `src` 与 `tests` 不同构，要在约定里写明，免得下次被当违规改 | — 记明 |

## 依赖契约

| 条款 | 现状 | 判定 |
|---|---|---|
| 抽出去必须发布、按版本号引 | `quanttide-work = "0.1.0-beta.4"`（crates.io，非 path） | ✓ |
| **两侧版本对齐** | studio 引 `^0.1.0-beta.5`，cli 引 `0.1.0-beta.4`——**两侧不同步**，而对表要求两边算出同一个结果 | ✗ 张力 |
| 官方 SDK 优先 | `clap` / `serde` / `serde_json` / `serde_yaml` 是生态标准；`ureq` 轻量同步 HTTP（provider 是自研接口，无官方 SDK） | ✓ |
| 依赖许可清单 | 无一处列明已许可依赖 | ✗ 缺 |
| 一条发布线一个包 | **做得好**：`cli/v*` tag → 校验版本与 CHANGELOG → 三平台构建；`scripts/validate-*.sh` 成对 | ✓（可作 studio 范本） |

## 平台契约

| 条款 | 现状 | 判定 |
|---|---|---|
| 技术栈按角色 | Rust 二进制 ✓ | ✓ |
| 门禁：Rust = `cargo fmt --check` + `cargo clippy` + 测试 | CI 三条齐（`cargo fmt --check`、`cargo test --locked`、`cargo clippy --locked`） | ✓ |
| 发布前 `cargo publish --dry-run` | CI 有 | ✓ |
| IaC 目录 `manifests/terraform/` | 无（同 studio，待确认归属） | ✗ 待确认 |
| 可观测与安全 | 无后端；`health` 探活 provider（`GET /health`） | — 豁免/适配 |

## 与文档、工作流的对照

- `docs/dev-guide/layers.md` **过时**：说 `outcome.rs` 还在 `src/`（实际已抽到工具箱）、没提 `paths.rs` 与 `prompts.rs`、把这一批写成"拟建"；`catalog.rs` 那条要随 `search` 改名改
- `docs/api-references/` 混着两类：命令参考（`task.md` `workflow.md` `find.md` …）与概念参考（`workspace.md` 讲布局与资产表，不是命令）
- `prompts.rs`（cli 侧）与 studio 的 `prompts.dart` **两份同义话术**，且两边头注释互相矛盾（cli 说"只是命令行这一侧的说法"、studio 说"只是工作台这一侧的说法"）——要么合一，要么写明哪里不同
- `data/` 已在 `.gitignore`，本地工作区数据不入库 ✓
