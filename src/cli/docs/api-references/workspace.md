# 工作区

工作区的落点、资产表与结果形状。

## 文件布局

```text
<数据仓>/
├── workflows/<工作流>.yaml      定义：步骤与判据（可用 --workflows 另指）
├── tasks/<任务>.yaml            任务：start + workflow + 运行上下文 + log
└── artifacts/
    ├── report/<任务>.md         报告：程序维护「执行记录」「闸门项」两节
    └── journal/<任务>.md        日志：叙事，人写
```

程序**不写产物的任何一节**：报告、日志都由写它们的人或智能体来写。产物落点先看任务里的 `artifacts` 声明，没声明就落草稿区 `artifacts/<类型>/<任务>.md`；声明了就按声明落（可指正式仓）。声明了落点的产物，起任务时程序只备一份空骨架。

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
