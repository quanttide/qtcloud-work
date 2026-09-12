import 'package:flutter/material.dart';

import '../repositories/client.dart';
import '../models/workflow.dart';
import '../models/workspace.dart';
import '../widgets/chat.dart';
import '../widgets/criteria_panel.dart';
import '../widgets/definition_view.dart';
import '../widgets/step_chain.dart';

/// 流程页：一条定义。左边对话，右边两态（步骤 / 定义）。（见 doc/screens/flow.md）
class FlowScreen extends StatefulWidget {
  const FlowScreen({
    super.key,
    required this.client,
    required this.workflow,
    required this.workspace,
    this.onCreated,
  });

  final QtcloudWork client;
  final WorkflowDetail workflow;
  final Workspace workspace;

  /// 起了一件任务之后通知外面（刷新列表、去打开它）。
  final Future<void> Function(String name)? onCreated;

  @override
  State<FlowScreen> createState() => _FlowScreenState();
}

class _FlowScreenState extends State<FlowScreen> {
  bool _definition = false;
  bool _busy = false;
  int? _selected;

  void _tell(String text) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(text)));
  }

  /// 起一件任务：`task --new <名字> --workflow <这条>`。
  Future<void> _create() async {
    final name = await _askName();
    if (name == null || name.trim().isEmpty) return;
    setState(() => _busy = true);
    try {
      final result = await widget.client.create(
        name.trim(),
        widget.workflow.name,
      );
      if (!mounted) return;
      if (result.ok) {
        _tell('已起任务：${name.trim()}');
        await widget.onCreated?.call(name.trim());
      } else {
        _tell(result.lines.join('　'));
      }
    } catch (error) {
      if (mounted) _tell('$error');
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  Future<String?> _askName() {
    final field = TextEditingController(text: widget.workflow.name);
    return showDialog<String>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('起一件任务'),
        content: TextField(
          controller: field,
          autofocus: true,
          decoration: const InputDecoration(labelText: '任务名'),
          onSubmitted: (value) => Navigator.of(context).pop(value),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('取消'),
          ),
          FilledButton(
            onPressed: () => Navigator.of(context).pop(field.text),
            child: const Text('起'),
          ),
        ],
      ),
    );
  }

  /// 核对定义：`workflow <名字> --check`。
  Future<void> _check() async {
    setState(() => _busy = true);
    try {
      final result = await widget.client.workflowCheck(widget.workflow.name);
      if (!mounted) return;
      await showDialog<void>(
        context: context,
        builder: (context) => AlertDialog(
          title: Text(result.ok ? '定义没问题' : '定义有几处要改'),
          content: SingleChildScrollView(
            child: SelectableText(result.lines.join('\n')),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(context).pop(),
              child: const Text('知道了'),
            ),
          ],
        ),
      );
    } catch (error) {
      if (mounted) _tell('$error');
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final workflow = widget.workflow;
    final step = _selected == null ? null : workflow.steps[_selected!];
    return Row(
      children: [
        Expanded(
          child: ChatPanel(
            placeholder: '说这条流程该怎么走…',
            workingDir: widget.workspace.root,
            brief:
                '你在量潮工作云工作台里，看的是工作流定义「${workflow.name}」。\n'
                '定义文件：${workflow.path}\n'
                '要改就在这条对话里说，pi 会在工作流目录里改这个文件。\n\n'
                '定义原文：\n${workflow.yaml}',
            opening: [
              const ChatMessage(
                text:
                    '这一屏聊的是这条流程该怎么走。改定义就直接说——'
                    'pi 会在工作流目录里改那个 .yaml。',
              ),
              ChatMessage(text: '${workflow.name}：${workflow.summary}'),
            ],
          ),
        ),
        const VerticalDivider(width: 1),
        SizedBox(
          width: 520,
          child: Column(
            children: [
              Container(
                height: 48,
                padding: const EdgeInsets.symmetric(horizontal: 14),
                child: Row(
                  children: [
                    Text(workflow.name),
                    const SizedBox(width: 8),
                    Expanded(
                      child: Text(
                        workflow.summary,
                        overflow: TextOverflow.ellipsis,
                        style: Theme.of(context).textTheme.bodySmall,
                      ),
                    ),
                    const SizedBox(width: 8),
                    PopupMenuButton<String>(
                      enabled: !_busy,
                      tooltip: '更多',
                      onSelected: (value) {
                        if (value == 'check') _check();
                        if (value == 'new') _create();
                      },
                      itemBuilder: (context) => const [
                        PopupMenuItem(value: 'new', child: Text('起一件任务')),
                        PopupMenuItem(value: 'check', child: Text('检查定义')),
                      ],
                    ),
                    const SizedBox(width: 8),
                    SegmentedButton<bool>(
                      style: SegmentedButton.styleFrom(
                        visualDensity: VisualDensity.compact,
                        padding: const EdgeInsets.symmetric(horizontal: 10),
                      ),
                      segments: const [
                        ButtonSegment(value: false, label: Text('步骤')),
                        ButtonSegment(value: true, label: Text('定义')),
                      ],
                      selected: {_definition},
                      onSelectionChanged: (value) =>
                          setState(() => _definition = value.first),
                    ),
                  ],
                ),
              ),
              const Divider(height: 1),
              Expanded(
                child: _definition
                    ? DefinitionView(
                        workflow: workflow.yaml,
                        path: workflow.path,
                      )
                    : StepChain(
                        workflow: workflow,
                        selected: _selected,
                        onSelect: (index) => setState(() => _selected = index),
                      ),
              ),
              if (!_definition) ...[
                const Divider(height: 1),
                CriteriaPanel(step: step),
              ],
            ],
          ),
        ),
      ],
    );
  }
}
