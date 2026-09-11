import 'package:flutter/material.dart';

import 'cli/qtcloud_work.dart';
import 'models/task.dart';
import 'models/workspace.dart';

void main() {
  runApp(
    QtcloudWorkStudioApp(
      client: QtcloudWork(workspace: Workspace.fromEnvironment()),
      workspace: Workspace.fromEnvironment(),
    ),
  );
}

/// 构建时注入的版本号（`flutter build web --release --dart-define=APP_VERSION=...`）。
const String appVersion = String.fromEnvironment(
  'APP_VERSION',
  defaultValue: 'dev',
);

/// 工作台。现在只有任务页：左边任务，右边这次走成什么样。
class QtcloudWorkStudioApp extends StatelessWidget {
  const QtcloudWorkStudioApp({
    super.key,
    required this.client,
    required this.workspace,
  });

  final QtcloudWork client;
  final Workspace workspace;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: '量潮工作云工作台',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark(useMaterial3: true),
      home: TaskScreen(client: client, workspace: workspace),
    );
  }
}

class TaskScreen extends StatefulWidget {
  const TaskScreen({super.key, required this.client, required this.workspace});

  final QtcloudWork client;
  final Workspace workspace;

  @override
  State<TaskScreen> createState() => _TaskScreenState();
}

class _TaskScreenState extends State<TaskScreen> {
  List<TaskSummary> _tasks = const [];
  TaskDetail? _detail;
  String? _error;
  bool _busy = false;

  @override
  void initState() {
    super.initState();
    _reload();
  }

  Future<void> _reload() async {
    setState(() {
      _busy = true;
      _error = null;
    });
    try {
      final tasks = await widget.client.tasks();
      final detail = tasks.isEmpty
          ? null
          : await widget.client.task(tasks.first.name);
      if (!mounted) return;
      setState(() {
        _tasks = tasks;
        _detail = detail;
      });
    } catch (error) {
      if (!mounted) return;
      setState(() => _error = '$error');
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  Future<void> _open(String name) async {
    setState(() => _busy = true);
    try {
      final detail = await widget.client.task(name);
      if (!mounted) return;
      setState(() => _detail = detail);
    } catch (error) {
      if (!mounted) return;
      setState(() => _error = '$error');
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  Future<void> _next() async {
    final name = _detail?.name;
    if (name == null) return;
    setState(() => _busy = true);
    try {
      await widget.client.next(name);
      await _open(name);
    } catch (error) {
      if (!mounted) return;
      setState(() => _error = '$error');
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('量潮工作云工作台'),
        actions: [
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 12),
            child: Center(child: Text('工作区 ${widget.workspace.root}')),
          ),
          IconButton(
            onPressed: _busy ? null : _reload,
            icon: const Icon(Icons.refresh),
            tooltip: '刷新',
          ),
        ],
      ),
      body: _error != null
          ? _ErrorPane(message: _error!, onRetry: _reload)
          : Row(
              children: [
                SizedBox(
                  width: 260,
                  child: _TaskList(
                    tasks: _tasks,
                    selected: _detail?.name,
                    onTap: _open,
                  ),
                ),
                const VerticalDivider(width: 1),
                Expanded(child: _TaskPane(detail: _detail, busy: _busy, onNext: _next)),
              ],
            ),
    );
  }
}

class _TaskList extends StatelessWidget {
  const _TaskList({required this.tasks, required this.selected, required this.onTap});

  final List<TaskSummary> tasks;
  final String? selected;
  final ValueChanged<String> onTap;

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: tasks.length,
      itemBuilder: (context, index) {
        final task = tasks[index];
        return ListTile(
          selected: task.name == selected,
          title: Text(task.name),
          subtitle: Text('${task.workflow}　下一步：${task.next}'),
          onTap: () => onTap(task.name),
        );
      },
    );
  }
}

class _TaskPane extends StatelessWidget {
  const _TaskPane({required this.detail, required this.busy, required this.onNext});

  final TaskDetail? detail;
  final bool busy;
  final VoidCallback onNext;

  @override
  Widget build(BuildContext context) {
    final task = detail;
    if (task == null) {
      return const Center(child: Text('还没有任务。'));
    }
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        Text(task.name, style: Theme.of(context).textTheme.titleLarge),
        const SizedBox(height: 4),
        Text('${task.workflowName}　开工 ${task.start}'),
        const SizedBox(height: 4),
        Text(task.stateLine.isEmpty ? '—' : task.stateLine),
        const SizedBox(height: 12),
        LinearProgressIndicator(value: task.progress),
        const SizedBox(height: 16),
        Text('步骤', style: Theme.of(context).textTheme.titleMedium),
        for (final step in task.steps)
          ListTile(
            dense: true,
            leading: Icon(step.done ? Icons.check_circle : Icons.circle_outlined),
            title: Text(step.name),
            subtitle: Text(step.done ? '走过' : '还没走'),
          ),
        const SizedBox(height: 16),
        Text('产物', style: Theme.of(context).textTheme.titleMedium),
        Text('报告　${task.products.report}'),
        Text('日志　${task.products.journal}'),
        Text('流水　${task.products.log}'),
        const SizedBox(height: 16),
        Text('流水（最近几条）', style: Theme.of(context).textTheme.titleMedium),
        for (final entry in task.journal)
          ListTile(
            dense: true,
            title: Text('${entry.step}　${entry.detail}',
                maxLines: 2, overflow: TextOverflow.ellipsis),
            subtitle: Text('${entry.at}　${entry.kind.label}'),
          ),
        const SizedBox(height: 16),
        FilledButton.icon(
          onPressed: busy || task.finished ? null : onNext,
          icon: const Icon(Icons.play_arrow),
          label: const Text('走下一步'),
        ),
      ],
    );
  }
}

class _ErrorPane extends StatelessWidget {
  const _ErrorPane({required this.message, required this.onRetry});

  final String message;
  final VoidCallback onRetry;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(message, textAlign: TextAlign.center),
            const SizedBox(height: 12),
            OutlinedButton(onPressed: onRetry, child: const Text('再试一次')),
          ],
        ),
      ),
    );
  }
}
