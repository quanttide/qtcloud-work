import 'dart:convert';

import 'package:quanttide_work/quanttide_work.dart';

/// 信封：动作结果。`ok` 定退出码，`lines` 给人看，`columns` 与 `rows` 给窗口画。
///
/// 这一份领域模型已经抽到工具箱 `quanttide_work`——这里只把工具箱那份引出来，
/// 加上一个命令行那一种输出。
export 'package:quanttide_work/quanttide_work.dart' show Outcome, short;

/// 把结果编成命令行那一种输出（`--json` 时是信封，否则一行一行）。
String encodeOutcome(Outcome outcome, {bool asJson = true}) {
  if (asJson) return jsonEncode(outcome.toJson());
  return '${outcome.lines.join('\n')}\n';
}
