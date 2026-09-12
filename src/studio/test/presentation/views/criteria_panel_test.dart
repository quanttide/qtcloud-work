import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/infrastructure/envelope.dart';
import 'package:qtcloud_work_studio/domain/workflow.dart';
import 'package:qtcloud_work_studio/presentation/views/criteria_panel.dart';

import '../../support/fake_runner.dart';

void main() {
  final workflow = WorkflowDetail.fromData(
    TableResult.fromStdout(fixture('workflow_detail')).data,
  );

  testWidgets('没选步骤时给一句话', (tester) async {
    await tester.pumpWidget(
      const MaterialApp(home: Scaffold(body: CriteriaPanel())),
    );
    expect(find.text('点一个步骤，看它谁做、判据有几条'), findsOneWidget);
  });

  testWidgets('选了就列出表头与逐条判据', (tester) async {
    final audit = workflow.steps.firstWhere((step) => step.name == 'audit');
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(body: CriteriaPanel(step: audit)),
      ),
    );
    expect(find.text('audit'), findsOneWidget);
    expect(find.text('4 条判据（机器判 3）'), findsOneWidget);
    // 逐条列出来：谁判 + 那句话（照定义文件里写的）
    expect(find.text('机器'), findsNWidgets(3));
    expect(find.text('人'), findsOneWidget);
    expect(find.text('站点版本与变更记录两处对齐'), findsOneWidget);
  });
}
