import 'package:flutter/material.dart';
import 'package:qtcloud_work_studio/quanttide_work.dart' as qt;
import '../repositories/local/tasks.dart';

/// 状态面板：任务页右栏，从上到下五块。（见 doc/views/status-panel.md）
///
/// 走过的步骤、下一步、进度都由领域模型的任务聚合算——界面只管画。
class StatusPanel extends StatelessWidget {
  const StatusPanel({
    super.key,
    required this.task,
    required this.workflow,
    required this.artifacts,
    required this.busy,
    required this.onNext,
    required this.onDone,
    required this.onJournal,
  });

  final qt.Task task;
  final qt.Workflow? workflow;

  /// 三样产物的落点（按工作区算好的，相对数据仓）。
  final Map<String, String> artifacts;
  final bool busy;
  final VoidCallback onNext;
  final VoidCallback onDone;
  final VoidCallback onJournal;

  /// 闸门：这一步本来就是人做的，或留着人拍板的判据。
  List<qt.Step> get _gates =>
      workflow?.steps
          .where((step) => step.isHuman || step.gates.isNotEmpty)
          .toList() ??
      const [];

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final flow = workflow;
    final done = flow == null ? const <String>[] : workspaceOf(task, flow).doneSteps(task);
    final total = flow?.steps.length ?? 0;
    final finished = total > 0 && done.length == total;
    final progress = total == 0 ? 0.0 : done.length / total;
    final current = flow == null ? null : workspaceOf(task, flow).nextStep(task);
    final recent = task.journal.length <= 5
        ? task.journal
        : task.journal.sublist(task.journal.length - 5);
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        Row(
          children: [
            Icon(
              finished ? Icons.check_circle : Icons.play_circle,
              size: 16,
              color: theme.colorScheme.primary,
            ),
            const SizedBox(width: 6),
            Text(finished ? '走完' : '运行中', style: theme.textTheme.titleSmall),
            const Spacer(),
            Text('${done.length} / $total'),
          ],
        ),
        const SizedBox(height: 8),
        LinearProgressIndicator(value: progress),
        const SizedBox(height: 12),
        Text(
          current == null ? '没有下一步' : '当前步骤　$current',
          style: theme.textTheme.bodyMedium,
        ),
        const SizedBox(height: 6),
        Text('关联流程　${task.workflowName}', style: theme.textTheme.bodyMedium),
        _block(theme, '闸门（留给人）', [
          if (_gates.isEmpty) '没有留给人那一步',
          for (final gate in _gates) '${gate.name}　待人放行',
        ]),
        _block(theme, '产物', [
          '报告　${artifacts['report'] ?? ''}',
          '日志　${artifacts['journal'] ?? ''}',
          '流水　${artifacts['log'] ?? ''}',
        ]),
        _block(theme, '流水（最近几条）', [
          if (recent.isEmpty) '还没有流水',
          for (final entry in recent)
            '${entry.at}　${entry.step}　${entry.detail}',
        ]),
        const SizedBox(height: 16),
        Row(
          children: [
            FilledButton.icon(
              onPressed: busy || finished ? null : onNext,
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
