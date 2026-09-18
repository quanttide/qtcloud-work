//! clap 的命令树：工作流与工单两组动词式子命令。
//!
//! 命令面与规格端点表一一对应（端点表里没有的操作，命令行里也没有）；
//! 全局选项（位置装载）在 [`super::Cli`]。

use clap::Subcommand;

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
    /// 工作流：过程的定义——一串有序的步骤，每步写着谁做与怎么算完
    #[command(
        after_help = "例子：\n  qtcloud-work workflow create 试一条 --steps 甲,乙\n细节看 docs/api-references/workflow.md",
        subcommand
    )]
    Workflow(WorkflowCommand),
    /// 工单：一次行程的账本——封面落笔即封，流水只增不改
    #[command(
        after_help = "例子：\n  qtcloud-work order create 试一条 --workflow 试一条\n细节看 docs/api-references/order.md",
        subcommand
    )]
    Order(OrderCommand),
    /// 导览：按用途列出命令；给了话题就说那一条的要点
    #[command(
        after_help = "例子：\n  qtcloud-work help\n  qtcloud-work help order\n细节看 docs/api-references/help.md"
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

#[derive(Subcommand)]
pub(crate) enum WorkflowCommand {
    /// 写一条工作流：步骤各配一份判据骨架
    #[command(
        after_help = "例子：\n  qtcloud-work workflow create 试一条 --steps 甲,乙 --note 试\n细节看 docs/api-references/workflow.md"
    )]
    Create {
        /// 工作流名（文件名即工作流名）
        name: String,
        /// 步骤，逗号分开：甲,乙,丙
        #[arg(long, default_value = "")]
        steps: String,
        /// 这条工作流是干什么的
        #[arg(long, default_value = "")]
        note: String,
    },
    /// 看这条工作流的步骤与判据
    #[command(
        after_help = "例子：\n  qtcloud-work workflow show 试一条\n细节看 docs/api-references/workflow.md"
    )]
    Show {
        /// 工作流名
        name: String,
    },
    /// 有哪些工作流
    #[command(
        after_help = "例子：\n  qtcloud-work workflow list\n细节看 docs/api-references/workflow.md"
    )]
    List,
    /// 定义核对：判据路径须在区内、描述点到的小节须有判据覆盖
    #[command(
        after_help = "例子：\n  qtcloud-work workflow check 试一条\n细节看 docs/api-references/workflow.md"
    )]
    Check {
        /// 工作流名
        name: String,
    },
    /// 存成一份可带走的文件
    #[command(
        after_help = "例子：\n  qtcloud-work workflow export 试一条 /tmp/试一条.yaml\n细节看 docs/api-references/workflow.md"
    )]
    Export {
        /// 工作流名
        name: String,
        /// 落到哪（目录则落同名文件）
        target: std::path::PathBuf,
    },
    /// 导进来一份工作流文件（先照 schema 验，重名挡）
    #[command(
        after_help = "例子：\n  qtcloud-work workflow import /tmp/试一条.yaml --as 另一条\n细节看 docs/api-references/workflow.md"
    )]
    Import {
        /// 从哪读
        file: std::path::PathBuf,
        /// 导入时另起名字
        #[arg(long = "as", default_value = "")]
        as_name: String,
    },
}

#[derive(Subcommand)]
pub(crate) enum OrderCommand {
    /// 开工单：凭证与时刻由账本查填，封面落笔即封
    #[command(
        after_help = "例子：\n  qtcloud-work order create 试一条 --workflow 试一条\n细节看 docs/api-references/order.md"
    )]
    Create {
        /// 工单名（工作区内唯一）
        name: String,
        /// 跑哪条工作流
        #[arg(long)]
        workflow: String,
        /// 一句话任务，给执行的人看
        #[arg(long, default_value = "")]
        description: String,
    },
    /// 读全貌：封面加全量流水，进度照流水推导
    #[command(
        after_help = "例子：\n  qtcloud-work order show 试一条\n细节看 docs/api-references/order.md"
    )]
    Show {
        /// 工单名
        name: String,
    },
    /// 列本工作区的工单
    #[command(
        after_help = "例子：\n  qtcloud-work order list\n  qtcloud-work order list --workflow 试一条\n细节看 docs/api-references/order.md"
    )]
    List {
        /// 只列引着这条工作流的
        #[arg(long, default_value = "")]
        workflow: String,
    },
    /// 走下一步：AI 执行者交给 pi 跑，程序核 rule 判据
    #[command(
        after_help = "例子：\n  qtcloud-work order next 试一条 --note 一句话\n细节看 docs/api-references/order.md"
    )]
    Next {
        /// 工单名
        name: String,
        /// 记一句这一步做了什么
        #[arg(long, default_value = "")]
        note: String,
    },
    /// 人记一笔：闸门放行，或人自己做完记一笔
    #[command(
        after_help = "例子：\n  qtcloud-work order done 试一条 甲 --note 人做的\n细节看 docs/api-references/order.md"
    )]
    Done {
        /// 工单名
        name: String,
        /// 步骤名
        step: String,
        /// 记一句这一步做了什么
        #[arg(long, default_value = "")]
        note: String,
    },
    /// 日志：叙事落产物
    #[command(
        after_help = "例子：\n  qtcloud-work order journal 试一条 「这次为什么这么做」\n细节看 docs/api-references/order.md"
    )]
    Journal {
        /// 工单名
        name: String,
        /// 要写进日志的一段话
        words: String,
    },
    /// 删一张白纸：流水非空即拒
    #[command(
        after_help = "例子：\n  qtcloud-work order delete 试一条\n细节看 docs/api-references/order.md"
    )]
    Delete {
        /// 工单名
        name: String,
    },
}
