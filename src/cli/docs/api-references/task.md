# task

六个动作加任务的字段。

## task --list

```bash
qtcloud-work task --list
```

列出数据仓里的任务、各自跑哪条工作流、下一步是什么。

## task --new

```bash
qtcloud-work task --new <名字> --workflow <工作流>
```

起一件任务：工作流不在就挡；任务名撞上已有的就挡，不覆盖。写下 start、运行上下文（`root` / `data` / `workflows`）与空流水，备好报告与日志。同一份工作流可以起多件任务。

## task <名字>

```bash
qtcloud-work task <名字>
```

看步骤状态、开工、工作流、步骤数、指令与产物路径、最近五条流水、下一步。运行上下文用任务里记的，`--root` / `--workflows` 可省。

## task <名字> --next

```bash
qtcloud-work task <名字> --next
```

走下一步。执行者是 agent 就把这一步交给 `pi` 跑，跑完核判据；执行者是 human 就提示轮到你。随后跑 rule 判据、交 agent 审、列 human 闸门、记一笔流水、写报告。所有步骤都走过则报「所有步骤都走过了」。

## task <名字> --done <步骤> [--note 一句话]

```bash
qtcloud-work task <名字> --done <步骤> [--note 一句话]
```

人为地记一步，不交给 AI。照常跑 rule 判据、列 human 闸门；`--note` 是这一步做了什么，省了按判据拼一句。

## task <名字> --journal <一段话>

```bash
qtcloud-work task <名字> --journal <一段话>
```

日志收叙事。先去掉模板里的占位行，再追加这一段。日志要人来写，空话挡回去。

任务是工作流的一次执行，存的是**运行数据**——程序自己的账，不是产物。

## 任务 schema

任务是一份 YAML，落 `<数据仓>/tasks/<任务>.yaml`，一个任务一个文件，文件名即任务名。任务是**运行数据**，不是产物。

| 字段 | 说明 |
| :-- | :-- |
| `name` | 任务名 |
| `start` | 开工时间，`%Y-%m-%d %H:%M` |
| `workflow` | 跑哪条工作流，任务与工作流之间唯一的链接 |
| `root` | 工作区根，绝对路径 |
| `data` / `workflows` | 数据仓与工作流目录，能相对工作区根就相对 |
| `log` | 流水，只增不改 |
| `gates` | 闸门项：等人拍板的事项，走一步累着写 |
| `products` | 这次执行往哪写产物（`report:` / `journal:` 各一条路径，可省） |

流水一条四项：`at` 时刻、`step` 步骤名、`detail` 一句话、`ok` 是否成功。状态从流水推出来：流水里 `ok` 为真且步骤名在定义里的，算走过的步骤；第一个没走过的就是下一步。
