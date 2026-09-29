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
#   scripts/target_emulator.sh --self-test        the --check verdicts, against a stub QEMU
#
# EXIT CODES  0 ok · 2 usage · 1 mismatch or failure · 20 could not be run: the tool is
#   unavailable, or it matches the pin and the §3.2 agreement it needs is not built yet
#   (`TARGET_VERIFIED=no`). A mismatch outranks the second — it is found, not missing (PROGRAM.10.1).
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
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

# The `--check` verdicts, each against a scratch repository and a stub QEMU — the logic that
# classifies, not the emulator, which the real `--check` exercises. A stub that answers like the
# pinned release lets every arm run on a machine with no QEMU at all.
self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/target_emulator/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/targets" "$work/bin"
    git -C "$work" init -q
    cp "$ROOT/$ENV_FILE" "$work/$ENV_FILE"
    cat > "$work/bin/qemu-system-riscv64" <<'STUB'
#!/usr/bin/env bash
case "$*" in
  --version) echo "QEMU emulator version 11.1.1" ;;
  "-machine help") printf 'Supported machines are:\nvirt                 RISC-V VirtIO board\n' ;;
esac
STUB
    chmod +x "$work/bin/qemu-system-riscv64"
  }
  set_env() { sed -i.bak "s|^$1=.*|$1=$2|" "$work/$ENV_FILE"; rm -f "$work/$ENV_FILE.bak"; }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && PATH="$work/bin:$PATH" bash "$SELF" --check 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2; return
    fi
    if ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh
  arm "the pinned release, its machine offered, the agreement unbuilt: could not be run" 20 "could not be run: $DEVICE_TREE_FIXTURE does not exist"
  fresh; set_env QEMU_VERSION_PINNED 11.1.0
  arm "another release is a mismatch, and outranks the unbuilt agreement" 1 "PINNED RELEASE MISMATCH"
  fresh; set_env QEMU_MACHINE sifive_u
  arm "a machine the emulator does not offer is a mismatch" 1 "machine 'sifive_u' is not offered"
  fresh; set_env QEMU_VERSION_PINNED none-yet
  arm "no pin at all is a mismatch, not an absence" 1 "NO RELEASE IS PINNED YET"
  fresh; set_env TARGET_VERIFIED yes
  arm "verified with no fixture behind it is refused" 1 "a verification with nothing behind it"
  fresh; set_env TARGET_VERIFIED yes; mkdir -p "$work/docs/targets"; : > "$work/$DEVICE_TREE_FIXTURE"
  arm "verified, pinned, offered and with its fixture: passes" 0 "found: QEMU emulator version 11.1.1"
  fresh; set_env QEMU_BINARY qemu-system-nowhere
  arm "an emulator that is not installed could not be run" 20 "required tool unavailable: qemu-system-nowhere"
  rm -rf "$work"
  echo "target-emulator self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
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

    # ⭐ Two different reasons not to say "verified", kept apart (PROGRAM.10.1). Everything above
    # is a comparison that RAN — a disagreement is a failure. What follows is a comparison that
    # cannot run yet, because its inputs are owned by a leaf that has not delivered them: §14.3
    # calls that a quarantine, and the runner accepts exit 20 for it only from this step.
    if [ "$TARGET_VERIFIED" != "yes" ]; then
      note "TARGET_VERIFIED=$TARGET_VERIFIED — this configuration is still a PROPOSAL"
      [ -f "$DEVICE_TREE_FIXTURE" ] ||
        note "  the §3.2 agreement check could not be run: $DEVICE_TREE_FIXTURE does not exist"
      note "  leaf $TARGET_VERIFIED_BY owns flipping it, with the evidence that justifies it"
      [ "$rc" -eq 0 ] && rc=20
    elif [ ! -f "$DEVICE_TREE_FIXTURE" ]; then
      note "TARGET_VERIFIED=yes, and $DEVICE_TREE_FIXTURE does not exist — a verification with nothing behind it"
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

  --self-test)
    self_test
    exit $?
    ;;

  ""|-h|--help)
    sed -n '3,23p' "$0" | sed 's/^# \{0,1\}//'
    exit 0
    ;;

  *)
    die "unknown option '$1' — try --help" 2
    ;;
esac
