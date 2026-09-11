/// 动作结果：命令行与窗口共用的一份算出来的东西。
///
/// `ok` 定退出码，`lines` 给命令行印，`columns` 与 `rows` 给窗口画；
/// 动作之间不互相打印，都只交出这一层。
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

  /// 在最前面插一句。
  Outcome withFirst(String line) {
    lines.insert(0, line);
    return this;
  }

  Map<String, Object?> toJson() => payload != null && payload is Map
      ? (payload! as Map<String, Object?>)
      : {'ok': ok, 'lines': lines, 'columns': columns, 'rows': rows};

  @override
  String toString() => 'Outcome(ok: $ok, rows: ${rows.length})';
}

/// 路径相对根写短一点；不在根底下就原样。
String short(String root, String path) {
  final normalizedRoot = root.endsWith('/') ? root : '$root/';
  if (path.startsWith(normalizedRoot)) {
    return path.substring(normalizedRoot.length);
  }
  return path;
}
