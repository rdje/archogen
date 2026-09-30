#!/usr/bin/env bash
# scripts/target_spike.sh — build and run the `riscv-virt-up` architecture spike under the pinned QEMU (leaf `M2.8.4`).
#
# ⭐ WHAT IT PROVES. `M2.8`'s spike: startup, a machine-timer interrupt taken and returned from, the interrupted context
# preserved in 23 registers, and output on the UART the device tree names. Each is a line the image prints. The run
# passes only when every line appears, in order, and the image powers the machine off with status 0.
#
# ⛔ AND THAT IT CAN FAIL. The same image built with its `clobber` feature corrupts one register on the way out of the
# trap. That build must report `context CLOBBERED` and power off with status 2. A context check that never saw a
# clobbered context would pass for a handler that restored nothing.
#
# Every QEMU option and the Rust target come from `targets/riscv-virt-up.env`, as `target_emulator.sh`'s do.
#
# CONTRACT: exit 0 = the spike ran and the negative control was caught; 1 = either did not; 20 = QEMU is not on PATH.
# `--self-test` runs the checking logic against stub runs in `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
ENV_FILE="targets/riscv-virt-up.env"
SPIKE="targets/riscv-virt-up/spike"
EXPECTED=(
  "spike: boot on riscv-virt-up"
  "spike: timer armed"
  "spike: machine-timer interrupt taken"
  "spike: interrupt returned"
  "spike: context preserved in 23 registers"
  "spike: ok"
)

# $1 = output, $2 = exit status → 0 when every expected line appears in order and the status is 0
verdict_ok() {
  local out="$1" status="$2" line at=0 rest="$1"
  for line in "${EXPECTED[@]}"; do
    case "$rest" in
      *"$line"*) rest="${rest#*"$line"}" ;;
      *) echo "target-spike: missing, or out of order: \`$line\`" >&2; at=1 ;;
    esac
  done
  if [ "$status" != 0 ]; then echo "target-spike: the image powered off with status $status, not 0" >&2; at=1; fi
  [ "$at" -eq 0 ]
}

# $1 = output, $2 = exit status → 0 when the negative control was caught
verdict_caught() {
  case "$1" in *"spike: context CLOBBERED"*) ;; *) echo "target-spike: the clobbered build did not report it" >&2; return 1 ;; esac
  [ "$2" = 2 ] || { echo "target-spike: the clobbered build powered off with status $2, not 2" >&2; return 1; }
}

self_test() {
  local arms=0 ok=0
  arm() { arms=$((arms + 1)); if "$2" "$3" "$4" 2>/dev/null; then got=0; else got=1; fi
    if [ "$got" = "$5" ]; then ok=$((ok + 1)); echo "  ✅ $1"; else echo "SELF-TEST: $1 — got $got, expected $5" >&2; fi; }
  local good; good="$(printf '%s\n' "${EXPECTED[@]}")"
  arm "every line, in order, with status 0, passes" verdict_ok "$good" 0 0
  arm "a missing line fails" verdict_ok "$(printf '%s\n' "${EXPECTED[@]}" | grep -v 'context preserved')" 0 1
  arm "lines out of order fail" verdict_ok "$(printf '%s\n' "${EXPECTED[@]}" | sed '1{h;d};2{G}')" 0 1
  arm "a nonzero status fails even with every line" verdict_ok "$good" 3 1
  arm "the negative control caught passes" verdict_caught "spike: context CLOBBERED" 2 0
  arm "a negative control that reports nothing fails" verdict_caught "spike: ok" 0 1
  arm "a negative control with the wrong status fails" verdict_caught "spike: context CLOBBERED" 0 1
  echo "target-spike self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") ;;
  *) echo "target-spike: unknown argument '$1' — none, or --self-test" >&2; exit 2 ;;
esac

set -a; . "$ENV_FILE"; set +a
command -v "$QEMU_BINARY" >/dev/null 2>&1 || { echo "target-spike: $QEMU_BINARY is not on PATH" >&2; exit 20; }
export CARGO_TARGET_DIR="$ROOT/target/spike"
run() { # $1 = extra cargo arguments → prints the run's output, returns its status
  ( cd "$SPIKE" && RUSTFLAGS="-C link-arg=-T$ROOT/$SPIKE/link.x" cargo build -q --release --target "$RUST_TARGET" $1 ) \
    || { echo "target-spike: the spike does not build" >&2; return 99; }
  # A hang is a failure, never a pass: ten seconds, then SIGALRM.
  # shellcheck disable=SC2086
  perl -e 'alarm 10; exec @ARGV' "$QEMU_BINARY" -machine "$QEMU_MACHINE" -cpu "$QEMU_CPU" -smp "$QEMU_SMP" \
    -m "$QEMU_MEMORY" -bios "$QEMU_BIOS" $QEMU_DISPLAY -kernel "$CARGO_TARGET_DIR/$RUST_TARGET/release/spike"
}
rc=0
out="$(run "")"; status=$?
printf '%s\n' "$out" | sed 's/^/target-spike: /'
verdict_ok "$out" "$status" || rc=1
control="$(run "--features clobber")"; status=$?
if verdict_caught "$control" "$status"; then
  echo "target-spike: the negative control was caught: a clobbered register reports \`context CLOBBERED\`, status 2"
else
  rc=1
fi
# Leave the tree's spike as the real one, not the control.
( cd "$SPIKE" && RUSTFLAGS="-C link-arg=-T$ROOT/$SPIKE/link.x" cargo build -q --release --target "$RUST_TARGET" ) || rc=1
exit "$rc"
