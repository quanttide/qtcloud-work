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
| 工作区 | `find` `catalog` `audit` `material` | 按名找文档、列目录、审计资产、看材料四字段 |
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

任务里记着这次执行的三处位置，此后 `task <名字>` 不必再写 `--root` / `--workflows`。产物的落点由任务声明（`products`），要落正式仓就在那里写路径。

## 判据

判据挂在步骤上，按谁判分三类：`rule`（程序按 `path` / `absent` / `file`+`contains` / `run` 当场核）、`agent`（智能体照判准审）、`human`（进闸门项等人拍板）。一步的每一条流水（执行与审查）都 ok 才算走过。

## 文档

- [使用指南](docs/user-guide/index.md)——怎么用，含真事用例；
- [接口参考](docs/api-references/index.md)——命令、字段、判据、占位的参考；
- [开发指南](docs/dev-guide/index.md)——分层、扩展点与五类测试。

## 开发

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
sh scripts/validate-usecases.sh      # 文档用例与测试出处对账
```

发布走 `qtcloud-devops`：`release audit` 预检、`release publish` 建 tag 与 Release；推 `cli/v*` tag 触发本仓 `release-cli` 工作流（校验、质量门禁、多平台产物、crates.io）。

## 许可

Apache-2.0
