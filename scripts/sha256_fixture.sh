#!/usr/bin/env bash
# scripts/sha256_fixture.sh — the system's SHA-256 over every message length the padding treats differently
# (leaf `M2.7.2`).
#
# The catalog's content hash is SHA-256 implemented in the workspace (`crates/archogen-evidence/src/sha256.rs`),
# because the engine depends on nothing outside `std` and spawns no process. So this oracle is written down rather
# than run: the script hashes deterministic messages with the system's own tool (`sha256sum`, or `shasum -a 256`
# where coreutils is absent) and writes the digests to the fixture `crates/archogen-evidence/tests/sha256.rs`
# compares against. Byte `i` of the message of length `n` is `(7 × i + 3) mod 256`. The lengths are every `n` from
# 0 to 129, which puts the padding's `0x80` byte and the 64-bit length at every offset of one block and across
# two, and a few longer ones across block and page boundaries.
#
# USAGE:
#   bash scripts/sha256_fixture.sh           # rewrite the fixture
#   bash scripts/sha256_fixture.sh --check   # compare the fixture with a fresh run: exit 0 agree, 1 differ,
#                                            # 2 no tool to compare with
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

FIXTURE=crates/archogen-evidence/tests/fixtures/sha256_boundaries.txt
SCRATCH=target/doctrine_scratch/sha256_fixture
mkdir -p "$SCRATCH"

sha() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1
  elif command -v shasum >/dev/null 2>&1; then shasum -a 256 "$1" | cut -d' ' -f1
  else echo "sha256-fixture: no SHA-256 tool (sha256sum or shasum) to compare with" >&2; exit 2
  fi
}

emit() {
  echo "# The system's SHA-256 of message n, byte i = (7*i + 3) mod 256, written by scripts/sha256_fixture.sh."
  echo "# <length> sha256:<digest>"
  for n in $(seq 0 129) 1000 4095 4096 4097 65536; do
    perl -e 'my $n = shift; binmode STDOUT; print map { chr((7 * $_ + 3) % 256) } 0 .. $n - 1' "$n" \
      > "$SCRATCH/message"
    echo "$n sha256:$(sha "$SCRATCH/message")"
  done
}

case "${1:-}" in
  --check)
    emit > "$SCRATCH/fresh.txt"
    if diff -u "$FIXTURE" "$SCRATCH/fresh.txt"; then
      echo "sha256-fixture: the fixture agrees with the system's tool ($(grep -vc '^#' "$FIXTURE") lengths)"
    else
      echo "sha256-fixture: the fixture differs from the system's tool" >&2
      exit 1
    fi
    ;;
  "")
    mkdir -p "$(dirname "$FIXTURE")"
    emit > "$FIXTURE"
    echo "sha256-fixture: wrote $FIXTURE"
    ;;
  *)
    echo "usage: bash scripts/sha256_fixture.sh [--check]" >&2
    exit 2
    ;;
esac
