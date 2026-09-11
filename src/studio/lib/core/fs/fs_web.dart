/// 网页版没有本地文件系统。
///
/// 工作流定义、任务文件、产物都在本地磁盘上，浏览器够不着——
/// 网页版要走服务端（`--server` 那一路），这里如实报错，不假装成功。
Never _unsupported() =>
    throw UnsupportedError('网页版没有本地文件系统：工作流与任务都在本地磁盘上，请用桌面客户端，或接服务端。');

bool fileExists(String path) => false;

bool dirExists(String path) => false;

String readText(String path) => _unsupported();

void writeText(String path, String text) => _unsupported();

void makeDir(String path) => _unsupported();

List<String> listDir(String path) => const [];

String currentDir() => _unsupported();
