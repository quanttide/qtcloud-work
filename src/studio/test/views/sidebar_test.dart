import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/views/sidebar.dart';

void main() {
  Future<void> pump(WidgetTester tester, {required void Function(String) onSelect}) {
    return tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Row(
            children: [
              Sidebar(current: 'task', onSelect: onSelect),
            ],
          ),
        ),
      ),
    );
  }

  testWidgets('三项各两字', (tester) async {
    await pump(tester, onSelect: (_) {});
    expect(find.text('任务'), findsOneWidget);
    expect(find.text('流程'), findsOneWidget);
    expect(find.text('设置'), findsOneWidget);
  });

  testWidgets('点一下回话给外面', (tester) async {
    String? picked;
    await pump(tester, onSelect: (key) => picked = key);
    await tester.tap(find.text('流程'));
    expect(picked, 'flow');
  });
}
