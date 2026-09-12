/// 工作流模块的测试设计。
///
/// 被测对象分两层，数据来源各不相同：
///
/// - **定义层**：`<数据仓>/workflows/<名字>.yaml`，契约由规格
///   `docs/specification/process/workflow.md`·语法定义。正向用例覆盖合法定义的读入与展示、
///   四个动作（新建、导出、导入、核对）的结果；反向用例覆盖非法输入的报错：
///   字段缺失、字段冗余、取值越界、rule 判据缺少判法。
/// - **模型层**：`WorkflowDetail` / `CriteriaCounts`（`lib/models/`，界面背后的数据），
///   输入为信封的 `data` 字段，不读取面向人的 `lines`。期望值取自
///   `test/fixtures/workflow_detail.json`（命令行的真实输出），不使用手工构造的样例。
///
/// 规格或真实输出与实现不一致时，先修订文档，再更新测试。
///
/// `good` 为定义层共用的输入夹具：一份最小且 rule / agent / human 三类判据齐备的定义。
library;

import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/core/definition.dart';
import 'package:qtcloud_work_studio/models/executor.dart';
import 'package:qtcloud_work_studio/models/table_result.dart';
import 'package:qtcloud_work_studio/models/workflow.dart';

TableResult fixture(String name) =>
    TableResult.fromStdout(File('test/fixtures/$name.json').readAsStringSync());

late Directory tmp;

/// 工作流定义默认落在 `<数据仓>/workflows/` 下。
String write(String name, String text) {
  final dir = Directory('${tmp.path}/workflows')..createSync(recursive: true);
  final path = '${dir.path}/$name';
  File(path).writeAsStringSync(text);
  return path;
}

const good = '''
name: demo
description: 走一遍给我看
steps:
- name: 甲
  description: 做甲，「收尾」一节写清
  executor: agent
  criteria:
  - executor: rule
    description: 日志在
    path: data/journal/README.md
  - executor: agent
    description: 甲做干净了
- name: 乙
  executor: human
  criteria:
  - executor: human
    description: 人拍板
''';

