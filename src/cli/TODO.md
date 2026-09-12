# TODO：把 cli 收到契约下

逐条列出要改的每一个细节。依据是 [STATUS.md](STATUS.md) 的体检结果；路线摘要见 [ROADMAP.md](ROADMAP.md)。

**每段做完的通用判据**（缺一不算完）：

```bash
cd apps/qtcloud-work/src/cli
cargo fmt --check && cargo clippy --locked && cargo test --locked
sh scripts/validate-usecases.sh
```

---

## 段一 · 正名与归类（先做）

**1.1** 命令改名：`find` → `search`（与 studio 同批做，两条命令面必须一起改）

- 改：`src/catalog.rs` 的 `find()`、`src/cli.rs` 的分发、`src/help.rs`「工作区」组；`docs/api-references/find.md` 改名 `search.md`；README 命令表；`docs/user-guide/` 里若有用例写 `find` 一并改
- 判据：`grep -rn "\bfind\b" src docs README.md` 只剩 Rust 标准库语义；`help` 那页写着 `search`
- 注意：命令面已发布到 crates.io → 破坏性变更，走版本号与 CHANGELOG

**1.2** 把分类写下来（聚合 / 领域服务 / 适配），落进 3.4 的约定文件，作为后面立目录的依据

## 段二 · 立目录（分类成目录）

现在 `src/` 是平的：聚合件、服务件、适配件同层。按契约分开。

**2.1** `src/` 分三类落位

- **聚合**：`task/`、`workflow/`、`material/`、`catalog/`、`workspace/`
- **领域服务**：`search`、`audit`（是否收进 `service/` 一层，待拍板——服务件少，平铺也不违反契约）
- **适配**：`cli.rs`（入口与发射）、`help.rs`、`prompts.rs`、`paths.rs`、`health`（是否收进 `platform/`，待拍板）
- 判据：聚合件不再与适配件同层；`src/` 顶层只剩目录 + 少量适配件 + `main.rs`

**2.2** `workspace` 立聚合：`workspace_root` / `data_dir` 从 `cli.rs` 搬出，`paths.rs` 并入

- 判据：`cli.rs` 里不再有工作区落点算法；`workspace/` 有自己的文件

**2.3** `search` 从 `catalog.rs` 分出（服务依赖聚合：用 `catalog` 建的名字索引）

- 判据：`catalog/` 里不再有按名找文档的动作；依赖方向是 `search → catalog`，反向为零

**2.4** `health` 从 `cli.rs` 分出（适配：远端探活）

- 判据：`cli.rs` 里不再有 HTTP 相关代码

## 段三 · 拆长文件（转聚合的实质）

触发已命中：3 个文件超阈，最长 945 行。

**3.1** `task.rs` 945 行 → 任务聚合内多件（状态推导 / 走一步 / 流水 / 报告 / 日志各一件）

- 判据：`wc -l src/**/*.rs` 无一件 >250；通用判据全绿（**行为不变**是硬要求）

**3.2** `cli.rs` 541 行 → 只留「clap 定义 + 定位 + 发射」，业务调用下移

- 判据：同 3.1 的行数判据

**3.3** `workflow.rs` 459 行 → 拆（YAML 读写 / schema 校验 / 五个动作）

- 判据：同 3.1

**3.4** 横切约束集中一处：建 `src/CONVENTIONS.md`（或 `docs/dev-guide/conventions.md`）

- 收进去：三类的归类判据（**有没有自己的定义**——`catalog` 有定义故为聚合，`search`、`audit` 为服务）、命名（能力名，不造 `-er`）、依赖单向（服务可依赖聚合，聚合不得依赖服务）、单文件 ≤250 行、测试按用例组织（与 `validate-usecases.sh` 配套，不是按模块）
- 判据：文件在；`docs/dev-guide/layers.md` 不再兼任约定

## 段四 · 契约对齐

**4.1** `quanttide-work` 版本两侧同步：cli 现在是 `0.1.0-beta.4`、studio 是 `^0.1.0-beta.5`——对表要求两边算出同一个结果，工具库版本必须跟上

- 判据：两侧 Cargo.toml / pubspec.yaml 引同一版本

**4.2** 依赖许可清单：列明已许可依赖与理由（`quanttide-work`、`clap`、`serde`、`serde_json`、`serde_yaml`、`ureq`），新增依赖先入清单

- 判据：`Cargo.toml` 的每个依赖都在清单里

**4.3** IaC 归属：确认 cli 的发布物（crates.io + 三平台二进制）与基础设施由哪里管；不归本仓就写一句归属说明

**4.4**（可选）单文件行数进 CI：加一条脚本，`src/**/*.rs` 超 250 行即红

## 段五 · 文档与工作流对齐

**5.1** `docs/dev-guide/layers.md` 重写：改成三类模块的落点图，去掉"拟建"说法，`outcome.rs` 那段按实际（已抽到工具箱）

- 判据：文档结构与 `src/` 一一对得上

**5.2** `docs/api-references/` 分两类：命令参考（`task.md` `workflow.md` `search.md` …）与概念参考（`workspace.md` 讲布局与资产表，不是命令）——分开放或标明

- 判据：目录里不再混两类

**5.3** `prompts` 两侧说法一致：`cli/src/prompts.rs` 与 `studio/lib/repositories/local/prompts.dart` 头注释互相矛盾（各说"只是自己这一侧的说法"）——要么合一，要么写明差异

- 判据：两处注释不再互相打脸；若确为两份，写明各自说法的差别

**5.4** README 与 CHANGELOG 随 `search` 改名更新（CHANGELOG 记在 Unreleased）

**5.5** `validate-usecases.sh` 的扫描范围随目录调整（现在找 `tests/*.rs`、`docs/user-guide/*.md`）

- 判据：脚本跑得过，用例号两边集合仍相等
