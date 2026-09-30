#!/usr/bin/env bash
# scripts/wasm_build.sh — compile the engine's I/O-free crates for the browser target, `wasm32-unknown-unknown`
# (leaf `API.1`).
#
# ⭐ WHY A MEASUREMENT FIRST. The programmatic-interface decision promises a wasm binding. "Should be feasible"
# is not a fact; "compiles, or here is exactly what breaks" is. This step is the `wasm-build` step of the
# `integration` tier, in the shape of `no-std-build`.
#
# THE CRATE SET IS DERIVED, never listed: every workspace member (`cargo metadata`) whose production code names
# none of `std::fs`, `std::process`, `std::net` or `std::env`. Production code is each `src/**/*.rs` up to its
# first `#[cfg(test)]` line. A member excluded by that rule is named, with the line that excluded it. A new crate
# is in scope the day it is added, without anyone editing this file.
#
# ⚠️ HONEST LIMIT: the rule reads text. A path written some other way (an alias, a re-export, a macro) is not seen,
# and a crate that compiles for wasm32 is not thereby usable in a browser — `std::fs` compiles there and fails at
# run time, which is exactly why a crate that uses it is excluded rather than counted.
#
# CONTRACT: exit 0 = every in-scope crate compiled; 1 = one did not (cargo's error shown), or the set is empty;
# `--list` prints the set and the exclusions and builds nothing. `--self-test` runs the derivation against scratch
# workspaces in `target/doctrine_scratch/` — no compiler needed.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

TARGET="wasm32-unknown-unknown"
# ⛔ No `\b`: it is not a word boundary in every awk or grep this runs under (BSD `awk` read it as something else,
# and the step then compiled all nine members and passed — the arms caught it). Boundaries are spelled out instead.
IO='std::(fs|process|net|env)([^A-Za-z0-9_]|$)|use std::\{([^}]*[ ,])?(fs|process|net|env)[ ,}]'

# Every member as "<name>\t<directory>", from the workspace's own metadata.
members() {
  cargo metadata --no-deps --format-version 1 2>/dev/null | python3 -c '
import json, os, sys
for p in json.load(sys.stdin)["packages"]:
    print(p["name"] + "\t" + os.path.dirname(p["manifest_path"]))'
}

# The first production line of $1 (a crate directory) that does I/O, as "file:line: text", or nothing.
first_io() {
  local file hit
  while IFS= read -r file; do
    hit="$(awk '/^[[:space:]]*#\[cfg\(test\)\]/ { exit } { print FNR ": " $0 }' "$file" | grep -m 1 -E -- "$IO")"
    if [ -n "$hit" ]; then printf '%s:%s\n' "${file#"$ROOT"/}" "$hit"; return; fi
  done < <(find "$1/src" -name '*.rs' 2>/dev/null | sort)
}

# In-scope names on stdout; exclusions on stderr.
derive() {
  local name dir hit
  while IFS=$'\t' read -r name dir; do
    [ -n "$name" ] || continue
    hit="$(first_io "$dir")"
    if [ -n "$hit" ]; then
      printf 'wasm-build: excluded %-18s %s\n' "$name" "$(printf '%s' "$hit" | sed 's/[[:space:]]\{2,\}/ /g' | cut -c1-120)" >&2
    else
      printf '%s\n' "$name"
    fi
  done < <(members)
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/wasm_build/selftest"
  crate() { # $1 = name, $2 = body of src/lib.rs
    mkdir -p "$work/$1/src"
    printf '[package]\nname = "%s"\nversion = "0.0.0"\nedition = "2021"\n' "$1" > "$work/$1/Cargo.toml"
    printf '%b' "$2" > "$work/$1/src/lib.rs"
  }
  fresh() {
    rm -rf "$work"; mkdir -p "$work"; git -C "$work" init -q
    printf '[workspace]\nmembers = ["pure", "reads"]\nresolver = "2"\n' > "$work/Cargo.toml"
    crate pure 'pub fn add(a: u32, b: u32) -> u32 { a + b }\n'
    crate reads 'pub fn load(p: &str) -> String { std::fs::read_to_string(p).unwrap_or_default() }\n'
  }
  arm() { # $1 = name, $2 = text the in-scope list must equal, $3 = text the exclusions must carry ("" for none)
    local name="$1" want="$2" must="$3" got err
    arms=$((arms + 1))
    got="$(cd "$work" && bash "$SELF" --list 2>"$work/.err" | paste -sd, -)"; err="$(cat "$work/.err")"
    if [ "$got" != "$want" ]; then echo "SELF-TEST: $name — in scope '$got', expected '$want'" >&2; return; fi
    if [ -n "$must" ] && ! printf '%s' "$err" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — the exclusion does not say \`$must\`: $err" >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh; arm "a crate that reads files is excluded, naming the line" "pure" "excluded reads"
  fresh; crate reads 'use std::{fs, io};\npub fn load(p: &str) -> io::Result<String> { fs::read_to_string(p) }\n'
  arm "a grouped import of fs is seen too" "pure" "use std::{fs, io};"
  fresh; crate reads 'pub fn f() {}\n#[cfg(test)]\nmod tests { #[test] fn t() { let _ = std::fs::read("x"); } }\n'
  arm "I/O inside the test module does not exclude a crate" "pure,reads" ""
  fresh; crate pure 'pub fn run() { let _ = std::process::id(); }\n'
  arm "a process call excludes, as a file read does" "" "excluded pure"
  fresh; printf '[workspace]\nmembers = ["pure", "reads", "later"]\nresolver = "2"\n' > "$work/Cargo.toml"; crate later 'pub fn g() {}\n'
  arm "a member added later is in scope without editing the step" "pure,later" ""
  rm -rf "$work"
  echo "wasm-build self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  --list) derive; exit 0 ;;
  "") ;;
  *) echo "wasm-build: unknown argument '$1' — none, --list or --self-test" >&2; exit 2 ;;
esac

crates=()
while IFS= read -r name; do crates+=("-p" "$name"); done < <(derive)
if [ "${#crates[@]}" -eq 0 ]; then
  echo "wasm-build: no workspace member is I/O-free — nothing compiled, which is not a pass" >&2
  exit 1
fi
echo "wasm-build: compiling for $TARGET: $(printf '%s\n' "${crates[@]}" | grep -v '^-p$' | paste -sd' ' -)"
cargo build -q --target "$TARGET" "${crates[@]}"
