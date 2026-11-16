#!/usr/bin/env bash
# scripts/plan_check_wasm.sh — the configuration checker's browser module checked as an artifact (leaf `M3.5.7.2`,
# docs/decisions/decision_wasm-binding.md §11). The `plan-check-wasm` step of the `integration` tier.
#
# ⭐ WHAT IT PROVES. The module's judgement is the checker's, which the checker's own suites test, and its answer on
# the host is held to the executable's and the MCP tool's by crates/archogen-plan-check-wasm/tests/module.rs. What is
# new here is the compilation to WebAssembly, the loader and the digest it computes, so this checks exactly those, on
# the artifact a page loads:
#   1. the artifact imports nothing, and exports exactly §11's three functions, its memory and the linker's two
#      globals — read by the platform's own `WebAssembly.Module`, not from the source;
#   2. for every case of the population, the artifact's answer, reached through
#      crates/archogen-plan-check-wasm/js/plan-check.mjs as a page reaches it, is byte-identical to the host build's
#      (`cargo run -p archogen-plan-check-wasm --example answers`), the producer's digest the loader's;
#   3. for every case, the answer agrees with `archogen-plan-check` run over the case's files: the same record but for
#      its producer, with the executable's exit code, or a refusal where the executable refused the call. One text
#      is set aside and counted: a `module-not-found` finding's repair, which names the file the executable looked
#      for, where the module, given texts, has none to name.
# THE POPULATION is derived on every run, since no plan is tracked: every tracked `*.eadl` outside `docs/feedback/`
# (the vendors' own reproduction inputs) and `docs/evaluation/` (the sealed set's place, never read), each with its
# empty plan and its bare one, and the accepted plans the checker's own tests write (the example's comment).
#
# CONTRACT: exit 0 all three hold · 1 a leg failed, naming the case · 2 a build or usage failure, a toolchain without
# the `wasm32-unknown-unknown` target included · 20 unavailable: no `node`, which is never a pass. `--self-test` runs
# the RED arms in target/doctrine_scratch/plan_check_wasm/.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

