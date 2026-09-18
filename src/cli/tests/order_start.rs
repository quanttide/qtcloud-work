//! 场景：开工单——封面落笔即封，`id` / `workflow_id` / `created_at` 账本方查填；
//! 工作流不在就挡，重名不覆盖。

mod common;

use common::{Fixture, 冒烟工作流};

// 用例：一
#[test]
fn start_order_seals_the_cover() {
    let fix = Fixture::new("start");
    fix.workflow("AI冒烟", 冒烟工作流);
    fix.pi("printf '你好\\n' > 问候.md\necho 通过");

    let created = fix.run_full(true, &["order", "create", "AI冒烟", "--workflow", "AI冒烟"]);
    assert!(created.ok(), "order create 没跑通: {}", created.crop());
    assert!(
        fix.data.join("workorders/AI冒烟.yaml").is_file(),
        "工单文件没落进账本"
    );
    assert!(
        !fix.artifacts.join("report/AI冒烟.md").exists(),
        "程序不建产物：产物是执行时写的"
    );

    let order = fix.order_yaml("AI冒烟");
    assert!(order.contains("id:"), "凭证号没记:\n{order}");
    assert!(
        order.contains("workflow_id:") && !order.contains("workflow_id: ''"),
        "所引工作流的凭证没查填:\n{order}"
    );
    assert!(order.contains("created_at:"), "开单时刻没记:\n{order}");
    assert!(order.contains("records: []"), "流水该从空账开:\n{order}");
    assert!(
        !order.contains("gates:"),
        "闸门不落封面字段（由定义加流水推导）:\n{order}"
    );
    assert!(!order.contains("root:"), "位置不进工单文件:\n{order}");

    // 封面落笔即封：重名即拒，不覆盖。
    let again = fix.run_full(true, &["order", "create", "AI冒烟", "--workflow", "AI冒烟"]);
    assert!(!again.ok(), "重名该挡住: {}", again.crop());

    // 工作流不在：挡。
    let missing = fix.run_full(
        true,
        &["order", "create", "没有这条", "--workflow", "没有这条"],
    );
    assert!(!missing.ok(), "工作流不在该挡住: {}", missing.crop());
}
