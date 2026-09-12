import 'package:flutter/material.dart';
import 'package:quanttide_work/quanttide_work.dart' as qt;

import '../views/chat.dart';
import '../views/criteria_panel.dart';
import '../views/definition_view.dart';
import '../views/step_chain.dart';

/// 流程页：一条定义。左边对话，右边两态（步骤 / 定义）。（见 doc/screens/flow.md）
///
/// 只认传进来的领域对象与回调；拿数据、改状态在 `states/workbench_bloc.dart`。
class FlowScreen extends StatefulWidget {
  const FlowScreen({
    super.key,
    required this.workflow,
    required this.path,
    required this.yaml,
    required this.workspace,
    required this.busy,
    required this.onCreate,
    required this.onCheck,
  });

  final qt.Workflow workflow;

  /// 定义落在哪、原文是什么（定义态要看）。
  final String path;
  final String yaml;
  final qt.RunContext workspace;
  final bool busy;

  /// 起一件任务：把名字交出去，跑的是这条定义。
  final ValueChanged<String> onCreate;

  /// 核对定义：把结果拿回来弹窗（这是查询，不进状态）。
  final Future<({bool ok, List<String> lines})> Function() onCheck;

  @override
  State<FlowScreen> createState() => _FlowScreenState();
}

class _FlowScreenState extends State<FlowScreen> {
  // 这一屏自己的一点界面状态：看步骤还是看原文、选中第几步。
  bool _definition = false;
  int? _selected;

  /// 几步、几条判据。
  String get _summary {
    final total = widget.workflow.steps.fold(
      0,
      (sum, step) => sum + step.criteria.length,
    );
    return '${widget.workflow.steps.length} 步 · $total 条判据';
  }

  void _tell(String text) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(text)));
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

  /// 起一件任务：`task --new <名字> --workflow <这条>`。
  Future<void> _create() async {
    final name = await _askName();
    if (name == null || name.trim().isEmpty) return;
    widget.onCreate(name.trim());
  }

  /// 核对定义：`workflow <名字> --check`。
  Future<void> _check() async {
    final ({bool ok, List<String> lines}) result;
    try {
      result = await widget.onCheck();
    } catch (error) {
      if (mounted) _tell('$error');
      return;
    }
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
                '定义文件：${widget.path}\n'
                '要改就在这条对话里说，pi 会在工作流目录里改这个文件。\n\n'
                '定义原文：\n${widget.yaml}',
            opening: [
              const ChatMessage(
                text:
                    '这一屏聊的是这条流程该怎么走。改定义就直接说——'
                    'pi 会在工作流目录里改那个 .yaml。',
              ),
              ChatMessage(text: '${workflow.name}：$_summary'),
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
                        _summary,
                        overflow: TextOverflow.ellipsis,
                        style: Theme.of(context).textTheme.bodySmall,
                      ),
                    ),
                    const SizedBox(width: 8),
                    PopupMenuButton<String>(
                      enabled: !widget.busy,
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
                        workflow: widget.yaml,
                        path: widget.path,
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
