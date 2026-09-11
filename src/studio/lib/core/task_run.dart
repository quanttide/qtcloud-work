import 'audit.dart';
import 'definition.dart';
import 'host/host.dart';
import 'outcome.dart';
import 'task.dart';

/// 走一步：执行者是 AI 的交给 `pi` 跑，然后程序自己判机械判据、把闸门项记进任务文件，
/// 事实记进流水与报告。（与命令行 `src/task.rs` 的 execute 一条一条对齐）

/// 交给 AI 的那一段话：这一步做什么、判据是什么、产物落在哪。
String promptFor(Task task, Step step) {
  final steps = task.steps().map((item) => item.name).join('、');
  return '你在按一条工作流走一步。只做这一步，做完就停。\n\n'
      '工作区：${task.root}\n'
      '数据仓：${task.data}\n'
      '任务：${task.name}（开工：${task.start}）\n'
      '工作流：${task.workflowName}——${task.workflow().description}\n'
      '步骤：$steps\n'
      '这一步：${step.name}\n'
      '做什么：\n${step.description}\n\n'
      '判据（程序随后自己核对，你不能改判据、也不许改判据文件）：\n${criteriaText(step)}\n\n'
      '本任务的三样东西（报告与日志是产物，流水是执行痕迹）：\n'
      '  产物：${task.relative(task.artifact(reportKind))}'
      '（程序不碰产物内容，谁写谁定；闸门项记在任务文件里）\n'
      '  日志：${task.relative(task.artifact(journalKind))}\n'
      '  流水：${task.relative(task.artifact(logKind))}（就在任务文件里）\n'
      '工作流里用 {{report}} / {{journal}} / {{log}} 指这三样；'
      '工作内容写进报告，别动程序那两节。\n'
      '规矩：数据只写数据仓；工作区里只动「做什么」点名的东西。最后用一句话说明你做了什么。\n';
}

/// 判据清单：每条一行「谁判：说明」。
String criteriaText(Step step) {
  final lines = step.criteria.map((criterion) {
    final executor = '${criterion['executor'] ?? ''}';
    final described = descriptionOf(criterion);
    return '- $executor：$described';
  }).toList();
  return lines.isEmpty ? '（这一步没有判据）' : lines.join('\n');
}

/// 交给智能体审的那一段话：产物 + 判准，逐条回答。
String judgePrompt(Task task, Step step, List<Map> criteria) {
  final listed = <String>[];
  for (var i = 0; i < criteria.length; i++) {
    listed.add('${i + 1}. ${criteria[i]['description'] ?? ''}');
  }
  return '你是审查者，不是执行者。别改产物、别改判据文件。\n\n'
      '工作区：${task.root}\n'
      '要审的东西：这一步的产物在 ${task.artifactsDir}（也可以看工作区里相关文件）\n'
      '这一步做什么：${step.description}\n\n'
      '判准（逐条判）：\n${listed.join('\n')}\n\n'
      '对每条输出一行，格式只能是「序号. 通过 — 一句话理由」或「序号. 不通过 — 一句话理由」，'
      '最后不要写别的。\n';
}

String oneLine(String text, int limit) {
  final lines = text.split('\n').where((line) => line.trim().isNotEmpty).toList();
  final line = lines.isEmpty ? '' : lines.last.trim();
  return line.length > limit ? line.substring(0, limit) : line;
}

/// 从智能体的回答里读一条结论：先认「序号. …」那行，没有就整段兜底。
(String, String) verdictOf(String out, int index) {
  final prefix = '$index.';
  for (final line in out.split('\n')) {
    final stripped = line.trim();
    if (stripped.startsWith(prefix)) {
      final tail = stripped.substring(prefix.length).trim();
      final verdict = tail.startsWith('不通过')
          ? '✗'
          : tail.startsWith('通过')
          ? '✓'
          : '待判';
      return (verdict, tail);
    }
  }
  final flat = out.replaceAll(' ', '');
  if (flat.contains('不通过')) return ('✗', oneLine(out, 80));
  if (flat.contains('通过')) return ('✓', oneLine(out, 80));
  return ('待判', oneLine(out, 80));
}

/// 让智能体按判准审一遍；返回（说明，结论，理由），并记一笔 `·审` 流水。
List<(String, String, String)> judgeByAi(Task task, Step step, List<Map> criteria) {
  final run = runPi(judgePrompt(task, step, criteria), task.root);
  final rows = <(String, String, String)>[];
  for (var i = 0; i < criteria.length; i++) {
    final note = '${criteria[i]['description'] ?? ''}'.trim();
    if (!run.ran) {
      rows.add((note, '待判', '智能体没跑成：${oneLine(run.out, 80)}'));
      continue;
    }
    final (verdict, reason) = verdictOf(run.out, i + 1);
    rows.add((note, verdict, reason));
  }
  final allPass = rows.every((row) => row.$2 == '✓');
  final detail =
      'AI 审查（同一模型）：${rows.map((row) => '${row.$1}→${row.$2}').join('；')}';
  task.record('${step.name}·审', detail, allPass);
  return rows;
}

/// 把 `{{report}}` / `{{journal}}` / `{{log}}` / `{{artifacts}}` 换成本次任务的产物路径。
String expand(Task task, String value) {
  final out = StringBuffer();
  var rest = value;
  while (true) {
    final at = rest.indexOf('{{');
    if (at < 0) {
      out.write(rest);
      break;
    }
    out.write(rest.substring(0, at));
    final after = rest.substring(at + 2);
    final end = after.indexOf('}}');
    if (end < 0) {
      out.write('{{');
      rest = after;
      continue;
    }
    final kind = after.substring(0, end);
    final path = switch (kind) {
      'report' => task.artifact(reportKind),
      'journal' => task.artifact(journalKind),
      'log' => task.artifact(logKind),
      'artifacts' => task.artifactsDir,
      _ => null,
    };
    if (path == null) {
      out.write('{{$kind}}');
    } else {
      out.write(relativeToRoot(task, path));
    }
    rest = after.substring(end + 2);
  }
  return out.toString();
}

