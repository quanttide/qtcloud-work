import 'package:flutter/material.dart';

import '../../domain/workflow.dart';

/// 判据面板：点一个步骤，下面列出这一步的全部判据。（见 doc/views/criteria-panel.md）
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
              Text(
                current.executor.stepLabel,
                style: theme.textTheme.bodySmall,
              ),
              const Spacer(),
              Text(
                '${current.criteria.total} 条判据（机器判 ${current.criteria.rule}）',
                style: theme.textTheme.bodySmall,
              ),
            ],
          ),
          const SizedBox(height: 8),
          for (final item in current.items)
            Padding(
              padding: const EdgeInsets.symmetric(vertical: 3),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  SizedBox(
                    width: 64,
                    child: Text(
                      item.executor.criterionLabel,
                      style: theme.textTheme.bodySmall?.copyWith(
                        color: theme.colorScheme.primary,
                      ),
                    ),
                  ),
                  Expanded(
                    child: Text(item.text, style: theme.textTheme.bodySmall),
                  ),
                ],
              ),
            ),
          if (current.items.isEmpty)
            Text(
              '这一步的判据没有逐条文字（定义文件里没写 `description`，也没写路径类的字段）。',
              style: theme.textTheme.bodySmall?.copyWith(
                color: theme.disabledColor,
              ),
            ),
        ],
      ),
    );
  }
}
