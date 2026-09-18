//! 缺省矩阵：四个可省的位置（--root / --data / --workflows / --artifacts）各缺一次，
//! 缺省各是什么——根用当前目录、账本落 XDG 工作区键、工作流目录跟账本、产物落根下 `artifacts/`。
//!
//! 用例：一（起一件工单并走一步）、用例：八（产物落点）

mod common;

use common::{Fixture, 冒烟工作流};

#[test]
fn 四处位置缺省各是什么行为() {
    // 一、不写 --root：用当前目录，并把用的是哪个根印出来
    let fix = Fixture::new("defaults");
    fix.file("data/insight/试.md", "# 试\n");
    let found = fix.run(false, &["search", "试"]);
    assert!(found.ok(), "缺省该用当前目录: {}", found.crop());
    assert!(
        found.crop().contains(&fix.root.display().to_string()),
        "该印出用的是哪个根: {}",
        found.crop()
    );

    // 二、不写 --data：账本落 $XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/
    fix.workflow("AI冒烟", 冒烟工作流);
    fix.pi("printf '你好\\n' > 问候.md\necho 通过");
    let created = fix.run(
        true,
        &[
            "--workflows",
            fix.flows.to_str().unwrap(),
            "order",
            "create",
            "AI冒烟",
            "--workflow",
            "AI冒烟",
        ],
    );
    assert!(created.ok(), "{}", created.crop());
    let workspaces = fix.xdg.join("qtcloud-work/workspaces");
    let mut books: Vec<_> = std::fs::read_dir(&workspaces)
        .expect("账本仓该在 XDG 底下")
        .map(|entry| entry.expect("读账本仓").path())
        .collect();
    assert_eq!(books.len(), 1, "一个工作区键一本账");
    let ledger = books.pop().expect("恰好一本");
    assert!(
        ledger.join("workorders/AI冒烟.yaml").is_file(),
        "工单落在缺省账本里: {}",
        ledger.display()
    );

    // 三、不写 --workflows：定义跟在账本里（账本的 workflows/）
    let listed = fix.run(
        false,
        &[
            "--data",
            fix.data.to_str().unwrap(),
            "workflow",
            "create",
            "账本里的",
            "--steps",
            "甲",
        ],
    );
    assert!(listed.ok(), "{}", listed.crop());
    assert!(
        fix.data.join("workflows/账本里的.yaml").is_file(),
        "不给 --workflows，定义该落进账本"
    );
    let shown = fix.run(
        false,
        &[
            "--data",
            fix.data.to_str().unwrap(),
            "workflow",
            "show",
            "账本里的",
        ],
    );
    assert!(shown.ok(), "读也该从账本读: {}", shown.crop());

    // 四、不写 --artifacts：产物落 <根>/artifacts/（{{report}} 按缺省展开）
    fix.workflow(
        "写报告",
        "name: 写报告\ndescription: 试\nsteps:\n- name: 写\n  description: 把结论写进报告\n  criteria:\n  - executor: rule\n    description: 结论在\n    file: '{{report}}'\n    contains: 结论\n",
    );
    common::write(
        &fix.root.join("artifacts/report/写报告.md"),
        "# 报告\n## 结论\n成了\n",
    );
    let made = fix.run_ledger(&["order", "create", "写报告", "--workflow", "写报告"]);
    assert!(made.ok(), "{}", made.crop());
    let stepped = fix.run_ledger(&["order", "next", "写报告"]);
    assert!(
        stepped.ok(),
        "{{report}} 缺省该展开到 <根>/artifacts: {}",
        stepped.crop()
    );
}
