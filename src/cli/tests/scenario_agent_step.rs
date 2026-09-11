//! 场景：走一步（agent）——交给 `pi` 干活，回来核 rule 判据、记一笔、写报告；
//! `pi` 没跑成那一次不算过。
//!

mod support;

use support::{Fixture, 冒烟工作流};

// 用例：一
#[test]
fn scenario_take_one_agent_step_through_pi() {
    let fix = Fixture::new("step");
    fix.workflow("AI冒烟", 冒烟工作流);
    fix.pi("printf '你好\\n' > 问候.md\necho 通过");
    fix.run_full(true, &["task", "--new", "AI冒烟", "--workflow", "AI冒烟"]);

    let stepped = fix.run_recorded(&["task", "AI冒烟", "--next"]);
    assert!(stepped.ok(), "--next 走一步没跑通: {}", stepped.crop());
    let record = fix.task_yaml("AI冒烟");
    assert!(record.contains("step: 问候"), "流水没记这一步:\n{record}");
    assert!(record.contains("ok: true"), "这一步没算过:\n{record}");
    assert!(fix.report("AI冒烟").contains("问候"), "报告没写这一步");

    // 同类场景的另一半：pi 失败 → 这一步不算过，流水留 ✗。
    let broken = Fixture::new("step-fail");
    broken.workflow("AI冒烟", 冒烟工作流);
    broken.pi("echo 我没干成 >&2\nexit 1");
    broken.run_full(true, &["task", "--new", "AI冒烟", "--workflow", "AI冒烟"]);
    broken.run_recorded(&["task", "AI冒烟", "--next"]);
    let failed = broken.task_yaml("AI冒烟");
    assert!(failed.contains("ok: false"), "AI 没跑成不该算过:\n{failed}");
}
