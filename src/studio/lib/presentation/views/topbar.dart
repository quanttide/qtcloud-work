import 'package:flutter/material.dart';

import '../../domain/workspace.dart';

/// 顶栏：只有工作区切换。（见 doc/views/topbar.md）
class Topbar extends StatelessWidget {
  const Topbar({
    super.key,
    required this.workspace,
    this.busy = false,
    required this.onRefresh,
  });

  final Workspace workspace;
  final bool busy;
  final VoidCallback onRefresh;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 48,
      padding: const EdgeInsets.symmetric(horizontal: 14),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surface,
        border: Border(
          bottom: BorderSide(color: Theme.of(context).dividerColor),
        ),
      ),
      child: Row(
        children: [
          const Icon(Icons.workspaces_outline, size: 16),
          const SizedBox(width: 8),
          // 工作区只有一个时，这里就是它的名字：根的末一段
          Text(_name(workspace.root)),
          const Spacer(),
          Text('工作区 --root', style: Theme.of(context).textTheme.bodySmall),
          const SizedBox(width: 8),
          IconButton(
            onPressed: busy ? null : onRefresh,
            icon: const Icon(Icons.refresh, size: 18),
            tooltip: '刷新',
          ),
        ],
      ),
    );
  }

  String _name(String root) {
    final parts = root.split('/').where((part) => part.isNotEmpty).toList();
    return parts.isEmpty ? root : parts.last;
  }
}
