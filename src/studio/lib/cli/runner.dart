import 'dart:io';

/// 一次子进程调用的结果。
class RunOutput {
  const RunOutput({required this.exitCode, this.stdout = '', this.stderr = ''});

  final int exitCode;
  final String stdout;
  final String stderr;
}

/// 怎么把命令跑起来。抽出来是为了测试里能换成假的（不起子进程）。
abstract class Runner {
  Future<RunOutput> run(String executable, List<String> arguments);
}

class ProcessRunner implements Runner {
  const ProcessRunner();

  @override
  Future<RunOutput> run(String executable, List<String> arguments) async {
    final result = await Process.run(executable, arguments);
    return RunOutput(
      exitCode: result.exitCode,
      stdout: '${result.stdout}',
      stderr: '${result.stderr}',
    );
  }
}
