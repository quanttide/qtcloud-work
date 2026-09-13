/// 一次工作的三处位置：工作区根、数据仓、工作流目录。
///
/// 位置不进模型——领域模型只管内容（「平台管位置，工具库管内容」），所以这一份是
/// 窗口自己的，与命令行任务上的三处一致。工作流目录空着即跟在数据仓里。
class RunContext {
  const RunContext({required this.root, required this.data, this.workflows = ''});

  final String root;
  final String data;
  final String workflows;
}
