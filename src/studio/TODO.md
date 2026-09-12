# TODO：把 studio 收到契约下

逐条列出要改的每一个细节。依据是 [STATUS.md](STATUS.md) 的体检结果；路线摘要见 [ROADMAP.md](ROADMAP.md)。

**每段做完的通用判据**（缺一不算完）：

```bash
cd apps/qtcloud-work/src/studio
flutter analyze && flutter test
sh scripts/parity.sh          # 24 条必须仍一致
```

---

## 段一 · 正名（零风险，先做）

**1.1** `lib/widgets/` → `lib/views/`（借家法要连命名一起借；`doc/views/` 已经叫 views）

- 改：`lib/widgets/*.dart`（8 件）移到 `lib/views/`；`test/widgets/` 同步改名；`app.dart`、`screens/*.dart` 的 import；README 目录图
- 判据：`test -d lib/views && test ! -d lib/widgets`；`grep -rn "widgets/" lib test README.md | grep -v "Widget"` 为空

**1.2** 把文件头注释里「命令行那份仍留着」的说法改掉（与段三同批做，见 3.1）

**1.3** 命令改名：`find` → `search`（判据：它带 `--show`，承诺的是"给你要找的东西"，不是"报位置"）

- 改：CLI `src/cli.rs` 分发、`src/catalog.rs` 的 `find()` 与 `help.rs`「工作区」组那条；studio `dispatch.dart` 的 `case 'find'`；两边 doc 里的命令参考（`docs/api-references`）、README；本仓 ROADMAP/TODO 里的模块名
- 判据：`grep -rn "\bfind\b" apps/qtcloud-work/src/studio/lib apps/qtcloud-work/src/cli/src/deliverable` 里只剩 Flutter/Dart 的 `find` 语义；`help` 那页写着 `search`
- 注意：命令面是对外承诺，`cli/v*` 发布线已开 → 属破坏性变更，走版本号与 CHANGELOG；studio 侧还没搬，现在改最便宜

## 段二 · 清混用（结构契约的硬禁）

**2.1** `lib/repositories/local/` 不再同层并存「聚合名」与「职能名」

- 现状：聚合名 `tasks` `workflows` `task_run`；职能名 `dispatch` `rules` `help` `prompts` `paths` `yaml` `environment*` `fs/` `host/`
- 改法（二选一，做完只留一种维度）：
  - **甲**：职能件下移一层——`local/platform/`（environment、fs、host、paths、yaml）+ `local/` 余下（dispatch、rules、help、prompts + 聚合件）
  - **乙**：聚合件上移一层——`local/task/`、`local/workflow/`、`local/run/`，职能件留在 `local/`
- 判据：`ls local/*.dart` 里不同时出现聚合名与职能名；一份聚合名单写进 `lib/CONVENTIONS.md`（见 4.3），门禁按名单查
- 影响：12 个文件 + import + `test/repositories/local/` 同构目录

## 段三 · 收敛两套实现（角色唯一）

**3.1** 撤掉起命令行那层：删 `repositories/client.dart` 与 `repositories/runner.dart`，界面全部走 `local/`

- 改：`app.dart` 装配处只留本地那套；screens 里 `CliFailure` 的用法换成 `LocalFailure`；删 `test/repositories/client_test.dart`、`test/support/fake_runner.dart`；`local/dispatch.dart` 头注释里「界面走 client.dart」的说法改掉；README「界面跑的是 studio 自己那份实现…命令行那份仍留着」整段改写；doc 里「命令行还没给」这类占位说明一并撤
- 判据：`test ! -f lib/repositories/client.dart`；`grep -rn "Process.run" lib/repositories | wc -l` 为 0（`local/host/` 里跑 `pi` 的不算）；通用判据全绿
- 注意：这一步是工作流 `code-implement-studio` 的「交接」，做完 studio 才算能自己把活干完

**3.2** 保留 `bin/qtcloud.dart` 作对表入口（studio 侧命令面）

- 判据：`sh scripts/parity.sh` 仍跑得起来、24 条一致

## 段四 · 转聚合（触发已命中）

触发已命中两条：单文件行数越界、目录混用。