SCRATCH="$ROOT/target/doctrine_scratch/plan_check_wasm"
WASM="$ROOT/target/wasm32-unknown-unknown/release/archogen_plan_check_wasm.wasm"
EXE="$ROOT/target/debug/archogen-plan-check"
note() { printf 'plan-check-wasm: %s\n' "$1" >&2; }

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  rm -rf "$work"; mkdir -p "$work"
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, then the command
    local name="$1" want="$2" must="$3" out rc; shift 3
    arms=$((arms + 1))
    out="$("$@" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! grep -qF -- "$must" <<< "$out"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  if ! command -v node >/dev/null 2>&1; then
    echo "SELF-TEST: every arm needs node, and node is not on PATH" >&2
    echo "plan-check-wasm self-test: 0 pass / 1 fail (node absent)"
    return 1
  fi
  local compare=(node "$ROOT/scripts/plan_check_wasm.mjs" compare)
  local record='{"format":"archogen-result/1","producer":{"name":"archogen-plan-check-wasm","sha256":"sha256:aa"},"status":"ok","exit":0,"findings":[]}'
  local executed='{"format":"archogen-result/1","producer":{"name":"archogen-plan-check","sha256":"sha256:bb"},"status":"ok","exit":0,"findings":[]}'
  local refusal='{"format":"archogen-plan-check-wasm-refusal/1","status":"usage","exit":2,"note":"another profile"}'
  printf '%s\n%s\n' '{"case":"a, the empty plan"}' '{"case":"b, the bare plan"}' > "$work/cases"
  printf '%s\n%s\n' "$record" "$refusal" > "$work/host"
  cp "$work/host" "$work/wasm"
  printf '0\t%s\n2\t\n' "$executed" > "$work/exec"
  arm "sides that agree on every case pass, the producer apart" 0 "2 case(s), 0 with a module not found" \
    "${compare[@]}" "$work/cases" "$work/host" "$work/wasm" "$work/exec"
  printf '%s\n%s\n' "${record/sha256:aa/sha256:cc}" "$refusal" > "$work/wasm-differs"
  arm "an answer that differs from the host build's is refused, naming the case" 1 \
    "a, the empty plan: the artifact's answer differs from the host build's" \
    "${compare[@]}" "$work/cases" "$work/host" "$work/wasm-differs" "$work/exec"
  printf '0\t%s\n2\t\n' "${executed/\"findings\":[]/\"findings\":[{}]}" > "$work/exec-findings"
  arm "a record that differs from the executable's beyond its producer is refused" 1 \
    "a, the empty plan: the artifact's record differs from the executable's beyond its producer" \
    "${compare[@]}" "$work/cases" "$work/host" "$work/wasm" "$work/exec-findings"
  printf '3\t%s\n2\t\n' "$executed" > "$work/exec-code"
  arm "an exit code that differs from the record's is refused" 1 \
    "the artifact's record says exit 0, and the executable exits 3" \
    "${compare[@]}" "$work/cases" "$work/host" "$work/wasm" "$work/exec-code"
  printf '0\t%s\n0\t%s\n' "$executed" "$executed" > "$work/exec-judged"
  arm "a refusal where the executable judged is refused" 1 \
    "b, the bare plan: the artifact refused it (another profile), and the executable exited 0" \
    "${compare[@]}" "$work/cases" "$work/host" "$work/wasm" "$work/exec-judged"
  printf '%s\n%s\n' "${record/archogen-plan-check-wasm/archogen-plan-check}" "$refusal" > "$work/host-named"
  cp "$work/host-named" "$work/wasm-named"
  arm "a record naming another producer is refused" 1 "names \`archogen-plan-check\` as its producer" \
    "${compare[@]}" "$work/cases" "$work/host-named" "$work/wasm-named" "$work/exec"
  local missing='{"code":"module-not-found","message":"module `m` was not found","repair":"REPAIR"}'
  local other='{"code":"plan-record","message":"m","repair":"REPAIR"}'
  printf '%s\n%s\n' "${record/\"findings\":[]/\"findings\":[${missing/REPAIR/no module named m}]}" "$refusal" > "$work/host-missing"
  cp "$work/host-missing" "$work/wasm-missing"
  printf '0\t%s\n2\t\n' "${executed/\"findings\":[]/\"findings\":[${missing/REPAIR/no file m.eadl}]}" > "$work/exec-missing"
  arm "a module not found, whose repair names the executable's file, is set aside and counted" 0 \
    "2 case(s), 1 with a module not found" \
    "${compare[@]}" "$work/cases" "$work/host-missing" "$work/wasm-missing" "$work/exec-missing"
  printf '%s\n%s\n' "${record/\"findings\":[]/\"findings\":[${other/REPAIR/one}]}" "$refusal" > "$work/host-other"
  cp "$work/host-other" "$work/wasm-other"
  printf '0\t%s\n2\t\n' "${executed/\"findings\":[]/\"findings\":[${other/REPAIR/another}]}" > "$work/exec-other"
  arm "another finding's repair is compared" 1 "the artifact's record differs from the executable's beyond its producer" \
    "${compare[@]}" "$work/cases" "$work/host-other" "$work/wasm-other" "$work/exec-other"
  head -1 "$work/wasm" > "$work/wasm-short"
  arm "a case the artifact did not answer is refused" 1 "b, the bare plan: the artifact gave no answer" \
    "${compare[@]}" "$work/cases" "$work/host" "$work/wasm-short" "$work/exec"
  # Minimal modules written byte by byte: the magic and version, a type section with one `() -> ()`, then
  #   an import of `env.f` (section 2), or
  #   one function (section 3) exported as `x` (section 7) with an empty body (section 10).
  printf '\000asm\001\000\000\000\001\004\001\140\000\000\002\011\001\003env\001f\000\000' > "$work/imports.wasm"
  printf '\000asm\001\000\000\000\001\004\001\140\000\000\003\002\001\000\007\005\001\001x\000\000\012\004\001\002\000\013' \
    > "$work/exports.wasm"
  arm "a module that imports anything is refused" 1 "imports function \`env.f\`" \
    node "$ROOT/scripts/plan_check_wasm.mjs" inspect "$work/imports.wasm"
  arm "a module that exports what the record does not list, and not what it does, is refused" 1 \
    "exports function \`x\`, which the record does not list" node "$ROOT/scripts/plan_check_wasm.mjs" inspect "$work/exports.wasm"
  arm "a module missing an export the record lists is refused" 1 \
    "does not export the function \`plan_check\`" node "$ROOT/scripts/plan_check_wasm.mjs" inspect "$work/exports.wasm"
  rm -rf "$work"
  echo "plan-check-wasm self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") ;;
  *) echo "usage: bash scripts/plan_check_wasm.sh [--self-test]" >&2; exit 2 ;;
