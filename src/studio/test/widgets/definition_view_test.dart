import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/widgets/definition_view.dart';

void main() {
  testWidgets('定义态给原文，也标出它是哪个文件', (tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: DefinitionView(
            workflow: 'name: devops-release\nsteps:\n  - name: version\n',
            path: '/w/data/profile/quanttide/workflows/devops-release.yaml',
          ),
        ),
      ),
    );
    expect(find.textContaining('devops-release.yaml'), findsOneWidget);
    expect(find.textContaining('name: devops-release'), findsOneWidget);
  });
}
