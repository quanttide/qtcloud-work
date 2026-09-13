# TODO：cli 待办

可直接动手的事项，逐条列、做完就勾。体检数据见 [STATUS.md](STATUS.md)。

做完任一条的通用判据（缺一不算完）：

```bash
cd apps/qtcloud-work/src/cli
cargo fmt --check && cargo clippy --all-targets --locked -- -D warnings && cargo test --locked
sh scripts/validate-usecases.sh
sh scripts/validate-line-count.sh
```

## 产物落点

- [ ] **按名字算**：`Task::artifact(category)` 改成按产物名字算（规格 `piece/artifact.md`：产物 = 名字 + 规格，类别不参与落点）；任务的 `artifacts` 声明与 `--json` 键跟进。判据：类别不再当名字传给 `Artifact::named`

## 校验

- [ ] **工作流名校验**：`workflow/yaml.rs::load` 里核 `name` 与文件名一致（文件名只有装载这一侧知道，工具箱拿不到）。判据：定义里两者不一致时当场报错

## 空工作区

- [ ] **换成真工作区**：`src/task/mod.rs` 用 `Task::workspace()`、`src/workflow/check.rs` 用 `Workspace::of` 替掉 `Workspace::default()`（这三个都不读 `self`，beta.6 够）。判据：`src/` 里不再有 `Workspace::default()`

## 发布线

- [ ] **两条线写明**：在 `apps/qtcloud-work/README.md` 的发布一节写明 cli 与 studio 各自一条线（tag、发布物与目标注册表）。判据：两条线的 tag 与注册表都写清
