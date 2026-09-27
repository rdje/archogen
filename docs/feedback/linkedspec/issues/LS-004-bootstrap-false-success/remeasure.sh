#!/usr/bin/env bash
# LS-004 re-measurement — does RGX's public bootstrap stop at the first failed prerequisite, and
# does it stop claiming a seed that did not happen?
#
# This is NOT repro.sh. repro.sh provoked the failure through LS-001's workspace collision and by
# calling PGEN's internal make target directly. Both routes are gone: the collision is fixed, and the
# vendor's guide now says applications "should not reproduce PGEN's internal generation steps or
# modify either submodule". This instrument drives only RGX's **published** downstream interface —
# `make -C <checkout>/rgx bootstrap`, through the vendor's documented storage wrapper — exactly as
# the completion notice describes its own controls.
#
#   usage: remeasure.sh self-test                  (also accepted: --self-test)
#          remeasure.sh classify <log> <exit-code> <files-in-generated> <failure|success|reuse>
#          remeasure.sh arm failure --store <empty-dir>   # one empty-store offline control
#          remeasure.sh arm success                       # fresh preparation (needs network)
#          remeasure.sh arm reuse                         # prepared reuse; expect a no-op
#          [--app-root <dir>] [--checkout <dir>]
#
# ── The criteria, pre-registered from the report's own "Expected" ────────────────────────────────
# The report asked for two things: "Fail at the first failed prerequisite, and do not claim a seed
# that did not happen." Mechanical form, over a run's log and its aftermath:
#   errors          lines starting with `error` — distinct failing invocations
#   missing_product lines reporting a build product that is not there
#   seeds           lines claiming a generated file was seeded
#   later           named steps announced AFTER the first failure
#   nfiles          files actually in `generated/` when the run ended
#   symptom 1 — continued past a failed prerequisite := errors >= 2, OR a missing build product was
#               invoked, OR a named step was announced after the first failure, OR a seed was claimed
#               by a run that then FAILED (on a successful preparation the seed really happened, so
#               the claim indicts nothing)
#   symptom 2 — claimed a seed that did not happen  := seeds >= 1 AND nfiles == 0
# Note the tense distinction the report turns on: "Seeding generated/ebnf.rs ..." announces an
# attempt and is ordinary build logging; "generated/ebnf.rs seeded." claims completion, and only the
# second one can be false. `seeds` counts the completion claim.
# Verdicts per arm:
#   failure arm: exit 0 -> 2 (the control did not provoke a failure, so it decides nothing);
#                symptom 1 or 2 -> 1; neither -> 0 (stopped at the first failure, claimed nothing)
#   success arm: exit != 0 -> 2 (preparation failed for another reason; report it, do not classify);
#                nfiles == 0 -> 1 (a success exit that produced nothing IS the defect class); else 0
#   reuse arm:   exit 0 -> 0; exit != 0 -> 2
#
# CONTRACT (exit code is the verdict):
#   0 = the defect is GONE for this arm · 1 = STILL PRESENT · 2 = could not run or could not decide
set -uo pipefail

APP_ROOT=""
CHECKOUT=""
STORE=""
ARM=""

sha() { if command -v sha256sum >/dev/null 2>&1; then sha256sum "$@"; else shasum -a 256 "$@"; fi; }
mtime() {
  stat -c '%y' "$1" 2>/dev/null | cut -d. -f1 && return 0
  /usr/bin/stat -f '%Sm' -t '%Y-%m-%d %H:%M' "$1" 2>/dev/null && return 0
  echo unknown
}

