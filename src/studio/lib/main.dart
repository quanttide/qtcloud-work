import 'package:flutter/material.dart';

import 'repositories/client.dart';
import 'repositories/local/environment.dart';
import 'app.dart';

/// 构建时注入的版本号（`flutter build web --release --dart-define=APP_VERSION=...`）。
const String appVersion = String.fromEnvironment(
  'APP_VERSION',
  defaultValue: 'dev',
);

void main() {
  final workspace = workspaceFromEnvironment();
  runApp(
    QtcloudWorkStudioApp(
      client: QtcloudWork(workspace: workspace),
      workspace: workspace,
    ),
  );
}
