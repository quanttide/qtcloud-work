import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/repositories/local/tasks.dart';
import 'package:qtcloud_work_studio/repositories/envelope.dart';
import 'package:qtcloud_work_studio/models/task.dart';

TableResult fixture(String name) =>
    TableResult.fromStdout(File('test/fixtures/$name.json').readAsStringSync());

late Directory tmp;

const flow = '''
name: demo
description: 走一遍给我看
steps:
- name: 甲
  description: 做甲
  executor: agent
  criteria:
  - executor: rule
    description: 日志在
    path: data/journal/README.md
- name: 乙
  executor: human
  criteria:
  - executor: human
    description: 人拍板
''';

void main() {
  setUp(() {
    tmp = Directory.systemTemp.createTempSync('task');
    Directory('${tmp.path}/workflows').createSync(recursive: true);
    File('${tmp.path}/workflows/demo.yaml').writeAsStringSync(flow);
  });
  tearDown(() => tmp.deleteSync(recursive: true));

  Task makeTask() =>
      createTask(tmp.path, tmp.path, 'demo', 'demo', '${tmp.path}/workflows');

  group('流水里的步骤名', () {
    test('三种后缀认得出', () {
      expect(JournalKind.of('audit'), JournalKind.run);
      expect(JournalKind.of('profile·审'), JournalKind.agentReview);
      expect(JournalKind.of('site·判'), JournalKind.machineJudge);
    });

    test('去掉后缀就是那一步', () {
      final entry = JournalEntry.fromLine(
        '2026-09-11 20:37\u3000audit·判\u3000站点版本与变更记录两处对齐',
      );
      expect(entry.baseStep, 'audit');
      expect(entry.kind, JournalKind.machineJudge);
      expect(entry.at, '2026-09-11 20:37');
      expect(entry.detail, '站点版本与变更记录两处对齐');
    });

    test('正文里还有分隔符也不丢', () {
      final entry = JournalEntry.fromLine(
        '20:39\u3000conclude\u3000一句话\u3000又一句话',
      );
      expect(entry.detail, '一句话\u3000又一句话');
    });
  });

  group('任务详情', () {
    test('从真实输出读出这次任务', () {
      final task = TaskDetail.fromData(fixture('task_detail').data);
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
      final task = TaskDetail.fromData(fixture('task_detail').data);
      expect(task.products.report, 'artifacts/report/learn-task-create.md');
      expect(task.products.journal, 'artifacts/journal/learn-task-create.md');
      expect(task.products.log, 'tasks/learn-task-create.yaml');
    });

    test('流水取最近五条，按时序', () {
      final task = TaskDetail.fromData(fixture('task_detail').data);
      expect(task.journal.length, 5);
      expect(
        task.journal.first.at.compareTo(task.journal.last.at) <= 0,
        isTrue,
      );
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
      final task = TaskDetail.fromData(result.data);
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

  group('流水怎么算走过', () {
    test('执行 ok 才算', () {
      final task = makeTask();
      task.record('甲', '走了一遍', true);
      expect(task.done(), ['甲']);
    });

    test('执行 ✗ 不算', () {
      final task = makeTask();
      task.record('甲', '没走成', false);
      expect(task.done(), isEmpty);
    });

    test('审查 ✗ 投反对票：执行 ok 也不算', () {
      final task = makeTask();
      task.record('甲', '走了一遍', true);
      task.record('甲·审', '审查没过', false);
      expect(task.done(), isEmpty);
    });

    test('机器判据 ✗ 也投反对票', () {
      final task = makeTask();
      task.record('甲', '走了一遍', true);
      task.record('甲·判', '判据没过', false);
      expect(task.done(), isEmpty);
    });

    test('重走一遍都 ok，就不算走过', () {
      final task = makeTask();
      task.record('甲', '第一次', false);
      task.record('甲', '第二次', true);
      expect(task.done(), ['甲']);
    });

    test('重走 ok 之后再挨一刀，就又不算', () {
      final task = makeTask();
      task.record('甲', '第一次', false);
      task.record('甲', '第二次', true);
      task.record('甲·审', '审查没过', false);
      expect(task.done(), isEmpty);
    });

    test('工作流上没有的步骤名不算数', () {
      final task = makeTask();
      task.record('丙', '奇怪的一步', true);
      expect(task.done(), isEmpty);
    });
  });

  group('下一步与状态行', () {
    test('下一步取第一个没走到的', () {
      final task = makeTask();
      expect(task.nextStep()?.name, '甲');
      task.record('甲', '走了一遍', true);
      expect(task.nextStep()?.name, '乙');
      task.record('乙', '人拍板', true);
      expect(task.nextStep(), isNull);
      expect(stateLine(task), '2 个步骤都走过了');
    });

    test('状态行说下一步是谁', () {
      expect(stateLine(makeTask()), '下一步：甲');
    });
  });

  group('起任务、看任务、列任务', () {
    test('起的任务带着运行上下文', () {
      final task = makeTask();
      final body = task.payload();
      expect(body['name'], 'demo');
      expect(body['workflow'], 'demo');
      expect(body['root'], tmp.path);
      expect(body['log'], isEmpty);
      expect(task.start, isNotEmpty);
    });

    test('产品没声明就落草稿区', () {
      final task = makeTask();
      expect(task.artifact('report'), '${tmp.path}/artifacts/report/demo.md');
      expect(task.artifact('log'), task.file);
    });

    test('状态：行是「步骤 / 状态」，末了还有指令与产物', () {
      final task = makeTask();
      task.record('甲', '走了一遍', true);
      final shown = taskStatus(
        tmp.path,
        tmp.path,
        'demo',
        '${tmp.path}/workflows',
      );
      expect(shown.ok, isTrue);
      expect(shown.columns, ['步骤', '状态']);
      expect(shown.rows, [
        ['甲', '✓'],
        ['乙', '—'],
      ]);
      expect(shown.lines.first, '任务：demo');
      expect(shown.lines, contains('  工作流：demo——走一遍给我看'));
      expect(shown.lines, contains('下一步：乙'));
      expect(shown.lines, contains('指令：tasks/demo.yaml'));
      expect(shown.lines.last.startsWith('  2026'), isTrue);
    });

    test('列表：没走完的说下一步，走完的说走完', () {
      final task = makeTask();
      final listed = taskList(tmp.path, tmp.path, '${tmp.path}/workflows');
      expect(listed.columns, ['任务', '工作流', '下一步']);
      expect(listed.rows, [
        ['demo', 'demo', '甲'],
      ]);
      task.record('甲', '走了一遍', true);
      task.record('乙', '人拍板', true);
      expect(taskList(tmp.path, tmp.path, '${tmp.path}/workflows').rows, [
        ['demo', 'demo', '走完'],
      ]);
    });

    test('同一件任务不覆盖', () {
      makeTask();
      final again = taskNew(
        tmp.path,
        tmp.path,
        'demo',
        'demo',
        '${tmp.path}/workflows',
      );
      expect(again.ok, isFalse);
      expect(again.lines.first, contains('已经有这件任务'));
    });

    test('没有这条工作流就不给起', () {
      final bad = taskNew(
        tmp.path,
        tmp.path,
        'x',
        'nope',
        '${tmp.path}/workflows',
      );
      expect(bad.ok, isFalse);
      expect(bad.lines.first, contains('没有这条工作流'));
    });
  });

  group('日志收叙事', () {
    test('一段一段往下写，并记一笔流水', () {
      final task = makeTask();
      taskJournal(tmp.path, tmp.path, 'demo', '第一段', '${tmp.path}/workflows');
      taskJournal(tmp.path, tmp.path, 'demo', '第二段', '${tmp.path}/workflows');
      final text = File(task.artifact('journal')).readAsStringSync();
      expect(text, contains('第一段'));
      expect(text, contains('第二段'));
      expect(task.events().last['step'], '历史');
      expect(task.events().map((e) => e['detail']), ['第一段', '第二段']);
    });

    test('空话不给记', () {
      makeTask();
      final result = taskJournal(
        tmp.path,
        tmp.path,
        'demo',
        '   ',
        '${tmp.path}/workflows',
      );
      expect(result.ok, isFalse);
      expect(result.lines.first, contains('日志要人来写'));
    });
  });
}
