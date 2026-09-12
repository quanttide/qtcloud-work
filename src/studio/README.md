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
├── main.dart          工作台（现在只有任务页：左边任务，右边这次走成什么样）
├── models/            界面背后的数据：工作流定义、任务记录、三处位置
└── cli/               命令行客户端：起子进程、读统一信封（ok / columns / rows / lines）
test/
├── models/            模型与领域实现（工作流、任务、三处位置、信封）
├── cli/               命令怎么拼、信封怎么读、导览（用假 runner，不起子进程）
├── views/ screens/    界面块与三个页面
├── fixtures/          真实输出，从命令行抓下来的
└── widget_test.dart   工作台冒烟
doc/                   原型与设计说明（screens / views / models 三轴）
```

六个平台（android、ios、linux、macos、windows、web）的目录都已初始化；
以后加平台或补平台文件，执行 `flutter create .` 即可。

界面跑的是 studio 自己那份实现（`lib/core/`，与命令行同一个结果）；命令行那份仍留着，两边各自都能把活干完。
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
