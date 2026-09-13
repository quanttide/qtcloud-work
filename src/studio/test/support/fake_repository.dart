import 'package:qtcloud_work_studio/repositories/studio_repository.dart';
import 'package:qtcloud_work_studio/quanttide_work.dart' as qt;

import 'fixtures.dart';

/// 假的工作台数据边界：读的是命令行真实输出（`test/fixtures/`），动作只记一笔。
///
/// 交出来的跟真实现一样是工具箱的领域对象——界面测试不碰文件与进程。
class FakeRepository implements StudioRepository {
  FakeRepository({
    this.healthLines = const ['provider：无已部署'],
    this.finished = true,
  });

  final List<String> healthLines;

  /// 任务算不算走完了（默认拿真实输出里那件走完的）；没走完就把流水清空。
  final bool finished;

  final List<String> calls = [];

  qt.Task get _task {
    final payload = Map<String, Object?>.from(fixturePayload('task_detail'));
    if (!finished) payload['log'] = <Object?>[];
    return qt.Task.of(payload);
  }

  qt.Workflow get _workflow =>
      qt.Workflow.of(fixturePayload('workflow_detail'));

  @override
  Future<List<({String name, String workflow, String next})>> tasks() async {
    final rows = fixtureRows('task_list');
    return [
      for (final row in rows)
        (
          name: row.isNotEmpty ? row[0] : '',
          workflow: row.length > 1 ? row[1] : '',
          next: row.length > 2 ? row[2] : '',
        ),
    ];
  }

  @override
  Future<({qt.Task task, qt.Workflow workflow, Map<String, String> artifacts})>
  task(String name) async => (
    task: _task,
    workflow: _workflow,
    artifacts: Map<String, String>.from(
      (fixtureData('task_detail')['artifacts'] as Map?) ?? const {},
    ),
  );

  @override
  Future<void> next(String name) async => calls.add('next $name');

  @override
  Future<void> done(String name, String step, {String note = ''}) async =>
      calls.add('done $name $step');

  @override
  Future<void> journal(String name, String text) async =>
      calls.add('journal $name $text');

  @override
  Future<void> create(String name, String workflow) async =>
      calls.add('create $name $workflow');

  @override
  Future<List<({String name, String steps, String path})>> workflows() async {
    final rows = fixtureRows('workflow_list');
    return [
      for (final row in rows)
        (
          name: row.isNotEmpty ? row[0] : '',
          steps: row.length > 1 ? row[1] : '',
          path: row.length > 2 ? row[2] : '',
        ),
    ];
  }

  @override
  Future<({qt.Workflow workflow, String path, String yaml})> workflow(
    String name,
  ) async {
    final data = fixtureData('workflow_detail');
    return (
      workflow: _workflow,
      path: '${data['path'] ?? ''}',
      yaml: '${data['yaml'] ?? ''}',
    );
  }

  @override
  Future<({bool ok, List<String> lines})> check(String name) async =>
      (ok: true, lines: const ['定义没问题']);

  @override
  Future<List<String>> health() async => healthLines;
}
