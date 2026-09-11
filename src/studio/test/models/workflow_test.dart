import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/models/executor.dart';
import 'package:qtcloud_work_studio/models/table_result.dart';
import 'package:qtcloud_work_studio/models/workflow.dart';

TableResult fixture(String name) =>
    TableResult.fromStdout(File('test/fixtures/$name.json').readAsStringSync());

void main() {
  group('判据计数', () {
    test('读命令行写的那一行', () {
      final counts = CriteriaCounts.parse('3 rule / 1 agent / 0 human');
      expect(counts.rule, 3);
      expect(counts.agent, 1);
      expect(counts.human, 0);
      expect(counts.total, 4);
      expect(counts.of(Executor.rule), 3);
    });

    test('缺项当零', () {
      expect(CriteriaCounts.parse('2 agent').total, 2);
      expect(CriteriaCounts.parse('').total, 0);
    });
  });

  group('工作流详情', () {
    test('从真实输出读出名字、位置、步骤与判据', () {
      final workflow = WorkflowDetail.fromResult(fixture('workflow_detail'));
      expect(workflow.name, 'learn-task-create');
      expect(workflow.path.endsWith('learn-task-create.yaml'), isTrue);
      expect(workflow.steps.length, 5);
      expect(workflow.steps.first.name, 'profile');
      expect(workflow.steps.first.executor, Executor.agent);
      expect(workflow.steps.first.criteria.total, 4);
      expect(workflow.criteria.total, 4 + 5 + 4 + 3 + 2);
      expect(workflow.summary, '5 步 · 18 条判据');
    });

    test('人执行的步骤认得出来', () {
      final workflow = WorkflowDetail.fromResult(fixture('workflow_detail'));
      final audit = workflow.steps.firstWhere((step) => step.name == 'audit');
      expect(audit.criteria.human, 1);
      expect(audit.criteria.of(Executor.human), 1);
    });
  });
}
