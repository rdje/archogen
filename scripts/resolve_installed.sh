#!/usr/bin/env bash
# scripts/resolve_installed.sh — `archogen resolve` run end to end from an install (leaf `M3.2.6.2`,
# docs/specs/adapters/decision_engine-adapters.md §10). The `resolve-installed` step of the `integration` tier.
#
# ⭐ WHAT IT PROVES. The front's tests hold `archogen resolve` against a fake checker, and the checker's tests hold it
# against plans the generator wrote in-process: nothing ran the two programs as a user does. This builds an install
# with `cargo xtask install` — the trust build's executables, and the manifest that vouches for them — and runs that
# install's `archogen resolve`, checked, over each description of the population:
#   - the plan written, the front launching the install's `archogen-plan-check`, which accepts it, its record in
#     `<plan>.check/`;
#   - for a plan through an engine adapter, the plan binding an instance and the command saying the plan is
#     conditional on what it owes (record §9);
#   - for a plan through the description's own providers, that install's `archogen build` too, and the package it writes
#     built offline and run, its trace saying what it is (leaf `M4.2.3.1`, decision_hosted-build.md §5).
# THE POPULATION: examples/s0-heartbeat/system.eadl, a plan through the description's own providers; and
# crates/eadl-resolve/tests/search/adapter-deadline-over-delay.eadl and adapter-epoch-extender.eadl, a plan through
# each kind of the shipped adapter catalog.
#
# CONTRACT: exit 0 every plan accepted · 1 a run fell short, named · 2 the install could not be built or holds no
# front, or a usage error. `--install <dir>` runs an install already built instead of building one. `--self-test`
# runs the RED arms against stub installs in target/doctrine_scratch/resolve_installed/.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

SCRATCH="$ROOT/target/doctrine_scratch/resolve_installed"
CASES=(
  "examples/s0-heartbeat/system.eadl|own"
  "crates/eadl-resolve/tests/search/adapter-deadline-over-delay.eadl|adapter"
  "crates/eadl-resolve/tests/search/adapter-epoch-extender.eadl|adapter"
)
note() { printf 'resolve-installed: %s\n' "$1" >&2; }

# Each case resolved, checked, by the install at $1, its plan written under $2.
run_cases() {
  local install="$1" out="$2" fail=0 entry path shape plan said rc
  for entry in "${CASES[@]}"; do
    path="${entry%%|*}"; shape="${entry##*|}"
    plan="$out/$(basename "$path" .eadl).plan"
    rm -rf "$plan" "$plan.check"
    said="$("$install/archogen" resolve "$path" --out "$plan" 2>&1)"; rc=$?
    if [ "$rc" -ne 0 ]; then
      note "$path: \`archogen resolve\` exited $rc, not 0"; printf '%s\n' "$said" | sed -n '1,6p' | sed 's/^/    /' >&2
      fail=1; continue
    fi
    if ! grep -qF 'checked by `archogen-plan-check`: accepted' <<< "$said"; then
      note "$path: the install's checker did not accept the plan"; fail=1; continue
    fi
    if [ ! -f "$plan" ] || [ ! -d "$plan.check" ]; then
      note "$path: the plan or its check record is missing"; fail=1; continue
    fi
    if [ "$shape" = adapter ]; then
      if ! grep -qF '(adapter ' "$plan"; then note "$path: the plan binds no engine adapter"; fail=1; continue; fi
      if ! grep -qF '  conditional: it binds engine adapters' <<< "$said"; then
        note "$path: the plan binds an engine adapter, and the command does not say it is conditional"; fail=1; continue
      fi
    fi
    if [ "$shape" = own ]; then
      local package="$out/$(basename "$path" .eadl).package" trace
      rm -rf "$package" "$package.building"
      said="$("$install/archogen" build "$path" --out "$package" 2>&1)"; rc=$?
      if [ "$rc" -ne 0 ]; then
        note "$path: \`archogen build\` exited $rc, not 0"; printf '%s\n' "$said" | sed -n '1,6p' | sed 's/^/    /' >&2
        fail=1; continue
      fi
      if ! trace="$(CARGO_TARGET_DIR="$package/target" cargo run --quiet --offline --manifest-path "$package/Cargo.toml" 2>&1)"; then
        note "$path: the package the install generated did not build and run"; printf '%s\n' "$trace" | sed -n '1,6p' | sed 's/^/    /' >&2
        fail=1; continue
      fi
      if ! grep -qF '; archogen-playground-trace/3' <<< "$trace" || ! grep -qF ' release ' <<< "$trace"; then
        note "$path: the package ran, and printed no trace of a release"; fail=1; continue
      fi
      echo "  ✅ $path: built by the install, and the package it generated ran"
    fi
    echo "  ✅ $path: resolved, and accepted by the install's checker"
  done
  return "$fail"
}

