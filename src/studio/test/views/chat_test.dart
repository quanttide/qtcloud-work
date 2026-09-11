import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/views/chat.dart';

void main() {
  testWidgets('消息摆出来，草案卡也是消息', (tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: ChatPanel(
            placeholder: '说目标…',
            messages: const [
              ChatMessage(text: '给命令行走一次预发布。', fromUser: true),
              ChatMessage(text: '先出流程，确认后执行。', draft: true),
            ],
          ),
        ),
      ),
    );
    expect(find.text('给命令行走一次预发布。'), findsOneWidget);
    expect(find.text('先出流程，确认后执行。'), findsOneWidget);
    expect(find.text('说目标…'), findsOneWidget);
  });

  testWidgets('命令行还没有对话这一路：默认不可发', (tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: ChatPanel(messages: [], placeholder: '说目标…'),
        ),
      ),
    );
    final button = tester.widget<FilledButton>(find.byType(FilledButton));
    expect(button.onPressed, isNull);
  });
}
