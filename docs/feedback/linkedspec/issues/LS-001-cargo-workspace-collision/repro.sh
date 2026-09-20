#!/usr/bin/env bash
# Reproducer for LS-001 — Cargo workspace collision.
#
# Runs entirely against a LinkedSpec checkout. Needs nothing from the consuming repo.
# It builds the documented layout (an application repo that is a Cargo workspace,
# with LinkedSpec vendored at vendor/linkedspec), shows the failure, applies the
# proposed fix, and shows it pass. Exit 0 means BOTH halves behaved as described.
#
# usage: LS-001-workspace-collision.sh /path/to/linkedspec [workdir]
set -uo pipefail

LINKEDSPEC="${1:?usage: $0 /path/to/linkedspec [workdir]}"
WORK="${2:-$(dirname "$0")/.repro-work}"
[ -d "$LINKEDSPEC/examples/integration/rust" ] || { echo "not a linkedspec checkout: $LINKEDSPEC" >&2; exit 2; }

rm -rf "$WORK"; mkdir -p "$WORK/vendor"
# A minimal application repo that is a Cargo workspace — the common case.
cat > "$WORK/Cargo.toml" <<'TOML'
[workspace]
resolver = "2"
members = ["app"]
TOML
mkdir -p "$WORK/app/src"
cat > "$WORK/app/Cargo.toml" <<'TOML'
[package]
name = "app"
version = "0.1.0"
edition = "2021"
TOML
echo 'fn main() {}' > "$WORK/app/src/main.rs"

# The guide's documented vendoring location.
rsync -a --exclude "target/" "$LINKEDSPEC/" "$WORK/vendor/linkedspec/"

probe() {  # $1 = manifest under test, relative to the vendored checkout
  ( cd "$WORK" && cargo metadata --format-version 1 --no-deps \
      --manifest-path "vendor/linkedspec/$1" >/dev/null 2>"$WORK/err.txt" )
  return $?
}

echo "== Part 1: the documented layout, as shipped =="
fail_count=0
for m in examples/integration/rust/Cargo.toml rgx/subs/pgen/rust/Cargo.toml; do
  if probe "$m"; then
    echo "  UNEXPECTED PASS: $m"
  else
    if grep -q "believes it's in a workspace when it's not" "$WORK/err.txt"; then
      echo "  REPRODUCED: $m -> 'believes it's in a workspace when it's not'"
      fail_count=$((fail_count + 1))
    else
      echo "  FAILED FOR ANOTHER REASON: $m"; head -3 "$WORK/err.txt" | sed 's/^/    /'
    fi
  fi
done

echo "== Part 2: the proposed fix — an empty [workspace] table in each manifest =="
pass_count=0
for m in examples/integration/rust/Cargo.toml rgx/subs/pgen/rust/Cargo.toml; do
  printf '\n[workspace]\n' >> "$WORK/vendor/linkedspec/$m"
  if probe "$m"; then echo "  FIXED: $m"; pass_count=$((pass_count + 1))
  else echo "  STILL BROKEN: $m"; head -3 "$WORK/err.txt" | sed 's/^/    /'; fi
done

echo
if [ "$fail_count" -eq 2 ] && [ "$pass_count" -eq 2 ]; then
  echo "RESULT: reproduced (2/2) and fixed by the proposal (2/2)."; exit 0
fi
echo "RESULT: did not behave as described (reproduced $fail_count/2, fixed $pass_count/2)."; exit 1
