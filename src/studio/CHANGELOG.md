# Changelog

本文件仅记录 **studio（量潮知识工作云工作台，Flutter Web）** 的版本变更。

## [Unreleased]

- 初始化全部平台客户端：android、ios、linux、macos、windows、web（原先只有 web）
- 按命令行现状实现 `lib/`：模型（工作流定义、任务记录、三处位置）与命令行客户端（子进程 + 统一信封）
- 补 `test/`：24 个测试，含从命令行抓下来的真实输出夹具；任务页冒烟测试