# ── the classifier: pure, and the only thing self-test exercises ────────────────────────────────
classify() { # $1 log, $2 exit code, $3 nfiles, $4 arm kind
  local log="$1" rc="$2" nfiles="$3" kind="$4"
  local errors missing seeds later sym1 sym2
  [ -f "$log" ] || { echo "LS-004 remeasure: log not found: $log" >&2; return 2; }
  case "$kind" in failure|success|reuse) ;; *) echo "LS-004 remeasure: unknown arm kind: $kind" >&2; return 2 ;; esac

  errors="$(grep -cE '^error' "$log" || true)"
  missing="$(grep -cE 'No such file or directory' "$log" || true)"
  seeds="$(grep -cE 'seeded\.' "$log" || true)"
  # A named step announced AFTER the first failure means preparation kept working on a foundation it
  # already knew was missing — the report's first symptom, in the form the notice words it.
  later="$(awk '/^error/{after=1; next} after && /Step [A-Z]|Generating the|Bootstrap regex|Seeding |seeded\./{n++} END{print n+0}' "$log")"

  sym1=0; sym2=0
  # A seed claim only indicts the run if the run FAILED: on a successful preparation the seed really
  # happened, and printing "symptom present" there would misreport a good run.
  { [ "$errors" -ge 2 ] || [ "$missing" -ge 1 ] || [ "$later" -ge 1 ] \
    || { [ "$seeds" -ge 1 ] && [ "$rc" -ne 0 ]; }; } && sym1=1
  { [ "$seeds" -ge 1 ] && [ "$nfiles" -eq 0 ]; } && sym2=1

  echo "== LS-004 classification: $kind arm =="
  echo "  exit status .......................... $rc"
  echo "  '^error' lines (distinct failures) ... $errors"
  echo "  missing build product invoked ........ $missing"
  echo "  seeding messages printed ............. $seeds"
  echo "  named steps after the first failure .. $later"
  echo "  files in generated/ afterwards ....... $nfiles"
  echo "  symptom 1 — continued past a failed prerequisite ... $([ "$sym1" -eq 1 ] && echo PRESENT || echo 'not seen')"
  echo "  symptom 2 — claimed a seed that did not happen ..... $([ "$sym2" -eq 1 ] && echo PRESENT || echo 'not seen')"

  case "$kind" in
    failure)
      if [ "$rc" -eq 0 ]; then
        echo "LS-004 remeasure: RESULT — could not decide: the empty-store control exited 0, so it never"
        echo "  provoked the missing prerequisite this arm exists to provoke."
        return 2
      fi
      if [ "$sym1" -eq 1 ] || [ "$sym2" -eq 1 ]; then
        echo "LS-004 remeasure: RESULT — the defect is STILL PRESENT: preparation did not stop at the"
        echo "  first failed prerequisite, or claimed a seed it did not produce."
        return 1
      fi
      echo "LS-004 remeasure: RESULT — the defect is GONE: preparation failed at the first missing"
      echo "  prerequisite, ran no later step, and claimed no seed."
      return 0 ;;
    success)
      if [ "$rc" -ne 0 ]; then
        echo "LS-004 remeasure: RESULT — could not decide: preparation exited $rc, which is a failure of"
        echo "  the published interface rather than evidence about this defect. Preserve the log."
        return 2
      fi
      if [ "$nfiles" -eq 0 ]; then
        echo "LS-004 remeasure: RESULT — the defect is STILL PRESENT: a success exit with nothing generated."
        return 1
      fi
      echo "LS-004 remeasure: RESULT — the defect is GONE: preparation exited 0 and produced $nfiles files."
      return 0 ;;
    reuse)
      if [ "$rc" -ne 0 ]; then
        echo "LS-004 remeasure: RESULT — could not decide: the prepared reuse run exited $rc."
        return 2
      fi
      echo "LS-004 remeasure: RESULT — the defect is GONE: reuse of a prepared checkout exits 0."
      return 0 ;;
  esac
}

