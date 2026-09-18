//! 领域事件：三件事各落一行 JSONL——负载带工作区与工单、记录的身份字段与全文；
//! 事件文件只增不改，去重是下游按 `id` 做的事。

mod common;

use common::Fixture;
use serde_json::Value as Json;

// 用例：一（起一件工单并走一步那件事的账）
#[test]
fn 三件事各落一行账() {
    let fix = Fixture::new("events");
    fix.run_full(false, &["workflow", "create", "AI冒烟", "--steps", "问候"]);
    let workflow_id = fix.workflow_id("AI冒烟");
    fix.pi("printf '你好\\n' > 问候.md\necho 通过");
    fix.run_full(true, &["order", "create", "AI冒烟", "--workflow", "AI冒烟"]);
    fix.run_ledger(&["order", "next", "AI冒烟"]);

    let text = common::read(&fix.data.join("events.jsonl"));
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 3, "三件事三行:\n{text}");

    let events: Vec<Json> = lines
        .iter()
        .map(|line| serde_json::from_str(line).expect("每行都是 JSON"))
        .collect();

    // 工作流已创建：带工作区、派生凭证与名字。
    assert_eq!(events[0]["event"], "WorkflowCreated", "{text}");
    assert!(
        events[0]["workspace_id"]
            .as_str()
            .is_some_and(|id| !id.is_empty()),
        "事件该带工作区 id:\n{text}"
    );
    assert_eq!(events[0]["workflow_id"], workflow_id, "{text}");
    assert_eq!(events[0]["name"], "AI冒烟", "{text}");

    // 工单已创建：带工单凭证、名字与所引工作流。
    assert_eq!(events[1]["event"], "WorkOrderCreated", "{text}");
    let order_yaml = fix.order_yaml("AI冒烟");
    let order_id = order_yaml
        .lines()
        .find_map(|line| line.strip_prefix("id: "))
        .expect("工单有凭证");
    assert_eq!(events[1]["order_id"], order_id, "{text}");
    assert_eq!(events[1]["name"], "AI冒烟", "{text}");
    assert_eq!(events[1]["workflow_id"], workflow_id, "{text}");

    // 工作记录已追加：带记录凭证、页码、步骤凭证与全文。
    assert_eq!(events[2]["event"], "WorkRecorded", "{text}");
    assert_eq!(events[2]["order_id"], order_id, "{text}");
    assert_eq!(events[2]["seq"], 1, "{text}");
    assert!(
        events[2]["record_id"]
            .as_str()
            .is_some_and(|id| !id.is_empty()),
        "记录事件该带记录 id:\n{text}"
    );
    assert!(
        events[2]["step_id"]
            .as_str()
            .is_some_and(|id| !id.is_empty()),
        "记录事件该带步骤凭证:\n{text}"
    );
    assert!(
        events[2]["record"]["step"] == "问候",
        "记录全文该在事件里:\n{text}"
    );
}
