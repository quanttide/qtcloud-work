import 'package:qtcloud_work_studio/models/task.dart';
import 'package:qtcloud_work_studio/models/workflow.dart';
import 'package:qtcloud_work_studio/repositories/envelope.dart';
import 'package:qtcloud_work_studio/repositories/studio_repository.dart';

import 'fake_runner.dart';

/// 假的工作台数据边界：读的是命令行真实输出，动作只记一笔。
///
/// 界面测试不碰文件与进程——问什么答什么；四个动作（走一步、记一步、
/// 记日志、起任务）留下的痕迹进 [calls]，供断言。
class FakeRepository implements StudioRepository {
  FakeRepository({this.healthLines = const ['provider：无已部署'], this.finished = true});

  final List<String> healthLines;

  /// 任务详情读回来时，步骤算不算都走过了（默认拿真实输出里那件走完的）。
  final bool finished;

  final List<String> calls = [];

  @override
  Future<List<TaskSummary>> tasks() async =>
      TableResult.fromStdout(fixture('task_list')).rows
          .map(TaskSummary.fromRow)
          .toList(growable: false);

  @override
  Future<TaskDetail> task(String name) async {
    final data = Map<String, Object?>.from(fixtureData('task_detail'));
    if (!finished) {
      data['steps'] = [
        for (final step in (data['steps'] as List).cast<Map>())
          {...step, 'done': false},
      ];
      data['state'] = '下一步：profile';
    }
    return TaskDetail.fromData(data);
  }

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
  Future<List<WorkflowSummary>> workflows() async =>
      TableResult.fromStdout(fixture('workflow_list')).rows
          .map(WorkflowSummary.fromRow)
          .toList(growable: false);

  @override
  Future<WorkflowDetail> workflow(String name) async =>
      WorkflowDetail.fromData(fixtureData('workflow_detail'));

  @override
  Future<DefinitionCheck> check(String name) async =>
      const DefinitionCheck(ok: true, lines: ['定义没问题']);

  @override
  Future<List<String>> health() async => healthLines;
}
