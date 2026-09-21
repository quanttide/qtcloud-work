//! 工作区身份：坏了当场报，不喂空串给凭证派生；外部工作流目录自备，缺了报错不代建。

mod common;

use common::Fixture;

#[test]
fn 身份缺必选字段当场报不空算() {
    let fix = Fixture::new("identity-broken");
    fix.run_ledger(&["workflow", "create", "试一条", "--steps", "甲"]);
    // 把身份改坏：删掉 id，只留名字——像被人手工编辑过的样子。
    common::write(
        fix.data.join("workspace.yaml").as_path(),
        "name: 试\ntitle: 试\ndescription: 空\n",
    );
    let made = fix.run_ledger(&["workflow", "create", "再试一条", "--steps", "甲"]);
    assert!(!made.ok(), "坏身份该报错: {}", made.crop());
    assert!(
        made.crop().contains("缺必选字段") && made.crop().contains("id"),
        "该点明缺了 id: {}",
        made.crop()
    );
}

#[test]
fn 外部工作流目录缺了报错不代建() {
    let fix = Fixture::new("identity-flows");
    let missing = fix.root.join("没有这么个目录");
    let made = fix.run(
        true,
        &[
            "--data",
            fix.data.to_str().unwrap(),
            "--workflows",
            missing.to_str().unwrap(),
            "workflow",
            "create",
            "试一条",
            "--steps",
            "甲",
        ],
    );
    assert!(!made.ok(), "外部目录缺了该报错: {}", made.crop());
    assert!(
        made.crop().contains("不代建"),
        "该说明外部目录自备: {}",
        made.crop()
    );
    assert!(!missing.exists(), "不代建就是不建: {}", missing.display());
}
