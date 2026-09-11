# qtcloud-work

量潮知识工作云——知识工作云服务。

## 仓库目录

```
qtcloud-work/
├── .github/workflows/    # release-cli：tag 触发的 CLI 发布
└── src/
    └── cli/              # 命令行工具（Rust）：provider API 的 AI/脚本入口
```

## 发布

CLI 以 `cli/vX.Y.Z` 标签触发发布：先校验版本与变更记录，再构建三平台二进制、挂到 GitHub Release、发布到 crates.io。

```bash
git tag cli/v0.1.0 && git push origin cli/v0.1.0
```

## 许可

[CC BY 4.0](LICENSE)
