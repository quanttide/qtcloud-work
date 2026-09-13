# catalog：目录

目录是工作区的快照：按资产表清点实际有什么，建成名字索引。仓库变了要重扫。

## 落点

`catalog/mod.rs`——名字索引与动作。`Entry { category, path, names }`；`Catalog::unregistered` 找未登记的顶层子目录；动作 `catalog` 输出 `{root, count, entries}`。

## 索引怎么建

- 从资产表的每一格出发，递归收 `.md`；跳过 `.git`、`node_modules`、`.venv`、`build`、`dist`、`.dart_tool`、`__pycache__` 与 `README.md` / `CHANGELOG.md` / `LICENSE`。
- 每个条目的名字集合：资产的中文名与英文名、目录名、仓库的中文名（`README` 里的量潮名）、文件名（去扩展名）、篇内第一个一级标题。命名规则规定英文文件名与中文标题不互译，两边都收。
- 索引同时服务 [search](search.md)（按名找）与 [audit](audit.md)（未登记）。

## 依赖方向

`search → catalog` 单向，`catalog` 不得依赖 `search`。

## 测试

依赖方向由 `tests/contract.rs` 的「动作层不依赖入口层」钉住。
