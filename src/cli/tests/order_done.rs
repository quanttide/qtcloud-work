//! 场景：人记一笔（人的路径）——human 步骤程序不抢着做，人做完用 `order done` 记一笔，
//! 照常核 rule 判据，`--note` 的原话进流水；闸门不落字段，由定义加流水推导。

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
        &["order", "create", "数据归仓", "--workflow", "数据归仓"],
    );

    // 闸门先在：还没放的 human 判据，待拍板清单里看得见。
    let before = fix.run_ledger(&["order", "show", "数据归仓"]);
    assert!(
        before.crop().contains("闸门：指令：创始人过目"),
        "没放的闸门该在待拍板清单里: {}",
        before.crop()
    );

    let first = fix.run_ledger(&[
        "order",
        "done",
        "数据归仓",
        "材料",
        "--note",
        "AGENTS.md、日志",
    ]);
    assert!(first.ok(), "order done 材料 没跑通: {}", first.crop());
    let second = fix.run_ledger(&[
        "order",
        "done",
        "数据归仓",
        "指令",
        "--note",
        "目标/步骤/验收 已写",
    ]);
    assert!(second.ok(), "order done 指令 没跑通: {}", second.crop());

    let order = fix.order_yaml("数据归仓");
    assert!(
        order.contains("step: 材料") && order.contains("AGENTS.md、日志"),
        "第一笔的 note 该进流水:\n{order}"
    );
    assert!(
        order.contains("step: 指令") && order.contains("目标/步骤/验收 已写"),
        "第二笔的 note 该进流水:\n{order}"
    );
    assert!(
        !order.contains("is_succeeded: false"),
        "过了的步骤记过:\n{order}"
    );
    assert!(
        order.contains("step_id:") && !order.contains("step_id: ''"),
        "step_id 账本方查填:\n{order}"
    );

    // 记完的两站不再进待拍板清单。
    let after = fix.run_ledger(&["order", "show", "数据归仓"]);
    let shown = after.crop();
    assert!(!shown.contains("闸门："), "放完的闸门不该再挂: {shown}");
    assert!(shown.contains("走完了"), "两站都过该走完: {shown}");

    // 没有这一步：挡。
    let missing = fix.run_ledger(&["order", "done", "数据归仓", "查无此步"]);
    assert!(!missing.ok(), "定义里没有的步骤该挡住: {}", missing.crop());
}
