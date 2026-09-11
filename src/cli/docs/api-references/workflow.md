# workflow

六个动作加定义的 schema。

## workflow --list

```bash
qtcloud-work workflow --list
```

列出工作流、各自的步骤与文件位置。

## workflow --new

```bash
qtcloud-work workflow --new <名字> --steps 甲,乙,丙 [--note 一句话]
```

写一条工作流到 `<工作流目录>/<名字>.yaml`。每步给一份判据骨架：一条 rule（`path: data/journal/README.md`）加一条 human。`--note` 是工作流的一句话描述，不给就用一句默认。

## workflow <名字>

```bash
qtcloud-work workflow <名字>
```

看这条工作流的步骤、谁执行、各有几条 rule / agent / human。

## workflow <名字> --check

```bash
qtcloud-work workflow <名字> --check
```

核对这条定义的声明与判据对不对得上：判据里的路径（`path` / `file`）在不在工作区里；描述里提到的报告小节（`## 名字` 或「名字」一节）有没有判据覆盖。按任务落点的占位（`{{report}}` / `{{journal}}` / `{{log}}`）在定义这一层核不了，跳过。有一件对不上就退出码 1。

## workflow <名字> --export <文件>

```bash
qtcloud-work workflow <名字> --export <文件>
```

把定义原样存成一份可带走的文件，步骤、执行者、判据一字不改。目标是目录时，存成目录下的同名文件。

## workflow --import <文件> [--as <名字>]

```bash
qtcloud-work workflow --import <文件> [--as <名字>]
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

不认识的字段直接报错，不是忽略。顶层只认 `name` / `description` / `steps`，步骤只认 `name` / `description` / `executor` / `criteria`，判据只认 `executor` / `description` / `path` / `absent` / `file` / `contains` / `run`。

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

执行判定：一步算过，等于这一步所有 `rule` 通过、所有 `agent` 判为通过；`human` 只列闸门，不影响过不过。人为地记一步（`--done`）时不跑智能体，`agent` 判据算**待判**——不挡这一步，原样进闸门项等人看。AI 没跑成则该步不算过，流水留 ✗。`agent` 审查由智能体逐条答「通过 / 不通过 加一句理由」，同一步的执行者与审查者若同一个智能体，流水注明「同一模型」。

## 占位

判据里可写占位，执行时换成本次任务的真实路径（相对工作区根，跨仓则绝对），所以工作流不写死任务名：

- `{{report}}` 本任务的报告；
- `{{journal}}` 本任务的日志；
- `{{log}}` 本任务的流水（就在任务文件里）；
- `{{artifacts}}` 本任务的产物目录。
