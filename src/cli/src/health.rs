//! 探活已部署的 provider（适配：远端 HTTP）。
//!
//! 本地知识工作不依赖它；基地址优先级 `--server` > 环境变量
//! `QTCLOUD_WORK_API_BASE_URL` > 默认网关。

const DEFAULT_API_BASE: &str = "https://api.quanttide.com/qtcloud-work";

/// 解析 API 基地址：--server 参数 > 环境变量 QTCLOUD_WORK_API_BASE_URL > 默认网关
pub fn resolve_base(cli_server: &Option<String>) -> String {
    if let Some(s) = cli_server
        && !s.trim().is_empty()
    {
        return s.trim_end_matches('/').to_string();
    }
    match std::env::var("QTCLOUD_WORK_API_BASE_URL") {
        Ok(s) if !s.trim().is_empty() => s.trim_end_matches('/').to_string(),
        _ => DEFAULT_API_BASE.to_string(),
    }
}

pub fn health(base: &str, json: bool) {
    let url = format!("{base}/health");
    let resp = ureq::get(&url).call().map_err(|e| match e {
        ureq::Error::Status(code, r) => {
            format!("HTTP {code}: {}", r.into_string().unwrap_or_default())
        }
        other => format!("请求 {url} 失败: {other}"),
    });
    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            eprintln!("错误: {e}");
            std::process::exit(1);
        }
    };
    let text = resp.into_string().unwrap_or_default();
    if json {
        match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(v) => println!("{}", serde_json::to_string(&v).unwrap()),
            Err(_) => println!("{}", serde_json::json!({ "raw": text })),
        }
        return;
    }
    println!("✓ qtcloud-work 可用 @ {base}");
    if !text.trim().is_empty() {
        println!("{text}");
    }
}
