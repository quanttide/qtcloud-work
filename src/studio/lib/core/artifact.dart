import 'fs/fs.dart';

/// 资产层：第二大脑应该有什么、叫什么、落在哪。
///
/// 依据量潮第二大脑章程第九条（程序型）与第十三条（陈述型）。
class Asset {
  const Asset(this.kind, this.name);

  /// 中文用名。
  final String kind;

  /// 目录名。
  final String name;
}

/// 陈述型九宫格与不占格资产。
const List<Asset> stated = [
  Asset('报告', 'report'),
  Asset('参考', 'library'),
  Asset('历史', 'history'),
  Asset('日志', 'journal'),
  Asset('档案', 'profile'),
  Asset('宣传册', 'brochure'),
  Asset('路线图', 'roadmap'),
  Asset('洞察', 'insight'),
  Asset('意图', 'intention'),
  Asset('语境', 'context'),
  Asset('归档', 'archive'),
];

/// 程序型九宫格。
const List<Asset> procedural = [
  Asset('章程', 'bylaw'),
  Asset('规格', 'specification'),
  Asset('工具箱', 'toolkit'),
  Asset('手册', 'handbook'),
  Asset('案例', 'gallery'),
  Asset('平台', 'platform'),
  Asset('教程', 'tutorial'),
  Asset('札记', 'essay'),
  Asset('实验室', 'example'),
];

List<Asset> assets() => [...stated, ...procedural];

/// 非同名目录的三格，按命名规则找独立仓库。
({String container, String suffix})? locatePattern(String name) =>
    switch (name) {
      'toolkit' => (container: 'packages', suffix: '-toolkit'),
      'platform' => (container: 'apps', suffix: ''),
      'example' => (container: 'examples', suffix: ''),
      _ => null,
    };

/// 落点：文档类入 `data/` 或 `docs/` 的同名目录，独立仓库按命名规则找。
List<String> locate(String root, Asset asset) {
  for (final candidate in [
    '$root/data/${asset.name}',
    '$root/docs/${asset.name}',
  ]) {
    if (dirExists(candidate)) return [candidate];
  }
  final pattern = locatePattern(asset.name);
  if (pattern != null) {
    final base = '$root/${pattern.container}';
    final found = listDir(base).where(dirExists).where((path) {
      final name = path.split('/').last;
      return pattern.suffix.isEmpty
          ? !name.startsWith('.')
          : name.endsWith(pattern.suffix);
    }).toList();
    return found;
  }
  return const [];
}

/// 资产表有而工作区无的格子。
List<Asset> missing(String root) =>
    assets().where((asset) => locate(root, asset).isEmpty).toList();

/// 补建缺的格子：文档类建 `data/` 或 `docs/` 下的同名目录，各带一份 README。
/// 独立仓库那三格不凭空建——它们是另外的仓库。
List<String> makeAssets(String root, [List<Asset>? wanted]) {
  final list = wanted ?? missing(root);
  final created = <String>[];
  for (final asset in list) {
    if (locatePattern(asset.name) != null) continue;
    final container = stated.any((item) => item.name == asset.name)
        ? 'data'
        : 'docs';
    final path = '$root/$container/${asset.name}';
    makeDir(path);
    final readme = '$path/README.md';
    if (!fileExists(readme)) {
      writeText(readme, '# 量潮知识工作${asset.kind}\n');
    }
    created.add(path);
  }
  return created;
}

/// 工作区根：从起点往上找，直到看见数据层。
String repoRoot([String? from]) {
  var dir = from ?? currentDir();
  while (true) {
    if (dirExists('$dir/data/journal')) return dir;
    final at = dir.lastIndexOf('/');
    if (at <= 0) {
      throw StateError('未找到第二大脑仓库根（应在含 data/journal 的目录下使用）');
    }
    dir = dir.substring(0, at);
  }
}
