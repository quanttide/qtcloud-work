import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/repositories/local/local_repository.dart';
import 'package:quanttide_work/quanttide_work.dart' as qt;

/// 本地那套实现：直接调命令面，不编信封。数据落在临时目录里，不碰仓。
void main() {
  late Directory tmp;
  late LocalRepository repository;

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

  setUp(() {
    tmp = Directory.systemTemp.createTempSync('local-repo');
    Directory('${tmp.path}/flows').createSync(recursive: true);
    File('${tmp.path}/flows/demo.yaml').writeAsStringSync(flow);
    repository = LocalRepository(
      qt.RunContext(
        root: tmp.path,
        data: tmp.path,
        workflows: '${tmp.path}/flows',
      ),
    );
  });
  tearDown(() => tmp.deleteSync(recursive: true));

  test('列任务与列工作流读的是同一条命令的结果', () async {
    expect((await repository.workflows()).map((item) => item.name), ['demo']);
    await repository.create('demo', 'demo');
    final tasks = await repository.tasks();
    expect(tasks.map((item) => item.name), ['demo']);
    expect(tasks.single.workflow, 'demo');
  });

  test('起一件任务再读回来', () async {
    await repository.create('demo', 'demo');
    final opened = await repository.task('demo');
    expect(opened.task.name, 'demo');
    expect(opened.task.workflowName, 'demo');
    expect(opened.workflow.steps.map((item) => item.name), ['甲', '乙']);
  });

  test('没有这条工作流就不给起，话说清楚', () async {
    expect(
      () => repository.create('x', 'nope'),
      throwsA(
        isA<LocalFailure>().having(
          (e) => e.message,
          'message',
          contains('没有这条工作流'),
        ),
      ),
    );
  });

  test('定义核对把结果原样交回来（有要改的也不算出错）', () async {
    final check = await repository.check('demo');
    expect(check.ok, isFalse);
    expect(check.lines, isNotEmpty);
  });
}
