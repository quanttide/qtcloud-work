import 'dart:io';

import 'package:qtcloud_work_studio/cli/qtcloud_work.dart';
import 'package:qtcloud_work_studio/cli/runner.dart';
import 'package:qtcloud_work_studio/models/workspace.dart';

const testWorkspace = Workspace(
  root: '/w',
  data: '/w/data/context/qtcloud-work',
  workflows: '/w/data/profile/quanttide/workflows',
);

String fixture(String name) =>
    File('test/fixtures/$name.json').readAsStringSync();

/// 假的 runner：记下每次调用的参数，按关键字回话。
class FakeRunner implements Runner {
  FakeRunner(this.replies);

  /// 键是参数里能对上的那一截（如 `task --list`），值是 stdout。
  final Map<String, String> replies;
  final List<List<String>> calls = [];

  @override
  Future<RunOutput> run(String executable, List<String> arguments) async {
    calls.add([executable, ...arguments]);
    final joined = arguments.join(' ');
    for (final entry in replies.entries) {
      if (joined.contains(entry.key)) {
        return RunOutput(exitCode: 0, stdout: entry.value);
      }
    }
    return const RunOutput(
      exitCode: 1,
      stdout: '{"ok": false, "lines": ["假 runner 没有这条回复"]}',
    );
  }
}

/// 默认都回真实夹具的客户端。顺序有用：列表那条要先匹配上。
QtcloudWork fakeClient({Map<String, String> extra = const {}}) {
  return QtcloudWork(
    workspace: testWorkspace,
    runner: FakeRunner({
      'task --list': fixture('task_list'),
      'task ': fixture('task_detail'),
      'workflow --list': fixture('workflow_list'),
      'workflow ': fixture('workflow_detail'),
      'health': '{"ok":true,"lines":["provider：无已部署"]}',
      ...extra,
    }),
  );
}
