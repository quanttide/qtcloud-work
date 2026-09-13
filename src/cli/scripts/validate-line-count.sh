#!/bin/sh
# 行数门禁：src/ 下任一 .rs 超过 250 行即红。
#
# 阈值的出处是 src/CONVENTIONS.md「单文件 ≤250 行」。

set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cli=$(dirname -- "$here")
src="$cli/src"
limit=250

[ -d "$src" ] || { echo "找不到源码目录：$src" >&2; exit 1; }

over=$(find "$src" -name '*.rs' -exec wc -l {} + | awk -v limit="$limit" '$2 != "total" && $1 > limit { print $1 " 行  " $2 }')
if [ -n "$over" ]; then
	echo "有文件超过 $limit 行：" >&2
	printf '%s\n' "$over" >&2
	exit 1
fi

echo "行数门禁过得去：src/ 下没有超过 $limit 行的文件"
