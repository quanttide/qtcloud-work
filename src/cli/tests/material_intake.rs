//! 场景：语境条目粗加工进材料——几步依次走完，粗加工落到 `materials/<分类>/index.md`，
//! 两道 human 闸门挂进待拍板清单（闸门不落字段，由定义加流水推导）。
//!

mod common;

use common::Fixture;

// 用例：四
#[test]
fn context_entries_into_material() {
    let fix = Fixture::new("material");
    fix.workflow(
        "context-to-profile",
        "name: context-to-profile\ndescription: 语境条目粗加工进材料库\nsteps:\n- name: pull\n  description: 拉语境并把条目清单写进报告\n  criteria:\n  - executor: rule\n    description: 条目清单在\n    path: pull.md\n- name: classify\n  description: 逐条认分类\n  criteria:\n  - executor: rule\n    description: 分类写下来了\n    path: classify.md\n  - executor: human\n    description: 分类裁决\n- name: coarsen\n  description: 粗加工写进 materials/<分类>/index.md\n  criteria:\n  - executor: rule\n    description: 材料格在\n    path: materials/课程/index.md\n- name: move-out\n  description: 把已迁出的条目从语境删掉\n  criteria:\n  - executor: rule\n    description: 迁出记录在\n    path: move-out.md\n- name: commit\n  description: 分层提交推送再回工作区更新指针\n  criteria:\n  - executor: rule\n    description: 提交完了\n    path: commit.md\n  - executor: human\n    description: 创始人点头\n",
    );
    fix.pi(
        "printf '清单\\n' > pull.md\nprintf '分类\\n' > classify.md\nmkdir -p materials/课程\nprintf '粗加工\\n' > materials/课程/index.md\nprintf '迁出\\n' > move-out.md\nprintf '提交\\n' > commit.md\necho 通过",
    );
    fix.run_full(
        true,
        &[
            "order",
            "create",
            "context-to-profile",
            "--workflow",
            "context-to-profile",
        ],
    );

    // 机器路径走一步、人的闸门放行一步：classify 与 commit 各有一道闸。
    let stepped = fix.run_ledger(&["order", "next", "context-to-profile"]);
    assert!(stepped.ok(), "next 走 pull 没跑通: {}", stepped.crop());
    let stepped = fix.run_ledger(&["order", "next", "context-to-profile"]);
    assert!(stepped.ok(), "next 走 classify 没跑通: {}", stepped.crop());
    let gate = fix.run_ledger(&["order", "show", "context-to-profile"]);
    assert!(
        gate.crop().contains("闸门：classify：分类裁决"),
        "human 判据该进待拍板清单: {}",
        gate.crop()
    );
    let done = fix.run_ledger(&[
        "order",
        "done",
        "context-to-profile",
        "classify",
        "--note",
        "人放行",
    ]);
    assert!(done.ok(), "done 放行 classify 没跑通: {}", done.crop());

    for step in ["coarsen", "move-out", "commit"] {
        let stepped = fix.run_ledger(&["order", "next", "context-to-profile"]);
        assert!(stepped.ok(), "next 走 {step} 没跑通: {}", stepped.crop());
    }
    let done = fix.run_ledger(&[
        "order",
        "done",
        "context-to-profile",
        "commit",
        "--note",
        "创始人点头",
    ]);
    assert!(done.ok(), "done 放行 commit 没跑通: {}", done.crop());

    let order = fix.order_yaml("context-to-profile");
    assert!(
        order.matches("is_succeeded: true").count() >= 5,
        "五步都该记一笔:\n{order}"
    );
    assert!(
        fix.root.join("materials/课程/index.md").is_file(),
        "粗加工的材料格没落盘"
    );

    // 都走完了：待拍板清单空了，下一步是走完。
    let after = fix.run_ledger(&["order", "show", "context-to-profile"]);
    assert!(
        !after.crop().contains("闸门："),
        "放行完的闸门不该再挂: {}",
        after.crop()
    );
    assert!(
        after.crop().contains("走完了"),
        "五步都过该走完: {}",
        after.crop()
    );
}
