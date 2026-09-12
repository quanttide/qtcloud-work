import 'package:quanttide_work/quanttide_work.dart' as qt;

import 'package:quanttide_work/quanttide_work.dart' show Outcome;
import 'runner.dart';
import 'studio_repository.dart';

/// 命令行说不行（`ok` 为假）。把它印的话带出来。
class CliFailure implements Exception {
  const CliFailure(this.result);

  final Outcome result;

  String get message =>
      result.lines.isEmpty ? '命令行没有成功，也没说为什么' : result.lines.join('\n');

  @override
  String toString() => 'CliFailure: $message';
}

/// 命令行客户端：把命令跑起来，把结果（`ok` / `lines` / `columns` / `rows` / `data`）
/// 读成工具箱里的领域对象。
///
/// 界面不自己去读工作流与任务文件的原文——那是命令行的活。两套实现之一；
/// 另一套是 `local/local_repository.dart`（不起命令面，直接算）。
class QtcloudWork implements StudioRepository {
  QtcloudWork({
    required this.workspace,
    Runner? runner,
    this.binary = 'qtcloud-work',
  }) : runner = runner ?? CoreRunner(workspace);

  /// 三处位置（就是工具箱里的运行上下文）。
  final qt.RunContext workspace;
  final Runner runner;
  final String binary;

  /// 跑一条命令，拿结果，不管 `ok`。核对定义要这一条——「有地方要改」
  /// 也是结果，不算出错。
  Future<Outcome> envelope(List<String> arguments) async {
    final output = await runner.run(binary, [
      '--root',
      workspace.root,
      '--data',
      workspace.data,
      '--workflows',
      workspace.workflows,
      '--json',
      ...arguments,
    ]);
    return Outcome.fromStdout(output.stdout);
  }

  /// 跑一条命令，要求成功；不行就抛 [CliFailure]。
  Future<Outcome> call(List<String> arguments) async {
    final result = await envelope(arguments);
    if (!result.ok) throw CliFailure(result);
    return result;
  }

  // ---- 任务（执行侧）----

  @override
  Future<List<({String name, String workflow, String next})>> tasks() async {
    final rows = (await call(['task', '--list'])).rows;
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
  Future<({qt.Task task, qt.Workflow workflow, Map<String, String> products})>
  task(String name) async {
    final data =
        (await call(['task', name])).data ?? const <String, Object?>{};
    final task = qt.Task.of(name, _payload(data));
    final products = Map<String, String>.from(
      (data['products'] as Map?) ?? const <String, Object?>{},
    );
    return (
      task: task,
      workflow: (await workflow(task.workflowName)).workflow,
      products: products,
    );
  }

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
  Future<List<({String name, String steps, String path})>> workflows() async {
    final rows = (await call(['workflow', '--list'])).rows;
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
    final data =
        (await call(['workflow', name])).data ?? const <String, Object?>{};
    return (
      workflow: qt.Workflow.of(name, _payload(data)),
      path: '${data['path'] ?? ''}',
      yaml: '${data['yaml'] ?? ''}',
    );
  }

  @override
  Future<({bool ok, List<String> lines})> check(String name) async {
    final result = await envelope(['workflow', name, '--check']);
    return (ok: result.ok, lines: result.lines);
  }

  // ---- 其他 ----

  @override
  Future<List<String>> health() async => (await call(['health'])).lines;
}

/// 信封那一栏 `data` 里托着的原文（任务文件 / 定义文件的内容）。
Map _payload(Map<String, Object?> data) =>
    (data['payload'] as Map?) ?? const <String, Object?>{};
