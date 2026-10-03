#!/usr/bin/env bash
# scripts/check_rust_format.sh — RUST-FORMAT: every Rust source a commit stages is in canonical format (leaf `PROGRAM.51`).
#
# ⭐ WHY THIS EXISTS. `COMMIT.md` step 2 asks for the `focused` tier — format, lints, the whole suite — whenever Rust
# changes, and nothing at commit time held anyone to it: `ARCHOGEN-M2-0394` committed two unformatted files after
# running the tests and clippy alone, and the tier's `fmt` step failed at HEAD until `M2.22` (`2026-10-03`). Of that
# tier, the format step is the one cheap enough for the pre-commit path.
#
# THE POPULATION is the commit's: every staged `*.rs` added, copied, modified or renamed, outside `vendor/`, read from
# the index — the bytes that will be committed, not the working tree's. When something is staged but no Rust, there is
# nothing to judge. When nothing is staged at all — CI, a clean tree — it is every tracked `*.rs` outside `vendor/`,
# read from the working tree. An empty population then is a breach.
#
# HOW each file is judged: its bytes through `rustfmt --edition <the workspace's> --emit stdout`, compared byte for
# byte with the input. Read from stdin, rustfmt follows no `mod` declaration, so one file is judged alone, and a file
# rustfmt cannot parse is refused. ⚠️ Not `--check`: on stdin it prints its diff and still exits 0 (measured on the
# pin, rustfmt of rustc 1.95.0, `2026-10-03`; the ledger's `rust-toolchain` entry).
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

fail=0
checked=0
note() { printf 'RUST-FORMAT: %s\n' "$1" >&2; fail=$((fail + 1)); }

# The workspace's edition, from the root manifest; 2021 where none is written (a scratch repository).
edition="$(sed -n 's/^edition[[:space:]]*=[[:space:]]*"\([0-9]*\)".*/\1/p' Cargo.toml 2>/dev/null | head -1)"
edition="${edition:-2021}"

# Judge one file: $1 = its path, $2 = `index` or `tree`, where its bytes are read from.
judge() {
  local path="$1" from="$2" line
  if [ "$from" = index ]; then
    git show ":$path" > "$tmp/in.rs" 2>/dev/null || { note "$path: its staged bytes cannot be read"; return; }
  else
    cp "$path" "$tmp/in.rs" 2>/dev/null || { note "$path: it cannot be read"; return; }
  fi
  checked=$((checked + 1))
  if ! rustfmt --edition "$edition" --emit stdout < "$tmp/in.rs" > "$tmp/out.rs" 2> "$tmp/err"; then
    note "$path: rustfmt cannot read it — $(head -1 "$tmp/err")"
    return
  fi
  if ! cmp -s "$tmp/in.rs" "$tmp/out.rs"; then
    line="$(cmp "$tmp/in.rs" "$tmp/out.rs" 2>/dev/null | sed -n 's/.* line \([0-9]*\).*/\1/p')"
    note "$path is not in canonical format (first difference at line ${line:-?}) — run \`cargo fmt\` and stage the result"
  fi
}

run() {
  local file staged
  command -v rustfmt >/dev/null 2>&1 || {
    note "rustfmt is not on PATH — the pinned toolchain's rustfmt component (rust-toolchain.toml) is required"
    return
  }
  mkdir -p "$ROOT/target" || { note "cannot make $ROOT/target"; return; }
  tmp="$(mktemp -d "$ROOT/target/rust_format.XXXXXX")" || { note "cannot make a scratch directory under target/"; return; }
  if ! git diff --cached --quiet 2>/dev/null; then
    staged="$(git diff --cached --name-only --diff-filter=ACMR -- '*.rs' ':(exclude)vendor/**')"
    [ -n "$staged" ] || { mode="staged, no Rust among it"; return; }
    mode="staged"
    while IFS= read -r file; do
      [ -n "$file" ] && judge "$file" index
    done <<< "$staged"
  else
    mode="tracked"
    staged="$(git ls-files -- '*.rs' ':(exclude)vendor/**')"
    [ -n "$staged" ] || { note "no tracked Rust source and nothing staged — an empty population is a breach, not a pass"; return; }
    while IFS= read -r file; do
      [ -n "$file" ] && [ -f "$file" ] && judge "$file" tree
    done <<< "$staged"
  fi
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/rust_format/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/src" "$work/vendor/other"
    git -C "$work" init -q
    git -C "$work" config user.name selftest; git -C "$work" config user.email selftest@example.invalid
    printf 'fn main() {}\n' > "$work/src/main.rs"
    git -C "$work" add -A
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry ("" for a pass)
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh; arm "a canonical staged file passes" 0 ""
  fresh; printf 'fn  main( ){}\n' > "$work/src/main.rs"; git -C "$work" add -A
  arm "an unformatted staged file is refused, by name" 1 "src/main.rs is not in canonical format"
  fresh; printf 'fn  main( ){}\n' > "$work/src/main.rs"; git -C "$work" add -A; printf 'fn main() {}\n' > "$work/src/main.rs"
  arm "the staged bytes are judged, not a fix left in the working tree" 1 "src/main.rs is not in canonical format"
  fresh; printf 'fn main() {}\n' > "$work/src/main.rs"; git -C "$work" add -A; printf 'fn  main( ){}\n' > "$work/src/main.rs"
  arm "an unformatted working tree over canonical staged bytes passes" 0 ""
  fresh; printf 'mod absent;\nfn main() {}\n' > "$work/src/main.rs"; git -C "$work" add -A
  arm "a mod declaration is not followed: one file is judged alone" 0 ""
  fresh; printf 'fn main( {\n' > "$work/src/main.rs"; git -C "$work" add -A
  arm "a file rustfmt cannot read is refused" 1 "rustfmt cannot read it"
  fresh; printf 'fn  x( ){}\n' > "$work/vendor/other/lib.rs"; git -C "$work" add -A
  arm "a file under vendor/ is outside the population" 0 ""
  fresh; git -C "$work" commit -q -m base; printf 'notes\n' > "$work/notes.md"; git -C "$work" add -A
  arm "a commit staging no Rust has nothing to judge" 0 ""
  fresh; printf 'fn  main( ){}\n' > "$work/src/main.rs"; git -C "$work" add -A; git -C "$work" commit -q -m base
  arm "with nothing staged, every tracked file is judged" 1 "src/main.rs is not in canonical format"
  fresh; git -C "$work" rm -q -r --cached src; rm -rf "$work/src"
  arm "nothing staged and nothing tracked is a breach, not a pass" 1 "an empty population is a breach"
  rm -rf "$work"
  echo "rust-format self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

tmp=""
mode=""
trap '[ -n "$tmp" ] && rm -rf "$tmp"' EXIT
run
if [ "$fail" -ne 0 ]; then
  echo "RUST-FORMAT: $fail breach(es) — every Rust source a commit stages is in canonical format (COMMIT.md step 2)" >&2
  exit 1
fi
echo "rust-format: OK ($checked $mode Rust source file(s), each in canonical format)"
exit 0
