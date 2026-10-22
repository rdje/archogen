#!/usr/bin/env bash
# exec_pressure.sh — load this host's exec path the way concurrent builds do, so that a test's wall-clock margin can
# be measured under that load instead of guessed at (leaf PROGRAM.99).
#
# Why: on 2026-10-21 the `xtask` suite failed 73 and then 31 tests while other projects' builds ran on the host, and
# its failing tests passed alone and on a quiet host. A build hands the kernel one new program after another — build
# scripts, test binaries, each freshly linked — and the host checks each new code signature before it runs. Each
# worker here does the same: it compiles a program no earlier exec has seen (a unique string, so a unique signature)
# and runs it, over and over. Everything is written under this repository's `target/exec-pressure/`; no other
# project is touched.
#
# Usage:
#   bash scripts/exec_pressure.sh --probe [--workers N]        time a fresh program built and run, quiet and loaded
#   bash scripts/exec_pressure.sh [--workers N] -- COMMAND…     run COMMAND under the load; the load stops with it
#
# --workers defaults to the number of logical processors.
#
# Exit codes:
#   --probe   0 the load at least doubled the median time · 1 it did not, so no reproduction may claim this load
#             · 2 usage, no C compiler, or no clock finer than a second
#   COMMAND   the command's own status (2 when the load cannot be started)

set -uo pipefail

ROOT="$(git rev-parse --show-toplevel)" || exit 2
cd "$ROOT" || exit 2

die() { echo "exec-pressure: $*" >&2; exit 2; }

workers=""
probe=0
command=()
while [ $# -gt 0 ]; do
  case "$1" in
    --probe) probe=1; shift ;;
    --workers) [ $# -ge 2 ] || die "--workers needs a count"; workers="$2"; shift 2 ;;
    --) shift; command=("$@"); break ;;
    *) die "unknown argument '$1' — see the header for usage" ;;
  esac
done
[ -n "$workers" ] || workers="$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo 4)"
case "$workers" in ''|*[!0-9]*|0) die "--workers takes a positive count, not '$workers'" ;; esac
if [ "$probe" -eq 1 ] && [ ${#command[@]} -gt 0 ]; then die "--probe takes no command"; fi
if [ "$probe" -eq 0 ] && [ ${#command[@]} -eq 0 ]; then die "give --probe, or -- and a command to run under load"; fi
command -v cc >/dev/null 2>&1 || die "no C compiler (cc) on PATH"
[ -n "${EPOCHREALTIME:-}" ] || die "this bash has no EPOCHREALTIME (bash 5 or later is needed for a sub-second clock)"

mkdir -p "$ROOT/target/exec-pressure" || die "cannot create target/exec-pressure"
run_dir="$(mktemp -d "$ROOT/target/exec-pressure/run.XXXXXX")" || die "cannot create a run directory"
pids=()

# A worker stops at the top of its loop once the stop file exists, so the program it is compiling or running finishes
# first and nothing it started outlives the run; waiting on each worker then waits for all of them.
stop_load() {
  local pid
  : > "$run_dir/stop"
  for pid in "${pids[@]+"${pids[@]}"}"; do wait "$pid" 2>/dev/null; done
  pids=()
  rm -f "$run_dir/stop"
}
cleanup() { stop_load; rm -rf "$run_dir"; }
trap cleanup EXIT
trap 'exit 130' INT TERM

# One fresh program at `$1`: its string names the run, the slot and a counter, so no two share a signature.
compile_fresh() {
  printf '#include <stdio.h>\nint main(void) { return puts("%s") < 0; }\n' "$2" | cc -x c -o "$1" - 2>/dev/null
}

worker() {
  local dir="$run_dir/w$1" n=0
  mkdir -p "$dir" || return
  while [ ! -e "$run_dir/stop" ]; do
    n=$((n + 1))
    compile_fresh "$dir/p" "$run_dir w$1 $n" && "$dir/p" >/dev/null
    rm -f "$dir/p"
  done
}

start_load() {
  local i
  for ((i = 1; i <= workers; i++)); do
    worker "$i" &
    pids+=("$!")
  done
}

# The microseconds a fresh program takes to be built and then run once — the unit of work a build repeats.
build_and_run() {
  local p="$run_dir/probe" t0 t1
  t0=$EPOCHREALTIME
  compile_fresh "$p" "$run_dir probe $1" || return 1
  "$p" >/dev/null || return 1
  t1=$EPOCHREALTIME
  rm -f "$p"
  echo $(( 10#${t1/[.,]/} - 10#${t0/[.,]/} ))
}

median() { sort -n | awk '{ v[NR] = $1 } END { print (NR % 2) ? v[(NR + 1) / 2] : int((v[NR / 2] + v[NR / 2 + 1]) / 2) }'; }

syspolicyd_cpu() {
  ps -A -o pcpu=,comm= 2>/dev/null | awk '/syspolicyd$/ { printf "%s", $1; found = 1 } END { if (!found) printf "n/a" }'
}

if [ "$probe" -eq 1 ]; then
  samples=15
  quiet="$(for ((k = 1; k <= samples; k++)); do build_and_run "quiet $k" || exit 1; done | median)" \
    || die "a quiet sample failed to compile or run"
  start_load
  sleep 5
  loaded="$(for ((k = 1; k <= samples; k++)); do build_and_run "loaded $k" || exit 1; done | median)" \
    || die "a loaded sample failed to compile or run"
  cpu="$(syspolicyd_cpu)"
  stop_load
  [ "$quiet" -gt 0 ] || quiet=1
  ratio="$(awk -v a="$loaded" -v b="$quiet" 'BEGIN { printf "%.1f", a / b }')"
  echo "exec-pressure: median build and run of a fresh program — quiet ${quiet} µs, under ${workers} workers ${loaded} µs (×${ratio}); syspolicyd ${cpu} % CPU"
  if awk -v a="$loaded" -v b="$quiet" 'BEGIN { exit !(a >= 2 * b) }'; then
    echo "exec-pressure: the load at least doubles the time a fresh program takes to build and run on this host"
    exit 0
  fi
  echo "exec-pressure: the load did not double the latency — no reproduction may claim it" >&2
  exit 1
fi

start_load
sleep 5
echo "exec-pressure: ${workers} workers running; syspolicyd $(syspolicyd_cpu) % CPU; running: ${command[*]}" >&2
"${command[@]}"
status=$?
echo "exec-pressure: the command exited ${status}; syspolicyd $(syspolicyd_cpu) % CPU; stopping the load" >&2
exit "$status"
