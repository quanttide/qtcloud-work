# catalog

```bash
qtcloud-work catalog [--json] [--out <文件>]
```

按资产类别列出工作区里的全部条目。`--out` 落一份目录快照：

```json
{
  "root": "quanttide-work",
  "count": 79,
  "entries": [{ "category": "档案", "path": "data/profile", "names": ["profile", "档案"] }]
}
```

每条含类别、路径与全部名字。名字收资产的中英名、目录名、README 里的中文名、文档文件名与篇内一级标题。
