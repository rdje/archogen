#!/usr/bin/env bash
# scripts/handoff_census.sh — the handoff census, sampled twice (leaf `PROGRAM.54`).
#
# ⭐ WHY. `scripts/check_no_background_jobs.sh`, the scaffold's census, judges one instant. A terminal can respawn a
# helper every few seconds whose command line names the checkout — iTerm's `pidinfo --git-state <repo> 4 1`, measured
# `2026-10-05` by three review readers at 4–5 s old, PIDs changing on every respawn — so the census's verdict depended
# on when it ran: refused at one instant, `handoff: OK` at the next. A job that can rewrite tracked files outlives a
# few seconds; a helper does not. So a process counts here only when a second sample, taken after a pause, still holds
# it, by its PID and its command line: a property, as the census's own header asks, and no list of names.
#
# The census itself is the scaffold's (`scripts/update_scaffold.sh`'s NEUTRAL list): an edit to it would be undone by
# the next update, and the scaffold's repository is read-only from here, so the second sample lives in this wrapper
# and the property is proposed to the scaffold's owner (`PROGRAM.75`).
#
# Usage:  bash scripts/handoff_census.sh             the census, and a second sample if it names anything
#         bash scripts/handoff_census.sh --self-test  its arms, on processes of their own
# The pause is `HANDOFF_RESAMPLE_SECONDS`, 8 unless set.
# Exit:   0 = nothing that persists across both samples · 1 = a process persists, named · 2 = the census could not run
#
# ⚠️ HONEST LIMIT: a job that starts after the first sample is seen only by the second, and is not counted — the census
# run once more after it finds it; and one whose PID and command line both change between samples reads as gone.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)" || exit 2
CENSUS="$ROOT/scripts/check_no_background_jobs.sh"
PAUSE="${HANDOFF_RESAMPLE_SECONDS:-8}"

# The processes a census run names: its indented lines, `<pid> <etime> <command>`, reduced to `<pid> <command>`.
named() { printf '%s\n' "$1" | awk '/^  [0-9]+ / { pid = $1; $1 = ""; $2 = ""; sub(/^  */, ""); print pid " " $0 }'; }

census() {
  local first rc second rc2 persisting gone
  first="$(bash "$CENSUS" 2>&1)"; rc=$?
  if [ "$rc" -ne 1 ]; then printf '%s\n' "$first"; return "$rc"; fi
  sleep "$PAUSE"
  second="$(bash "$CENSUS" 2>&1)"; rc2=$?
  if [ "$rc2" -eq 2 ]; then printf '%s\n' "$second"; return 2; fi
  persisting="$(comm -12 <(named "$first" | sort) <(named "$second" | sort))"
  gone="$(comm -23 <(named "$first" | sort) <(named "$second" | sort))"
  [ -z "$gone" ] || printf '%s\n' "$gone" | sed "s/^/transient, gone $PAUSE s later: /"
  if [ -n "$persisting" ]; then
    printf 'handoff: %s process(es) named by both samples, %s s apart — not handoff-ready:\n' \
      "$(printf '%s\n' "$persisting" | wc -l | tr -d ' ')" "$PAUSE"
    printf '%s\n' "$persisting" | sed 's/^/  /'
    return 1
  fi
  printf 'handoff: OK — whatever the first sample named was gone %s s later (PROGRAM.54)\n' "$PAUSE"
  return 0
}

self_test() {
  local arms=0 ok=0 out rc short long
  # A process whose command line names this checkout, as the census's PROJECT-WORK arm reads it: a shell that waits
  # on its sleep, so the shell itself — its argv holding the path — lives as long as the sleep.
  spawn() { bash -c 'sleep "$1"; :' "$ROOT/handoff-census-arm" "$1" >/dev/null 2>&1 & echo $!; }
  arm() { # $1 = name, $2 = what must hold
    arms=$((arms + 1))
    if eval "$2"; then ok=$((ok + 1)); echo "  ✅ $1"; else echo "SELF-TEST: $1" >&2; printf '%s\n' "$out" | sed 's/^/    /' >&2; fi
  }
  short="$(spawn 2)"
  out="$(HANDOFF_RESAMPLE_SECONDS=4 bash "${BASH_SOURCE[0]}" 2>&1)"; rc=$?
  arm "a process the first sample names and the second does not is not counted" \
    "printf '%s' \"\$out\" | grep -qE '^transient, gone 4 s later: $short ' && ! printf '%s' \"\$out\" | grep -qE '^  $short '"
  long="$(spawn 30)"
  out="$(HANDOFF_RESAMPLE_SECONDS=2 bash "${BASH_SOURCE[0]}" 2>&1)"; rc=$?
  arm "a process both samples hold is named, the run refused" \
    "[ \"\$rc\" -eq 1 ] && printf '%s' \"\$out\" | grep -qE '^  $long .*handoff-census-arm'"
  kill "$long" 2>/dev/null; pkill -f "$ROOT/handoff-census-arm" 2>/dev/null; wait 2>/dev/null
  echo "handoff-census self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test ;;
  "") census ;;
  *) printf 'usage: bash scripts/handoff_census.sh [--self-test]\n' >&2; exit 2 ;;
esac
