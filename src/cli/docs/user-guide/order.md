# 工单：一次行程

工单是工作流的一次行程，落账本仓 `<账本>/workorders/<工单>.yaml`。同一份工作流可以起任意多件工单，各有各的流水与产物，互不串。

- `qtcloud-work order create <名字> --workflow <工作流> [--description 一句话]` 开一件工单，工作流不在就挡，重名即拒；
- `qtcloud-work order show <名字>` 看封面、步骤状态、进度条、待拍板的闸门、流水、下一步；
- `qtcloud-work order list [--workflow <工作流>]` 列出账上的工单与各自进度；
- `qtcloud-work order next <名字> [--note 一句话]` 走下一步；
- `qtcloud-work order done <名字> <步骤> [--note 一句话]` 人做完了一步（或闸门放行），自己记一笔；
- `qtcloud-work order journal <名字> <一段话>` 日志收叙事（人写）；
- `qtcloud-work order delete <名字>` 删一张白纸——流水非空的工单删不得。

账本归 CLI：工单落 `$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/`，不进版控；产物归工作区，落 `<工作区根>/artifacts/`。程序不替人写产物：报告与日志由写它们的人或智能体来写。每一步走完，`order next` / `order done` 末尾都印一条十格进度条（如 `[███░░░░░░░] 1/3`）与下一步。

`order next` 只走机器路径。执行者是 `agent`（默认，能用 AI 跑的都用 AI）就把这一步交给 `pi` 跑，跑完程序自己核判据；执行者是 `human` 的步骤程序不抢着做，提示轮到你，等你 `order done`。带 `human` 判据的闸门站，程序核完自己的半程就等人放行——这一笔先不记。判据不许 AI 写、不许 AI 改——改了就是自评自过。AI 没跑成（`pi` 不在、超时、产物没落成）不记账，修好重走。

流水只增不改：判没过照样记（`is_succeeded: false`），重走再记一笔，两笔都在账上，以最新一条为准。想抹掉重写？账本拒绝——同 `id` 即拒、页码跳号即拒、时刻倒流即拒。
