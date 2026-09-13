//! 状态真值表：状态从流水推出来，所以「流水序列 → 下一步」该有一张表。
//!
//! 用例：一（起任务与走一步那件事的推演），用例：五（人为地记一步）

mod common;

use common::Fixture;

fn 三步() -> Fixture {
    let fix = Fixture::new("state");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 甲\n  description: 第一步\n- name: 乙\n  description: 第二步\n- name: 丙\n  description: 第三步\n",
    );
    fix
}

#[test]
fn 流水序列决定下一步() {
    /// 一格：名字、流水序列、期望的下一步（「走完」表示全部走过）。
    type Case = (&'static str, Vec<(&'static str, bool)>, &'static str);
    let cases: Vec<Case> = vec![
        ("什么都没记", vec![], "甲"),
        ("只执行过", vec![("甲", true)], "乙"),
        (
            "执行过了、审查判 ✗",
            vec![("甲", true), ("甲·审", false)],
            "甲",
        ),
        (
            "审查 ✗ 后重走一次都 ok",
            vec![
                ("甲", true),
                ("甲·审", false),
                ("甲", true),
                ("甲·审", true),
            ],
            "乙",
        ),
        ("执行本身就没成", vec![("甲", false)], "甲"),
        (
            "后一步先记了（乱序）",
            vec![("丙", true), ("甲", true)],
            "乙",
        ),
        (
            "三步都过",
            vec![
                ("甲", true),
                ("甲·审", true),
                ("乙", true),
                ("甲·审", true),
                ("丙", true),
            ],
            "走完",
        ),
        ("名字不在定义里的流水不算数", vec![("查无此步", true)], "甲"),
    ];
    for (name, log, want) in cases {
        let fix = 三步();
        fix.task_with("试一条", "试一条", &log);
        let view = fix.run_recorded(&["task", "试一条"]);
        let shown = view.crop();
        let want = if want == "走完" {
            "都走过了".to_string()
        } else {
            format!("下一步：{want}")
        };
        assert!(shown.contains(&want), "{name}：期望「{want}」\n{shown}");
    }
}

// 用例：一（起任务与走一步那件事的推演）
#[test]
fn 状态里带进度条() {
    let fix = 三步();
    fix.task_with("试一条", "试一条", &[("甲", true)]);
    let shown = fix.run_recorded(&["task", "试一条"]).crop();
    assert!(
        shown.contains("进度：[███░░░░░░░] 1/3"),
        "进度条该走过一格：\n{shown}"
    );

    let done = 三步();
    done.task_with(
        "试一条",
        "试一条",
        &[("甲", true), ("乙", true), ("丙", true)],
    );
    let full = done.run_recorded(&["task", "试一条"]).crop();
    assert!(
        full.contains("进度：[██████████] 3/3"),
        "走完了该满格：\n{full}"
    );
}
