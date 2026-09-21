//! 工作区与导览那一组的子命令分派。

use super::super::{Cli, emit};
use crate::artifact;
use crate::audit;
use crate::catalog;
use crate::health;
use crate::help;
use crate::material;
use crate::outcome::Outcome;
use crate::search;
use crate::workspace::local;

pub(crate) fn help(topic: Option<&str>, cli: &Cli) -> i32 {
    match topic {
        Some(name) => match help::topic(name) {
            Some(lines) => emit(Outcome::lines(true, lines), cli),
            None => emit(
                Outcome::lines(
                    false,
                    vec![format!(
                        "没有这条命令：{name}（`qtcloud-work help` 看全部）"
                    )],
                ),
                cli,
            ),
        },
        None => emit(help::guide(), cli),
    }
}

pub(crate) fn health(cli: &Cli) {
    health::health(&health::resolve_base(&cli.server), cli.json);
}

pub(crate) fn search(name: &str, show: bool, cli: &Cli) -> i32 {
    let root = local::root(cli.root.as_deref());
    let result =
        search::search(&root, name, show).with_first(format!("工作区：{}", root.display()));
    emit(result, cli)
}

pub(crate) fn catalog(cli: &Cli) -> i32 {
    let root = local::root(cli.root.as_deref());
    emit(
        catalog::catalog(&root).with_first(format!("工作区：{}", root.display())),
        cli,
    )
}

pub(crate) fn audit(make: bool, cli: &Cli) -> i32 {
    let root = local::root(cli.root.as_deref());
    if cli.dry_run {
        let missing = artifact::missing(&root);
        let mut lines = vec![
            format!("工作区：{}", root.display()),
            "预演：不落盘".to_string(),
        ];
        lines.extend(
            missing
                .iter()
                .map(|a| format!("会补建：{}（{}）", a.category, a.name)),
        );
        return emit(Outcome::lines(true, lines), cli);
    }
    emit(
        audit::audit(&root, make).with_first(format!("工作区：{}", root.display())),
        cli,
    )
}

pub(crate) fn material(paths: &[String], cli: &Cli) -> i32 {
    let root = local::root(cli.root.as_deref());
    let paths = if paths.is_empty() { None } else { Some(paths) };
    emit(
        material::material(&root, paths).with_first(format!("工作区：{}", root.display())),
        cli,
    )
}
