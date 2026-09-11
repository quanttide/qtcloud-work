import 'fs/fs.dart';
import 'outcome.dart';
import 'yaml.dart';

/// 工作流：串联的工作步骤——过程的编排定义，用 YAML 存。
///
/// 定义要有固定的意义，所以字段名、取值、判据种类都由 schema 定死，
/// 不认识的字段直接报错。（与 `qtcloud-work` 的定义层同一套规矩）
const String agent = 'agent';
const String human = 'human';
const String rule = 'rule';
const List<String> executors = [agent, human];
const List<String> criterionTypes = [rule, agent, human];
const List<String> topFields = ['name', 'description', 'steps'];
const List<String> stepFields = ['name', 'description', 'executor', 'criteria'];
const List<String> criterionFields = [
  'executor',
  'description',
  'path',
  'absent',
  'file',
  'contains',
  'run',
];

/// 这份文件不像一份工作流。
class DefinitionError implements Exception {
  DefinitionError(this.message);

  final String message;

  @override
  String toString() => message;
}

String textOf(Object? value, String key) {
  if (value is! Map) return '';
  final item = value[key];
  return item is String ? item.trim() : '';
}

List<String> unknownFields(
  Map<Object?, Object?> mapping,
  List<String> allowed,
) => mapping.keys
    .map((key) => '$key')
    .where((key) => !allowed.contains(key))
    .toList();

/// 读一份定义：不是映射、缺字段、取值不对，当场报错。
Object loadDefinition(String path) {
  final file = path.split('/').last;
  String text;
  try {
    text = readText(path);
  } catch (error) {
    throw DefinitionError('$file 读不了：$error');
  }
  Object? payload;
  try {
    payload = parseYaml(text);
  } catch (error) {
    throw DefinitionError('$file 不是合法的 YAML：$error');
  }
  if (payload is! Map) {
    throw DefinitionError('$file 的顶层不是映射（name / steps）');
  }
  final top = payload.cast<Object?, Object?>();
  if (textOf(payload, 'name').isEmpty) {
    throw DefinitionError('$file 少了 name');
  }
  final steps = payload['steps'];
  if (steps is! List || steps.isEmpty) {
    throw DefinitionError('$file 少了 steps（至少一个步骤）');
  }
  final unknown = unknownFields(top, topFields);
  if (unknown.isNotEmpty) {
    throw DefinitionError(
      '$file 顶层有不认识的字段：${unknown.join('、')}（只认 ${topFields.join('、')}）',
    );
  }
  for (var index = 0; index < steps.length; index++) {
    final position = index + 1;
    final step = steps[index];
    if (step is! Map) {
      throw DefinitionError('$file 第 $position 个步骤少了 name');
    }
    if (textOf(step, 'name').isEmpty) {
      throw DefinitionError('$file 第 $position 个步骤少了 name');
    }
    final extra = unknownFields(step.cast<Object?, Object?>(), stepFields);
    if (extra.isNotEmpty) {
      throw DefinitionError(
        '$file 第 $position 个步骤有不认识的字段：${extra.join('、')}（只认 ${stepFields.join('、')}）',
      );
    }
    var executor = textOf(step, 'executor');
    if (executor.isEmpty) executor = agent;
    if (!executors.contains(executor)) {
      throw DefinitionError(
        '$file 第 $position 个步骤的 executor 只能是 ${executors.join(' 或 ')}，实得 $executor',
      );
    }
    final rawCriteria = step['criteria'];
    final List criteria;
    if (rawCriteria == null) {
      criteria = const [];
    } else if (rawCriteria is List) {
      criteria = rawCriteria;
    } else {
      throw DefinitionError('$file 第 $position 个步骤的 criteria 应当是列表');
    }
    for (var order = 0; order < criteria.length; order++) {
      final place = '第 $position 个步骤第 ${order + 1} 条判据';
      final criterion = criteria[order];
      final kind = textOf(criterion, 'executor');
      if (!criterionTypes.contains(kind)) {
        throw DefinitionError(
          '$file $place的 executor 只能是 ${criterionTypes.join(' / ')}（谁判：规则引擎 / 智能体 / 人）',
        );
      }
      if (criterion is! Map) {
        throw DefinitionError('$file $place不是映射');
      }
      final odd = unknownFields(
        criterion.cast<Object?, Object?>(),
        criterionFields,
      );
      if (odd.isNotEmpty) {
        throw DefinitionError(
          '$file $place有不认识的字段：${odd.join('、')}（只认 ${criterionFields.join('、')}）',
        );
      }
      final given = [
        'path',
        'absent',
        'file',
        'contains',
        'run',
      ].where((name) => criterion[name] != null).toList();
      if (kind == rule) {
        if (given.isEmpty) {
          throw DefinitionError(
            '$file $place是 rule，得写一条判法（path / absent / file+contains / run）',
          );
        }
        if (given.contains('contains') && !given.contains('file')) {
          throw DefinitionError('$file $place写了 contains，还得写 file');
        }
        if (given.contains('file') && !given.contains('contains')) {
          throw DefinitionError('$file $place写了 file，还得写 contains');
        }
        final others = given
            .where((name) => name != 'file' && name != 'contains')
            .toList();
        if (others.length > 1 ||
            (others.isNotEmpty && given.contains('file'))) {
          throw DefinitionError(
            '$file $place的判法只能一种：path / absent / file+contains / run',
          );
        }
      } else {
        if (textOf(criterion, 'description').isEmpty) {
          throw DefinitionError(
            '$file $place是 $kind，必须写 description（判准 / 要人拍板的事）',
          );
        }
        if (given.isNotEmpty) {
          throw DefinitionError(
            '$file $place是 $kind，不该带 ${given.join('、')}（那是 rule 的字段）',
          );
        }
      }
    }
  }
  return payload;
}

