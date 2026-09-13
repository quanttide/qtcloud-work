# 开发指南

程序怎么继续长。使用与命令看[使用指南](../user-guide/index.md)，字段与 schema 看[接口参考](../api-references/index.md)（命令参考与概念参考两类）。

这份指南按四篇拆开：

- [分层](layers.md)——一层一件事、动作与对象同住；
- [状态机与数据流](state.md)——状态从流水推出来、走一步做了什么；
- [测试与门禁](testing.md)——五类测试与五条门禁命令；
- [依赖与许可](dependencies.md)——每个依赖的用途与许可；
- [待决事项](decisions.md)——尚未定的决策，按背景 / 选项 / 影响 / 建议梳理；
- [扩展与边界](extending.md)——加动作、加判据、接 provider，以及不做什么。

crate 在 `apps/qtcloud-work/src/cli/`，Rust 加 clap，二进制名 `qtcloud-work`。当前只有一条 provider 探活命令；这一轮把实验室 `kg` 里跑通的本地知识工作做法搬进来，provider 的接口层等就位后另起一轮。

```text
src/cli/
├── Cargo.toml          包装：bin 名 qtcloud-work
├── src/                源码（分层见下）
├── tests/              用例测试：一个场景一个文件，夹具在 common/
├── scripts/            版本与变更校验（发布用）、用例对账
└── docs/               user-guide / dev-guide / api-references
```
