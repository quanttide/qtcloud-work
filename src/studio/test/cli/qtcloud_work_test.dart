import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/cli/qtcloud_work.dart';

import '../support/fake_runner.dart';

const workspace = testWorkspace;

void main() {
  group('命令怎么拼', () {
    test('每次都带上三处位置与 --json', () async {
      final runner = FakeRunner({'--list': fixture('task_list')});
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
      final runner = FakeRunner({'--done': '{"ok":true,"rows":[]}'});
      final client = QtcloudWork(workspace: workspace, runner: runner);
      await client.done('demo', 'audit', note: '预检 7/7');
      expect(runner.calls.single.sublist(8), ['task', 'demo', '--done', 'audit', '--note', '预检 7/7']);
    });

    test('不说那句话就只带 --done', () async {
      final runner = FakeRunner({'--done': '{"ok":true,"rows":[]}'});
      final client = QtcloudWork(workspace: workspace, runner: runner);
      await client.done('demo', 'audit');
      expect(runner.calls.single.sublist(8), ['task', 'demo', '--done', 'audit']);
    });

    test('走下一步、记日志、核对定义各是各的命令', () async {
      final runner = FakeRunner({
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
      final runner = FakeRunner({'--list': fixture('task_list')});
      final client = QtcloudWork(workspace: workspace, runner: runner);
      final tasks = await client.tasks();
      expect(tasks.length, 5);
      expect(tasks.first.workflow, 'compare-course-profile');
    });

    test('工作流详情读成模型', () async {
      final runner = FakeRunner({'learn-task-create': fixture('workflow_detail')});
      final client = QtcloudWork(workspace: workspace, runner: runner);
      final workflow = await client.workflow('learn-task-create');
      expect(workflow.steps.length, 5);
      expect(workflow.criteria.total, 18);
    });

    test('命令行说不行就抛，把它印的话带出来', () async {
      final runner = FakeRunner({'nope': '{"ok":false,"lines":["没有这条工作流：nope"]}'});
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
      final runner = FakeRunner({'--list': fixture('workflow_list')});
      final client = QtcloudWork(workspace: workspace, runner: runner);
      final workflows = await client.workflows();
      expect(workflows.length, 5);
      expect(workflows.map((w) => w.name), contains('devops-release'));
      expect(workflows.first.steps, 'outline、doc、test、code');
    });
  });
}
