import 'package:flutter_bloc/flutter_bloc.dart';

import '../models/task.dart';
import '../models/workflow.dart';
import '../repositories/studio_repository.dart';

/// 工作台状态：看哪一页、有哪些任务与工作流、当前那件/那条的现状。
class WorkbenchState {
  const WorkbenchState({
    this.page = 'task',
    this.busy = false,
    this.error,
    this.note,
    this.tasks = const [],
    this.task,
    this.taskWorkflow,
    this.workflows = const [],
    this.workflow,
  });

  /// 当前那一屏：`task` / `flow` / `settings`。
  final String page;

  /// 有没有活在跑（顶栏与面板据此禁用按钮）。
  final bool busy;

  /// 上一次失败要说的话；没有就是 null。
  final String? error;

  /// 要弹一句的话；弹过就清掉。
  final String? note;

  final List<TaskSummary> tasks;

  /// 当前那件任务。
  final TaskDetail? task;

  /// 当前任务跑的那条工作流——状态面板要看闸门。
  final WorkflowDetail? taskWorkflow;

  final List<WorkflowSummary> workflows;

  /// 当前那条工作流定义。
  final WorkflowDetail? workflow;

  WorkbenchState copyWith({
    String? page,
    bool? busy,
    String? error,
    String? note,
    bool clearError = false,
    bool clearNote = false,
    List<TaskSummary>? tasks,
    TaskDetail? task,
    WorkflowDetail? taskWorkflow,
    List<WorkflowSummary>? workflows,
    WorkflowDetail? workflow,
  }) => WorkbenchState(
    page: page ?? this.page,
    busy: busy ?? this.busy,
    error: clearError ? null : (error ?? this.error),
    note: clearNote ? null : (note ?? this.note),
    tasks: tasks ?? this.tasks,
    task: task ?? this.task,
    taskWorkflow: taskWorkflow ?? this.taskWorkflow,
    workflows: workflows ?? this.workflows,
    workflow: workflow ?? this.workflow,
  );
}

/// 工作台事件。
sealed class WorkbenchEvent {
  const WorkbenchEvent();
}

/// 拉一遍任务与工作流，各打开第一条。
class WorkbenchLoad extends WorkbenchEvent {
  const WorkbenchLoad();
}

/// 换一屏。
class WorkbenchShowPage extends WorkbenchEvent {
  const WorkbenchShowPage(this.page);

  final String page;
}

/// 打开一件任务（状态面板跟着换）。
class WorkbenchOpenTask extends WorkbenchEvent {
  const WorkbenchOpenTask(this.name);

  final String name;
}

/// 打开一条工作流定义。
class WorkbenchOpenWorkflow extends WorkbenchEvent {
  const WorkbenchOpenWorkflow(this.name);

  final String name;
}

/// 走当前任务的下一步。
class WorkbenchRunNext extends WorkbenchEvent {
  const WorkbenchRunNext();
}

/// 人为地把当前步骤记一步。
class WorkbenchRecordDone extends WorkbenchEvent {
  const WorkbenchRecordDone();
}

/// 日志收叙事。
class WorkbenchJournal extends WorkbenchEvent {
  const WorkbenchJournal(this.text);

  final String text;
}

/// 起一件任务（跑当前那条定义），起来后换到任务页打开它。
class WorkbenchCreateTask extends WorkbenchEvent {
  const WorkbenchCreateTask(this.name);

  final String name;
}

/// 弹过那句话了，可以清掉。
class WorkbenchNoteTaken extends WorkbenchEvent {
  const WorkbenchNoteTaken();
}

/// 工作台的状态机：一页一页地拿数据，动作之后把列表与当前那条重新拉齐。
class WorkbenchBloc extends Bloc<WorkbenchEvent, WorkbenchState> {
  WorkbenchBloc(this._repository) : super(const WorkbenchState()) {
    on<WorkbenchLoad>(_onLoad);
    on<WorkbenchShowPage>(_onShowPage);
    on<WorkbenchOpenTask>(_onOpenTask);
    on<WorkbenchOpenWorkflow>(_onOpenWorkflow);
    on<WorkbenchRunNext>(_onRunNext);
    on<WorkbenchRecordDone>(_onRecordDone);
    on<WorkbenchJournal>(_onJournal);
    on<WorkbenchCreateTask>(_onCreateTask);
    on<WorkbenchNoteTaken>((_, emit) => emit(state.copyWith(clearNote: true)));
  }

  final StudioRepository _repository;

  /// 任务那条工作流——取不到就不显示闸门，不挡这一屏。
  Future<WorkflowDetail?> _workflowOf(TaskDetail task) async {
    try {
      return await _repository.workflow(task.workflowName);
    } catch (_) {
      return null;
    }
  }

