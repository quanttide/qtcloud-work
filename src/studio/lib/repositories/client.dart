import '../models/task.dart';
import '../models/workflow.dart';
import '../models/workspace.dart';
import 'envelope.dart';
import 'runner.dart';
import 'studio_repository.dart';

/// 命令行说不行（`ok` 为假）。把它印的话带出来。
class CliFailure implements Exception {
  const CliFailure(this.result);

  final TableResult result;

  String get message =>
      result.lines.isEmpty ? '命令行没有成功，也没说为什么' : result.lines.join('\n');

  @override
  String toString() => 'CliFailure: $message';
}

/// 命令行客户端：把命令跑起来，把统一信封（`ok` / `columns` / `rows` / `lines`）
/// 读成模型。
///
/// 界面不自己去读工作流与任务文件的原文——那是命令行的活。两套实现之一；
/// 另一套是 `local/local_repository.dart`（不起命令面，直接算）。
class QtcloudWork implements StudioRepository {
  QtcloudWork({
    required this.workspace,
    Runner? runner,
    this.binary = 'qtcloud-work',
  }) : runner = runner ?? CoreRunner(workspace);

  final Workspace workspace;
  final Runner runner;
  final String binary;

  /// 跑一条命令，拿统一信封，不管 `ok`。[arguments] 不含三处位置与 `--json`，
  /// 这里统一带上。核对定义要这一条——「有地方要改」也是结果，不算出错。
  Future<TableResult> envelope(List<String> arguments) async {
    final output = await runner.run(binary, [
      ...workspace.globalArgs,
      '--json',
      ...arguments,
    ]);
    return TableResult.fromStdout(output.stdout);
  }

  /// 跑一条命令，要求成功；不行就抛 [CliFailure]。
  Future<TableResult> call(List<String> arguments) async {
    final result = await envelope(arguments);
    if (!result.ok) throw CliFailure(result);
    return result;
  }

  // ---- 任务（执行侧）----

  @override
  Future<List<TaskSummary>> tasks() async =>
      (await call(['task', '--list'])).rows
          .map(TaskSummary.fromRow)
          .toList(growable: false);

  @override
  Future<TaskDetail> task(String name) async =>
      TaskDetail.fromData((await call(['task', name])).data);

  @override
  Future<void> next(String name) async {
    await call(['task', name, '--next']);
  }

  @override
  Future<void> done(String name, String step, {String note = ''}) async {
    await call([
      'task',
      name,
      '--done',
      step,
      if (note.isNotEmpty) ...['--note', note],
    ]);
  }

  @override
  Future<void> journal(String name, String text) async {
    await call(['task', name, '--journal', text]);
  }

  @override
  Future<void> create(String name, String workflow) async {
    await call(['task', '--new', name, '--workflow', workflow]);
  }

  // ---- 工作流（定义侧）----

  @override
  Future<List<WorkflowSummary>> workflows() async =>
      (await call(['workflow', '--list'])).rows
          .map(WorkflowSummary.fromRow)
          .toList(growable: false);

  @override
  Future<WorkflowDetail> workflow(String name) async =>
      WorkflowDetail.fromData((await call(['workflow', name])).data);

  @override
  Future<DefinitionCheck> check(String name) async {
    final result = await envelope(['workflow', name, '--check']);
    return DefinitionCheck(ok: result.ok, lines: result.lines);
  }

  // ---- 其他 ----

  @override
  Future<List<String>> health() async => (await call(['health'])).lines;
}
