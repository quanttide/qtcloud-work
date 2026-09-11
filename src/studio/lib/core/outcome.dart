import 'dart:convert';

import 'package:quanttide_work/quanttide_work.dart';

/// 信封：动作结果。`ok` 定退出码，`lines` 给人看，`columns` 与 `rows` 给窗口画。
///
/// 这一份领域模型已经抽到工具箱 `quanttide_work`——这里只把工具箱那份引出来，
/// 加上一个命令行那一种输出。
export 'package:quanttide_work/quanttide_work.dart' show Outcome, short;

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
