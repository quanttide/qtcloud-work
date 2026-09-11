//! 动作结果：命令行与窗口共用的一份算出来的东西。
//!
//! `ok` 定退出码，`lines` 给命令行印，`columns` 与 `rows` 给窗口画；
//! 动作之间不互相打印，都只交出这一层。
//!
//! 这一份领域模型已经抽到工具箱 `quanttide-work`——这里只把工具箱那份原样引过来，
//! 加上一个命令行自己的路径写法。

pub use quanttide_work::envelope::Outcome as Result;

use std::path::Path;

/// 路径相对根写短一点；不在根底下就原样。
pub fn short(root: &Path, path: &Path) -> String {
    quanttide_work::envelope::short(&root.to_string_lossy(), &path.to_string_lossy())
}