# ── self-test: the classifier against the historical log and the shapes it must not accept ──────
self_test() {
  local t arms=0 ok=0 rc
  t="$(mktemp -d)" || return 2

  # RED arm — the log tail exactly as frozen in evidence/OBSERVED.txt at PGEN db6f8c68: two failing
  # cargo invocations, a missing build product invoked twice, one false seed, nothing generated.
  cat > "$t/historical.log" <<'LOG'
error: current package believes it's in a workspace when it's not:
/bin/bash: ./target/debug/ast_pipeline: No such file or directory
/bin/bash: ./target/debug/ast_pipeline: No such file or directory
🌱 generated/ebnf.rs seeded.
error: current package believes it's in a workspace when it's not:
make: *** [regex_parser_bootstrap] Error 101
LOG

  # A run that stops at the first missing prerequisite and claims nothing.
  cat > "$t/failfast.log" <<'LOG'
error: no matching package named `aho-corasick` found
location searched: registry `crates-io`
required by package `pgen v1.1.106`
As a reminder, you're using offline mode (--offline) while the registry index is not populated
make: *** [bootstrap] Error 101
LOG

  # The same run, but claiming a seed it did not produce.
  printf 'generated/ebnf.rs seeded.\n' > "$t/failfast-seeded.log"
  cat "$t/failfast.log" >> "$t/failfast-seeded.log"

  # One failure, no seed claim — but a later named step anyway: the report's first symptom in the
  # form the completion notice words it ("without later named steps").
  cat > "$t/failfast-continued.log" <<'LOG'
error: no matching package named `aho-corasick` found
location searched: registry `crates-io`
  Step B: generate the ebnf parser from the seeded front end...
make: *** [bootstrap] Error 2
LOG

  printf 'Bootstrap complete\n' > "$t/success.log"

  # A successful preparation that really did seed: the claim is true, so it must not be read as the
  # symptom. This arm exists because the first version of the classifier got it wrong and printed
  # "symptom 1 PRESENT" over a clean run.
  cat > "$t/success-seeded.log" <<'LOG'
🌱 Seeding generated/ebnf.rs (one-time bootstrap, Rust frontend)...
generated/ebnf.rs seeded.
✅ Bootstrap complete.
LOG

  echo "LS-004 remeasure: self-test"
  arms=9; ok=0
  classify "$t/historical.log" 2 0 failure >/dev/null 2>&1; rc=$?
  [ "$rc" -eq 1 ] && { ok=$((ok+1)); echo "  arm passes: the historical false-success run is STILL PRESENT (exit $rc)"; } || echo "  ARM FAILED: historical log — expected 1, got $rc" >&2
  classify "$t/failfast.log" 101 0 failure >/dev/null 2>&1; rc=$?
  [ "$rc" -eq 0 ] && { ok=$((ok+1)); echo "  arm passes: stopping at the first missing prerequisite is GONE (exit $rc)"; } || echo "  ARM FAILED: fail-fast log — expected 0, got $rc" >&2
  classify "$t/failfast-seeded.log" 101 0 failure >/dev/null 2>&1; rc=$?
  [ "$rc" -eq 1 ] && { ok=$((ok+1)); echo "  arm passes: a seed claim with nothing generated is STILL PRESENT (exit $rc)"; } || echo "  ARM FAILED: seeded fail-fast log — expected 1, got $rc" >&2
  classify "$t/failfast-continued.log" 101 0 failure >/dev/null 2>&1; rc=$?
  [ "$rc" -eq 1 ] && { ok=$((ok+1)); echo "  arm passes: a named step after the failure is STILL PRESENT (exit $rc)"; } || echo "  ARM FAILED: continued log — expected 1, got $rc" >&2
  classify "$t/success.log" 0 12 success >/dev/null 2>&1; rc=$?
  [ "$rc" -eq 0 ] && { ok=$((ok+1)); echo "  arm passes: a success exit that produced files is GONE (exit $rc)"; } || echo "  ARM FAILED: success arm — expected 0, got $rc" >&2
  classify "$t/success.log" 0 0 success >/dev/null 2>&1; rc=$?
  [ "$rc" -eq 1 ] && { ok=$((ok+1)); echo "  arm passes: a success exit that produced nothing is STILL PRESENT (exit $rc)"; } || echo "  ARM FAILED: empty success arm — expected 1, got $rc" >&2
  classify "$t/success-seeded.log" 0 12 success >/dev/null 2>&1; rc=$?
  [ "$rc" -eq 0 ] && { ok=$((ok+1)); echo "  arm passes: a true seed claim on a successful run is not a symptom (exit $rc)"; } || echo "  ARM FAILED: seeded success arm — expected 0, got $rc" >&2
  classify "$t/success.log" 0 0 failure >/dev/null 2>&1; rc=$?
  [ "$rc" -eq 2 ] && { ok=$((ok+1)); echo "  arm passes: a failure control that exited 0 is refused (exit $rc)"; } || echo "  ARM FAILED: refusal arm — expected 2, got $rc" >&2
  classify "$t/failfast.log" 101 0 success >/dev/null 2>&1; rc=$?
  [ "$rc" -eq 2 ] && { ok=$((ok+1)); echo "  arm passes: a failed preparation is refused, not classified (exit $rc)"; } || echo "  ARM FAILED: success refusal arm — expected 2, got $rc" >&2

  rm -rf "$t"
  echo "self-test: $ok/$arms arms passed"
  [ "$ok" -eq "$arms" ] || return 1
  return 0
}

