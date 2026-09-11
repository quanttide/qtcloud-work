import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/cli/qtcloud_work.dart';
import 'package:qtcloud_work_studio/main.dart';
import 'package:qtcloud_work_studio/models/workspace.dart';

import 'cli/qtcloud_work_test.dart' show RecordingRunner, fixture;

const workspace = Workspace(root: '/w', data: '/w/data', workflows: '/w/workflows');

void main() {
  testWidgets('任务页把步骤、产物与流水摆出来', (tester) async {
    // 顺序有用：列表那条先匹配，其余 `task <名字>` 都给详情夹具
    final runner = RecordingRunner({
      'task --list': fixture('task_list'),
      'task ': fixture('task_detail'),
    });
    final client = QtcloudWork(workspace: workspace, runner: runner);
    await tester.pumpWidget(
      QtcloudWorkStudioApp(client: client, workspace: workspace),
    );
    await tester.pumpAndSettle();

    // 产物与流水在滚动区下面，模型层已经逐字段核过；这里只看首屏
    expect(find.text('量潮工作云工作台'), findsOneWidget);
    expect(find.text('learn-task-create'), findsWidgets);
    expect(find.text('5 个步骤都走过了'), findsOneWidget);
    expect(find.text('profile'), findsWidgets);
  });
}
