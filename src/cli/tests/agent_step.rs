//! 场景：走一步（agent）——交给 `pi` 干活，回来核 rule 判据、记一笔、写报告；
//! `pi` 没跑成那一次不算过。
//!

mod common;

use common::{Fixture, 冒烟工作流};

// 用例：一
#[test]
fn take_one_agent_step_through_pi() {
    let fix = Fixture::new("step");
    fix.workflow("AI冒烟", 冒烟工作流);
    fix.pi("printf '你好\\n' > 问候.md\necho 通过");
    fix.run_full(true, &["task", "--new", "AI冒烟", "--workflow", "AI冒烟"]);

    let stepped = fix.run_recorded(&["task", "AI冒烟", "--next"]);
    assert!(stepped.ok(), "--next 走一步没跑通: {}", stepped.crop());
    let record = fix.task_yaml("AI冒烟");
    assert!(record.contains("step: 问候"), "流水没记这一步:\n{record}");
    assert!(record.contains("ok: true"), "这一步没算过:\n{record}");
    assert!(
        !fix.data.join("artifacts/report/AI冒烟.md").exists(),
        "程序不写产物：这一步的报告该由写它的人来写"
    );

    // 同类场景的另一半：pi 失败 → 这一步不算过，流水留 ✗。
    let broken = Fixture::new("step-fail");
    broken.workflow("AI冒烟", 冒烟工作流);
    broken.pi("echo 我没干成 >&2\nexit 1");
    broken.run_full(true, &["task", "--new", "AI冒烟", "--workflow", "AI冒烟"]);
    broken.run_recorded(&["task", "AI冒烟", "--next"]);
    let failed = broken.task_yaml("AI冒烟");
    assert!(failed.contains("ok: false"), "AI 没跑成不该算过:\n{failed}");

    // 同类场景的第三半：执行跑成了，但审查判 ✗ —— 这一步仍不算走过，下一步还是它。
    let judged_out = Fixture::new("step-review-fail");
    judged_out.workflow(
        "AI冒烟",
        "name: AI冒烟\ndescription: 冒烟\nsteps:\n- name: 问候\n  description: 写一行中文问候\n  criteria:\n  - executor: rule\n    description: 问候落在\n    path: 问候.md\n  - executor: agent\n    description: 问候是中文\n",
    );
    judged_out.pi("printf '你好\\n' > 问候.md\necho 不通过");
    judged_out.run_full(true, &["task", "--new", "AI冒烟", "--workflow", "AI冒烟"]);
    let stepped = judged_out.run_recorded(&["task", "AI冒烟", "--next"]);
    assert!(
        !stepped.ok(),
        "审查判 ✗ 时这一步不该算过: {}",
        stepped.crop()
    );
    let view = judged_out.run_recorded(&["task", "AI冒烟"]);
    assert!(
        view.crop().contains("下一步：问候"),
        "这一步没过，下一步还是它: {}",
        view.crop()
    );

    // 同类场景的第四半：重走一次、这次审查也过 —— 算走过，上一笔失败不再压着它。
    judged_out.pi("printf '你好\\n' > 问候.md\necho 通过");
    let again = judged_out.run_recorded(&["task", "AI冒烟", "--next"]);
    assert!(again.ok(), "重走一次该过: {}", again.crop());
    let after = judged_out.run_recorded(&["task", "AI冒烟"]);
    assert!(
        !after.crop().contains("下一步：问候"),
        "重走一次都 ok 就该算走过: {}",
        after.crop()
    );
}
