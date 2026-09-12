/// 网页版起不了子进程，也不直接走 HTTP 探活（跨域）。
({int code, String out, String err}) runShell(String command, String cwd) =>
    (code: 127, out: '', err: '网页版起不了子进程');

({bool ran, String out}) runPi(String prompt, String cwd) =>
    (ran: false, out: '网页版起不了 pi');

Future<({bool ran, String out})> runPiAsync(String prompt, String cwd) async =>
    (ran: false, out: '网页版起不了 pi');

({bool ok, String body}) httpGet(String url) => (ok: false, body: '网页版不做探活');

String? envOf(String name) => null;
