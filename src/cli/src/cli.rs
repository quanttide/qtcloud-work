//! 量潮知识工作云 CLI —— 入口与发射。
//!
//! 这一层只做三件事：clap 定义、定位（工作区根 / 账本 / 工作流目录 / 产物落点）、
//! 把结果发射出去。参数分派与业务调用下移到 `handlers`；算法都在聚合与服务里，
//! 结果都走 `crate::outcome::Outcome` 那一层。位置不进模型：全部由启动参数装载。

mod commands;
mod emit;
mod handlers;

use clap::Parser;
use commands::{Command, OrderCommand, WorkflowCommand};
use std::path::PathBuf;

pub(crate) use emit::emit;

/// 量潮知识工作云 CLI —— 本地知识工作与知识工作云 API 的辅助入口（主要供 AI 使用）。
///
/// 本地动作按工作流的定义走：工作区根、账本与产物落点决定动作在哪跑。
#[derive(Parser)]
#[command(
    name = "qtcloud-work",
    version,
    about = "量潮知识工作云 CLI：知识工作与知识工作云服务入口",
    disable_help_subcommand = true,
    disable_help_flag = true
)]
pub(crate) struct Cli {
    /// 看帮助：当前命令的选项与例子
    #[arg(short = 'h', long = "help", action = clap::ArgAction::Help, global = true)]
    help: Option<bool>,

    /// 工作区根（判据路径的基准）；装载顺序 命令行 > 环境变量 QTCLOUD_WORK_ROOT > 往上找到含 data/journal 的第二大脑
    #[arg(long, global = true)]
    pub(crate) root: Option<PathBuf>,
    /// 账本（工作区身份、工单、事件）；缺省 $XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>
    #[arg(long, global = true)]
    pub(crate) data: Option<PathBuf>,
    /// 工作流目录，缺省跟在账本里
    #[arg(long, global = true)]
    pub(crate) workflows: Option<PathBuf>,
    /// 产物落点（报告与日志）；缺省 <工作区根>/artifacts
    #[arg(long, global = true)]
    pub(crate) artifacts: Option<PathBuf>,
    /// 以 JSON 输出到标准输出
    #[arg(long, global = true)]
    pub(crate) json: bool,
    /// 结果另存一份到文件
    #[arg(long, global = true)]
    pub(crate) out: Option<PathBuf>,
    /// 预演：只说要写什么，不落盘
    #[arg(long = "dry-run", global = true)]
    pub(crate) dry_run: bool,
    /// API 基地址覆盖（provider 探活用）
    #[arg(long, global = true)]
    pub(crate) server: Option<String>,

    #[command(subcommand)]
    pub(crate) command: Command,
}

/// 入口：解析环境里的参数、跑一遍、把退出码交出去（`main.rs` 只有一行调它）。
pub fn run_from_env() -> i32 {
    let cli = Cli::parse();
    run(&cli)
}

fn run(cli: &Cli) -> i32 {
    match &cli.command {
        Command::Help { topic } => handlers::help(topic.as_deref(), cli),
        Command::Health => {
            handlers::health(cli);
            0
        }
        Command::Search { name, show } => handlers::search(name, *show, cli),
        Command::Catalog => handlers::catalog(cli),
        Command::Audit { make } => handlers::audit(*make, cli),
        Command::Material { paths } => handlers::material(paths, cli),
        Command::Workflow(command) => handlers::workflow(command, cli),
        Command::Order(command) => handlers::order(command, cli),
    }
}
