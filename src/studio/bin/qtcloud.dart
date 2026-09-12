//! studio 侧的命令入口。
//!
//! 与 `qtcloud-work` 同一个命令面、同一个结果（ok / lines / columns / rows / data），
//! 用途有二：一是让 studio 不依赖命令行也能干活，二是让「对表」有得比——
//! 同一处工作区、同一条命令，两边各跑一次，信封必须一样。
//!
//! 命令面在 `lib/repositories/local/dispatch.dart` 一处实现，界面那层也走它。
//!
//! 用法：dart run bin/qtcloud.dart [--root R] [--data D] [--workflows W] [--json] <命令…>

import 'dart:convert';
import 'dart:io';

import 'package:qtcloud_work_studio/repositories/local/dispatch.dart';

void main(List<String> argv) {
  final args = [...argv];
  var root = '.';
  var data = 'data';
  String? workflows;
  String? server;
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
      case '--server':
        server = args[++i];
      case '--json':
        asJson = true;
      default:
        rest.add(args[i]);
    }
  }

  final outcome = dispatch(rest, root: root, data: data, workflows: workflows, server: server);
  stdout.write(
    asJson
        ? jsonEncode(outcome.toJson())
        : '${outcome.lines.join('\n')}\n',
  );
  exit(outcome.ok ? 0 : 1);
}
