/// 网页版没有进程环境，改成构建时注入：`flutter build web --dart-define=QTCLOUD_WORK_DATA=...`。
Map<String, String> processEnvironment() => const {
  'QTCLOUD_WORK_ROOT': String.fromEnvironment('QTCLOUD_WORK_ROOT'),
  'QTCLOUD_WORK_DATA': String.fromEnvironment('QTCLOUD_WORK_DATA'),
  'QTCLOUD_WORK_WORKFLOWS': String.fromEnvironment('QTCLOUD_WORK_WORKFLOWS'),
};