# ── the arm runners: documented storage, published interface, nothing invented ──────────────────
setup_paths() {
  local here
  if [ -z "$APP_ROOT" ]; then
    here="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
    APP_ROOT="$(git -C "$here" rev-parse --show-toplevel 2>/dev/null || true)"
    [ -n "$APP_ROOT" ] || { echo "LS-004 remeasure: pass --app-root <dir>" >&2; return 2; }
  fi
  [ -n "$CHECKOUT" ] || CHECKOUT="$APP_ROOT/vendor/linkedspec"
  CHECKOUT="$(cd -- "$CHECKOUT" 2>/dev/null && pwd -P)" || { echo "LS-004 remeasure: checkout not found" >&2; return 2; }
  PGEN="$CHECKOUT/rgx/subs/pgen"
  DATA="$APP_ROOT/.app-data"
  LOGDIR="$DATA/ls004"
  [ -d "$PGEN/rust" ] || { echo "LS-004 remeasure: no nested PGEN checkout — initialize it first (SETUP.md)" >&2; return 2; }
  [ -f "$CHECKOUT/tools/project_data_run.sh" ] || { echo "LS-004 remeasure: the documented storage wrapper is missing from this checkout" >&2; return 2; }
  command -v make >/dev/null || { echo "LS-004 remeasure: make not found" >&2; return 2; }
  command -v cargo >/dev/null || { echo "LS-004 remeasure: cargo not found" >&2; return 2; }
  mkdir -p "$LOGDIR" || return 2
  return 0
}

# Preserve the parser sources that predate this run, once, and verify the copy by file count.
back_up_generated() {
  local dst="$DATA/pgen-generated-before-remeasure" a b
  [ -d "$PGEN/generated" ] || { echo "  generated/ absent — nothing to back up"; return 0; }
  if [ -d "$dst" ]; then echo "  a backup already exists at <app-data>/pgen-generated-before-remeasure — kept"; return 0; fi
  mkdir -p "$dst" || return 2
  cp -Rp "$PGEN/generated/." "$dst/" || return 2
  a="$(find "$PGEN/generated" -type f | wc -l | tr -d ' ')"
  b="$(find "$dst" -type f | wc -l | tr -d ' ')"
  [ "$a" = "$b" ] || { echo "LS-004 remeasure: backup verification failed ($a vs $b files)" >&2; return 2; }
  echo "  backed up $a generated files to <app-data>/pgen-generated-before-remeasure (verified by count)"
}

run_bootstrap() { # $1 offline true|false, $2 cargo home, $3 log
  ( cd "$APP_ROOT" && \
    LINKEDSPEC_PROJECT_DATA_ROOT="$DATA/linkedspec" \
    LINKEDSPEC_CACHE_ROOT="$DATA/linkedspec/cache" \
    LINKEDSPEC_SCRATCH_ROOT="$DATA/linkedspec/scratch" \
    CARGO_HOME="$2" \
    CARGO_TARGET_DIR="$DATA/target" \
    CARGO_NET_OFFLINE="$1" \
    bash "$CHECKOUT/tools/project_data_run.sh" env -u CARGO_TARGET_DIR \
      make -C "$CHECKOUT/rgx" bootstrap ) >"$3" 2>&1
  return $?
}

nfiles_generated() { find "$PGEN/generated" -type f 2>/dev/null | wc -l | tr -d ' '; }

parser_hash() { # $1 directory — one digest over its generated Rust sources, path-relative.
  # ⛔ Not `xargs sha`: sha is a shell function and xargs cannot call one — it fails and the digest
  # degenerates to the hash of empty input, which would report every run as "UNCHANGED".
  local d="$1" n out
  [ -d "$d" ] || { echo none; return 0; }
  n="$(cd "$d" && find . -name '*.rs' -type f | wc -l | tr -d ' ')"
  [ "$n" -gt 0 ] || { echo none; return 0; }
  out="$( cd "$d" && find . -name '*.rs' -type f | sort | while IFS= read -r f; do sha "$f"; done | sha | cut -c1-16 )"
  printf '%s\n' "${out:-none}"
}

if [ "${1:-}" = self-test ] || [ "${1:-}" = --self-test ]; then self_test; exit $?; fi

if [ "${1:-}" = classify ]; then
  [ "$#" -eq 5 ] || { echo "usage: remeasure.sh classify <log> <exit-code> <nfiles> <failure|success|reuse>" >&2; exit 2; }
  classify "$2" "$3" "$4" "$5"; exit $?
fi

while [ $# -gt 0 ]; do
  case "$1" in
    --app-root) APP_ROOT="${2:?--app-root needs a directory}"; shift 2 ;;
    --checkout) CHECKOUT="${2:?--checkout needs a directory}"; shift 2 ;;
    --store)    STORE="${2:?--store needs a directory}"; shift 2 ;;
    arm)        ARM="${2:?arm needs failure|success|reuse}"; shift 2 ;;
    -h|--help)  sed -n '2,40p' "$0"; exit 0 ;;
    *)          echo "LS-004 remeasure: unknown argument: $1" >&2; exit 2 ;;
  esac
