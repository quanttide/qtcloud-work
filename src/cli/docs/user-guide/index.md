# 知识工作命令行

平台侧的知识工作命令行，把实验室里跑通的那套本地做法扶正。中心是工作流与任务：工作流是串联的步骤（定义），任务是一次执行（实例）。底座是仓库里的 YAML 与 Markdown，不启服务、不连数据库。

命令名是 `qtcloud-work`。构建：

```bash
cargo build --release
# 产物：target/release/qtcloud-work
```

本指南讲怎么用。程序怎么继续长看[开发指南](../dev-guide/index.md)，每条命令与数据的字段看[接口参考](../api-references/index.md)。

动手之前的三条约定：**数据仓有默认**（不写 `--data` 就用当前目录下的 `data/`，开发环境的默认位置，不进版本库，用哪个会印到标准错误）；**写入型动作都可以先预演**（`--dry-run` 只说要写什么、不落盘）；**结果与错误分家**（结果与 `--json` 走标准输出，提示与错误走标准错误，脚本里读得干净）。每个命令都支持 `--json`，`--out <文件>` 另存一份。

## 走一遍

```bash
qtcloud-work workflow --new 课程档案比对 --steps 定位,比对,结论 --note "比对两边的档案"
qtcloud-work task --new 课程档案比对 --workflow 课程档案比对
qtcloud-work task 课程档案比对 --next
qtcloud-work task 课程档案比对
```

`workflow --new` 写下一份 YAML；`task --new` 起一件任务，备好报告与日志；`--next` 走下一步，程序按这一步的执行者分派；`task <名字>` 看步骤状态与流水。

## 工作流：串联的步骤

工作流是定义，落在 `<数据仓>/workflows/<名字>.yaml`，文件名即工作流名。步骤按出现顺序衔接，每步写着谁做（`executor`）与怎么算完（`criteria`）。

- `qtcloud-work workflow --list` 列出工作流、各自的步骤与位置；
- `qtcloud-work workflow --new <名字> --steps 甲,乙,丙 [--note 一句话]` 写一条工作流，每步给一份判据骨架（一条 rule 加一条 human）；
- `qtcloud-work workflow <名字>` 看步骤、谁执行、几条 rule / agent / human；
- `qtcloud-work workflow <名字> --export <文件>` 把定义原样存成一份可带走的文件；
- `qtcloud-work workflow --import <文件> [--as 名字]` 导进来，先按 schema 验一遍，重名挡回去，用 `--as` 换名。

工作流是数据不是代码。加一步、减一步、改判据、换执行者，动这份 YAML 就行，程序一行不用改；进 git 能 diff、能回退。字段与取值由 schema 定死，不认识的字段直接报错，不是「像不像」，是合不合语法。

## 任务：走下一步

任务是一次执行，落在 `<数据仓>/tasks/<任务>.yaml`。同一份工作流可以起任意多件任务，各有各的步骤状态、流水与产物，互不串。

- `qtcloud-work task --list` 列出任务、跑哪条工作流、下一步；
- `qtcloud-work task --new <名字> --workflow <工作流>` 起一件任务，工作流不在就挡；
- `qtcloud-work task <名字>` 看步骤状态、开工、工作流、指令与产物路径、最近五条流水、下一步；
- `qtcloud-work task <名字> --next` 走下一步；
- `qtcloud-work task <名字> --done <步骤> [--note 一句话]` 人做完了一步，自己记一笔；
- `qtcloud-work task <名字> --journal <一段话>` 日志收叙事（人写）。

`--next` 按这一步的执行者分派。执行者是 `agent`（默认，能用 AI 跑的都用 AI）就把这一步交给 `pi` 跑，跑完程序自己核判据；执行者是 `human` 就不抢着做，提示轮到你，做完用 `--done` 记一笔。判据不许 AI 写、不许 AI 改——改了就是自评自过。AI 没跑成（`pi` 不在、超时、产物没落成），这一步不算过，流水留 ✗，修好再来。

运行上下文随任务走：工作区根、数据仓、工作流目录（`root` / `data` / `workflows`）记在任务文件里。此后 `task <名字>` 不写 `--root` / `--workflows` 就用记着的，不靠全局配置，也不靠你现在在哪；工作区上的动作（查看工作区那一族）不写 `--root` 就用当前目录，并把用的是哪个根印在第一行。

## 查看工作区

工作区层面的四个只读动作：

- `qtcloud-work find <名字> [--show]` 按名找文档，认文件名与篇内一级标题，精确不中模糊兜底，`--show` 连正文一起看；
- `qtcloud-work catalog [--json] [--out 文件]` 按资产种类列出全部条目，`--out` 落一份 `{root, count, entries}`；
- `qtcloud-work audit [--json] [--out 文件] [--make]` 审计「资产表有而工作区无」与「工作区有而未登记」，`--make` 补建缺的文档格，独立仓库那三格不凭空建；
- `qtcloud-work material [路径…] [--json] [--out 文件]` 列材料的四字段与阶段，不给路径就扫 `data/journal` 与 `data/profile` 下的 md。

## 判据：谁判

判据挂在步骤上，按谁判分三类。`rule` 由程序按字段判，四种判法：`path` 存在、`absent` 不存在、`file` 加 `contains` 文件含某段文字、`run` 命令退出码为零；路径相对工作区根，写绝对路径则按绝对路径。`agent` 由智能体照说明审，逐条回答通过或不通过。`human` 不跑，原样进报告的闸门项等人拍板。

