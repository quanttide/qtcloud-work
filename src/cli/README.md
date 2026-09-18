# qtcloud-work CLI

量潮工作云命令行 —— 把知识工作做成可执行的编排：**工作流**定义干什么、**工单**记这一趟怎么走、**判据**判算不算完。

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
| 工作流 | `workflow create / show / list / check / export / import` | 定义是 YAML；`check` 核对判据里的路径与描述提到的小节 |
| 工单 | `order create / show / list / next / done / journal / delete` | 开单、走一步、闸门放行、写日志、销白纸 |
| 导览 | `help [<话题>]` | 按用途列出命令；给了话题说那一条要点 |
| 探活 | `health` | `GET /health`，`--json` 透传服务端响应 |

全局四处位置，装载顺序 命令行 > 环境变量 `QTCLOUD_WORK_ROOT`（根）> 缺省：`--root` 工作区根（缺省往上找第二大脑）、`--data` 账本（缺省 `$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/`）、`--workflows` 工作流目录（缺省跟在账本里）、`--artifacts` 产物落点（缺省 `<工作区根>/artifacts/`）。写动作支持 `--dry-run`；每个动作支持 `--json` 与 `--out <文件>`。

## 数据怎么放

```
$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/   账本：归 CLI
├── workspace.yaml              工作区身份（id 供凭证派生，首跑生成）
├── events.jsonl                领域事件
└── workorders/<工单>.yaml      工单：封面（id / workflow_id / created_at）+ 流水

<工作区根>/artifacts/            产物：报告、日志（谁写谁定，程序不写内容）
```

位置不进模型：三处位置全部由启动参数装载——账本是这台机器上的账，产物是跟着工作区走的内容。

## 判据

判据挂在步骤上，按谁判分三类：`rule`（程序按 `path` / `absent` / `file`+`contains` / `run` 当场核）、`agent`（智能体照判准审）、`human`（进待拍板清单等人放行）。一步算过，等于所有 `rule` 通过、所有 `agent` 判过；闸门站等人 `order done`。

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
| `clap` | 4 | MIT OR Apache-2.0 | 命令行解析（derive） |
| `serde` | 1 | MIT OR Apache-2.0 | 序列化派生，供 `serde_json` / `serde_yaml` 用 |
| `serde_json` | 1 | MIT OR Apache-2.0 | `--json` 输出与窗口那一栏的结构化数据 |
| `serde_yaml` | 0.9 | MIT OR Apache-2.0 | 工作流与工单文件的 YAML 读写 |
| `ureq` | 2 | MIT OR Apache-2.0 | provider 探活（`health`）的同步 HTTP 客户端 |

领域模型（`outcome` / `criterion` / `workflow` / `order` / `workspace` / `artifact`）随本仓源码一并维护，不再引外部包。传递依赖（`Cargo.lock` 那一层）不列，需要审计时用 `cargo deny` 或 `cargo license` 现场生成。

## 许可

Apache-2.0
