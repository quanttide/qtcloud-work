# qtcloud-work CLI

量潮知识工作云 **命令行客户端** —— 已部署 provider 的辅助入口，主要供 **AI/脚本** 调用。

纯服务端客户端：只通过 HTTP 对接 provider API，**不读写本地文件**。

## 服务端地址

默认指向 **系统级 API 网关** `https://api.quanttide.com/qtcloud-work`。可覆盖：

```bash
# 环境变量（推荐，一次配置）
export QTCLOUD_WORK_API_BASE_URL=https://api.quanttide.com/qtcloud-work
qtcloud-work health

# 或每次显式指定 --server（优先级最高）
qtcloud-work --server http://localhost:8080 health
```

优先级：`--server` > `QTCLOUD_WORK_API_BASE_URL` > 默认网关。

## 子命令

| 命令 | 说明 |
|------|------|
| `health` | 探活（`GET /health`），`--json` 时透传服务端响应 |

## 构建

```bash
cargo build --release
# 产物：target/release/qtcloud-work
```

## 许可

Apache-2.0
