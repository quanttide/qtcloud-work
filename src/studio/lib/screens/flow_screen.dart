import 'package:flutter/material.dart';

import '../cli/qtcloud_work.dart';
import '../models/workflow.dart';
import '../views/chat.dart';
import '../views/criteria_panel.dart';
import '../views/definition_view.dart';
import '../views/step_chain.dart';

/// 流程页：一条定义。左边对话，右边两态（步骤 / 定义）。（见 doc/screens/flow.md）
class FlowScreen extends StatefulWidget {
  const FlowScreen({super.key, required this.client, required this.workflow});

  final QtcloudWork client;
  final WorkflowDetail workflow;

  @override
  State<FlowScreen> createState() => _FlowScreenState();
}

class _FlowScreenState extends State<FlowScreen> {
  bool _definition = false;
  int? _selected;

  @override
  Widget build(BuildContext context) {
    final workflow = widget.workflow;
    final step = _selected == null ? null : workflow.steps[_selected!];
    return Row(
      children: [
        Expanded(
          child: ChatPanel(
            placeholder: '说这条流程该怎么走…',
            messages: [
              const ChatMessage(text: '命令行还没有对话这一路：改流程现在改工作流目录里的 .yaml，'
                  '或者 `workflow --new` 起一条新的。'),
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
                    SegmentedButton<bool>(
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
                    ? DefinitionView(workflow: workflow.path)
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
