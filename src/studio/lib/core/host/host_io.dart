import 'dart:io';

/// 跑一条 shell 命令，工作目录给定；返回（退出码，标准输出，标准错误）。
({int code, String out, String err}) runShell(String command, String cwd) {
  final result = Process.runSync('sh', ['-c', command], workingDirectory: cwd);
  return (
    code: result.exitCode,
    out: '${result.stdout}',
    err: '${result.stderr}',
  );
}

/// 把一段话交给 pi 跑（非交互），工作目录是工作区根。
({bool ran, String out}) runPi(String prompt, String cwd) {
  try {
    final result = Process.runSync('pi', [
      '-p',
      '--no-session',
      prompt,
    ], workingDirectory: cwd);
    final out = '${result.stdout}'.trim();
    final err = '${result.stderr}'.trim();
    return (ran: result.exitCode == 0, out: out.isEmpty ? err : out);
  } catch (error) {
    return (ran: false, out: '没找到 pi：$error');
  }
}

/// 探活一个地址，返回（拿到没有，正文）。
({bool ok, String body}) httpGet(String url) {
  try {
    final client = HttpClient();
    final request = client.getUrl(Uri.parse(url));
    final response = request.then((r) => r.close()).then((r) {
      return r.transform(const SystemEncoding().decoder).join();
    });
    // 同步等一段：命令行那边是阻塞调用，这里也当阻塞用
    var done = false;
    var body = '';
    var ok = false;
    response
        .then((text) {
          done = true;
          ok = true;
          body = text;
        })
        .catchError((Object error) {
          done = true;
          body = '$error';
        });
    final deadline = DateTime.now().add(const Duration(seconds: 10));
    while (!done && DateTime.now().isBefore(deadline)) {
      sleep(const Duration(milliseconds: 50));
    }
    return (ok: ok, body: body);
  } catch (error) {
    return (ok: false, body: '$error');
  }
}

/// 读一个环境变量。
String? envOf(String name) => Platform.environment[name];
