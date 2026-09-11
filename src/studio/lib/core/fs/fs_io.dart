import 'dart:io';

/// 桌面与移动端：本地文件系统就在手边。
bool fileExists(String path) => File(path).existsSync();

bool dirExists(String path) => Directory(path).existsSync();

String readText(String path) => File(path).readAsStringSync();

void writeText(String path, String text) {
  final parent = File(path).parent;
  if (!parent.existsSync()) parent.createSync(recursive: true);
  File(path).writeAsStringSync(text);
}

void makeDir(String path) => Directory(path).createSync(recursive: true);

List<String> listDir(String path) {
  final dir = Directory(path);
  if (!dir.existsSync()) return const [];
  return dir.listSync().map((entity) => entity.path).toList()..sort();
}

/// 各类动作的工作目录：进程当前目录。
String currentDir() => Directory.current.path;
