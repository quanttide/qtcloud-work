//! 场景：几步 AI 依次走完——比对两份课程档案那份真事：开单、逐站走、走到完结。
//! 位置装载的纪律在 `run_context`，这里只看行程本身。

mod common;

use common::Fixture;

// 用例：三
#[test]
fn 三步依次走完() {
    let fix = Fixture::new("flow");
    fix.workflow(
        "compare-course-profile",
        "name: compare-course-profile\ndescription: 比对两份课程档案\nsteps:\n- name: locate\n  description: 找齐两边档案\n  criteria:\n  - executor: rule\n    description: 两边档案清单在\n    path: locate.md\n- name: compare\n  description: 逐项对照\n  criteria:\n  - executor: rule\n    description: 对照写下来了\n    path: compare.md\n- name: conclude\n  description: 写下处置建议\n  criteria:\n  - executor: rule\n    description: 结论在\n    path: conclude.md\n",
    );
    fix.pi(
        "for f in locate.md compare.md conclude.md; do printf '内容\\n' > \"$f\"; done\necho 通过",
    );
    fix.run_full(
        true,
        &[
            "order",
            "create",
            "compare-course-profile",
            "--workflow",
            "compare-course-profile",
        ],
    );

    for step in ["locate", "compare", "conclude"] {
        let stepped = fix.run_ledger(&["order", "next", "compare-course-profile"]);
        assert!(stepped.ok(), "next 走 {step} 没跑通: {}", stepped.crop());
        let order = fix.order_yaml("compare-course-profile");
        assert!(
            order.contains(&format!("step: {step}\n")),
            "{step} 该记了一笔:\n{order}"
        );
    }
    let after = fix.run_ledger(&["order", "show", "compare-course-profile"]);
    assert!(
        after.crop().contains("走完了"),
        "三步都过该走完: {}",
        after.crop()
    );
}
