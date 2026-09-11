# 三处位置

设置页装的就是这三个路径——它们是命令行唯一的输入，也是界面唯一要记住的东西。

| 位置 | 选项 | 装什么 | 例 |
|---|---|---|---|
| 工作区 | `--root` | 文档与代码本体；判据里的相对路径从这算 | `/home/iguo/repos/quanttide/domains/quanttide-work` |
| 数据仓 | `--data` | 任务、流水、产物草稿 | `data/context/qtcloud-work` |
| 工作流目录 | `--workflows` | 工作流定义（`.yaml`） | `data/profile/quanttide/workflows` |

缺省：数据仓取当前目录下的 `data/`，工作流目录取数据仓下的 `workflows/`。

探活已部署的 provider：`health`。
