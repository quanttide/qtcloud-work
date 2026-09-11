# 接口参考

命令与数据的字段参考。怎么用看[使用指南](../user-guide/index.md)，程序分层看[开发指南](../dev-guide/index.md)。

每个子命令都能 `--help`，输出里有它做什么、一两个例子与选项；`-h` 只表示帮助。

## 全局选项

三个全局选项决定动作在哪跑，各自的来源写死，不许靠猜。

- `--root <路径>` 工作区根。任务上的动作不写时取任务里记的；工作区上的动作（`find` / `catalog` / `audit` / `material`）不写时取当前目录，并在输出的第一行印出用的是哪个根——不向上搜索、不凭仓库特征猜。
- `--data <路径>` 数据仓（任务与产物草稿）。任务上的动作不写时取任务里记的；`task --new` 与工作区上的动作必须显式给出，不给就报错并说明怎么给。
- `--workflows <路径>` 工作流目录，默认跟在数据仓里 `<数据仓>/workflows/`。定义常是固定资产，用这一项另指一处。

## 输出与退出码

标准输出只放结果与 `--json` 的 JSON，管道里不混进别的东西；提示、进度与错误一律走标准错误。每个动作都支持 `--json`：`--json` 把结果以 JSON 输出到标准输出，`--out <文件>` 另存一份到文件，两个可以一起用。

成功退出码 0，失败非 0（`ok=False` 为 1）；失败时标准错误的最后一行给一句「下一步敲什么」。结果为 `ok=False`，`lines` 照常打印。

写入型动作（`workflow --new` / `--import`、`task --new` / `--next` / `--done` / `--journal`、`audit --make`）都支持 `--dry-run`：把要写什么、写去哪打印出来，不落盘。`--next` 走一步时先打印一句「这一步交给谁」，Ctrl-C 立刻退且这一步不算过（流水留 ✗），交给 AI 的超时 900 秒。

## 契约与版本

`--json` 的字段与退出码是脚本依赖的契约，只加不改：要改先加新字段、后废旧的，旧字段先留一轮。`--version` 打印版本号，与 `CHANGELOG.md` 头一行、`scripts/validate-version.sh` 的校验一致。

## 查看工作区

### find

```bash
qtcloud-work find <名字> [--show]
```

按名找文档，认文件名与篇内一级标题。先精确匹配，不中再模糊兜底。`--show` 连正文一起打印；命中目录则列目录下的条目。找不到返回失败。

### catalog

```bash
qtcloud-work catalog [--json] [--out <文件>]
```

按资产种类列出工作区里的全部条目。`--out` 落一份目录快照：

```json
{
  "root": "quanttide-work",
  "count": 79,
  "entries": [{ "kind": "档案", "path": "data/profile", "names": ["profile", "档案"] }]
}
```

每条含种类、路径与全部名字。名字收资产的中英名、目录名、README 里的中文名、文档文件名与篇内一级标题。

### audit

```bash
qtcloud-work audit [--json] [--out <文件>] [--make]
```

审计两件事：资产表有而工作区无的格子，工作区有而未登记的顶层目录。`--make` 补建缺的文档格（在 `data/<名>` 或 `docs/<名>` 下建目录与 README），独立仓库那三格不凭空建；`--make` 也吃 `--dry-run`。`--out` 落 `{root, result, missing, unregistered}`。

### material

```bash
qtcloud-work material [<路径>…] [--json] [--out <文件>]
```

列出材料的四字段与阶段。不给路径就扫 `data/journal` 与 `data/profile` 下的 md。四字段：

| 字段 | 取值 |
| :-- | :-- |
| `type` | 扩展名 |
| `content` | 正文首段截 40 字（正文取 md / txt / rst） |
| `source` | 上级目录加文件名 |
| `created_at` | 文件所在仓库首次提交的日期，退回文件名里的日期 |

阶段由位置承担：落在 `journal` 下的是「原始」，其余是「材料」。`--out` 落 `{count, materials}`，每条是 `path` 加这四个字段与 `stage`。

