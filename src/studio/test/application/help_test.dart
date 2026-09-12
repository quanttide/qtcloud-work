import 'package:flutter_test/flutter_test.dart';
import 'package:qtcloud_work_studio/application/help.dart';

void main() {
  test('导览按四组列，行是「组 / 命令 / 做什么」', () {
    final guide = helpGuide();
    expect(guide.ok, isTrue);
    expect(guide.columns, ['组', '命令', '做什么']);
    expect(guide.lines.first, '量潮工作云命令行——把知识工作做成可执行的编排。');
    expect(guide.lines, contains('工作区'));
    expect(guide.lines, contains('工作流（定义侧）'));
    expect(guide.lines, contains('任务（执行侧）'));
    expect(guide.lines, contains('其他'));
    expect(guide.lines.last, contains('话题：`qtcloud-work help <命令>`'));
    expect(guide.rows.length, 4 + 6 + 6 + 2);
  });

  test('命令那一列宽 42，做了什么从第 45 格起', () {
    final guide = helpGuide();
    final line = guide.lines.firstWhere((l) => l.contains('find <名字>'));
    expect(line.indexOf('按名找文档'), 45);
  });

  test('给了话题就说那一条的要点', () {
    final lines = helpTopic('task');
    expect(lines, isNotNull);
    expect(lines!.first, 'task --list——有哪些任务、下一步（任务（执行侧））');
    expect(lines.last, contains('docs/api-references/task.md'));
  });

  test('认不出的话题就说没有这条', () {
    expect(helpTopic('nope'), isNull);
    final result = helpOf('nope');
    expect(result.ok, isFalse);
    expect(result.lines.first, '没有这条命令：nope（`qtcloud-work help` 看全部）');
  });

  test('不给话题就是导览', () {
    expect(helpOf(null).rows.length, helpGuide().rows.length);
    expect(helpOf('  ').rows.length, helpGuide().rows.length);
  });
}
