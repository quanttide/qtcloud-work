# TODO：cli 待办

可直接动手的细节，逐条列、做完就勾。要人拍板的事见 [ROADMAP.md](ROADMAP.md)；体检数据见 [STATUS.md](STATUS.md)。

做完任一条的通用判据（缺一不算完）：

```bash
cd apps/qtcloud-work/src/cli
cargo fmt --check && cargo clippy --locked && cargo test --locked
sh scripts/validate-usecases.sh
```

## 依赖与门禁

- [ ] **许可清单**：列明 `Cargo.toml` 的每个依赖（`quanttide-work`、`clap`、`serde`、`serde_json`、`serde_yaml`、`ureq`）与理由；新增依赖先入清单。判据：`Cargo.toml` 的每个依赖都在清单里
- [ ] **行数门禁**（可选，见 ROADMAP）：加脚本，`src/**/*.rs` 超 250 行即红。判据：脚本能拦

## 文档与实现同构

- [ ] **重写 `docs/dev-guide/layers.md`**：改成三类模块的落点图，去掉「拟建」；`outcome.rs` 那段按实际（已抽到工具箱）。判据：文档结构与 `src/` 一一对得上
- [ ] **`docs/api-references/` 分两类**：命令参考（`task.md` `workflow.md` `search.md` …）与概念参考（`workspace.md` 讲布局与资产表）分开放或标明。判据：目录里不再混两类
- [ ] **`prompts` 两侧统一口径**：`src/prompts.rs` 与 `studio/lib/repositories/local/prompts.dart` 头注释互相打脸——合一或写明差异（选哪边见 ROADMAP）。判据：两处注释不再互相打脸
- [ ] **用例对账去空转**：`docs/user-guide/` 补「## 用例」标题，测试按用例标出处。判据：`validate-usecases.sh` 打印的两行用例号一致且非空

## 测试补缺

- [ ] **流水落点**：断 `Task::artifact("log")` 等于任务文件本身（studio 逮住过，本侧缺）。判据：测试在、跑绿

## 等上游定规（定了再落）

- [ ] **`name` 与文件名一致**：规矩定在工具箱 `validate` 还是本侧 `load`——见 ROADMAP
- [ ] **「类别 / 名字」两轴重理**：按「产物类别」新定义重理平台——见 ROADMAP
- [ ] **借一个空工作区**：工具箱把 `place` / `expanded` / `check` 收成关联函数后，删掉本侧 `Workspace::default()` 那行
