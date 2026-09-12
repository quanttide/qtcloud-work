import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/models/task.dart';
import 'package:qtcloud_work_studio/models/workflow.dart';
import 'package:qtcloud_work_studio/states/workbench_bloc.dart';

import '../support/fake_repository.dart';

/// 工作台状态机：一页一页地拿数据，动作之后把列表与当前那条重新拉齐。
///
/// 数据来源是 [FakeRepository]（读命令行真实输出）；这一层只验状态怎么走，
/// 不碰文件与进程。
void main() {
  late FakeRepository repository;
  late WorkbenchBloc bloc;

  setUp(() {
    repository = FakeRepository();
    bloc = WorkbenchBloc(repository);
  });
  tearDown(() => bloc.close());

  /// 先订阅再派事件，免得漏掉中间那几帧。
  Future<WorkbenchState> until(bool Function(WorkbenchState) ready) {
    final states = bloc.stream;
    return states.firstWhere(ready);
  }

  test('拉一遍：任务与工作流各打开第一条', () async {
    final ready = until((s) => !s.busy && s.tasks.isNotEmpty);
    bloc.add(const WorkbenchLoad());
    final state = await ready;
    expect(state.task?.name, 'learn-task-create');
    expect(state.taskWorkflow?.name, 'learn-task-create');
    expect(state.workflow?.name, 'learn-task-create');
    expect(state.page, 'task');
  });

  test('换一屏只动 page', () async {
    final ready = until((s) => s.page == 'flow');
    bloc.add(const WorkbenchShowPage('flow'));
    expect((await ready).busy, isFalse);
  });

  test('走下一步：动作交给仓储，回来弹一句', () async {
    final loaded = until((s) => s.task != null);
    bloc.add(const WorkbenchLoad());
    await loaded;
    final done = until((s) => !s.busy && s.note != null);
    bloc.add(const WorkbenchRunNext());
    final state = await done;
    expect(repository.calls, contains('next learn-task-create'));
    expect(state.note, isNotEmpty);
  });

  test('记一步用当前那一步', () async {
    // 用一件还没走完的任务：走完的那件没有「当前步骤」。
    final unfinishedRepository = FakeRepository(finished: false);
    final unfinished = WorkbenchBloc(unfinishedRepository);
    addTearDown(unfinished.close);
    final loaded = unfinished.stream.firstWhere((s) => s.task != null);
    unfinished.add(const WorkbenchLoad());
    final task = (await loaded).task!;
    final done = unfinished.stream.firstWhere((s) => !s.busy && s.note != null);
    unfinished.add(const WorkbenchRecordDone());
    await done;
    expect(
      unfinishedRepository.calls,
      contains('done learn-task-create ${task.currentStep}'),
    );
  });

  test('起一件任务：换到任务页、打开刚起的、弹一句', () async {
    final loaded = until((s) => s.workflow != null);
    bloc.add(const WorkbenchLoad());
    await loaded;
    final created = until((s) => s.note != null);
    bloc.add(const WorkbenchCreateTask('新的'));
    final state = await created;
    expect(repository.calls, contains('create 新的 learn-task-create'));
    expect(state.page, 'task');
    expect(state.note, '起了任务：新的');
  });

  test('失败留在 error 里，不静默', () async {
    final failing = _FailingRepository();
    final failingBloc = WorkbenchBloc(failing);
    addTearDown(failingBloc.close);
    final ready = failingBloc.stream.firstWhere((s) => s.error != null);
    failingBloc.add(const WorkbenchLoad());
    expect((await ready).error, contains('拉不动'));
  });
}

class _FailingRepository extends FakeRepository {
  @override
  Future<List<TaskSummary>> tasks() async => throw const LocalFailureProbe();

  @override
  Future<List<WorkflowSummary>> workflows() async =>
      throw const LocalFailureProbe();
}

class LocalFailureProbe implements Exception {
  const LocalFailureProbe();

  @override
  String toString() => '拉不动';
}
