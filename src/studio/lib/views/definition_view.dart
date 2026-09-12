import 'package:flutter/material.dart';

/// 定义态：这条流程的 YAML 原文，只读。（见 doc/views/definition-view.md）
class DefinitionView extends StatelessWidget {
  const DefinitionView({super.key, required this.workflow, this.path = ''});

  /// 定义文件的原文。
  final String workflow;

  /// 原文是从哪个文件读来的。
  final String path;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(path, style: theme.textTheme.bodySmall),
          const SizedBox(height: 8),
          SelectableText(
            workflow.isEmpty ? '（读不到定义文件）' : workflow,
            style: theme.textTheme.bodySmall?.copyWith(
              fontFamily: 'monospace',
              height: 1.5,
            ),
          ),
        ],
      ),
    );
  }
}
