# 用例

下面的用例都是真事。流水出自实验室的 `kg`（同一套做法，Python 写的原型），命令名按平台侧现在的写法给出；平台侧实现之后，这些命令要对齐着再走一遍。

## 用例 一、起一件工单并走一步

2026-09-10 晚上试 `AI冒烟`。工作流只有一步「问候」，执行者是 AI，判据一条 rule：问候文件含「你好」。

```bash
qtcloud-work order create AI冒烟 --workflow AI冒烟
qtcloud-work order next AI冒烟
```

`order next` 把「写一行中文问候」交给 `pi`。AI 写下「你好」，程序随后核对判据、过、记一笔：

```text
问候：交给 AI（agent）跑
  AI 跑完了：我在 …/问候.md 写入了一行中文问候「你好」。
✓ 问候：问候落在
  ✓ 问候落在（…/问候.md 含「你好」）
```

当日 22:11 又跑了一遍同一件工单，用来看字段化后的判据还判不判得动。两次流水都 ok。

## 用例 二、三类判据各判各的

同一晚试 `三类判据`。工作流一步「写一句」，挂三条判据：rule 看产物落成、agent 审「内容是中文且只有一行」、human 留「创始人认可」。

```bash
qtcloud-work order create 三类判据 --workflow 三类判据
qtcloud-work order next 三类判据
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

2026-09-10 有一件真事：比对 work 侧的个人课程草稿与课程研发档案，看两边口径、重叠、缺口与格式差。先在实验室手工走完 `课程档案比对`（定位、比对、结论三步，各用 `order done` 记一笔）；后来把同一件事写成工作流 `compare-course-profile`，定义放固定资产目录 `data/profile/iGuo/workflows/`，工单开在账本里：

```bash
qtcloud-work --data data/context/qtcloud-work \
  --workflows data/profile/iGuo/workflows \
  order create compare-course-profile --workflow compare-course-profile
qtcloud-work --data data/context/qtcloud-work order next compare-course-profile
```

三步都交给 AI。`locate` 找齐两边档案，`compare` 在报告里逐项对照，`conclude` 写下处置建议。每条 rule 判据当场核过，最后一步留一道 human 闸门「创始人点头（回流与并法怎么定）」。位置不进模型：账本与产物由启动参数装载，后续命令带同一组参数即可。

## 用例 四、把语境条目收进材料

2026-09-11 把「语境条目粗加工进材料库」这条日常流程写成工作流 `context-to-profile`，五步：`pull` 拉语境并把条目清单写进报告，`classify` 逐条认分类，`coarsen` 粗加工写进 `materials/<分类>/index.md`，`move-out` 把已迁出的条目从语境删掉，`commit` 分层提交推送再回工作区更新指针。

```bash
qtcloud-work --data data/context/qtcloud-work \
  --workflows data/profile/iGuo/workflows \
  order create context-to-profile --workflow context-to-profile
qtcloud-work --data data/context/qtcloud-work order next context-to-profile
```

实跑一遍：四条语境条目里三条已在材料里，只补了缺的那条。`classify` 与 `coarsen` 各有一条 agent 审查，两道 human 闸门（分类裁决、创始人点头）挂进待拍板清单。末条 `commit` 的判据原先写「工作区里两个指针都已记录」，当场判不过——程序的记账总在提交之后落笔，仓库永远带脏；改成只盯指针（`--ignore-submodules=dirty`）才判得动。判据是给自己立的，立完要真跑一遍。

## 用例 五、闸门放行

有些步骤是人做的，程序不抢着做，人做完用 `order done` 放行。2026-09-10 的 `数据归仓` 就是这么走的：工作流有材料、指令等步骤，人做完两步，各记一笔。

```bash
qtcloud-work order done 数据归仓 材料 --note "AGENTS.md、日志"
qtcloud-work order done 数据归仓 指令 --note "目标/步骤/验收 已写"
```

`compare-course-profile` 的收尾也是这么放的（「创始人点头」那道闸，note 写下点头的口径）。`order done` 会照常跑 rule 判据、把原话记进流水，只是不替人做这一步。

## 用例 六、流水只增不改

2026-09-11 `review-ui-shot` 的「第一眼」站：第一笔记的是没过（`is_succeeded: false`）；补完复盘，同一站重走一笔、记过。账上两笔都在——旧账原样在账上，以最新一条为准。

```bash
qtcloud-work order next review-ui-shot      # 第一笔：判 ✗，照记
qtcloud-work order next review-ui-shot      # 重走：判 ✓，追加在后
qtcloud-work order journal review-ui-shot   # 两笔都在账上
```

想抹掉重写？账本拒绝：同 `id` 即拒（复制的账）、`seq` 跳号即拒（被抽走的账）、时刻倒流即拒（倒流的账不可信）。流水是账，账不改。

## 用例 七、凭证按名派生

写 `AI冒烟` 这条工作流时，没编过任何 UUID——定义文件里不写 `id`，读进来时按「工作区 id + 名字」现算（uuid5，命名空间钉死，各平台算得一致）。

```bash
qtcloud-work workflow create AI冒烟 --steps 问候
qtcloud-work workflow show AI冒烟 --json   # data.workflow_id 是现算出来的
```

同一个工作区里读一百次是同一枚；同名工作流在另一个工作区，凭证就换了一枚——账不会记串。工单与工作记录的凭证由程序发，落笔后不变。

## 用例 八、产物落点

报告与日志是内容，账本是账，两处分家：产物落 `<工作区根>/artifacts/`，账本归 CLI（缺省 XDG 工作区键）。定义里的 `{{report}}` 就指到产物落点——判据 `file: '{{report}}'` 按「产物落点 + 工单名」展开核对，换机器、挪仓库，产物跟着工作区走，账本各落各的。

```bash
qtcloud-work order create 写报告 --workflow 写报告   # 产物落点缺省 <根>/artifacts/
qtcloud-work --artifacts /tmp/这一趟 order next 写报告   # 指到哪就落哪
```
