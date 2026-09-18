//! 凭证：定义不写 `id`——工作流按「工作区 id + 名字」派生，步骤按「工作流凭证 + 名字」派生；
//! 同一工作区两次读一样，跨工作区各是各的；工单与工作记录的凭证由程序发。

mod common;

use common::Fixture;

// 用例：七
#[test]
fn 定义落盘不带凭证_读时现算() {
    let fix = Fixture::new("credentials");
    fix.run_ledger(&["workflow", "create", "试一条", "--steps", "甲,乙"]);

    // 落盘的那份里没有 id。
    let on_disk = common::read(&fix.flows.join("试一条.yaml"));
    assert!(!on_disk.contains("id:"), "定义文件不该带凭证:\n{on_disk}");

    // 读两次，算出的是同一枚。
    let first = fix.workflow_id("试一条");
    let second = fix.workflow_id("试一条");
    assert_eq!(first, second, "同一工作区同一名字该算出同一枚凭证");

    // 工作步骤也有凭证（show 的 data 里带）。
    let shown = fix.run_ledger(&["workflow", "show", "试一条", "--json"]);
    assert!(
        shown.stdout().contains("step_ids") && shown.stdout().contains("甲"),
        "步骤凭证该在给窗口的那一栏: {}",
        shown.crop()
    );
}

#[test]
fn 同名跨工作区凭证不同() {
    let one = Fixture::new("credentials-ws1");
    let other = Fixture::new("credentials-ws2");
    one.run_ledger(&["workflow", "create", "试一条", "--steps", "甲"]);
    other.run_ledger(&["workflow", "create", "试一条", "--steps", "甲"]);
    let left = one.workflow_id("试一条");
    let right = other.workflow_id("试一条");
    assert_ne!(left, right, "不同工作区算出的凭证该不同");
}

#[test]
fn 工单与工作记录带凭证() {
    let fix = Fixture::new("credentials-order");
    fix.run_ledger(&["workflow", "create", "AI冒烟", "--steps", "问候"]);
    fix.file("data/journal/README.md", "# 日志\n");
    let created = fix.run_ledger(&["order", "create", "试一单", "--workflow", "AI冒烟"]);
    assert!(created.ok(), "{}", created.crop());
    let order = fix.order_yaml("试一单");
    assert!(order.contains("id:"), "工单该有程序发的凭证:\n{order}");
    assert!(
        order.contains("workflow_id:"),
        "工单该有所引工作流的凭证:\n{order}"
    );

    let done = fix.run_ledger(&["order", "done", "试一单", "问候", "--note", "写了"]);
    assert!(done.ok(), "{}", done.crop());
    let order = fix.order_yaml("试一单");
    assert!(
        order.contains("step_id:"),
        "工作记录该有这一站的凭证:\n{order}"
    );
}
