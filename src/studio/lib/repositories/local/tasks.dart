import 'package:quanttide_work/quanttide_work.dart' show Outcome;
import 'paths.dart';
import 'workflows.dart';
import 'fs/fs.dart' as fs;
import 'package:quanttide_work/quanttide_work.dart' as qt;
import 'yaml.dart';

/// 任务：工作流的一次执行实例。
///
/// `<数据仓>/tasks/<任务>.yaml` 是一次执行：跑哪条工作流 + 自带的运行上下文
/// （`root` / `data` / `workflows`）+ 流水；产物落在 `artifacts/` 下。
const String logKind = 'log';
const String reportKind = 'report';
const String journalKind = 'journal';

/// 本地时间，格式与命令行一致（`date +%Y-%m-%d %H:%M`）。
String now() {
  final at = DateTime.now();
  String two(int value) => value.toString().padLeft(2, '0');
  return '${at.year}-${two(at.month)}-${two(at.day)} ${two(at.hour)}:${two(at.minute)}';
}

class Task {
  Task({
    required this.root,
    required this.data,
    required this.name,
    this.workflows,
  });

  final String root;
  final String data;
  final String name;
  final String? workflows;

  String get file => '$data/tasks/$name.yaml';

  String get artifactsDir => '$data/artifacts';

  bool get exists => fs.fileExists(file);

  /// 这件任务在工具箱里的样子（内容那一层交给工具箱）。
  qt.Task get shared => qt.Task.of(name, payload());

  Map payload() {
    if (!exists) return <String, Object?>{};
    try {
      final value = parseYaml(fs.readText(file));
      return value is Map ? value : <String, Object?>{};
    } catch (_) {
      return <String, Object?>{};
    }
  }

  void save(Map payload) => fs.writeText(file, dumpYaml(payload));

  /// 这次执行往哪写产物（任务是运行数据，产物与它没有从属关系）。
  Map<String, String> artifacts() {
    final raw = payload()['artifacts'];
    if (raw is! Map) return <String, String>{};
    final found = <String, String>{};
    raw.forEach((key, value) {
      if (key is String && value is String) found[key] = value;
    });
    return found;
  }

  /// 闸门项：等人拍板的事项，记在任务文件里。
  List<String> gates() =>
      (payload()['gates'] as List?)?.whereType<String>().toList() ?? const [];

  void setGates(List<String> notes) {
    final body = payload();
    body['gates'] = [...notes];
    save(body);
  }

  /// 这次执行往哪写产物——落点在工具箱里（规范「任务 / 语法」）。
  String artifact(String kind) => shared.artifact(
    kind,
    qt.RunContext(root: root, data: data, workflows: workflows ?? ''),
  );

  String get start => textOf(payload(), 'start');

  String get workflowName => textOf(payload(), 'workflow');

  WorkflowFile workflow() => openWorkflow(data, workflowName, workflows);

  List<Step> steps() => workflow().steps;

  /// 流水：任务文件里的 `log` 一节，一条一条按发生顺序。
  List<Map> events() =>
      (payload()['log'] as List?)?.whereType<Map>().toList() ?? const [];

  /// 哪些步骤走过了：算法在工具箱的任务聚合里（附加判定投票、重新执行从头算）。
  List<String> done() => shared.doneSteps(workflow().shared);

  Step? nextStep() {
    final next = shared.nextStep(workflow().shared);
    if (next == null) return null;
    for (final step in steps()) {
      if (step.name == next) return step;
    }
    return null;
  }

  /// 记一笔流水：读任务文件、追加一条、写回去（流水只增不改）。
  void record(String step, String detail, bool ok) {
    final body = payload();
    final events = (body['log'] as List?)?.toList() ?? <Object?>[];
    events.add({'at': now(), 'step': step, 'detail': detail, 'ok': ok});
    body['log'] = events;
    save(body);
  }

  /// 相对数据仓写短一点；不在数据仓底下就原样。
  String relative(String path) => short(data, path);
}

