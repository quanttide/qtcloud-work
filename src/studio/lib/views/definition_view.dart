import 'package:flutter/material.dart';

/// 定义态：这条流程的 YAML 原文在哪。（见 doc/views/definition-view.md）
class DefinitionView extends StatelessWidget {
  const DefinitionView({super.key, required this.workflow});

  final String workflow;
  final String? path = null;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SelectableText(workflow, style: theme.textTheme.titleSmall),
          const SizedBox(height: 8),
          Text(
            '定义原文是命令行的工作流目录里的那个 .yaml。'
            '命令行现在不给原文，只给位置（`workflow <名字>` 的第一行）。',
            style: theme.textTheme.bodySmall,
          ),
        ],
      ),
    );
  }
}
