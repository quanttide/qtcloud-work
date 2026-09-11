//! 导览：按用途把命令分组，列成一张人读得懂的清单。
//!
//! 与 `--help` 的分工：`--help` 是某个命令的选项全集（机器生成的用法），
//! `help` 是这份导览——先告诉你有哪些命令、各属于哪一组，再看细节。

use crate::outcome::Result;

/// 一组命令：分组名 + 组里每条的「名字、一句话」。
const GROUPS: &[(&str, &[(&str, &str)])] = &[
    (
        "工作区",
        &[
            ("find <名字> [--show]", "按名找文档"),
            ("catalog", "按资产种类列条目"),
            ("audit [--make]", "审计资产表与工作区"),
            ("material [<路径>…]", "材料的四字段与阶段"),
        ],
    ),
    (
        "工作流（定义侧）",
        &[
            ("workflow --list", "有哪些工作流"),
            ("workflow --new <名字> --steps 甲,乙", "写一条工作流"),
            ("workflow <名字>", "看步骤、谁执行、几条判据"),
            ("workflow <名字> --check", "核对判据里的路径与描述里的小节"),
            ("workflow <名字> --export <文件>", "原样带走一份"),
            ("workflow --import <文件> [--as 名字]", "导进来一份"),
        ],
    ),
    (
        "任务（执行侧）",
        &[
            ("task --list", "有哪些任务、下一步"),
            ("task --new <名字> --workflow <工作流>", "起一件任务"),
            ("task <名字>", "步骤状态与流水"),
            ("task <名字> --next", "走下一步"),
            ("task <名字> --done <步骤> [--note 一句话]", "人为地记一步"),
            ("task <名字> --journal <一段话>", "日志收叙事"),
        ],
    ),
    (
        "其他",
        &[("health", "探活已部署的 provider"), ("help", "这份导览")],
    ),
];

/// 按话题给一句要点；认不出的话题就说没有这条。
pub fn topic(name: &str) -> Option<Vec<String>> {
    let name = name.trim();
    for (group, items) in GROUPS {
        for (usage, what) in *items {
            if usage.split_whitespace().next() == Some(name) {
                return Some(vec![
                    format!("{usage}——{what}（{group}）"),
                    format!(
                        "细则看 `qtcloud-work {name} --help`，契约看 docs/api-references/{name}.md"
                    ),
                ]);
            }
        }
    }
    None
}

/// 导览：三处位置、分组命令、下一步。
pub fn guide() -> Result {
    let mut result = Result::new(true);
    result.lines = vec![
        "量潮工作云命令行——把知识工作做成可执行的编排。".to_string(),
        String::new(),
    ];
    result.columns = vec!["组".to_string(), "命令".to_string(), "做什么".to_string()];
    for (group, items) in GROUPS {
        result.lines.push((*group).to_string());
        for (usage, what) in *items {
            result.lines.push(format!("  {usage:42} {what}"));
            result
                .rows
                .push(vec![group.to_string(), usage.to_string(), what.to_string()]);
        }
        result.lines.push(String::new());
    }
    result.lines.push(
        "三处位置：--root 工作区 / --data 数据仓 / --workflows 工作流目录（缺省见 `--help`）。"
            .to_string(),
    );
    result.lines.push("话题：`qtcloud-work help <命令>` 看它一句话要点；`qtcloud-work <命令> --help` 看全部选项。".to_string());
    result
}
