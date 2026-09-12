import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/widgets/topbar.dart';
import 'package:quanttide_work/quanttide_work.dart' as qt;


void main() {
  testWidgets('只有工作区，并说明它是 --root', (tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Topbar(
            workspace: const qt.RunContext(
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
