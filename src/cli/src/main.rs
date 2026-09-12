//! 入口：声明模块，把命令行交给 `cli` 模块。

mod artifact;
mod audit;
mod catalog;
mod cli;
mod help;
mod material;
mod paths;
mod prompts;
mod task;
mod workflow;

fn main() {
    std::process::exit(cli::run_from_env());
}
