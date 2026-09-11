//! 量潮知识工作云 CLI —— 本地知识工作做法（实验室 kg 的等价物）加 provider 探活。
//!
//! 命令行只解析参数、定位工作区与数据仓、把动作层算出的结果印出来；
//! 算法都在动作层（report.rs / task.rs / workflow.rs / checks.rs 等），命令行与窗口共用。

mod assets;
mod catalog;
mod checks;
mod material;
mod records;
mod report;
mod task;
mod workflow;

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

/// 量潮知识工作云 CLI —— 本地知识工作与知识工作云 API 的辅助入口（主要供 AI 使用）。
///
/// 本地动作按工作流的定义走：工作区、数据仓与工作流目录三处位置决定动作在哪跑。
#[derive(Parser)]
#[command(
    name = "qtcloud-work",
    version,
    about = "量潮知识工作云 CLI：知识工作与知识工作云服务入口"
)]
struct Cli {
    /// 工作区根；任务上的动作不写时用任务里记的，工作区上的动作不写时用当前目录
    #[arg(long, global = true)]
    root: Option<PathBuf>,
    /// 数据仓（任务与产物草稿）
    #[arg(long, global = true)]
    data: Option<PathBuf>,
    /// 工作流目录，默认 <数据仓>/workflows/
    #[arg(long, global = true)]
    workflows: Option<PathBuf>,
    /// 以 JSON 输出到标准输出
    #[arg(long, global = true)]
    json: bool,
    /// 结果另存一份到文件
    #[arg(long, global = true)]
    out: Option<PathBuf>,
    /// 预演：只说要写什么，不落盘
    #[arg(long = "dry-run", global = true)]
    dry_run: bool,
    /// API 基地址覆盖（provider 探活用）
    #[arg(long, global = true)]
    server: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 按名找文档
    Find {
        /// 要找的名字
        name: String,
        /// 连正文一起打印
        #[arg(long)]
        show: bool,
    },
    /// 按资产种类列出工作区里的全部条目
    Catalog,
    /// 审计工作区：资产表有而工作区无、工作区有而未登记
    Audit {
        /// 补建缺的文档格
        #[arg(long)]
        make: bool,
    },
    /// 列材料的四字段与阶段
    Material {
        /// 要看的路径；不给就扫 data/journal 与 data/profile 下的 md
        paths: Vec<String>,
    },
    /// 工作流：串联的步骤
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
        /// 导进来一份工作流文件
        #[arg(long = "import")]
        import_from: Option<PathBuf>,
        /// 导入时另起名字
        #[arg(long = "as", default_value = "")]
        as_name: String,
    },
    /// 任务：工作流的一次执行实例
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
    /// 探活（GET /health）
    Health,
}

const DEFAULT_API_BASE: &str = "https://api.quanttide.com/qtcloud-work";

