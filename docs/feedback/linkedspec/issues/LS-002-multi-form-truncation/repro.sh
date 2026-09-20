#!/usr/bin/env bash
# LS-002 reproducer — self-contained.
#
# Everything this needs is in THIS directory: inputs in ./evidence/*.eadl, the frozen
# observation in ./evidence/EXPECTED.txt. Nothing outside it is read.
#
#   usage: repro.sh --bin <lispish_file> --grammar <Lispish.spec>
#
# Build those two once — see ../../SETUP.md — then every issue's repro.sh takes the same
# two arguments.
#
# CONTRACT (exit code is the verdict):
#   0 = the frozen observation still reproduces — the defect is present
#   3 = behaviour CHANGED — possibly fixed; re-read README.md and compare
#   2 = could not run (bad arguments)
set -uo pipefail
HERE="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
BIN="${LISPISH_BIN:-}"; GRAMMAR="${LISPISH_GRAMMAR:-}"

while [ $# -gt 0 ]; do
  case "$1" in
    --bin) BIN="$2"; shift 2 ;;
    --grammar) GRAMMAR="$2"; shift 2 ;;
    -h|--help) sed -n '2,14p' "$0"; exit 0 ;;
    *) echo "LS-002: unknown argument: $1" >&2; exit 2 ;;
  esac
done
[ -n "$BIN" ] && [ -x "$BIN" ] || { echo "LS-002: --bin <lispish_file> is required; see ../../SETUP.md" >&2; exit 2; }
[ -n "$GRAMMAR" ] && [ -f "$GRAMMAR" ] || { echo "LS-002: --grammar <Lispish.spec> is required" >&2; exit 2; }

err="$(mktemp)"; trap 'rm -f "$err"' EXIT
observe() {
  for input in "$HERE"/evidence/*.eadl; do
    out="$("$BIN" --grammar "$GRAMMAR" "$input" 2>"$err")"; rc=$?
    # Strip the absolute input path out of diagnostics so the frozen file stays portable.
    [ -n "$out" ] || out="<no value: $(sed "s|$input|$(basename "$input")|g" "$err" | tr '\n' ' ' | cut -c1-72 | sed 's/[[:space:]]*$//')>"
    printf '%-34s | %-4s | %s\n' "$(basename "$input")" "$rc" "$out"
  done
}

expected="$HERE/evidence/EXPECTED.txt"
if [ ! -f "$expected" ]; then observe; exit 0; fi

actual="$(observe)"
frozen="$(grep -v '^#' "$expected")"
echo "$actual"
echo
if [ "$actual" = "$frozen" ]; then
  echo "LS-002: RESULT — observation matches evidence/EXPECTED.txt; the issue reproduces."
  exit 0
fi
echo "LS-002: RESULT — observation DIFFERS from evidence/EXPECTED.txt."
echo "Behaviour changed; this may mean the issue is fixed. Difference (expected < / actual >):"
diff <(printf '%s\n' "$frozen") <(printf '%s\n' "$actual") | sed 's/^/  /'
exit 3
