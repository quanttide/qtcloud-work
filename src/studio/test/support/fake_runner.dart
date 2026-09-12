import 'dart:io';

import 'package:quanttide_work/quanttide_work.dart' show Outcome;
import 'package:qtcloud_work_studio/repositories/runner.dart';
import 'package:quanttide_work/quanttide_work.dart' as qt;

/// 测试用的那处工作区——就是工具箱里的运行上下文。
const testWorkspace = qt.RunContext(
  root: '/w',
  data: '/w/data/context/qtcloud-work',
  workflows: '/w/data/profile/quanttide/workflows',
);

/// 命令行的真实输出（`test/fixtures/*.json`）原样读进来。
String fixture(String name) =>
    File('test/fixtures/$name.json').readAsStringSync();

/// 真实输出里给界面那一栏（`data`）——界面从不读面向人的 `lines`。
Map<String, Object?> fixtureData(String name) =>
    Outcome.fromStdout(fixture(name)).data ?? const <String, Object?>{};

/// 真实输出里的表（`rows`）——名单类的命令用这一栏，没有 `data`。
List<List<String>> fixtureRows(String name) =>
    Outcome.fromStdout(fixture(name)).rows;

/// 那一栏里托着的原文：任务文件 / 定义文件的内容，装成领域对象要用。
Map<String, Object?> fixturePayload(String name) => Map<String, Object?>.from(
  (fixtureData(name)['payload'] as Map?) ?? const <String, Object?>{},
);

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
