# search：按名找文档

领域服务：没有自己的定义，用 `catalog` 建的名字索引做一件事——按名找文档。

## 落点

`search/mod.rs`——`matches` 与动作 `search`。

## 匹配

先精确（名字与查询相等），不中再模糊兜底（互相包含）。`--show` 连正文或目录内容一起看。

依赖方向单向：`search → catalog`，`catalog` 不得依赖 `search`（有测试钉住）。
