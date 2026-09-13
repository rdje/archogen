#!/usr/bin/env bash
# target_emulator.sh — render, check, or run the PINNED emulator configuration.
#
# ⭐ WHY THIS IS A SCRIPT AND NOT A README PARAGRAPH. ROADMAP.md §3.2 requires the emulator
# configuration to be pinned and its produced hardware description compared against the eADL
# platform fixture — "do not rely on changing defaults". A configuration that lives in prose is
# retyped slightly differently by every caller, and the differences are exactly the ones that
# change the platform.
#
# ⛔ AND WHY IT REPORTS ABSENCE LOUDLY. §14.3: "A required tool skipped or unavailable is
# reported as such, not a passed check." `--check` exits nonzero when QEMU is missing and says
# what is missing. It never prints anything that could be mistaken for a verified target.
#
# USAGE
#   scripts/target_emulator.sh --print            the pinned invocation, for a given image
#   scripts/target_emulator.sh --check            is the toolchain present, and does it match?
#   scripts/target_emulator.sh --dump-dtb <out>   write the device tree QEMU generates
#   scripts/target_emulator.sh --run <image.elf>  run the pinned configuration
#
# EXIT CODES  0 ok · 2 usage · 20 required tool unavailable · 1 mismatch or failure
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
ENV_FILE="targets/riscv-virt-up.env"

die()  { printf 'target-emulator: %s\n' "$1" >&2; exit "${2:-1}"; }
note() { printf 'target-emulator: %s\n' "$1" >&2; }

[ -f "$ENV_FILE" ] || die "$ENV_FILE is missing — the pinned configuration is the contract" 1
# shellcheck source=/dev/null
set -a; . "$ENV_FILE"; set +a

invocation() {
  local image="${1:-<image.elf>}"
  printf '%s -machine %s -cpu %s -smp %s -m %s -bios %s %s -kernel %s' \
    "$QEMU_BINARY" "$QEMU_MACHINE" "$QEMU_CPU" "$QEMU_SMP" "$QEMU_MEMORY" \
    "$QEMU_BIOS" "$QEMU_DISPLAY" "$image"
}

case "${1:-}" in
  --print)
    invocation "${2:-}"; echo
    ;;

  --check)
    rc=0
    if ! command -v "$QEMU_BINARY" >/dev/null 2>&1; then
      note "required tool unavailable: $QEMU_BINARY is not on PATH"
      note "  the riscv-virt-up environment cannot be exercised on this machine"
      note "  install it, then re-run; do NOT record an emulator result without it"
      exit 20
    fi
    installed="$("$QEMU_BINARY" --version 2>/dev/null | head -1)"
    note "found: $installed"

    if [ "$QEMU_VERSION_PINNED" = "none-yet" ]; then
      note "NO RELEASE IS PINNED YET (QEMU_VERSION_PINNED=none-yet)"
      note "  pin the installed release in $ENV_FILE, then re-run --check"
      rc=1
    elif ! printf '%s' "$installed" | grep -qF "$QEMU_VERSION_PINNED"; then
      note "PINNED RELEASE MISMATCH: pinned '$QEMU_VERSION_PINNED', installed '$installed'"
      note "  a different release is a different virtual platform (§3.2)"
      rc=1
    fi

    # Does the installed QEMU actually offer the pinned machine?
    if ! "$QEMU_BINARY" -machine help 2>/dev/null | grep -qE "^${QEMU_MACHINE}[[:space:]]"; then
      note "machine '$QEMU_MACHINE' is not offered by the installed $QEMU_BINARY"
      rc=1
    fi

    if [ "$TARGET_VERIFIED" != "yes" ]; then
      note "TARGET_VERIFIED=$TARGET_VERIFIED — this configuration is still a PROPOSAL"
      note "  leaf $TARGET_VERIFIED_BY owns flipping it, with the evidence that justifies it"
      rc=1
    fi
    exit "$rc"
    ;;

  --dump-dtb)
    out="${2:-}"
    [ -n "$out" ] || die "--dump-dtb needs an output path" 2
    command -v "$QEMU_BINARY" >/dev/null 2>&1 \
      || die "required tool unavailable: $QEMU_BINARY is not on PATH" 20
    # QEMU writes the generated device tree and exits. This is the §3.2 agreement input: the
    # platform the emulator actually presents, not the one we assumed it would.
    "$QEMU_BINARY" -machine "$QEMU_MACHINE,$QEMU_DTB_DUMP_OPTION=$out" \
      -cpu "$QEMU_CPU" -smp "$QEMU_SMP" -m "$QEMU_MEMORY" -bios "$QEMU_BIOS" $QEMU_DISPLAY \
      >/dev/null 2>&1
    [ -s "$out" ] || die "no device tree was written to $out" 1
    note "device tree written to $out — compare it against $DEVICE_TREE_FIXTURE"
    ;;

  --run)
    image="${2:-}"
    [ -n "$image" ] || die "--run needs an image path" 2
    [ -f "$image" ] || die "no such image: $image" 2
    command -v "$QEMU_BINARY" >/dev/null 2>&1 \
      || die "required tool unavailable: $QEMU_BINARY is not on PATH" 20
    if [ "$TARGET_VERIFIED" != "yes" ]; then
      note "WARNING: TARGET_VERIFIED=$TARGET_VERIFIED — any observation from this run is"
      note "  provisional and must not be recorded as target evidence (§7.1)"
    fi
    # shellcheck disable=SC2086  # QEMU_DISPLAY is a pinned option string, deliberately split
    exec $(invocation "$image")
    ;;

  ""|-h|--help)
    sed -n '3,20p' "$0" | sed 's/^# \{0,1\}//'
    exit 0
    ;;

  *)
    die "unknown option '$1' — try --help" 2
    ;;
esac
