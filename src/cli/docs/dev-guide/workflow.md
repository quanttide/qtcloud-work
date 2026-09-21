# workflow：工作流

工作流是串联的步骤（定义），用 YAML 存，落在 `<工作流目录>/<名字>.yaml`（缺省跟在账本里，`--workflows` 可另指）。

## 落点

- `workflow/model.rs`——领域模型：`Step` / `Workflow`，闸门（`gates`）由定义推导；
- `workflow/read.rs`——读法与语法校验：`of` / `validate`；
- `workflow/mod.rs`——类型与出入口：`WorkflowFile`（定义连同它的文件位置）、动作出口；
- `workflow/yaml.rs`——YAML 读写与 schema 校验：`load` / `dump` / `WorkflowError`；
- `workflow/check.rs`——定义核对：判据写下的路径在不在区内、描述点到的小节有没有覆盖；
- `workflow/actions.rs`——动作：写（`create`）、看、列、核对（`check`）、导出、导入。

## 规矩

- 模型在 `model`，读法与语法校验在 `read`；文件读写与信封归本侧。
- 定义要有固定意义：字段名与取值由 schema 定死，不认识的字段直接报错。
- 定义是数据不是代码：加一步、减一步、改判据、换执行者，动 YAML 即可，程序一行不改。
- 定义不写死工单名：判据里的占位跑起来才换成本趟行程的真实路径（见 [work-record](work-record.md)）。
- **定义里不抄别处拥有的事实**：会变的分类目录、落点、名字一律指过去，不抄进来。抄了，事实源一变定义就撒谎——执行者照旧表归类，落不进去的只能报「拿不准」，把人的闸门派去裁一个不是判断问题的东西。
- 定义不写凭证：`id` 按「工作区 id + 名字」现算，写下的凭证不是派生的凭证。
- 工作流名与文件名必须一致，由本侧 `load` 核。

## 判据三类

判据挂在步骤上，按谁判分三类：`rule` 由程序按字段判四种判法、`agent` 照说明审、`human` 进待拍板清单。判据的语法与翻译在 `crate::criterion`，本侧只负责真去跑（见 [audit](audit.md)）。

## 测试

定义核对在 `tests/definition_check.rs`——判据里的路径在不在、描述提到的小节有没有覆盖；凭证在 `tests/credentials.rs`。
