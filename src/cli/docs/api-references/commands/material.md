# material

```bash
qtcloud-work material [<路径>…] [--json] [--out <文件>]
```

列出材料的四字段与阶段。不给路径就扫 `data/journal` 与 `data/profile` 下的 md。字段与阶段见[工作区·材料](../concepts/workspace.md)；`--out` 落 `{count, materials}`，每条是 `path` 加四字段与 `stage`。
