//! 用例测试：照 `docs/user-guide/index.md` 里的用例设计。
//!
//! 一条用例一组测试，每组测试上面一行注释写出处（形如 `// 用例：一`）。
//! `scripts/validate-usecases.sh` 拿这份出处与文档里的用例号对账。
//!
//! 测试跑的是编出来的 `qtcloud-work` 可执行文件，用临时工作区与临时数据仓，
//! 不碰真仓库。交给 `pi` 的地方用一个临时的 `pi` 桩脚本顶替，免得测试依赖真模型。

use std::path::{Path, PathBuf};
use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_qtcloud-work")
}

fn temp_root(tag: &str) -> PathBuf {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static N: AtomicUsize = AtomicUsize::new(0);
    let n = N.fetch_add(1, Ordering::SeqCst);
    let mut p = std::env::temp_dir();
    p.push(format!("qtcloud-work-usecases-{}-{}-{}", std::process::id(), tag, n));
    if p.exists() {
        std::fs::remove_dir_all(&p).expect("清旧临时目录");
    }
    std::fs::create_dir_all(&p).expect("建临时目录");
    p
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("读 {} 失败: {e}", path.display()))
}

fn write(path: &Path, body: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("建父目录");
    }
    std::fs::write(path, body).expect("写文件");
}

fn write_workflow(dir: &Path, name: &str, body: &str) {
    write(&dir.join(format!("{name}.yaml")), body);
}

struct Run {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

impl Run {
    fn ok(&self) -> bool {
        self.code == Some(0)
    }
    fn crop(&self) -> String {
        format!("exit={:?}\n--- stdout ---\n{}\n--- stderr ---\n{}", self.code, self.stdout.trim(), self.stderr.trim())
    }
}

/// 在临时工作区里跑可执行文件；`path_extra` 若非空，则前置进 PATH（放 `pi` 桩）。
fn run(root: &Path, path_extra: Option<&Path>, args: &[&str]) -> Run {
    let mut cmd = Command::new(bin());
    cmd.current_dir(root).args(args);
    if let Some(extra) = path_extra {
        let joined = match std::env::var_os("PATH") {
            Some(p) => format!("{}:{}", extra.display(), p.to_string_lossy()),
            None => extra.display().to_string(),
        };
        cmd.env("PATH", joined);
    }
    let out = cmd.output().expect("跑 qtcloud-work");
    Run {
        code: out.status.code(),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// 造一个 `pi` 桩：按 body 的 shell 片段干活（在工作区根下），并回一句「通过」。
fn stub_pi(dir: &Path, body: &str) -> PathBuf {
    let pi = dir.join("pi");
    write(&pi, &format!("#!/bin/sh\n{body}\n"));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&pi).expect("pi 元数据").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&pi, perms).expect("给 pi 可执行位");
    }
    pi
}

fn task_yaml(data: &Path, name: &str) -> String {
    read(&data.join("tasks").join(format!("{name}.yaml")))
}

fn report_md(data: &Path, name: &str) -> String {
    read(&data.join("artifacts").join("report").join(format!("{name}.md")))
}

// 用例：一
// 起一件任务并走一步：工作流只有一步「问候」，执行者是 AI，判据一条 rule（问候文件含「你好」）；
// task --new 备好任务、报告与日志，--next 把它交给 pi，核过判据后流水记一笔。
#[test]
fn usecase_1_start_task_and_take_one_step() {
    let root = temp_root("u1");
    let data = root.join("data");
    write_workflow(
        &data.join("workflows"),
        "AI冒烟",
        "name: AI冒烟\ndescription: 冒烟\nsteps:\n- name: 问候\n  description: 写一行中文问候到 问候.md\n  criteria:\n  - executor: rule\n    description: 问候落在\n    file: 问候.md\n    contains: 你好\n",
    );
    let stub = root.join("stub");
    stub_pi(&stub, "printf '你好\\n' > 问候.md\necho 通过");

    let r = root.to_str().unwrap();
    let d = data.to_str().unwrap();
    let created = run(&root, Some(&stub), &["--root", r, "--data", d, "task", "--new", "AI冒烟", "--workflow", "AI冒烟"]);
    assert!(created.ok(), "task --new 没跑通: {}", created.crop());
    assert!(data.join("tasks/AI冒烟.yaml").is_file(), "任务文件没落盘");
    assert!(data.join("artifacts/report/AI冒烟.md").is_file(), "报告没备好");
    assert!(data.join("artifacts/journal/AI冒烟.md").is_file(), "日志没备好");

    let stepped = run(&root, Some(&stub), &["--root", r, "--data", d, "task", "AI冒烟", "--next"]);
    assert!(stepped.ok(), "--next 走一步没跑通: {}", stepped.crop());
    let task = task_yaml(&data, "AI冒烟");
    assert!(task.contains("step: 问候"), "流水没记这一步:\n{task}");
    assert!(task.contains("ok: true"), "这一步没算过:\n{task}");
    let report = report_md(&data, "AI冒烟");
    assert!(report.contains("问候"), "报告没写这一步:\n{report}");
}

