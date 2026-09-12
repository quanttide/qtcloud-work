import 'fs/fs.dart' as fs;
import 'host/host.dart';

/// 判据：跑定义里写下的机械核对。
///
/// 规则引擎的判据是**字段**，不是一行小语法：`path` 存在、`absent` 不存在、
/// `file` + `contains` 含这段文字、`run` 这条命令退出码为零。路径相对工作区根。
// 判据的翻译（说明怎么写、四种判法怎么认）在工具箱里；这里只剩「真去跑」。
export 'package:quanttide_work/quanttide_work.dart'
    show RuleKind, RuleItem, itemsOf;

import 'package:quanttide_work/quanttide_work.dart';

/// 跑一条判据，返回（是否通过，说明）。
(bool, String) checkRule(String root, RuleItem item) {
  switch (item.kind) {
    case RuleKind.path:
      final target = item.args[0];
      return (
        fs.fileExists('$root/$target') || fs.dirExists('$root/$target'),
        target,
      );
    case RuleKind.absent:
      final target = item.args[0];
      return (
        !fs.fileExists('$root/$target') && !fs.dirExists('$root/$target'),
        target,
      );
    case RuleKind.contains:
      final target = item.args[0];
      final needle = item.args[1];
      if (!fs.fileExists('$root/$target')) return (false, '$target 不存在');
      return (
        fs.readText('$root/$target').contains(needle),
        '$target 含「$needle」',
      );
    case RuleKind.run:
      final command = item.args[0];
      final result = runShell(command, root);
      if (result.code == 0) return (true, command);
      final text = result.err.trim().isEmpty
          ? result.out.trim()
          : result.err.trim();
      final lines = text.split('\n').where((l) => l.trim().isNotEmpty).toList();
      final tail = lines.isEmpty ? '无输出' : lines.last.trim();
      return (false, '$command——$tail');
    case null:
      return (false, '不认得的判据');
  }
}

/// 跑全部要跑的判据，返回（逐条结果，不跑的——留给智能体或人）。
(List<(RuleItem, bool, String)>, List<RuleItem>) runRules(
  String root,
  List<RuleItem> items,
) {
  final results = <(RuleItem, bool, String)>[];
  for (final item in items.where((i) => i.machine)) {
    final (passed, spec) = checkRule(root, item);
    results.add((item, passed, spec));
  }
  return (results, items.where((i) => !i.machine).toList());
}

/// 探活：命令行那条路是 GET `<网关>/health`。
Outcome healthOf(String base) {
  final result = httpGet('$base/health');
  if (!result.ok) {
    return Outcome.failed(['请求 $base/health 失败：${result.body}']);
  }
  return Outcome(true, lines: result.body.split('\n'));
}
