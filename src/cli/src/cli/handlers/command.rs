//! 工作流与任务那一组的子命令分派。

use super::super::{Cli, emit, fail};
use crate::task;
use crate::workflow;
use crate::workspace;
use quanttide_work::outcome::Outcome;

pub(crate) struct WorkflowArgs<'a> {
    pub(crate) name: Option<&'a str>,
    pub(crate) list: bool,
    pub(crate) new: bool,
    pub(crate) steps: &'a str,
    pub(crate) note: &'a str,
    pub(crate) export: Option<&'a std::path::Path>,
    pub(crate) import_from: Option<&'a std::path::Path>,
    pub(crate) check: bool,
    pub(crate) as_name: &'a str,
}

pub(crate) fn workflow(args: WorkflowArgs<'_>, cli: &Cli) -> i32 {
    let data = workspace::data_dir(cli.data.as_deref()).unwrap_or_else(|e| fail(&e));
    let workflows = cli.workflows.as_deref();
    if args.list {
        return emit(workflow::workflow_list(&data, workflows), cli);
    }
    if args.check {
        let root = workspace::root(cli.root.as_deref()).unwrap_or_else(|e| fail(&e));
        return emit(
            workflow::workflow_check(&data, args.name.unwrap_or(""), &root, workflows),
            cli,
        );
    }
    if args.new {
        if cli.dry_run {
            return emit(
                Outcome::lines(
                    true,
                    vec![format!("预演：会写 {},{}", data.display(), args.steps)],
                ),
                cli,
            );
        }
        let steps: Vec<String> = args
            .steps
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        return emit(
            workflow::workflow_new(&data, args.name.unwrap_or(""), &steps, args.note, workflows),
            cli,
        );
    }
    if let Some(source) = args.import_from {
        if cli.dry_run {
            return emit(
                Outcome::lines(true, vec![format!("预演：会导入 {}", source.display())]),
                cli,
            );
        }
        return emit(
            workflow::workflow_import(&data, source, args.as_name, workflows),
            cli,
        );
    }
    let Some(name) = args.name else {
        return emit(
            Outcome::lines(
                false,
                vec![
                    "用法：qtcloud-work workflow <名字>，或 --list / --new / --import".to_string(),
                ],
            ),
            cli,
        );
    };
    if let Some(target) = args.export {
        if cli.dry_run {
            return emit(
                Outcome::lines(true, vec![format!("预演：会导出到 {}", target.display())]),
                cli,
            );
        }
        return emit(
            workflow::workflow_export(&data, name, target, workflows),
            cli,
        );
    }
    emit(workflow::workflow_show(&data, name, workflows), cli)
}

pub(crate) struct TaskArgs<'a> {
    pub(crate) name: Option<&'a str>,
    pub(crate) list: bool,
    pub(crate) new: bool,
    pub(crate) workflow: &'a str,
    pub(crate) next: bool,
    pub(crate) done: Option<&'a str>,
    pub(crate) note: &'a str,
    pub(crate) journal: Option<&'a str>,
}

pub(crate) fn task(args: TaskArgs<'_>, cli: &Cli) -> i32 {
    let data = workspace::data_dir(cli.data.as_deref()).unwrap_or_else(|e| fail(&e));
    let root = cli.root.as_deref();
    let workflows = cli.workflows.as_deref();
    if args.list {
        return emit(task::task_list(root, &data, workflows), cli);
    }
    if args.new {
        let Some(name) = args.name else {
            return emit(
                Outcome::lines(
                    false,
                    vec!["用法：qtcloud-work task --new <名字> --workflow <工作流>".to_string()],
                ),
                cli,
            );
        };
        if cli.dry_run {
            return emit(
                Outcome::lines(
                    true,
                    vec![format!(
                        "预演：会起任务 {name}（数据仓 {}）",
                        data.display()
                    )],
                ),
                cli,
            );
        }
        let root = workspace::root(cli.root.as_deref()).unwrap_or_else(|e| fail(&e));
        return emit(
            task::task_new(&root, &data, name, args.workflow, workflows),
            cli,
        );
    }
    let Some(name) = args.name else {
        return emit(
            Outcome::lines(false, vec!["用法：qtcloud-work task <名字>，或 task --list / --new <名字> --workflow <工作流>".to_string()]),
            cli,
        );
    };
    if let Some(words) = args.journal {
        if cli.dry_run {
            return emit(
                Outcome::lines(true, vec![format!("预演：会给 {name} 记日志")]),
                cli,
            );
        }
        return emit(task::task_journal(root, &data, name, words, workflows), cli);
    }
    if args.next {
        if cli.dry_run {
            return emit(
                Outcome::lines(true, vec![format!("预演：会走 {name} 的下一步")]),
                cli,
            );
        }
        return emit(
            task::task_step(root, &data, name, "", args.note, true, workflows),
            cli,
        );
    }
    if let Some(step) = args.done {
        if cli.dry_run {
            return emit(
                Outcome::lines(true, vec![format!("预演：会记 {name} 的 {step}")]),
                cli,
            );
        }
        return emit(
            task::task_step(root, &data, name, step, args.note, false, workflows),
            cli,
        );
    }
    emit(task::task_status(root, &data, name, workflows), cli)
}
