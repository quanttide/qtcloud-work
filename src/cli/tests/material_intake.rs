//! 场景：语境条目粗加工进材料——几步依次走完，粗加工落到 `materials/<分类>/index.md`，
//! 两道 human 闸门挂进报告。
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
    fix.run_full(true, &["task", "--new", "context-to-profile", "--workflow", "context-to-profile"]);

    for step in ["pull", "classify", "coarsen", "move-out", "commit"] {
        let stepped = fix.run_recorded(&["task", "context-to-profile", "--next"]);
        assert!(stepped.ok(), "--next 走 {step} 没跑通: {}", stepped.crop());
    }
    assert!(
        fix.task_yaml("context-to-profile").matches("ok: true").count() >= 5,
        "五步都该记一笔"
    );
    assert!(fix.root.join("materials/课程/index.md").is_file(), "粗加工的材料格没落盘");
    let report = fix.report("context-to-profile");
    assert!(
        report.contains("分类裁决") && report.contains("创始人点头"),
        "两道 human 闸门该挂进报告:\n{report}"
    );
}