## 工作流

### workflow --list

```bash
qtcloud-work workflow --list
```

列出工作流、各自的步骤与文件位置。

### workflow --new

```bash
qtcloud-work workflow --new <名字> --steps 甲,乙,丙 [--note 一句话]
```

写一条工作流到 `<工作流目录>/<名字>.yaml`。每步给一份判据骨架：一条 rule（`path: data/journal/README.md`）加一条 human。`--note` 是工作流的一句话描述，不给就用一句默认。

### workflow <名字>

```bash
qtcloud-work workflow <名字>
```

看这条工作流的步骤、谁执行、各有几条 rule / agent / human。

### workflow <名字> --export <文件>

```bash
qtcloud-work workflow <名字> --export <文件>
```

把定义原样存成一份可带走的文件，步骤、执行者、判据一字不改。目标是目录时，存成目录下的同名文件。

### workflow --import <文件> [--as <名字>]

```bash
qtcloud-work workflow --import <文件> [--as <名字>]
```

把一份工作流导进来。先按 schema 验一遍，不是工作流的文件挡回来；重名挡回来，用 `--as` 换名。导入后落在 `<工作流目录>/`。

## 任务

### task --list

```bash
qtcloud-work task --list
```

列出数据仓里的任务、各自跑哪条工作流、下一步是什么。

### task --new

```bash
qtcloud-work task --new <名字> --workflow <工作流>
```

起一件任务：工作流不在就挡；任务名撞上已有的就挡，不覆盖。写下 start、运行上下文（`root` / `data` / `workflows`）与空流水，备好报告与日志。同一份工作流可以起多件任务。

### task <名字>

```bash
qtcloud-work task <名字>
```

看步骤状态、开工、工作流、步骤数、指令与产物路径、最近五条流水、下一步。运行上下文用任务里记的，`--root` / `--workflows` 可省。

### task <名字> --next

```bash
qtcloud-work task <名字> --next
```

走下一步。执行者是 agent 就把这一步交给 `pi` 跑，跑完核判据；执行者是 human 就提示轮到你。随后跑 rule 判据、交 agent 审、列 human 闸门、记一笔流水、写报告。所有步骤都走过则报「所有步骤都走过了」。

### task <名字> --done <步骤> [--note 一句话]

```bash
qtcloud-work task <名字> --done <步骤> [--note 一句话]
```

人为地记一步，不交给 AI。照常跑 rule 判据、列 human 闸门；`--note` 是这一步做了什么，省了按判据拼一句。

### task <名字> --journal <一段话>

```bash
qtcloud-work task <名字> --journal <一段话>
```

日志收叙事。先去掉模板里的占位行，再追加这一段。日志要人来写，空话挡回去。

## 工作流 schema

工作流是 YAML，顶层三个字段：

| 字段 | 必填 | 说明 |
| :-- | :-- | :-- |
| `name` | 是 | 工作流名，与所在目录的文件名一致 |
| `description` | 否 | 一句话说清这条工作流干什么 |
| `steps` | 是 | 步骤的非空列表，顺序即衔接顺序 |

步骤四个字段：

| 字段 | 必填 | 说明 |
| :-- | :-- | :-- |
| `name` | 是 | 步骤名 |
| `description` | 否 | 这一步做什么，给执行者看 |
| `executor` | 否 | `agent` 或 `human`，默认 `agent` |
| `criteria` | 否 | 判据列表 |

拓扑上按有向无环图理解，当前实现按顺序走，顺序即排序。

不认识的字段直接报错，不是忽略。顶层只认 `name` / `description` / `steps`，步骤只认 `name` / `description` / `executor` / `criteria`，判据只认 `executor` / `description` / `path` / `absent` / `file` / `contains` / `run`。

## 任务 schema

任务是一份 YAML，落 `<数据仓>/tasks/<任务>.yaml`，一个任务一个文件，文件名即任务名。

