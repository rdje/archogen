#!/usr/bin/env bash
# scripts/ci_provision.sh — install the `integration` tier's external tools at their pins, each download checked
# against a recorded digest before it is used (leaf `PROGRAM.10.4`).
#
# ⭐ WHY A SCRIPT, AND WHY FROM SOURCE. The pins are the contract (`docs/book/src/ledger.md`): mdBook at the release
# the book was measured with, QEMU at `QEMU_VERSION_PINNED` in `targets/riscv-virt-up.env`. A workflow that installs
# "a QEMU" from the runner's package manager gets whatever that image ships, and `target_emulator.sh --check` then
# refuses it as a release mismatch — correctly, because a different release is a different virtual platform
# (§3.2). So the pinned QEMU is built from its release tarball, for the one target the tier runs.
#
# WHERE: everything under `target/ci/` — downloads in `downloads/`, the build in `build/`, the tools in
# `tools/bin/`. Under GitHub Actions that directory is appended to `$GITHUB_PATH`; elsewhere the script prints the
# `PATH` line to use. A tool already at its pin is kept, so a cached `target/ci/tools/` costs nothing.
#
# THE PINS are data, in `.github/ci-tools.env` (mdBook's release, and every digest) and `targets/riscv-virt-up.env`
# (QEMU's release — the target's contract). `SOURCE-LEDGER` holds each `*_VERSION_PINNED` to its ledger entry.
#
# ⚠️ WHAT THE DIGESTS ARE WORTH, retrieved `2026-09-30`. mdBook's agree with the digests GitHub publishes for the
# release assets: two derivations. QEMU's is the tarball as served over HTTPS by download.qemu.org: one
# derivation. Its GPG signature exists and is not checked here.
#
# CONTRACT: exit 0 = every tool present at its pin; 1 = a download, a digest, a build or a reported version
# disagreed; 2 = no artifact is recorded for this platform, or the pin moved past its recorded digest.
# `--self-test` runs the verifying download against `file://` fixtures in `target/doctrine_scratch/` — no network.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

PINS=".github/ci-tools.env"
[ -f "$PINS" ] || { printf 'ci-provision: %s is missing — it holds every pin and digest this script uses\n' "$PINS" >&2; exit 2; }
# shellcheck source=/dev/null
set -a; . "$PINS"; set +a
MDBOOK_VERSION="$MDBOOK_VERSION_PINNED"

CI="$ROOT/target/ci"
DL="$CI/downloads"
TOOLS="$CI/tools"
BIN="$TOOLS/bin"

note() { printf 'ci-provision: %s\n' "$1" >&2; }

# The mdBook asset for this platform and its digest, or nothing.
mdbook_asset() {
  case "$(uname -s)/$(uname -m)" in
    Linux/x86_64) echo "mdbook-v$MDBOOK_VERSION-x86_64-unknown-linux-gnu.tar.gz $MDBOOK_SHA256_LINUX_X86_64" ;;
    Darwin/arm64) echo "mdbook-v$MDBOOK_VERSION-aarch64-apple-darwin.tar.gz $MDBOOK_SHA256_DARWIN_ARM64" ;;
  esac
}

sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1
  else shasum -a 256 "$1" | cut -d' ' -f1; fi
}

# Download $1 to $2 and keep it only if its digest is $3. A file already there with that digest is reused.
fetch() {
  local url="$1" file="$2" want="$3" got
  if [ -f "$file" ] && [ "$(sha256_of "$file")" = "$want" ]; then return 0; fi
  mkdir -p "$(dirname "$file")"
  rm -f "$file" "$file.partial"
  if ! curl -sSfL --retry 3 -o "$file.partial" "$url"; then
    note "download failed: $url"; rm -f "$file.partial"; return 1
  fi
  got="$(sha256_of "$file.partial")"
  if [ "$got" != "$want" ]; then
    note "DIGEST MISMATCH for $url: recorded $want, received $got — refused, and not kept"
    rm -f "$file.partial"; return 1
  fi
  mv "$file.partial" "$file"
}

jobs() { nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 2; }

install_mdbook() {
  local want="mdbook v$MDBOOK_VERSION" asset sha have
  if [ -x "$BIN/mdbook" ] && [ "$("$BIN/mdbook" --version 2>/dev/null)" = "$want" ]; then
    note "mdbook: $want already in place"; return 0
  fi
  read -r asset sha <<< "$(mdbook_asset)"
  [ -n "${asset:-}" ] || { note "no mdBook artifact is recorded for $(uname -s)/$(uname -m)"; return 2; }
  fetch "https://github.com/rust-lang/mdBook/releases/download/v$MDBOOK_VERSION/$asset" "$DL/$asset" "$sha" || return 1
  mkdir -p "$BIN"
  tar -xzf "$DL/$asset" -C "$BIN" mdbook || { note "could not unpack $asset"; return 1; }
  have="$("$BIN/mdbook" --version 2>/dev/null)"
  [ "$have" = "$want" ] || { note "the unpacked mdbook reports '$have', and the pin is '$want'"; return 1; }
  note "mdbook: $want installed"
}

