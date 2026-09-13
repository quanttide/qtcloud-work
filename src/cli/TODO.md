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

## 命名

- [ ] **`src/artifact/` 改名 `asset/`**：这一层按实是资产表，`artifact` 留给规格里的产物；目录、引用、`src/CONVENTIONS.md` 与 dev-guide 的 `artifact.md` 跟着改。判据：`src/` 下不再有 `artifact/`，`catalog` / `audit` 仍绿

## 等工具箱

- [ ] **工作流名校验**：工具箱 `validate` 收文件名，核 `name` 与文件名一致，本侧不再静默取不到。判据：定义里两者不一致时当场报错
- [ ] **借一个空工作区**：工具箱把 `place` / `expanded` / `check` 收成关联函数后，删掉本侧 `Workspace::default()` 那行。判据：`src/task/mod.rs` 里不再有 `Workspace::default()`

## 发布线

- [ ] **两条线写明**：在 `apps/qtcloud-work/README.md` 的发布一节写明 cli 与 studio 各自一条线（tag、发布物与目标注册表）。判据：两条线的 tag 与注册表都写清
