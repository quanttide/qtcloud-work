//! 状态真值表：进度与完结只推导，所以「records 的 step 序列 → 下一步」该有一张表。
//! 不再按 `·审` / `·判` 后缀投票——流水里一笔记一件事，过没过看 `is_succeeded`。
//!
//! 用例：一（起工单与走一步那件事的推演），用例：五（人记一笔）

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
fn records序列决定下一步() {
    /// 一格：名字、records 序列（步骤名、过没过）、期望的下一步（「走完」表示全部走过）。
    type Case = (&'static str, Vec<(&'static str, bool)>, &'static str);
    let cases: Vec<Case> = vec![
        ("什么都没记", vec![], "甲"),
        ("记过一笔", vec![("甲", true)], "乙"),
        ("这笔没过", vec![("甲", false)], "甲"),
        (
            "没过之后重走一笔过了",
            vec![("甲", false), ("甲", true)],
            "乙",
        ),
        (
            "后一步先记了（乱序）",
            vec![("丙", true), ("甲", true)],
            "乙",
        ),
        (
            "三步都过",
            vec![("甲", true), ("乙", true), ("丙", true)],
            "走完",
        ),
        ("名字不在定义里的记录不算数", vec![("查无此步", true)], "甲"),
    ];
    for (name, records, want) in cases {
        let fix = 三步();
        let workflow_id = fix.workflow_id("试一条");
        fix.order_with("试一条", &workflow_id, &records);
        let view = fix.run_ledger(&["order", "show", "试一条"]);
        let shown = view.crop();
        let want = if want == "走完" {
            "走完了：3 个步骤都过了".to_string()
        } else {
            format!("下一步：{want}")
        };
        assert!(shown.contains(&want), "{name}：期望「{want}」\n{shown}");
    }
}

// 用例：一（起工单与走一步那件事的推演）
#[test]
fn 状态里带进度条() {
    let fix = 三步();
    let seeded = fix.run_ledger(&["workflow", "create", "试一条", "--steps", "甲,乙,丙"]);
    assert!(seeded.ok(), "{}", seeded.crop());
    let workflow_id = fix.workflow_id("试一条");
    fix.order_with("试一条", &workflow_id, &[("甲", true)]);

    let shown = fix.run_ledger(&["order", "show", "试一条"]).crop();
    assert!(shown.contains("进度：1/3"), "进度该走过一格：\n{shown}");

    let listed = fix.run_ledger(&["order", "list"]).crop();
    assert!(
        listed.contains("[███░░░░░░░] 1/3"),
        "清单上的进度条该走过三格：\n{listed}"
    );
}
