//! 契约快照：重构不该改掉人说得出、机器读得到的接口。
//!
//! 用例：一（起一件任务并走一步）、用例：二（三类判据）

mod common;

use common::{Fixture, bin};
use std::process::Command;

fn help(args: &[&str]) -> String {
    let out = Command::new(bin())
        .args(args)
        .output()
        .expect("跑 qtcloud-work --help");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// 子命令清单：重构时增删子命令，快照会红，改的时候必须是有意的。
#[test]
fn 子命令清单是契约() {
    let text = help(&["--help"]);
    let mut names: Vec<String> = Vec::new();
    let mut inside = false;
    for line in text.lines() {
        if line.starts_with("Commands:") {
            inside = true;
            continue;
        }
        if !inside {
            continue;
        }
        if line.trim().is_empty() || line.starts_with("Options:") {
            break;
        }
        if let Some(name) = line.split_whitespace().next() {
            names.push(name.to_string());
        }
    }
    let want = [
        "find", "catalog", "audit", "material", "workflow", "task", "health", "help",
    ];
    for name in want {
        assert!(
            names.iter().any(|n| n == name),
            "少了子命令：{name}（现有 {names:?}）"
        );
    }
    assert_eq!(
        names.len(),
        want.len(),
        "子命令多了或少了，快照该跟着改（现有 {names:?}）"
    );
}

/// --json 的字段是脚本依赖的契约，只加不改。
#[test]
fn json字段是契约() {
    let fix = Fixture::new("contract");
    fix.file("data/journal/README.md", "# 日志\n");

    let catalog = fix.run(false, &["catalog", "--json"]);
    assert!(catalog.ok(), "{}", catalog.crop());
    for key in ["\"root\"", "\"count\"", "\"entries\""] {
        assert!(
            catalog.crop().contains(key),
            "catalog --json 少了 {key}: {}",
            catalog.crop()
        );
    }

    // 缺资产时 audit 的退出码是 1，JSON 照样是契约
    let audit = fix.run(false, &["audit", "--json"]);
    for key in ["\"root\"", "\"result\""] {
        assert!(
            audit.crop().contains(key),
            "audit --json 少了 {key}: {}",
            audit.crop()
        );
    }

    let material = fix.run(false, &["material", "--json"]);
    for key in ["\"count\"", "\"materials\""] {
        assert!(
            material.crop().contains(key),
            "material --json 少了 {key}: {}",
            material.crop()
        );
    }
}

/// 依赖方向：动作层不依赖入口层（重构搬家时最容易被顺手破坏的一条）。
#[test]
fn 动作层不依赖入口层() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let layers = [
        "outcome.rs",
        "artifact.rs",
        "audit.rs",
        "catalog.rs",
        "material.rs",
        "workflow.rs",
        "task.rs",
    ];
    for file in layers {
        let text = std::fs::read_to_string(src.join(file)).expect("读源码");
        assert!(
            !text.contains("crate::cli"),
            "{file} 不该依赖入口模块 cli（动作层与入口层要分开）"
        );
    }
}
