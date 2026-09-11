import 'package:flutter/material.dart';

/// 构建时注入的版本号（`flutter build web --dart-define=APP_VERSION=...`）
const String appVersion = String.fromEnvironment('APP_VERSION', defaultValue: 'dev');

void main() {
  runApp(const QtcloudWorkStudioApp());
}

class QtcloudWorkStudioApp extends StatelessWidget {
  const QtcloudWorkStudioApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: '量潮知识工作云 · 工作台',
      theme: ThemeData(
        colorSchemeSeed: const Color(0xFF0F766E),
        useMaterial3: true,
      ),
      home: const HomePage(),
    );
  }
}

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Scaffold(
      appBar: AppBar(
        title: const Text('量潮知识工作云 · 工作台'),
        backgroundColor: theme.colorScheme.primaryContainer,
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text('知识工作云工作台', style: theme.textTheme.headlineMedium),
            const SizedBox(height: 8),
            Text('版本 $appVersion', style: theme.textTheme.bodyMedium),
          ],
        ),
      ),
    );
  }
}
