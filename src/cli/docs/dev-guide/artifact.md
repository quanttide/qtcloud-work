# artifact：资产表

`src/artifact/` 装的是第二大脑资产表，不是任务的产物。

## 落点

`artifact/mod.rs`——二十格与落点规则。`Asset { category, name }`，一对名字：中文名与英文名。

## 二十格

`STATED`（陈述型 11）：报告 report、参考 library、历史 history、日志 journal、档案 profile、宣传册 brochure、路线图 roadmap、洞察 insight、意图 intention、语境 context、归档 archive。

`PROCEDURAL`（程序型 9）：章程 bylaw、规格 specification、工具箱 toolkit、手册 handbook、案例 gallery、平台 platform、教程 tutorial、札记 essay、实验室 example。

## 落点

文档类入 `data/<英文名>` 或 `docs/<英文名>`；独立仓库的三格按命名规则找——工具箱 `packages/*-toolkit`、平台 `apps/*`、实验室 `examples/*`。

## 用途

- `missing`——资产表有而工作区无的格子（[audit](audit.md) 用）；
- `make`——补建缺的格子，独立仓库那三格不凭空建；
- `repo_root`——从当前目录往上找含 `data/journal` 的仓库根。

这一格管工作区的文档格，与规格里的「产物」（名字 + 规格）、「产物类别」不是一物；`artifact` 一名在两侧指的不是同一件东西。

## 决定

这一层是资产表，`artifact` 名留给规格里的产物；本目录改名 `asset/`。动手见 [TODO](../../TODO.md)。