/// 一个工作步骤：叫什么、做什么、谁执行、怎么算完。
class Step {
  Step(this.payload);

  final Map payload;

  String get name => textOf(payload, 'name');

  String get description => textOf(payload, 'description');

  String get executor {
    final value = textOf(payload, 'executor');
    return value.isEmpty ? agent : value;
  }

  bool get isHuman => executor == human;

  List<Map> get criteria =>
      (payload['criteria'] as List?)?.cast<Map>().toList() ?? const [];

  List<Map> of(String kind) =>
      criteria.where((item) => textOf(item, 'executor') == kind).toList();

  List<Map> get rules => of(rule);
  List<Map> get agents => of(agent);
  List<Map> get gates => of(human);

  Map<String, int> get counts => {
    rule: rules.length,
    agent: agents.length,
    human: gates.length,
  };
}

/// 过程的编排定义：一串步骤。
class Workflow {
  Workflow({
    required this.name,
    required this.payload,
    required this.workflows,
  });

  final String name;
  Map payload;
  final String workflows;

  String get file => '$workflows/$name.yaml';

  bool get exists => fileExists(file);

  Workflow reload() {
    if (exists) {
      try {
        payload = loadDefinition(file) as Map;
      } catch (_) {
        // 读不了就保持现状，交给调用方按 exists 判断
      }
    }
    return this;
  }

  String get description => textOf(payload, 'description');

  List<Step> get steps =>
      (payload['steps'] as List?)?.map((item) => Step(item as Map)).toList() ??
      const [];

  Step? step(String name) {
    for (final item in steps) {
      if (item.name == name) return item;
    }
    return null;
  }

  String toYaml() => dumpYaml(payload);
}

/// 工作流目录：默认跟在数据仓里，可另指一处固定资产目录。
String workflowsDir(String data, [String? workflows]) =>
    workflows ?? '$data/workflows';

/// 写一条工作流：步骤串联，每步给一份判据骨架（执行者默认 AI）。
Workflow createWorkflow(
  String data,
  String name,
  List<String> steps,
  String note, [
  String? workflows,
]) {
  final payload = <String, Object?>{
    'name': name,
    'description': note.trim().isEmpty ? '步骤串联：写清每步做什么、谁执行、怎么判。' : note.trim(),
    'steps': [
      for (final step in steps)
        {
          'name': step,
          'description': '<$step这一步做什么>',
          'executor': agent,
          'criteria': [
            {'executor': rule, 'path': 'data/journal/README.md'},
            {'executor': human, 'description': '<只能人拍板的>'},
          ],
        },
    ],
  };
  final flow = Workflow(
    name: name,
    payload: payload,
    workflows: workflowsDir(data, workflows),
  );
  writeText(flow.file, flow.toYaml());
  return flow;
}

Workflow openWorkflow(String data, String name, [String? workflows]) =>
    Workflow(
      name: name,
      payload: const {},
      workflows: workflowsDir(data, workflows),
    ).reload();

/// 把一条工作流存成一份可带走的文件（原样，不改内容）。
String exportWorkflow(Workflow flow, String target) {
  final path = dirExists(target) ? '$target/${flow.name}.yaml' : target;
  writeText(path, flow.toYaml());
  return path;
}

