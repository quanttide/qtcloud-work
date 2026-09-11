import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

/// 单看一屏或一块时，套一层最小的 Material 外壳。
Widget wrapScreen(Widget child) => MaterialApp(home: Scaffold(body: child));

/// 给测试一块够大的画布：桌面窗口那么宽，免得被窄屏的溢出挡住。
Future<void> pumpScreen(WidgetTester tester, Widget child) async {
  tester.view.physicalSize = const Size(1600, 1000);
  tester.view.devicePixelRatio = 1.0;
  addTearDown(tester.view.resetPhysicalSize);
  addTearDown(tester.view.resetDevicePixelRatio);
  await tester.pumpWidget(wrapScreen(child));
}
