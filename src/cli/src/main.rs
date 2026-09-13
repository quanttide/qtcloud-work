//! 入口：声明模块，把命令行交给 `cli` 模块。

mod artifact;
mod audit;
mod catalog;
mod cli;
mod criterion;
mod error;
mod executor;
mod fields;
mod health;
mod help;
mod material;
mod outcome;
mod paths;
mod prompts;
mod search;
mod task;
mod workflow;
mod workspace;

fn main() {
    std::process::exit(cli::run_from_env());
}
