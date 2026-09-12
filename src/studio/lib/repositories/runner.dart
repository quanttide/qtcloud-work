import 'local/dispatch.dart';
import 'local/outcome.dart';
import 'package:quanttide_work/quanttide_work.dart' as qt;

/// 一次调用的结果：命令行那边是一个子进程的输出，这里是同一份信封。
class RunOutput {
  const RunOutput({required this.exitCode, this.stdout = '', this.stderr = ''});

  final int exitCode;
  final String stdout;
  final String stderr;
}

/// 怎么把命令跑起来。抽出来是为了测试里能换成假的（不碰真实文件）。
abstract class Runner {
  Future<RunOutput> run(String executable, List<String> arguments);
}

/// 默认的跑法：不走子进程，直接在 studio 自己这套实现上跑（`lib/repositories/local`）。
class CoreRunner implements Runner {
  CoreRunner(this.workspace);

  final qt.RunContext workspace;

  @override
  Future<RunOutput> run(String executable, List<String> arguments) async {
    final args = [...arguments];
    String? root;
    String? data;
    String? workflows;
    String? server;
    var asJson = false;
    final rest = <String>[];
    for (var i = 0; i < args.length; i++) {
      switch (args[i]) {
        case '--root':
          root = args[++i];
        case '--data':
          data = args[++i];
        case '--workflows':
          workflows = args[++i];
        case '--server':
          server = args[++i];
        case '--json':
          asJson = true;
        default:
          rest.add(args[i]);
      }
    }
    final outcome = dispatch(
      rest,
      root: root ?? workspace.root,
      data: data ?? workspace.data,
      workflows: workflows ?? workspace.workflows,
      server: server,
    );
    return RunOutput(
      exitCode: outcome.ok ? 0 : 1,
      stdout: encodeOutcome(outcome, asJson: asJson),
    );
  }
}
