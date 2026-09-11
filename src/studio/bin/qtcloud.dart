//! studio 侧的命令入口。
//!
//! 与 `qtcloud-work` 同一个命令面、同一个输出信封（ok / columns / rows / lines），
//! 用途有二：一是让 studio 不依赖命令行也能干活，二是让「对表」有得比——
//! 同一处工作区、同一条命令，两边各跑一次，信封必须一样。
//!
//! 用法：dart run bin/qtcloud.dart [--root R] [--data D] [--workflows W] [--json] <命令…>

import 'dart:convert';
import 'dart:io';

import 'package:qtcloud_work_studio/core/definition.dart';
import 'package:qtcloud_work_studio/core/help.dart';
import 'package:qtcloud_work_studio/core/task.dart';
import 'package:qtcloud_work_studio/core/outcome.dart';

void main(List<String> argv) {
  final args = [...argv];
  var root = '.';
  var data = 'data';
  String? workflows;
  var asJson = false;
  final rest = <String>[];

  for (var i = 0; i < args.length; i++) {
    switch (args[i]) {
      case '--root':
        root = args[++i];
      case '--data':
        data = args[++i];
      case '--workflows':
        workflows = args[++i];
      case '--json':
        asJson = true;
      default:
        rest.add(args[i]);
    }
  }

  final outcome = dispatch(rest, root: root, data: data, workflows: workflows);
  if (asJson) {
    stdout.writeln(jsonEncode(outcome.toJson()));
  } else {
    for (final line in outcome.lines) {
      stdout.writeln(line);
    }
  }
  exit(outcome.ok ? 0 : 1);
}

Outcome dispatch(
  List<String> args, {
  required String root,
  required String data,
  String? workflows,
}) {
  if (args.isEmpty) return Outcome.failed(['给一条命令。`help` 看有哪些。']);
  final command = args.first;
  final tail = args.sublist(1);
  switch (command) {
    case 'workflow':
      return _workflow(tail, data, root, workflows);
    case 'help':
      return helpOf(tail.isEmpty ? null : tail.first);
    case 'task':
      return _task(tail, data, root, workflows);
    case 'find':
    case 'catalog':
    case 'audit':
    case 'material':
    case 'health':
      return Outcome.failed(['还没搬：$command']);
    default:
      return Outcome.failed(['不认识这条命令：$command']);
  }
}

Outcome _workflow(
  List<String> args,
  String data,
  String root,
  String? workflows,
) {
  if (args.isEmpty) {
    return Outcome.failed(['给一条子命令：--list / <名字> / --new …']);
  }
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
  if (rest.contains('--check')) return workflowCheck(data, name, root, workflows);
  if (rest.contains('--export')) {
    return workflowExport(data, name, rest[rest.indexOf('--export') + 1], workflows);
  }
  return workflowShow(data, name, workflows);
}

Outcome _task(List<String> args, String data, String root, String? workflows) {
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
  if (rest.contains('--journal')) {
    return taskJournal(root, data, name, rest[rest.indexOf('--journal') + 1], workflows);
  }
  if (rest.contains('--next') || rest.contains('--done')) {
    return Outcome.failed(['还没搬：task --next / --done（要先把规则引擎那块搬过来）']);
  }
  return taskStatus(root, data, name, workflows);
}
