# 工作流：串联的步骤

工作流是定义，落在 `<工作流目录>/<名字>.yaml`（缺省跟在账本里，固定资产用 `--workflows` 另指一处），文件名即工作流名。步骤按出现顺序衔接，每步写着谁做（`executor`）与怎么算完（`criteria`）。

- `qtcloud-work workflow list` 列出工作流、各自的步骤与位置；
- `qtcloud-work workflow create <名字> --steps 甲,乙,丙 [--note 一句话]` 写一条工作流，每步给一份判据骨架（一条 rule 加一条 human）；
- `qtcloud-work workflow show <名字>` 看步骤、谁执行、几条 rule / agent / human；
- `qtcloud-work workflow check <名字>` 核对声明与判据对不对得上；
- `qtcloud-work workflow export <名字> <文件>` 把定义原样存成一份可带走的文件；
- `qtcloud-work workflow import <文件> [--as 名字]` 导进来，先按 schema 验一遍，重名挡回去，用 `--as` 换名。

定义文件里不写凭证：读进来时按「工作区 id + 名字」现算，同名工作流在另一个工作区就是另一枚。定义里也别抄会变的事实——分类目录、落点、名字一律指过去。

工作流是数据不是代码。加一步、减一步、改判据、换执行者，动这份 YAML 就行，程序一行不用改；进 git 能 diff、能回退。字段与取值由 schema 定死，不认识的字段直接报错，不是「像不像」，是合不合语法。
