import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:quanttide_work/quanttide_work.dart' as qt;
import 'package:qtcloud_work_studio/screens/flow_screen.dart';
import 'package:qtcloud_work_studio/widgets/criteria_panel.dart';
import 'package:qtcloud_work_studio/widgets/step_chain.dart';

import '../support/fake_runner.dart';
import '../support/widget.dart';

final _workspace = qt.RunContext(
  root: '/w',
  data: '/w/data',
  workflows: '/w/flows',
);

void main() {
  final workflow = qt.Workflow.of('learn-task-create', fixturePayload('workflow_detail'));

  Widget screen() => FlowScreen(
    workflow: workflow,
    path: '/w/flows/learn-task-create.yaml',
    yaml: 'name: learn-task-create\n',
    workspace: _workspace,
    busy: false,
    onCreate: (_) {},
    onCheck: () async => (ok: true, lines: const ['定义没问题']),
  );

  testWidgets('默认是步骤态，点一个步骤看判据', (tester) async {
    await pumpScreen(tester, screen());
    expect(find.byType(StepChain), findsOneWidget);
    expect(find.byType(CriteriaPanel), findsOneWidget);
    await tester.tap(find.text('site'));
    await tester.pumpAndSettle();
    expect(find.text('5 条判据（机器判 4）'), findsOneWidget);
  });

  testWidgets('切到定义态换成原文那处', (tester) async {
    await pumpScreen(tester, screen());
    await tester.tap(find.text('定义'));
    await tester.pumpAndSettle();
    expect(find.byType(StepChain), findsNothing);
    expect(find.textContaining('learn-task-create.yaml'), findsOneWidget);
  });
}
