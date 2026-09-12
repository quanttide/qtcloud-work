# Changelog

本文件仅记录 **studio（量潮知识工作云工作台，Flutter Web）** 的版本变更。

## [Unreleased]

- `lib/` 按 Bloc 家法重排：`models/`（界面要用的模型）、`repositories/`（接口 + 命令行客户端与本地两套实现，本地那套另带平台读写）、`screens/`、`widgets/`；`test/` 跟着分层
- toolkit 升到 `quanttide_work 0.1.0-beta.1`：判据与定义改成不可变值对象（`Criterion`、`Workflow.of`），`TaskDetail` / `WorkflowDetail` 从信封的 `data` 装配（`fromData`）
- 删死代码：`lib/core/artifact.dart`（无人引用）与 `tasks.dart` 里重复的 `textOf`
- 初始化全部平台客户端：android、ios、linux、macos、windows、web（原先只有 web）
- 按命令行现状实现 `lib/`：模型（工作流定义、任务记录、三处位置）与命令行客户端（子进程 + 统一信封）
- 按 `doc/` 实现界面：三屏（任务、流程、设置）+ 七个视图（侧栏、顶栏、对话、状态面板、步骤链、判据面板、定义态）
- 补 `test/`：44 个测试（模型、视图、屏、工作台冒烟），夹具是从命令行抓下来的真实输出
- Linux 客户端可构建可运行：`flutter build linux --release` 后直接跑
- 三处命令行还没给的东西记进 `doc/index.md`：对话、判据逐条文字、定义原文
