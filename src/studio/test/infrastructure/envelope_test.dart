import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/infrastructure/envelope.dart';

String fixture(String name) =>
    File('test/fixtures/$name.json').readAsStringSync();

void main() {
  group('统一信封', () {
    test('读得进 ok / columns / rows / lines', () {
      final result = TableResult.fromStdout(fixture('workflow_list'));
      expect(result.ok, isTrue);
      expect(result.columns, ['工作流', '步骤', '位置']);
      expect(result.rows.length, 5);
      expect(result.rows.first.first, 'code-implement');
    });

    test('valueAfter 去掉前缀与两侧空白', () {
      final result = TableResult.fromStdout(fixture('task_detail'));
      expect(result.valueAfter('任务：'), 'learn-task-create');
      expect(result.valueAfter('开工：'), '2026-09-11 20:28');
      expect(result.valueAfter('不存在的：'), isNull);
    });

    test('linesAfter 取标记之后的全部行', () {
      final result = TableResult.fromStdout(fixture('task_detail'));
      final journal = result.linesAfter('流水（最近五条）：');
      expect(journal.length, 5);
      expect(journal.first.startsWith('2026-09-11 20:36'), isTrue);
    });
  });
}
