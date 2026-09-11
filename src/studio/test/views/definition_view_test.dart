import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/views/definition_view.dart';

void main() {
  testWidgets('定义态给位置，并说明命令行还没给原文', (tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: DefinitionView(
            workflow: '/w/data/profile/quanttide/workflows/devops-release.yaml',
          ),
        ),
      ),
    );
    expect(find.textContaining('devops-release.yaml'), findsOneWidget);
    expect(find.textContaining('命令行现在不给原文'), findsOneWidget);
  });
}
