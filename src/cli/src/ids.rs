//! 凭证：UUID——跨边界的身份。
//!
//! 规格：工作流与工作步骤各带一枚全球凭证，永不重发；工单与工作记录同理
//! （出处：`docs/specification/process/workflow.md`、`work-order.md`、`work-record.md`）。
//!
//! 本仓的两条线：
//!
//! - **人写的定义不带凭证**（工作流、工作步骤）：落盘只有名字与内容，凭证按
//!   「工作区 id + 名字」现算（[`derive`]）——单个工作区里名字本就唯一，
//!   (工作区 id, 名字) 就够定一件东西，再把算得的值抄进文件等于同一件事记两处。
//!   定义读得到处都能跑：指着一处固定资产目录就能跑，程序不必先导入、
//!   也不往人写的文件里添字。
//! - **程序写的账本带凭证**（工单、工作记录）：它们是事实的实例，不是有名字的定义；
//!   凭证由账本生成、落盘随事件走，认它做幂等与跨区归并。
//!
//! [`derive`] 的算法各平台一致：命名空间恒为 [`NAMESPACE`]，串取
//! 「上级凭证/类别/名字」，即 `工作区 id/workflow/名字` 与 `工作流凭证/step/名字`。

/// uuid5 的命名空间，钉死——各平台算得一样。
pub const NAMESPACE: uuid::Uuid = uuid::uuid!("90cf5627-95f3-5e25-ba48-47d50dee6e09");

/// 程序自己发的凭证：工单与工作记录用。
pub fn new_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// 按名派生凭证：同一组字段在同一工作区里永远算出同一枚。
pub fn derive(kind: &str, parts: &[&str]) -> String {
    let mut items = parts.to_vec();
    items.push(kind);
    uuid::Uuid::new_v5(&NAMESPACE, items.join("/").as_bytes()).to_string()
}

/// 像不像一枚 UUID。
pub fn is_id(value: &str) -> bool {
    uuid::Uuid::parse_str(value.trim()).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_is_stable_and_kind_aware() {
        let first = derive("workflow", &["ws-1", "课程档案比对"]);
        let again = derive("workflow", &["ws-1", "课程档案比对"]);
        assert_eq!(first, again);
        assert_ne!(first, derive("workflow", &["ws-2", "课程档案比对"]));
        assert_ne!(first, derive("step", &["ws-1", "课程档案比对"]));
        assert!(is_id(&first));
        assert!(!is_id("不是凭证"));
    }
}
