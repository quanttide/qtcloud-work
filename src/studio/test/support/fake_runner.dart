import 'dart:io';

import 'package:qtcloud_work_studio/models/workspace.dart';
import 'package:qtcloud_work_studio/repositories/envelope.dart';
import 'package:qtcloud_work_studio/repositories/runner.dart';

/// 界面测试用的那处工作区：三处位置都是假的，不碰真实文件。
const testWorkspace = Workspace(
  root: '/w',
  data: '/w/data/context/qtcloud-work',
  workflows: '/w/data/profile/quanttide/workflows',
);

/// 命令行的真实输出（`test/fixtures/*.json`）原样读进来。
String fixture(String name) =>
    File('test/fixtures/$name.json').readAsStringSync();

/// 真实输出里给界面那一栏（`data`）——界面从不读面向人的 `lines`。
Map<String, Object?> fixtureData(String name) =>
    TableResult.fromStdout(fixture(name)).data;

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
