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
        "search", "catalog", "audit", "material", "workflow", "order", "health", "help",
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

/// 导览（`help`）列出全部命令：加命令忘了写进导览，这条会红。
#[test]
fn 导览列出全部命令() {
    let fix = Fixture::new("help");
    let out = fix.run(false, &["help"]);
    assert!(out.ok(), "help 没跑通: {}", out.crop());
    for name in [
        "search", "catalog", "audit", "material", "workflow", "order", "health", "help",
    ] {
        assert!(out.crop().contains(name), "导览少了 {name}: {}", out.crop());
    }
    let bad = fix.run(false, &["help", "查无此命令"]);
    assert!(!bad.ok(), "认不出的名字该报错: {}", bad.crop());
}

/// 每条命令的帮助里要有例子：加命令忘了写例子，这条会红。
#[test]
fn 每条命令的帮助都有例子() {
    let fix = Fixture::new("help-examples");
    for name in [
        "search", "catalog", "audit", "material", "workflow", "order", "help", "health",
    ] {
        let out = fix.run(false, &[name, "--help"]);
        assert!(out.ok(), "{name} --help 没跑通: {}", out.crop());
        let shown = format!("{}{}", out.crop(), out.crop());
        assert!(
            shown.contains("例子：") && shown.contains("qtcloud-work"),
            "{name} 的帮助里没有例子: {}",
            out.crop()
        );
    }
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

    // order 的 data 是新模型的契约：有 id / workflow_id / records，没有 v1 的 log / gates / start。
    let made = fix.run_ledger(&["workflow", "create", "试一条", "--steps", "一步"]);
    assert!(made.ok(), "{}", made.crop());
    let made = fix.run_ledger(&["order", "create", "试一条", "--workflow", "试一条"]);
    assert!(made.ok(), "{}", made.crop());
    let shown = fix.run_ledger(&["order", "show", "试一条", "--json"]);
    assert!(shown.ok(), "{}", shown.crop());
    for key in ["\"id\"", "\"workflow_id\"", "\"records\""] {
        assert!(
            shown.stdout().contains(key),
            "order --json 少了 {key}: {}",
            shown.crop()
        );
    }
    for gone in ["\"log\"", "\"gates\"", "\"start\""] {
        assert!(
            !shown.stdout().contains(gone),
            "order --json 不该再有 v1 字段 {gone}: {}",
            shown.crop()
        );
    }
}

/// 依赖方向：聚合与服务不依赖入口层（重构搬家时最容易被顺手破坏的一条）。
#[test]
fn 动作层不依赖入口层() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let layers = [
        "workspace/mod.rs",
        "workspace/check.rs",
        "workspace/locate.rs",
        "workspace/place.rs",
        "workspace/progress.rs",
        "order/mod.rs",
        "order/actions.rs",
        "order/execute.rs",
        "order/inspect.rs",
        "order/journal.rs",
        "order/model.rs",
        "order/record.rs",
        "order/ai.rs",
        "workflow/mod.rs",
        "workflow/actions.rs",
        "workflow/model.rs",
        "workflow/read.rs",
        "workflow/yaml.rs",
        "events.rs",
        "ids.rs",
        "artifact/mod.rs",
        "artifact/model.rs",
        "criterion/mod.rs",
        "criterion/model.rs",
        "search/mod.rs",
        "health.rs",
        "audit/mod.rs",
        "catalog/mod.rs",
        "material/mod.rs",
    ];
    for file in layers {
        let text = std::fs::read_to_string(src.join(file)).expect("读源码");
        assert!(
            !text.contains("crate::cli"),
            "{file} 不该依赖入口模块 cli（聚合与服务要和入口层分开）"
        );
    }
}
