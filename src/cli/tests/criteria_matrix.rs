//! 判据执行矩阵：谁执行（agent / human）× 怎么走（--next / --done）× 判据三类。
//!
//! 用例：二（三类判据各判各的）、用例：五（人做的步骤人记一笔）

mod common;

use common::Fixture;

fn 一步三类判据() -> Fixture {
    let fix = Fixture::new("matrix");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 写一行到 产物.md\n  criteria:\n  - executor: rule\n    description: 产物在\n    path: 产物.md\n  - executor: agent\n    description: 内容是中文\n  - executor: human\n    description: 创始人点头\n",
    );
    fix
}

#[test]
fn 谁执行乘怎么走() {
    // 一、agent 执行、判据全过
    let fix = 一步三类判据();
    fix.pi("printf '你好\\n' > 产物.md\necho 通过");
    fix.run_full(true, &["task", "--new", "试一条", "--workflow", "试一条"]);
    let stepped = fix.run_recorded(&["task", "试一条", "--next"]);
    assert!(stepped.ok(), "全过该算过: {}", stepped.crop());
    assert!(
        fix.gates("试一条").contains("创始人点头"),
        "human 判据该进闸门"
    );

    // 二、agent 执行、agent 判据不过
    let fix = 一步三类判据();
    fix.pi("printf '你好\\n' > 产物.md\necho 1. 不通过");
    fix.run_full(true, &["task", "--new", "试一条", "--workflow", "试一条"]);
    let stepped = fix.run_recorded(&["task", "试一条", "--next"]);
    assert!(!stepped.ok(), "审查不过不该算过: {}", stepped.crop());

    // 三、agent 执行、rule 判据不过（没写产物）
    let fix = 一步三类判据();
    fix.pi("echo 通过");
    fix.run_full(true, &["task", "--new", "试一条", "--workflow", "试一条"]);
    let stepped = fix.run_recorded(&["task", "试一条", "--next"]);
    assert!(!stepped.ok(), "rule 不过不该算过: {}", stepped.crop());
    let view = fix.run_recorded(&["task", "试一条"]);
    assert!(
        view.crop().contains("下一步：一步"),
        "rule 不过时下一步还是它: {}",
        view.crop()
    );

    // 四、人的步骤、人为地记一步：rule 过、agent 判据算待判，不挡
    let fix = Fixture::new("matrix-human");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  executor: human\n  description: 人写一行到 产物.md\n  criteria:\n  - executor: rule\n    description: 产物在\n    path: 产物.md\n  - executor: agent\n    description: 内容是中文\n",
    );
    fix.file("产物.md", "你好\n");
    fix.run_full(false, &["task", "--new", "试一条", "--workflow", "试一条"]);
    let done = fix.run_recorded(&["task", "试一条", "--done", "一步", "--note", "写了"]);
    assert!(done.ok(), "人的步骤该能记完: {}", done.crop());
    assert!(
        fix.gates("试一条").contains("内容是中文"),
        "待判的 agent 判据该进闸门"
    );

    // 五、人的步骤、人为地记一步但 rule 不过
    let fix = Fixture::new("matrix-human-fail");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  executor: human\n  description: 人写一行\n  criteria:\n  - executor: rule\n    description: 产物在\n    path: 产物.md\n",
    );
    fix.run_full(false, &["task", "--new", "试一条", "--workflow", "试一条"]);
    let done = fix.run_recorded(&["task", "试一条", "--done", "一步", "--note", "写了"]);
    assert!(!done.ok(), "rule 不过不该算过: {}", done.crop());
    let view = fix.run_recorded(&["task", "试一条"]);
    assert!(
        view.crop().contains("下一步：一步"),
        "还是它: {}",
        view.crop()
    );

    // 六、人的步骤、不记一笔就 --next：程序不抢着做
    let fix = Fixture::new("matrix-human-wait");
    fix.workflow(
        "试一条",
        "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  executor: human\n  description: 人等一等\n  criteria: []\n",
    );
    fix.run_full(false, &["task", "--new", "试一条", "--workflow", "试一条"]);
    let stepped = fix.run_recorded(&["task", "试一条", "--next"]);
    assert!(stepped.ok(), "人的步骤该等人: {}", stepped.crop());
    assert!(
        stepped.crop().contains("轮到你"),
        "该提示轮到人: {}",
        stepped.crop()
    );
}
