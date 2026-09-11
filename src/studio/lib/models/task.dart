import 'table_result.dart';

/// 一条流水的来路。步骤名后面那截决定它是谁记的。
enum JournalKind {
  /// 执行者走了一遍这一步。
  run('', '执行'),

  /// AI 按 `agent` 判据审了这一步。
  agentReview('·审', 'AI 审'),

  /// 机器按 `rule` 判据判了这一步。
  machineJudge('·判', '机器判');

  const JournalKind(this.suffix, this.label);

  final String suffix;
  final String label;

  static JournalKind of(String step) => values.firstWhere(
    (kind) => kind.suffix.isNotEmpty && step.endsWith(kind.suffix),
    orElse: () => JournalKind.run,
  );
}

/// 流水里的一条。
class JournalEntry {
  const JournalEntry({
    required this.at,
    required this.step,
    required this.detail,
  });

  final String at;
  final String step;
  final String detail;

  /// 去掉 `·审` / `·判` 之后的步骤名。
  String get baseStep {
    final kind = JournalKind.of(step);
    return kind.suffix.isEmpty
        ? step
        : step.substring(0, step.length - kind.suffix.length);
  }

  JournalKind get kind => JournalKind.of(step);

  /// 命令行印的是 `时间　步骤　一句话`（全角空格分隔）。
  factory JournalEntry.fromLine(String line) {
    final parts = line.split('\u3000');
    return JournalEntry(
      at: parts.isNotEmpty ? parts[0].trim() : '',
      step: parts.length > 1 ? parts[1].trim() : '',
      detail: parts.length > 2 ? parts.sublist(2).join('\u3000').trim() : '',
    );
  }
}

/// 一个步骤在这次任务里的状态。
class TaskStep {
  const TaskStep({required this.name, required this.done});

  final String name;
  final bool done;

  /// 命令行用 `✓` 标走过，其余都算没走。
  factory TaskStep.fromRow(List<String> row) => TaskStep(
    name: row.isNotEmpty ? row[0] : '',
    done: row.length > 1 && row[1] == '✓',
  );
}

/// 一件任务的三个落点。
class TaskProducts {
  const TaskProducts({
    required this.report,
    required this.journal,
    required this.log,
  });

  final String report;
  final String journal;
  final String log;

  static const empty = TaskProducts(report: '', journal: '', log: '');

  /// 命令行印的是 `产物：A、B　流水：C`。
  factory TaskProducts.parse(String? text) {
    if (text == null || text.isEmpty) return empty;
    final halves = text.split('\u3000');
    final left = halves.first.startsWith('产物：')
        ? halves.first.substring('产物：'.length)
        : halves.first;
    final files = left.split('、').map((e) => e.trim()).toList();
    final log = halves.length > 1 && halves[1].startsWith('流水：')
        ? halves[1].substring('流水：'.length).trim()
        : '';
    return TaskProducts(
      report: files.isNotEmpty ? files[0] : '',
      journal: files.length > 1 ? files[1] : '',
      log: log,
    );
  }
}

/// 一件任务：工作流的一次执行实例。
class TaskDetail {
  const TaskDetail({
    required this.name,
    required this.start,
    required this.workflowName,
    required this.workflowDescription,
    required this.steps,
    required this.stateLine,
    required this.products,
    required this.journal,
  });

  final String name;
  final String start;
  final String workflowName;
  final String workflowDescription;
  final List<TaskStep> steps;

  /// 命令行给的那一句：「5 个步骤都走过了」或「下一步：audit」。
  final String stateLine;
  final TaskProducts products;
  final List<JournalEntry> journal;

  int get doneCount => steps.where((step) => step.done).length;

  double get progress => steps.isEmpty ? 0 : doneCount / steps.length;

  /// 第一个没走到的步骤。
  String? get currentStep {
    for (final step in steps) {
      if (!step.done) return step.name;
    }
    return null;
  }

  bool get finished => steps.isNotEmpty && doneCount == steps.length;

  factory TaskDetail.fromResult(TableResult result) {
    final data = result.data;
    final products =
        (data['products'] as Map?)?.cast<String, Object?>() ?? const {};
    return TaskDetail(
      name: '${data['name'] ?? ''}',
      start: '${data['start'] ?? ''}',
      workflowName: '${data['workflow'] ?? ''}',
      workflowDescription: '${data['description'] ?? ''}',
      steps: [
        for (final step in (data['steps'] as List? ?? const []).cast<Map>())
          TaskStep(name: '${step['name'] ?? ''}', done: step['done'] == true),
      ],
      stateLine: '${data['state'] ?? ''}',
      products: TaskProducts(
        report: '${products['report'] ?? ''}',
        journal: '${products['journal'] ?? ''}',
        log: '${products['log'] ?? ''}',
      ),
      journal: [
        for (final entry in (data['journal'] as List? ?? const []).cast<Map>())
          JournalEntry(
            at: '${entry['at'] ?? ''}',
            step: '${entry['step'] ?? ''}',
            detail: '${entry['detail'] ?? ''}',
          ),
      ],
    );
  }
}

/// 列表里的一条任务。
class TaskSummary {
  const TaskSummary({
    required this.name,
    required this.workflow,
    required this.next,
  });

  final String name;
  final String workflow;
  final String next;

  factory TaskSummary.fromRow(List<String> row) => TaskSummary(
    name: row.isNotEmpty ? row[0] : '',
    workflow: row.length > 1 ? row[1] : '',
    next: row.length > 2 ? row[2] : '',
  );
}
