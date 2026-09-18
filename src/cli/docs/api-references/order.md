# order

七个动作加工单的字段。工单是工作流的一次行程：封面写着走哪条工作流，内页是流水。

## order create

```bash
qtcloud-work order create <名字> --workflow <工作流> [--description 一句话]
```

开一件工单：凭证（`id`）、所引工作流的凭证（`workflow_id`）与开工时刻由账本查填，封面落笔即封——工作流不在就挡；工单名撞上已有的就拒，不覆盖。同一份工作流可以起多件工单。

## order show

```bash
qtcloud-work order show <名字>
```

看这件行程：封面（`id` / 工作流 / 开工 / 描述）、各步骤走过没有、进度条、待拍板的闸门、流水（页码、时刻、过没过、一句话）、产物与日志的落点、下一步。进度条是十格加「走过/总数」的写法（如 `[███░░░░░░░] 1/3`）。

## order list

```bash
qtcloud-work order list [--workflow <工作流>]
```

列出账上的工单、各自进度条与下一步；`--workflow` 只列引着那条工作流的。

## order next

```bash
qtcloud-work order next <名字> [--note 一句话]
```

走下一步。只走机器路径：执行者是 `agent` 就交给 `pi` 跑，回来核 rule 判据、交 `agent` 审；执行者是 `human` 的步骤程序不抢着做，提示轮到你，等你 `order done`。带 `human` 判据的闸门站，程序核完自己的半程就等人放行——这一笔不记。AI 没跑成不记账（流水里是事实，不是失败记录），修好重走。`--note` 是这一步做了什么，省了按判据拼一句。所有步骤都走过则报「所有步骤都走过了」。

## order done

```bash
qtcloud-work order done <名字> <步骤> [--note 一句话]
```

人记一笔：闸门放行，或人自己做完记一笔。程序仍核 rule 判据——过了记过，不过记没过（`is_succeeded: false`，原样在账上），退出码非 0。`--note` 的原话进流水。步骤不在所引工作流里就挡。

## order journal

```bash
qtcloud-work order journal <名字> <一段话>
```

日志收叙事，落产物落点的日志文件，不记流水。日志要人来写，空话挡回去。

## order delete

```bash
qtcloud-work order delete <名字>
```

删一张白纸：流水非空即拒——有账不销，账本不销户。

工单是**账本数据**——程序自己的账，落账本仓，不是产物。

## 工单 schema

工单是一份 YAML，落 `<账本>/workorders/<工单>.yaml`，一件工单一个文件，文件名即工单名。

| 字段 | 说明 |
| :-- | :-- |
| `id` | 凭证号，程序生成，落笔后不变 |
| `name` | 工单名，工作区内唯一 |
| `description` | 一句话说这趟干什么，给执行的人（含智能体）看 |
| `workflow_id` | 所引工作流的凭证，账本方查填，落笔即封 |
| `created_at` | 开单时刻，账本方生成 |
| `records` | 流水，只增不改 |

位置不进模型：工单文件里没有 `root` / `data` / `workflows`，三处位置由启动参数装载。进度与完结不落字段，由流水对照定义推导：每站有一条 `is_succeeded` 为真的记录即走过；带 `human` 判据的闸门站，通过须出自人（`order done`）。

流水一笔八项：

| 字段 | 说明 |
| :-- | :-- |
| `id` | 记录凭证，程序生成，重放同 `id` 即拒 |
| `seq` | 页码，自 1 起严格递增不跳号 |
| `created_at` | 记账时刻，倒流即拒 |
| `order_id` | 认账：这笔挂在哪件工单上 |
| `step` | 步骤名 |
| `step_id` | 这一站的凭证，按 `step` 查填 |
| `description` | 一句话：做了什么、判了什么 |
| `is_succeeded` | 过没过 |

旧记录原样在账上，以最新一条为准是推导，不是销毁。
