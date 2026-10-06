#!/usr/bin/env bash
# scripts/check_untracked_code.sh — UNTRACKED-CODE: no commit is made while a file in a code path is untracked
# (leaf `PROGRAM.66`).
#
# ⭐ WHY THIS EXISTS. `ARCHOGEN-M3-0485` was made with `git commit -a`, which stages no new file, while its own
# `xtask/src/main.rs` declared `mod trust_verify;` and `xtask/src/trust_verify.rs` sat untracked beside it. The tree
# the commit recorded did not build (`error[E0583]: file not found for module`), and every check run before it —
# the focused tier, the doctrine gate — read the working tree, where the file was, and passed. A file in a code path
# that git does not track is either meant for the commit or a leftover; either way a commit made beside it records a
# tree nobody ran. `M3.6.3.5`'s correction, `ARCHOGEN-M3-0486`, committed the file.
#
# THE RULE: `git ls-files --others --exclude-standard` lists no path that a pattern of `.doctrine/code_paths.txt` —
# the project's own statement of what is code, one extended regular expression per line — matches. An ignored file
# (`target/`, `.archogen-data/`) is not untracked in this sense, and a file outside every code path, a note or a
# draft, is not this gate's. A repository with no `.doctrine/code_paths.txt` cannot tell code from anything else,
# which is a breach, not a pass. In CI a fresh checkout holds no untracked file, so the gate passes there.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

fail=0
note() { printf 'UNTRACKED-CODE: %s\n' "$1" >&2; fail=$((fail + 1)); }

run() {
  local patterns path
  [ -f .doctrine/code_paths.txt ] || {
    note "no .doctrine/code_paths.txt — nothing says which files are code, so none can be judged"
    return
  }
  patterns="$(grep -vE '^[[:space:]]*(#|$)' .doctrine/code_paths.txt)"
  [ -n "$patterns" ] || { note ".doctrine/code_paths.txt holds no pattern"; return; }
  while IFS= read -r -d '' path; do
    if printf '%s\n' "$path" | grep -qE -f <(printf '%s\n' "$patterns"); then
      note "\`$path\` is in a code path and untracked — stage it with the commit (\`git add\`), or delete it, or ignore it in .gitignore"
    fi
  done < <(git ls-files --others --exclude-standard -z)
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/untracked_code/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/src" "$work/.doctrine"
    git -C "$work" init -q
    printf '# code\n(^|/)crates/\n\\.(rs|sh)$\n' > "$work/.doctrine/code_paths.txt"
    printf '/target\n' > "$work/.gitignore"
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
  fresh; arm "every file tracked passes" 0 ""
  fresh; printf 'pub fn f() {}\n' > "$work/src/new.rs"
  arm "an untracked Rust file is refused, by name — 0485's module" 1 "\`src/new.rs\` is in a code path and untracked"
  fresh; mkdir -p "$work/crates/x"; printf 'data\n' > "$work/crates/x/table.txt"
  arm "an untracked file under a code directory is refused" 1 "\`crates/x/table.txt\` is in a code path"
  fresh; printf 'pub fn f() {}\n' > "$work/src/new.rs"; git -C "$work" add src/new.rs
  arm "a new file staged with the commit passes" 0 ""
  fresh; printf 'notes\n' > "$work/notes.md"
  arm "an untracked file outside every code path is not this gate's" 0 ""
  fresh; mkdir -p "$work/target"; printf 'pub fn f() {}\n' > "$work/target/scratch.rs"
  arm "an ignored file is not untracked" 0 ""
  fresh; git -C "$work" rm -q --cached .doctrine/code_paths.txt; rm "$work/.doctrine/code_paths.txt"
  arm "no statement of what is code is a breach, not a pass" 1 "no .doctrine/code_paths.txt"
  rm -rf "$work"
  echo "untracked-code self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

run
if [ "$fail" -ne 0 ]; then
  echo "UNTRACKED-CODE: $fail breach(es) — no commit is made while a file in a code path is untracked" >&2
  exit 1
fi
echo "untracked-code: OK (no untracked file in a code path)"
exit 0
