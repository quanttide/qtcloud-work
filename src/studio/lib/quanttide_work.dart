/// 量潮知识工作领域模型。
///
/// 这一份是**不变的核心逻辑**：把知识工作的规范（`docs/specification`）里
/// 不因平台而变的那部分，封成一套可执行的正本——定义的语法与不变量、
/// 判据的取值、任务流水的语义与「走过」的判定、落点与占位的展开。
///
/// 分成一个一个领域模型（产物 / 工作流 / 任务 / 工作区 / 结果 / 判据 / 执行者），一个模型一个目录
/// （只有常量或信封的仍单文件）。读定义（`fields`）、错误（`error`）、路径（`paths`）
/// 是横切件，另立中立模块，聚合只向下依赖它们。
///
/// 这些模型原抽在工具箱 `quanttide-work-toolkit`，现并回本仓，由工作室自持；
/// 界面与仓储向它对齐，不各写一份；说法（拼句、退出码、路径怎么显示）
/// 与文案（提示词）留在各自的平台，不进这里。
library;

export 'artifact/artifact.dart';
export 'criterion/criterion.dart';
export 'error.dart';
export 'executor.dart';
export 'outcome.dart';
export 'paths.dart';
export 'task/task.dart';
export 'workflow/workflow.dart';
export 'workspace/workspace.dart';
