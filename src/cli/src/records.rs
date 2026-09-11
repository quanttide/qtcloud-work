//! 记录的段位与骨架：报告两节、日志模板。
//!
//! 段位是程序唯一的说法——报告回写与日志生成都从这里取，不在别处再写一遍。

/// 日志模板里的占位行，收叙事时先去掉它。
pub const JOURNAL_PLACEHOLDER: &str = "（这个任务的来龙去脉，你写）";

pub fn report_template(title: &str) -> String {
    let title = if title.is_empty() {
        "<任务的名字>"
    } else {
        title
    };
    format!("# 报告：{title}\n\n## 执行记录\n\n## 闸门项\n")
}

pub fn journal_template(title: &str) -> String {
    let title = if title.is_empty() {
        "<任务的名字>"
    } else {
        title
    };
    format!("# 日志：{title}\n\n{JOURNAL_PLACEHOLDER}\n")
}

/// 把某一节的正文换掉，其它节原样保留；没有这一节就补在后面。
pub fn replace_section(text: &str, title: &str, body: &[String]) -> String {
    let marker = format!("## {title}");
    let block = format!("## {title}\n\n{}\n", body.join("\n"));
    match text.find(&marker) {
        None => format!("{}\n\n{}", text.trim_end(), block),
        Some(at) => {
            let head = &text[..at];
            let tail = &text[at + marker.len()..];
            match tail.find("## ") {
                None => format!("{head}{block}"),
                Some(next) => format!("{head}{block}\n{}", &tail[next..]),
            }
        }
    }
}
