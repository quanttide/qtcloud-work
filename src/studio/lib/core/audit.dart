import 'fs/fs.dart' as fs;
import 'host/host.dart';
import 'outcome.dart';

/// 判据：跑定义里写下的机械核对。
///
/// 规则引擎的判据是**字段**，不是一行小语法：`path` 存在、`absent` 不存在、
/// `file` + `contains` 含这段文字、`run` 这条命令退出码为零。路径相对工作区根。
enum RuleKind { path, absent, contains, run }

/// 一条要跑的判据：说明 + 怎么判（[kind] 为空即不跑，交给智能体或人）。
class RuleItem {
  RuleItem(this.description, {this.kind, this.args = const []});

  final String description;
  final RuleKind? kind;
  final List<String> args;

  bool get machine => kind != null;
}

String _text(Map criterion, String key) {
  final value = criterion[key];
  return value is String ? value : '';
}

/// 说明：写了就用写的，没写按字段拼一句。
String descriptionOf(Map criterion) {
  final written = _text(criterion, 'description');
  if (written.trim().isNotEmpty) return written.trim();
  final path = _text(criterion, 'path');
  if (path.isNotEmpty) return '存在：$path';
  final absent = _text(criterion, 'absent');
  if (absent.isNotEmpty) return '不存在：$absent';
  final file = _text(criterion, 'file');
  if (file.isNotEmpty) return '含「${_text(criterion, 'contains')}」：$file';
  final run = _text(criterion, 'run');
  if (run.isNotEmpty) return '跑通：$run';
  return '';
}

/// 把定义里的判据翻成要跑的东西：rule 的跑，agent / human 的不跑。
List<RuleItem> itemsOf(List<Map> criteria) {
  final items = <RuleItem>[];
  for (final criterion in criteria) {
    final description = descriptionOf(criterion);
    final isRule = _text(criterion, 'executor') == 'rule';
    if (!isRule) {
      items.add(RuleItem(description));
      continue;
    }
    final path = _text(criterion, 'path');
    final absent = _text(criterion, 'absent');
    final file = _text(criterion, 'file');
    final run = _text(criterion, 'run');
    if (path.isNotEmpty) {
      items.add(RuleItem(description, kind: RuleKind.path, args: [path]));
    } else if (absent.isNotEmpty) {
      items.add(RuleItem(description, kind: RuleKind.absent, args: [absent]));
    } else if (file.isNotEmpty) {
      items.add(RuleItem(
        description,
        kind: RuleKind.contains,
        args: [file, _text(criterion, 'contains')],
      ));
    } else if (run.isNotEmpty) {
      items.add(RuleItem(description, kind: RuleKind.run, args: [run]));
    }
  }
  return items;
}

/// 跑一条判据，返回（是否通过，说明）。
(bool, String) checkRule(String root, RuleItem item) {
  switch (item.kind) {
    case RuleKind.path:
      final target = item.args[0];
      return (fs.fileExists('$root/$target') || fs.dirExists('$root/$target'), target);
    case RuleKind.absent:
      final target = item.args[0];
      return (!fs.fileExists('$root/$target') && !fs.dirExists('$root/$target'), target);
    case RuleKind.contains:
      final target = item.args[0];
      final needle = item.args[1];
      if (!fs.fileExists('$root/$target')) return (false, '$target 不存在');
      return (fs.readText('$root/$target').contains(needle), '$target 含「$needle」');
    case RuleKind.run:
      final command = item.args[0];
      final result = runShell(command, root);
      if (result.code == 0) return (true, command);
      final text = result.err.trim().isEmpty ? result.out.trim() : result.err.trim();
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
