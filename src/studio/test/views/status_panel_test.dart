import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/quanttide_work.dart' as qt;
import 'package:qtcloud_work_studio/views/status_panel.dart';

import '../support/fixtures.dart';

void main() {
  final task = qt.Task.of(fixturePayload('task_detail'));
  final workflow = qt.Workflow.of(fixturePayload('workflow_detail'));

  Future<void> pump(WidgetTester tester, {bool busy = false}) {
    return tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: StatusPanel(
            task: task,
            workflow: workflow,
            artifacts: Map<String, String>.from(
              (fixtureData('task_detail')['artifacts'] as Map?) ?? const {},
            ),
            busy: busy,
            onNext: () {},
            onDone: () {},
            onJournal: () {},
          ),
        ),
      ),
    );
  }

  testWidgets('状态行：走完了、5 / 5、进度满', (tester) async {
    await pump(tester);
    expect(find.text('走完'), findsOneWidget);
    expect(find.text('5 / 5'), findsOneWidget);
    expect(find.text('没有下一步'), findsOneWidget);
    final bar = tester.widget<LinearProgressIndicator>(
      find.byType(LinearProgressIndicator),
    );
    expect(bar.value, 1.0);
  });

  testWidgets('闸门看的是判据里的人：audit 那条挂着人签', (tester) async {
    await pump(tester);
    expect(find.text('闸门（留给人）'), findsOneWidget);
    expect(find.textContaining('audit'), findsWidgets);
    expect(find.textContaining('待人放行'), findsOneWidget);
  });

  testWidgets('产物三个落点都在', (tester) async {
    await pump(tester);
    expect(
      find.textContaining('artifacts/report/learn-task-create.md'),
      findsOneWidget,
    );
    expect(
      find.textContaining('artifacts/journal/learn-task-create.md'),
      findsOneWidget,
    );
    expect(find.textContaining('tasks/learn-task-create.yaml'), findsOneWidget);
  });

  testWidgets('流水带 ·判 后缀', (tester) async {
    await pump(tester);
    expect(find.textContaining('audit·判'), findsOneWidget);
    expect(find.textContaining('conclude'), findsWidgets);
  });

  testWidgets('走完了就不给再走', (tester) async {
    await pump(tester);
    final button = tester.widget<FilledButton>(
      find.ancestor(of: find.text('走下一步'), matching: find.byType(FilledButton)),
    );
    expect(button.onPressed, isNull);
  });
}
