//! 用例测试共用的夹具（Rust 惯例：共用代码放 tests/common/）：临时工作区、数据仓、工作流目录、`pi` 桩，与跑命令的壳。
//!
//! 每个场景一个测试文件，共用的东西放这里；文件之间互不依赖，加一个场景就加一个文件。

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_qtcloud-work")
}

pub fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("读 {} 失败: {e}", path.display()))
}

pub fn write(path: &Path, body: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("建父目录");
    }
    std::fs::write(path, body).expect("写文件");
}

/// 一次跑出来的结果：退出码与两路输出。
pub struct Run {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

impl Run {
    pub fn ok(&self) -> bool {
        self.code == Some(0)
    }
    pub fn crop(&self) -> String {
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
pub struct Fixture {
    pub root: PathBuf,
    pub data: PathBuf,
    pub flows: PathBuf,
    stub: PathBuf,
}

impl Fixture {
    pub fn new(tag: &str) -> Self {
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

    pub fn workflow(&self, name: &str, body: &str) {
        write(&self.flows.join(format!("{name}.yaml")), body);
    }

    pub fn file(&self, rel: &str, body: &str) {
        write(&self.root.join(rel), body);
    }

    /// 造一个 `pi` 桩：按 body 的 shell 片段干活（在工作区根下），并回一句「通过」。
    /// 传一个必定失败的片段，就得到「AI 没跑成」那一半场景。
    pub fn pi(&self, body: &str) {
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
    pub fn run(&self, stub: bool, args: &[&str]) -> Run {
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
    pub fn run_full(&self, stub: bool, args: &[&str]) -> Run {
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
    pub fn run_recorded(&self, args: &[&str]) -> Run {
        let mut full: Vec<&str> = vec!["--data", self.data.to_str().unwrap()];
        full.extend_from_slice(args);
        self.run(false, &full)
    }

    pub fn task_yaml(&self, name: &str) -> String {
        read(&self.data.join("tasks").join(format!("{name}.yaml")))
    }

    pub fn report(&self, name: &str) -> String {
        read(&self.data.join("artifacts/report").join(format!("{name}.md")))
    }
}

/// 一条最简工作流：一步「问候」，执行者是 AI，判据一条 rule（问候文件含「你好」）。
pub const 冒烟工作流: &str = "name: AI冒烟\ndescription: 冒烟\nsteps:\n- name: 问候\n  description: 写一行中文问候到 问候.md\n  criteria:\n  - executor: rule\n    description: 问候落在\n    file: 问候.md\n    contains: 你好\n";
