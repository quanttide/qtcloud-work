import 'package:flutter/material.dart';
import 'package:quanttide_work/quanttide_work.dart' as qt;

import '../widgets/chat.dart';
import '../widgets/status_panel.dart';

/// 任务页：一次执行实例。左边对话，右边状态面板。（见 doc/screens/task.md）
///
/// 只认传进来的领域对象与回调；拿数据、改状态在 `states/workbench_bloc.dart`。
class TaskScreen extends StatelessWidget {
  const TaskScreen({
    super.key,
    required this.task,
    required this.workflow,
    required this.artifacts,
    required this.workspace,
    required this.busy,
    required this.onNext,
    required this.onDone,
    required this.onJournal,
  });

  final qt.Task task;

  /// 这件任务跑的那条定义——状态面板要看闸门、算下一步。
  final qt.Workflow? workflow;

  /// 三样产物的落点（按工作区算好的）。
  final Map<String, String> artifacts;
  final qt.RunContext workspace;
  final bool busy;
  final VoidCallback onNext;
  final VoidCallback onDone;
  final ValueChanged<String> onJournal;

  /// 发话时随带的背景：这一屏在看什么。
  String _brief() {
    final flow = workflow;
    final lines = [
      '你在量潮工作云工作台里，看的是任务「${task.name}」（跑的是工作流 ${task.workflowName}）。',
      '开工：${task.start}',
      '当前状态：${flow == null ? '定义取不到' : task.stateLine(flow)}',
      '产物落点：报告 ${artifacts['report'] ?? ''}／流水 ${artifacts['journal'] ?? ''}／日志 ${artifacts['log'] ?? ''}',
      '任务文件：${workspace.data}/tasks/${task.name}.yaml',
      '工作流定义：${workspace.workflows}/${task.workflowName}.yaml',
    ];
    if (task.journal.isNotEmpty) {
      lines.add('最近流水：');
      lines.addAll([
        for (final entry in task.journal)
          '  ${entry.at}　${entry.step}　${entry.detail}',
      ]);
    }
    return lines.join('\n');
  }

  Future<void> _journal(BuildContext context) async {
    final controller = TextEditingController();
    final text = await showDialog<String>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('记日志'),
        content: TextField(
          controller: controller,
          autofocus: true,
          maxLines: 3,
          decoration: const InputDecoration(hintText: '一句话：这一步是怎么走的'),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('取消'),
          ),
          FilledButton(
            onPressed: () => Navigator.pop(context, controller.text),
            child: const Text('记下'),
          ),
        ],
      ),
    );
    if (text == null || text.isEmpty) return;
    onJournal(text);
  }

  @override
  Widget build(BuildContext context) {
    final flow = workflow;
    return Row(
      children: [
        Expanded(
          child: ChatPanel(
            placeholder: '说目标，或者对流程提修改…',
            workingDir: workspace.root,
            brief: _brief(),
            opening: [
              const ChatMessage(
                text:
                    '这一屏聊的是目标与流程：想改的是流程定义，不是这一次的产物——'
                    '产物不满意，改流程再跑一遍。话都交给本机的 pi。',
              ),
              ChatMessage(
                text: '这次跑的是 ${task.workflowName}，'
                    '${flow == null ? '定义取不到' : task.stateLine(flow)}',
              ),
            ],
          ),
        ),
        const VerticalDivider(width: 1),
        SizedBox(
          width: 420,
          child: StatusPanel(
            task: task,
            workflow: flow,
            artifacts: artifacts,
            busy: busy,
            onNext: onNext,
            onDone: onDone,
            onJournal: () => _journal(context),
          ),
        ),
      ],
    );
  }
}
