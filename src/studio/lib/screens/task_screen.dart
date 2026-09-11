import 'package:flutter/material.dart';

import '../cli/qtcloud_work.dart';
import '../models/task.dart';
import '../models/workflow.dart';
import '../models/workspace.dart';
import '../views/chat.dart';
import '../views/status_panel.dart';

/// 任务页：一次执行实例。左边对话，右边状态面板。（见 doc/screens/task.md）
class TaskScreen extends StatefulWidget {
  const TaskScreen({
    super.key,
    required this.client,
    required this.task,
    required this.workspace,
    required this.onReload,
    this.busy = false,
  });

  final QtcloudWork client;
  final TaskDetail task;
  final Workspace workspace;
  final Future<void> Function() onReload;
  final bool busy;

  @override
  State<TaskScreen> createState() => _TaskScreenState();
}

class _TaskScreenState extends State<TaskScreen> {
  WorkflowDetail? _workflow;

  @override
  void initState() {
    super.initState();
    _loadWorkflow();
  }

  @override
  void didUpdateWidget(covariant TaskScreen oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.task.workflowName != widget.task.workflowName) {
      _loadWorkflow();
    }
  }

  /// 发话时随带的背景：这一屏在看什么。
  String _brief() {
    final task = widget.task;
    final lines = [
      '你在量潮工作云工作台里，看的是任务「${task.name}」（跑的是工作流 ${task.workflowName}）。',
      '开工：${task.start}',
      '当前状态：${task.stateLine}',
      '产物落点：报告 ${task.products.report}／流水 ${task.products.journal}／日志 ${task.products.log}',
      '任务文件：${widget.workspace.data}/tasks/${task.name}.yaml',
      '工作流定义：${widget.workspace.workflows}/${task.workflowName}.yaml',
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

  Future<void> _loadWorkflow() async {
    try {
      final workflow = await widget.client.workflow(widget.task.workflowName);
      if (!mounted) return;
      setState(() => _workflow = workflow);
    } catch (_) {
      // 关联流程取不到就不显示闸门，不挡这一屏
    }
  }

  Future<void> _journal() async {
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
    await widget.client.journal(widget.task.name, text);
    await widget.onReload();
  }

  Future<void> _done() async {
    final step = widget.task.currentStep;
    if (step == null) return;
    await widget.client.done(widget.task.name, step);
    await widget.onReload();
  }

  Future<void> _next() async {
    await widget.client.next(widget.task.name);
    await widget.onReload();
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
          child: ChatPanel(
            placeholder: '说目标，或者对流程提修改…',
            workingDir: widget.workspace.root,
            brief: _brief(),
            opening: [
              const ChatMessage(
                text:
                    '这一屏聊的是目标与流程：想改的是流程定义，不是这一次的产物——'
                    '产物不满意，改流程再跑一遍。话都交给本机的 pi。',
              ),
              ChatMessage(
                text:
                    '这次跑的是 ${widget.task.workflowName}，${widget.task.stateLine}',
              ),
            ],
          ),
        ),
        const VerticalDivider(width: 1),
        SizedBox(
          width: 420,
          child: StatusPanel(
            task: widget.task,
            workflow: _workflow,
            busy: widget.busy,
            onNext: _next,
            onDone: _done,
            onJournal: _journal,
          ),
        ),
      ],
    );
  }
}
