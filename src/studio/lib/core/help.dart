import 'outcome.dart';

/// 导览：按用途把命令分组，列成一张人读得懂的清单。
///
/// 与 `--help` 的分工：`--help` 是某个命令的选项全集（机器生成的用法），
/// `help` 是这份导览——先告诉你有哪些命令、各属于哪一组，再看细节。
///
/// 这份数据与命令行那一份逐字相同（`src/help.rs` 的 GROUPS）。
class HelpItem {
  const HelpItem(this.usage, this.what);

  final String usage;
  final String what;
}

class HelpGroup {
  const HelpGroup(this.name, this.items);

  final String name;
  final List<HelpItem> items;
}

const List<HelpGroup> helpGroups = [
  HelpGroup('工作区', [
    HelpItem('find <名字> [--show]', '按名找文档'),
    HelpItem('catalog', '按资产种类列条目'),
    HelpItem('audit [--make]', '审计资产表与工作区'),
    HelpItem('material [<路径>…]', '材料的四字段与阶段'),
  ]),
  HelpGroup('工作流（定义侧）', [
    HelpItem('workflow --list', '有哪些工作流'),
    HelpItem('workflow --new <名字> --steps 甲,乙', '写一条工作流'),
    HelpItem('workflow <名字>', '看步骤、谁执行、几条判据'),
    HelpItem('workflow <名字> --check', '核对判据里的路径与描述里的小节'),
    HelpItem('workflow <名字> --export <文件>', '原样带走一份'),
    HelpItem('workflow --import <文件> [--as 名字]', '导进来一份'),
  ]),
  HelpGroup('任务（执行侧）', [
    HelpItem('task --list', '有哪些任务、下一步'),
    HelpItem('task --new <名字> --workflow <工作流>', '起一件任务'),
    HelpItem('task <名字>', '步骤状态与流水'),
    HelpItem('task <名字> --next', '走下一步'),
    HelpItem('task <名字> --done <步骤> [--note 一句话]', '人为地记一步'),
    HelpItem('task <名字> --journal <一段话>', '日志收叙事'),
  ]),
  HelpGroup('其他', [
    HelpItem('health', '探活已部署的 provider'),
    HelpItem('help', '这份导览'),
  ]),
];

/// 按话题给一句要点；认不出的话题就说没有这条。
List<String>? helpTopic(String name) {
  final wanted = name.trim();
  for (final group in helpGroups) {
    for (final item in group.items) {
      if (item.usage.split(RegExp(r'\s+')).first == wanted) {
        return [
          '${item.usage}——${item.what}（${group.name}）',
          '细则看 `qtcloud-work $wanted --help`，契约看 docs/api-references/$wanted.md',
        ];
      }
    }
  }
  return null;
}

/// 导览：三处位置、分组命令、下一步。
Outcome helpGuide() {
  final result = Outcome(true)
    ..lines = [
      '量潮工作云命令行——把知识工作做成可执行的编排。',
      '',
    ]
    ..columns = ['组', '命令', '做什么'];
  for (final group in helpGroups) {
    result.lines.add(group.name);
    for (final item in group.items) {
      result.lines.add('  ${item.usage.padRight(42)} ${item.what}');
      result.rows.add([group.name, item.usage, item.what]);
    }
    result.lines.add('');
  }
  result.lines.add('三处位置：--root 工作区 / --data 数据仓 / --workflows 工作流目录（缺省见 `--help`）。');
  result.lines.add('话题：`qtcloud-work help <命令>` 看它一句话要点；`qtcloud-work <命令> --help` 看全部选项。');
  return result;
}

/// `help` 那一支：给了话题说要点，没给就是导览。
Outcome helpOf(String? topic) {
  if (topic == null || topic.trim().isEmpty) return helpGuide();
  final lines = helpTopic(topic);
  if (lines == null) {
    return Outcome.failed(['没有这条命令：$topic（`qtcloud-work help` 看全部）']);
  }
  return Outcome(true, lines: lines);
}