install_qemu() {
  local pinned want src log have
  pinned="$(sed -n 's/^QEMU_VERSION_PINNED=//p' targets/riscv-virt-up.env)"
  want="QEMU emulator version $pinned"
  if [ "$pinned" != "$QEMU_SOURCE_RELEASE" ]; then
    note "the pin is QEMU $pinned and $PINS records the digest of $QEMU_SOURCE_RELEASE — record the new release's digest first"
    return 2
  fi
  if [ -x "$BIN/qemu-system-riscv64" ] && "$BIN/qemu-system-riscv64" --version 2>/dev/null | head -1 | grep -qF "$want"; then
    note "qemu: $want already in place"; return 0
  fi
  fetch "https://download.qemu.org/qemu-$pinned.tar.xz" "$DL/qemu-$pinned.tar.xz" "$QEMU_SOURCE_SHA256" || return 1
  src="$CI/build/qemu-$pinned"; log="$CI/build/qemu-$pinned.log"
  rm -rf "$src"; mkdir -p "$CI/build"
  tar -xJf "$DL/qemu-$pinned.tar.xz" -C "$CI/build" || { note "could not unpack the QEMU tarball"; return 1; }
  note "qemu: building $pinned for riscv64-softmmu with $(jobs) jobs — log in ${log#"$ROOT"/}"
  if ! ( cd "$src" && ./configure --prefix="$TOOLS" --target-list=riscv64-softmmu --disable-docs \
           && make -j"$(jobs)" && make install ) > "$log" 2>&1; then
    note "the QEMU build failed; the end of ${log#"$ROOT"/}:"; tail -n 30 "$log" >&2; return 1
  fi
  have="$("$BIN/qemu-system-riscv64" --version 2>/dev/null | head -1)"
  printf '%s' "$have" | grep -qF "$want" || { note "the built QEMU reports '$have', and the pin is '$want'"; return 1; }
  note "qemu: $have installed"
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/ci_provision/selftest" good
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, then the command
    local name="$1" want="$2" must="$3" out rc; shift 3
    arms=$((arms + 1))
    out="$("$@" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  rm -rf "$work"; mkdir -p "$work/origin"
  printf 'the pinned artifact\n' > "$work/origin/artifact"
  good="$(sha256_of "$work/origin/artifact")"
  arm "a download whose digest matches the record is kept" 0 "" fetch "file://$work/origin/artifact" "$work/dl/artifact" "$good"
  arms=$((arms + 1))
  if [ -f "$work/dl/artifact" ] && [ ! -e "$work/dl/artifact.partial" ]; then ok=$((ok + 1)); echo "  ✅ …and it is where the caller asked, with no partial left"
  else echo "SELF-TEST: the verified download is not in place" >&2; fi
  rm -rf "$work/dl"
  arm "a download whose digest differs is refused" 1 "DIGEST MISMATCH" fetch "file://$work/origin/artifact" "$work/dl/artifact" "0000$good"
  arms=$((arms + 1))
  if [ ! -e "$work/dl/artifact" ] && [ ! -e "$work/dl/artifact.partial" ]; then ok=$((ok + 1)); echo "  ✅ …and nothing of it is kept to be used later"
  else echo "SELF-TEST: a refused download was left on disk" >&2; fi
  mkdir -p "$work/dl"; printf 'tampered\n' > "$work/dl/artifact"
  arm "a file already there with another digest is replaced, not trusted" 0 "" fetch "file://$work/origin/artifact" "$work/dl/artifact" "$good"
  arms=$((arms + 1))
  if [ "$(sha256_of "$work/dl/artifact")" = "$good" ]; then ok=$((ok + 1)); echo "  ✅ …with the verified bytes"
  else echo "SELF-TEST: the tampered file survived" >&2; fi
  arm "an unreachable source is a failure" 1 "download failed" fetch "file://$work/origin/absent" "$work/dl/other" "$good"
  arm "every digest $PINS records is a sha256" 0 "" \
    bash -c "n=\$(grep -cE '^[A-Z0-9_]+_SHA256[A-Z0-9_]*=' '$ROOT/$PINS'); [ \"\$n\" -ge 3 ] && [ \"\$n\" = \"\$(grep -cE '^[A-Z0-9_]+_SHA256[A-Z0-9_]*=[0-9a-f]{64}\$' '$ROOT/$PINS')\" ]"
  rm -rf "$work"
  echo "ci-provision self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") ;;
  *) note "unknown argument '$1' — the script takes none, or --self-test"; exit 2 ;;
esac

rc=0
install_mdbook || rc=$?
[ "$rc" -eq 0 ] && { install_qemu || rc=$?; }
if [ "$rc" -ne 0 ]; then note "provisioning stopped (exit $rc) — the tier would report what is missing as a failure"; exit "$rc"; fi
if [ -n "${GITHUB_PATH:-}" ]; then
  echo "$BIN" >> "$GITHUB_PATH"
  note "added ${BIN#"$ROOT"/} to the job's PATH"
else
  note "done — put the tools first on PATH:  export PATH=\"\$PWD/${BIN#"$ROOT"/}:\$PATH\""
fi
exit 0
