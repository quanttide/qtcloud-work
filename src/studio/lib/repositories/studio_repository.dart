import 'package:quanttide_work/quanttide_work.dart' as qt;

/// 工作台的数据边界：任务、工作流、探活。界面与 Bloc 只依赖这一份接口。
///
/// 交出来的就是工具箱里的领域对象——一条定义是 [qt.Workflow]，一件任务是 [qt.Task]。
/// 界面要的派生值（走过几步、下一步、进度、判据条数）由领域对象自己算，
/// 不另造一套模型。
///
/// 唯一实现：[`local/local_repository.dart`](local/local_repository.dart)——直接调命令面。
///
/// 测试注入假实现（见 `test/support/`），不碰真实文件与进程。
abstract class StudioRepository {
  // ---- 任务（执行侧）----

  /// 有哪些任务：名字、跑哪条工作流、下一步。
  Future<List<({String name, String workflow, String next})>> tasks();

  /// 打开一件任务：任务本身 + 它跑的那条定义（算下一步要用）+ 三样产物的落点
  /// （落点要按工作区算，所以是这边算好递出来，相对数据仓）。
  Future<({qt.Task task, qt.Workflow workflow, Map<String, String> artifacts})>
  task(String name);

  /// 走下一步。这一步多半交给 AI 跑，会慢。
  Future<void> next(String name);

  /// 人为地记一步；[note] 是这一句话的说明。
  Future<void> done(String name, String step, {String note});

  /// 日志收叙事（只能人来写）。
  Future<void> journal(String name, String text);

  /// 起一件任务：跑 [workflow] 这条工作流。
  Future<void> create(String name, String workflow);

  // ---- 工作流（定义侧）----

  /// 有哪些定义：名字、步骤串、位置。
  Future<List<({String name, String steps, String path})>> workflows();

  /// 打开一条定义：聚合 + 文件位置 + 原文（定义态要看）。
  Future<({qt.Workflow workflow, String path, String yaml})> workflow(String name);

  /// 核对定义：过没过、给人看的话。
  Future<({bool ok, List<String> lines})> check(String name);

  // ---- 其他 ----

  /// 探活已部署的 provider。
  Future<List<String>> health();
}
