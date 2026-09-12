import 'package:flutter_test/flutter_test.dart';

import 'package:quanttide_work/quanttide_work.dart' as qt;
import 'package:qtcloud_work_studio/screens/task_screen.dart';

import '../support/fake_runner.dart';
import '../support/widget.dart';

final _workspace = qt.RunContext(
  root: '/w',
  data: '/w/data',
  workflows: '/w/flows',
);

void main() {
  final task = qt.Task.of('learn-task-create', fixturePayload('task_detail'));
  final workflow = qt.Workflow.of('learn-task-create', fixturePayload('workflow_detail'));

  testWidgets('左边对话、右边状态面板', (tester) async {
    await pumpScreen(
      tester,
      TaskScreen(
        task: task,
        workflow: workflow,
        artifacts: Map<String, String>.from(
          (fixtureData('task_detail')['artifacts'] as Map?) ?? const {},
        ),
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
