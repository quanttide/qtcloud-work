import 'package:quanttide_work/quanttide_work.dart' as qt;

export 'environment_io.dart' if (dart.library.js_interop) 'environment_web.dart';

import 'environment_io.dart'
    if (dart.library.js_interop) 'environment_web.dart';

/// 三处位置：从环境读，没给就用当前目录下的常见位置。
qt.RunContext workspaceFromEnvironment([Map<String, String>? environment]) {
  final env = environment ?? processEnvironment();
  return qt.RunContext(
    root: env['QTCLOUD_WORK_ROOT'] ?? '.',
    data: env['QTCLOUD_WORK_DATA'] ?? 'data/context/qtcloud-work',
    workflows: env['QTCLOUD_WORK_WORKFLOWS'] ?? 'data/profile/quanttide/workflows',
  );
}
