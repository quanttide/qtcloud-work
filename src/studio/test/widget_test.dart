import 'package:flutter_test/flutter_test.dart';

import 'package:qtcloud_work_studio/main.dart';

void main() {
  testWidgets('首页展示工作台标题与版本', (WidgetTester tester) async {
    await tester.pumpWidget(const QtcloudWorkStudioApp());

    expect(find.text('知识工作云工作台'), findsOneWidget);
    expect(find.text('版本 dev'), findsOneWidget);
  });
}
