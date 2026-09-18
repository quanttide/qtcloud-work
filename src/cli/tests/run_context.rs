//! 场景：位置装载——位置不进模型，全部由启动参数与环境定：
//! 不给 `--root` 往上找第二大脑，`QTCLOUD_WORK_ROOT` 指哪算哪，
//! `--data` / `--artifacts` 指到哪就落哪，只读动作不在任何根上落盘。

mod common;

use common::Fixture;

/// 向上搜索：人站在工作区的子目录里，根也认得出（从当前目录往上找 `data/journal`）。
#[test]
fn 不带根_往上找第二大脑() {
    let fix = Fixture::new("locate-root");
    std::fs::create_dir_all(fix.root.join("data/journal")).expect("建数据层");
    fix.file("data/insight/试.md", "# 试\n");
    let inner = fix.root.join("rooms/内间");
    std::fs::create_dir_all(&inner).expect("建子目录");
    let found = fix.run_in(&inner, &[], false, &["search", "试"]);
    assert!(found.ok(), "子目录里也该找得到根: {}", found.crop());
    assert!(
        found.crop().contains(&fix.root.display().to_string()),
        "该印出找到的那个根: {}",
        found.crop()
    );
}

/// 环境变量：人站在甲工作区，`QTCLOUD_WORK_ROOT` 指向乙——命令落在乙。
#[test]
fn 环境变量指哪就落哪() {
    let fix = Fixture::new("locate-env");
    fix.file("data/insight/试.md", "# 试\n");
    let elsewhere = fix.root.join("别处工作区");
    std::fs::create_dir_all(elsewhere.join("data/insight")).expect("建目录");
    std::fs::write(elsewhere.join("data/insight/别的.md"), "# 别的\n").expect("写文档");
    let found = fix.run_in(
        &fix.root,
        &[("QTCLOUD_WORK_ROOT", elsewhere.to_str().unwrap())],
        false,
        &["search", "别的"],
    );
    assert!(found.ok(), "环境变量指的工作区该用得上: {}", found.crop());
    assert!(
        found.crop().contains("别处工作区"),
        "该落在环境变量指的那个根: {}",
        found.crop()
    );
}

/// 显式给位置：`--data` 与 `--artifacts` 指到哪就落哪；产物落点由判据里的
/// `{{report}}` 占位验证——判据查的文件落在给的产物落点里，一步才走得过。
// 用例：八
#[test]
fn 账本与产物指哪落哪() {
    let fix = Fixture::new("locate-explicit");
    fix.workflow(
        "写报告",
        "name: 写报告\ndescription: 试\nsteps:\n- name: 写\n  description: 把结论写进报告\n  criteria:\n  - executor: rule\n    description: 结论在\n    file: '{{report}}'\n    contains: 结论\n",
    );
    common::write(
        &fix.artifacts.join("report/写报告.md"),
        "# 报告\n## 结论\n成了\n",
    );
    fix.pi("echo 通过");
    let created = fix.run_full(
        false,
        &["order", "create", "写报告", "--workflow", "写报告"],
    );
    assert!(created.ok(), "{}", created.crop());
    let stepped = fix.run_full(true, &["order", "next", "写报告"]);
    assert!(
        stepped.ok(),
        "{{report}} 该按 --artifacts 展开: {}",
        stepped.crop()
    );
    assert!(
        fix.order_yaml("写报告").contains("is_succeeded: true"),
        "判据查的是产物落点里的那份，查到了才记过:\n{}",
        fix.order_yaml("写报告")
    );
    assert!(
        fix.data.join("workorders/写报告.yaml").is_file(),
        "工单落在给的账本里"
    );
}

/// 只读纪律：只读动作不在任何根上建文件——账本是写动作开的，不写不开。
#[test]
fn 只读动作不落盘() {
    let fix = Fixture::new("locate-readonly");
    let data = fix.root.join("干净账本");
    let artifacts = fix.root.join("干净产物");
    std::fs::create_dir_all(&data).expect("建空账本目录");
    let listed = fix.run(
        false,
        &[
            "--data",
            data.to_str().unwrap(),
            "--artifacts",
            artifacts.to_str().unwrap(),
            "order",
            "list",
        ],
    );
    assert!(listed.ok(), "空账本也该列得出: {}", listed.crop());
    assert!(
        std::fs::read_dir(&data).expect("读账本").next().is_none(),
        "只读动作不在账本落盘"
    );
    assert!(!artifacts.exists(), "只读动作不建产物落点");
    assert!(
        !fix.root.join("artifacts").exists(),
        "缺省产物落点也不该被只读动作建出来"
    );
}
