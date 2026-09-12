import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/widgets/chat.dart';

void main() {
  testWidgets('开场的话摆出来，草案卡也是消息', (tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: ChatPanel(
            placeholder: '说目标…',
            workingDir: '/w',
            opening: const [
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

  testWidgets('发一句话，pi 的话贴回来', (tester) async {
    String? gotPrompt;
    String? gotDir;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: ChatPanel(
            placeholder: '说目标…',
            workingDir: '/w',
            brief: '背景：看的是这条流程',
            ask: (prompt, cwd) async {
              gotPrompt = prompt;
              gotDir = cwd;
              return (ran: true, out: '照这个走：甲、乙、丙');
            },
          ),
        ),
      ),
    );
    await tester.enterText(find.byType(TextField), '把流程改成三步');
    await tester.tap(find.byType(FilledButton));
    await tester.pumpAndSettle();

    expect(find.text('把流程改成三步'), findsOneWidget);
    expect(find.text('照这个走：甲、乙、丙'), findsOneWidget);
    // 背景随话一起递进去，pi 才知道在看什么；当前目录是工作区
    expect(gotPrompt, contains('背景：看的是这条流程'));
    expect(gotPrompt, contains('把流程改成三步'));
    expect(gotDir, '/w');
  });

  testWidgets('pi 没跑成也照样说清楚', (tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: ChatPanel(
            placeholder: '说目标…',
            workingDir: '/w',
            ask: (prompt, cwd) async => (ran: false, out: '没找到 pi'),
          ),
        ),
      ),
    );
    await tester.enterText(find.byType(TextField), '在吗');
    await tester.tap(find.byType(FilledButton));
    await tester.pumpAndSettle();
    expect(find.text('没跑成：没找到 pi'), findsOneWidget);
  });
}
