# 工作流（定义侧）

一条工作流就是**一串有序的步骤**。定义是 YAML，一个文件一条，放在工作流目录（`--workflows`）。

## 字段

| 字段 | 含义 | 例 |
|---|---|---|
| `name` | 名字，与文件名一致 | `devops-release` |
| `description` | 一段话：干什么、输入从哪来、产物落哪、怎么跑 | 见 `devops-release.yaml` |
| `steps[]` | 有序的步骤；顺序就是执行顺序 | 6 步 |
| `steps[].name` | 步骤名 | `audit` |
| `steps[].description` | 这一步的输入／过程／输出／验收 | — |
| `steps[].executor` | 谁做这一步：`agent`（AI）／`rule`（机器）／`human`（人） | `agent` |
| `steps[].criteria[]` | 判据：怎么算这一步走完 | 2～6 条 |

## 判据

每条判据自己也写 `executor`，和步骤的那个是两回事：步骤说谁干活，判据说谁来判。

| `executor` | 谁判 | 怎么写 |
|---|---|---|
| `rule` | 机器 | `path` 在不在；`path` + `contains` 文件里有没有这句话；`run` 跑条命令看退出码 |
| `agent` | AI | `description` 一句话，AI 照它判 |
| `human` | 人 | `description` 一句话，留给人 |

## 命令行

看一条：`workflow <名字>`；核对判据里的路径、描述里提到的小节有没有判据覆盖：`workflow <名字> --check`；
列全部：`workflow --list`；新建：`workflow --new`；带走一份／拿回来：`--export`／`--import`。

## 界面上

流程页的「步骤」态就是这个模型的可视化：一步一个节点，副标题写「执行者 · N 条判据」；
点开一个节点，上面的 `criteria` 逐条列出来。「定义」态直接显示这份 YAML。
