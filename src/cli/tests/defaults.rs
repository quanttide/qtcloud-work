//! 缺省参数矩阵：三个可省的位置（--root / --data / --workflows）各缺一次。
//!
//! 用例：一（起一件任务并走一步）、用例：三（运行上下文随任务记着）

mod common;

use common::Fixture;

#[test]
fn 三处位置缺省各是什么行为() {
    // 一、工作区动作不写 --root：用当前目录，并把用的是哪个根印出来
    let fix = Fixture::new("defaults");
    fix.file("data/insight/试.md", "# 试\n");
    let found = fix.run(false, &["find", "试"]);
    assert!(found.ok(), "缺省该用当前目录: {}", found.crop());
    assert!(
        found.crop().contains(&fix.root.display().to_string()),
        "该印出用的是哪个根: {}",
        found.crop()
    );

    // 二、工作区动作显式给 --root：用给的那个
    fix.workflow("试一条", "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 一步\n  criteria: []\n");
    let other = fix.root.join("别的根");
    std::fs::create_dir_all(&other).expect("建别的根");
    std::fs::create_dir_all(other.join("data/insight")).expect("建目录");
    std::fs::write(other.join("data/insight/别的.md"), "# 别的\n").expect("写文档");
    let explicit = fix.run(false, &["--root", other.to_str().unwrap(), "find", "别的"]);
    assert!(explicit.ok(), "显式给就该用它: {}", explicit.crop());
    assert!(
        explicit.crop().contains("别的根"),
        "该印出给的那个根: {}",
        explicit.crop()
    );

    // 三、任务动作只给 --data：工作区与工作流目录都取任务里记的
    fix.workflow("试一条", "name: 试一条\ndescription: 试\nsteps:\n- name: 一步\n  description: 一步\n  criteria: []\n");
    fix.run_full(true, &["task", "--new", "试一条", "--workflow", "试一条"]);
    let only_data = fix.run_recorded(&["task", "试一条"]);
    assert!(
        only_data.ok(),
        "只给数据仓就该开得起来: {}",
        only_data.crop()
    );
    assert!(
        only_data.crop().contains("下一步：一步"),
        "{}",
        only_data.crop()
    );

    // 四、任务动作给了别的 --root：命令行优先
    let elsewhere = fix.root.join("另一个工作区");
    std::fs::create_dir_all(&elsewhere).expect("建工作区");
    let overridden = fix.run(
        false,
        &[
            "--root",
            elsewhere.to_str().unwrap(),
            "--data",
            fix.data.to_str().unwrap(),
            "task",
            "试一条",
        ],
    );
    assert!(overridden.ok(), "命令行给了就该优先: {}", overridden.crop());
}
