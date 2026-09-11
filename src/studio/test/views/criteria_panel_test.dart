import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/models/table_result.dart';
import 'package:qtcloud_work_studio/models/workflow.dart';
import 'package:qtcloud_work_studio/views/criteria_panel.dart';

import '../support/fake_runner.dart';

void main() {
  final workflow = WorkflowDetail.fromResult(
    TableResult.fromStdout(fixture('workflow_detail')),
  );

  testWidgets('没选步骤时给一句话', (tester) async {
    await tester.pumpWidget(
      const MaterialApp(home: Scaffold(body: CriteriaPanel())),
    );
    expect(find.text('点一个步骤，看它谁做、判据有几条'), findsOneWidget);
  });

  testWidgets('选了就列出谁判、几条', (tester) async {
    final audit = workflow.steps.firstWhere((step) => step.name == 'audit');
    await tester.pumpWidget(
      MaterialApp(home: Scaffold(body: CriteriaPanel(step: audit))),
    );
    expect(find.text('audit'), findsOneWidget);
    expect(find.text('4 条判据（机器判 3）'), findsOneWidget);
    expect(find.text('机器　3 条'), findsOneWidget);
    expect(find.text('人　1 条'), findsOneWidget);
  });
}
