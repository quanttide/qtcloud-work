/// 谁执行。
///
/// 步骤上有执行者（这一步谁做），判据上也有（这一步谁来判），是两回事。
enum Executor {
  agent('agent', 'AI 执行', 'AI'),
  rule('rule', '机器判', '机器'),
  human('human', '人', '人');

  const Executor(this.code, this.stepLabel, this.criterionLabel);

  final String code;

  /// 步骤上的说法。
  final String stepLabel;

  /// 判据上的说法。
  final String criterionLabel;

  static Executor parse(String code) => values.firstWhere(
    (value) => value.code == code,
    orElse: () => Executor.agent,
  );
}
