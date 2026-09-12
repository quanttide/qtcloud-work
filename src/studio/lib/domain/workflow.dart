import 'executor.dart';

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

  @override
  String toString() => '$rule rule / $agent agent / $human human';
}

/// 一条判据：谁判 + 那句话。
class Criterion {
  const Criterion({required this.executor, required this.text});

  final Executor executor;
  final String text;
}

/// 工作流里的一步。
class WorkflowStep {
  const WorkflowStep({
    required this.name,
    required this.executor,
    required this.criteria,
    this.items = const [],
  });

  final String name;
  final Executor executor;
  final CriteriaCounts criteria;

  /// 逐条判据的文字（定义里怎么写的就怎么列）。
  final List<Criterion> items;
}

/// 一条定义：一串有序的步骤。
class WorkflowDetail {
  const WorkflowDetail({
    required this.name,
    required this.path,
    required this.steps,
    this.yaml = '',
  });

  final String name;
  final String path;
  final List<WorkflowStep> steps;

  /// 定义文件的原文。
  final String yaml;

  CriteriaCounts get criteria => steps.fold(
    const CriteriaCounts(),
    (sum, step) => CriteriaCounts(
      rule: sum.rule + step.criteria.rule,
      agent: sum.agent + step.criteria.agent,
      human: sum.human + step.criteria.human,
    ),
  );

  String get summary => '${steps.length} 步 · ${criteria.total} 条判据';

  /// 从命令行交出来的 `data` 来（界面不读给人看的那些话）。
  factory WorkflowDetail.fromData(Map<String, Object?> data) {
    return WorkflowDetail(
      name: '${data['name'] ?? ''}',
      path: '${data['path'] ?? ''}',
      yaml: '${data['yaml'] ?? ''}',
      steps: [
        for (final step in (data['steps'] as List? ?? const []).cast<Map>())
          WorkflowStep(
            name: '${step['name'] ?? ''}',
            executor: Executor.parse('${step['executor'] ?? ''}'),
            criteria: CriteriaCounts(
              rule: (step['rule'] as num?)?.toInt() ?? 0,
              agent: (step['agent'] as num?)?.toInt() ?? 0,
              human: (step['human'] as num?)?.toInt() ?? 0,
            ),
            items: [
              for (final item
                  in (step['criteria'] as List? ?? const []).cast<Map>())
                Criterion(
                  executor: Executor.parse('${item['executor'] ?? ''}'),
                  text: '${item['text'] ?? ''}',
                ),
            ],
          ),
      ],
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
