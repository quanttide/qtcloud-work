import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/presentation/app.dart';

import '../support/fake_runner.dart';

void main() {
  testWidgets('工作台起来就是任务页：顶栏 + 侧栏 + 任务', (tester) async {
    await tester.pumpWidget(
      QtcloudWorkStudioApp(client: fakeClient(), workspace: testWorkspace),
    );
    await tester.pumpAndSettle();

    expect(find.text('任务'), findsOneWidget);
    expect(find.text('流程'), findsOneWidget);
    expect(find.text('设置'), findsOneWidget);
    expect(find.text('量潮工作云工作台'), findsNothing); // 标题在窗口上，不在页面里
    expect(find.text('走完'), findsOneWidget);
  });

  testWidgets('切到流程页换成步骤链', (tester) async {
    await tester.pumpWidget(
      QtcloudWorkStudioApp(client: fakeClient(), workspace: testWorkspace),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.text('流程'));
    await tester.pumpAndSettle();
    expect(find.text('learn-task-create'), findsWidgets);
    expect(find.text('步骤'), findsOneWidget);
  });
}
