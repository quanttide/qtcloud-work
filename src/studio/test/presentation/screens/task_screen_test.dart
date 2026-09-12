import 'package:flutter_test/flutter_test.dart';

import 'package:qtcloud_work_studio/infrastructure/envelope.dart';
import 'package:qtcloud_work_studio/domain/workspace.dart';
import 'package:qtcloud_work_studio/domain/task.dart';
import 'package:qtcloud_work_studio/presentation/screens/task_screen.dart';

import '../../support/fake_runner.dart';
import '../../support/widget.dart';

const _workspace = Workspace(
  root: '/w',
  data: '/w/data',
  workflows: '/w/flows',
);

void main() {
  final task = TaskDetail.fromData(
    TableResult.fromStdout(fixture('task_detail')).data,
  );

  testWidgets('左边对话、右边状态面板', (tester) async {
    await pumpScreen(
      tester,
      TaskScreen(
        client: fakeClient(),
        workspace: _workspace,
        task: task,
        onReload: () async {},
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('说目标，或者对流程提修改…'), findsOneWidget);
    expect(find.text('闸门（留给人）'), findsOneWidget);
    expect(find.text('走下一步'), findsOneWidget);
  });
}
