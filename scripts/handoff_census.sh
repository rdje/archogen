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
# Required reads are held by PROGRAM.80: each actual process snapshot and open-handle read must succeed. A failed
# ancestry lookup can mean that process exited; it is not the required full process snapshot. The native scaffold
# script stays unchanged; this wrapper observes its reads and withholds its verdict unless both completed.
#
# ⚠️ HONEST LIMIT: a job that starts after the first sample is seen only by the second, and is not counted — the census
# run once more after it finds it; and one whose PID and command line both change between samples reads as gone.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)" || exit 2
CENSUS="$ROOT/scripts/check_no_background_jobs.sh"
PAUSE="${HANDOFF_RESAMPLE_SECONDS:-8}"

# The processes a census run names: its indented lines, `<pid> <etime> <command>`, reduced to `<pid> <command>`.
named() { printf '%s\n' "$1" | awk '/^  [0-9]+ / { pid = $1; $1 = ""; $2 = ""; sub(/^  */, ""); print pid " " $0 }'; }

sample() (
  # A private receipt for the actual reads, not a preflight that could succeed before the census itself fails.
  # Functions are exported only into this subshell's native census; missing receipts also refuse a changed census
  # that stops using these readers. Scratch and receipts stay on the repository volume and are removed on exit.
  local guard out rc missing=""
  if ! mkdir -p "$ROOT/target/doctrine_scratch" ||
     ! guard="$(mktemp -d "$ROOT/target/doctrine_scratch/handoff_census.XXXXXX")"; then
    printf 'handoff: census unavailable — cannot prepare read receipts\n' >&2; return 2
  fi
  trap 'rm -rf -- "$guard"' EXIT
  export HANDOFF_CENSUS_RECEIPTS="$guard"
  export HANDOFF_CENSUS_PS="$(type -P ps)" HANDOFF_CENSUS_LSOF="$(type -P lsof)"
  ps() {
    local output status
    output="$(command "$HANDOFF_CENSUS_PS" "$@")"; status=$?
    if [ "${1:-}" = -Ao ] && [ "$status" -eq 0 ] && [ -n "$output" ]; then
      : > "$HANDOFF_CENSUS_RECEIPTS/ps-ok" || return 2
    fi
    printf '%s\n' "$output"
    return "$status"
  }
  lsof() {
    local status
    command "$HANDOFF_CENSUS_LSOF" "$@"; status=$?
    if [ "$status" -eq 0 ]; then : > "$HANDOFF_CENSUS_RECEIPTS/lsof-ok" || return 2; fi
    return "$status"
  }
  export -f ps lsof
  out="$(bash "$CENSUS" 2>&1)"; rc=$?
  [ -f "$guard/ps-ok" ] || missing="required process snapshot"
  [ -f "$guard/lsof-ok" ] || missing="${missing:+$missing and }required open-handle read"
  if [ -n "$missing" ]; then
    printf 'handoff: census unavailable — %s did not complete; outcome unknown\n' "$missing" >&2
    return 2
  fi
  printf '%s\n' "$out"
  return "$rc"
)

