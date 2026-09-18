# 开发指南

程序怎么继续长。使用与命令看[使用指南](../user-guide/index.md)，字段与 schema 看[接口参考](../api-references/index.md)。

`src/` 顶层按三类落位：聚合 / 领域服务 / 适配。判据与命名规矩在 [`src/CONVENTIONS.md`](../../src/CONVENTIONS.md)；构建、门禁与依赖见 [README](../../README.md)。下面是落点图，图里每一项都对应实际目录或文件。

```text
src/
├── main.rs        入口：一行，把命令行交给 cli
├── cli.rs         适配：clap 定义、定位、发射
├── cli/           适配：命令树（commands.rs）、子命令分派（handlers/）与发射（emit.rs）
├── help.rs        适配：导览
├── prompts.rs     适配：给智能体的话术
├── health.rs      适配：provider 探活
├── outcome.rs     领域模型：结果信封
├── criterion/     领域模型：判据（模型 / 读法 / 翻成要跑什么）
├── executor.rs    领域模型：执行者取值
├── paths.rs       领域模型：占位
├── fields.rs      领域模型：定义字段表
├── error.rs       领域模型：定义读不通的错误
├── ids.rs         领域模型：凭证——新发与按名派生（uuid5，命名空间钉死）
├── clock.rs       领域模型：时刻
├── sha1.rs        领域模型：摘要（工作区键用）
├── events.rs      领域模型：领域事件落 JSONL
├── order/         聚合：工单（model / record 领域模型 + 账本、动作、看与列、交给 AI）
├── workflow/      聚合：工作流（model / read 领域模型 + yaml / actions）
├── catalog/       聚合：目录
├── artifact/      聚合：资产表（mod.rs）与产物实例（model.rs）
├── material/      聚合：材料
├── workspace/     聚合：工作区（装载与身份 locate / 落点 place / 核对 check / 流水判定 progress）
├── search/        领域服务：按名找文档
└── audit/         领域服务：判据与审计
```

## 聚合

有自己的定义：身份、生命周期、字段规矩。

- [work-order](work-order.md)——工单：封面、开单与销户、进度与完结的推导；
- [work-record](work-record.md)——工作记录：记账纪律、领域事件；
- [workflow](workflow.md)——YAML 定义、schema 与动作；
- [catalog](catalog.md)——扫成名字索引；
- [artifact](artifact.md)——资产表二十格与落点，产物实例（名字 + 规格）；
- [material](material.md)——材料的四字段与阶段；
- [workspace](workspace.md)——三处位置的装载、工作区身份与只读纪律。

## 领域服务

没有自己的定义，跨聚合只做一件事。

- [search](search.md)——按名找文档，依赖 `catalog`；
- [audit](audit.md)——跑机械判据、审计资产表与工作区。

## 适配

边界外的东西与入口。

- [adapter](adapter.md)——入口（`cli`）、导览（`help`）、给智能体的话术（`prompts`）、provider 探活（`health`）。

crate 在 `apps/qtcloud-work/src/cli/`，Rust 加 clap，二进制名 `qtcloud-work`。provider 的正式接口层等就位后另起一轮。
