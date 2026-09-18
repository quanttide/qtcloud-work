//! 定义核对：判据路径须在工作区内、描述点到的小节须有 `contains` 判据覆盖。
//! 只看写下的位置、不访问文件系统。
//!
//! 用例：一（起一件工单并走一步）

mod common;

use common::Fixture;

#[test]
fn 定义里的声明与判据对不对得上() {
    // 一、路径都在区内、小节都有判据 → 核对通过
    let fix = Fixture::new("check-ok");
    fix.file("材料.md", "内容\n");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 把「结论」一节写进报告\n  criteria:\n  - executor: rule\n    description: 材料在\n    path: 材料.md\n  - executor: rule\n    description: 结论在\n    file: '{{report}}'\n    contains: '## 结论'\n",
    );
    let ok = fix.run_full(false, &["workflow", "check", "试一条"]);
    assert!(ok.ok(), "该核对通过: {}", ok.crop());

    // 二、判据里的路径逃出工作区（相对路径带 ..）→ 红
    let fix = Fixture::new("check-escape");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 一步\n  criteria:\n  - executor: rule\n    description: 越界的路径\n    path: ../outside.md\n",
    );
    let bad = fix.run_full(false, &["workflow", "check", "试一条"]);
    assert!(!bad.ok(), "路径逃出工作区该红: {}", bad.crop());
    assert!(
        bad.crop().contains("../outside.md"),
        "该说清是哪个路径: {}",
        bad.crop()
    );

    // 三、绝对路径不在根底下 → 红
    let fix = Fixture::new("check-absolute");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 一步\n  criteria:\n  - executor: rule\n    description: 别处的文件\n    path: /elsewhere/文件.md\n",
    );
    let bad = fix.run_full(false, &["workflow", "check", "试一条"]);
    assert!(!bad.ok(), "绝对路径不在区内该红: {}", bad.crop());

    // 四、版本号写法（`## [X.Y.Z-pre.N]`）不是小节，不该误报
    let fix = Fixture::new("check-noise");
    fix.file("材料.md", "内容\n");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 把 Unreleased 收成 `## [X.Y.Z-pre.N]` 一节\n  criteria:\n  - executor: rule\n    description: 材料在\n    path: 材料.md\n",
    );
    let noise = fix.run_full(false, &["workflow", "check", "试一条"]);
    assert!(noise.ok(), "版本号写法不该被当成小节: {}", noise.crop());

    // 五、描述点到的小节没有判据覆盖 → 红
    let fix = Fixture::new("check-section");
    fix.file("材料.md", "内容\n");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 写进报告的「拿不准」一节\n  criteria:\n  - executor: rule\n    description: 材料在\n    path: 材料.md\n",
    );
    let bad = fix.run_full(false, &["workflow", "check", "试一条"]);
    assert!(!bad.ok(), "小节没判据该红: {}", bad.crop());
    assert!(
        bad.crop().contains("拿不准"),
        "该说清是哪一节: {}",
        bad.crop()
    );

    // 六、引号里的长句是叙述，不当小节名
    let fix = Fixture::new("check-quote");
    fix.file("材料.md", "内容\n");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 这一步要「先把两边都读清楚再下判断」，不许抢跑\n  criteria:\n  - executor: rule\n    description: 材料在\n    path: 材料.md\n",
    );
    let narrative = fix.run_full(false, &["workflow", "check", "试一条"]);
    assert!(narrative.ok(), "引号里的长句是叙述: {}", narrative.crop());
}
