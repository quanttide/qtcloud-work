# qtcloud-work CLI

量潮工作云命令行 —— 把知识工作做成可执行的编排：**工作流**定义干什么、**任务**记这一次怎么走、**判据**判算不算完。

本地文件驱动：读 YAML 与 Markdown、写记录、跑判据，不启服务、不连数据库。provider 探活（`health`）保留；接口层等就位后再接。

## 装与跑

```bash
cargo install qtcloud-work-cli        # 从 crates.io 装
cargo build --release                 # 或本仓构建，产物 target/release/qtcloud-work
qtcloud-work --help
```

## 命令面

| 组 | 命令 | 说明 |
| :-- | :-- | :-- |
| 工作区 | `search` `catalog` `audit` `material` | 按名找文档、列目录、审计资产、看材料四字段 |
| 工作流 | `workflow --list / --new / <名字> / --check / --export / --import` | 定义是 YAML；`--check` 核对判据里的路径与描述提到的小节 |
| 任务 | `task --new / <名字> / --next / --done / --journal / --list` | 走一步、人为记一步、写日志 |
| 导览 | `help [<话题>]` | 按用途列出命令；给了话题说那一条要点 |
| 探活 | `health` | `GET /health`，`--json` 透传服务端响应 |

全局三处位置：`--root` 工作区、`--data` 数据仓（任务与产物草稿，缺省当前目录下的 `data/`）、`--workflows` 工作流目录（缺省跟在数据仓里）。写动作支持 `--dry-run`；每个动作支持 `--json` 与 `--out <文件>`。

## 数据怎么放

```
<数据仓>/
├── workflows/<工作流>.yaml     定义（可另指固定资产目录）
├── tasks/<任务>.yaml           运行数据：start + workflow + 运行上下文 + 流水 + 闸门项 + 产物落点
└── artifacts/                  产物：报告、日志（谁写谁定，程序不写内容）
```

任务里记着这次执行的三处位置，此后 `task <名字>` 不必再写 `--root` / `--workflows`。产物的落点由任务声明（`artifacts`），要落正式仓就在那里写路径。

## 判据

判据挂在步骤上，按谁判分三类：`rule`（程序按 `path` / `absent` / `file`+`contains` / `run` 当场核）、`agent`（智能体照判准审）、`human`（进闸门项等人拍板）。一步的每一条流水（执行与审查）都 ok 才算走过。

## 文档

- [使用指南](docs/user-guide/index.md)——怎么用，含真事用例；
- [接口参考](docs/api-references/index.md)——命令、字段、判据、占位的参考；
- [开发指南](docs/dev-guide/index.md)——分层、扩展点与五类测试。

## 开发

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
sh scripts/validate-usecases.sh      # 文档用例与测试出处对账
sh scripts/validate-line-count.sh    # src/ 下单文件超 250 行即红
```

测试按用例组织：一个场景一个文件，测试上标 `// 用例：N`，与使用指南的「用例」对账；分层规矩见 [`src/CONVENTIONS.md`](src/CONVENTIONS.md)。

发布走 `qtcloud-devops`：`release audit` 预检、`release publish` 建 tag 与 Release；推 `cli/v*` tag 触发本仓 `release-cli` 工作流（校验、质量门禁、多平台产物、crates.io）。

## 依赖与许可

`Cargo.toml` 的每个直接依赖列在这里，写明用途与许可；新增依赖先入表。

| 依赖 | 版本 | 许可 | 用途 |
| :-- | :-- | :-- | :-- |
| `quanttide-work` | 0.1.0-beta.6 | CC-BY-4.0 | 领域模型与结果信封（`outcome` / `task` / `workflow` / `workspace` / `artifact`），cli 与 studio 共用 |
| `clap` | 4 | MIT OR Apache-2.0 | 命令行解析（derive） |
| `serde` | 1 | MIT OR Apache-2.0 | 序列化派生，供 `serde_json` / `serde_yaml` 用 |
| `serde_json` | 1 | MIT OR Apache-2.0 | `--json` 输出与窗口那一栏的结构化数据 |
| `serde_yaml` | 0.9 | MIT OR Apache-2.0 | 工作流与任务文件的 YAML 读写 |
| `ureq` | 2 | MIT OR Apache-2.0 | provider 探活（`health`）的同步 HTTP 客户端 |

`quanttide-work 0.1.0-beta.6` 的声明许可是 CC-BY-4.0（工具箱源已改 Apache-2.0，随下一号发布）；等工具箱发新号，cli 跟号后改成 Apache-2.0。传递依赖（`Cargo.lock` 那一层）不列，需要审计时用 `cargo deny` 或 `cargo license` 现场生成。

## 许可

Apache-2.0
