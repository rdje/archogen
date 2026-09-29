#!/usr/bin/env bash
# scripts/extended_fuzz.sh — the `extended` tier's `fuzz` step (leaf `PROGRAM.9.2`).
#
# Runs `crates/eadl-model/tests/fuzz.rs`'s `fuzz_extended` twice: once on the fixed seed, so a regression of
# anything the harness has ever passed is caught, and once on a fresh seed, so every scheduled run explores
# inputs no earlier run saw. The seed is printed; any failure prints its own replay command.
#
# ⛔ OVERFLOW CHECKS ARE ON (`CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true`). Release speed is wanted — 100 000
# cases per property — but a release build WRAPS on overflow instead of panicking. Measured: the harness's
# first release run passed over `to_exact_string`'s unchecked power of ten, which panicked in a debug build
# and, in release, printed a wrong decimal (leaf `M1.36`).
#
# The harness arms itself: each of its arms is a claim known to be false, which the generator must refute
# within the budget, so a property cannot pass without reaching the inputs it exists for.
#
# CONTRACT: exit 0 = every arm refuted and every property held, on both seeds; 1 = otherwise, each failure
# with its replay line. `FUZZ_SEED` fixes the exploring seed; `FUZZ_CASES` the budget.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
export CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true
CASES="${FUZZ_CASES:-100000}"
FIXED=6840158756780179457
FRESH="${FUZZ_SEED:-$(date +%s)}"

failed=0
for seed in "$FIXED" "$FRESH"; do
  started=$SECONDS
  if out="$(FUZZ_SEED="$seed" FUZZ_CASES="$CASES" cargo test -q --release -p eadl-model --test fuzz -- \
      --ignored --nocapture 2>&1)"; then
    echo "  ✓ seed $seed ($((SECONDS - started))s): $(printf '%s\n' "$out" | grep -c '✓ arm') arm(s) refuted, $(printf '%s\n' "$out" | grep -c '✓ property') propert(ies) held over $CASES case(s) each"
  else
    failed=$((failed + 1))
    echo "  ✗ seed $seed:" >&2
    printf '%s\n' "$out" | grep -E "failed at case|replay:|never refuted|panicked at" | head -n 12 | sed 's/^/      /' >&2
  fi
done
if [ "$failed" -ne 0 ]; then
  echo "EXTENDED-FUZZ: $failed seed(s) failed" >&2
  exit 1
fi
echo "extended-fuzz: OK — the fixed seed and seed $FRESH, $CASES case(s) per property, overflow checks on"
exit 0
