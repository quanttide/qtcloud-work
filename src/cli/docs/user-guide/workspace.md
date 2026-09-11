# 查看工作区

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
├── tasks/<任务>.yaml           任务：start + workflow + 上下文 + 流水 + 闸门项 + 产物落点
└── artifacts/
    ├── report/<任务>.md        报告：执行记录 + 闸门项（程序维护），其余节归人写
    └── journal/<任务>.md       日志：叙事（人写）
```

工作流默认也放在数据仓的 `workflows/`。定义常是固定资产，一次执行的草稿（任务、流水、产物）落数据仓，用 `--workflows` 把定义指到固定资产目录（例如 `data/profile/iGuo/workflows/`）。

下面的用例都是真事。流水出自实验室的 `kg`（同一套做法，Python 写的原型），命令名按平台侧现在的写法给出；平台侧实现之后，这些命令要对齐着再走一遍。
