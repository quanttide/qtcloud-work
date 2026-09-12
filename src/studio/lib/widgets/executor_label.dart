import 'package:flutter/material.dart';

/// 执行者在界面上的说法。
///
/// 工具箱里的执行者是 `agent` / `rule` / `human` 三个取值——那是领域的事；
/// 中文怎么说、拿什么颜色画，是界面的事，就放在这里。

/// 步骤上怎么说（步骤只有 AI 与人两种）。
String stepLabelOf(String code) => code == 'human' ? '人' : 'AI 执行';

/// 判据上怎么说。
String criterionLabelOf(String code) => switch (code) {
  'human' => '人',
  'rule' => '机器',
  _ => 'AI',
};

/// 步骤链左边那条按谁执行上色。
Color colorOf(String code, ColorScheme scheme) => switch (code) {
  'human' => scheme.primary,
  'rule' => scheme.secondary,
  _ => scheme.tertiary,
};