/// 判据按工作区根解析，占位也给工作区根视角的路径。
String relativeToRoot(Task task, String path) => short(task.root, path);

/// 判据里的占位先换成本次任务的真实路径，再去跑。
List<Map> expandedCriteria(Task task, List<Map> criteria) {
  return criteria.map((criterion) {
    final out = <String, Object?>{};
    criterion.forEach((key, value) {
      if (value is String && value.contains('{{')) {
        out['$key'] = expand(task, value);
      } else {
        out['$key'] = value;
      }
    });
    return out;
  }).toList();
}

/// 走一步的三种结果：过没过、给人看的几句、给窗口画的行。
class StepResult {
  StepResult(this.ok, this.lines, this.rows);

  final bool ok;
  final List<String> lines;
  final List<List<String>> rows;
}

StepResult execute(Task task, String root, String step, String note, bool auto) {
  final found = task.workflow().step(step);
  if (found == null) {
    return StepResult(false, ['工作流里没有这一步：$step'], []);
  }
  final lines = <String>[];
  if (auto && !found.isHuman) {
    lines.add('${found.name}：交给 AI（${found.executor}）跑');
    final run = runPi(promptFor(task, found), root);
    final one = run.out.isEmpty ? '（没输出）' : oneLine(run.out, 80);
    lines.add('  AI ${run.ran ? '跑完了' : '跑不动'}：$one');
    task.record(found.name, 'AI 执行：$one', run.ran);
    if (!run.ran) {
      writeGates(task, const []);
      lines.add('  （AI 没跑成，这一步不算过；修好再来）');
      return StepResult(false, lines, []);
    }
  } else if (found.isHuman && auto) {
    lines.add(
      '${found.name}：这一步的执行者是人（${found.executor}）——轮到你，'
      '做完用 task <名字> --done ${found.name}',
    );
    return StepResult(true, lines, []);
  }

  final (results, _) = runRules(root, itemsOf(expandedCriteria(task, found.rules)));
  final agents = found.agents;
  final judged = (auto && agents.isNotEmpty)
      ? judgeByAi(task, found, expandedCriteria(task, agents))
      : agents
            .map(
              (criterion) => (
                '${criterion['description'] ?? ''}'.trim(),
                '待判',
                '没跑智能体（人为地记一步）',
              ),
            )
            .toList();
  final gates = found.gates
      .map((criterion) => '${criterion['description'] ?? ''}'.trim())
      .toList();

  final rulesPass = results.every((row) => row.$2);
  final judgedPass = judged.every((row) => row.$2 == '✓' || row.$2 == '待判');
  final ok = rulesPass && judgedPass;

  final detail = note.trim().isNotEmpty
      ? note.trim()
      : results.isNotEmpty
      ? results.map((row) => row.$1.description).join('；')
      : '做完';

  if (!(auto && !found.isHuman)) {
    task.record(step, detail, ok);
  } else if (found.rules.isNotEmpty) {
    // 交给 AI 跑的步骤：机器判据这一半单独记一笔（审查那条只知道 agent 判据）。
    task.record('${found.name}·判', detail, rulesPass);
  }

  final gateLines = <String>[...gates];
  gateLines.addAll(
    judged.where((row) => row.$2 != '✓').map((row) => row.$1),
  );
  writeGates(task, gateLines);

  lines.add('${ok ? '✓' : '✗'} $step：$detail');
  lines.addAll(results.map((row) => '  ${row.$2 ? '✓' : '✗'} ${row.$1.description}（${row.$3}）'));
  lines.addAll(judged.map((row) => '  ${row.$2} ${row.$1}（${row.$3}）'));
  lines.addAll(gates.map((note) => '  ⧗ $note（留给人）'));

  final rows = <List<String>>[];
  rows.addAll(
    results.map((row) => [row.$1.description, row.$2 ? '✓' : '✗', row.$3]),
  );
  rows.addAll(judged.map((row) => [row.$1, row.$2, row.$3]));
  rows.addAll(gates.map((note) => [note, '闸门', '留给人拍板']));

  return StepResult(ok, lines, rows);
}

/// 闸门项是任务的状态，记进任务文件；产物一个字都不碰。
void writeGates(Task task, List<String> gates) {
  final notes = task.gates();
  for (final note in gates) {
    if (!notes.contains(note)) notes.add(note);
  }
  task.setGates(notes);
}

/// 走一步（`--next` 与 `--done` 都走这里）。
Outcome taskStep(
  String? root,
  String data,
  String name,
  String step,
  String note,
  bool auto,
  String? workflows,
) {
  final task = reopen(data, name, root, workflows);
  if (!task.exists) {
    return Outcome.failed(['没有这件任务：${short(data, task.file)}']);
  }
  var chosen = step.trim();
  if (chosen.isEmpty) {
    if (!auto) {
      return Outcome.failed(['请给步骤名（task <名字> 看有哪些步骤）']);
    }
    final next = task.nextStep();
    if (next == null) return Outcome(true, lines: ['所有步骤都走过了']);
    chosen = next.name;
  }
  final result = execute(task, task.root, chosen, note, auto);
  final outcome = Outcome(result.ok, lines: result.lines)
    ..columns = ['核对', '结论', '说明']
    ..rows = result.rows;
  outcome.lines.add(stateLine(task));
  return outcome;
}
