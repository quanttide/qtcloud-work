//! 场景：起一件任务——任务、报告、日志三样落盘，运行上下文记进任务文件；工作流不在就挡。
//!

mod support;

use support::{Fixture, 冒烟工作流};

// 用例：一
#[test]
fn scenario_start_task_lays_down_files_and_context() {
    let fix = Fixture::new("start");
    fix.workflow("AI冒烟", 冒烟工作流);
    fix.pi("printf '你好\\n' > 问候.md\necho 通过");

    let created = fix.run_full(true, &["task", "--new", "AI冒烟", "--workflow", "AI冒烟"]);
    assert!(created.ok(), "task --new 没跑通: {}", created.crop());
    assert!(fix.data.join("tasks/AI冒烟.yaml").is_file(), "任务文件没落盘");
    assert!(fix.data.join("artifacts/report/AI冒烟.md").is_file(), "报告没备好");
    assert!(fix.data.join("artifacts/journal/AI冒烟.md").is_file(), "日志没备好");

    let record = fix.task_yaml("AI冒烟");
    assert!(record.contains("start:"), "开工时间没记:\n{record}");
    assert!(record.contains("workflow: AI冒烟"), "跑哪条工作流没记:\n{record}");
    assert!(record.contains("workflows:"), "工作流目录没随任务记:\n{record}");
    assert!(record.contains("log:"), "流水没开:\n{record}");

    let missing = fix.run_full(true, &["task", "--new", "没有这条", "--workflow", "没有这条"]);
    assert!(!missing.ok(), "工作流不在该挡住: {}", missing.crop());
}
