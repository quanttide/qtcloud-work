import 'package:flutter/material.dart';

import '../cli/qtcloud_work.dart';
import '../models/task.dart';
import '../models/workflow.dart';
import '../views/chat.dart';
import '../views/status_panel.dart';

/// 任务页：一次执行实例。左边对话，右边状态面板。（见 doc/screens/task.md）
class TaskScreen extends StatefulWidget {
  const TaskScreen({
    super.key,
    required this.client,
    required this.task,
    required this.onReload,
    this.busy = false,
  });

  final QtcloudWork client;
  final TaskDetail task;
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
            messages: [
              const ChatMessage(text: '命令行还没有对话这一路：目标与修改现在走命令与工作流文件。'),
              ChatMessage(text: '这次跑的是 ${widget.task.workflowName}。'),
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
