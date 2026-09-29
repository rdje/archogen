#!/usr/bin/env bash
# scripts/extended_miri.sh — the `extended` tier's `miri` step (leaf `PROGRAM.9.1`).
#
# ⭐ WHY A SCRIPT AND NOT A BARE `cargo miri test`. Two things a bare command cannot do:
#   1. FIND MIRI WHERE IT LIVES. Miri ships with the `nightly` toolchain; this repository pins `stable`
#      (`rust-toolchain.toml`), so `cargo miri` run here asks the wrong toolchain. Measured: the step reported
#      "UNAVAILABLE" on a machine where `cargo +nightly miri --version` answered `miri 0.1.0`.
#   2. ARM ITSELF. This workspace contains no `unsafe` code, so a Miri run over it passing is exactly what it
#      would do if Miri were not looking at all. Before the real run, a scratch crate under `target/` with one
#      test that reads through a dangling pointer must be refused by the same command, or the step fails:
#      an instrument that cannot fail proves nothing (`docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`).
#
# THE POPULATION is every workspace member, read from the root `Cargo.toml`, and every test target in it, minus
# the test targets left out below — each with its measurement. A crate or test target added tomorrow is in
# scope the day it is added, and either runs under Miri or fails here until someone decides, in writing, why
# it cannot.
#
# Isolation is disabled (`-Zmiri-disable-isolation`) because the reader's, the model's and the analysis's tests
# read their fixtures from `docs/`; with isolation on, Miri stops at the first `open`. Measured `2026-09-29`.
#
# ⚠️ HONEST LIMIT (§13.3): Miri checks the executions the tests drive, for the undefined behaviour it can
# detect. Passing does not establish soundness.
#
# CONTRACT: exit 0 = the arm fired and every crate passed; 1 = the arm did not fire, or a crate failed;
# 2 = could not run (no Miri on the nightly toolchain). `--arm-only` runs the seeded arm alone; `--all` runs
# every test target, the ones left out below included.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
TOOLCHAIN="nightly"
export MIRIFLAGS="${MIRIFLAGS:--Zmiri-disable-isolation}"
# ⛔ Miri builds a sysroot before its first run, and by default puts it in the user's cache directory
# (`~/Library/Caches/org.rust-lang.miri` on macOS) — off this volume, against the data-locality rule. Miri's
# README: "When invoking `cargo miri setup`, [`MIRI_SYSROOT`] indicates where the sysroot will be put", and
# `cargo miri test` then uses it. So the sysroot lives under `target/`, built by the setup below.
export MIRI_SYSROOT="$ROOT/target/miri-sysroot"

# Test targets left out of the default run, each with its measurement. ⭐ THE BUDGET: a test target is left out
# only when it was measured to take more than 300 s under Miri on the reference machine — every target was
# timed alone, `2026-09-29`, 600 s cap (34 targets; 541 tests passed, 4 ignored, none failed). Each one left
# out walks a whole corpus, so what it drives under Miri is the same reader and checker code the per-feature
# suites drive, many times over; the cost is the walk, not new code. `--all` runs them too.
# ⛔ Not a crate-level list: the first version excluded three whole crates on guesses ("its tests spawn
# processes"), and two were wrong — `archogen-s0` spawns nothing, `xtask`'s tests are data checks. The four
# tests that do reach a child process are ignored where they are written, with `#[cfg_attr(miri, ignore = …)]`.
EXCLUDE=(
  "archogen-cli/module_cases|walks every module case directory through the CLI: 8 tests, 463 s under Miri"
  "eadl-front/conformance|walks the whole conformance suite: 12 tests, 0.08 s natively, over 600 s under Miri in two runs"
  "eadl-front/corpus|walks every tracked description: 14 tests, 397 s under Miri"
  "eadl-front/reference|reads the reference and checks every table against the reader: 46 tests, over 600 s under Miri"
  "eadl-model/rendering|renders every diagnostic of the corpus: 2 tests, 473 s under Miri"
)

if ! cargo +"$TOOLCHAIN" miri --version >/dev/null 2>&1; then
  echo "extended-miri: no Miri on the \`$TOOLCHAIN\` toolchain — \`rustup +$TOOLCHAIN component add miri\`" >&2
  exit 2
fi

if ! setup_out="$(cargo +"$TOOLCHAIN" miri setup 2>&1)"; then
  echo "EXTENDED-MIRI: \`cargo miri setup\` could not build the sysroot in $MIRI_SYSROOT" >&2
  printf '%s\n' "$setup_out" | tail -n 5 | sed 's/^/    /' >&2
  exit 1
fi