**4.1** 拆长文件，重排 `local/`——靶子目录（见 ROADMAP「交付哪些模块」）：聚合 `workflow/`、`task/`、`catalog/`、`material/`、`workspace/`；领域服务 `search/`、`audit/`；适配一处 `platform/`

- 聚合名用业务名词，服务名用能力名（`search/` 而不是 `searcher/`——`-er` 是类名惯例，不是目录名惯例）；归类看**有没有自己的定义**，不看是不是动词（`catalog` 有定义故为聚合，`search`、`audit` 只做一件事故为服务）
- 依赖单向：**服务可依赖聚合，聚合不得依赖服务**（`search` 用 `catalog` 的名字索引）
- `tasks.dart` 378 行、`workflows.dart` 316 行、`task_run.dart` 279 行——拆进各自目录，单文件降到 **≤250 行**
- 判据：上述目录都在；`local/*.dart` 下不再有散落单件；`find lib -name "*.dart" | xargs wc -l | awk '$1>250'` 为空；通用判据全绿（**行为不变**是硬要求）

**4.2** `states/workbench_bloc.dart` 349 行——按 Bloc 家法拆（bloc / event / state）或按聚合拆

- 判据：同 4.1 的行数判据；`flutter test` 里 `workbench_bloc_test.dart` 不变绿不算过

**4.3** 横切约束集中一处：建 `lib/CONVENTIONS.md`

- 收进去：界面只认传进来的模型与回调（不自己派生）、错误怎么说、路径怎么显示、聚合名单、「views 而不是 widgets」
- 判据：文件在；README 不再兼任约定表

## 段五 · 契约对齐（对外）

**5.1** 路由：要么改用 `go_router`（平台契约要求），要么在平台契约里为 studio 写一条例外（三屏自切屏 + `Navigator`）

- 判据：`pubspec.yaml` 里有 `go_router`，或平台契约里有例外条款

**5.2** CI 补 `dart format`：`release-studio.yml` 门禁加 `dart format --set-exit-if-changed .`

- 判据：工作流里有这一条；本地跑同一条也绿

**5.3** CanvasKit 自托管：先在无代理网络实测页面能否完整渲染；不能则构建时加 `--dart-define=FLUTTER_WEB_CANVASKIT_URL=/canvaskit/` 并自托管资源

- 判据：无代理网络下页面完整渲染，或构建命令里带了 define

**5.4** 依赖许可清单：在 `lib/CONVENTIONS.md` 里列明已许可依赖与理由（`flutter_bloc`、`yaml`、`cupertino_icons`、`flutter_lints`），新增依赖先入清单再引

- 判据：`pubspec.yaml` 的每个依赖都在清单里

**5.5** IaC 归属：确认 studio 的基础设施（OSS 桶 + CDN）由哪里管；若归本仓，补 `manifests/terraform/`，若不归，写一句归属说明

- 判据：有一处写明归属

## 段六 · 文档与工作流对齐

**6.1** 补 doc 欠账：`lib/repositories/`、`lib/states/`、`lib/views/executor_label.dart`、`app.dart`、`main.dart`

- 判据：doc 三轴（screens / views / models）与 `lib/` 一一对得上

**6.2** `doc/models/` 三件（task / workflow / workspace）定位重定：`lib/` 已无 models 层（界面用工具箱对象）——doc 里这三件应改称「契约形状」（`.json` 为准、`.md` 说明），或迁到 toolkit 的文档位

- 判据：doc 里不再暗示 `lib/` 有 models 层

**6.3** 工作流判据对齐：`code-implement-studio.yaml` 交接一步的 `lib/cli` 改成实际位置（或收敛后删掉该条判据）

- 判据：工作流判据与现状一致；`grep -rn "lib/cli" data/profile/quanttide/workflows/` 为空

**6.4** README 与 ROADMAP 里关于「两套实现」的说法随 3.1 更新

## 段七 · 发布线

**7.1** 定：studio 与 cli 是一条发布线，还是各发各的（契约：一条发布线一个包）

**7.2** 开 `studio/v*` 发布（CI 已备：tag → 构建 → OSS → CDN）

- 版本号由创始人拍板
