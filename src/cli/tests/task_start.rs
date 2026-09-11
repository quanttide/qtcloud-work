//! 场景：起一件任务——任务、报告、日志三样落盘，运行上下文记进任务文件；工作流不在就挡。
//!

mod common;

use common::{Fixture, 冒烟工作流};

// 用例：一
#[test]
fn start_task_lays_down_files_and_context() {
    let fix = Fixture::new("start");
    fix.workflow("AI冒烟", 冒烟工作流);
    fix.pi("printf '你好\\n' > 问候.md\necho 通过");

    let created = fix.run_full(true, &["task", "--new", "AI冒烟", "--workflow", "AI冒烟"]);
    assert!(created.ok(), "task --new 没跑通: {}", created.crop());
    assert!(
        fix.data.join("tasks/AI冒烟.yaml").is_file(),
        "任务文件没落盘"
    );
    assert!(
        !fix.data.join("artifacts/report/AI冒烟.md").exists(),
        "程序不建产物：没声明落点就不该有报告"
    );

    let record = fix.task_yaml("AI冒烟");
    assert!(record.contains("start:"), "开工时间没记:\n{record}");
    assert!(
        record.contains("workflow: AI冒烟"),
        "跑哪条工作流没记:\n{record}"
    );
    assert!(
        record.contains("workflows:"),
        "工作流目录没随任务记:\n{record}"
    );
    assert!(record.contains("log:"), "流水没开:\n{record}");
    assert!(record.contains("gates:"), "闸门项没有落处:\n{record}");
    assert!(record.contains("products:"), "产物落点没有落处:\n{record}");
    assert!(record.contains("gates:"), "闸门项没开:\n{record}");
    assert!(record.contains("products:"), "产物落点没开:\n{record}");

    let missing = fix.run_full(
        true,
        &["task", "--new", "没有这条", "--workflow", "没有这条"],
    );
    assert!(!missing.ok(), "工作流不在该挡住: {}", missing.crop());

    // 同类场景的另一半：不写 --data，落在当前目录下的 data/（开发环境的默认数据仓）。
    let plain = Fixture::new("default-data");
    plain.workflow("AI冒烟", 冒烟工作流); // 夹具把工作流放固定资产目录
    std::fs::create_dir_all(plain.root.join("data/workflows")).expect("建默认数据仓的工作流目录");
    std::fs::write(plain.root.join("data/workflows/AI冒烟.yaml"), 冒烟工作流).expect("写工作流");
    let defaulted = plain.run(false, &["task", "--new", "AI冒烟", "--workflow", "AI冒烟"]);
    assert!(
        defaulted.ok(),
        "不写 --data 该用当前目录的 data/: {}",
        defaulted.crop()
    );
    assert!(
        plain.root.join("data/tasks/AI冒烟.yaml").is_file(),
        "任务没落进默认数据仓"
    );
    assert!(
        defaulted.crop().contains("data"),
        "用的哪个数据仓该说出来: {}",
        defaulted.crop()
    );
}