/// 把一份工作流导进来：先照 schema 验一遍，再起个名字落进 workflows/。
Workflow importWorkflow(
  String data,
  String source,
  String name, [
  String? workflows,
]) {
  final payload = loadDefinition(source) as Map;
  var chosen = name.trim();
  if (chosen.isEmpty) {
    final fromPayload = textOf(payload, 'name');
    final stem = source.split('/').last.replaceAll(RegExp(r'\.yaml$'), '');
    chosen = fromPayload.isEmpty ? stem : fromPayload;
  }
  var flow = Workflow(
    name: chosen,
    payload: payload,
    workflows: workflowsDir(data, workflows),
  );
  if (flow.exists) {
    throw DefinitionError('已经有一条工作流叫「$chosen」：${flow.file}（换名字用 --as）');
  }
  payload['name'] = chosen;
  flow = Workflow(
    name: chosen,
    payload: payload,
    workflows: workflowsDir(data, workflows),
  );
  writeText(flow.file, flow.toYaml());
  return flow;
}

List<Workflow> listingWorkflows(String data, [String? workflows]) {
  final base = workflowsDir(data, workflows);
  if (!dirExists(base)) return const [];
  final paths = listDir(base).where((path) => path.endsWith('.yaml')).toList()
    ..sort();
  return [
    for (final path in paths)
      openWorkflow(
        data,
        path.split('/').last.replaceAll(RegExp(r'\.yaml$'), ''),
        workflows,
      ),
  ];
}

// ---- 动作 ----

Outcome workflowNew(
  String data,
  String name,
  List<String> steps,
  String note, [
  String? workflows,
]) {
  if (name.trim().isEmpty) return Outcome.failed(['请先给工作流起个名字']);
  if (steps.isEmpty) return Outcome.failed(['至少给一个步骤：--steps 甲,乙,丙']);
  final flow = createWorkflow(data, name.trim(), steps, note, workflows);
  return workflowShow(
    data,
    name.trim(),
    workflows,
  ).withFirst('写下工作流：${short(data, flow.file)}');
}

Outcome workflowShow(String data, String name, [String? workflows]) {
  final flow = openWorkflow(data, name, workflows);
  if (!flow.exists) {
    return Outcome.failed(['没有这条工作流：${short(data, flow.file)}']);
  }
  final result = Outcome(true)
    ..columns = ['步骤', '谁执行', '怎么算完']
    ..lines.add('工作流：${flow.name}（${short(data, flow.file)}）');
  for (final step in flow.steps) {
    final counts =
        '${step.rules.length} rule / ${step.agents.length} agent / ${step.gates.length} human';
    result.rows.add([step.name, step.executor, counts]);
    result.lines.add('  ${step.name}：${step.executor}　$counts');
  }
  return result;
}

Outcome workflowExport(
  String data,
  String name,
  String target, [
  String? workflows,
]) {
  final flow = openWorkflow(data, name, workflows);
  if (!flow.exists) {
    return Outcome.failed(['没有这条工作流：${short(data, flow.file)}']);
  }
  final saved = exportWorkflow(flow, target);
  return workflowShow(
    data,
    name,
    workflows,
  ).withFirst('已导出：$saved（步骤 ${flow.steps.length} 个，原样带走）');
}

Outcome workflowImport(
  String data,
  String source,
  String name, [
  String? workflows,
]) {
  if (!fileExists(source)) {
    return Outcome.failed(['没有这份文件：$source']);
  }
  try {
    final flow = importWorkflow(data, source, name, workflows);
    return workflowShow(
      data,
      flow.name,
      workflows,
    ).withFirst('已导入：${short(data, flow.file)}（步骤 ${flow.steps.length} 个）');
  } on DefinitionError catch (error) {
    return Outcome.failed([error.message]);
  }
}

Outcome workflowList(String data, [String? workflows]) {
  final found = listingWorkflows(data, workflows);
  final result = Outcome(true)..columns = ['工作流', '步骤', '位置'];
  for (final flow in found) {
    final steps = flow.steps.map((step) => step.name).join('、');
    result.rows.add([flow.name, steps, short(data, flow.file)]);
    result.lines.add('${flow.name.padRight(24)} 步骤：$steps');
  }
  if (found.isEmpty) {
    result.lines = ['还没有工作流：workflow --new <名字> --steps 甲,乙'];
  }
  return result;
}

