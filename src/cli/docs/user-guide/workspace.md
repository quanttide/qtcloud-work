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
$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/   # 账本：归 CLI
├── workspace.yaml               工作区身份：id 供凭证派生，首跑生成
├── events.jsonl                 领域事件：开工作流、开单、记账各一行
├── workorders/<工单>.yaml       工单：封面（id / workflow_id / created_at）+ 流水
└── workflows/                   工作流目录的缺省位置

<工作区根>/                                        # 工作区：内容
├── data/profile/iGuo/workflows/ 定义常是固定资产，--workflows 另指这里
└── artifacts/
    ├── report/<工单>.md         报告：写它的人或智能体写
    └── journal/<工单>.md        日志：叙事，人写
```

三处位置各归谁：**账本归 CLI**（缺省落 XDG 工作区键，程序每跑一趟都写，不入版控；`--data` 指到仓库就等于入版控），**产物归工作区**（缺省 `<工作区根>/artifacts/`，是内容，跟着工作区走），**位置不进模型**（工单文件里没有路径，换机器、挪仓库，重新装载即可）。跨工作区干活不必每条命令带 `--root`：设 `QTCLOUD_WORK_ROOT`，装载顺序为命令行 > 环境变量 > 向上搜索。

程序**不写产物的任何一节**：报告、日志都由写它们的人或智能体来写，落点由「产物落点 + 工单名」算出；判据里的 `{{report}}` / `{{journal}}` / `{{artifacts}}` 就指到这里。

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
