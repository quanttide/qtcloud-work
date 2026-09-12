import 'dart:io';

import 'package:quanttide_work/quanttide_work.dart' show Outcome;
import 'package:qtcloud_work_studio/repositories/local/run_context.dart';

/// 测试用的那处工作区——就是工具箱里的运行上下文。
const testWorkspace = RunContext(
  root: '/w',
  data: '/w/data/context/qtcloud-work',
  workflows: '/w/data/profile/quanttide/workflows',
);

/// 命令行的真实输出（`test/fixtures/*.json`）原样读进来。
String fixture(String name) =>
    File('test/fixtures/$name.json').readAsStringSync();

/// 真实输出里给界面那一栏（`data`）——界面从不读面向人的 `lines`。
Map<String, Object?> fixtureData(String name) {
  final data = Outcome.fromStdout(fixture(name)).data;
  return data is Map ? Map<String, Object?>.from(data) : const <String, Object?>{};
}

/// 真实输出里的表（`rows`）——名单类的命令用这一栏，没有 `data`。
List<List<String>> fixtureRows(String name) =>
    Outcome.fromStdout(fixture(name)).rows;

/// 那一栏里托着的原文：任务文件 / 定义文件的内容，装成领域对象要用。
Map<String, Object?> fixturePayload(String name) => Map<String, Object?>.from(
  (fixtureData(name)['payload'] as Map?) ?? const <String, Object?>{},
);
