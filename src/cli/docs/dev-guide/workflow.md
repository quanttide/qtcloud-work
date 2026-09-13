# workflow：工作流

工作流是串联的步骤（定义），用 YAML 存，落在 `<工作流目录>/<名字>.yaml`（默认 `<数据仓>/workflows/`）。

## 落点

- `workflow/model.rs`——领域模型：`Step` / `Workflow`；
- `workflow/read.rs`——读法与语法校验：`of` / `validate`；
- `workflow/mod.rs`——类型与出入口：`WorkflowFile`（定义连同它的文件位置）、动作出口；
- `workflow/yaml.rs`——YAML 读写与 schema 校验：`load` / `dump` / `WorkflowError`；
- `workflow/check.rs`——定义核对：判据里的路径在不在、描述提到的小节有没有覆盖；
- `workflow/actions.rs`——动作：写（`--new`）、看、核对（`--check`）、导出、导入、列。

## 规矩

- 模型在 `model`，读法与语法校验在 `read`，定义核对在 `check`；本侧只管文件读写与信封。
- 定义要有固定意义：字段名与取值由 schema 定死，不认识的字段直接报错。
- 定义是数据不是代码：加一步、减一步、改判据、换执行者，动 YAML 即可，程序一行不改。
- 定义不写死任务名：判据里的占位跑起来才换成本次任务的真实路径（见 [task](task.md)）。
- 工作流名取自定义里的 `name` 字段，不是文件名；两者不一致时会静默取不到（见下「决定」）。

## 判据三类

判据挂在步骤上，按谁判分三类：`rule` 由程序按字段判四种判法、`agent` 照说明审、`human` 进闸门。判据的语法与翻译在 `crate::criterion`，本侧只负责真去跑（见 [audit](audit.md)）。

## 测试

定义核对在 `tests/definition_check.rs`——判据里的路径在不在、描述提到的小节有没有覆盖。

## 决定

工作流名与文件名必须一致，由本侧 `load` 核（文件名只有装载这一侧知道）。动手见 [TODO](../../TODO.md)。
