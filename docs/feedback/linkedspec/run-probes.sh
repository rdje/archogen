#!/usr/bin/env bash
# Reproduce the Lispish observations in FEEDBACK.md (findings A and B).
#
# Runs against any LinkedSpec checkout; does NOT need archogen's crates.
#
#   usage: run-probes.sh [--bin PATH] [--grammar PATH] [--probes DIR]
#
# Defaults match archogen's layout. From a bare LinkedSpec checkout, pass:
#   --bin     <target>/debug/lispish_file
#   --grammar <linkedspec>/specs/Lispish.spec
#   --probes  <this dir>/probes
#
# CONTRACT: one row per probe; exit 0 if every probe ran; read-only apart from stderr capture.
set -uo pipefail
HERE="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
ROOT="$(cd "$HERE/../../.." && pwd -P)"

BIN="${LISPISH_BIN:-$ROOT/.app-data/target/debug/lispish_file}"
GRAMMAR="${LISPISH_GRAMMAR:-$ROOT/vendor/linkedspec/specs/Lispish.spec}"
PROBES="${LISPISH_PROBES:-$HERE/probes}"
RUN="${LISPISH_RUNNER:-$ROOT/vendor/linkedspec/tools/project_data_run.sh}"

while [ $# -gt 0 ]; do
  case "$1" in
    --bin) BIN="$2"; shift 2 ;;
    --grammar) GRAMMAR="$2"; shift 2 ;;
    --probes) PROBES="$2"; shift 2 ;;
    --runner) RUN="$2"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done

[ -x "$BIN" ]     || { echo "run-probes: no executable at $BIN (see FEEDBACK.md 'How to reproduce')" >&2; exit 1; }
[ -f "$GRAMMAR" ] || { echo "run-probes: no grammar at $GRAMMAR" >&2; exit 1; }
[ -d "$PROBES" ]  || { echo "run-probes: no probe directory at $PROBES" >&2; exit 1; }

# The project_data_run.sh wrapper is optional; without it, invoke the binary directly.
if [ -x "$RUN" ]; then
  export LINKEDSPEC_PROJECT_DATA_ROOT="${LINKEDSPEC_PROJECT_DATA_ROOT:-$ROOT/.app-data/linkedspec}"
  export LINKEDSPEC_CACHE_ROOT="${LINKEDSPEC_CACHE_ROOT:-$LINKEDSPEC_PROJECT_DATA_ROOT/cache}"
  export LINKEDSPEC_SCRATCH_ROOT="${LINKEDSPEC_SCRATCH_ROOT:-$LINKEDSPEC_PROJECT_DATA_ROOT/scratch}"
  invoke() { bash "$RUN" "$BIN" --grammar "$GRAMMAR" "$1"; }
else
  invoke() { "$BIN" --grammar "$GRAMMAR" "$1"; }
fi

err="$(mktemp)"; trap 'rm -f "$err"' EXIT
printf '%-34s | %-4s | %s\n' "probe" "exit" "stdout (or stderr head)"
printf -- '%s\n' "-----------------------------------|------|------------------------"
for probe in "$PROBES"/*.eadl; do
  out="$(invoke "$probe" 2>"$err")"; rc=$?
  [ -n "$out" ] || out="$(head -c 110 "$err" | tr '\n' ' ')"
  printf '%-34s | %-4s | %s\n' "$(basename "$probe")" "$rc" "$out"
done
