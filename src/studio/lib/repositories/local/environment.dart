export 'environment_io.dart' if (dart.library.js_interop) 'environment_web.dart';

import 'environment_io.dart'
    if (dart.library.js_interop) 'environment_web.dart';
import 'run_context.dart';

/// 三处位置：从环境读，没给就用当前目录下的常见位置。
RunContext workspaceFromEnvironment([Map<String, String>? environment]) {
  final env = environment ?? processEnvironment();
  return RunContext(
    root: env['QTCLOUD_WORK_ROOT'] ?? '.',
    data: env['QTCLOUD_WORK_DATA'] ?? 'data/context/qtcloud-work',
    workflows:
        env['QTCLOUD_WORK_WORKFLOWS'] ?? 'data/profile/quanttide/workflows',
  );
}
