# qtcloud-work Studio

量潮知识工作云工作台（Flutter Web）——知识工作云的图形界面入口。

## 运行

```bash
flutter pub get
flutter run -d chrome
```

## 构建与发布

```bash
flutter build web --release --dart-define=APP_VERSION=0.1.0
```

推送 `studio/vX.Y.Z` 标签触发 `.github/workflows/release-studio.yml`：构建 Web 版并发布到 OSS（`qtcloud-work-studio`）与 CDN（`work.cloud.quanttide.com`）。

## 平台目录

本仓库当前只保留 Web 目标所需文件；需要 Android、iOS、桌面端时执行：

```bash
flutter create .
```

## 许可

[CC BY 4.0](../../LICENSE)