esac

command -v node >/dev/null 2>&1 || { note "UNAVAILABLE — node is not on PATH, so the artifact cannot be run"; exit 20; }
# The target is not optional: `rust-toolchain.toml` lists it, so a toolchain without it is a broken install.
installed="$(rustup target list --installed 2>/dev/null)"
grep -qx wasm32-unknown-unknown <<< "$installed" ||
  { note "the wasm32-unknown-unknown target is not installed, and rust-toolchain.toml lists it"; exit 2; }

rm -rf "$SCRATCH/cases"; mkdir -p "$SCRATCH/cases"
cargo build -q --release -p archogen-plan-check-wasm --target wasm32-unknown-unknown ||
  { note "the artifact did not build"; exit 2; }
cargo build -q -p archogen-plan-check || { note "archogen-plan-check did not build"; exit 2; }
git ls-files -z '*.eadl' | tr '\0' '\n' | grep -v -e '^docs/feedback/' -e '^docs/evaluation/' > "$SCRATCH/list"
[ -s "$SCRATCH/list" ] || { note "the population is empty — nothing tracked to derive it from"; exit 1; }

if ! problems="$(node scripts/plan_check_wasm.mjs inspect "$WASM" 2>&1)"; then
  printf '%s\n' "$problems" >&2; exit 1
fi
# shellcheck disable=SC2046
cargo run -q -p archogen-plan-check-wasm --example answers -- --module "$WASM" --out "$SCRATCH/cases" \
  $(cat "$SCRATCH/list") > "$SCRATCH/host" || { note "the host build could not answer"; exit 2; }
node scripts/plan_check_wasm.mjs answer "$WASM" "$SCRATCH/cases/cases.jsonl" > "$SCRATCH/wasm" ||
  { note "the artifact could not be driven through the loader"; exit 1; }
node scripts/plan_check_wasm.mjs dirs "$SCRATCH/cases/cases.jsonl" > "$SCRATCH/dirs" || exit 2
: > "$SCRATCH/exec"
while IFS=$'\t' read -r dir name; do
  output="$(cd "$dir" && "$EXE" "$name" plan.json 2>/dev/null)"; code=$?
  printf '%s\t%s\n' "$code" "$output" >> "$SCRATCH/exec"
done < "$SCRATCH/dirs"

if ! result="$(node scripts/plan_check_wasm.mjs compare "$SCRATCH/cases/cases.jsonl" "$SCRATCH/host" "$SCRATCH/wasm" "$SCRATCH/exec")"; then
  printf '%s\n' "$result" | sed 's/^/plan-check-wasm: /' >&2
  note "the artifact disagrees with the host build or the executable"
  exit 1
fi
echo "plan-check-wasm: OK — the artifact imports nothing and exports what the record lists; $result answered byte for byte as the host build answers them, each agreeing with archogen-plan-check over its files ($(node --version))"
exit 0
