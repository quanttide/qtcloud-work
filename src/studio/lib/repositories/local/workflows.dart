import 'fs/fs.dart';
import 'outcome.dart';
import 'yaml.dart';

/// 工作流定义：一串有序的步骤。
///
/// 定义这一类**领域模型**（字段表、校验、步骤与判据的视图、定义核对）都在工具箱
/// `quanttide_work` 里——两侧共用一份规矩。这一层只剩工作室自己的两件事：
/// 文件读写（`<工作流目录>/<名字>.yaml`）与把动作写成信封。
export 'package:quanttide_work/quanttide_work.dart'
    show
        Step,
        Workflow,
        Finding,
        DefinitionError,
        agent,
        human,
        rule,
        executors,
        criterionTypes,
        criterionOf,
        readCriterion,
        textOf,
        unknownFields,
        looksLikeSection,
        expandPlaceholders;

import 'package:quanttide_work/quanttide_work.dart';

/// 读一份定义：文件读进来、YAML 解开、照 schema 验一遍。
///
/// 校验的规矩在工具箱里——报错文字与命令行一字不差。
Object loadDefinition(String path) {
  final file = path.split('/').last;
  final String text;
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
  Workflow.fromValue(payload, file: file);
  return payload!;
}

/// 工作流目录：默认跟在数据仓里，可另指一处固定资产目录。
String workflowsDir(String data, [String? workflows]) =>
    workflows ?? '$data/workflows';

/// 一条定义**连同它的文件位置**（工具箱那份只管内容，不管文件）。
class WorkflowFile {
  WorkflowFile({
    required this.name,
    required this.payload,
    required this.workflows,
  });

  final String name;
  Map payload;
  final String workflows;

  String get file => '$workflows/$name.yaml';

  bool get exists => fileExists(file);

  WorkflowFile reload() {
    if (exists) {
      try {
        payload = loadDefinition(file) as Map;
      } catch (_) {
        // 读不了就保持现状，交给调用方按 exists 判断
      }
    }
    return this;
  }

  /// 内容那一层交给工具箱。
  Workflow get shared => Workflow.of(name, payload);

  String get description => shared.description;

  /// 步骤：按定义里的顺序——这就是「串联」。
  List<Step> get steps => shared.steps;

  Step? step(String name) => shared.step(name);

  String toYaml() => dumpYaml(payload);
}

/// 写一条工作流：步骤串联，每步给一份判据骨架（执行者默认 AI）。
WorkflowFile createWorkflow(
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
  final flow = WorkflowFile(
    name: name,
    payload: payload,
    workflows: workflowsDir(data, workflows),
  );
  writeText(flow.file, flow.toYaml());
  return flow;
}

WorkflowFile openWorkflow(String data, String name, [String? workflows]) =>
    WorkflowFile(
      name: name,
      payload: const {},
      workflows: workflowsDir(data, workflows),
    ).reload();

/// 把一条工作流存成一份可带走的文件（原样，不改内容）。
String exportWorkflow(WorkflowFile flow, String target) {
  final path = dirExists(target) ? '$target/${flow.name}.yaml' : target;
  writeText(path, flow.toYaml());
  return path;
}

/// 把一份工作流导进来：先照 schema 验一遍，再起个名字落进 workflows/。
WorkflowFile importWorkflow(
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
  var flow = WorkflowFile(
    name: chosen,
    payload: payload,
    workflows: workflowsDir(data, workflows),
  );
  if (flow.exists) {
    throw DefinitionError('已经有一条工作流叫「$chosen」：${flow.file}（换名字用 --as）');
  }
  payload['name'] = chosen;
  flow = WorkflowFile(
    name: chosen,
    payload: payload,
    workflows: workflowsDir(data, workflows),
  );
  writeText(flow.file, flow.toYaml());
  return flow;
}

List<WorkflowFile> listingWorkflows(String data, [String? workflows]) {
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
  outcomeData[result] = {
    'payload': flow.payload,
    'path': short(data, flow.file),
    'yaml': flow.exists ? readText(flow.file) : '',
  };
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
  final found = flow.shared.check(data, (written) {
    final target = written.startsWith('/') ? written : '$root/$written';
    return fileExists(target) || dirExists(target);
  });
  return Outcome(allOk(found))
    ..lines = [
      '工作流：${flow.file.split('/').last.replaceAll(RegExp(r'\.yaml$'), '')}',
      ...describeFindings(found),
    ];
}

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