# ── the arm: Miri must refuse a seeded read through a dangling pointer ───────────────────────────────────────
arm() {
  local work="$ROOT/target/doctrine_scratch/extended_miri/arm" out rc
  rm -rf "$work"; mkdir -p "$work/src"
  printf '[package]\nname = "miri-arm"\nversion = "0.0.0"\nedition = "2021"\n\n[workspace]\n' > "$work/Cargo.toml"
  cat > "$work/src/lib.rs" <<'RS'
#[test]
fn reads_through_a_dangling_pointer() {
    let p = {
        let v = vec![7u8; 4];
        v.as_ptr()
    };
    // The vector is dropped: this read is undefined behaviour, and nothing but Miri will notice.
    let x = unsafe { *p };
    assert!(x == 7 || x != 7);
}
RS
  out="$(cd "$work" && CARGO_TARGET_DIR="$ROOT/target/doctrine_scratch/extended_miri/target" \
    cargo +"$TOOLCHAIN" miri test -q 2>&1)"; rc=$?
  rm -rf "$work"
  if [ "$rc" -ne 0 ] && printf '%s' "$out" | grep -q "Undefined Behavior"; then
    echo "  ✓ arm: Miri refused the seeded dangling-pointer read"
    return 0
  fi
  echo "EXTENDED-MIRI: the seeded dangling-pointer read was NOT refused (exit $rc) — this wiring cannot fail, so its pass would mean nothing" >&2
  printf '%s\n' "$out" | tail -n 5 | sed 's/^/    /' >&2
  return 1
}
arm || exit 1
[ "${1:-}" = "--arm-only" ] && exit 0

# ── the real run ─────────────────────────────────────────────────────────────────────────────────────────────
# Every member the root manifest declares (globs expanded), as `name<TAB>directory` — the derivation
# `scripts/check_book_coverage.sh` uses.
members="$(awk '/^members[[:space:]]*=/{f=1} f{print} f&&/\]/{exit}' Cargo.toml | tr -d '[]"' |
  sed 's/^members[[:space:]]*=//' | tr ',' '\n' | while IFS= read -r entry; do
    entry="$(printf '%s' "$entry" | tr -d '[:space:]')"; [ -n "$entry" ] || continue
    for dir in $entry; do
      [ -f "$dir/Cargo.toml" ] || continue
      printf '%s\t%s\n' "$(grep -m1 -E '^name[[:space:]]*=' "$dir/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/')" "${dir%/}"
    done
  done | sort -u)"
[ -n "$members" ] || { echo "EXTENDED-MIRI: the root Cargo.toml declares no workspace member this step could read" >&2; exit 1; }

# The reason `crate/test-target` is left out, or failure when it is not.
left_out() { local e; for e in ${EXCLUDE[@]+"${EXCLUDE[@]}"}; do [ "${e%%|*}" = "$1" ] && { printf '%s' "${e#*|}"; return 0; }; done; return 1; }

# An exclusion that names no test target of any member is stale: remove it rather than let it hide one later.
# Checked before the run, so a stale list fails in seconds rather than after it.
stale=0
for e in ${EXCLUDE[@]+"${EXCLUDE[@]}"}; do
  key="${e%%|*}"; crate="${key%%/*}"; name="${key#*/}"
  dir="$(printf '%s\n' "$members" | awk -F'\t' -v c="$crate" '$1 == c { print $2 }')"
  [ -n "$dir" ] && [ -f "$dir/tests/$name.rs" ] ||
    { echo "EXTENDED-MIRI: the exclusion \`$key\` names no test target of a workspace member — remove it" >&2; stale=$((stale + 1)); }
done
[ "$stale" -eq 0 ] || exit 1

ALL=0; [ "${1:-}" = "--all" ] && ALL=1
failed=0; crates=0; tests=0; skipped=0
while IFS=$'\t' read -r crate dir; do
  [ -n "$crate" ] || continue
  # A crate with a test target left out is run target by target: its library, its binaries, and every test
  # target that is not left out. Any other crate is run whole.
  sel=(); partial=0; kept=()
  for t in "$dir"/tests/*.rs; do
    [ -f "$t" ] || continue
    name="$(basename "$t" .rs)"
    if [ "$ALL" -eq 0 ] && reason="$(left_out "$crate/$name")"; then
      echo "  – $crate/$name: not run under Miri — $reason"; partial=1; skipped=$((skipped + 1))
    else
      kept+=(--test "$name")
    fi
  done
  if [ "$partial" -eq 1 ]; then
    [ -f "$dir/src/lib.rs" ] && sel+=(--lib)
    { [ -f "$dir/src/main.rs" ] || [ -d "$dir/src/bin" ]; } && sel+=(--bins)
    sel+=(${kept[@]+"${kept[@]}"})
  fi
  started=$SECONDS
  if out="$(cargo +"$TOOLCHAIN" miri test -q -p "$crate" ${sel[@]+"${sel[@]}"} 2>&1)"; then
    crates=$((crates + 1))
    n="$(printf '%s\n' "$out" | grep -E '^test result' | awk '{p+=$4} END{print p+0}')"; tests=$((tests + n))
    echo "  ✓ $crate ($((SECONDS - started))s): $n test(s) under Miri"
  else
    failed=$((failed + 1))
    echo "  ✗ $crate ($((SECONDS - started))s):" >&2
    printf '%s\n' "$out" | grep -E 'Undefined Behavior|unsupported operation|panicked at|error' | head -n 6 | sed 's/^/      /' >&2
  fi
done <<< "$members"

if [ "$failed" -ne 0 ]; then
  echo "EXTENDED-MIRI: $failed failure(s)" >&2
  exit 1
fi
echo "extended-miri: OK — the arm fired; $tests test(s) in $crates crate(s) passed under Miri; $skipped test target(s) left out, each named above with its measured reason"
exit 0
