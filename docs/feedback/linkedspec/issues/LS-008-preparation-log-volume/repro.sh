#!/usr/bin/env bash
# LS-008 reproducer — how much log does ONE SUCCESSFUL run of RGX's published preparation write?
#
# The guide says that if the preparation command fails, a consumer must "stop before building the
# application and preserve its exit status and full log". This measures what "full log" costs on a run
# that SUCCEEDS: its bytes, its lines, and how many of them are PGEN's `[PGEN][DBG]` progress lines.
#
# USAGE
#   repro.sh --log <file> --exit <code>
#       measure a log already captured, with the exit code of the run that wrote it — so the finding can
#       be replayed without a fresh bootstrap
#   repro.sh --run [--app-root <dir>] [--checkout <dir>]
#       run the documented fresh preparation, then measure its log. The checkout defaults to
#       <app-root>/vendor/linkedspec. The log is kept under <app-root>/.app-data/ls008/ and reported, never
#       deleted: at the measured size a consumer decides whether to keep it.
#   repro.sh --self-test
#       the verdict logic against small synthetic logs
#
# VERDICT (the exit code)
#   0  reproduces — a successful run's log is at least 100 MiB (THRESHOLD_BYTES)
#   3  changed    — a successful run's log is below that
#   2  could not run — no log, or the run it came from did not succeed
#
# ⚠️ The threshold is a judgement, stated so it can be argued with: 100 MiB is a log no consumer keeps by
# accident. The report's own measurement is far above it, so a small change in either direction does not
# move the verdict.
set -uo pipefail
HERE="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
# Scratch lives under <root>/target/feedback_scratch/, where <root> is the enclosing work tree if there is one
# and this directory otherwise — on the reader's own volume, never in a system temporary directory.
ROOT="$(git -C "$HERE" rev-parse --show-toplevel 2>/dev/null || printf '%s' "$HERE")"
THRESHOLD_BYTES=$((100 * 1024 * 1024))

# $1 = log, $2 = exit code of the run that wrote it
measure() {
  local log="$1" rc="$2" bytes lines dbg
  [ -f "$log" ] || { echo "LS-008: no such log: $log" >&2; return 2; }
  bytes="$(wc -c < "$log" | tr -d ' ')"
  lines="$(wc -l < "$log" | tr -d ' ')"
  dbg="$(grep -c '\[PGEN\]\[DBG\]' "$log" || true)"
  echo "  exit code of the run ......... $rc"
  echo "  log size ..................... $bytes bytes ($((bytes / 1000000)) MB)"
  echo "  log lines .................... $lines"
  echo "  [PGEN][DBG] lines ............ $dbg"
  if [ "$rc" != 0 ]; then
    echo "LS-008: the run did not succeed (exit $rc) — this issue is about what a SUCCESSFUL run writes" >&2
    return 2
  fi
  if [ "$bytes" -ge "$THRESHOLD_BYTES" ]; then
    echo "LS-008: REPRODUCED — one successful preparation wrote $((bytes / 1000000)) MB of log"
    return 0
  fi
  echo "LS-008: CHANGED — a successful preparation wrote $((bytes / 1000000)) MB, below the $((THRESHOLD_BYTES / 1048576)) MiB threshold"
  return 3
}

self_test() {
  local work arms=0 ok=0 rc
  mkdir -p "$ROOT/target/feedback_scratch" || return 2
  work="$(mktemp -d "$ROOT/target/feedback_scratch/LS-008.XXXXXX")" || return 2
  trap 'rm -rf "$work"' RETURN
  # a big synthetic log: 101 MiB of progress lines
  awk 'BEGIN { line = "[PGEN][DBG] 🧠 synthetic progress line, padded to make the file large ........................................"; for (i = 0; i < 1200000; i++) print line }' > "$work/big.log"
  printf '[rgx] done\n' > "$work/small.log"
  arm() { # $1 = name, $2 = expected rc, then the measure arguments
    local name="$1" want="$2"; shift 2
    arms=$((arms + 1))
    measure "$@" >/dev/null 2>&1; rc=$?
    if [ "$rc" -eq "$want" ]; then ok=$((ok + 1)); echo "  ✅ $name"; else echo "  ARM FAILED: $name — expected $want, got $rc" >&2; fi
  }
  arm "a successful run with a log over the threshold reproduces" 0 "$work/big.log" 0
  arm "a successful run with a small log is a change" 3 "$work/small.log" 0
  arm "a failed run is not this issue's evidence" 2 "$work/big.log" 101
  arm "a missing log could not be measured" 2 "$work/absent.log" 0
  echo "LS-008 self-test: $ok/$arms arms passed"
  [ "$ok" -eq "$arms" ]
}

