# 命令面

每个动作的参数与行为。全局三处位置与输出约定看[入口](index.md)。

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

### workflow <名字> --check

```bash
qtcloud-work workflow <名字> --check
```

核对这条定义的声明与判据对不对得上：判据里的路径（`path` / `file`）在不在工作区里；描述里提到的报告小节（`## 名字` 或「名字」一节）有没有判据覆盖。按任务落点的占位（`{{report}}` / `{{journal}}` / `{{log}}`）在定义这一层核不了，跳过。有一件对不上就退出码 1。

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
