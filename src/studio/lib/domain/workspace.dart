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

  Map<String, String> toJson() => {
    'root': root,
    'data': data,
    'workflows': workflows,
  };
}