/// 解析 API 基地址：--server 参数 > 环境变量 QTCLOUD_WORK_API_BASE_URL > 默认网关
fn resolve_base(cli_server: &Option<String>) -> String {
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

fn health(base: &str, json: bool) {
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

/// 工作区根：给了就用给的，没给就用当前目录。
fn workspace_root(cli: &Cli) -> Result<PathBuf, String> {
    match &cli.root {
        Some(root) => Ok(root.clone()),
        None => std::env::current_dir().map_err(|e| e.to_string()),
    }
}

/// 数据仓：任务与产物草稿落在这里，必须显式给出。
fn data_dir(cli: &Cli) -> Result<PathBuf, String> {
    cli.data
        .clone()
        .ok_or_else(|| "请给数据仓：--data <路径>（任务与产物草稿落在这里）".to_string())
}

fn emit(result: report::Result, cli: &Cli) -> i32 {
    if let Some(out) = &cli.out {
        catalog::write_json(out, &result.to_json());
    }
    if cli.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&result.to_json()).unwrap_or_default()
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

fn main() {
    let cli = Cli::parse();
    let code = run(&cli);
    std::process::exit(code);
}

fn run(cli: &Cli) -> i32 {
    match &cli.command {
        Command::Health => {
            health(&resolve_base(&cli.server), cli.json);
            0
        }

        Command::Find { name, show } => {
            let root = workspace_root(cli).unwrap_or_else(|e| fail(&e));
            let result =
                report::find(&root, name, *show).with_first(format!("工作区：{}", root.display()));
            emit(result, cli)
        }
        Command::Catalog => {
            let root = workspace_root(cli).unwrap_or_else(|e| fail(&e));
            emit(
                report::catalog(&root).with_first(format!("工作区：{}", root.display())),
                cli,
            )
        }
        Command::Audit { make } => {
            let root = workspace_root(cli).unwrap_or_else(|e| fail(&e));
            if cli.dry_run {
                let missing = assets::missing(&root);
                let mut lines = vec![
                    format!("工作区：{}", root.display()),
                    "预演：不落盘".to_string(),
                ];
                lines.extend(
                    missing
                        .iter()
                        .map(|a| format!("会补建：{}（{}）", a.kind, a.name)),
                );
                return emit(report::Result::lines(true, lines), cli);
            }
            emit(
                report::audit(&root, *make).with_first(format!("工作区：{}", root.display())),
                cli,
            )
        }
        Command::Material { paths } => {
            let root = workspace_root(cli).unwrap_or_else(|e| fail(&e));
            let paths = if paths.is_empty() {
                None
            } else {
                Some(paths.as_slice())
            };
            emit(
                report::material(&root, paths).with_first(format!("工作区：{}", root.display())),
                cli,
            )
        }

        Command::Workflow {
            name,
            list,
            new,
            steps,
            note,
            export,
            import_from,
            as_name,
        } => {
            let data = data_dir(cli).unwrap_or_else(|e| fail(&e));
            let workflows = cli.workflows.as_deref();
            if *list {
                return emit(report::workflow_list(&data, workflows), cli);
            }
            if *new {
                if cli.dry_run {
                    return emit(
                        report::Result::lines(
                            true,
                            vec![format!("预演：会写 {},{}", data.display(), steps)],
                        ),
                        cli,
                    );
                }
                let steps: Vec<String> = steps
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                return emit(
                    report::workflow_new(
                        &data,
                        name.as_deref().unwrap_or(""),
                        &steps,
                        note,
                        workflows,
                    ),
                    cli,
                );
            }
            if let Some(source) = import_from {
                if cli.dry_run {
                    return emit(
                        report::Result::lines(
                            true,
                            vec![format!("预演：会导入 {}", source.display())],
                        ),
                        cli,
                    );
                }
                return emit(
                    report::workflow_import(&data, source, as_name, workflows),
                    cli,
                );
            }
            let Some(name) = name else {
                return emit(
                    report::Result::lines(
                        false,
                        vec![
                            "用法：qtcloud-work workflow <名字>，或 --list / --new / --import"
                                .to_string(),
                        ],
                    ),
                    cli,
                );
            };
            if let Some(target) = export {
                if cli.dry_run {
                    return emit(
                        report::Result::lines(
                            true,
                            vec![format!("预演：会导出到 {}", target.display())],
                        ),
                        cli,
                    );
                }
                return emit(report::workflow_export(&data, name, target, workflows), cli);
            }
            emit(report::workflow_show(&data, name, workflows), cli)
        }

        Command::Task {
            name,
            list,
            new,
            workflow,
            next,
            done,
            note,
            journal,
        } => {
            let data = data_dir(cli).unwrap_or_else(|e| fail(&e));
            let root = cli.root.as_deref();
            let workflows = cli.workflows.as_deref();
            if *list {
                return emit(report::task_list(root, &data, workflows), cli);
            }
            if *new {
                let Some(name) = name else {
                    return emit(
                        report::Result::lines(
                            false,
                            vec![
                                "用法：qtcloud-work task --new <名字> --workflow <工作流>"
                                    .to_string(),
                            ],
                        ),
                        cli,
                    );
                };
                if cli.dry_run {
                    return emit(
                        report::Result::lines(
                            true,
                            vec![format!(
                                "预演：会起任务 {name}（数据仓 {}）",
                                data.display()
                            )],
                        ),
                        cli,
                    );
                }
                let root = workspace_root(cli).unwrap_or_else(|e| fail(&e));
                return emit(
                    report::task_new(&root, &data, name, workflow, workflows),
                    cli,
                );
            }
            let Some(name) = name else {
                return emit(
                    report::Result::lines(false, vec!["用法：qtcloud-work task <名字>，或 task --list / --new <名字> --workflow <工作流>".to_string()]),
                    cli,
                );
            };
            if let Some(words) = journal {
                if cli.dry_run {
                    return emit(
                        report::Result::lines(true, vec![format!("预演：会给 {name} 记日志")]),
                        cli,
                    );
                }
                return emit(
                    report::task_journal(root, &data, name, words, workflows),
                    cli,
                );
            }
            if *next {
                if cli.dry_run {
                    return emit(
                        report::Result::lines(true, vec![format!("预演：会走 {name} 的下一步")]),
                        cli,
                    );
                }
                return emit(
                    report::task_step(root, &data, name, "", note, true, workflows),
                    cli,
                );
            }
            if let Some(step) = done {
                if cli.dry_run {
                    return emit(
                        report::Result::lines(true, vec![format!("预演：会记 {name} 的 {step}")]),
                        cli,
                    );
                }
                return emit(
                    report::task_step(root, &data, name, step, note, false, workflows),
                    cli,
                );
            }
            emit(report::task_status(root, &data, name, workflows), cli)
        }
    }
}

fn fail(message: &str) -> ! {
    eprintln!("错误：{message}");
    std::process::exit(1);
}

/// `find` 等动作在 `report` 里需要 `Path` 判断，这里留个引用以免警告。
#[allow(dead_code)]
fn _path_kind(path: &Path) -> bool {
    path.is_dir()
}
