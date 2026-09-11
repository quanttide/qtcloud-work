import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/cli/qtcloud_work.dart';
import 'package:qtcloud_work_studio/cli/runner.dart';
import 'package:qtcloud_work_studio/models/workspace.dart';

const workspace = Workspace(
  root: '/w',
  data: '/w/data/context/qtcloud-work',
  workflows: '/w/data/profile/quanttide/workflows',
);

/// 假的 runner：记下每次调用的参数，按脚本回话。
class RecordingRunner implements Runner {
  RecordingRunner(this.replies);

  /// 键是子命令（如 `task --list`），值是 stdout。
  final Map<String, String> replies;
  final List<List<String>> calls = [];

  @override
  Future<RunOutput> run(String executable, List<String> arguments) async {
    calls.add([executable, ...arguments]);
    for (final entry in replies.entries) {
      if (arguments.contains(entry.key) || arguments.join(' ').contains(entry.key)) {
        return RunOutput(exitCode: 0, stdout: entry.value);
      }
    }
    return const RunOutput(exitCode: 1, stdout: '{"ok": false, "lines": ["没有这条回复"]}');
  }
}

String fixture(String name) => File('test/fixtures/$name.json').readAsStringSync();

void main() {
  group('命令怎么拼', () {
    test('每次都带上三处位置与 --json', () async {
      final runner = RecordingRunner({'--list': fixture('task_list')});
      final client = QtcloudWork(workspace: workspace, runner: runner);
      await client.taskList();
      expect(runner.calls.single, [
        'qtcloud-work',
        '--root', '/w',
        '--data', '/w/data/context/qtcloud-work',
        '--workflows', '/w/data/profile/quanttide/workflows',
        '--json',
        'task', '--list',
      ]);
    });

    test('记一步带得上一句话', () async {
      final runner = RecordingRunner({'--done': '{"ok":true,"rows":[]}'});
      final client = QtcloudWork(workspace: workspace, runner: runner);
      await client.done('demo', 'audit', note: '预检 7/7');
      expect(runner.calls.single.sublist(8), ['task', 'demo', '--done', 'audit', '--note', '预检 7/7']);
    });

    test('不说那句话就只带 --done', () async {
      final runner = RecordingRunner({'--done': '{"ok":true,"rows":[]}'});
      final client = QtcloudWork(workspace: workspace, runner: runner);
      await client.done('demo', 'audit');
      expect(runner.calls.single.sublist(8), ['task', 'demo', '--done', 'audit']);
    });

    test('走下一步、记日志、核对定义各是各的命令', () async {
      final runner = RecordingRunner({
        '--next': '{"ok":true,"rows":[]}',
        '--journal': '{"ok":true,"rows":[]}',
        '--check': '{"ok":true,"rows":[]}',
      });
      final client = QtcloudWork(workspace: workspace, runner: runner);
      await client.next('demo');
      await client.journal('demo', '今天走了三步');
      await client.workflowCheck('devops-release');
      expect(runner.calls[0].sublist(8), ['task', 'demo', '--next']);
      expect(runner.calls[1].sublist(8), ['task', 'demo', '--journal', '今天走了三步']);
      expect(runner.calls[2].sublist(8), ['workflow', 'devops-release', '--check']);
    });
  });

  group('读回模型', () {
    test('任务列表读成模型', () async {
      final runner = RecordingRunner({'--list': fixture('task_list')});
      final client = QtcloudWork(workspace: workspace, runner: runner);
      final tasks = await client.tasks();
      expect(tasks.length, 5);
      expect(tasks.first.workflow, 'compare-course-profile');
    });

    test('工作流详情读成模型', () async {
      final runner = RecordingRunner({'learn-task-create': fixture('workflow_detail')});
      final client = QtcloudWork(workspace: workspace, runner: runner);
      final workflow = await client.workflow('learn-task-create');
      expect(workflow.steps.length, 5);
      expect(workflow.criteria.total, 18);
    });

    test('命令行说不行就抛，把它印的话带出来', () async {
      final runner = RecordingRunner({'nope': '{"ok":false,"lines":["没有这条工作流：nope"]}'});
      final client = QtcloudWork(workspace: workspace, runner: runner);
      expect(
        () => client.workflow('nope'),
        throwsA(
          isA<CliFailure>().having((e) => e.message, 'message', contains('没有这条工作流')),
        ),
      );
    });
  });

  group('工作流页', () {
    test('列表给出名字、步骤串与位置', () async {
      final runner = RecordingRunner({'--list': fixture('workflow_list')});
      final client = QtcloudWork(workspace: workspace, runner: runner);
      final workflows = await client.workflows();
      expect(workflows.length, 5);
      expect(workflows.map((w) => w.name), contains('devops-release'));
      expect(workflows.first.steps, 'outline、doc、test、code');
    });
  });
}
