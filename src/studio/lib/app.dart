import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:quanttide_work/quanttide_work.dart' as qt;

import 'repositories/studio_repository.dart';
import 'screens/flow_screen.dart';
import 'screens/settings_screen.dart';
import 'screens/task_screen.dart';
import 'states/workbench_bloc.dart';
import 'widgets/sidebar.dart';
import 'widgets/topbar.dart';

/// 工作台：顶栏 + 侧栏 + 当前那一屏。（见 doc/index.md）
class QtcloudWorkStudioApp extends StatelessWidget {
  const QtcloudWorkStudioApp({
    super.key,
    required this.repository,
    required this.workspace,
  });

  final StudioRepository repository;
  final qt.RunContext workspace;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: '量潮工作云工作台',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark(useMaterial3: true),
      // 仓储给需要自己问一句的地方（核对定义、探活）；页面本身只认 Bloc。
      home: RepositoryProvider<StudioRepository>.value(
        value: repository,
        child: BlocProvider(
          create: (_) => WorkbenchBloc(repository)..add(const WorkbenchLoad()),
          child: Workbench(workspace: workspace),
        ),
      ),
    );
  }
}

/// 外壳：顶栏（工作区 + 刷新）、侧栏（三项）、当前那一屏。
class Workbench extends StatelessWidget {
  const Workbench({super.key, required this.workspace});

  final qt.RunContext workspace;

  @override
  Widget build(BuildContext context) {
    return BlocConsumer<WorkbenchBloc, WorkbenchState>(
      listenWhen: (previous, current) => current.note != null,
      listener: (context, state) {
        ScaffoldMessenger.of(context)
          ..hideCurrentSnackBar()
          ..showSnackBar(SnackBar(content: Text(state.note!)));
        context.read<WorkbenchBloc>().add(const WorkbenchNoteTaken());
      },
      builder: (context, state) => Scaffold(
        body: Column(
          children: [
            Topbar(
              workspace: workspace,
              busy: state.busy,
              onRefresh: () =>
                  context.read<WorkbenchBloc>().add(const WorkbenchLoad()),
            ),
            Expanded(
              child: Row(
                children: [
                  Sidebar(
                    current: state.page,
                    onSelect: (page) => context
                        .read<WorkbenchBloc>()
                        .add(WorkbenchShowPage(page)),
                  ),
                  const VerticalDivider(width: 1),
                  Expanded(child: _body(context, state)),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _body(BuildContext context, WorkbenchState state) {
    final bloc = context.read<WorkbenchBloc>();
    final repository = context.read<StudioRepository>();
    if (state.error != null) {
      return Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(state.error!, textAlign: TextAlign.center),
            const SizedBox(height: 12),
            OutlinedButton(
              onPressed: () => bloc.add(const WorkbenchLoad()),
              child: const Text('再试一次'),
            ),
          ],
        ),
      );
    }
    switch (state.page) {
      case 'flow':
        final workflow = state.workflow;
        if (workflow == null) return const Center(child: Text('还没有工作流。'));
        return Column(
          children: [
            Picker(
              value: workflow.name,
              items: [for (final item in state.workflows) item.name],
              onChanged: (name) => bloc.add(WorkbenchOpenWorkflow(name)),
            ),
            const Divider(height: 1),
            Expanded(
              child: FlowScreen(
                workflow: workflow,
                path: state.workflowPath,
                yaml: state.workflowYaml,
                workspace: workspace,
                busy: state.busy,
                onCreate: (name) => bloc.add(WorkbenchCreateTask(name)),
                onCheck: () => repository.check(workflow.name),
              ),
            ),
          ],
        );
      case 'settings':
        return SettingsScreen(workspace: workspace, onProbe: repository.health);
      default:
        final task = state.task;
        if (task == null) return const Center(child: Text('还没有任务。'));
        return Column(
          children: [
            Picker(
              value: task.name,
              items: [for (final item in state.tasks) item.name],
              onChanged: (name) => bloc.add(WorkbenchOpenTask(name)),
              trailing: '任务列表里有 ${state.tasks.length} 件',
            ),
            const Divider(height: 1),
            Expanded(
              child: TaskScreen(
                task: task,
                workflow: state.taskWorkflow,
                products: state.taskProducts,
                workspace: workspace,
                busy: state.busy,
                onNext: () => bloc.add(const WorkbenchRunNext()),
                onDone: () => bloc.add(const WorkbenchRecordDone()),
                onJournal: (text) => bloc.add(WorkbenchJournal(text)),
              ),
            ),
          ],
        );
    }
  }
}

/// 任务页与流程页顶上那一条：一个下拉换当前那条，右边一句备注。
class Picker extends StatelessWidget {
  const Picker({
    super.key,
    required this.value,
    required this.items,
    required this.onChanged,
    this.trailing,
  });

  final String value;
  final List<String> items;
  final ValueChanged<String> onChanged;
  final String? trailing;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 48,
      child: Row(
        children: [
          const SizedBox(width: 12),
          DropdownButton<String>(
            value: value,
            items: [
              for (final name in items)
                DropdownMenuItem(value: name, child: Text(name)),
            ],
            onChanged: (name) {
              if (name != null) onChanged(name);
            },
          ),
          if (trailing != null) ...[
            const SizedBox(width: 12),
            Text(trailing!, style: Theme.of(context).textTheme.bodySmall),
          ],
        ],
      ),
    );
  }
}
