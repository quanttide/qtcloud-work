# 工作区

工作区是一次工作的边界：一套工作流定义与上下文内的工件绑在一起，区内的件互相可见、可流转。查看工作区一族命令在它上面转。

## 四个只读动作

- `qtcloud-work search <名字> [--show]` 按名找文档，认文件名与篇内一级标题，精确不中模糊兜底，`--show` 连正文一起看；
- `qtcloud-work catalog [--json] [--out 文件]` 按资产类别列出全部条目，`--out` 落一份 `{root, count, entries}`；
- `qtcloud-work audit [--json] [--out 文件] [--make]` 审计「资产表有而工作区无」与「工作区有而未登记」，`--make` 补建缺的文档格，独立仓库那三格不凭空建；
- `qtcloud-work material [路径…] [--json] [--out 文件]` 列材料的四字段与阶段，不给路径就扫 `data/journal` 与 `data/profile` 下的 md。

## 判据：谁判

判据挂在步骤上，按谁判分三类。`rule` 由程序按字段判，四种判法：`path` 存在、`absent` 不存在、`file` 加 `contains` 文件含某段文字、`run` 命令退出码为零；路径相对工作区根，写绝对路径则按绝对路径。`agent` 由智能体照说明审，逐条回答通过或不通过。`human` 不跑，原样进报告的闸门项等人拍板。

一步算过，等于这一步所有 `rule` 通过、所有 `agent` 判为通过；`human` 只列闸门，不影响过不过。动作结果 `ok=False` 时命令退出码为 1，报告仍照写。

## 文件布局

```text
<数据仓>/
├── workflows/<工作流>.yaml      定义：步骤与判据（可用 --workflows 另指）
├── tasks/<任务>.yaml            任务：start + workflow + 运行上下文 + log + gates + artifacts
└── artifacts/
    ├── report/<任务>.md         报告：程序维护「执行记录」「闸门项」两节
    └── journal/<任务>.md        日志：叙事，人写
```

工作流默认放在数据仓的 `workflows/`；定义常是固定资产，用 `--workflows` 可另指一处（例如 `data/profile/iGuo/workflows/`）。

程序**不写产物的任何一节**：报告、日志都由写它们的人或智能体来写。产物落点先看任务里的 `artifacts` 声明，没声明就落草稿区 `artifacts/<类别>/<任务>.md`；声明了就按声明落（可指正式仓）。声明了落点的产物，起任务时程序只备一份空骨架。

## 资产与目录

资产表二十格：陈述型 11（报告、参考、历史、日志、档案、宣传册、路线图、洞察、意图、语境、归档），程序型 9（章程、规格、工具箱、手册、案例、平台、教程、札记、实验室）。每格有一对名字：中文名与英文名。

落点：文档类入 `data/<英文名>` 或 `docs/<英文名>`，独立仓库按命名规则找——工具箱 `packages/*-toolkit`、平台 `apps/*`、实验室 `examples/*`。

目录层扫描时跳过 `.git`、`node_modules`、`.venv`、`build`、`dist`、`.dart_tool`、`__pycache__` 这些目录，以及 `README.md`、`CHANGELOG.md`、`LICENSE` 门面文件。

## 材料

材料的四字段：

| 字段 | 取值 |
| :-- | :-- |
| `type` | 扩展名 |
| `content` | 正文首段截 40 字（正文取 md / txt / rst） |
| `source` | 上级目录加文件名 |
| `created_at` | 文件所在仓库首次提交的日期，退回文件名里的日期 |

阶段由位置承担：落在 `journal` 下的是「原始」，其余是「材料」。`material --out` 落 `{count, materials}`，每条是 `path` 加这四个字段与 `stage`。
