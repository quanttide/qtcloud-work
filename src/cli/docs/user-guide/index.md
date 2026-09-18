# 知识工作命令行

平台侧的知识工作命令行，把实验室里跑通的那套本地做法扶正。中心是工作流与工单：工作流是串联的步骤（定义），工单是一次行程（封面加流水）。底座是仓库里的 YAML 与 Markdown，不启服务、不连数据库。

命令名是 `qtcloud-work`。构建：

```bash
cargo build --release
# 产物：target/release/qtcloud-work
```

本指南讲怎么用。程序怎么继续长看[开发指南](../dev-guide/index.md)，每条命令与数据的字段看[接口参考](../api-references/index.md)（一个命令一篇）。

动手之前的三条约定：**位置有缺省**（不写 `--root` 先看环境变量 `QTCLOUD_WORK_ROOT`、再往上找第二大脑；账本缺省落 `$XDG_DATA_HOME/qtcloud-work/workspaces/<工作区键>/`，产物缺省落 `<工作区根>/artifacts/`；位置不进模型，全部由启动参数装载）；**写入型动作都可以先预演**（`--dry-run` 只说要写什么、不落盘）；**结果与错误分家**（结果与 `--json` 走标准输出，提示与错误走标准错误，脚本里读得干净）。每个命令都支持 `--json`，`--out <文件>` 另存一份。

这份指南按三篇拆开，用例单独一篇：

- [工作流](workflow.md)——定义怎么写、命令怎么用；
- [工单](order.md)——开单、走一步、闸门放行、账落哪；
- [工作区](workspace.md)——查看工作区的四个只读动作、判据谁判、三处位置各归谁；
- [用例](usecases.md)——八件真事，照着走一遍。

字段与命令的细节看[接口参考](../api-references/index.md)，程序怎么继续长看[开发指南](../dev-guide/index.md)。

## 走一遍

```bash
qtcloud-work workflow create 课程档案比对 --steps 定位,比对,结论 --note "比对两边的档案"
qtcloud-work order create 课程档案比对 --workflow 课程档案比对
qtcloud-work order next 课程档案比对
qtcloud-work order show 课程档案比对
```

`workflow create` 写下一份 YAML；`order create` 开一件工单；`order next` 走下一步——机器路径交给 AI，人做的步骤轮到你；`order show` 看步骤状态、流水与待拍板的闸门。

## 边界

不改工作区，除非你点了明确要写的动作（开单、记日志、补建格子、导出）。不是服务，没有端点、没有数据库、没有后台进程。认名字与位置：名字（文件名加篇内标题）管找到，位置（资产与落点规则）管归属。账写账本仓，产物写工作区。
