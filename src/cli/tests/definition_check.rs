//! 定义核对：判据里的路径在不在、描述提到的小节有没有判据覆盖。
//!
//! 用例：一（起一件任务并走一步）

mod common;

use common::Fixture;

#[test]
fn 定义里的声明与判据对不对得上() {
    // 一、路径都在、小节都有判据 → 核对通过
    let fix = Fixture::new("check-ok");
    fix.file("材料.md", "内容\n");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 把「结论」一节写进报告\n  criteria:\n  - executor: rule\n    description: 材料在\n    path: 材料.md\n  - executor: rule\n    description: 结论在\n    file: '{{report}}'\n    contains: '## 结论'\n",
    );
    fix.file("data/artifacts/report/试一条.md", "## 结论\n");
    let ok = fix.run_full(false, &["workflow", "试一条", "--check"]);
    assert!(ok.ok(), "该核对通过: {}", ok.crop());

    // 二、判据里的路径不在 → 红
    let fix = Fixture::new("check-path");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 一步\n  criteria:\n  - executor: rule\n    description: 材料在\n    path: 没有这个.md\n",
    );
    let bad = fix.run_full(false, &["workflow", "试一条", "--check"]);
    assert!(!bad.ok(), "路径不在该红: {}", bad.crop());
    assert!(
        bad.crop().contains("没有这个.md"),
        "该说清是哪个路径: {}",
        bad.crop()
    );

    // 三、版本号写法（`## [X.Y.Z-pre.N]`）不是报告小节，不该误报
    let fix = Fixture::new("check-noise");
    fix.file("材料.md", "内容\n");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 把 Unreleased 收成 `## [X.Y.Z-pre.N]` 一节\n  criteria:\n  - executor: rule\n    description: 材料在\n    path: 材料.md\n",
    );
    let noise = fix.run_full(false, &["workflow", "试一条", "--check"]);
    assert!(noise.ok(), "版本号写法不该被当成报告小节: {}", noise.crop());

    // 四、描述提到的小节没有判据覆盖 → 红
    let fix = Fixture::new("check-section");
    fix.file("材料.md", "内容\n");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 写进报告的「拿不准」一节\n  criteria:\n  - executor: rule\n    description: 材料在\n    path: 材料.md\n",
    );
    let bad = fix.run_full(false, &["workflow", "试一条", "--check"]);
    assert!(!bad.ok(), "小节没判据该红: {}", bad.crop());
    assert!(
        bad.crop().contains("拿不准"),
        "该说清是哪一节: {}",
        bad.crop()
    );
}
