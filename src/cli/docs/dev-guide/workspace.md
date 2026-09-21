# workspace：工作区

工作区是一次工作的边界：一套工作流定义与上下文内的工件绑在一起。领域那一侧只认内容（定义与工单），不认位置；物理位置由平台在装载时给（规范 `docs/specification/place/workspace.md`）。

## 装载

`locate.rs` 的 `Locate` 把启动参数落成三处位置，装载顺序写死：命令行 > 环境变量 `QTCLOUD_WORK_ROOT`（只管根）> 缺省规矩。

- **根（root）**：判据路径的基准、`run` 判据的工作目录、工作区级动作扫描的面。缺省从当前目录往上找含 `data/journal` 的第二大脑，找不到就用当前目录；
- **账本（data）**：工作区身份、工单、事件。缺省 `$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/`，键由根派生（可读名加短码）——账本是「这台机器上的这个工作区」的账；
- **产物（artifacts）**：缺省 `<根>/artifacts/`；
- **工作流目录（workflows）**：缺省跟在账本里，定义常是固定资产，用 `--workflows` 另指。

**位置不进模型**：这些都不写进工单文件，换机器、挪仓库，重新装载即可。

## 身份

工作区身份落账本仓的 `workspace.yaml`：`id` / `name` / `title` / `description` / `created_at` / `updated_at`。写动作前把账本开出来，身份缺则首跑生成；`id` 供凭证派生用——宁可落盘不可空算。只读动作不开账本，不在任何根上建文件。

## 三处位置的分工

**账本归 CLI、产物归工作区。** 账本是这台机器上的账，程序每跑一趟都写——落 XDG，不入版控；产物（报告与日志）是内容，跟着工作区走——落 `<根>/artifacts/`。混在一处，程序每跑一趟就往仓库写账本；分开放，两边各得其所。

## 模型与三件操作

| 操作 | 做什么 |
| :-- | :-- |
| 落点 `place` | 产物落在产物落点的哪一格 |
| 核对 `check` | 判据写下的路径在不在区内、描述点到的小节有没有 `contains` 覆盖（只看写下的位置，不访问文件系统；占位运行时展开，不核） |
| 流水判定 `done_steps` / `next_step` / `finished` | 拿流水对照定义逐站对账（见 [work-order](work-order.md)·推导） |

路径怎么显示给人看：`short` 相对工作区根写短一点，不在根底下就原样。

## 测试

装载与只读纪律在 `tests/run_context.rs`（向上搜索、环境变量、指哪落哪、只读不落盘）；缺省矩阵在 `tests/defaults.rs`——四个可省位置各缺一次的行为。

---

拆法：一步步说清楚
先看现在的问题在哪
现在磁盘上是这样：
src/workspace/
├── mod.rs        ← 混了两拨东西：
│                    领域侧：模型、落点计算、进度推导、定义核对
│                    平台侧：root()、account()、workspace_key()、short()
├── model.rs      ← 领域（Workspace 结构体）
├── place.rs      ← 领域（纯字符串计算）
├── progress.rs   ← 领域（纯推导）
├── check.rs      ← 领域（纯核对，不碰文件系统）
└── locate.rs     ← 平台（std::fs 写盘、serde_yaml、生成 id、ensure()）
workspace 这一个名字底下住了两种模块：只算不碰盘的领域逻辑 和 专门碰盘的装载逻辑。order 和 workflow 需要的是后者（落盘、找目录），但引用时只能写 use crate::workspace::Locate——于是它们「引用了 workspace 聚合」，环就这么来的。
拆法：把碰盘的那半搬出去，单独给它一个门牌
第一步：新建一个模块，比如叫 src/locate/（叫 platform/、place_loader/ 都行，名字无所谓）：
src/locate/
└── mod.rs        ← 从 workspace 搬过来的：
                     Locate 结构体（连 ensure、workspace_id、
                     workorders_dir、order_file、artifact_path 等方法）
                     root()
                     account()
                     workspace_key()
                     repo_root()
