//! 流水纪律：只增不改——同 id 即拒、seq 自 1 起不跳号、step_id 账本方查填、
//! 时刻不倒流、旧记录原样在账上（以最新一条为准是推导，不是销毁）。

mod common;

use common::Fixture;

const 白纸: &str = "id: 11111111-1111-4111-8111-111111111111\nname: 试一条\ndescription: 试\nworkflow_id: 44444444-4444-4444-8444-444444444444\ncreated_at: 2026-09-11T10:00:00\nrecords:\n";

// 用例：六
#[test]
fn 撞号的账是复制的账() {
    let fix = Fixture::new("records-dup-id");
    fix.workflow("试一条", 冒烟定义());
    let workflow_id = fix.workflow_id("试一条");
    let body = 白纸.replace("44444444-4444-4444-8444-444444444444", &workflow_id);
    let records = "- id: 22222222-2222-4222-8222-000000000001\n  seq: 1\n  created_at: 2026-09-11T10:01:00\n  order_id: 11111111-1111-4111-8111-111111111111\n  step: 问候\n  step_id: 33333333-3333-4333-8333-000000000001\n  description: 一笔\n  is_succeeded: true\n- id: 22222222-2222-4222-8222-000000000001\n  seq: 2\n  created_at: 2026-09-11T10:02:00\n  order_id: 11111111-1111-4111-8111-111111111111\n  step: 问候\n  step_id: 33333333-3333-4333-8333-000000000002\n  description: 重放同 id\n  is_succeeded: true\n";
    common::write(
        &fix.data.join("workorders/试一条.yaml"),
        &format!("{body}{records}"),
    );
    let shown = fix.run_ledger(&["order", "show", "试一条"]);
    assert!(!shown.ok(), "同 id 两笔该拒: {}", shown.crop());
    assert!(
        shown.crop().contains("撞号"),
        "该说清是撞号: {}",
        shown.crop()
    );
}

#[test]
fn 跳号的账是被抽走的账() {
    let fix = Fixture::new("records-gap");
    fix.workflow("试一条", 冒烟定义());
    let workflow_id = fix.workflow_id("试一条");
    let body = 白纸.replace("44444444-4444-4444-8444-444444444444", &workflow_id);
    let records = "- id: 22222222-2222-4222-8222-000000000001\n  seq: 1\n  created_at: 2026-09-11T10:01:00\n  order_id: 11111111-1111-4111-8111-111111111111\n  step: 问候\n  step_id: 33333333-3333-4333-8333-000000000001\n  description: 一笔\n  is_succeeded: true\n- id: 22222222-2222-4222-8222-000000000002\n  seq: 3\n  created_at: 2026-09-11T10:02:00\n  order_id: 11111111-1111-4111-8111-111111111111\n  step: 问候\n  step_id: 33333333-3333-4333-8333-000000000002\n  description: 跳过 2\n  is_succeeded: true\n";
    common::write(
        &fix.data.join("workorders/试一条.yaml"),
        &format!("{body}{records}"),
    );
    let shown = fix.run_ledger(&["order", "show", "试一条"]);
    assert!(!shown.ok(), "seq 跳号该拒: {}", shown.crop());
    assert!(
        shown.crop().contains("跳号"),
        "该说清是跳号: {}",
        shown.crop()
    );
}

#[test]
fn 倒流的账不可信() {
    let fix = Fixture::new("records-time");
    fix.workflow("试一条", 冒烟定义());
    let workflow_id = fix.workflow_id("试一条");
    let body = 白纸.replace("44444444-4444-4444-8444-444444444444", &workflow_id);
    let records = "- id: 22222222-2222-4222-8222-000000000001\n  seq: 1\n  created_at: 2026-09-11T10:05:00\n  order_id: 11111111-1111-4111-8111-111111111111\n  step: 问候\n  step_id: 33333333-3333-4333-8333-000000000001\n  description: 后发生\n  is_succeeded: true\n- id: 22222222-2222-4222-8222-000000000002\n  seq: 2\n  created_at: 2026-09-11T10:01:00\n  order_id: 11111111-1111-4111-8111-111111111111\n  step: 问候\n  step_id: 33333333-3333-4333-8333-000000000002\n  description: 先发生\n  is_succeeded: true\n";
    common::write(
        &fix.data.join("workorders/试一条.yaml"),
        &format!("{body}{records}"),
    );
    let shown = fix.run_ledger(&["order", "show", "试一条"]);
    assert!(!shown.ok(), "时间倒流该拒: {}", shown.crop());
    assert!(
        shown.crop().contains("倒流"),
        "该说清是倒流: {}",
        shown.crop()
    );
}

#[test]
fn 只增不改_旧账原样在() {
    let fix = Fixture::new("records-append");
    fix.workflow("AI冒烟", 冒烟定义());
    fix.pi("echo 不通过");
    fix.run_full(true, &["order", "create", "AI冒烟", "--workflow", "AI冒烟"]);
    // 第一笔：rule 判据不过，记下没过。
    let _ = fix.run_ledger(&["order", "next", "AI冒烟"]);
    // 重走一笔过了。
    fix.pi("printf '你好\\n' > 问候.md\necho 通过");
    let again = fix.run_ledger(&["order", "next", "AI冒烟"]);
    assert!(again.ok(), "重走该过: {}", again.crop());
    let order = fix.order_yaml("AI冒烟");
    assert!(
        order.contains("is_succeeded: false") && order.contains("is_succeeded: true"),
        "旧账原样在，新账追加在后:\n{order}"
    );
    assert!(
        order.contains("seq: 1") && order.contains("seq: 2"),
        "页码自 1 起连续:\n{order}"
    );
    let first_before = order.split("seq: 2").next().unwrap_or_default().to_string();
    assert!(
        first_before.contains("is_succeeded: false"),
        "第一笔还是没过:\n{order}"
    );
}

fn 冒烟定义() -> &'static str {
    "name: AI冒烟\ndescription: 冒烟\nsteps:\n- name: 问候\n  description: 写一行中文问候\n  criteria:\n  - executor: rule\n    description: 问候落在\n    path: 问候.md\n"
}
