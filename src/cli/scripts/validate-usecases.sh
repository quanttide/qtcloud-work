#!/bin/sh
# 用例对账：文档里的用例号与测试里的出处，两边集合必须相等。
#
#   文档：docs/user-guide/*.md       标题形如「## 用例 一、起一件任务并走一步」
#   测试：tests/*.rs                   每条测试上面一行「// 用例：一」（common/ 里的夹具不算）
#
# 从本脚本所在目录的上一级（cli 根）找这两份，对不上就报错退出 1。

set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cli=$(dirname -- "$here")
guide="$cli/docs/user-guide"
tests="$cli/tests"

[ -d "$guide" ] || { echo "找不到使用指南目录：$guide" >&2; exit 1; }
[ -d "$tests" ] || { echo "找不到用例测试目录：$tests" >&2; exit 1; }

doc_cases=$(find "$guide" -name '*.md' -exec grep -hoE '^## 用例 [一二三四五六七八九十百零]+' {} + | sed 's/^## 用例 //' | sort -u)
test_cases=$(find "$tests" -name '*.rs' -not -path '*/common/*' -exec grep -hoE '^// 用例：[一二三四五六七八九十百零]+' {} + | sed 's|^// 用例：||' | sort -u)

echo "文档里的用例号：$(printf '%s' "$doc_cases" | tr '\n' ' ')"
echo "测试里的出处号：$(printf '%s' "$test_cases" | tr '\n' ' ')"

if [ -z "$doc_cases" ]; then
	echo "文档里没有用例，对账空转。" >&2
	exit 1
fi

if [ "$doc_cases" = "$test_cases" ]; then
	count=$(printf '%s\n' "$doc_cases" | grep -c .)
	echo "对账过得去：两边名字相等，共 $count 条"
	exit 0
fi

echo "对账不过：两边名字不相等" >&2
if [ -n "$doc_cases" ]; then
	printf '%s\n' "$doc_cases" | while IFS= read -r c; do
		printf '%s\n' "$test_cases" | grep -qxF "$c" || echo "只在文档里：$c" >&2
	done
fi
if [ -n "$test_cases" ]; then
	printf '%s\n' "$test_cases" | while IFS= read -r c; do
		printf '%s\n' "$doc_cases" | grep -qxF "$c" || echo "只在测试里：$c" >&2
	done
fi
exit 1
