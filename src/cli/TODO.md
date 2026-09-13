# TODO：cli 待办

可直接动手的细节，逐条列、做完就勾。要往哪走见 [ROADMAP.md](ROADMAP.md)，跨仓库的待决事项见本文末；体检数据见 [STATUS.md](STATUS.md)。

做完任一条的通用判据（缺一不算完）：

```bash
cd apps/qtcloud-work/src/cli
cargo fmt --check && cargo clippy --all-targets --locked -- -D warnings && cargo test --locked
sh scripts/validate-usecases.sh
sh scripts/validate-line-count.sh
```

## 依赖与门禁

- [x] **许可清单**：列明 `Cargo.toml` 的每个依赖与理由。见 `docs/dev-guide/dependencies.md`
- [x] **行数门禁**：`scripts/validate-line-count.sh`，`src/**/*.rs` 超 250 行即红；已进 CI
- [ ] **跟工具箱新号**：`quanttide-work` 源许可已改 Apache-2.0，等下一号发布后跟号，并把 `dependencies.md` 里 beta.6 的 CC-BY-4.0 改成 Apache-2.0

## 文档与实现同构

- [x] **重写 `docs/dev-guide/layers.md`**：三类模块的落点图，对齐 `src/`
- [x] **`docs/api-references/` 分两类**：`commands/` 与 `concepts/`
- [ ] **`prompts` 两侧统一口径**：`src/prompts.rs` 与 `studio/lib/repositories/local/prompts.dart` 头注释互相打脸——合一或写明差异（选哪边见「待决事项」）。判据：两处注释不再互相打脸
- [x] **用例对账去空转**：`docs/user-guide/usecases.md` 恢复，两行用例号非空且一致

## 测试补缺

- [x] **流水落点**：`tests/task_start.rs` 断 `Task::artifact("log")` 等于任务文件本身

## 待决事项

跨仓库的决策都在这里，定了再落。

- [ ] **`name` 与文件名一致**：定义里两者不一致时静默取不到——规矩定在工具箱 `validate` 还是本侧 `load`
- [ ] **「类别 / 名字」两轴重理**：规格新立「产物类别」（`report` / `journal` 是类别，一件产物的名字另算）——`Artifact` 纳不纳入类别、落点按类别还是按名字，定了再重理平台
- [ ] **cli 用词**：规格叫「产物类别」，cli 的文档与注释仍写「资产表 / 资产」——跟不跟
- [ ] **`prompts`**：cli 与 studio 两份，合一还是各留一份并写明差异
- [ ] **发布线**：studio 与 cli 一条线还是两条（cli 那条已开）
- [ ] **IaC 归属**：cli 的发布物（crates.io + 三平台二进制）与基础设施归本仓还是平台仓
- [ ] **依赖许可**：传递依赖（`Cargo.lock` 那一层）的许可要不要列
- [ ] **借一个空工作区**：工具箱把 `place` / `expanded` / `check` 收成关联函数后，删掉本侧 `Workspace::default()` 那行
