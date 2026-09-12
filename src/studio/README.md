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

## 目录

```
lib/
├── main.dart           装配：读环境、起工作台
├── domain/             领域模型：三处位置、执行者、任务与定义的读模型
├── application/        用例：工作流、任务、走一步、命令面与导览
├── infrastructure/     适配器：信封、命令行客户端与运行器、判据执行、平台读写（fs / host / env / YAML）
└── presentation/       界面：app、screens/、views/
test/
├── domain/             领域模型（工作流、任务、三处位置）
├── application/        用例（导览）
├── infrastructure/     信封、命令行客户端、环境
├── presentation/       界面块与页面、工作台冒烟
├── support/            假 runner、起界面的小工具
└── fixtures/           真实输出，从命令行抓下来的
doc/                   原型与设计说明（screens / views / models 三轴）
```

六个平台（android、ios、linux、macos、windows、web）的目录都已初始化；
以后加平台或补平台文件，执行 `flutter create .` 即可。

界面跑的是 studio 自己那份实现（`lib/application/`，与命令行同一个结果）；命令行那份仍留着，两边各自都能把活干完。
三处位置从环境读（`QTCLOUD_WORK_ROOT`／`QTCLOUD_WORK_DATA`／`QTCLOUD_WORK_WORKFLOWS`）。

## 校验

```bash
flutter analyze
flutter test
```

CI 跑的就是这两条，都在 `.github/workflows/release-studio.yml`：
**改了 studio 只跑到门禁为止**（不部署），**推 `studio/*` tag 才是门禁过了再构建部署**。

## 许可

[CC BY 4.0](../../LICENSE)