/// 起一件任务：写下指令（跑哪条工作流、要什么），备好产物三家。
Task createTask(
  String root,
  String data,
  String name,
  String workflowName,
  String? workflows,
) {
  final task = Task(root: root, data: data, name: name, workflows: workflows);
  if (!task.exists) {
    final payload = <String, Object?>{
      'name': name,
      'start': now(),
      'workflow': workflowName,
      'log': <Object?>[],
      'gates': <Object?>[],
      'artifacts': <String, Object?>{},
      ...taskContext(root, data, workflows),
    };
    task.save(payload);
  }
  for (final kind in [reportKind, journalKind]) {
    final declared = (task.artifacts()[kind] ?? '').trim().isNotEmpty;
    if (declared && !fs.fileExists(task.artifact(kind))) {
      fs.writeText(task.artifact(kind), '# $kind：$name\n');
    }
  }
  return task;
}

/// 这次执行自带的运行上下文：工作区按绝对记，草稿仓与工作流目录能相对就相对。
Map<String, String> taskContext(String root, String data, String? workflows) {
  String asWritten(String? path) {
    if (path == null) return '';
    return path.startsWith(root)
        ? path.substring(root.length).replaceFirst(RegExp(r'^/'), '')
        : path;
  }

  return {
    'root': root,
    'data': asWritten(data),
    'workflows': asWritten(workflows),
  };
}

String _expandHome(String value) {
  if (value.startsWith('~/')) {
    final home = fs.env('HOME');
    if (home != null && home.isNotEmpty) return '$home/${value.substring(2)}';
  }
  return value;
}

/// 开一件任务：命令行给了就用命令行的，没给就用任务里记的。
Task reopen(String data, String name, String? root, String? workflows) {
  final raw = Task(root: '.', data: data, name: name).payload();
  final recordedRoot = textOf(raw, 'root');
  final resolvedRoot =
      root ??
      (recordedRoot.isEmpty ? repoRootFrom('.') : _expandHome(recordedRoot));
  String? resolvedFlows = workflows;
  if (resolvedFlows == null) {
    final recorded = textOf(raw, 'workflows');
    if (recorded.isNotEmpty) {
      resolvedFlows = recorded.startsWith('/')
          ? recorded
          : '$resolvedRoot/$recorded';
    }
  }
  return Task(
    root: resolvedRoot,
    data: data,
    name: name,
    workflows: resolvedFlows,
  );
}

/// 往上找仓库根；找不到就用起点（命令行那边是报错，这里给个能用的默认）。
String repoRootFrom(String from) {
  try {
    // 借资产层的规则：含 data/journal 的那一层
    var dir = from;
    while (true) {
      if (fs.dirExists('$dir/data/journal')) return dir;
      final at = dir.lastIndexOf('/');
      if (at <= 0) return from;
      dir = dir.substring(0, at);
    }
  } catch (_) {
    return from;
  }
}

List<Task> listingTasks(String? root, String data, String? workflows) {
  final base = '$data/tasks';
  if (!fs.dirExists(base)) return const [];
  final paths =
      fs.listDir(base).where((path) => path.endsWith('.yaml')).toList()..sort();
  return [
    for (final path in paths)
      reopen(
        data,
        path.split('/').last.replaceAll(RegExp(r'\.yaml$'), ''),
        root,
        workflows,
      ),
  ];
}

String stateLine(Task task) => task.shared.stateLine(task.workflow().shared);

// ---- 动作 ----

Outcome taskNew(
  String root,
  String data,
  String name,
  String workflowName,
  String? workflows,
) {
  if (name.trim().isEmpty) return Outcome.failed(['请先给这件任务起个名字']);
  final flow = openWorkflow(data, workflowName, workflows);
  if (!flow.exists) {
    return Outcome.failed([
      '没有这条工作流：${short(data, flow.file)}（qtcloud-work workflow --list 看有哪些）',
    ]);
  }
  final existing = reopen(data, name.trim(), root, workflows);
  if (existing.exists) {
    return Outcome.failed(['已经有这件任务：${short(data, existing.file)}（换个名字，不覆盖）']);
  }
  final task = createTask(
    root,
    data,
    name.trim(),
    workflowName.trim(),
    workflows,
  );
  return taskStatus(
    root,
    data,
    name.trim(),
    workflows,
  ).withFirst('起了：${short(data, task.file)}');
}

