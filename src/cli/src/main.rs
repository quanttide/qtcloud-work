use clap::{Parser, Subcommand};

/// 量潮知识工作云 CLI —— 知识工作云 API 的辅助入口（主要供 AI 使用）
///
/// 纯服务端客户端：只对接已部署 provider 的 HTTP 接口，不读写本地文件。
/// API 基地址：`--server <BASE_URL>` > 环境变量 `QTCLOUD_WORK_API_BASE_URL` > 默认系统级网关
/// `https://api.quanttide.com/qtcloud-work`（与 studio/provider 约定一致）。
#[derive(Parser)]
#[command(name = "qtcloud-work", version, about = "量潮知识工作云 CLI：知识工作云服务入口")]
struct Cli {
    /// API 基地址覆盖（默认 https://api.quanttide.com/qtcloud-work；未设时读环境变量 QTCLOUD_WORK_API_BASE_URL）
    #[arg(long, global = true)]
    server: Option<String>,
    /// 以 JSON 输出（AI 友好，直接透传服务端响应）
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 探活（GET /health）
    Health,
}

/// 生产 API 基地址（系统级 API 网关，与其它 qtcloud 服务约定一致）
const DEFAULT_API_BASE: &str = "https://api.quanttide.com/qtcloud-work";

/// 解析 API 基地址：--server 参数 > 环境变量 QTCLOUD_WORK_API_BASE_URL > 默认网关
fn resolve_base(cli_server: &Option<String>) -> String {
    if let Some(s) = cli_server {
        if !s.trim().is_empty() {
            return s.trim_end_matches('/').to_string();
        }
    }
    match std::env::var("QTCLOUD_WORK_API_BASE_URL") {
        Ok(s) if !s.trim().is_empty() => s.trim_end_matches('/').to_string(),
        _ => DEFAULT_API_BASE.to_string(),
    }
}

fn exit_err(e: &str) -> ! {
    eprintln!("错误: {e}");
    std::process::exit(1);
}

/// health：GET /health，直接透传服务端响应
fn health(base: &str, json: bool) {
    let url = format!("{base}/health");
    let resp = ureq::get(&url).call().map_err(|e| match e {
        ureq::Error::Status(code, r) => format!(
            "HTTP {code}: {}",
            r.into_string().unwrap_or_default()
        ),
        other => format!("请求 {url} 失败: {other}"),
    });
    let resp = match resp {
        Ok(r) => r,
        Err(e) => exit_err(&e),
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

fn main() {
    let cli = Cli::parse();
    let base = resolve_base(&cli.server);

    match &cli.command {
        Command::Health => health(&base, cli.json),
    }
}
