# 状态：studio 对照软件工程契约

体检日 2026-09-12。条款来自 [契约原型](../../../../code/data/insight/code-agent/contract.md)（结构契约、依赖契约、阶段条款）与 code 手册的平台契约。

## 规模

| 项 | 数 |
|---|---|
| `lib/` Dart 文件 | 35 |
| `lib/` 总行数 | 3 675 |
| 最长的四个文件 | `repositories/local/tasks.dart` 378、`states/workbench_bloc.dart` 349、`repositories/local/workflows.dart` 316、`repositories/local/task_run.dart` 279 |
| `test/` | 与 `lib/` 同构：repositories / states / screens / widgets + support / fixtures |

`parity.sh` 现值：24 条命令两侧结果一致（命令行 Rust 与 studio Dart 各跑一次，比 `ok`／`columns`／`rows`／`data`）。

## 结构契约

| 条款 | 现状 | 判定 |
|---|---|---|
| 借家法要连命名一起借（那一层叫 `views`） | `lib/widgets/` 8 个文件；`doc/views/` 已叫 views——同一层两个名字 | ✗ 待正名 |
| 同一目录不得混用层名与聚合名（硬禁） | `repositories/local/` 里聚合名（`tasks` `workflows` `task_run`）与职能名（`dispatch` `rules` `help` `prompts` `paths` `yaml` `environment*` `fs/` `host/`）并存 | ✗ 违规 |
| 单文件行数越界即触发转聚合 | 4 个文件 >250 行，最长 378 | ✗ 已触发 |
| 角色唯一 | `repositories/` 下两套实现并存：`client.dart`（起命令行）+ `local/`（自己算） | ✗ 待收敛 |
| 依赖单向（端侧引用工具箱，不反向定义） | `quanttide_work` 是依赖；`lib/` 无本地模型（`lib/models/` 已删） | ✓ |
| 不留第二份模型 | 界面直接用 `qt.Task`／`qt.Workflow`／`Outcome` | ✓ |
| 组装与实现分离 | `main.dart` 装配、`app.dart` 外壳；命令面在 `repositories/local/dispatch.dart` 一处实现（`bin/qtcloud.dart` 与界面都走它） | ✓ |
| 测试跟着分层 | `test/` 与 `lib/` 同构 | ✓ |
| 横切约束集中一处 | 无此物；约定散在 README、`pubspec.yaml` 注释与文件头注释里 | ✗ 缺 |

## 依赖契约

| 条款 | 现状 | 判定 |
|---|---|---|
| 抽出去必须发布、按版本号引 | `quanttide_work: ^0.1.0-beta.5`（非 path） | ✓ |
| 官方 SDK 优先 | 用 `yaml` 包解析，无自写解析器 | ✓ |
| 默认选型成文 | Bloc ✓、Material ✓；`go_router` ✗——现用 `app.dart` 自切屏 + `Navigator` | ✗ 未对齐 |
| 外部依赖的坑写进契约 | 构建命令无 `FLUTTER_WEB_CANVASKIT_URL`（框架手册要求自托管、发布前无代理网络验证） | ✗ 待核实 |
| 依赖许可清单 | 无一处列明已许可依赖（`flutter_bloc` `yaml` `cupertino_icons` `flutter_lints`） | ✗ 缺 |
| 一条发布线一个包 | 发布线未开（`cli/v*` 已开，studio 侧未定「一条线还是各发各的」） | — 待定 |

## 平台契约

| 条款 | 现状 | 判定 |
|---|---|---|
| 技术栈按角色 | Flutter Web + Bloc ✓ | ✓ |
| 桶命名 `{产品线}-{用途}` | `qtcloud-work-studio` | ✓ |
| 域名 `{产品}.cloud.quanttide.com` | `work.cloud.quanttide.com` | ✓ |
| IaC 目录 `manifests/terraform/` | `apps/qtcloud-work/` 下无 `manifests/` | ✗ 待确认归属 |
| 门禁：Dart = `dart format` + `flutter analyze` | CI 只跑 `flutter analyze` + `flutter test`，无 `dart format` | ✗ 缺一条 |
| 门禁本地从严与 CI 一致 | `analysis_options.yaml` 用 `flutter_lints`；CI 与本地同两条 | ✓ |
| 可观测与安全 | 无后端，不适用 | — 豁免 |

## 与工作流的对照

`data/profile/quanttide/workflows/code-implement-studio.yaml`（点货／搬运／对表／交接／收口）现状：

- 交接一步的判据写的是「起子进程调命令行那层（`lib/cli`）撤了」——**`lib/cli` 不存在**，实际那层在 `lib/repositories/client.dart` + `runner.dart`；判据要跟着现状改
- 交接的另一条判据 grep「命令行还没给」这类占位说明，`doc/` 与 README 里还有（见 `ROADMAP.md` 的文档欠账）
- 点货／搬运的尺子 `parity.sh` 可用 ✓，24 条一致
