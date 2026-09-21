//! 工单聚合：一次行程的账本。
//!
//! 规格出处：`docs/specification/process/work-order.md` 与 `work-record.md`。
//! 工单是行程的封面——封皮上写着走哪条工作流（`workflow_id` 落笔即封），
//! 内页是流水（`records`，只增不改）。工单落在账本里
//! `<账本>/workorders/<名字>.yaml`，流水内嵌、不另落盘；账本在哪由平台装载
//! （`crate::locate::Locate`），位置不进工单文件。
//!
//! 这一件装账本本身：文件读写、开单、追加、销白纸。子件按事分：
//! 领域模型在 `model`、流水纪律在 `record`、走一步在 `execute`、
//! 交给 AI 的两段话在 `ai`、动作在 `actions`、看与列在 `inspect`、日志叙事在 `journal`。

pub mod actions;
pub mod ai;
pub mod execute;
pub mod inspect;
pub mod journal;
pub(crate) mod model;
pub mod record;

pub use actions::{order_create, order_delete, order_done, order_journal, order_next};
pub use inspect::{order_list, order_show};
pub use model::{WorkOrder, WorkRecord};

use crate::ids;
use crate::locate::Locate;
use crate::workflow::{self, Workflow};
use std::path::PathBuf;

/// 一本打开的账：位置装载 + 工单内容 + 所引工作流（凭证已补）。
pub struct Order {
    pub locate: Locate,
    pub payload: WorkOrder,
    pub workflow: Workflow,
}

impl Order {
    pub fn file(&self) -> PathBuf {
        self.locate.order_file(&self.payload)
    }

    /// 落盘：工单 YAML 写回账本。
    pub fn save(&self) -> Result<(), String> {
        crate::locate::write_yaml(&self.file(), &self.payload.to_yaml())
    }

    /// 走过了哪几步（进度只推导，见 `crate::workspace::progress`）。
    pub fn done_steps(&self) -> Vec<String> {
        crate::workspace::progress::done_steps(&self.payload, &self.workflow)
    }

    pub fn next_step(&self) -> Option<&workflow::Step> {
        crate::workspace::progress::next_step(&self.payload, &self.workflow)
    }

    pub fn state_line(&self) -> String {
        crate::workspace::progress::state_line(&self.payload, &self.workflow)
    }

    /// 追加一笔：id 由追加方给（幂等键），seq 与 step_id 账本方分配查填。
    pub fn append(
        &mut self,
        record_id: &str,
        step: &str,
        description: &str,
        is_succeeded: bool,
    ) -> Result<WorkRecord, String> {
        record::append_check(&self.payload, record_id)?;
        let found = self
            .workflow
            .step(step)
            .ok_or_else(|| format!("所引工作流里没有这一步：{step}（先看看定义）"))?;
        let stamp = crate::clock::now();
        if let Some(last) = self.payload.records.last()
            && !last.created_at.is_empty()
            && stamp < last.created_at
        {
            return Err(format!(
                "时间倒序：新记录（{stamp}）早于末条（{}）",
                last.created_at
            ));
        }
        let record = WorkRecord {
            id: record_id.to_string(),
            seq: self.payload.records.len() + 1,
            created_at: stamp,
            order_id: self.payload.id.clone(),
            step: found.name.clone(),
            step_id: found.id.clone(),
            description: description.trim().to_string(),
            is_succeeded,
        };
        self.payload.records.push(record.clone());
        record::validate(&self.payload, "工单")?;
        self.save()?;
        Ok(record)
    }
}

/// 读一本账：文件在、账成形、所引工作流还在（认 `workflow_id`）。
pub fn open(locate: &Locate, name: &str) -> Result<Order, String> {
    let path = locate.workorders_dir().join(format!("{name}.yaml"));
    let text =
        std::fs::read_to_string(&path).map_err(|_| format!("没有这件工单：{}", path.display()))?;
    let payload: serde_yaml::Value = serde_yaml::from_str(&text)
        .map_err(|e| format!("{} 不是合法的 YAML：{e}", path.display()))?;
    let order = record::validate_yaml(&payload, &path.display().to_string())?;
    let workflow = workflow_by_id(locate, &order.workflow_id)
        .ok_or_else(|| format!("这单引的工作流不见了（workflow_id={}）", order.workflow_id))?;
    Ok(Order {
        locate: locate.clone(),
        payload: order,
        workflow,
    })
}

/// 按凭证找工作流：区内定义按名现算凭证，逐一对照。
pub fn workflow_by_id(locate: &Locate, workflow_id: &str) -> Option<Workflow> {
    workflow::listing(locate)
        .into_iter()
        .map(|flow| flow.credentialed())
        .filter_map(Result::ok)
        .find(|flow| flow.id == workflow_id)
}

/// 账上有哪些工单；`workflow` 给了名字就只列引着那条工作流的。
pub fn listing(locate: &Locate, workflow: &str) -> Result<Vec<Order>, String> {
    let base = locate.workorders_dir();
    if !base.is_dir() {
        return Ok(Vec::new());
    }
    let mut names: Vec<String> = std::fs::read_dir(&base)
        .map(|entries| {
            entries
                .flatten()
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .map(|ext| ext == "yaml")
                        .unwrap_or(false)
                })
                .filter_map(|entry| {
                    entry
                        .path()
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                })
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    let mut found = Vec::new();
    for name in names {
        let order = open(locate, &name)?;
        if !workflow.trim().is_empty() && order.workflow.name != workflow.trim() {
            continue;
        }
        found.push(order);
    }
    Ok(found)
}

/// 开工单：名字与所引工作流由请求给，凭证与时刻由账本查填。封面落笔即封。
pub fn create(
    locate: &Locate,
    name: &str,
    workflow_name: &str,
    description: &str,
) -> Result<Order, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("请先给这单起个名字".to_string());
    }
    let path = locate.workorders_dir().join(format!("{name}.yaml"));
    if path.is_file() {
        return Err(format!("已经有一件工单叫「{name}」：{}", path.display()));
    }
    let flow = workflow::open(locate, workflow_name.trim());
    if !flow.exists() {
        return Err(format!("没有这条工作流：{}", flow.file().display()));
    }
    let credentialed = flow.credentialed()?;
    let order = WorkOrder {
        id: ids::new_id(),
        name: name.to_string(),
        description: description.trim().to_string(),
        workflow_id: credentialed.id.clone(),
        created_at: crate::clock::now(),
        records: Vec::new(),
    };
    record::validate(&order, "工单")?;
    let opened = Order {
        locate: locate.clone(),
        payload: order,
        workflow: credentialed,
    };
    opened.save()?;
    Ok(opened)
}

/// 删一张白纸：流水非空即拒——账本不销户。
pub fn delete(locate: &Locate, name: &str) -> Result<PathBuf, String> {
    let order = open(locate, name)?;
    if !order.payload.records.is_empty() {
        return Err(format!(
            "有账不销：这单已经有 {} 笔流水，删不得",
            order.payload.records.len()
        ));
    }
    let path = order.file();
    std::fs::remove_file(&path).map_err(|e| format!("{} 删不了：{e}", path.display()))?;
    Ok(path)
}
