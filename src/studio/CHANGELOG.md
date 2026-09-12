# Changelog

本文件仅记录 **studio（量潮知识工作云工作台，Flutter Web）** 的版本变更。

## [Unreleased]

- `lib/` 按 Bloc 家法重排：`repositories/`（接口 + 命令行客户端与本地两套实现）、`states/`（Bloc）、`screens/`、`widgets/`；`test/` 跟着分层
- 状态交给 Bloc：`states/workbench_bloc.dart`（状态 + 事件 + Bloc 一个文件）取代 `StatefulWidget` + `setState`；界面件只认传进来的模型与回调
- 加 `repositories/studio_repository.dart` 接口（任务 / 工作流 / 探活）：`client.dart`（读统一信封）与 `local/local_repository.dart`（直接调命令面）各实现一套
- toolkit 升到 `quanttide_work 0.1.0-beta.2`（按聚合重写）：判据与定义用不可变值对象，「走过」的算法改调任务聚合（`qt.Task.doneSteps / nextStep / stateLine`），定义核对改调 `flow.check`
- 信封（`Outcome` / `short`）、定义核对的拼句（`describeFindings` / `allOk`）、提示词（`prompts`）从工具箱搬回工作台：`repositories/local/{outcome,prompts}.dart`
- 干掉 `lib/models/`：界面与 Bloc 直接拿工具箱的领域对象（`qt.Task` / `qt.Workflow`），走过几步、下一步、进度、判据条数由领域对象自己算；执行者的中文说法搬进 `lib/widgets/executor_label.dart`；信封那一栏 `data` 改托任务与定义的原文
- 删死代码：`lib/core/artifact.dart`（无人引用）与 `tasks.dart` 里重复的 `textOf`
- 初始化全部平台客户端：android、ios、linux、macos、windows、web（原先只有 web）
- 按命令行现状实现 `lib/`：模型（工作流定义、任务记录、三处位置）与命令行客户端（子进程 + 统一信封）
- 按 `doc/` 实现界面：三屏（任务、流程、设置）+ 七个视图（侧栏、顶栏、对话、状态面板、步骤链、判据面板、定义态）
- 补 `test/`：44 个测试（模型、视图、屏、工作台冒烟），夹具是从命令行抓下来的真实输出
- Linux 客户端可构建可运行：`flutter build linux --release` 后直接跑
- 三处命令行还没给的东西记进 `doc/index.md`：对话、判据逐条文字、定义原文