census() {
  local first rc second rc2 persisting gone
  first="$(sample 2>&1)"; rc=$?
  if [ "$rc" -ne 1 ]; then printf '%s\n' "$first"; return "$rc"; fi
  sleep "$PAUSE"
  second="$(sample 2>&1)"; rc2=$?
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
  local arms=0 ok=0 out rc short long work="$ROOT/target/doctrine_scratch/handoff_census/selftest"
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
  # Synthetic OS readers isolate required-read failures from real process access and from transient ancestry reads.
  rm -rf "${work:?}"; mkdir -p "$work/scripts" "$work/bin"
  cp "${BASH_SOURCE[0]}" "$work/scripts/handoff_census.sh"
  cp "$CENSUS" "$work/scripts/check_no_background_jobs.sh"
  cat > "$work/bin/ps" <<'SH'
#!/usr/bin/env bash
if [ "${1:-}" = -Ao ]; then printf '999999 00:00 synthetic\n'; else exit 1; fi
SH
  printf '#!/usr/bin/env bash\nexit 0\n' > "$work/bin/lsof"
  chmod +x "$work/bin/ps" "$work/bin/lsof"
  out="$(PATH="$work/bin:$PATH" bash "$work/scripts/handoff_census.sh" 2>&1)"; rc=$?
  arm "successful required reads pass even when an ancestry process has exited (PROGRAM.80)" \
    "[ \"\$rc\" -eq 0 ] && printf '%s' \"\$out\" | grep -q 'handoff: OK'"
  cp "$work/bin/ps" "$work/bin/ps.good"
  printf '#!/usr/bin/env bash\nexit 126\n' > "$work/bin/ps"
  out="$(PATH="$work/bin:$PATH" bash "$work/scripts/handoff_census.sh" 2>&1)"; rc=$?
  arm "a denied required process snapshot refuses, never OK (PROGRAM.80)" \
    "[ \"\$rc\" -eq 2 ] && printf '%s' \"\$out\" | grep -q 'required process snapshot' && ! printf '%s' \"\$out\" | grep -q 'handoff: OK'"
  printf '#!/usr/bin/env bash\nexit 0\n' > "$work/bin/ps"
  out="$(PATH="$work/bin:$PATH" bash "$work/scripts/handoff_census.sh" 2>&1)"; rc=$?
  arm "an empty successful process snapshot refuses, never OK (PROGRAM.80)" \
    "[ \"\$rc\" -eq 2 ] && printf '%s' \"\$out\" | grep -q 'required process snapshot' && ! printf '%s' \"\$out\" | grep -q 'handoff: OK'"
  cp "$work/bin/ps.good" "$work/bin/ps"
  printf '#!/usr/bin/env bash\nexit 1\n' > "$work/bin/lsof"
  out="$(PATH="$work/bin:$PATH" bash "$work/scripts/handoff_census.sh" 2>&1)"; rc=$?
  arm "a failed required open-handle read refuses, never OK (PROGRAM.80)" \
    "[ \"\$rc\" -eq 2 ] && printf '%s' \"\$out\" | grep -q 'required open-handle read' && ! printf '%s' \"\$out\" | grep -q 'handoff: OK'"
  cat > "$work/bin/ps" <<'SH'
#!/usr/bin/env bash
[ "${1:-}" = -Ao ] || exit 1
n=0; [ ! -f "$HANDOFF_CENSUS_ARM_DIR/ps-count" ] || read -r n < "$HANDOFF_CENSUS_ARM_DIR/ps-count"
n=$((n + 1)); printf '%s\n' "$n" > "$HANDOFF_CENSUS_ARM_DIR/ps-count"
if [ "$n" -eq 2 ] && [ "$HANDOFF_CENSUS_ARM_FAIL" = ps ]; then exit 126; fi
printf '999999 00:00 %s/census-held\n' "$HANDOFF_CENSUS_ARM_DIR"
SH
  cat > "$work/bin/lsof" <<'SH'
#!/usr/bin/env bash
n=0; [ ! -f "$HANDOFF_CENSUS_ARM_DIR/lsof-count" ] || read -r n < "$HANDOFF_CENSUS_ARM_DIR/lsof-count"
n=$((n + 1)); printf '%s\n' "$n" > "$HANDOFF_CENSUS_ARM_DIR/lsof-count"
if [ "$n" -eq 2 ] && [ "$HANDOFF_CENSUS_ARM_FAIL" = lsof ]; then exit 1; fi
SH
  local reader reason
  for reader in ps lsof; do
    rm -f "$work/ps-count" "$work/lsof-count"
    out="$(HANDOFF_CENSUS_ARM_DIR="$work" HANDOFF_CENSUS_ARM_FAIL="$reader" HANDOFF_RESAMPLE_SECONDS=0 \
      PATH="$work/bin:$PATH" bash "$work/scripts/handoff_census.sh" 2>&1)"; rc=$?
    case "$reader" in ps) reason='required process snapshot' ;; lsof) reason='required open-handle read' ;; esac
    arm "a failed $reader read in the second sample refuses, never OK (PROGRAM.80)" \
      "[ \"\$rc\" -eq 2 ] && printf '%s' \"\$out\" | grep -q '$reason' && ! printf '%s' \"\$out\" | grep -q 'handoff: OK'"
  done
  printf '#!/usr/bin/env bash\nprintf "handoff: OK synthetic\\n"\n' > "$work/scripts/check_no_background_jobs.sh"
  out="$(PATH="$work/bin:$PATH" bash "$work/scripts/handoff_census.sh" 2>&1)"; rc=$?
  arm "a census that makes neither required read cannot report OK (PROGRAM.80)" \
    "[ \"\$rc\" -eq 2 ] && ! printf '%s' \"\$out\" | grep -q 'handoff: OK'"
  rm -rf "${work:?}"
  echo "handoff-census self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test ;;
  "") census ;;
  *) printf 'usage: bash scripts/handoff_census.sh [--self-test]\n' >&2; exit 2 ;;
esac
