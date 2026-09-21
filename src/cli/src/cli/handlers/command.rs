//! 工作流与工单那一组的子命令分派。

use super::super::{Cli, OrderCommand, WorkflowCommand, emit};
use crate::outcome::Outcome;
use crate::workspace::{LocalWorkspace, short};

/// 装载：三处位置由启动参数定，写动作另开账本。
fn locate(cli: &Cli) -> LocalWorkspace {
    LocalWorkspace::resolve(
        cli.root.as_deref(),
        cli.data.as_deref(),
        cli.workflows.as_deref(),
        cli.artifacts.as_deref(),
    )
}

pub(crate) fn workflow(args: &WorkflowCommand, cli: &Cli) -> i32 {
    let locate = locate(cli);
    match args {
        WorkflowCommand::Create { name, steps, note } => {
            let split: Vec<String> = steps
                .split(',')
                .map(|item| item.trim().to_string())
                .filter(|item| !item.is_empty())
                .collect();
            if cli.dry_run {
                return emit(
                    Outcome::lines(
                        true,
                        vec![format!(
                            "预演：会写 {}（步骤 {}）",
                            locate
                                .workflows_dir()
                                .join(format!("{name}.yaml"))
                                .display(),
                            split.join("、")
                        )],
                    ),
                    cli,
                );
            }
            emit(
                crate::workflow::workflow_create(&locate, name, &split, note),
                cli,
            )
        }
        WorkflowCommand::Show { name } => emit(crate::workflow::workflow_show(&locate, name), cli),
        WorkflowCommand::List => emit(crate::workflow::workflow_list(&locate), cli),
        WorkflowCommand::Check { name } => {
            emit(crate::workflow::workflow_check(&locate, name), cli)
        }
        WorkflowCommand::Export { name, target } => {
            if cli.dry_run {
                return emit(
                    Outcome::lines(true, vec![format!("预演：会导出到 {}", target.display())]),
                    cli,
                );
            }
            emit(crate::workflow::workflow_export(&locate, name, target), cli)
        }
        WorkflowCommand::Import { file, as_name } => {
            if cli.dry_run {
                return emit(
                    Outcome::lines(true, vec![format!("预演：会导入 {}", file.display())]),
                    cli,
                );
            }
            emit(
                crate::workflow::workflow_import(&locate, file, as_name),
                cli,
            )
        }
    }
}

pub(crate) fn order(args: &OrderCommand, cli: &Cli) -> i32 {
    let locate = locate(cli);
    match args {
        OrderCommand::Create {
            name,
            workflow,
            description,
        } => {
            let flow = crate::workflow::open(&locate, workflow.trim());
            if !flow.exists() {
                return emit(
                    Outcome::lines(
                        false,
                        vec![format!(
                            "没有这条工作流：{}（qtcloud-work workflow list 看有哪些）",
                            short(&locate.root, &flow.file())
                        )],
                    ),
                    cli,
                );
            }
            if cli.dry_run {
                return emit(
                    Outcome::lines(
                        true,
                        vec![format!(
                            "预演：会开工单 {name}（账本 {}）",
                            locate.workorders_dir().display()
                        )],
                    ),
                    cli,
                );
            }
            emit(
                crate::order::order_create(&locate, name, workflow, description),
                cli,
            )
        }
        OrderCommand::Show { name } => emit(crate::order::order_show(&locate, name), cli),
        OrderCommand::List { workflow } => emit(crate::order::order_list(&locate, workflow), cli),
        OrderCommand::Next { name, note } => {
            if cli.dry_run {
                return emit(
                    Outcome::lines(true, vec![format!("预演：会走 {name} 的下一步")]),
                    cli,
                );
            }
            emit(crate::order::order_next(&locate, name, note), cli)
        }
        OrderCommand::Done { name, step, note } => {
            if cli.dry_run {
                return emit(
                    Outcome::lines(true, vec![format!("预演：会记 {name} 的 {step}")]),
                    cli,
                );
            }
            emit(crate::order::order_done(&locate, name, step, note), cli)
        }
        OrderCommand::Journal { name, words } => {
            if cli.dry_run {
                return emit(
                    Outcome::lines(true, vec![format!("预演：会给 {name} 记日志")]),
                    cli,
                );
            }
            emit(crate::order::order_journal(&locate, name, words), cli)
        }
        OrderCommand::Delete { name } => {
            if cli.dry_run {
                return emit(
                    Outcome::lines(true, vec![format!("预演：会删白纸 {name}")]),
                    cli,
                );
            }
            emit(crate::order::order_delete(&locate, name), cli)
        }
    }
}
