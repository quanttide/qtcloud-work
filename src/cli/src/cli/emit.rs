//! 发射：把动作算出的结果印出去，并给出退出码。

use super::Cli;
use crate::catalog;
use quanttide_work::outcome::Outcome;
use serde_json::Value as Json;

pub(crate) fn emit(result: Outcome, cli: &Cli) -> i32 {
    if let Some(out) = &cli.out {
        // `--out` 落的是原文那一栏（`--json` 的四样里那一栏的内容）。
        catalog::write_json(out, &result.to_output_json());
    }
    if cli.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&envelope_json(&result)).unwrap_or_default()
        );
    } else {
        for line in &result.lines {
            println!("{line}");
        }
        if !result.ok {
            eprintln!("下一步：看 `qtcloud-work <子命令> --help`，或先补齐上面缺的东西");
        }
    }
    if result.ok { 0 } else { 1 }
}

/// 信封 + 旧键留一轮。
///
/// `--json` 的字段是脚本依赖的契约，只加不改：原文摊在 `data` 里是一轮，
/// 原来的顶层键（`count` / `entries` / `result` …）再留一轮，下一轮删。
fn envelope_json(result: &Outcome) -> Json {
    let mut envelope = result.to_json();
    let (Some(Json::Object(data)), Json::Object(top)) = (&result.data, &mut envelope) else {
        return envelope;
    };
    for (key, value) in data {
        top.entry(key.clone()).or_insert_with(|| value.clone());
    }
    envelope
}
