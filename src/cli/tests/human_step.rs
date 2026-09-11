//! 场景：人做的步骤人记一笔——human 步骤程序不抢着做，人做完用 `--done` 记一笔，
//! 照常核 rule 判据、把闸门列进报告，`--note` 的原话进流水。
//!

mod common;

use common::Fixture;

// 用例：五
#[test]
fn human_step_recorded_by_hand() {
    let fix = Fixture::new("hand");
    fix.workflow(
        "数据归仓",
        "name: 数据归仓\ndescription: 数据归仓\nsteps:\n- name: 材料\n  executor: human\n  description: 把材料归位\n  criteria:\n  - executor: rule\n    description: AGENTS 在\n    path: AGENTS.md\n- name: 指令\n  executor: human\n  description: 把指令写下来\n  criteria:\n  - executor: rule\n    description: 指令在\n    path: 目标.md\n  - executor: human\n    description: 创始人过目\n",
    );
    fix.file("AGENTS.md", "# 约定\n");
    fix.file("目标.md", "# 目标\n");
    fix.run_full(
        false,
        &["task", "--new", "数据归仓", "--workflow", "数据归仓"],
    );

    let first = fix.run_recorded(&[
        "task",
        "数据归仓",
        "--done",
        "材料",
        "--note",
        "AGENTS.md、日志",
    ]);
    assert!(first.ok(), "--done 材料 没跑通: {}", first.crop());
    let second = fix.run_recorded(&[
        "task",
        "数据归仓",
        "--done",
        "指令",
        "--note",
        "目标/步骤/验收 已写",
    ]);
    assert!(second.ok(), "--done 指令 没跑通: {}", second.crop());

    let record = fix.task_yaml("数据归仓");
    assert!(
        record.contains("step: 材料") && record.contains("AGENTS.md、日志"),
        "第一笔的 note 该进流水:\n{record}"
    );
    assert!(
        record.contains("step: 指令") && record.contains("目标/步骤/验收 已写"),
        "第二笔的 note 该进流水:\n{record}"
    );
    assert!(
        !record.contains("ok: false"),
        "human 步骤按 note 记应算过:\n{record}"
    );
    assert!(
        fix.gates("数据归仓").contains("创始人过目"),
        "human 闸门该记在任务文件里"
    );
}