| 字段 | 说明 |
| :-- | :-- |
| `name` | 任务名 |
| `start` | 开工时间，`%Y-%m-%d %H:%M` |
| `workflow` | 跑哪条工作流，任务与工作流之间唯一的链接 |
| `root` | 工作区根，绝对路径 |
| `data` / `workflows` | 数据仓与工作流目录，能相对工作区根就相对 |
| `log` | 流水，只增不改 |

流水一条四项：`at` 时刻、`step` 步骤名、`detail` 一句话、`ok` 是否成功。状态从流水推出来：流水里 `ok` 为真且步骤名在定义里的，算走过的步骤；第一个没走过的就是下一步。

## 判据

判据三个字段：`executor`、`description` 与判法。`executor` 答谁判，取 `rule`、`agent` 或 `human`。

`rule` 由规则引擎按字段判，四种判法各一种，路径相对工作区根，写绝对路径则按绝对路径：

| 判法 | 意思 |
| :-- | :-- |
| `path` | 该路径存在 |
| `absent` | 该路径不存在 |
| `file` 加 `contains` | 该文件含这段文字，两字段成对 |
| `run` | 该命令在工作区根退出码为零 |

`agent` 与 `human` 只写 `description`：前者是给智能体的判准，后者是留给人拍板的事项，不许带规则字段。`description` 可省，省了按判法生成一句。

schema 约束：`rule` 必须正好一种判法；`file` 与 `contains` 成对；`agent` 与 `human` 必须写 `description`；步骤的 `executor` 只能是 `agent` 或 `human`。

执行判定：一步算过，等于这一条所有 `rule` 通过、所有 `agent` 判为通过；`human` 只列闸门，不影响过不过。AI 没跑成则该步不算过，流水留 ✗。`agent` 审查由智能体逐条答「通过 / 不通过 加一句理由」，同一步的执行者与审查者若同一个智能体，流水注明「同一模型」。

## 占位

判据里可写占位，执行时换成本次任务的真实路径（相对工作区根，跨仓则绝对），所以工作流不写死任务名：

- `{{report}}` 本任务的报告；
- `{{journal}}` 本任务的日志；
- `{{log}}` 本任务的流水（就在任务文件里）；
- `{{artifacts}}` 本任务的产物目录。

## 文件布局

```text
<数据仓>/
├── workflows/<工作流>.yaml      定义：步骤与判据（可用 --workflows 另指）
├── tasks/<任务>.yaml            任务：start + workflow + 运行上下文 + log
└── artifacts/
    ├── report/<任务>.md         报告：程序维护「执行记录」「闸门项」两节
    └── journal/<任务>.md        日志：叙事，人写
```

报告只被程序改那两节，其余节原样保留；日志收叙事时先去掉模板占位行再追加。

## 结果与 JSON

每个动作算出一个结果，四项：`ok` 通不通、`lines` 命令行要打印的话、`columns` 与 `rows` 给窗口画的同一份表格。命令行的 `--json` 与窗口都从这层取，算法只写一遍。

### 已知未接线

`task --new` 的 `--about`（一句话说这次要什么）在实验室里被解析了却没有接下去，本轮不替它编行为，接口参考也不列它。

## 资产与目录

资产表二十格：陈述型 11（报告、参考、历史、日志、档案、宣传册、路线图、洞察、意图、语境、归档），程序型 9（章程、规格、工具箱、手册、案例、平台、教程、札记、实验室）。每格有一对名字：中文名与英文名。

落点：文档类入 `data/<英文名>` 或 `docs/<英文名>`，独立仓库按命名规则找——工具箱 `packages/*-toolkit`、平台 `apps/*`、实验室 `examples/*`。

目录层扫描时跳过 `.git`、`node_modules`、`.venv`、`build`、`dist`、`.dart_tool`、`__pycache__` 这些目录，以及 `README.md`、`CHANGELOG.md`、`LICENSE` 门面文件。