// 用例：二
// 三类判据各判各的：一步「写一句」挂三条判据，rule 当场核、agent 照说明审、human 原样进闸门。
#[test]
fn usecase_2_three_kinds_of_criteria() {
    let root = temp_root("u2");
    let data = root.join("data");
    write_workflow(
        &data.join("workflows"),
        "三类判据",
        "name: 三类判据\ndescription: 三类判据\nsteps:\n- name: 写一句\n  description: 写一行中文到 话.md\n  criteria:\n  - executor: rule\n    description: 话落在\n    path: 话.md\n  - executor: agent\n    description: 内容是中文且只有一行\n  - executor: human\n    description: 创始人认可\n",
    );
    let stub = root.join("stub");
    stub_pi(&stub, "printf '规矩是死的，人是活的。\\n' > 话.md\necho 通过");

    let r = root.to_str().unwrap();
    let d = data.to_str().unwrap();
    let created = run(&root, Some(&stub), &["--root", r, "--data", d, "task", "--new", "三类判据", "--workflow", "三类判据"]);
    assert!(created.ok(), "task --new 没跑通: {}", created.crop());

    let stepped = run(&root, Some(&stub), &["--root", r, "--data", d, "task", "三类判据", "--next"]);
    assert!(stepped.ok(), "--next 没跑通: {}", stepped.crop());
    let task = task_yaml(&data, "三类判据");
    assert!(task.contains("step: 写一句") && task.contains("ok: true"), "rule 与 agent 都过了才算这一步过:\n{task}");
    let report = report_md(&data, "三类判据");
    assert!(report.contains("创始人认可"), "human 判据该原样进闸门:\n{report}");
    assert!(report.contains("闸门") || report.contains("⧗"), "闸门项没列出来:\n{report}");
}

// 用例：三
// 比对两份课程档案：三步（定位、比对、结论）都交给 AI；运行上下文随任务记着，
// 后续命令不写 --workflows 也认得出定义在哪。
#[test]
fn usecase_3_compare_course_profiles() {
    let root = temp_root("u3");
    let data = root.join("data/context/qtcloud-work");
    let flows = root.join("data/profile/iGuo/workflows");
    write_workflow(
        &flows,
        "compare-course-profile",
        "name: compare-course-profile\ndescription: 比对两份课程档案\nsteps:\n- name: locate\n  description: 找齐两边档案\n  criteria:\n  - executor: rule\n    description: 两边档案清单在\n    path: locate.md\n- name: compare\n  description: 逐项对照\n  criteria:\n  - executor: rule\n    description: 对照写下来了\n    path: compare.md\n- name: conclude\n  description: 写下处置建议\n  criteria:\n  - executor: rule\n    description: 结论在\n    path: conclude.md\n",
    );
    let stub = root.join("stub");
    stub_pi(&stub, "for f in locate.md compare.md conclude.md; do printf '内容\\n' > \"$f\"; done\necho 通过");

    let r = root.to_str().unwrap();
    let d = data.to_str().unwrap();
    let f = flows.to_str().unwrap();
    let created = run(&root, Some(&stub), &["--root", r, "--data", d, "--workflows", f, "task", "--new", "compare-course-profile", "--workflow", "compare-course-profile"]);
    assert!(created.ok(), "task --new 没跑通: {}", created.crop());

    let record = task_yaml(&data, "compare-course-profile");
    assert!(record.contains("workflows"), "运行上下文里的工作流目录没记:\n{record}");

    for step in ["locate", "compare", "conclude"] {
        let stepped = run(&root, Some(&stub), &["--root", r, "--data", d, "task", "compare-course-profile", "--next"]);
        assert!(stepped.ok(), "--next 走 {step} 没跑通: {}", stepped.crop());
    }
    let record = task_yaml(&data, "compare-course-profile");
    assert!(record.matches("ok: true").count() >= 3, "三步都该记一笔:\n{record}");

    // 上下文记在任务里：不再写 --workflows 也看得到任务状态。
    let status = run(&root, None, &["--data", d, "task", "compare-course-profile"]);
    assert!(status.ok(), "不给 --workflows 就看不了任务: {}", status.crop());
}

