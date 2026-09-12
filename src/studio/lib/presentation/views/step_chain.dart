import 'package:flutter/material.dart';

import '../../domain/executor.dart';
import '../../domain/workflow.dart';

/// 步骤链：一步一个节点，竖着排，一步接一步。（见 doc/views/step-chain.md）
class StepChain extends StatelessWidget {
  const StepChain({
    super.key,
    required this.workflow,
    this.selected,
    required this.onSelect,
  });

  final WorkflowDetail workflow;
  final int? selected;
  final ValueChanged<int> onSelect;

  static Color colorOf(Executor executor, ColorScheme scheme) =>
      switch (executor) {
        Executor.agent => scheme.tertiary,
        Executor.rule => scheme.secondary,
        Executor.human => scheme.primary,
      };

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        for (var i = 0; i < workflow.steps.length; i++) ...[
          InkWell(
            onTap: () => onSelect(i),
            borderRadius: BorderRadius.circular(8),
            child: Container(
              decoration: BoxDecoration(
                color: scheme.surfaceContainerHighest,
                borderRadius: BorderRadius.circular(8),
                // 选中用一圈整齐的边；左边那条按执行者上色，画在圆角里面
                border: Border.all(
                  color: i == selected ? scheme.primary : scheme.outlineVariant,
                ),
              ),
              child: ClipRRect(
                borderRadius: BorderRadius.circular(8),
                child: Row(
                  children: [
                    Container(
                      width: 3,
                      height: 38,
                      color: colorOf(workflow.steps[i].executor, scheme),
                    ),
                    Expanded(
                      child: Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 10),
                        child: Row(
                          children: [
                            Text(workflow.steps[i].name),
                            const Spacer(),
                            Text(
                              '${workflow.steps[i].executor.stepLabel} · '
                              '${workflow.steps[i].criteria.total} 条判据',
                              style: Theme.of(context).textTheme.bodySmall,
                            ),
                          ],
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
          if (i != workflow.steps.length - 1)
            Container(
              margin: const EdgeInsets.only(left: 24),
              width: 1,
              height: 14,
              color: scheme.outlineVariant,
            ),
        ],
      ],
    );
  }
}