第二步：workspace 里删掉这些，剩下的都是纯领域：
src/workspace/
├── mod.rs        ← 只剩 short() 挪去 cli 后的壳，或直接并进 model
├── model.rs
├── place.rs
├── progress.rs
└── check.rs
第三步：改引用方。所有原来写 use crate::workspace::Locate 的地方（主要是 order 和 cli），改成 use crate::locate::Locate。纯文本替换，不改任何逻辑。
为什么环就没了：逐条边看
环的边 1：order → workspace
order 引用 workspace，为的是什么？看代码就能确认——order 执行时要落盘，用的是 Locate（order_file、artifact_path、ensure）。这些现在住在 locate 里了，所以这条边变成：
之前：order ──→ workspace   （跨进聚合，成环）
之后：order ──→ locate      （指向一个独立的装载模块，单向，无环）
环的边 2：workspace → order
这条边来自 progress.rs 的 done(order: &WorkOrder, ...)——workspace 要读工单的流水记录来推导进度。这条边不用断，也不该断：工作区聚合持有工单、推导进度，本来就是设计意图（model.rs 里 Workspace 就装着它们）。
拆完之后它变成这样：
之前：workspace ⇄ order          （互相引用，环）
之后：locate ──→ order            （装载模块认得工单）
      locate ──→ workspace ...? 
      workspace ──→ order         （单向，无环）
关键点：环的本质是「A 用 B、B 又用 A」。拆之前，order 用 workspace 的 Locate，workspace 的 Locate 又反过来给 order 建文件路径——两个方向都对同一批代码。拆之后，「碰盘的能力」归 locate，「领域内容」归 workspace：
• order 要落盘 → 找 locate（不认识 workspace）
• workspace 要算进度 → 找 order（不认识 locate）
• locate 要建目录 → 认 order 和 workspace 都可以（它是中间的编排层，向下依赖不构成环）
三方变成一个 三角形单向图，没有对边，环消失。
画成图对比
拆之前：
        ┌─────────  use Locate  ─────────┐
        │                                ▼
      order  ←────  done(order)  ───  workspace
        ▲                                │
        └──────── use Locate ────────────┘
              （两条反向边 = 两个环）
     workflow ⇄ workspace 同理
拆之后：
                cli
                 │
                 ▼
              locate          ← 新模块（原 workspace 的平台半边）
              ↙    ↘
          order     workspace
              ↘    ↙   │
               ▼▼      ▼
            （workspace ─→ order 仍是单向）
  order → locate，locate → order？不——
  locate 里 order_file(order) 需要 WorkOrder 类型，
  所以真实方向是 locate ─→ order ─→ locate？
等一下，这里有个我该主动指出的细节：Locate::order_file(&self, order: &WorkOrder) 的参数用了 WorkOrder 类型，所以 locate → order 是真实依赖；而 order → locate 也是真实依赖（落盘要用）。这两个之间还是一对互引——只是这次是「应用编排层 ⇄ 聚合」，在分层架构里这是允许且常见的方向（类似 application service 和 domain 互认类型），不再违反「聚合之间不互引」的初衷。如果你连这个也想消掉，还有最后一招：
可选第四步：把 order_file 的签名从
pub fn order_file(&self, order: &WorkOrder) -> PathBuf   // 认聚合类型
改成
pub fn order_file(&self, name: &str) -> PathBuf          // 只认原始值
调用方传 order.name 就行。locate 从此只依赖 ids、fields、clock 这些 infra 小件，连聚合类型都不认识，成为纯粹的平台装载层。artifact_path 里那个 Artifact::named(category) 同理可以改成收字符串。
拆完的最终形态
src/
├── cli/           入口
├── locate/        ← 新：平台装载（root/账本/ensure/路径），只依赖 infra
├── order/         聚合，依赖 locate（落盘）+ infra，不依赖 workspace
├── workflow/      聚合
├── workspace/     聚合，纯领域（model/place/progress/check），单向依赖 order/workflow
└── ...其余不动
• 两个环：消失；
• ensure()、repo_root、account：从聚合归位到装载层；
• workspace 的 15 个 dependents：拆成「用领域核心的」和「用装载的」两拨真实数字；
• 顺带解决：ensure 本来就该是应用服务行为，short 挪去 cli。
工作量估计：主要是搬文件 + 全局替换 crate::workspace::Locate → crate::locate::Locate，逻辑一行不用改。要不要动 order_file 的签名（第四步）看你——不动也已经消掉了聚合间环。