一步算过，等于这一步所有 `rule` 通过、所有 `agent` 判为通过；`human` 只列闸门，不影响过不过。动作结果 `ok=False` 时命令退出码为 1，报告仍照写。

## 数据：三家分放

```text
<数据仓>/
├── tasks/<任务>.yaml           任务：start + workflow + 运行上下文 + 流水（log 就在文件里）
└── artifacts/
    ├── report/<任务>.md        报告：执行记录 + 闸门项（程序维护），其余节归人写
    └── journal/<任务>.md       日志：叙事（人写）
```

工作流默认也放在数据仓的 `workflows/`。定义常是固定资产，一次执行的草稿（任务、流水、产物）落数据仓，用 `--workflows` 把定义指到固定资产目录（例如 `data/profile/iGuo/workflows/`）。

下面的用例都是真事。流水出自实验室的 `kg`（同一套做法，Python 写的原型），命令名按平台侧现在的写法给出；平台侧实现之后，这些命令要对齐着再走一遍。

## 用例 一、起一件任务并走一步

2026-09-10 晚上试 `AI冒烟`。工作流只有一步「问候」，执行者是 AI，判据一条 rule：问候文件含「你好」。

```bash
qtcloud-work task --new AI冒烟 --workflow AI冒烟
qtcloud-work task AI冒烟 --next
```

`--next` 把「写一行中文问候」交给 `pi`。AI 写下「你好」，程序随后核对判据、过、记一笔：

```text
问候：交给 AI（agent）跑
  AI 跑完了：我在 …/问候.md 写入了一行中文问候「你好」。
✓ 问候：问候落在
  ✓ 问候落在（…/问候.md 含「你好」）
```

当日 22:11 又跑了一遍同一件任务，用来看字段化后的判据还判不判得动。两次流水都 ok。

## 用例 二、三类判据各判各的

同一晚试 `三类判据`。工作流一步「写一句」，挂三条判据：rule 看产物落成、agent 审「内容是中文且只有一行」、human 留「创始人认可」。

```bash
qtcloud-work task --new 三类判据 --workflow 三类判据
qtcloud-work task 三类判据 --next
```

rule 当场判过，agent 判「✓」，human 没跑——原样进报告的闸门项：

```text
✓ 写一句：AI 执行：我在 …/话.md 写了一行中文：「规矩是死的，人是活的，但都得落到实处。」
  ✓ 话落在（path:…/话.md）
  ✓ 内容是中文且只有一行（…）
  ⧗ 创始人认可（留给人）
```

三种判据各判各的，谁判就写谁，一步里的三份结论互不冒充。

## 用例 三、比对两份课程档案

2026-09-10 有一件真事：比对 work 侧的个人课程草稿与课程研发档案，看两边口径、重叠、缺口与格式差。先在实验室手工走完 `课程档案比对`（定位、比对、结论三步，各用 `--done` 记一笔）；后来把同一件事写成工作流 `compare-course-profile`，定义放固定资产目录 `data/profile/iGuo/workflows/`，任务落草稿区：

```bash
qtcloud-work --data data/context/qtcloud-work \
  --workflows data/profile/iGuo/workflows \
  task --new compare-course-profile --workflow compare-course-profile
qtcloud-work --data data/context/qtcloud-work task compare-course-profile --next
```

三步都交给 AI。`locate` 找齐两边档案，`compare` 在报告里逐项对照，`conclude` 写下处置建议。每条 rule 判据当场核过，最后一步留一道 human 闸门「创始人点头（回流与并法怎么定）」。任务文件里记着 root / data / workflows，后续 `task compare-course-profile` 不必再写。

## 用例 四、把语境条目收进材料

2026-09-11 把「语境条目粗加工进材料库」这条日常流程写成工作流 `context-to-profile`，五步：`pull` 拉语境并把条目清单写进报告，`classify` 逐条认分类，`coarsen` 粗加工写进 `materials/<分类>/index.md`，`move-out` 把已迁出的条目从语境删掉，`commit` 分层提交推送再回工作区更新指针。

```bash
qtcloud-work --data data/context/qtcloud-work \
  --workflows data/profile/iGuo/workflows \
  task context-to-profile --next
```

实跑一遍：四条语境条目里三条已在材料里，只补了缺的那条。`classify` 与 `coarsen` 各有一条 agent 审查，两道 human 闸门（分类裁决、创始人点头）挂进报告。末条 `commit` 的判据原先写「工作区里两个指针都已记录」，当场判不过——程序的记账总在提交之后落笔，仓库永远带脏；改成只盯指针（`--ignore-submodules=dirty`）才判得动。判据是给自己立的，立完要真跑一遍。

## 用例 五、人做的步骤人记一笔

有些步骤是人做的，程序不抢着做，做完自己记一笔。2026-09-10 的 `数据归仓` 就是这么走的：工作流有材料、指令等步骤，人做完两步，用 `--done` 各记一笔。

```bash
qtcloud-work task 数据归仓 --done 材料 --note "AGENTS.md、日志"
qtcloud-work task 数据归仓 --done 指令 --note "目标/步骤/验收 已写"
```

`课程档案比对` 的三步也是这么记的（定位、比对、结论各一条 note）。`--done` 会照常跑 rule 判据、把 human 闸门列进报告，只是不替人做这一步。

## 边界

不改工作区，除非你点了明确要写的动作（起任务、记材料、补建格子、导出）。不是服务，没有端点、没有数据库、没有后台进程。认名字与位置：名字（文件名加篇内标题）管找到，位置（资产与落点规则）管归属。数据只写数据仓。