// 用例：四
// 把语境条目收进材料：五步 pull→classify→coarsen→move-out→commit 一条条走完；
// coarsen 把条目粗加工进 materials/<分类>/index.md，两道 human 闸门挂进报告。
#[test]
fn usecase_4_context_into_material() {
    let root = temp_root("u4");
    let data = root.join("data/context/qtcloud-work");
    let flows = root.join("data/profile/iGuo/workflows");
    write_workflow(
        &flows,
        "context-to-profile",
        "name: context-to-profile\ndescription: 语境条目粗加工进材料库\nsteps:\n- name: pull\n  description: 拉语境并把条目清单写进报告\n  criteria:\n  - executor: rule\n    description: 条目清单在\n    path: pull.md\n- name: classify\n  description: 逐条认分类\n  criteria:\n  - executor: rule\n    description: 分类写下来了\n    path: classify.md\n  - executor: human\n    description: 分类裁决\n- name: coarsen\n  description: 粗加工写进 materials/<分类>/index.md\n  criteria:\n  - executor: rule\n    description: 材料格在\n    path: materials/课程/index.md\n- name: move-out\n  description: 把已迁出的条目从语境删掉\n  criteria:\n  - executor: rule\n    description: 迁出记录在\n    path: move-out.md\n- name: commit\n  description: 分层提交推送再回工作区更新指针\n  criteria:\n  - executor: rule\n    description: 提交完了\n    path: commit.md\n  - executor: human\n    description: 创始人点头\n",
    );
    let stub = root.join("stub");
    stub_pi(
        &stub,
        "printf '清单\\n' > pull.md\nprintf '分类\\n' > classify.md\nmkdir -p materials/课程\nprintf '粗加工\\n' > materials/课程/index.md\nprintf '迁出\\n' > move-out.md\nprintf '提交\\n' > commit.md\necho 通过",
    );

    let r = root.to_str().unwrap();
    let d = data.to_str().unwrap();
    let f = flows.to_str().unwrap();
    let created = run(&root, Some(&stub), &["--root", r, "--data", d, "--workflows", f, "task", "--new", "context-to-profile", "--workflow", "context-to-profile"]);
    assert!(created.ok(), "task --new 没跑通: {}", created.crop());

    for step in ["pull", "classify", "coarsen", "move-out", "commit"] {
        let stepped = run(&root, Some(&stub), &["--root", r, "--data", d, "task", "context-to-profile", "--next"]);
        assert!(stepped.ok(), "--next 走 {step} 没跑通: {}", stepped.crop());
    }
    let record = task_yaml(&data, "context-to-profile");
    assert!(record.matches("ok: true").count() >= 5, "五步都该记一笔:\n{record}");
    assert!(root.join("materials/课程/index.md").is_file(), "粗加工的材料格没落盘");
    let report = report_md(&data, "context-to-profile");
    assert!(report.contains("分类裁决") && report.contains("创始人点头"), "两道 human 闸门该挂进报告:\n{report}");
}

// 用例：五
// 人做的步骤人记一笔：human 步骤程序不抢着做，人做完用 --done 记一笔，
// 照常核 rule 判据、把闸门列进报告；--note 的原话进流水。
#[test]
fn usecase_5_human_steps_recorded_by_hand() {
    let root = temp_root("u5");
    let data = root.join("data");
    write_workflow(
        &data.join("workflows"),
        "数据归仓",
        "name: 数据归仓\ndescription: 数据归仓\nsteps:\n- name: 材料\n  executor: human\n  description: 把材料归位\n  criteria:\n  - executor: rule\n    description: AGENTS 在\n    path: AGENTS.md\n- name: 指令\n  executor: human\n  description: 把指令写下来\n  criteria:\n  - executor: rule\n    description: 指令在\n    path: 目标.md\n  - executor: human\n    description: 创始人过目\n",
    );
    write(&root.join("AGENTS.md"), "# 约定\n");
    write(&root.join("目标.md"), "# 目标\n");

    let r = root.to_str().unwrap();
    let d = data.to_str().unwrap();
    let created = run(&root, None, &["--root", r, "--data", d, "task", "--new", "数据归仓", "--workflow", "数据归仓"]);
    assert!(created.ok(), "task --new 没跑通: {}", created.crop());

    let first = run(&root, None, &["--root", r, "--data", d, "task", "数据归仓", "--done", "材料", "--note", "AGENTS.md、日志"]);
    assert!(first.ok(), "--done 材料 没跑通: {}", first.crop());
    let second = run(&root, None, &["--root", r, "--data", d, "task", "数据归仓", "--done", "指令", "--note", "目标/步骤/验收 已写"]);
    assert!(second.ok(), "--done 指令 没跑通: {}", second.crop());

    let record = task_yaml(&data, "数据归仓");
    assert!(record.contains("step: 材料") && record.contains("AGENTS.md、日志"), "第一笔的 note 该进流水:\n{record}");
    assert!(record.contains("step: 指令") && record.contains("目标/步骤/验收 已写"), "第二笔的 note 该进流水:\n{record}");
    assert!(!record.contains("ok: false"), "human 步骤按 note 记应算过:\n{record}");
    let report = report_md(&data, "数据归仓");
    assert!(report.contains("创始人过目"), "human 闸门该列进报告:\n{report}");
}
