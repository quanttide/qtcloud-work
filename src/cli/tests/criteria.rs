//! 场景：一步挂三类判据——rule 当场核、agent 照判准审、human 进待拍板清单，谁判就写谁。

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
        &["order", "create", "三类判据", "--workflow", "三类判据"],
    );

    let stepped = fix.run_ledger(&["order", "next", "三类判据"]);
    assert!(stepped.ok(), "order next 没跑通: {}", stepped.crop());
    // rule 与 agent 都过了，但这站挂着闸——程序不记账，等人放行。
    let order = fix.order_yaml("三类判据");
    assert!(
        order.contains("records: []"),
        "闸门没过不记账，流水里不该有这一笔:\n{order}"
    );

    // 带闸门的站：程序核过的这半算数，放行那半等人。
    let shown = fix.run_ledger(&["order", "show", "三类判据"]);
    let view = shown.crop();
    assert!(
        view.contains("闸门：写一句：创始人认可"),
        "human 判据该进待拍板清单:\n{view}"
    );
    assert!(
        view.contains("下一步：写一句"),
        "等人放行时下一步还是它:\n{view}"
    );

    // 放行之后：这一站才算走过。
    let released = fix.run_ledger(&["order", "done", "三类判据", "写一句", "--note", "认可"]);
    assert!(released.ok(), "放行没跑通: {}", released.crop());
    let final_view = fix.run_ledger(&["order", "show", "三类判据"]);
    assert!(
        final_view.crop().contains("走完了"),
        "放行后该走完: {}",
        final_view.crop()
    );
    assert!(
        fix.order_yaml("三类判据").contains("is_succeeded: true"),
        "放行记的这一笔该是过了"
    );
}
