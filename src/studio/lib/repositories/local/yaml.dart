import 'package:yaml/yaml.dart';

/// 读一份 YAML，转成普通的 Dart 结构（Map / List / String / num / bool / null）。
Object? parseYaml(String text) => _plain(loadYaml(text));

Object? _plain(Object? value) {
  if (value is YamlMap || value is Map) {
    final map = <String, Object?>{};
    (value as Map).forEach((key, item) {
      map['$key'] = _plain(item);
    });
    return map;
  }
  if (value is YamlList || value is List) {
    return (value as List).map(_plain).toList();
  }
  if (value is YamlScalar) return value.value;
  return value;
}

/// 写一份 YAML。只用块状写法，够我们的定义与任务文件用。
String dumpYaml(Object? value) {
  final buffer = StringBuffer();
  _dump(value, 0, buffer);
  return buffer.toString();
}

void _dump(Object? value, int indent, StringBuffer buffer) {
  final pad = '  ' * indent;
  if (value is Map) {
    for (final entry in value.entries) {
      final key = '${entry.key}';
      final item = entry.value;
      if (_isScalar(item)) {
        buffer.writeln('$pad$key: ${_scalar(item)}');
      } else if (item is List && item.isEmpty) {
        buffer.writeln('$pad$key: []');
      } else if (item is Map && item.isEmpty) {
        buffer.writeln('$pad$key: {}');
      } else {
        buffer.writeln('$pad$key:');
        _dump(item, indent + 1, buffer);
      }
    }
    return;
  }
  if (value is List) {
    for (final item in value) {
      if (_isScalar(item)) {
        buffer.writeln('$pad- ${_scalar(item)}');
        continue;
      }
      // 列表里的映射：第一行跟在 `- ` 后面，其余行跟着同一层缩进
      final nested = StringBuffer();
      _dump(item, indent + 1, nested);
      final lines = nested
          .toString()
          .split('\n')
          .where((l) => l.isNotEmpty)
          .toList();
      for (var i = 0; i < lines.length; i++) {
        buffer.writeln(i == 0 ? '$pad- ${lines[i].trimLeft()}' : lines[i]);
      }
    }
    return;
  }
  buffer.writeln('$pad${_scalar(value)}');
}

bool _isScalar(Object? value) =>
    value == null || value is String || value is num || value is bool;

const List<String> _specials = [
  '-',
  '?',
  ':',
  ',',
  '[',
  ']',
  '{',
  '}',
  '#',
  '&',
  '*',
  '!',
  '|',
  '>',
  '%',
  '@',
  '`',
  '"',
  "'",
];

/// 这个值写成裸的会不会被读成别的东西。
bool _unsafe(String text) {
  if (text != text.trim()) return true;
  if (text.endsWith(':')) return true;
  if (text.contains(': ')) return true;
  if (text.contains(' #')) return true;
  if (text.contains('\n')) return true;
  if (_specials.contains(text[0])) return true;
  if (RegExp(r'^(true|false|null|~|-?[0-9.]+)$').hasMatch(text)) return true;
  return false;
}

String _scalar(Object? value) {
  if (value == null) return '""';
  if (value is bool || value is num) return '$value';
  final text = '$value';
  if (text.isEmpty) return '""';
  if (!_unsafe(text)) return text;
  final escaped = text
      .replaceAll(r'\', r'\\')
      .replaceAll('"', r'\"')
      .replaceAll('\n', r'\n');
  return '"$escaped"';
}
