import 'package:flutter/material.dart';

import '../models/executor.dart';
import '../models/task.dart';
import '../models/workflow.dart';

/// 状态面板：任务页右栏，从上到下五块。（见 doc/views/status-panel.md）
class StatusPanel extends StatelessWidget {
  const StatusPanel({
    super.key,
    required this.task,
    required this.workflow,
    required this.busy,
    required this.onNext,
    required this.onDone,
    required this.onJournal,
  });

  final TaskDetail task;
  final WorkflowDetail? workflow;
  final bool busy;
  final VoidCallback onNext;
  final VoidCallback onDone;
  final VoidCallback onJournal;

  /// 闸门：判据里有「人」的那几步——步骤执行者是 AI 也可能卡着人签。
  /// （见 doc/models/task.md「闸门与产物」）
  List<WorkflowStep> get _gates =>
      workflow?.steps
          .where(
            (step) =>
                step.executor == Executor.human || step.criteria.human > 0,
          )
          .toList() ??
      const [];

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        Row(
          children: [
            Icon(
              task.finished ? Icons.check_circle : Icons.play_circle,
              size: 16,
              color: theme.colorScheme.primary,
            ),
            const SizedBox(width: 6),
            Text(
              task.finished ? '走完' : '运行中',
              style: theme.textTheme.titleSmall,
            ),
            const Spacer(),
            Text('${task.doneCount} / ${task.steps.length}'),
          ],
        ),
        const SizedBox(height: 8),
        LinearProgressIndicator(value: task.progress),
        const SizedBox(height: 12),
        Text(
          task.currentStep == null ? '没有下一步' : '当前步骤　${task.currentStep}',
          style: theme.textTheme.bodyMedium,
        ),
        const SizedBox(height: 6),
        Text('关联流程　${task.workflowName}', style: theme.textTheme.bodyMedium),
        _block(theme, '闸门（留给人）', [
          if (_gates.isEmpty) '没有留给人那一步',
          for (final gate in _gates) '${gate.name}　待人放行',
        ]),
        _block(theme, '产物', [
          '报告　${task.products.report}',
          '日志　${task.products.journal}',
          '流水　${task.products.log}',
        ]),
        _block(theme, '流水（最近几条）', [
          if (task.journal.isEmpty) '还没有流水',
          for (final entry in task.journal)
            '${entry.at}　${entry.step}　${entry.detail}',
        ]),
        const SizedBox(height: 16),
        Row(
          children: [
            FilledButton.icon(
              onPressed: busy || task.finished ? null : onNext,
              icon: const Icon(Icons.play_arrow, size: 18),
              label: const Text('走下一步'),
            ),
            const SizedBox(width: 8),
            OutlinedButton(
              onPressed: busy ? null : onDone,
              child: const Text('记一步'),
            ),
            const Spacer(),
            PopupMenuButton<String>(
              tooltip: '更多',
              icon: const Icon(Icons.more_horiz),
              onSelected: (value) {
                if (value == 'journal') onJournal();
              },
              itemBuilder: (context) => const [
                PopupMenuItem(value: 'journal', child: Text('记日志')),
              ],
            ),
          ],
        ),
        const SizedBox(height: 6),
        Text(
          '走下一步 = --next，记一步 = --done，记日志 = --journal',
          style: theme.textTheme.bodySmall,
        ),
      ],
    );
  }

  Widget _block(ThemeData theme, String title, List<String> lines) {
    return Padding(
      padding: const EdgeInsets.only(top: 14),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(title, style: theme.textTheme.labelMedium),
          const SizedBox(height: 4),
          for (final line in lines)
            Padding(
              padding: const EdgeInsets.symmetric(vertical: 1),
              child: Text(line, style: theme.textTheme.bodySmall),
            ),
        ],
      ),
    );
  }
}