run_preparation() {
  local app_root="$1" checkout="$2" pgen data log stamp rc
  checkout="$(cd -- "$checkout" 2>/dev/null && pwd -P)" || { echo "LS-008: checkout not found" >&2; return 2; }
  pgen="$checkout/rgx/subs/pgen"
  [ -d "$pgen/rust" ] || { echo "LS-008: no nested PGEN checkout — see SETUP.md" >&2; return 2; }
  [ -f "$checkout/tools/project_data_run.sh" ] || { echo "LS-008: the documented storage wrapper is missing" >&2; return 2; }
  data="$app_root/.app-data"
  mkdir -p "$data/ls008" || return 2
  stamp="$(date +%Y%m%d-%H%M%S)"
  log="$data/ls008/prepare-$stamp.log"
  # A FRESH preparation: the published contract's own steps after a pin change — regenerate the parser,
  # and do not trust a target/ from before. Both are ignored build products of the checkout.
  if [ -d "$pgen/generated" ] && [ ! -d "$data/ls008/generated-before" ]; then
    cp -Rp "$pgen/generated" "$data/ls008/generated-before" || return 2
  fi
  rm -rf "$pgen/generated" "$pgen/rust/target" || return 2
  echo "== LS-008 — one fresh run of RGX's published preparation =="
  echo "  checkout ..................... $(git -C "$checkout" rev-parse --short HEAD 2>/dev/null || echo unknown)"
  echo "  interface .................... make -C <checkout>/rgx bootstrap, via the documented storage wrapper"
  echo "  log .......................... <app-root>/.app-data/ls008/$(basename "$log")"
  ( cd "$app_root" && \
    LINKEDSPEC_PROJECT_DATA_ROOT="$data/linkedspec" \
    LINKEDSPEC_CACHE_ROOT="$data/linkedspec/cache" \
    LINKEDSPEC_SCRATCH_ROOT="$data/linkedspec/scratch" \
    CARGO_HOME="$data/cargo-home" \
    CARGO_TARGET_DIR="$data/target" \
    CARGO_NET_OFFLINE=false \
    bash "$checkout/tools/project_data_run.sh" env -u CARGO_TARGET_DIR \
      make -C "$checkout/rgx" bootstrap ) > "$log" 2>&1
  rc=$?
  measure "$log" "$rc"
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  --log)
    [ "${3:-}" = --exit ] && [ -n "${4:-}" ] || { echo "usage: repro.sh --log <file> --exit <code>" >&2; exit 2; }
    measure "$2" "$4"; exit $? ;;
  --run)
    shift
    APP_ROOT=""; CHECKOUT=""
    while [ $# -gt 0 ]; do
      case "$1" in
        --app-root) APP_ROOT="$2"; shift 2 ;;
        --checkout) CHECKOUT="$2"; shift 2 ;;
        *) echo "LS-008: unknown argument $1" >&2; exit 2 ;;
      esac
    done
    [ -n "$APP_ROOT" ] || APP_ROOT="$(git -C "$HERE" rev-parse --show-toplevel 2>/dev/null)"
    [ -n "$APP_ROOT" ] || { echo "LS-008: pass --app-root <dir>" >&2; exit 2; }
    [ -n "$CHECKOUT" ] || CHECKOUT="$APP_ROOT/vendor/linkedspec"
    run_preparation "$APP_ROOT" "$CHECKOUT"; exit $? ;;
  *) sed -n '2,24p' "$0"; exit 2 ;;
esac
