//! 入口：声明模块，把命令行交给 `cli` 模块。

mod artifact;
mod audit;
mod catalog;
mod cli;
mod clock;
mod criterion;
mod error;
mod events;
mod executor;
mod fields;
mod health;
mod help;
mod ids;
mod locate;
mod material;
mod order;
mod outcome;
mod paths;
mod prompts;
mod search;
mod sha1;
mod workflow;
mod workspace;

fn main() {
    std::process::exit(cli::run_from_env());
}
