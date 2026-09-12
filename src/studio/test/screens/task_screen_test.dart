import 'package:flutter_test/flutter_test.dart';

import 'package:qtcloud_work_studio/models/workspace.dart';
import 'package:qtcloud_work_studio/models/task.dart';
import 'package:qtcloud_work_studio/models/workflow.dart';
import 'package:qtcloud_work_studio/screens/task_screen.dart';

import '../support/fake_runner.dart';
import '../support/widget.dart';

const _workspace = Workspace(
  root: '/w',
  data: '/w/data',
  workflows: '/w/flows',
);

void main() {
  final task = TaskDetail.fromData(fixtureData('task_detail'));
  final workflow = WorkflowDetail.fromData(fixtureData('workflow_detail'));

  testWidgets('左边对话、右边状态面板', (tester) async {
    await pumpScreen(
      tester,
      TaskScreen(
        task: task,
        workflow: workflow,
        workspace: _workspace,
        busy: false,
        onNext: () {},
        onDone: () {},
        onJournal: (_) {},
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('说目标，或者对流程提修改…'), findsOneWidget);
    expect(find.text('闸门（留给人）'), findsOneWidget);
    expect(find.text('走下一步'), findsOneWidget);
  });
}
