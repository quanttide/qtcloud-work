import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/models/table_result.dart';
import 'package:qtcloud_work_studio/models/task.dart';

TableResult fixture(String name) =>
    TableResult.fromStdout(File('test/fixtures/$name.json').readAsStringSync());

void main() {
  group('流水里的步骤名', () {
    test('三种后缀认得出', () {
      expect(JournalKind.of('audit'), JournalKind.run);
      expect(JournalKind.of('profile·审'), JournalKind.agentReview);
      expect(JournalKind.of('site·判'), JournalKind.machineJudge);
    });

    test('去掉后缀就是那一步', () {
      final entry = JournalEntry.fromLine('2026-09-11 20:37\u3000audit·判\u3000站点版本与变更记录两处对齐');
      expect(entry.baseStep, 'audit');
      expect(entry.kind, JournalKind.machineJudge);
      expect(entry.at, '2026-09-11 20:37');
      expect(entry.detail, '站点版本与变更记录两处对齐');
    });

    test('正文里还有分隔符也不丢', () {
      final entry = JournalEntry.fromLine('20:39\u3000conclude\u3000一句话\u3000又一句话');
      expect(entry.detail, '一句话\u3000又一句话');
    });
  });

  group('任务详情', () {
    test('从真实输出读出这次任务', () {
      final task = TaskDetail.fromResult(fixture('task_detail'));
      expect(task.name, 'learn-task-create');
      expect(task.workflowName, 'learn-task-create');
      expect(task.workflowDescription.startsWith('把一份学习任务书从个人档案送出去'), isTrue);
      expect(task.steps.length, 5);
      expect(task.steps.first.name, 'profile');
      expect(task.doneCount, 5);
      expect(task.finished, isTrue);
      expect(task.currentStep, isNull);
      expect(task.progress, 1.0);
      expect(task.stateLine, '5 个步骤都走过了');
    });

    test('产物三个落点分得开', () {
      final task = TaskDetail.fromResult(fixture('task_detail'));
      expect(task.products.report, 'artifacts/report/learn-task-create.md');
      expect(task.products.journal, 'artifacts/journal/learn-task-create.md');
      expect(task.products.log, 'tasks/learn-task-create.yaml');
    });

    test('流水取最近五条，按时序', () {
      final task = TaskDetail.fromResult(fixture('task_detail'));
      expect(task.journal.length, 5);
      expect(task.journal.first.at.compareTo(task.journal.last.at) <= 0, isTrue);
    });

    test('没走到的步骤标成 —，当前步骤取第一个没走的', () {
      const result = TableResult(
        ok: true,
        lines: ['任务：demo', '工作流：devops-release——走一次预发布', '下一步：audit'],
        rows: [
          ['version', '✓'],
          ['gate', '✓'],
          ['audit', '—'],
          ['publish', '—'],
        ],
        data: {
          'name': 'demo',
          'start': '2026-09-12 09:00',
          'workflow': 'devops-release',
          'description': '走一次预发布',
          'steps': [
            {'name': 'version', 'done': true},
            {'name': 'gate', 'done': true},
            {'name': 'audit', 'done': false},
            {'name': 'publish', 'done': false},
          ],
          'state': '下一步：audit',
          'products': {'report': 'r', 'journal': 'j', 'log': 'l'},
          'journal': [],
        },
      );
      final task = TaskDetail.fromResult(result);
      expect(task.doneCount, 2);
      expect(task.currentStep, 'audit');
      expect(task.finished, isFalse);
      expect(task.progress, 0.5);
      expect(task.stateLine, '下一步：audit');
    });
  });

  group('任务列表', () {
    test('从真实输出读出名字、工作流与下一步', () {
      final rows = fixture('task_list').rows;
      final tasks = rows.map(TaskSummary.fromRow).toList();
      expect(tasks.length, 5);
      expect(tasks.first.name, 'compare-course-profile');
      expect(tasks.first.workflow, 'compare-course-profile');
      expect(tasks.first.next, '走完');
    });
  });
}
