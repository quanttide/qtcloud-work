# material：材料

材料是还没做成成品的输入，四个字段都能自动填。

## 落点

`material/mod.rs`——`Material { type, content, source, created_at, stage }` 与动作 `material`。

## 四字段

| 字段 | 取值 |
| :-- | :-- |
| `type` | 扩展名 |
| `content` | 正文首段截 40 字（正文取 md / txt / rst） |
| `source` | 上级目录加文件名 |
| `created_at` | 文件所在仓库首次提交的日期，退回文件名里的日期 |

阶段不占字段，由资产位置承担：落在 `journal` 下的是「原始」，其余是「材料」。缺字段时动作 `ok=false`。

不给路径就扫 `data/journal` 与 `data/profile` 下的 md。
