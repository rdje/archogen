#!/usr/bin/env bash
# scripts/run_self_tests.sh — run every gate's own RED arms, discovered by census (leaf `PROGRAM.28`).
#
# ⭐ WHY THIS EXISTS. A gate's `--self-test` proves the gate can fail — but only when someone runs it, and until
# `PROGRAM.28` nothing did: no tier, no CI workflow, not the doctrine driver. An arm that breaks, or starts
# passing for the wrong reason, stayed unseen until a person happened to invoke it. This runner is the
# `self-tests` step of the `integration` tier and of the CI doctrine workflow.
#
# THE POPULATION is discovered, never listed: every `scripts/*.sh` and `knowledge-map/scripts/*.sh` that handles
# `--self-test` — the gates, the tools that classify (`target_emulator.sh`, `PROGRAM.10.1`) and this runner's own
# arms — plus `scripts/selftest_spine.sh`, which arms the scaffold-owned gates from outside (`PROGRAM.18.2`). A
# script armed tomorrow is run the day it is armed. An empty population is a breach, because a runner with
# nothing to run reports success about nothing.
#
# ⭐ AS A RUNNER RUNS THEM (`PROGRAM.10.2`). A CI runner has no git identity, no global ignore file, no signing
# key — nothing this machine's `~/.gitconfig` supplies. Every self-test runs with the global and system git
# configuration hidden and identity auto-detection off, so an arm that leans on the developer's machine fails
# here first rather than on the runner. Measured when it was added: all 23 passed under it — each scratch
# repository that commits sets its own identity — so this guards the next arm, not a known one.
#
# CONTRACT: exit 0 = every self-test passed; 1 = at least one failed, each named; 2 = nothing to run.
# `--self-test` runs this runner's own arms against stub gates in `target/doctrine_scratch/`.
# Seams, for those arms only: `SELF_TESTS_GLOB` (where gates are looked for) and `SELF_TESTS_HARNESS` (the
# outside harness).
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

GLOB="${SELF_TESTS_GLOB:-scripts/*.sh knowledge-map/scripts/*.sh}"
HARNESS="${SELF_TESTS_HARNESS-scripts/selftest_spine.sh}"

# Run a command as a bare runner would: no global or system git configuration, no identity it did not set.
bare() {
  env -u GIT_AUTHOR_NAME -u GIT_AUTHOR_EMAIL -u GIT_COMMITTER_NAME -u GIT_COMMITTER_EMAIL \
    GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null \
    GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=user.useConfigOnly GIT_CONFIG_VALUE_0=true "$@"
}

# Every armed gate, one per line: a script that handles the `--self-test` argument.
armed() {
  local f
  for f in $GLOB; do
    [ -f "$f" ] || continue
    grep -q -- '--self-test' "$f" && printf '%s\n' "$f"
  done
  [ -n "$HARNESS" ] && [ -f "$HARNESS" ] && printf '%s\n' "$HARNESS"
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/run_self_tests/selftest"
  stub() { # $1 = path, $2 = exit code of its self-test
    mkdir -p "$(dirname "$1")"
    printf '#!/usr/bin/env bash\n[ "${1:-}" = "--self-test" ] || exit 0\necho "stub self-test"\nexit %s\n' "$2" > "$1"
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(SELF_TESTS_GLOB="$work/check_*.sh" SELF_TESTS_HARNESS="${ARM_HARNESS:-}" bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  rm -rf "$work"; mkdir -p "$work"
  stub "$work/check_alpha.sh" 0; stub "$work/check_beta.sh" 0
  arm "every armed gate passing is a pass, and each is named" 0 "check_beta.sh"
  stub "$work/check_beta.sh" 1
  arm "one armed gate whose arms fail is a failure, named" 1 "✗ $work/check_beta.sh"
  stub "$work/check_beta.sh" 0; printf '#!/usr/bin/env bash\nexit 1\n' > "$work/check_gamma.sh"
  arm "a gate with no self-test is not run — it has nothing to run" 0 "2 self-test(s) passed"
  stub "$work/check_delta.sh" 0
  arm "a gate armed later is discovered without editing the runner" 0 "check_delta.sh"
  ARM_HARNESS="$work/harness.sh"; stub "$work/harness.sh" 1
  arm "the outside harness is run too, and its failure counts" 1 "✗ $work/harness.sh"
  ARM_HARNESS=""; rm -f "$work"/check_*.sh
  arm "nothing to run is not a pass" 2 "no gate carries a --self-test"
  # An arm that commits with whatever identity this machine has: it passes in a developer's shell, and must not
  # pass here, because a runner has none.
  printf '#!/usr/bin/env bash\n[ "${1:-}" = "--self-test" ] || exit 0\nd="%s/ident"; rm -rf "$d"; git init -q "$d" 2>/dev/null && git -C "$d" commit -q --allow-empty -m x\n' "$work" > "$work/check_ident.sh"
  arm "an arm leaning on this machine's git identity fails, as it would on a runner" 1 "✗ $work/check_ident.sh"
  printf '#!/usr/bin/env bash\n[ "${1:-}" = "--self-test" ] || exit 0\nd="%s/ident"; rm -rf "$d"; git init -q "$d" 2>/dev/null && git -C "$d" -c user.name=arm -c user.email=arm@example.invalid commit -q --allow-empty -m x\n' "$work" > "$work/check_ident.sh"
  arm "an arm that sets its own identity passes" 0 "check_ident.sh"
  rm -rf "$work"
  echo "run-self-tests self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

gates="$(armed)"
if [ -z "$gates" ]; then
  echo "SELF-TESTS: no gate carries a --self-test under '$GLOB' — nothing ran, which is not a pass" >&2
  exit 2
fi
passed=0; failed=0
while IFS= read -r gate; do
  started=$SECONDS
  if out="$(bare bash "$gate" --self-test 2>&1)"; then
    passed=$((passed + 1))
    printf '  ✓ %-52s %3ss  %s\n' "$gate" "$((SECONDS - started))" "$(printf '%s\n' "$out" | tail -1)"
  else
    failed=$((failed + 1))
    printf '  ✗ %-52s %3ss  its RED arms did not all pass:\n' "$gate" "$((SECONDS - started))" >&2
    printf '%s\n' "$out" | grep -E '^SELF-TEST|self-test' | head -8 | sed 's/^/      /' >&2
  fi
done <<< "$gates"
if [ "$failed" -ne 0 ]; then
  echo "SELF-TESTS: $failed of $((passed + failed)) self-test(s) failed — a gate that cannot show it fails is a gate nobody knows works" >&2
  exit 1
fi
echo "self-tests: OK — $passed self-test(s) passed"
exit 0
