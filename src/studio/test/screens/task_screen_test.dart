import 'package:flutter_test/flutter_test.dart';

import 'package:qtcloud_work_studio/quanttide_work.dart' as qt;
import 'package:qtcloud_work_studio/screens/task_screen.dart';

import '../support/fixtures.dart';
import '../support/widget.dart';
import 'package:qtcloud_work_studio/repositories/local/run_context.dart';

final _workspace = RunContext(
  root: '/w',
  data: '/w/data',
  workflows: '/w/flows',
);

void main() {
  final task = qt.Task.of(fixturePayload('task_detail'));
  final workflow = qt.Workflow.of(fixturePayload('workflow_detail'));

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
