# health

```bash
qtcloud-work [--server <基地址>] health
```

探活已部署的 provider：`GET /health`。基地址优先级 `--server` > 环境变量 `QTCLOUD_WORK_API_BASE_URL` > 默认网关 `https://api.quanttide.com/qtcloud-work`。`--json` 时透传服务端响应。这一支留给接口层；本地知识工作（`search` / `catalog` / `audit` / `material` / `workflow` / `order`）不依赖它。
