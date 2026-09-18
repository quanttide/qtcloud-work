//! 用例测试共用的夹具（Rust 惯例：共用代码放 tests/common/）：临时工作区、账本、
//! 工作流目录、产物落点、`pi` 桩，与跑命令的壳。
//!
//! 每个场景一个测试文件，共用的东西放这里；文件之间互不依赖，加一个场景就加一个文件。

#![allow(dead_code)]

use serde_json::Value as Json;
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
    pub fn stdout(&self) -> &str {
        &self.stdout
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

/// 一个临时工作区：根、账本、工作流目录、产物落点、`pi` 桩各一份。
///
/// 账本与工作流目录故意分开放（账本与固定资产目录各一处）；
/// 每次跑都把 `XDG_DATA_HOME` 指进临时目录，缺省账本不会写到真机器上。
pub struct Fixture {
    pub root: PathBuf,
    pub data: PathBuf,
    pub flows: PathBuf,
    pub artifacts: PathBuf,
    pub xdg: PathBuf,
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
            data: root.join("ledger"),
            flows: root.join("data/profile/iGuo/workflows"),
            artifacts: root.join("outputs"),
            xdg: root.join("xdg"),
            stub: root.join("stub"),
            root,
        };
        std::fs::create_dir_all(&fix.flows).expect("建工作流目录");
        // 缺省摆一个「不该被调到」的桩：测试忘了 `fix.pi()`，真调到 `pi` 就当场炸，
        // 绝不落到机器上的真 `pi`（慢，还会误判通过）。
        fix.pi("echo 测试里没摆 pi 桩：先 fix.pi() >&2\nexit 3");
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

    /// 跑可执行文件。`stub` 为真时把 `pi` 桩前置进 PATH；`XDG_DATA_HOME` 一律指进临时目录。
    pub fn run(&self, stub: bool, args: &[&str]) -> Run {
        self.run_in(&self.root, &[], stub, args)
    }

    /// 带上三处位置跑（工作区、账本、工作流目录、产物落点都显式给）。
    pub fn run_full(&self, stub: bool, args: &[&str]) -> Run {
        let mut full: Vec<&str> = vec![
            "--root",
            self.root.to_str().unwrap(),
            "--data",
            self.data.to_str().unwrap(),
            "--workflows",
            self.flows.to_str().unwrap(),
            "--artifacts",
            self.artifacts.to_str().unwrap(),
        ];
        full.extend_from_slice(args);
        self.run(stub, &full)
    }

    /// 只给账本与定义目录跑（工作区根与产物落点靠缺省）。
    /// 交给 `pi` 的地方一律用桩顶替，免得测试依赖真模型。
    pub fn run_ledger(&self, args: &[&str]) -> Run {
        let mut full: Vec<&str> = vec![
            "--data",
            self.data.to_str().unwrap(),
            "--workflows",
            self.flows.to_str().unwrap(),
        ];
        full.extend_from_slice(args);
        self.run(true, &full)
    }

    /// 在指定目录、带指定环境变量跑：缺省根的向上搜索与 `QTCLOUD_WORK_ROOT` 靠它。
    pub fn run_in(&self, dir: &Path, vars: &[(&str, &str)], stub: bool, args: &[&str]) -> Run {
        let mut cmd = Command::new(bin());
        cmd.current_dir(dir)
            .args(args)
            .env("XDG_DATA_HOME", &self.xdg);
        for (key, value) in vars {
            cmd.env(key, value);
        }
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

    pub fn order_yaml(&self, name: &str) -> String {
        read(&self.data.join("workorders").join(format!("{name}.yaml")))
    }

    /// 派生凭证：从 `workflow show --json` 的 `data.workflow_id` 读出。
    pub fn workflow_id(&self, name: &str) -> String {
        let shown = self.run_ledger(&["workflow", "show", name, "--json"]);
        assert!(shown.ok(), "workflow show 没跑通: {}", shown.crop());
        let payload: Json =
            serde_json::from_str(shown.stdout().trim()).expect("workflow show 的 JSON 读不出来");
        payload["data"]["workflow_id"]
            .as_str()
            .expect("data.workflow_id 缺了")
            .to_string()
    }

    /// 直接写一件工单（给定流水），用来摆状态：真值表靠它。
    ///
    /// `workflow_id` 用 [`Self::workflow_id`] 取真值；记录的 id / step_id 用占位 UUID——
    /// 账本只验格式，不验它们是哪一枚派生出来的。
    pub fn order_with(&self, name: &str, workflow_id: &str, records: &[(&str, bool)]) {
        let mut body = format!(
            "id: 11111111-1111-4111-8111-111111111111\nname: {name}\ndescription: 试\nworkflow_id: {workflow_id}\ncreated_at: 2026-09-11T10:00:00\nrecords:\n"
        );
        for (index, (step, ok)) in records.iter().enumerate() {
            body.push_str(&format!(
                "- id: 22222222-2222-4222-8222-{:012x}\n  seq: {}\n  created_at: 2026-09-11T10:0{}\n  order_id: 11111111-1111-4111-8111-111111111111\n  step: {step}\n  step_id: 33333333-3333-4333-8333-{:012x}\n  description: 试\n  is_succeeded: {ok}\n",
                index,
                index + 1,
                index,
                index
            ));
        }
        write(
            &self.data.join("workorders").join(format!("{name}.yaml")),
            &body,
        );
    }
}

/// 一条最简工作流：一步「问候」，执行者是 AI，判据一条 rule（问候文件含「你好」）。
pub const 冒烟工作流: &str = "name: AI冒烟\ndescription: 冒烟\nsteps:\n- name: 问候\n  description: 写一行中文问候到 问候.md\n  criteria:\n  - executor: rule\n    description: 问候落在\n    file: 问候.md\n    contains: 你好\n";
