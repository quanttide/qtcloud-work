import '../../models/task.dart';
import '../../models/workflow.dart';
import '../../models/workspace.dart';
import '../studio_repository.dart';
import 'dispatch.dart';
import 'outcome.dart';

/// 本地那套实现：不起命令面、不编信封，直接把命令派给 `dispatch`，从
/// 结构化那一栏装模型。
///
/// 与命令行客户端（`../client.dart`）算的是同一套结果——同一处工作区、
/// 同一条命令；差只差在中间过不过一次信封。
class LocalFailure implements Exception {
  const LocalFailure(this.lines);

  final List<String> lines;

  String get message => lines.isEmpty ? '没跑成，也没说为什么' : lines.join('\n');

  @override
  String toString() => 'LocalFailure: $message';
}

class LocalRepository implements StudioRepository {
  LocalRepository(this.workspace);

  final Workspace workspace;

  /// 派一条命令，成功才给结果。
  Outcome _run(List<String> arguments) {
    final outcome = dispatch(
      arguments,
      root: workspace.root,
      data: workspace.data,
      workflows: workspace.workflows,
    );
    if (!outcome.ok) throw LocalFailure(outcome.lines);
    return outcome;
  }

  /// 界面那一栏：界面不读给人看的话，只读这个。
  Map<String, Object?> _data(List<String> arguments) =>
      outcomeData[_run(arguments)] ?? const {};

  // ---- 任务（执行侧）----

  @override
  Future<List<TaskSummary>> tasks() async => _run(['task', '--list']).rows
      .map(TaskSummary.fromRow)
      .toList(growable: false);

  @override
  Future<TaskDetail> task(String name) async =>
      TaskDetail.fromData(_data(['task', name]));

  @override
  Future<void> next(String name) async {
    _run(['task', name, '--next']);
  }

  @override
  Future<void> done(String name, String step, {String note = ''}) async {
    _run([
      'task',
      name,
      '--done',
      step,
      if (note.isNotEmpty) ...['--note', note],
    ]);
  }

  @override
  Future<void> journal(String name, String text) async {
    _run(['task', name, '--journal', text]);
  }

  @override
  Future<void> create(String name, String workflow) async {
    _run(['task', '--new', name, '--workflow', workflow]);
  }

  // ---- 工作流（定义侧）----

  @override
  Future<List<WorkflowSummary>> workflows() async => _run(['workflow', '--list'])
      .rows
      .map(WorkflowSummary.fromRow)
      .toList(growable: false);

  @override
  Future<WorkflowDetail> workflow(String name) async =>
      WorkflowDetail.fromData(_data(['workflow', name]));

  @override
  Future<DefinitionCheck> check(String name) async {
    final outcome = dispatch(
      ['workflow', name, '--check'],
      root: workspace.root,
      data: workspace.data,
      workflows: workspace.workflows,
    );
    return DefinitionCheck(ok: outcome.ok, lines: outcome.lines);
  }

  // ---- 其他 ----

  @override
  Future<List<String>> health() async => _run(['health']).lines;
}
