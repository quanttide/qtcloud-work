import 'package:quanttide_work/quanttide_work.dart' show Outcome;
import 'package:quanttide_work/quanttide_work.dart' as qt;

import '../studio_repository.dart';
import 'dispatch.dart';

/// 命令面说不行（`ok` 为假）。把它印的话带出来。
class LocalFailure implements Exception {
  const LocalFailure(this.lines);

  final List<String> lines;

  String get message => lines.isEmpty ? '没跑成，也没说为什么' : lines.join('\n');

  @override
  String toString() => 'LocalFailure: $message';
}

/// 实现：不起命令面、不编信封，直接把命令派给 `dispatch`，
/// 再从结构化那一栏拿回任务与定义的原文，装成工具箱的领域对象。
///
/// 与 `bin/qtcloud.dart` 算的是同一套结果——同一处工作区、同一条命令面。
class LocalRepository implements StudioRepository {
  LocalRepository(this.workspace);

  final qt.RunContext workspace;

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

  /// 界面那一栏：界面不读给人看的话，只读这一栏。
  Map<String, Object?> _data(List<String> arguments) =>
      _run(arguments).data ?? const {};

  // ---- 任务（执行侧）----

  @override
  Future<List<({String name, String workflow, String next})>> tasks() async {
    final rows = _run(['task', '--list']).rows;
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
  task(String name) async {
    final data = _data(['task', name]);
    final task = qt.Task.of(name, _payload(data));
    final artifacts = Map<String, String>.from(
      (data['artifacts'] as Map?) ?? const <String, Object?>{},
    );
    return (
      task: task,
      workflow: (await workflow(task.workflowName)).workflow,
      artifacts: artifacts,
    );
  }

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
  Future<List<({String name, String steps, String path})>> workflows() async {
    final rows = _run(['workflow', '--list']).rows;
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
    final data = _data(['workflow', name]);
    return (
      workflow: qt.Workflow.of(name, _payload(data)),
      path: '${data['path'] ?? ''}',
      yaml: '${data['yaml'] ?? ''}',
    );
  }

  @override
  Future<({bool ok, List<String> lines})> check(String name) async {
    final outcome = dispatch(
      ['workflow', name, '--check'],
      root: workspace.root,
      data: workspace.data,
      workflows: workspace.workflows,
    );
    return (ok: outcome.ok, lines: outcome.lines);
  }

  // ---- 其他 ----

  @override
  Future<List<String>> health() async => _run(['health']).lines;
}

/// 结构化那一栏里托着的原文（任务文件 / 定义文件的内容）。
Map _payload(Map<String, Object?> data) =>
    (data['payload'] as Map?) ?? const <String, Object?>{};
