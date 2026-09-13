//! 量潮知识工作云 CLI —— 入口与发射。
//!
//! 这一层只做三件事：clap 定义、定位（工作区 / 数据仓 / 工作流目录）、把结果发射出去。
//! 参数分派与业务调用下移到 `handlers`；算法都在聚合与服务里，结果都走
//! `crate::outcome::Outcome` 那一层。

mod emit;
mod handlers;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub(crate) use emit::emit;

/// 量潮知识工作云 CLI —— 本地知识工作与知识工作云 API 的辅助入口（主要供 AI 使用）。
///
/// 本地动作按工作流的定义走：工作区、数据仓与工作流目录三处位置决定动作在哪跑。
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

    /// 工作区根；任务上的动作不写时用任务里记的，工作区上的动作不写时用当前目录
    #[arg(long, global = true)]
    pub(crate) root: Option<PathBuf>,
    /// 数据仓（任务与产物草稿）
    #[arg(long, global = true)]
    pub(crate) data: Option<PathBuf>,
    /// 工作流目录，默认 <数据仓>/workflows/
    #[arg(long, global = true)]
    pub(crate) workflows: Option<PathBuf>,
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

#[derive(Subcommand)]
pub(crate) enum Command {
    /// 按名找文档
    #[command(
        after_help = "例子：\n  qtcloud-work search 材料\n  qtcloud-work search 材料 --show\n细节看 docs/api-references/search.md"
    )]
    Search {
        /// 要找的名字
        name: String,
        /// 连正文一起打印
        #[arg(long)]
        show: bool,
    },
    /// 按资产类别列出工作区里的全部条目
    #[command(
        after_help = "例子：\n  qtcloud-work catalog\n  qtcloud-work catalog --json\n细节看 docs/api-references/catalog.md"
    )]
    Catalog,
    /// 审计工作区：资产表有而工作区无、工作区有而未登记
    #[command(
        after_help = "例子：\n  qtcloud-work audit\n  qtcloud-work audit --make --dry-run\n细节看 docs/api-references/audit.md"
    )]
    Audit {
        /// 补建缺的文档格
        #[arg(long)]
        make: bool,
    },
    /// 列材料的四字段与阶段
    #[command(
        after_help = "例子：\n  qtcloud-work material\n  qtcloud-work material data/journal/iGuo/2026-09-11.md\n细节看 docs/api-references/material.md"
    )]
    Material {
        /// 要看的路径；不给就扫 data/journal 与 data/profile 下的 md
        paths: Vec<String>,
    },
    /// 工作流：串联的步骤
    #[command(
        after_help = "用法是「不看名字看标志」：--list 列、--new 写、--check 核、--export 存、--import 导；给了名字就是看这一条。\n\n配套：--new 要带 --steps（逗号分开）；--check / --export 要带名字；--import 要带文件，重名用 --as 换一个。\n\n例子：\n  qtcloud-work workflow --list\n  qtcloud-work workflow --new 试一条 --steps 甲,乙 --note 试\n  qtcloud-work workflow 试一条 --check\n细节看 docs/api-references/workflow.md"
    )]
    Workflow {
        /// 工作流名
        name: Option<String>,
        /// 有哪些工作流
        #[arg(long)]
        list: bool,
        /// 写一条工作流
        #[arg(long)]
        new: bool,
        /// 步骤，逗号分开：甲,乙,丙
        #[arg(long, default_value = "")]
        steps: String,
        /// 这条工作流是干什么的
        #[arg(long, default_value = "")]
        note: String,
        /// 存成一份可带走的文件
        #[arg(long)]
        export: Option<PathBuf>,
        /// 核对这条定义：判据里的路径在不在、描述里提到的小节有没有判据
        #[arg(long)]
        check: bool,
        /// 导进来一份工作流文件
        #[arg(long = "import")]
        import_from: Option<PathBuf>,
        /// 导入时另起名字
        #[arg(long = "as", default_value = "")]
        as_name: String,
    },
    /// 任务：工作流的一次执行实例
    #[command(
        after_help = "用法是「不看名字看标志」：--list 列、--new 起、--next 走一步、--done 人为记一步、--journal 写日志；给了名字就是看这一件。\n\n配套：--new 要带 --workflow；--done 可带步骤名（不给就记现场那一步）；--note 只跟 --next / --done 走。\n\n例子：\n  qtcloud-work task --new 试一条 --workflow 试一条\n  qtcloud-work task 试一条 --next\n  qtcloud-work task 试一条 --done 甲 --note 人做的\n细节看 docs/api-references/task.md"
    )]
    Task {
        /// 任务名
        name: Option<String>,
        /// 有哪些任务
        #[arg(long)]
        list: bool,
        /// 起一件任务
        #[arg(long)]
        new: bool,
        /// 跑哪条工作流
        #[arg(long, default_value = "")]
        workflow: String,
        /// 走下一步（AI 执行者交给 pi 跑）
        #[arg(long)]
        next: bool,
        /// 人为地记一步
        #[arg(long)]
        done: Option<String>,
        /// 记一句这一步做了什么
        #[arg(long, default_value = "")]
        note: String,
        /// 日志：写下这次的来龙去脉
        #[arg(long)]
        journal: Option<String>,
    },
    /// 导览：按用途列出命令；给了话题就说那一条的要点
    #[command(
        after_help = "例子：\n  qtcloud-work help\n  qtcloud-work help task\n细节看 docs/api-references/help.md"
    )]
    Help {
        /// 要看要点的话题（命令名）
        topic: Option<String>,
    },
    /// 探活已部署的 provider
    #[command(
        after_help = "例子：\n  qtcloud-work health\n  qtcloud-work --server http://localhost:8080 health\n细节看 docs/api-references/health.md"
    )]
    Health,
}

/// 入口：解析环境里的参数、跑一遍、把退出码交出去（`main.rs` 只有一行调它）。
pub fn run_from_env() -> i32 {
    let cli = Cli::parse();
    run(&cli)
}

pub(crate) fn fail(message: &str) -> ! {
    eprintln!("错误：{message}");
    std::process::exit(1);
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
        Command::Workflow {
            name,
            list,
            new,
            steps,
            note,
            export,
            import_from,
            check,
            as_name,
        } => handlers::workflow(
            handlers::WorkflowArgs {
                name: name.as_deref(),
                list: *list,
                new: *new,
                steps,
                note,
                export: export.as_deref(),
                import_from: import_from.as_deref(),
                check: *check,
                as_name,
            },
            cli,
        ),
        Command::Task {
            name,
            list,
            new,
            workflow,
            next,
            done,
            note,
            journal,
        } => handlers::task(
            handlers::TaskArgs {
                name: name.as_deref(),
                list: *list,
                new: *new,
                workflow,
                next: *next,
                done: done.as_deref(),
                note,
                journal: journal.as_deref(),
            },
            cli,
        ),
    }
}