void main() {
  setUp(() => tmp = Directory.systemTemp.createTempSync('definition'));
  tearDown(() => tmp.deleteSync(recursive: true));

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

  group('工作流的形状', () {
    // 规范：docs/specification/process/workflow.md·语法
    // 「顶层三个字段：name、description 与 steps」
    test('顶层字段就这三个', () {
      expect(topFields, ['name', 'description', 'steps']);
    });
  });

  group('读一份定义', () {
    test('合法的读得进', () {
      final payload = loadDefinition(write('demo.yaml', good)) as Map;
      expect(payload['name'], 'demo');
      expect((payload['steps'] as List).length, 2);
    });

    test('少了 name 报错', () {
      expect(
        () => loadDefinition(write('a.yaml', 'steps:\n- name: 甲\n')),
        throwsA(
          isA<DefinitionError>().having(
            (e) => e.message,
            'message',
            contains('少了 name'),
          ),
        ),
      );
    });

    test('少了 steps 报错', () {
      expect(
        () => loadDefinition(write('b.yaml', 'name: demo\n')),
        throwsA(predicate((e) => '$e'.contains('少了 steps'))),
      );
    });

    test('顶层有不认识的字段报错', () {
      expect(
        () => loadDefinition(
          write('c.yaml', 'name: demo\nversion: 2\nsteps:\n- name: 甲\n'),
        ),
        throwsA(predicate((e) => '$e'.contains('顶层有不认识的字段：version'))),
      );
    });

    test('步骤的 executor 只能是 agent 或 human', () {
      expect(
        () => loadDefinition(
          write('d.yaml', 'name: demo\nsteps:\n- name: 甲\n  executor: robot\n'),
        ),
        throwsA(
          predicate(
            (e) => '$e'.contains('executor 只能是 agent 或 human，实得 robot'),
          ),
        ),
      );
    });

    test('判据的 executor 只能是 rule / agent / human', () {
      expect(
        () => loadDefinition(
          write(
            'e.yaml',
            'name: demo\nsteps:\n- name: 甲\n  criteria:\n  - executor: robot\n    run: "true"\n',
          ),
        ),
        throwsA(predicate((e) => '$e'.contains('只能是 rule / agent / human'))),
      );
    });

    test('rule 得写一条判法', () {
      expect(
        () => loadDefinition(
          write(
            'f.yaml',
            'name: demo\nsteps:\n- name: 甲\n  criteria:\n  - executor: rule\n',
          ),
        ),
        throwsA(predicate((e) => '$e'.contains('得写一条判法'))),
      );
    });

    test('contains 与 file 必须成对，判法只能一种', () {
      expect(
        () => loadDefinition(
          write(
            'g.yaml',
            'name: demo\nsteps:\n- name: 甲\n  criteria:\n  - executor: rule\n    file: a.md\n    contains: 一句话\n    path: x\n',
          ),
        ),
        throwsA(predicate((e) => '$e'.contains('判法只能一种'))),
      );
      expect(
        () => loadDefinition(
          write(
            'h.yaml',
            'name: demo\nsteps:\n- name: 甲\n  criteria:\n  - executor: rule\n    contains: 一句话\n',
          ),
        ),
        throwsA(predicate((e) => '$e'.contains('还得写 file'))),
      );
      expect(
        () => loadDefinition(
          write(
            'i.yaml',
            'name: demo\nsteps:\n- name: 甲\n  criteria:\n  - executor: rule\n    file: a.md\n',
          ),
        ),
        throwsA(predicate((e) => '$e'.contains('还得写 contains'))),
      );
    });

    test('agent 与 human 必须写 description，且不带 rule 的字段', () {
      expect(
        () => loadDefinition(
          write(
            'j.yaml',
            'name: demo\nsteps:\n- name: 甲\n  criteria:\n  - executor: agent\n    run: x\n',
          ),
        ),
        throwsA(predicate((e) => '$e'.contains('必须写 description'))),
      );
      expect(
        () => loadDefinition(
          write(
            'k.yaml',
            'name: demo\nsteps:\n- name: 甲\n  criteria:\n  - executor: agent\n    description: 判准\n    run: x\n',
          ),
        ),
        throwsA(predicate((e) => '$e'.contains('不该带 run'))),
      );
    });
  });

  group('一条定义怎么读', () {
    test('步骤、执行者、判据分得开', () {
      final flow = openWorkflow(tmp.path, 'demo')
        ..payload = loadDefinition(write('demo.yaml', good)) as Map;
      expect(flow.steps.length, 2);
      expect(flow.steps[0].executor, 'agent');
      expect(flow.steps[1].isHuman, isTrue);
      expect(flow.steps[0].rules.length, 1);
      expect(flow.steps[0].agents.length, 1);
      expect(flow.steps[1].gates.length, 1);
      expect(flow.description, '走一遍给我看');
    });

    test('展示：行是「步骤 / 谁执行 / 怎么算完」', () {
      write('demo.yaml', good);
      final shown = workflowShow(tmp.path, 'demo');
      expect(shown.ok, isTrue);
      expect(shown.columns, ['步骤', '谁执行', '怎么算完']);
      expect(shown.rows[0], ['甲', 'agent', '1 rule / 1 agent / 0 human']);
      expect(shown.rows[1], ['乙', 'human', '0 rule / 0 agent / 1 human']);
      expect(workflowList(tmp.path).rows.length, 1);
    });

    test('没有这条工作流就说没有', () {
      final shown = workflowShow(tmp.path, 'nope');
      expect(shown.ok, isFalse);
      expect(shown.lines.first, contains('没有这条工作流'));
    });
  });

  group('新建、导出、导入', () {
    test('新建写的骨架能被读回', () {
      final made = workflowNew(tmp.path, 'demo2', ['甲', '乙'], '说明');
      expect(made.ok, isTrue);
      final flow = openWorkflow(tmp.path, 'demo2');
      expect(flow.steps.map((s) => s.name), ['甲', '乙']);
      expect(flow.steps.first.rules.length, 1);
      expect(flow.steps.first.gates.length, 1);
    });

    test('没名字、没步骤都不给建', () {
      expect(workflowNew(tmp.path, ' ', ['甲'], '').ok, isFalse);
      expect(workflowNew(tmp.path, 'demo3', [], '').ok, isFalse);
    });

    test('导出原样带走，导入换个名字落进来', () {
      write('demo.yaml', good);
      final target = '${tmp.path}/backup.yaml';
      expect(workflowExport(tmp.path, 'demo', target).ok, isTrue);
      expect(File(target).existsSync(), isTrue);

      final imported = workflowImport(tmp.path, target, 'other');
      expect(imported.ok, isTrue);
      expect(openWorkflow(tmp.path, 'other').steps.length, 2);

      final again = workflowImport(tmp.path, target, 'other');
      expect(again.ok, isFalse);
      expect(again.lines.first, contains('已经有一条工作流叫「other」'));
    });

    test('导入时不写新名字就用定义里的名字；同名已在那条就挡住', () {
      write('demo.yaml', good);
      final target = '${tmp.path}/copy.yaml';
      workflowExport(tmp.path, 'demo', target);

      final sameName = workflowImport(tmp.path, target, '');
      expect(sameName.ok, isFalse);
      expect(sameName.lines.first, contains('已经有一条工作流叫「demo」'));

      final other = Directory.systemTemp.createTempSync('definition2');
      addTearDown(() => other.deleteSync(recursive: true));
      final imported = workflowImport(other.path, target, '');
      expect(imported.ok, isTrue);
      expect(openWorkflow(other.path, 'demo').name, 'demo');
    });
  });

  group('核对定义', () {
    test('判据里的路径在不在', () {
      Directory('${tmp.path}/data/journal').createSync(recursive: true);
      File('${tmp.path}/data/journal/README.md').writeAsStringSync('# 日志\n');
      write('demo.yaml', good);
      final checked = workflowCheck(tmp.path, 'demo', tmp.path);
      expect(
        checked.lines.any(
          (line) => line.contains('✓ 甲·data/journal/README.md'),
        ),
        isTrue,
      );
    });

    test('路径不在就红', () {
      write('demo.yaml', good);
      final checked = workflowCheck(tmp.path, 'demo', tmp.path);
      expect(checked.ok, isFalse);
      expect(checked.lines.any((line) => line.contains('✗')), isTrue);
    });

    test('描述里提到的报告小节没人覆盖就红', () {
      Directory('${tmp.path}/data/journal').createSync(recursive: true);
      File('${tmp.path}/data/journal/README.md').writeAsStringSync('# 日志\n');
      write('demo.yaml', good);
      final checked = workflowCheck(tmp.path, 'demo', tmp.path);
      expect(
        checked.lines.any(
          (line) => line.contains('description 提到的报告小节有没有判据覆盖：收尾'),
        ),
        isTrue,
      );
      expect(checked.ok, isFalse);
    });

    test('覆盖上了就绿', () {
      Directory('${tmp.path}/data/journal').createSync(recursive: true);
      File('${tmp.path}/data/journal/README.md').writeAsStringSync('# 日志\n');
      write(
        'demo.yaml',
        good.replaceFirst(
          'path: data/journal/README.md',
          'path: data/journal/README.md\n  - executor: rule\n    file: data/journal/README.md\n    contains: "## 收尾"',
        ),
      );
      final checked = workflowCheck(tmp.path, 'demo', tmp.path);
      expect(
        checked.lines.any((line) => line.contains('✓ description')),
        isTrue,
      );
      expect(checked.ok, isTrue);
    });

    test('版本号写法与路径不算小节名', () {
      expect(looksLikeSection('收尾'), isTrue);
      expect(looksLikeSection('[X.Y.Z-pre.1]'), isFalse);
      expect(looksLikeSection('{{report}}'), isFalse);
      expect(looksLikeSection('data/report/x.md'), isFalse);
    });

    test('占位按数据仓展开', () {
      expect(
        expandPlaceholders('{{report}}/x.md', '/d'),
        '/d/artifacts/report/x.md',
      );
      expect(expandPlaceholders('{{log}}', '/d'), '/d/tasks');
    });
  });
}
