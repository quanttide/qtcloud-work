# 依赖与许可

`Cargo.toml` 的每个直接依赖列在这里，写明用途与许可。新增依赖先入这张表，再进 `Cargo.toml`。

| 依赖 | 版本 | 许可 | 用途 |
| :-- | :-- | :-- | :-- |
| `quanttide-work` | 0.1.0-beta.6 | CC-BY-4.0 | 领域模型与结果信封（`outcome` / `task` / `workflow` / `workspace` / `artifact`），cli 与 studio 共用 |
| `clap` | 4 | MIT OR Apache-2.0 | 命令行解析（derive） |
| `serde` | 1 | MIT OR Apache-2.0 | 序列化派生，供 `serde_json` / `serde_yaml` 用 |
| `serde_json` | 1 | MIT OR Apache-2.0 | `--json` 输出与窗口那一栏的结构化数据 |
| `serde_yaml` | 0.9 | MIT OR Apache-2.0 | 工作流与任务文件的 YAML 读写 |
| `ureq` | 2 | MIT OR Apache-2.0 | provider 探活（`health`）的同步 HTTP 客户端 |

判据：`Cargo.toml` 的每个依赖都在上表里。

范围：只列直接依赖。传递依赖的许可（`Cargo.lock` 那一层）不在本表，需要时另建一份。

## 待决

`quanttide-work 0.1.0-beta.6` 的声明许可是 CC-BY-4.0（工具箱源已改 Apache-2.0，随下一号发布）。等工具箱发新号，cli 跟号后把上表的 CC-BY-4.0 改成 Apache-2.0。
