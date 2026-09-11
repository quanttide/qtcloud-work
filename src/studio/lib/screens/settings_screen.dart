import 'package:flutter/material.dart';

import '../cli/qtcloud_work.dart';
import '../models/workspace.dart';

/// 设置页：三处位置 + provider 探活。（见 doc/screens/settings.md）
class SettingsScreen extends StatefulWidget {
  const SettingsScreen({
    super.key,
    required this.client,
    required this.workspace,
  });

  final QtcloudWork client;
  final Workspace workspace;

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  String? _health;

  Future<void> _probe() async {
    try {
      final result = await widget.client.health();
      if (!mounted) return;
      setState(() => _health = result.lines.join('\n'));
    } catch (error) {
      if (!mounted) return;
      setState(() => _health = '$error');
    }
  }

  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: const EdgeInsets.all(20),
      children: [
        _row(context, '工作区（--root）', widget.workspace.root),
        _row(context, '数据仓（--data）', widget.workspace.data),
        _row(context, '工作流目录（--workflows）', widget.workspace.workflows),
        const SizedBox(height: 12),
        Row(
          children: [
            OutlinedButton(onPressed: _probe, child: const Text('探活（health）')),
          ],
        ),
        if (_health != null)
          Padding(
            padding: const EdgeInsets.only(top: 8),
            child: SelectableText(_health!),
          ),
      ],
    );
  }

  Widget _row(BuildContext context, String key, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(key, style: Theme.of(context).textTheme.labelMedium),
          const SizedBox(height: 2),
          SelectableText(value),
        ],
      ),
    );
  }
}
