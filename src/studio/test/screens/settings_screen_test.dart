import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/screens/settings_screen.dart';

import '../support/fixtures.dart';
import '../support/widget.dart';

void main() {
  testWidgets('三处位置都在，也能探活', (tester) async {
    await pumpScreen(
      tester,
      SettingsScreen(
        workspace: testWorkspace,
        onProbe: () async => const ['provider：无已部署'],
      ),
    );
    expect(find.text('工作区（--root）'), findsOneWidget);
    expect(find.text('数据仓（--data）'), findsOneWidget);
    expect(find.text('工作流目录（--workflows）'), findsOneWidget);
    expect(find.text(testWorkspace.workflows), findsOneWidget);

    await tester.tap(find.text('探活（health）'));
    await tester.pumpAndSettle();
    expect(find.textContaining('provider'), findsOneWidget);
  });
}