  /// 拿齐两串列表；当前那件/那条还没选中就选第一条。
  Future<void> _onLoad(
    WorkbenchLoad event,
    Emitter<WorkbenchState> emit,
  ) async {
    emit(state.copyWith(busy: true, clearError: true));
    try {
      final tasks = await _repository.tasks();
      final workflows = await _repository.workflows();
      var task = state.task;
      var taskWorkflow = state.taskWorkflow;
      if (task == null && tasks.isNotEmpty) {
        task = await _repository.task(tasks.first.name);
        taskWorkflow = await _workflowOf(task);
      }
      var workflow = state.workflow;
      if (workflow == null && workflows.isNotEmpty) {
        workflow = await _repository.workflow(workflows.first.name);
      }
      emit(
        state.copyWith(
          busy: false,
          tasks: tasks,
          workflows: workflows,
          task: task,
          taskWorkflow: taskWorkflow,
          workflow: workflow,
        ),
      );
    } catch (error) {
      emit(state.copyWith(busy: false, error: '$error'));
    }
  }

  void _onShowPage(WorkbenchShowPage event, Emitter<WorkbenchState> emit) =>
      emit(state.copyWith(page: event.page));

  Future<void> _onOpenTask(
    WorkbenchOpenTask event,
    Emitter<WorkbenchState> emit,
  ) async {
    emit(state.copyWith(busy: true, clearError: true));
    try {
      final task = await _repository.task(event.name);
      emit(
        state.copyWith(
          busy: false,
          task: task,
          taskWorkflow: await _workflowOf(task),
        ),
      );
    } catch (error) {
      emit(state.copyWith(busy: false, error: '$error'));
    }
  }

  Future<void> _onOpenWorkflow(
    WorkbenchOpenWorkflow event,
    Emitter<WorkbenchState> emit,
  ) async {
    emit(state.copyWith(busy: true, clearError: true));
    try {
      final workflow = await _repository.workflow(event.name);
      emit(state.copyWith(busy: false, workflow: workflow));
    } catch (error) {
      emit(state.copyWith(busy: false, error: '$error'));
    }
  }

  /// 动作之后把任务列表与当前那件重新拉齐，并弹一句话。
  Future<void> _realign(
    Emitter<WorkbenchState> emit, {
    required String note,
  }) async {
    final tasks = await _repository.tasks();
    final name = state.task?.name;
    final task = name == null || !tasks.any((item) => item.name == name)
        ? state.task
        : await _repository.task(name);
    emit(
      state.copyWith(
        busy: false,
        tasks: tasks,
        task: task,
        taskWorkflow: task == null ? null : await _workflowOf(task),
        note: note,
      ),
    );
  }

  Future<void> _act(
    Emitter<WorkbenchState> emit,
    Future<void> Function(String name) action,
    String note,
  ) async {
    final name = state.task?.name;
    if (name == null) return;
    emit(state.copyWith(busy: true, clearError: true));
    try {
      await action(name);
      await _realign(emit, note: note);
    } catch (error) {
      emit(state.copyWith(busy: false, error: '$error'));
    }
  }

  Future<void> _onRunNext(
    WorkbenchRunNext event,
    Emitter<WorkbenchState> emit,
  ) => _act(emit, _repository.next, '走了一步');

  Future<void> _onRecordDone(
    WorkbenchRecordDone event,
    Emitter<WorkbenchState> emit,
  ) {
    final step = state.task?.currentStep;
    if (step == null) return Future.value();
    return _act(
      emit,
      (name) => _repository.done(name, step),
      '记了一步：$step',
    );
  }

  Future<void> _onJournal(
    WorkbenchJournal event,
    Emitter<WorkbenchState> emit,
  ) => _act(emit, (name) => _repository.journal(name, event.text), '日志记下一段');

  Future<void> _onCreateTask(
    WorkbenchCreateTask event,
    Emitter<WorkbenchState> emit,
  ) async {
    final name = event.name.trim();
    final workflow = state.workflow?.name;
    if (name.isEmpty || workflow == null) return;
    emit(state.copyWith(busy: true, clearError: true, page: 'task'));
    try {
      await _repository.create(name, workflow);
      final tasks = await _repository.tasks();
      final task = await _repository.task(name);
      emit(
        state.copyWith(
          busy: false,
          tasks: tasks,
          task: task,
          taskWorkflow: await _workflowOf(task),
          note: '起了任务：$name',
        ),
      );
    } catch (error) {
      emit(state.copyWith(busy: false, error: '$error'));
    }
  }
}
