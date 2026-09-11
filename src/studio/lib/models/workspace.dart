/// 三处位置。它们是命令行唯一的输入，也是界面唯一要记住的东西。
class Workspace {
  const Workspace({
    required this.root,
    required this.data,
    required this.workflows,
  });

  /// 工作区（`--root`）：文档与代码本体，判据里的相对路径从这算。
  final String root;

  /// 数据仓（`--data`）：任务、流水、产物草稿。
  final String data;

  /// 工作流目录（`--workflows`）：工作流定义。
  final String workflows;

  List<String> get globalArgs => [
    '--root',
    root,
    '--data',
    data,
    '--workflows',
    workflows,
  ];

  /// 从环境读，没给就用当前目录下的常见位置。
  factory Workspace.fromEnvironment([Map<String, String>? environment]) {
    final env = environment ?? const <String, String>{};
    return Workspace(
      root: env['QTCLOUD_WORK_ROOT'] ?? '.',
      data: env['QTCLOUD_WORK_DATA'] ?? 'data/context/qtcloud-work',
      workflows: env['QTCLOUD_WORK_WORKFLOWS'] ?? 'data/profile/quanttide/workflows',
    );
  }

  Map<String, String> toJson() => {
    'root': root,
    'data': data,
    'workflows': workflows,
  };
}
