# audit

```bash
qtcloud-work audit [--json] [--out <文件>] [--make]
```

审计两件事：资产表有而工作区无的格子，工作区有而未登记的顶层目录。`--make` 补建缺的文档格（在 `data/<名>` 或 `docs/<名>` 下建目录与 README），独立仓库那三格不凭空建；`--make` 也吃 `--dry-run`。`--out` 落 `{root, result, missing, unregistered}`。
