import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/models/table_result.dart';
import 'package:qtcloud_work_studio/models/workflow.dart';
import 'package:qtcloud_work_studio/views/step_chain.dart';

import '../support/fake_runner.dart';

void main() {
  final workflow = WorkflowDetail.fromResult(
    TableResult.fromStdout(fixture('workflow_detail')),
  );

  testWidgets('一步一个节点，副标题写执行者与判据条数', (tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: StepChain(workflow: workflow, onSelect: (_) {}),
        ),
      ),
    );
    for (final step in workflow.steps) {
      expect(find.text(step.name), findsOneWidget);
    }
    expect(
      find.text('AI 执行 · 4 条判据'),
      findsNWidgets(2),
    ); // profile 与 audit 都是 4 条
    expect(find.text('AI 执行 · 5 条判据'), findsOneWidget);
  });

  testWidgets('点节点把序号回给外面', (tester) async {
    int? picked;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: StepChain(
            workflow: workflow,
            onSelect: (index) => picked = index,
          ),
        ),
      ),
    );
    await tester.tap(find.text('audit'));
    expect(picked, 2);
  });
}
