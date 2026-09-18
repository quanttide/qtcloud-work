//! 时刻：账本按发生顺序记，秒级够用。
//!
//! 单独一处，全库一个口径。不引 chrono：用系统 `date` 取本地时间，
//! 格式与实验室 v2 一致（`%Y-%m-%dT%H:%M:%S`），字符串比较即时间比较。

pub fn now() -> String {
    let out = std::process::Command::new("date")
        .arg("+%Y-%m-%dT%H:%M:%S")
        .output();
    match out {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        Err(_) => String::new(),
    }
}
