# TODO：studio 待办

可直接动手的事项，逐条列、做完就勾。

每一条做完的通用判据（缺一不算完）：

```bash
cd apps/qtcloud-work/src/studio
flutter analyze && flutter test
sh scripts/parity.sh
```

## 收回工具箱（Dart）

取消对 pub.dev `quanttide_work` 的依赖，把 toolkit 抽走的 Dart 代码并回本仓，本仓自持。

- [x] **并模型**：`packages/quanttide-work-toolkit/packages/dart/lib/src` 的每个模型按同名并入 `lib/`
  - `artifact/`、`criterion/`、`task/`、`workflow/`、`workspace/` → `lib/` 下同名目录
  - `outcome.dart`、`error.dart`、`executor.dart`、`paths.dart`、`fields.dart` → `lib/` 根下同名文件
  - 注意：`repositories/local/tasks.dart` 有自己的 `Task`（带位置的句柄），toolkit 的 `Task` 是领域模型——按现在的 `as qt` 前缀引法保留，别混进一名字
  - 注意：`repositories/local/paths.dart` 是平台侧的路径显示，与 toolkit 的 `lib/paths.dart`（占位）不同物，import 别串
- [x] **改引用**：`package:quanttide_work/quanttide_work.dart` 全改写本仓 `package:qtcloud_work_studio/quanttide_work.dart`；各处 `show` / `as` 原样保留
- [x] **去依赖**：`pubspec.yaml` 删 `quanttide_work`；`flutter pub get` 跟 `pubspec.lock`
- [x] **判据**：`grep -rn "package:quanttide_work/" lib test` 为空；通用判据全绿
