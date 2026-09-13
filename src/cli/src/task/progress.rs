//! 任务聚合 / 进度：把走过的步数与总步数画成一条进度条。
//!
//! 数从流水推出来（见 `state`）；这一件只管画给人看——命令行画成字，
//! 窗口画成条，各画各的。落在 `lines` 这一栏里，`columns` / `rows` / `data`
//! 那几栏（窗口与脚本读的结构化结果）一个字都不动。

const WIDTH: usize = 10;

/// 十格进度条，附「走过/总数」：走过 `█`、没走 `░`，如 `[███░░░░░░░] 1/3`。
pub fn bar(done: usize, total: usize) -> String {
    let filled = match total {
        0 => 0,
        _ => (done * WIDTH / total).min(WIDTH),
    };
    let cells: String = (0..WIDTH)
        .map(|i| if i < filled { '█' } else { '░' })
        .collect();
    format!("[{cells}] {done}/{total}")
}
