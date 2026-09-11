//! 用例测试：照 `docs/user-guide/index.md` 里的用例设计。
//!
//! 按场景命名与分解：一个测试一个场景，同类场景合在一个测试里（正例与它的负例同测，
//! 免得负例在实现缺席时单独变绿）。每个测试上面一行写出处（形如 `// 用例：一`），
//! `scripts/validate-usecases.sh` 拿这份出处与文档里的用例号对账。
//!
//! 测试跑的是编出来的 `qtcloud-work` 可执行文件，用临时工作区与临时数据仓，
//! 不碰真仓库。交给 `pi` 的地方用一个临时的 `pi` 桩脚本顶替，免得测试依赖真模型。

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_qtcloud-work")
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

/// 一次跑出来的结果：退出码与两路输出。
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
        format!(
            "exit={:?}\n--- stdout ---\n{}\n--- stderr ---\n{}",
            self.code,
            self.stdout.trim(),
            self.stderr.trim()
        )
    }
}

/// 一个临时工作区：根目录、数据仓、工作流目录、`pi` 桩目录各一份。
///
/// 数据仓与工作流目录故意分开放（草稿仓与固定资产目录各一处），
/// 让「运行上下文随任务记着」这条能被真验到。
struct Fixture {
    root: PathBuf,
    data: PathBuf,
    flows: PathBuf,
    stub: PathBuf,
}

impl Fixture {
    fn new(tag: &str) -> Self {
        static N: AtomicUsize = AtomicUsize::new(0);
        let n = N.fetch_add(1, Ordering::SeqCst);
        let root = std::env::temp_dir().join(format!(
            "qtcloud-work-usecases-{}-{}-{}",
            std::process::id(),
            tag,
            n
        ));
        if root.exists() {
            std::fs::remove_dir_all(&root).expect("清旧临时目录");
        }
        std::fs::create_dir_all(&root).expect("建临时目录");
        let fix = Fixture {
            data: root.join("data/context/qtcloud-work"),
            flows: root.join("data/profile/iGuo/workflows"),
            stub: root.join("stub"),
            root,
        };
        std::fs::create_dir_all(&fix.flows).expect("建工作流目录");
        fix
    }

    fn workflow(&self, name: &str, body: &str) {
        write(&self.flows.join(format!("{name}.yaml")), body);
    }

    fn file(&self, rel: &str, body: &str) {
        write(&self.root.join(rel), body);
    }

