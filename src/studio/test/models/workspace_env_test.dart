import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/models/workspace.dart';

void main() {
  test('不带参数时读的是真实进程环境', () {
    final workspace = Workspace.fromEnvironment();
    final env = Platform.environment;
    expect(workspace.root, env['QTCLOUD_WORK_ROOT'] ?? '.');
    expect(
      workspace.data,
      env['QTCLOUD_WORK_DATA'] ?? 'data/context/qtcloud-work',
    );
  });

  test('给了表就用表里的，不看环境', () {
    final workspace = Workspace.fromEnvironment(const {
      'QTCLOUD_WORK_ROOT': '/w',
      'QTCLOUD_WORK_DATA': '/w/data',
      'QTCLOUD_WORK_WORKFLOWS': '/w/flows',
    });
    expect(workspace.root, '/w');
    expect(workspace.data, '/w/data');
    expect(workspace.workflows, '/w/flows');
  });
}
