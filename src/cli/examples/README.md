# 示例

这里放工作区的最小原料：照着敲，就能看到程序的设计意图——工作流定义干什么、工单记这一趟怎么走、判据判算不算完。

一个例子一个目录：`README.md` 是走查（一条条命令与看什么），`workflows/` 是定义（固定资产，进 git 能 diff），其余是待处理的输入。**账本与产物不入版控**：账本走缺省落 `$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/`（本机上的账），产物跑出来才有、落 `<根>/artifacts/`。走查里一律显式给 `--root` 与 `--workflows workflows`，账本与产物走缺省——位置由启动参数装载，不进模型。

## 清单

| 例子 | 一句话 | 演示的意图 | 走查 |
| :-- | :-- | :-- | :-- |
| `one-step` | 一步走完 | 定义、工单、判据怎么咬合；程序只记账，产物由人写 | `workflow check` · `order create` · `order show` · `order done` · `order show` |
| `three-criteria` | 三类判据各判各的 | 谁判就写谁；一步算过＝rule 全过＋agent 全过，`human` 只列闸门 | `order next` · `order show` · `workflow show` |
| `append-only` | 同一站重走一笔 | 流水是账、账不改：判没过照样记，重走再记一笔 | `order done` 两次（先 ✗ 后 ✓）· `order journal` · 手改账本看三种拒账 · `order delete` |
| `places` | 换地方跑同一件 | 位置不进模型：三处位置全由启动参数装载；写入型动作先预演 | `--dry-run` · `--data` 与 `--artifacts` 各换一处 · `workflow export` · `import --as` |
| `derived-credentials` | 不写凭证也算得出 | 凭证按名派生（工作区 id ＋ 名字），定义里不写会变的事实 | `workflow show --json` 看 `data.workflow_id` · 把根挪到别处再算，换一枚 |
| `look-around` | 看一圈工作区 | 资产表与四个只读动作；材料的四字段与阶段 | `search` · `catalog` · `audit --make` · `material --json` |

覆盖：`workflow` 六动作用到五个（`list` 在 `one-step` 里带一句），`order` 七动作用到六个，只读四动作全用到。`help` 与 `health` 不立例子——导览敲 `qtcloud-work help`，探活要服务端。

## 与既有件的关系

[使用指南·用例](../../docs/user-guide/usecases.md) 那八件是**真事与历史**，给测试出处对账（`scripts/validate-usecases.sh`）；这里是**原料与走查**，供照着敲，不承担对账。例子不另立测试、不配脚本：行为由 `tests/` 钉住，例子与测试打架以测试为准。
