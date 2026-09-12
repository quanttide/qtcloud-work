import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/core/definition.dart';

/// 工作流定义的形状。
///
/// 出处是规格：`docs/specification/process/workflow.md`·语法。
void main() {
  group('工作流的形状', () {
    // 规范：docs/specification/process/workflow.md·语法
    // 「顶层三个字段：name、description 与 steps」
    test('顶层字段就这三个', () {
      expect(topFields, ['name', 'description', 'steps']);
    });
  });
}