Outcome taskStatus(String? root, String data, String name, String? workflows) {
  if (name.trim().isEmpty) {
    return Outcome.failed(['请先选一件任务（qtcloud-work task --list 看有哪些）']);
  }
  final task = reopen(data, name, root, workflows);
  if (!task.exists) {
    return Outcome.failed(['没有这件任务：${short(data, task.file)}']);
  }
  final finished = task.done();
  final result = Outcome(true)..columns = ['步骤', '状态'];
  result.lines.add('任务：${task.name}');
  result.lines.add('  开工：${task.start.isEmpty ? '（没记）' : task.start}');
  result.lines.add(
    '  工作流：${task.workflowName}——${task.workflow().description}',
  );
  result.lines.add('  步骤：${task.steps().length} 个');
  for (final step in task.steps()) {
    final state = finished.contains(step.name) ? '✓' : '—';
    result.rows.add([step.name, state]);
    result.lines.add('  $state ${step.name}');
  }
  result.lines.add(stateLine(task));
  result.lines.add('指令：${short(data, task.file)}');
  result.lines.add(
    '产物：${short(data, task.artifact(reportKind))}、'
    '${short(data, task.artifact(journalKind))}　'
    '流水：${short(data, task.artifact(logKind))}',
  );
  final events = task.events();
  final tail = events.length <= 5 ? events : events.sublist(events.length - 5);
  result.data = {
    'payload': task.payload(),
    'artifacts': {
      'report': short(data, task.artifact(reportKind)),
      'journal': short(data, task.artifact(journalKind)),
      'log': short(data, task.artifact(logKind)),
    },
  };
  if (events.isNotEmpty) {
    result.lines.add('流水（最近五条）：');
    for (final event in tail) {
      result.lines.add(
        '  ${event['at'] ?? ''}　${event['step'] ?? ''}　${event['detail'] ?? ''}',
      );
    }
  }
  return result;
}

Outcome taskList(String? root, String data, String? workflows) {
  final found = listingTasks(root, data, workflows);
  final result = Outcome(true)..columns = ['任务', '工作流', '下一步'];
  for (final task in found) {
    final next = task.nextStep()?.name ?? '走完';
    result.rows.add([task.name, task.workflowName, next]);
    result.lines.add(
      '${task.name.padRight(24)} 工作流 ${task.workflowName}　下一步：$next',
    );
  }
  if (found.isEmpty) {
    result.lines = ['还没有任务：qtcloud-work task --new <名字> --workflow <工作流>'];
  }
  return result;
}

/// 日志收叙事：一段一段往下写。
void narrate(Task task, String words) {
  final path = task.artifact(journalKind);
  var text = fs.fileExists(path) ? fs.readText(path) : '# 日志：${task.name}\n';
  text = text
      .split('\n')
      .where((line) {
        final stripped = line.trim();
        return !(stripped.startsWith('（') && stripped.endsWith('）'));
      })
      .join('\n');
  text = text.trimRight();
  final trimmed = words.trim();
  fs.writeText(path, '$text\n\n$trimmed\n');
  task.record(
    '历史',
    trimmed.length > 40 ? trimmed.substring(0, 40) : trimmed,
    true,
  );
}

Outcome taskJournal(
  String? root,
  String data,
  String name,
  String words,
  String? workflows,
) {
  final task = reopen(data, name, root, workflows);
  if (!task.exists) {
    return Outcome.failed(['没有这件任务：${short(data, task.file)}']);
  }
  if (words.trim().isEmpty) {
    return Outcome.failed([
      '日志要人来写：${short(data, task.artifact(journalKind))}',
    ]);
  }
  narrate(task, words);
  return Outcome(
    true,
    lines: [
      '日志记下一段：${short(data, task.artifact(journalKind))}',
      stateLine(task),
    ],
  );
}