    /// 造一个 `pi` 桩：按 body 的 shell 片段干活（在工作区根下），并回一句「通过」。
    /// 传一个必定失败的片段，就得到「AI 没跑成」那一半场景。
    fn pi(&self, body: &str) {
        let pi = self.stub.join("pi");
        write(&pi, &format!("#!/bin/sh\n{body}\n"));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&pi).expect("pi 元数据").permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&pi, perms).expect("给 pi 可执行位");
        }
    }

    /// 跑可执行文件。`stub` 为真时把 `pi` 桩前置进 PATH。
    fn run(&self, stub: bool, args: &[&str]) -> Run {
        let mut cmd = Command::new(bin());
        cmd.current_dir(&self.root).args(args);
        if stub {
            let joined = match std::env::var_os("PATH") {
                Some(p) => format!("{}:{}", self.stub.display(), p.to_string_lossy()),
                None => self.stub.display().to_string(),
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

    /// 带上三处位置跑（工作区、数据仓、工作流目录都显式给）。
    fn run_full(&self, stub: bool, args: &[&str]) -> Run {
        let mut full: Vec<&str> = vec![
            "--root",
            self.root.to_str().unwrap(),
            "--data",
            self.data.to_str().unwrap(),
            "--workflows",
            self.flows.to_str().unwrap(),
        ];
        full.extend_from_slice(args);
        self.run(stub, &full)
    }

    /// 只给数据仓跑（工作区与工作流目录靠任务里记的上下文）。
    fn run_recorded(&self, args: &[&str]) -> Run {
        let mut full: Vec<&str> = vec!["--data", self.data.to_str().unwrap()];
        full.extend_from_slice(args);
        self.run(false, &full)
    }

    fn task_yaml(&self, name: &str) -> String {
        read(&self.data.join("tasks").join(format!("{name}.yaml")))
    }

    fn report(&self, name: &str) -> String {
        read(&self.data.join("artifacts/report").join(format!("{name}.md")))
    }
}

const 冒烟工作流: &str = "name: AI冒烟\ndescription: 冒烟\nsteps:\n- name: 问候\n  description: 写一行中文问候到 问候.md\n  criteria:\n  - executor: rule\n    description: 问候落在\n    file: 问候.md\n    contains: 你好\n";

// 场景：起一件任务——任务、报告、日志三样落盘，运行上下文记进任务文件；
// 工作流不在就挡（同类场景合在这里）。
// 用例：一
#[test]
fn scenario_start_task_lays_down_files_and_context() {
    let fix = Fixture::new("start");
    fix.workflow("AI冒烟", 冒烟工作流);
    fix.pi("printf '你好\\n' > 问候.md\necho 通过");

    let created = fix.run_full(true, &["task", "--new", "AI冒烟", "--workflow", "AI冒烟"]);
    assert!(created.ok(), "task --new 没跑通: {}", created.crop());
    assert!(fix.data.join("tasks/AI冒烟.yaml").is_file(), "任务文件没落盘");
    assert!(fix.data.join("artifacts/report/AI冒烟.md").is_file(), "报告没备好");
    assert!(fix.data.join("artifacts/journal/AI冒烟.md").is_file(), "日志没备好");

    let record = fix.task_yaml("AI冒烟");
    assert!(record.contains("start:"), "开工时间没记:\n{record}");
    assert!(record.contains("workflow: AI冒烟"), "跑哪条工作流没记:\n{record}");
    assert!(record.contains("workflows:"), "工作流目录没随任务记:\n{record}");
    assert!(record.contains("log: []") || record.contains("log:"), "流水没开:\n{record}");

    let missing = fix.run_full(true, &["task", "--new", "没有这条", "--workflow", "没有这条"]);
    assert!(!missing.ok(), "工作流不在该挡住: {}", missing.crop());
}

// 场景：走一步（agent）——交给 pi 干活，回来核 rule 判据、记一笔、写报告；
// pi 没跑成那一次不算过（同类场景合在这里）。
// 用例：一
#[test]
fn scenario_take_one_agent_step_through_pi() {
    let fix = Fixture::new("step");
    fix.workflow("AI冒烟", 冒烟工作流);
    fix.pi("printf '你好\\n' > 问候.md\necho 通过");
    fix.run_full(true, &["task", "--new", "AI冒烟", "--workflow", "AI冒烟"]);

    let stepped = fix.run_recorded(&["task", "AI冒烟", "--next"]);
    assert!(stepped.ok(), "--next 走一步没跑通: {}", stepped.crop());
    let record = fix.task_yaml("AI冒烟");
    assert!(record.contains("step: 问候"), "流水没记这一步:\n{record}");
    assert!(record.contains("ok: true"), "这一步没算过:\n{record}");
    assert!(fix.report("AI冒烟").contains("问候"), "报告没写这一步");

    // 同类另一半：pi 失败 → 这一步不算过，流水留 ✗。
    let broken = Fixture::new("step-fail");
    broken.workflow("AI冒烟", 冒烟工作流);
    broken.pi("echo 我没干成 >&2\nexit 1");
    broken.run_full(true, &["task", "--new", "AI冒烟", "--workflow", "AI冒烟"]);
    broken.run_recorded(&["task", "AI冒烟", "--next"]);
    let failed = broken.task_yaml("AI冒烟");
    assert!(failed.contains("ok: false"), "AI 没跑成不该算过:\n{failed}");
}

// 场景：一步挂三类判据——rule 当场核、agent 照说明审、human 原样进闸门，谁判就写谁。
// 用例：二
#[test]
fn scenario_one_step_with_three_kinds_of_criteria() {
    let fix = Fixture::new("criteria");
    fix.workflow(
        "三类判据",
        "name: 三类判据\ndescription: 三类判据\nsteps:\n- name: 写一句\n  description: 写一行中文到 话.md\n  criteria:\n  - executor: rule\n    description: 话落在\n    path: 话.md\n  - executor: agent\n    description: 内容是中文且只有一行\n  - executor: human\n    description: 创始人认可\n",
    );
    fix.pi("printf '规矩是死的，人是活的。\\n' > 话.md\necho 通过");
    fix.run_full(true, &["task", "--new", "三类判据", "--workflow", "三类判据"]);

    let stepped = fix.run_recorded(&["task", "三类判据", "--next"]);
    assert!(stepped.ok(), "--next 没跑通: {}", stepped.crop());
    let record = fix.task_yaml("三类判据");
    assert!(
        record.contains("step: 写一句") && record.contains("ok: true"),
        "rule 与 agent 都过了才算这一步过:\n{record}"
    );
    let report = fix.report("三类判据");
    assert!(report.contains("创始人认可"), "human 判据该原样进闸门:\n{report}");
    assert!(report.contains("闸门") || report.contains("⧗"), "闸门项没列出来:\n{report}");
}

// 场景：多步都交给 AI，依次走完；运行上下文随任务记着，后续命令不写
// --workflows 也认得出定义在哪。
// 用例：三
#[test]
fn scenario_three_ai_steps_with_recorded_context() {
    let fix = Fixture::new("compare");
    fix.workflow(
        "compare-course-profile",
        "name: compare-course-profile\ndescription: 比对两份课程档案\nsteps:\n- name: locate\n  description: 找齐两边档案\n  criteria:\n  - executor: rule\n    description: 两边档案清单在\n    path: locate.md\n- name: compare\n  description: 逐项对照\n  criteria:\n  - executor: rule\n    description: 对照写下来了\n    path: compare.md\n- name: conclude\n  description: 写下处置建议\n  criteria:\n  - executor: rule\n    description: 结论在\n    path: conclude.md\n",
    );
    fix.pi("for f in locate.md compare.md conclude.md; do printf '内容\\n' > \"$f\"; done\necho 通过");
    fix.run_full(true, &["task", "--new", "compare-course-profile", "--workflow", "compare-course-profile"]);

    let record = fix.task_yaml("compare-course-profile");
    assert!(record.contains("workflows:"), "运行上下文里的工作流目录没记:\n{record}");

    for step in ["locate", "compare", "conclude"] {
        let stepped = fix.run_recorded(&["task", "compare-course-profile", "--next"]);
        assert!(stepped.ok(), "--next 走 {step} 没跑通: {}", stepped.crop());
    }
    assert!(
        fix.task_yaml("compare-course-profile").matches("ok: true").count() >= 3,
        "三步都该记一笔"
    );

    let status = fix.run_recorded(&["task", "compare-course-profile"]);
    assert!(status.ok(), "不给 --workflows 就看不了任务: {}", status.crop());
}

// 场景：语境条目粗加工进材料——几步依次走完，粗加工落到 materials/<分类>/index.md，
// 两道 human 闸门挂进报告。
// 用例：四
#[test]
fn scenario_context_entries_into_material() {
    let fix = Fixture::new("material");
    fix.workflow(
        "context-to-profile",
        "name: context-to-profile\ndescription: 语境条目粗加工进材料库\nsteps:\n- name: pull\n  description: 拉语境并把条目清单写进报告\n  criteria:\n  - executor: rule\n    description: 条目清单在\n    path: pull.md\n- name: classify\n  description: 逐条认分类\n  criteria:\n  - executor: rule\n    description: 分类写下来了\n    path: classify.md\n  - executor: human\n    description: 分类裁决\n- name: coarsen\n  description: 粗加工写进 materials/<分类>/index.md\n  criteria:\n  - executor: rule\n    description: 材料格在\n    path: materials/课程/index.md\n- name: move-out\n  description: 把已迁出的条目从语境删掉\n  criteria:\n  - executor: rule\n    description: 迁出记录在\n    path: move-out.md\n- name: commit\n  description: 分层提交推送再回工作区更新指针\n  criteria:\n  - executor: rule\n    description: 提交完了\n    path: commit.md\n  - executor: human\n    description: 创始人点头\n",
    );
    fix.pi(
        "printf '清单\\n' > pull.md\nprintf '分类\\n' > classify.md\nmkdir -p materials/课程\nprintf '粗加工\\n' > materials/课程/index.md\nprintf '迁出\\n' > move-out.md\nprintf '提交\\n' > commit.md\necho 通过",
    );
    fix.run_full(true, &["task", "--new", "context-to-profile", "--workflow", "context-to-profile"]);

    for step in ["pull", "classify", "coarsen", "move-out", "commit"] {
        let stepped = fix.run_recorded(&["task", "context-to-profile", "--next"]);
        assert!(stepped.ok(), "--next 走 {step} 没跑通: {}", stepped.crop());
    }
    assert!(
        fix.task_yaml("context-to-profile").matches("ok: true").count() >= 5,
        "五步都该记一笔"
    );
    assert!(fix.root.join("materials/课程/index.md").is_file(), "粗加工的材料格没落盘");
    let report = fix.report("context-to-profile");
    assert!(
        report.contains("分类裁决") && report.contains("创始人点头"),
        "两道 human 闸门该挂进报告:\n{report}"
    );
}

// 场景：人做的步骤人记一笔——human 步骤程序不抢着做，人做完用 --done 记一笔，
// 照常核 rule 判据、把闸门列进报告，--note 的原话进流水。
// 用例：五
#[test]
fn scenario_human_step_recorded_by_hand() {
    let fix = Fixture::new("hand");
    fix.workflow(
        "数据归仓",
        "name: 数据归仓\ndescription: 数据归仓\nsteps:\n- name: 材料\n  executor: human\n  description: 把材料归位\n  criteria:\n  - executor: rule\n    description: AGENTS 在\n    path: AGENTS.md\n- name: 指令\n  executor: human\n  description: 把指令写下来\n  criteria:\n  - executor: rule\n    description: 指令在\n    path: 目标.md\n  - executor: human\n    description: 创始人过目\n",
    );
    fix.file("AGENTS.md", "# 约定\n");
    fix.file("目标.md", "# 目标\n");
    fix.run_full(false, &["task", "--new", "数据归仓", "--workflow", "数据归仓"]);

    let first = fix.run_recorded(&["task", "数据归仓", "--done", "材料", "--note", "AGENTS.md、日志"]);
    assert!(first.ok(), "--done 材料 没跑通: {}", first.crop());
    let second = fix.run_recorded(&["task", "数据归仓", "--done", "指令", "--note", "目标/步骤/验收 已写"]);
    assert!(second.ok(), "--done 指令 没跑通: {}", second.crop());

    let record = fix.task_yaml("数据归仓");
    assert!(
        record.contains("step: 材料") && record.contains("AGENTS.md、日志"),
        "第一笔的 note 该进流水:\n{record}"
    );
    assert!(
        record.contains("step: 指令") && record.contains("目标/步骤/验收 已写"),
        "第二笔的 note 该进流水:\n{record}"
    );
    assert!(!record.contains("ok: false"), "human 步骤按 note 记应算过:\n{record}");
    assert!(
        fix.report("数据归仓").contains("创始人过目"),
        "human 闸门该列进报告"
    );
}
