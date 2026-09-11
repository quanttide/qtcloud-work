import 'package:flutter/material.dart';

import 'app.dart';
import 'cli/qtcloud_work.dart';
import 'models/workspace.dart';

/// 构建时注入的版本号（`flutter build web --release --dart-define=APP_VERSION=...`）。
const String appVersion = String.fromEnvironment(
  'APP_VERSION',
  defaultValue: 'dev',
);

void main() {
  final workspace = Workspace.fromEnvironment();
  runApp(
    QtcloudWorkStudioApp(
      client: QtcloudWork(workspace: workspace),
      workspace: workspace,
    ),
  );
}
