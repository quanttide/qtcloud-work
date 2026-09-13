/// 路径怎么显示给人看：相对工作区根写短一点，不在根底下就原样。
///
/// 这是各自的平台的事（结果本身在领域模型里，路径怎么显示不在）。
String short(String root, String path) {
  final prefix = root.endsWith('/') ? root : '$root/';
  return path.startsWith(prefix) ? path.substring(prefix.length) : path;
}
