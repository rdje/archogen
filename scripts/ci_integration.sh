#!/usr/bin/env bash
# scripts/ci_integration.sh — the `integration` tier as CI runs it, and the blocking policy for its verdict
# (leaf `PROGRAM.10.3`; the reasoning is `docs/decisions/decision_incomplete-blocking-policy.md`).
#
# ⭐ THE POLICY, one arm per exit code the runner can give:
#   0   passed      → the job passes.
#   1   failed      → the job fails. A tool the workflow did not install lands here too, because the tier runs
#                     `--provisioned`: on an environment that claims to supply every tool, a missing one is its
#                     provisioning failing, not an absence.
#   20  incomplete  → the job PASSES, and says so on every run: a `::warning::` annotation per owned gap and the
#                     same list in the job summary. Under `--provisioned` nothing but a leaf-owned gap can produce
#                     it — a quarantine (`QUARANTINES` in `xtask/src/main.rs`: issue, owner, claim, scope) or a
#                     step not built (`Action::NotBuilt`, an owner leaf).
#   any other code  → the job fails with it (2 = usage).
#
# ⛔ WHY 20 DOES NOT BLOCK. Nothing a commit under test can change will close a gap another leaf owns, so a
# blocking 20 is a red build until that leaf lands — the red that people learn to ignore, with every real failure
# hidden inside it. What keeps a non-blocking 20 honest is not this script: it is `--provisioned`, which removes
# the one kind of absence the workflow itself could cause, and the runner refusing a stale quarantine.
#
# CONTRACT: exit 0 = the job passes; otherwise the code the job fails with. The tier's report is kept in
# `target/ci/integration.log`. `--self-test` runs the policy against a stub tier in `target/doctrine_scratch/`.
# Seams, for those arms only: `CI_INTEGRATION_TIER`, the command that runs the tier, and `CI_INTEGRATION_LOG`, where
# its report is kept. ⛔ The second is not optional, and was found by running the real job: the tier's own
# `self-tests` step runs these arms, and while they shared the real log they truncated it under the real run's
# `tee` — the log gained a hole of NUL bytes, `grep` read it as binary, and the one real gap went unnamed.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

TIER="${CI_INTEGRATION_TIER:-cargo xtask verify --tier integration --provisioned}"
LOG="${CI_INTEGRATION_LOG:-$ROOT/target/ci/integration.log}"

policy() {
  mkdir -p "$(dirname "$LOG")"
  # shellcheck disable=SC2086  # the tier command is a word list by design
  $TIER 2>&1 | tee "$LOG"
  local rc=${PIPESTATUS[0]} gaps
  case "$rc" in
    0)
      echo "ci-integration: passed"
      return 0
      ;;
    20)
      # Each owned gap, as the runner reports it, whitespace folded: `emulator 0.06s QUARANTINED — … leaf M2.8 …`.
      gaps="$(grep -E 'QUARANTINED — |NOT BUILT — ' "$LOG" | sed -E 's/^[[:space:]]*⚠[[:space:]]*//' | tr -s ' ')"
      [ -n "$gaps" ] || gaps="the report names no gap — read ${LOG#"$ROOT"/}"
      while IFS= read -r gap; do
        printf '::warning title=integration: incomplete, not a pass::%s\n' "$gap"
      done <<< "$gaps"
      if [ -n "${GITHUB_STEP_SUMMARY:-}" ]; then
        {
          echo "### \`integration\`: incomplete — not a pass"
          echo
          echo "Nothing failed. Each gap below is owned by the task-tree leaf it names (ROADMAP §14.3)."
          echo
          while IFS= read -r gap; do printf -- '- %s\n' "$gap"; done <<< "$gaps"
        } >> "$GITHUB_STEP_SUMMARY"
      fi
      echo "ci-integration: incomplete — the job passes under the recorded policy; each owned gap is annotated above"
      return 0
      ;;
    *)
      echo "ci-integration: the tier exited $rc — the job fails"
      return "$rc"
      ;;
  esac
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/ci_integration/selftest"
  local real="$ROOT/target/ci/integration.log" before after
  before="$(cksum < "$real" 2>/dev/null || echo absent)"
  stub() { # $1 = exit code, $2 = the report's gap line ("" for none)
    rm -rf "$work"; mkdir -p "$work"
    printf '#!/usr/bin/env bash\necho "tier: integration"\n[ -n "%s" ] && echo "  ⚠  %s"\nexit %s\n' "$2" "$2" "$1" > "$work/tier.sh"
    : > "$work/summary.md"
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, $4 = text the summary must carry ("" = none)
    local name="$1" want="$2" must="$3" summary="$4" out rc
    arms=$((arms + 1))
    out="$(CI_INTEGRATION_TIER="bash $work/tier.sh" CI_INTEGRATION_LOG="$work/integration.log" \
      GITHUB_STEP_SUMMARY="$work/summary.md" bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2; return
    fi
    if ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$summary" ] && ! grep -qF -- "$summary" "$work/summary.md"; then
      echo "SELF-TEST: $name — the job summary does not carry \`$summary\`" >&2; return
    fi
    if [ -z "$summary" ] && [ -s "$work/summary.md" ]; then
      echo "SELF-TEST: $name — wrote a job summary it had no reason to write" >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  stub 0 ""
  arm "a passed tier passes the job" 0 "ci-integration: passed" ""
  stub 1 ""
  arm "a failed tier fails the job" 1 "the tier exited 1 — the job fails" ""
  stub 20 "emulator             0.06s  QUARANTINED — could not be run; leaf M2.8 owns the gap"
  arm "an incomplete tier passes the job, annotated with its gap" 0 "::warning title=integration: incomplete, not a pass::emulator 0.06s QUARANTINED — could not be run; leaf M2.8 owns the gap" "leaf M2.8 owns the gap"
  stub 20 "board              NOT BUILT — tracked by leaf M5.1"
  arm "a step not built is an owned gap too, and is named" 0 "::warning title=integration: incomplete, not a pass::board NOT BUILT — tracked by leaf M5.1" "tracked by leaf M5.1"
  stub 20 ""
  arm "an incomplete report that names no gap still warns, pointing at the log" 0 "the report names no gap — read" "the report names no gap"
  stub 2 ""
  arm "a runner that would not start fails the job with its own code" 2 "the tier exited 2" ""
  rm -rf "$work"
  # These arms run inside the real job, from its `self-tests` step: they must not touch the log it is writing.
  arms=$((arms + 1)); after="$(cksum < "$real" 2>/dev/null || echo absent)"
  if [ "$before" = "$after" ]; then ok=$((ok + 1)); echo "  ✅ the arms leave the real job's log alone"
  else echo "SELF-TEST: the arms wrote ${real#"$ROOT"/}, the log of the real job that may be running them" >&2; fi
  echo "ci-integration self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi
policy
exit $?
