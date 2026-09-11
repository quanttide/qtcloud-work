//! 场景：多步都交给 AI，依次走完；运行上下文随任务记着，后续命令不写
//! `--workflows` 也认得出定义在哪。
//!

mod support;

use support::Fixture;

// 用例：三
#[test]
fn scenario_three_ai_steps_with_recorded_context() {
    let fix = Fixture::new("compare");
    fix.workflow(
        "compare-course-profile",
        "name: compare-course-profile\ndescription: 比对两份课程档案\nsteps:\n- name: locate\n  description: 找齐两边档案\n  criteria:\n  - executor: rule\n    description: 两边档案清单在\n    path: locate.md\n- name: compare\n  description: 逐项对照\n  criteria:\n  - executor: rule\n    description: 对照写下来了\n    path: compare.md\n- name: conclude\n  description: 写下处置建议\n  criteria:\n  - executor: rule\n    description: 结论在\n    path: conclude.md\n",
    );
    fix.pi("for f in locate.md compare.md conclude.md; do printf '内容\\n' > \"$f\"; done\necho 通过");
    fix.run_full(true, &["task", "--new", "compare-course-profile", "--workflow", "compare-course-profile"]);

    let record = fix.task_yaml("compare-course-profile");
    assert!(record.contains("workflows:"), "运行上下文里的工作流目录没记:\n{record}");

    for step in ["locate", "compare", "conclude"] {
        let stepped = fix.run_recorded(&["task", "compare-course-profile", "--next"]);
        assert!(stepped.ok(), "--next 走 {step} 没跑通: {}", stepped.crop());
    }
    assert!(
        fix.task_yaml("compare-course-profile").matches("ok: true").count() >= 3,
        "三步都该记一笔"
    );

    let status = fix.run_recorded(&["task", "compare-course-profile"]);
    assert!(status.ok(), "不给 --workflows 就看不了任务: {}", status.crop());
}
