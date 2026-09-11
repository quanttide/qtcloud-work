# 用例

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
