import 'executor.dart';
import 'table_result.dart';

/// 一条判据属于谁判。
class CriteriaCounts {
  const CriteriaCounts({this.rule = 0, this.agent = 0, this.human = 0});

  final int rule;
  final int agent;
  final int human;

  int get total => rule + agent + human;

  int of(Executor executor) => switch (executor) {
    Executor.rule => rule,
    Executor.agent => agent,
    Executor.human => human,
  };

  /// 命令行写的是 `3 rule / 1 agent / 0 human`。
  static CriteriaCounts parse(String text) {
    final counts = <String, int>{};
    for (final part in text.split('/')) {
      final bits = part.trim().split(RegExp(r'\s+'));
      if (bits.length == 2) {
        counts[bits[1]] = int.tryParse(bits[0]) ?? 0;
      }
    }
    return CriteriaCounts(
      rule: counts['rule'] ?? 0,
      agent: counts['agent'] ?? 0,
      human: counts['human'] ?? 0,
    );
  }

  @override
  String toString() => '$rule rule / $agent agent / $human human';
}

/// 工作流里的一步。
class WorkflowStep {
  const WorkflowStep({
    required this.name,
    required this.executor,
    required this.criteria,
  });

  final String name;
  final Executor executor;
  final CriteriaCounts criteria;
}

/// 一条定义：一串有序的步骤。
class WorkflowDetail {
  const WorkflowDetail({
    required this.name,
    required this.path,
    required this.steps,
  });

  final String name;
  final String path;
  final List<WorkflowStep> steps;

  CriteriaCounts get criteria => steps.fold(
    const CriteriaCounts(),
    (sum, step) => CriteriaCounts(
      rule: sum.rule + step.criteria.rule,
      agent: sum.agent + step.criteria.agent,
      human: sum.human + step.criteria.human,
    ),
  );

  String get summary => '${steps.length} 步 · ${criteria.total} 条判据';

  /// 从 `workflow <名字> --json` 的结果来：行是「步骤 / 谁执行 / 怎么算完」。
  factory WorkflowDetail.fromResult(TableResult result) {
    final head = result.lines.isEmpty ? '' : result.lines.first;
    final open = head.indexOf('（');
    final close = head.lastIndexOf('）');
    return WorkflowDetail(
      name: head.startsWith('工作流：')
          ? head.substring('工作流：'.length, open < 0 ? head.length : open)
          : '',
      path: open >= 0 && close > open ? head.substring(open + 1, close) : '',
      steps: result.rows
          .map(
            (row) => WorkflowStep(
              name: row.isNotEmpty ? row[0] : '',
              executor: Executor.parse(row.length > 1 ? row[1] : ''),
              criteria: CriteriaCounts.parse(row.length > 2 ? row[2] : ''),
            ),
          )
          .toList(growable: false),
    );
  }
}

/// 列表里的一条：名字、步骤串、位置。
class WorkflowSummary {
  const WorkflowSummary({
    required this.name,
    required this.steps,
    required this.path,
  });

  final String name;
  final String steps;
  final String path;

  factory WorkflowSummary.fromRow(List<String> row) => WorkflowSummary(
    name: row.isNotEmpty ? row[0] : '',
    steps: row.length > 1 ? row[1] : '',
    path: row.length > 2 ? row[2] : '',
  );
}
