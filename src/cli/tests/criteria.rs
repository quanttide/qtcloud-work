//! 场景：一步挂三类判据——rule 当场核、agent 照说明审、human 原样进闸门，谁判就写谁。
//!

mod common;

use common::Fixture;

// 用例：二
#[test]
fn one_step_with_three_kinds_of_criteria() {
    let fix = Fixture::new("criteria");
    fix.workflow(
        "三类判据",
        "name: 三类判据\ndescription: 三类判据\nsteps:\n- name: 写一句\n  description: 写一行中文到 话.md\n  criteria:\n  - executor: rule\n    description: 话落在\n    path: 话.md\n  - executor: agent\n    description: 内容是中文且只有一行\n  - executor: human\n    description: 创始人认可\n",
    );
    fix.pi("printf '规矩是死的，人是活的。\\n' > 话.md\necho 通过");
    fix.run_full(
        true,
        &["task", "--new", "三类判据", "--workflow", "三类判据"],
    );

    let stepped = fix.run_recorded(&["task", "三类判据", "--next"]);
    assert!(stepped.ok(), "--next 没跑通: {}", stepped.crop());
    let record = fix.task_yaml("三类判据");
    assert!(
        record.contains("step: 写一句") && record.contains("ok: true"),
        "rule 与 agent 都过了才算这一步过:\n{record}"
    );
    let gates = fix.gates("三类判据");
    assert!(
        gates.contains("创始人认可"),
        "human 判据该原样进闸门:\n{gates}"
    );
}
