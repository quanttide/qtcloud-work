import 'dart:convert';

/// 命令行的统一输出信封。
///
/// `ok` 定退出码，`lines` 给命令行印，`columns` 与 `rows` 给窗口画——
/// 这三样是命令行与界面之间的接口，界面不去读工作流与任务文件的原文。
class TableResult {
  const TableResult({
    required this.ok,
    this.columns = const [],
    this.lines = const [],
    this.rows = const [],
    this.data = const {},
  });

  final bool ok;
  final List<String> columns;
  final List<String> lines;
  final List<List<String>> rows;

  /// 界面用的结构化数据（core 交出来的那一栏）。命令行那边没有它。
  final Map<String, Object?> data;

  factory TableResult.fromJson(Map<String, dynamic> json) => TableResult(
    ok: json['ok'] == true,
    columns: _strings(json['columns']),
    lines: _strings(json['lines']),
    rows: (json['rows'] as List? ?? const [])
        .map(_strings)
        .toList(growable: false),
    data: (json['data'] as Map?)?.cast<String, Object?>() ?? const {},
  );

  factory TableResult.fromStdout(String stdout) =>
      TableResult.fromJson(jsonDecode(stdout) as Map<String, dynamic>);

  static List<String> _strings(Object? value) =>
      (value as List? ?? const []).map((e) => '$e').toList(growable: false);

  /// 取以 [prefix] 开头的那一行，去掉前缀与两侧空白。找不到给 null。
  String? valueAfter(String prefix) {
    for (final line in lines) {
      final trimmed = line.trimLeft();
      if (trimmed.startsWith(prefix)) {
        return trimmed.substring(prefix.length).trim();
      }
    }
    return null;
  }

  /// 取 [marker] 之后的全部行（去掉两侧空白与空行）。
  List<String> linesAfter(String marker) {
    final at = lines.indexWhere((line) => line.trim() == marker);
    if (at < 0) return const [];
    return lines
        .sublist(at + 1)
        .map((line) => line.trim())
        .where((line) => line.isNotEmpty)
        .toList(growable: false);
  }

  @override
  String toString() => 'TableResult(ok: $ok, rows: ${rows.length})';
}
