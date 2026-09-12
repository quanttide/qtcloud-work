import '../models/task.dart';
import '../models/workflow.dart';

/// 工作台的数据边界：任务、工作流、探活。界面与 Bloc 只依赖这一份接口。
///
/// 两套实现：
/// - [`client.dart`](client.dart)：命令行客户端——把命令跑起来，读统一信封
///   （`ok` / `columns` / `rows` / `lines`）再装成模型；
/// - [`local/local_repository.dart`](local/local_repository.dart)：本地那套——
///   直接调命令面，从结构化那一栏装模型，不编信封。
///
/// 测试注入假实现（见 `test/support/`），不碰真实文件与进程。
abstract class StudioRepository {
  // ---- 任务（执行侧）----

  /// 有哪些任务、各自的下一步。
  Future<List<TaskSummary>> tasks();

  /// 一件任务的现状（步骤 + 流水）。
  Future<TaskDetail> task(String name);

  /// 走下一步。这一步多半交给 AI 跑，会慢。
  Future<void> next(String name);

  /// 人为地记一步；[note] 是这一句话的说明。
  Future<void> done(String name, String step, {String note});

  /// 日志收叙事（只能人来写）。
  Future<void> journal(String name, String text);

  /// 起一件任务：跑 [workflow] 这条工作流。
  Future<void> create(String name, String workflow);

  // ---- 工作流（定义侧）----

  /// 有哪些工作流。
  Future<List<WorkflowSummary>> workflows();

  /// 一条定义的步骤、判据与原文。
  Future<WorkflowDetail> workflow(String name);

  /// 核对定义：判据里的路径在不在、描述提到的小节有没有判据覆盖。
  Future<DefinitionCheck> check(String name);

  // ---- 其他 ----

  /// 探活已部署的 provider。
  Future<List<String>> health();
}
