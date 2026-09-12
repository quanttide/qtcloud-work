import 'package:quanttide_work/quanttide_work.dart' show Outcome;
import 'rules.dart';
import 'workflows.dart';
import 'help.dart';
import 'host/host.dart';
import 'tasks.dart';
import 'task_run.dart';

/// 命令面：与 `qtcloud-work` 一条一条对齐。
///
/// 入口（`bin/qtcloud.dart`）与界面（`lib/repositories/client.dart`）都走这里；
/// 还没搬的命令如实报「还没搬」，不假装成功。
const String defaultApiBase = 'https://api.quanttide.com/qtcloud-work';

Outcome dispatch(
  List<String> args, {
  required String root,
  required String data,
  String? workflows,
  String? server,
}) {
  if (args.isEmpty) return Outcome.failed(['给一条命令。`help` 看有哪些。']);
  final command = args.first;
  final tail = args.sublist(1);
  switch (command) {
    case 'workflow':
      return workflowCommand(tail, data, root, workflows);
    case 'task':
      return taskCommand(tail, data, root, workflows);
    case 'help':
      return helpOf(tail.isEmpty ? null : tail.first);
    case 'health':
      return healthOf(server ?? baseFromEnv());
    case 'find':
    case 'catalog':
    case 'audit':
    case 'material':
      return Outcome.failed(['还没搬：$command']);
    default:
      return Outcome.failed(['不认识这条命令：$command']);
  }
}

/// 网关地址：`--server` 参数 > 环境变量 > 默认。
String baseFromEnv() {
  final fromEnv = envOf('QTCLOUD_WORK_API_BASE_URL');
  return (fromEnv == null || fromEnv.isEmpty) ? defaultApiBase : fromEnv;
}

Outcome workflowCommand(
  List<String> args,
  String data,
  String root,
  String? workflows,
) {
  if (args.isEmpty) return Outcome.failed(['给一条子命令：--list / <名字> / --new …']);
  final first = args.first;
  if (first == '--list') return workflowList(data, workflows);
  if (first == '--new') {
    var name = '';
    var note = '';
    var steps = <String>[];
    for (var i = 1; i < args.length; i++) {
      switch (args[i]) {
        case '--steps':
          steps = args[++i]
              .split(',')
              .map((s) => s.trim())
              .where((s) => s.isNotEmpty)
              .toList();
        case '--note':
          note = args[++i];
        default:
          name = args[i];
      }
    }
    return workflowNew(data, name, steps, note, workflows);
  }
  if (first == '--import') {
    var source = '';
    var as = '';
    for (var i = 1; i < args.length; i++) {
      if (args[i] == '--as') {
        as = args[++i];
      } else {
        source = args[i];
      }
    }
    return workflowImport(data, source, as, workflows);
  }
  final name = first;
  final rest = args.sublist(1);
  if (rest.contains('--check')) {
    return workflowCheck(data, name, root, workflows);
  }
  if (rest.contains('--export')) {
    return workflowExport(
      data,
      name,
      rest[rest.indexOf('--export') + 1],
      workflows,
    );
  }
  return workflowShow(data, name, workflows);
}

Outcome taskCommand(
  List<String> args,
  String data,
  String root,
  String? workflows,
) {
  if (args.isEmpty) return Outcome.failed(['给一条子命令：--list / <名字> / --new …']);
  final first = args.first;
  if (first == '--list') return taskList(root, data, workflows);
  if (first == '--new') {
    var name = '';
    var flow = '';
    for (var i = 1; i < args.length; i++) {
      if (args[i] == '--workflow') {
        flow = args[++i];
      } else {
        name = args[i];
      }
    }
    return taskNew(root, data, name, flow, workflows);
  }
  final name = first;
  final rest = args.sublist(1);
  var note = '';
  if (rest.contains('--note')) {
    note = rest[rest.indexOf('--note') + 1];
  }
  if (rest.contains('--journal')) {
    return taskJournal(
      root,
      data,
      name,
      rest[rest.indexOf('--journal') + 1],
      workflows,
    );
  }
  if (rest.contains('--next')) {
    return taskStep(root, data, name, '', note, true, workflows);
  }
  if (rest.contains('--done')) {
    final at = rest.indexOf('--done');
    final step = (at + 1 < rest.length && !rest[at + 1].startsWith('--'))
        ? rest[at + 1]
        : '';
    return taskStep(root, data, name, step, note, false, workflows);
  }
  return taskStatus(root, data, name, workflows);
}