done

[ -n "$ARM" ] || { echo "LS-004 remeasure: needs self-test, classify, or arm <failure|success|reuse>" >&2; exit 2; }
setup_paths || exit 2
stamp="$(date +%Y%m%d-%H%M%S)"
rev="$(git -C "$CHECKOUT" rev-parse --short HEAD 2>/dev/null || echo 'unknown revision')"

case "$ARM" in
  failure)
    [ -n "$STORE" ] || { echo "LS-004 remeasure: arm failure needs --store <empty-dir>" >&2; exit 2; }
    case "$STORE" in /*) ;; *) STORE="$APP_ROOT/$STORE" ;; esac
    if [ -d "$STORE" ] && [ -n "$(ls -A "$STORE" 2>/dev/null)" ]; then
      echo "LS-004 remeasure: --store must be empty or absent: $STORE" >&2; exit 2
    fi
    mkdir -p "$STORE" || exit 2
    log="$LOGDIR/failure-$stamp.log"
    echo "== LS-004 failure arm — empty offline package store =="
    echo "  checkout ..................... ${CHECKOUT##*/} at $rev"
    echo "  package store ................ ${STORE##*/} (created empty; CARGO_NET_OFFLINE=true)"
    echo "  interface .................... make -C <checkout>/rgx bootstrap, via the documented storage wrapper"
    back_up_generated || exit 2
    # RGX's published contract: `make bootstrap` is idempotent on EXISTENCE, so after a pin bump the
    # generated parser must be removed first or the run silently reuses the previous pin's parser.
    rm -rf "$PGEN/generated" || exit 2
    echo "  generated/ removed ........... yes (the published regeneration step after a pin bump)"
    echo "  log .......................... <app-data>/ls004/$(basename "$log")"
    echo
    run_bootstrap true "$STORE" "$log"; rc=$?
    echo "-- log (first 12 and last 6 lines) --"
    head -12 "$log" | sed 's/^/  /'
    echo "  ..."
    tail -6 "$log" | sed 's/^/  /'
    echo
    classify "$log" "$rc" "$(nfiles_generated)" failure
    exit $?
    ;;
  success|reuse)
    log="$LOGDIR/$ARM-$stamp.log"
    echo "== LS-004 $ARM arm — RGX's published preparation interface =="
    echo "  checkout ..................... ${CHECKOUT##*/} at $rev"
    echo "  package store ................ the retained application-local store (CARGO_NET_OFFLINE=false)"
    echo "  interface .................... make -C <checkout>/rgx bootstrap, via the documented storage wrapper"
    if [ "$ARM" = success ]; then
      back_up_generated || exit 2
      # Both removals are the published contract's own instructions after a PGEN pin bump:
      # regenerate the parser, and do not trust a target/ from before the bump.
      for stale in "$PGEN/generated" "$PGEN/rust/target"; do
        if [ -e "$stale" ]; then
          echo "  removing stale ............... ${stale##*/pgen/} ($(du -sh "$stale" 2>/dev/null | cut -f1), dated $(mtime "$stale"))"
          rm -rf "$stale" || exit 2
        fi
      done
      before_hash="$(parser_hash "$DATA/pgen-generated-before-remeasure")"
    else
      echo "  generated/ left in place ..... yes (this arm measures reuse of a prepared checkout)"
      before_hash="$(parser_hash "$PGEN/generated")"
    fi
    echo "  log .......................... <app-data>/ls004/$(basename "$log")"
    echo
    run_bootstrap false "$DATA/cargo-home" "$log"; rc=$?
    echo "-- log (first 8 and last 10 lines) --"
    head -8 "$log" | sed 's/^/  /'
    echo "  ..."
    tail -10 "$log" | sed 's/^/  /'
    echo
    echo "  files in generated/ .......... $(nfiles_generated)"
    after_hash="$(parser_hash "$PGEN/generated")"
    if [ "$before_hash" = "$after_hash" ]; then
      echo "  parser sources ............... UNCHANGED ($after_hash) — this run generated nothing new"
    else
      echo "  parser sources ............... regenerated: ${before_hash:-none} -> $after_hash"
    fi
    echo
    classify "$log" "$rc" "$(nfiles_generated)" "$ARM"
    exit $?
    ;;
  *) echo "LS-004 remeasure: unknown arm: $ARM" >&2; exit 2 ;;
esac
