import 'package:flutter/material.dart';

import 'cli/qtcloud_work.dart';
import 'models/task.dart';
import 'models/workflow.dart';
import 'models/workspace.dart';
import 'screens/flow_screen.dart';
import 'screens/settings_screen.dart';
import 'screens/task_screen.dart';
import 'views/sidebar.dart';
import 'views/topbar.dart';

/// 工作台：顶栏 + 侧栏 + 当前那一屏。（见 doc/index.md）
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
      home: Workbench(client: client, workspace: workspace),
    );
  }
}

class Workbench extends StatefulWidget {
  const Workbench({super.key, required this.client, required this.workspace});

  final QtcloudWork client;
  final Workspace workspace;

  @override
  State<Workbench> createState() => _WorkbenchState();
}

class _WorkbenchState extends State<Workbench> {
  String _page = 'task';
  bool _busy = false;
  String? _error;

  List<TaskSummary> _tasks = const [];
  TaskDetail? _task;
  List<WorkflowSummary> _workflowList = const [];
  WorkflowDetail? _workflow;

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
      final workflows = await widget.client.workflows();
      final workflow = workflows.isEmpty
          ? null
          : await widget.client.workflow(workflows.first.name);
      if (!mounted) return;
      setState(() {
        _tasks = tasks;
        _task = detail;
        _workflowList = workflows;
        _workflow = workflow;
      });
    } catch (error) {
      if (!mounted) return;
      setState(() => _error = '$error');
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  /// 起了一件任务：刷新列表、换到任务页、打开刚起的那件。
  Future<void> _created(String name) async {
    await _reload();
    if (!mounted) return;
    setState(() => _page = 'task');
    await _openTask(name);
  }

  Future<void> _openTask(String name) async {
    setState(() => _busy = true);
    try {
      final detail = await widget.client.task(name);
      if (!mounted) return;
      setState(() => _task = detail);
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
      body: Column(
        children: [
          Topbar(workspace: widget.workspace, busy: _busy, onRefresh: _reload),
          Expanded(
            child: Row(
              children: [
                Sidebar(
                  current: _page,
                  onSelect: (page) => setState(() => _page = page),
                ),
                const VerticalDivider(width: 1),
                Expanded(child: _body()),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _body() {
    if (_error != null) {
      return Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(_error!, textAlign: TextAlign.center),
            const SizedBox(height: 12),
            OutlinedButton(onPressed: _reload, child: const Text('再试一次')),
          ],
        ),
      );
    }
    switch (_page) {
      case 'flow':
        final workflow = _workflow;
        if (workflow == null) return const Center(child: Text('还没有工作流。'));
        return Column(
          children: [
            SizedBox(
              height: 48,
              child: Row(
                children: [
                  const SizedBox(width: 12),
                  DropdownButton<String>(
                    value: workflow.name,
                    items: [
                      for (final item in _workflowList)
                        DropdownMenuItem(
                          value: item.name,
                          child: Text(item.name),
                        ),
                    ],
                    onChanged: (name) async {
                      if (name == null) return;
                      final detail = await widget.client.workflow(name);
                      if (mounted) setState(() => _workflow = detail);
                    },
                  ),
                ],
              ),
            ),
            const Divider(height: 1),
            Expanded(
              child: FlowScreen(
                client: widget.client,
                workflow: workflow,
                workspace: widget.workspace,
                onCreated: _created,
              ),
            ),
          ],
        );
      case 'settings':
        return SettingsScreen(
          client: widget.client,
          workspace: widget.workspace,
        );
      default:
        final task = _task;
        if (task == null) return const Center(child: Text('还没有任务。'));
        return Column(
          children: [
            SizedBox(
              height: 48,
              child: Row(
                children: [
                  const SizedBox(width: 12),
                  DropdownButton<String>(
                    value: task.name,
                    items: [
                      for (final item in _tasks)
                        DropdownMenuItem(
                          value: item.name,
                          child: Text(item.name),
                        ),
                    ],
                    onChanged: (name) {
                      if (name != null) _openTask(name);
                    },
                  ),
                  const SizedBox(width: 12),
                  Text(
                    '任务列表里有 ${_tasks.length} 件',
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                ],
              ),
            ),
            const Divider(height: 1),
            Expanded(
              child: TaskScreen(
                client: widget.client,
                task: task,
                workspace: widget.workspace,
                busy: _busy,
                onReload: _reload,
              ),
            ),
          ],
        );
    }
  }
}
