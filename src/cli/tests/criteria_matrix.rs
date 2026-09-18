//! 判据执行矩阵：谁执行（agent / human）× 怎么走（next / done）× 判据三类。
//! 一格一测，格格独立。
//!
//! 用例：二（三类判据各判各的）、用例：五（人记一笔）

mod common;

use common::Fixture;

fn 一步三类判据(tag: &str) -> Fixture {
    let fix = Fixture::new(tag);
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 写一行到 产物.md\n  criteria:\n  - executor: rule\n    description: 产物在\n    path: 产物.md\n  - executor: agent\n    description: 内容是中文\n  - executor: human\n    description: 创始人点头\n",
    );
    fix
}

/// 一、agent 执行、判据全过但有闸门：不记账，等人放行。
#[test]
fn 全过但有闸门_等人放行() {
    let fix = 一步三类判据("matrix-ok");
    fix.pi("printf '你好\\n' > 产物.md\necho 通过");
    fix.run_full(true, &["order", "create", "试一条", "--workflow", "试一条"]);
    let stepped = fix.run_ledger(&["order", "next", "试一条"]);
    assert!(stepped.ok(), "程序核过的半程该算过: {}", stepped.crop());
    assert!(
        fix.run_ledger(&["order", "show", "试一条"])
            .crop()
            .contains("闸门：一步：创始人点头"),
        "human 判据该进待拍板清单"
    );
    assert!(
        fix.order_yaml("试一条").contains("records: []"),
        "闸门没放行不记账"
    );
}

/// 二、agent 执行、agent 判据不过。
#[test]
fn agent判不过_不算过() {
    let fix = 一步三类判据("matrix-agent-fail");
    fix.pi("printf '你好\\n' > 产物.md\necho 1. 不通过");
    fix.run_full(true, &["order", "create", "试一条", "--workflow", "试一条"]);
    let stepped = fix.run_ledger(&["order", "next", "试一条"]);
    assert!(!stepped.ok(), "审查不过不该算过: {}", stepped.crop());
}

/// 三、agent 执行、rule 判据不过（没写产物）。
#[test]
fn rule判不过_下一步还是它() {
    let fix = 一步三类判据("matrix-rule-fail");
    fix.pi("echo 通过");
    fix.run_full(true, &["order", "create", "试一条", "--workflow", "试一条"]);
    let stepped = fix.run_ledger(&["order", "next", "试一条"]);
    assert!(!stepped.ok(), "rule 不过不该算过: {}", stepped.crop());
    let view = fix.run_ledger(&["order", "show", "试一条"]);
    assert!(
        view.crop().contains("下一步：一步"),
        "rule 不过时下一步还是它: {}",
        view.crop()
    );
}

/// 四、人的步骤、人记一笔：rule 过就记；放行出自人。
#[test]
fn 人的步骤_rule过_记过() {
    let fix = Fixture::new("matrix-human");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  executor: human\n  description: 人写一行到 产物.md\n  criteria:\n  - executor: rule\n    description: 产物在\n    path: 产物.md\n  - executor: human\n    description: 创始人点头\n",
    );
    fix.file("产物.md", "你好\n");
    fix.run_full(
        false,
        &["order", "create", "试一条", "--workflow", "试一条"],
    );
    let done = fix.run_ledger(&["order", "done", "试一条", "一步", "--note", "写了"]);
    assert!(done.ok(), "人的步骤该能记完: {}", done.crop());
    assert!(
        fix.order_yaml("试一条").contains("is_succeeded: true"),
        "rule 过了这一笔该记过"
    );
}

/// 五、人的步骤、人记一笔但 rule 不过：记下的是没过。
#[test]
fn 人的步骤_rule不过_记没过() {
    let fix = Fixture::new("matrix-human-fail");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  executor: human\n  description: 人写一行\n  criteria:\n  - executor: rule\n    description: 产物在\n    path: 产物.md\n",
    );
    fix.run_full(
        false,
        &["order", "create", "试一条", "--workflow", "试一条"],
    );
    let done = fix.run_ledger(&["order", "done", "试一条", "一步", "--note", "写了"]);
    assert!(!done.ok(), "rule 不过不该算过: {}", done.crop());
    let view = fix.run_ledger(&["order", "show", "试一条"]);
    assert!(
        view.crop().contains("下一步：一步"),
        "还是它: {}",
        view.crop()
    );
}

/// 六、人的步骤、不记一笔就 next：程序不抢着做。
#[test]
fn 人的步骤_next不抢做() {
    let fix = Fixture::new("matrix-human-wait");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  executor: human\n  description: 人等一等\n  criteria: []\n",
    );
    fix.run_full(
        false,
        &["order", "create", "试一条", "--workflow", "试一条"],
    );
    let stepped = fix.run_ledger(&["order", "next", "试一条"]);
    assert!(stepped.ok(), "人的步骤该等人: {}", stepped.crop());
    assert!(
        stepped.crop().contains("轮到你") || stepped.crop().contains("等"),
        "该提示轮到人: {}",
        stepped.crop()
    );
}
