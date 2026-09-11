import 'package:flutter/material.dart';

/// 侧栏：三项，各两字，切页用，不装别的。（见 doc/views/sidebar.md）
class Sidebar extends StatelessWidget {
  const Sidebar({super.key, required this.current, required this.onSelect});

  final String current;
  final ValueChanged<String> onSelect;

  static const items = <({String key, String label, IconData icon})>[
    (key: 'task', label: '任务', icon: Icons.check_circle_outline),
    (key: 'flow', label: '流程', icon: Icons.account_tree_outlined),
    (key: 'settings', label: '设置', icon: Icons.settings_outlined),
  ];

  @override
  Widget build(BuildContext context) {
    // 用 Material 而不是带底色的 Container：ListTile 的水波要画在最近的 Material 上
    return Material(
      color: Theme.of(context).colorScheme.surface,
      child: Container(
        width: 150,
        padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 8),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            for (final item in items)
              ListTile(
                dense: true,
                selected: item.key == current,
                leading: Icon(item.icon, size: 18),
                title: Text(item.label),
                onTap: () => onSelect(item.key),
              ),
          ],
        ),
      ),
    );
  }
}
