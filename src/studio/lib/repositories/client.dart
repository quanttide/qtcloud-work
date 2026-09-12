import 'envelope.dart';
import '../models/task.dart';
import '../models/workflow.dart';
import '../models/workspace.dart';
import 'runner.dart';

/// 命令行说不行（`ok` 为假）。把它印的话带出来。
class CliFailure implements Exception {
  const CliFailure(this.result);

  final TableResult result;

  String get message =>
      result.lines.isEmpty ? '命令行没有成功，也没说为什么' : result.lines.join('\n');

  @override
  String toString() => 'CliFailure: $message';
}

/// 命令行的窗口化客户端。
///
/// 只做一件事：把命令跑起来，把统一信封（`ok` / `columns` / `rows` / `lines`）读成模型。
/// 界面不自己去读工作流与任务文件的原文——那是命令行的活。
class QtcloudWork {
  QtcloudWork({
    required this.workspace,
    Runner? runner,
    this.binary = 'qtcloud-work',
  }) : runner = runner ?? CoreRunner(workspace);

  final Workspace workspace;
  final Runner runner;
  final String binary;

  /// 跑一条命令，拿统一信封。[arguments] 不含三处位置与 `--json`，这里统一带上。
  Future<TableResult> call(List<String> arguments) async {
    final output = await runner.run(binary, [
      ...workspace.globalArgs,
      '--json',
      ...arguments,
    ]);
    final result = TableResult.fromStdout(output.stdout);
    if (!result.ok) throw CliFailure(result);
    return result;
  }

  // ---- 任务（执行侧）----

  Future<TableResult> taskList() => call(['task', '--list']);

  Future<List<TaskSummary>> tasks() async =>
      (await taskList()).rows.map(TaskSummary.fromRow).toList(growable: false);

  Future<TaskDetail> task(String name) async =>
      TaskDetail.fromData((await call(['task', name])).data);

  /// 走下一步。这一步多半交给 AI 跑，会慢。
  Future<TableResult> next(String name) => call(['task', name, '--next']);

  /// 人为地记一步。
  Future<TableResult> done(String name, String step, {String note = ''}) =>
      call([
        'task',
        name,
        '--done',
        step,
        if (note.isNotEmpty) ...['--note', note],
      ]);

  /// 日志收叙事。
  Future<TableResult> journal(String name, String text) =>
      call(['task', name, '--journal', text]);

  /// 起一件任务。
  Future<TableResult> create(String name, String workflow) =>
      call(['task', '--new', name, '--workflow', workflow]);

  // ---- 工作流（定义侧）----

  Future<TableResult> workflowList() => call(['workflow', '--list']);

  Future<List<WorkflowSummary>> workflows() async => (await workflowList()).rows
      .map(WorkflowSummary.fromRow)
      .toList(growable: false);

  Future<WorkflowDetail> workflow(String name) async =>
      WorkflowDetail.fromData((await call(['workflow', name])).data);

  /// 核对判据里的路径在不在、描述提到的小节有没有判据覆盖。
  Future<TableResult> workflowCheck(String name) =>
      call(['workflow', name, '--check']);

  // ---- 其他 ----

  /// 探活已部署的 provider。
  Future<TableResult> health() => call(['health']);
}
