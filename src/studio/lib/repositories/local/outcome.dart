import 'dart:convert';

/// 信封：动作结果。`ok` 定退出码，`lines` 给人看，`columns` 与 `rows` 给窗口画。
///
/// 原先抽在工具箱里两侧共用，后来决定它不是领域模型（是各平台自己的说法），
/// 就搬回这里。
class Outcome {
  Outcome(
    this.ok, {
    List<String>? lines,
    List<String>? columns,
    List<List<String>>? rows,
    this.payload,
  }) : lines = lines ?? <String>[],
       columns = columns ?? <String>[],
       rows = rows ?? <List<String>>[];

  Outcome.failed(List<String> lines) : this(false, lines: lines);

  bool ok;
  List<String> lines;
  List<String> columns;
  List<List<String>> rows;

  /// 有些动作要交出结构化的东西，而不是行列。
  Object? payload;

  Outcome withFirst(String line) {
    lines.insert(0, line);
    return this;
  }

  Map<String, Object?> toJson() => payload != null && payload is Map
      ? (payload! as Map<String, Object?>)
      : {'ok': ok, 'lines': lines, 'columns': columns, 'rows': rows};
}

/// 路径相对根写短一点；不在根底下就原样。
String short(String root, String path) {
  final prefix = root.endsWith('/') ? root : '$root/';
  return path.startsWith(prefix) ? path.substring(prefix.length) : path;
}

/// 给界面的结构化数据：界面不读给人看的那些话，只读这个。
///
/// 命令行那边没有这一栏——它印的是 `lines`。尺子只比 `ok / columns / rows`，
/// 所以这里的多出来的一栏不影响两边「结果一致」。
final Map<Outcome, Map<String, Object?>> outcomeData = Map.identity();

/// 把结果编成命令行那一种输出（`--json` 时是信封，否则一行一行）。
String encodeOutcome(Outcome outcome, {bool asJson = true}) {
  if (asJson) {
    final json = Map<String, Object?>.from(outcome.toJson());
    final data = outcomeData[outcome];
    if (data != null) json['data'] = data;
    return jsonEncode(json);
  }
  return '${outcome.lines.join('\n')}\n';
}
