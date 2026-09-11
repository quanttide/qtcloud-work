import 'package:flutter/material.dart';

import '../models/executor.dart';
import '../models/workflow.dart';

/// 判据面板：点一个步骤，看它谁做、判据有几条。（见 doc/views/criteria-panel.md）
class CriteriaPanel extends StatelessWidget {
  const CriteriaPanel({super.key, this.step});

  final WorkflowStep? step;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final current = step;
    if (current == null) {
      return Padding(
        padding: const EdgeInsets.all(16),
        child: Text('点一个步骤，看它谁做、判据有几条', style: theme.textTheme.bodySmall),
      );
    }
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Text(current.name, style: theme.textTheme.titleSmall),
              const SizedBox(width: 8),
              Text(current.executor.stepLabel, style: theme.textTheme.bodySmall),
              const Spacer(),
              Text(
                '${current.criteria.total} 条判据（机器判 ${current.criteria.rule}）',
                style: theme.textTheme.bodySmall,
              ),
            ],
          ),
          const SizedBox(height: 8),
          for (final entry in [
            (Executor.rule, current.criteria.rule),
            (Executor.agent, current.criteria.agent),
            (Executor.human, current.criteria.human),
          ])
            if (entry.$2 > 0)
              Padding(
                padding: const EdgeInsets.symmetric(vertical: 2),
                child: Text(
                  '${entry.$1.criterionLabel}　${entry.$2} 条',
                  style: theme.textTheme.bodySmall,
                ),
              ),
          const SizedBox(height: 8),
          Text(
            '判据的逐条文字命令行还没给（现在只给条数），'
            '要显示每一条得让 `workflow <名字>` 把它放进 rows。',
            style: theme.textTheme.bodySmall?.copyWith(color: theme.disabledColor),
          ),
        ],
      ),
    );
  }
}
