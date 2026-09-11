#!/bin/sh
# 对表：同一处工作区、同一条命令，命令行（Rust）与 studio（Dart）各跑一次，比输出信封。
#
# 用法：
#   sh scripts/parity.sh                    # 比默认一批命令
#   sh scripts/parity.sh workflow --list    # 只比这一条
#   sh scripts/parity.sh --report           # 只报告，不因「没搬」而失败
#
# 结论只有三种：一致 / 不一致 / 没搬（studio 侧还没实现这条）。
# 不一致与没搬都算不过——这就是「完整复制」的尺子。

set -u

STUDIO_DIR=$(cd "$(dirname "$0")/.." && pwd)
REPO_ROOT=$(cd "$STUDIO_DIR/../../../.." && pwd)
CLI_DIR="$REPO_ROOT/apps/qtcloud-work/src/cli"
DATA_DIR="${QTCLOUD_WORK_DATA:-$REPO_ROOT/data/context/qtcloud-work}"
WORKFLOWS_DIR="${QTCLOUD_WORK_WORKFLOWS:-$REPO_ROOT/data/profile/quanttide/workflows}"

REPORT_ONLY=0
if [ "${1:-}" = "--report" ]; then
  REPORT_ONLY=1
  shift
fi

if [ "$#" -gt 0 ]; then
  COMMANDS="$*"
else
  COMMANDS="workflow --list"
  for wf in "$WORKFLOWS_DIR"/*.yaml; do
    [ -e "$wf" ] || continue
    name=$(basename "$wf" .yaml)
    COMMANDS="$COMMANDS
workflow $name
workflow $name --check"
  done
  COMMANDS="$COMMANDS
task --list
help"
fi

CLI_BIN="$CLI_DIR/target/debug/qtcloud-work"

run_cli() {
  if [ -x "$CLI_BIN" ]; then
    "$CLI_BIN" --root "$REPO_ROOT" --data "$DATA_DIR" --workflows "$WORKFLOWS_DIR" --json "$@"
  else
    (cd "$CLI_DIR" && cargo run --quiet -- \
      --root "$REPO_ROOT" --data "$DATA_DIR" --workflows "$WORKFLOWS_DIR" --json "$@")
  fi
}

run_studio() {
  (cd "$STUDIO_DIR" && dart run bin/qtcloud.dart \
    --root "$REPO_ROOT" --data "$DATA_DIR" --workflows "$WORKFLOWS_DIR" --json "$@")
}

# 比一次：一致 / 不一致 / 没搬
verdict_of() {
  printf '%s' "$1" > /tmp/parity-cli.json
  printf '%s' "$2" > /tmp/parity-studio.json
  python3 - <<'PY'
import json
try:
    a = json.load(open('/tmp/parity-cli.json'))
except Exception:
    print('命令行没给出 JSON')
    raise SystemExit
try:
    b = json.load(open('/tmp/parity-studio.json'))
except Exception:
    print('没搬')
    raise SystemExit
lines = b.get('lines') or []
if lines and str(lines[0]).startswith('还没搬'):
    print('没搬')
    raise SystemExit
print('一致' if a == b else '不一致')
PY
}

ok_count=0
bad_count=0

while IFS= read -r line; do
  [ -n "$line" ] || continue
  cli_out=$(run_cli $line 2>/dev/null)
  studio_out=$(run_studio $line 2>/dev/null)
  verdict=$(verdict_of "$cli_out" "$studio_out")
  case "$verdict" in
    一致)
      printf '  一致　　%s\n' "$line"
      ok_count=$((ok_count + 1))
      ;;
    没搬)
      printf '  没搬　　%s\n' "$line"
      bad_count=$((bad_count + 1))
      ;;
    *)
      printf '  不一致　%s　（%s）\n' "$line" "$verdict"
      bad_count=$((bad_count + 1))
      ;;
  esac
done <<EOF
$COMMANDS
EOF

echo
echo "一致 $ok_count 条，没对上 $bad_count 条。"
if [ "$bad_count" -eq 0 ]; then
  echo "全部一致：这一批命令，studio 与命令行给的是同一个信封。"
  exit 0
fi
[ "$REPORT_ONLY" -eq 1 ] && exit 0
exit 1