// ---- 定义核对：声明与判据对不对得上 ----

/// 一条定义核对出来的一件事。
class Finding {
  Finding({required this.where, required this.what, required this.ok});

  final String where;
  final String what;
  final bool ok;
}

/// 核对一条工作流：
///
/// 一、判据里写到的路径（`path` / `file`，`{{…}}` 先按数据仓展开）在不在；
/// 二、description 里提到的报告小节有没有判据覆盖（至少一条 `contains` 写它）。
List<Finding> checkWorkflow(Workflow flow, String root, String data) {
  final found = <Finding>[];
  for (final step in flow.steps) {
    for (final criterion in step.rules) {
      final literal = (criterion['path'] ?? criterion['file']);
      if (literal is! String || literal.isEmpty) continue;
      // 按任务落点的占位在定义这一层核不了，跳过。
      if (literal.contains('{{report}}') ||
          literal.contains('{{journal}}') ||
          literal.contains('{{log}}')) {
        continue;
      }
      final written = expandPlaceholders(literal, data);
      final target = written.startsWith('/') ? written : '$root/$written';
      found.add(
        Finding(
          where: '${step.name}·$literal',
          what: '判据里的路径在不在：$written',
          ok: fileExists(target) || dirExists(target),
        ),
      );
    }
  }

  final covered = flow.steps
      .expand((step) => step.rules)
      .map((criterion) => criterion['contains'])
      .whereType<String>()
      .toList();

  final mentioned = <String>[];
  for (final step in flow.steps) {
    final text = step.description;
    // 两种写法都认：`## 名字` 与 「名字」一节 / 「名字」节
    final pieces = text.split('## ').skip(1);
    for (final piece in pieces) {
      final name = piece.split(RegExp(r'[\s`」]')).first.trim();
      if (looksLikeSection(name) && !mentioned.contains(name)) {
        mentioned.add(name);
      }
    }
    var rest = text;
    while (true) {
      final at = rest.indexOf('「');
      if (at < 0) break;
      final after = rest.substring(at + 1);
      final end = after.indexOf('」');
      if (end < 0) break;
      final name = after.substring(0, end).trim();
      final tail = after.substring(end + 1).trimLeft();
      final isSection =
          tail.startsWith('一节') ||
          tail.startsWith('节') ||
          tail.startsWith('两节');
      if (isSection && looksLikeSection(name) && !mentioned.contains(name)) {
        mentioned.add(name);
      }
      rest = after.substring(end + 1);
    }
  }
  for (final name in mentioned) {
    found.add(
      Finding(
        where: 'description',
        what: 'description 提到的报告小节有没有判据覆盖：$name',
        ok: covered.any((value) => value.contains(name)),
      ),
    );
  }
  return found;
}

/// 像不像报告小节的名字：中文短词。版本号写法、占位、路径都不算。
bool looksLikeSection(String name) {
  if (name.isEmpty || name.length > 12) return false;
  final banned = RegExp(r'[0-9\[\]\{\}\./`<>\-_]');
  return !banned.hasMatch(name);
}

/// 判据里的占位先按数据仓展开。
String expandPlaceholders(String value, String data) => value
    .replaceAll('{{artifacts}}', '$data/artifacts')
    .replaceAll('{{report}}', '$data/artifacts/report')
    .replaceAll('{{journal}}', '$data/artifacts/journal')
    .replaceAll('{{log}}', '$data/tasks');

/// 核对结果写成人读的一段。
List<String> describeFindings(List<Finding> found) {
  final lines = <String>['核对 ${found.length} 件事'];
  for (final item in found) {
    lines.add('  ${item.ok ? '✓' : '✗'} ${item.where}——${item.what}');
  }
  if (found.isEmpty) {
    lines.add('  （这条定义里没有可核对的路径与小节）');
  }
  return lines;
}

bool allOk(List<Finding> found) => found.every((item) => item.ok);

Outcome workflowCheck(
  String data,
  String name,
  String root, [
  String? workflows,
]) {
  final flow = openWorkflow(data, name, workflows);
  if (!flow.exists) {
    return Outcome.failed(['没有这条工作流：${short(data, flow.file)}']);
  }
  final found = checkWorkflow(flow, root, data);
  final result = Outcome(allOk(found))
    ..lines = [
      '工作流：${flow.file.split('/').last.replaceAll(RegExp(r'\.yaml$'), '')}',
      ...describeFindings(found),
    ];
  return result;
}