run() { # $1 = an install already built, or "" to build one
  local install="$1"
  if [ -z "$install" ]; then
    install="$SCRATCH/install"
    rm -rf "$install"; mkdir -p "$SCRATCH"
    if ! cargo xtask install --to "target/doctrine_scratch/resolve_installed/install" > "$SCRATCH/install.log" 2>&1; then
      note "\`cargo xtask install\` failed, its output in $SCRATCH/install.log:"; tail -n 5 "$SCRATCH/install.log" | sed 's/^/    /' >&2
      return 2
    fi
  fi
  if [ ! -x "$install/archogen" ]; then
    note "$install holds no \`archogen\` to run"
    return 2
  fi
  mkdir -p "$SCRATCH/plans"
  run_cases "$install" "$SCRATCH/plans" || return 1
  echo "resolve-installed: OK (${#CASES[@]} description(s) resolved from the install at $install, each plan accepted by its checker)"
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  # A stub install: `archogen resolve <path> --out <plan>` as the real front answers, but for what MODE spoils.
  stub() {
    rm -rf "$work"; mkdir -p "$work/install"
    cat > "$work/install/archogen" <<'SH'
#!/usr/bin/env bash
path="$2"; plan="$4"
if [ "$1" = build ]; then
  [ "${MODE:-}" = no-build ] && { echo "archogen: missing-fact: nothing was generated" >&2; exit 11; }
  mkdir -p "$plan/src"
  printf '[package]\nname = "stub"\nversion = "0.1.0"\nedition = "2021"\n\n[workspace]\n' > "$plan/Cargo.toml"
  case "${MODE:-}" in
    no-compile) echo 'not rust' > "$plan/src/main.rs" ;;
    no-trace) echo 'fn main() {}' > "$plan/src/main.rs" ;;
    *) printf 'fn main() {\n    println!("; archogen-playground-trace/3");\n    println!("0 s release t");\n}\n' > "$plan/src/main.rs" ;;
  esac
  exit 0
fi
case "$path" in *adapter-*) bind='(bind r f (adapter k p))' ;; *) bind='(bind r f p)' ;; esac
[ "${MODE:-}" = no-adapter ] && bind='(bind r f p)'
printf '%s\n' "$bind" > "$plan"
[ "${MODE:-}" = no-record ] || mkdir -p "$plan.check"
echo "$path: resolved system \`s\` against profile \`rt-static-up-v1\` into $plan (1 binding(s))"
case "$path" in *adapter-*) [ "${MODE:-}" = unconditional ] || echo "  conditional: it binds engine adapters, and holds once each obligation below is discharged and each cost charged" ;; esac
if [ "${MODE:-}" = refused ]; then echo "archogen: tool-failure: \`archogen-plan-check\` refuses the plan the search wrote" >&2; exit 1; fi
echo "  checked by \`archogen-plan-check\`: accepted ($plan.check/record)"
SH
    chmod +x "$work/install/archogen"
  }
  arm() { # $1 = name, $2 = MODE, $3 = expected rc, $4 = text the output must carry
    local name="$1" mode="$2" want="$3" must="$4" out rc
    arms=$((arms + 1))
    out="$(MODE="$mode" bash "$SELF" --install "$work/install" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2; return
    fi
    if ! grep -qF -- "$must" <<< "$out"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  stub; arm "every plan written, checked and accepted, an adapter's said conditional, passes" "" 0 "resolve-installed: OK (3 description(s)"
  stub; arm "a plan the install's checker refuses is a failure, naming the description" refused 1 "adapter-deadline-over-delay.eadl: \`archogen resolve\` exited 1, not 0"
  stub; arm "a plan with no check record is a failure" no-record 1 "the plan or its check record is missing"
  stub; arm "an adapter fixture whose plan binds no adapter is a failure" no-adapter 1 "the plan binds no engine adapter"
  stub; arm "a plan through an adapter not said to be conditional is a failure" unconditional 1 "does not say it is conditional"
  stub; arm "a build the install refuses is a failure" no-build 1 "\`archogen build\` exited 11"
  stub; arm "a generated package that does not compile is a failure" no-compile 1 "did not build and run"
  stub; arm "a generated package printing no trace is a failure" no-trace 1 "printed no trace of a release"
  stub; rm -f "$work/install/archogen"
  arm "an install holding no front is refused before anything runs" "" 2 "holds no \`archogen\` to run"
  rm -rf "$work"
  echo "resolve-installed self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  --install) [ "$#" -eq 2 ] || { echo "usage: bash scripts/resolve_installed.sh [--install <dir>] [--self-test]" >&2; exit 2; }; run "$2"; exit $? ;;
  '') run ""; exit $? ;;
  *) echo "usage: bash scripts/resolve_installed.sh [--install <dir>] [--self-test]" >&2; exit 2 ;;
esac
