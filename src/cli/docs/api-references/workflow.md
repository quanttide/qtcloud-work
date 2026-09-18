# workflow

六个动作加定义的 schema。

## workflow create

```bash
qtcloud-work workflow create <名字> --steps 甲,乙,丙 [--note 一句话]
```

写一条工作流到 `<工作流目录>/<名字>.yaml`。每步给一份判据骨架：一条 rule（`path: data/journal/README.md`）加一条 human。`--note` 是工作流的一句话描述，不给就用一句默认。定义文件里不写凭证——读进来时按「工作区 id + 名字」现算。

## workflow show

```bash
qtcloud-work workflow show <名字> [--json]
```

看这条工作流的步骤、谁执行、各有几条 rule / agent / human；`--json` 的 `data` 带派生出的 `workflow_id` 与各步骤的 `step_ids`。

## workflow list

```bash
qtcloud-work workflow list
```

列出工作流、各自的步骤与文件位置。

## workflow check

```bash
qtcloud-work workflow check <名字>
```

核对这条定义的声明与判据对不对得上，只看写下的位置、不访问文件系统：

- 判据里 `path` / `file` 的路径须在工作区内；
- 描述里点到的小节须有 `contains` 判据覆盖——小节只认干净的名字（`##` 起的标题或引号里的短名），引号里的长句当叙述。

占位（`{{report}}` / `{{journal}}` / `{{artifacts}}`）是运行时按产物落点展开的，不是写下的位置，不核。有一件对不上就退出码 1。

## workflow export

```bash
qtcloud-work workflow export <名字> <文件>
```

把定义原样存成一份可带走的文件，步骤、执行者、判据一字不改。目标是目录时，存成目录下的同名文件。

## workflow import

```bash
qtcloud-work workflow import <文件> [--as <名字>]
```

把一份工作流导进来。先按 schema 验一遍，不是工作流的文件挡回来；重名挡回来，用 `--as` 换名。导入后落在 `<工作流目录>/`。

工作流是编排定义：一串步骤，每步写着谁做与怎么算完。

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

不认识的字段直接报错，不是忽略。顶层只认 `name` / `description` / `steps`，步骤只认 `name` / `description` / `executor` / `criteria`，判据只认 `executor` / `description` / `path` / `absent` / `file` / `contains` / `run`。定义文件里不写 `id`——写下的凭证不是派生的凭证。

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

执行判定：一步算过，等于这一步所有 `rule` 通过、所有 `agent` 判为通过；`human` 只进待拍板清单，不影响过不过，放行须出自人（`order done`）。`order next` 走机器路径：`agent` 审查由智能体逐条答「通过 / 不通过 加一句理由」；AI 没跑成不记账，修好重走。带闸门的站，程序核完自己的半程就等人——这一笔不记。

## 占位

判据里可写占位，执行时按「产物落点 + 工单名」换成本次行程的真实路径（相对工作区根，跨仓则绝对），所以工作流不写死工单名：

- `{{report}}` 本趟的报告；
- `{{journal}}` 本趟的日志；
- `{{artifacts}}` 本趟的产物落点。

定义里不抄别处拥有的事实：会变的分类目录、落点、名字一律指过去，不抄进来。
