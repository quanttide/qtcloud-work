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
├── app.dart            工作台外壳：顶栏 + 侧栏 + 当前那一屏
├── quanttide_work.dart 领域模型出口：产物 / 判据 / 工作流 / 任务 / 工作区 / 结果
├── artifact/           领域模型：产物实例（名字 + 规格）
├── criterion/          领域模型：判据
├── workflow/           领域模型：工作流
├── task/               领域模型：任务与流水
├── workspace/          领域模型：工作区（落点 / 流水 / 核对）
├── outcome.dart 等     领域模型横切件：结果 / 错误 / 执行者 / 占位 / 字段表
├── repositories/       数据边界：交出来的就是本仓的领域对象
│   ├── studio_repository.dart   接口（界面与 Bloc 只依赖它）
│   └── local/           实现：命令面（dispatch / 导览）与平台读写（fs / host / env / YAML、判据执行）
│                        路径怎么显示在 `local/paths.dart`
├── states/             状态：workbench_bloc.dart（状态 + 事件 + Bloc 一个文件）
├── screens/            三个页面：任务、流程、设置
└── views/              页面里可复用的块（`executor_label.dart` 是执行者怎么说）
test/
├── repositories/ states/
├── screens/ views/     页面与块、工作台冒烟
├── support/            假仓储、样例数据、起界面的小工具
└── fixtures/           真实输出，从命令行抓下来的
doc/                   原型与设计说明（screens / views / models 三轴）
```

六个平台（android、ios、linux、macos、windows、web）的目录都已初始化；
以后加平台或补平台文件，执行 `flutter create .` 即可。

界面只有一套实现（`lib/repositories/local/`）：不起子进程、不依赖命令行，算出来的结果与命令行一致。
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
