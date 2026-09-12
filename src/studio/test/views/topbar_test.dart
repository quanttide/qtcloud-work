import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/views/topbar.dart';
import 'package:qtcloud_work_studio/repositories/local/run_context.dart';


void main() {
  testWidgets('只有工作区，并说明它是 --root', (tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Topbar(
            workspace: const RunContext(
              root: '/home/iguo/repos/quanttide/domains/quanttide-work',
              data: 'data/context/qtcloud-work',
              workflows: 'data/profile/quanttide/workflows',
            ),
            onRefresh: () {},
          ),
        ),
      ),
    );
    expect(find.text('--root'), findsNothing);
    expect(find.textContaining('--root'), findsOneWidget);
    expect(find.text('quanttide-work'), findsOneWidget);
  });
}
