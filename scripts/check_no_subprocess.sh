#!/usr/bin/env bash
# scripts/check_no_subprocess.sh — NO-SUBPROCESS: the product spawns no process and executes nothing (leaf `API.2`).
#
# ⭐ WHY THIS EXISTS. §10.4's safety argument for handing archogen to an arbitrary agent — the MCP server, the wasm
# binding — rests on one property: every programmatic operation is computation over an in-memory description.
# `archogen build` writes a crate tree and stops; nothing in the product compiles, links or runs anything. That was
# true when it was measured (`2026-09-28`) and written down nowhere, so a `Command::new` added to a product crate
# would have passed every gate in the tree (`docs/decisions/decision_programmatic-interface.md`).
#
# THE POPULATION is derived: every tracked `crates/*/src/**/*.rs`, the product's production code. `xtask/` and
# `scripts/`, which legitimately drive a toolchain, are outside it by construction, and so is every `tests/`
# directory. An empty population is a breach.
#
# THE TEST HALF of a source file is excluded, and the exclusion is narrow on purpose: it starts at a
# `#[cfg(test)]` that opens a *module* (the next item line is `mod …`), and runs to the end of the file. A
# `#[cfg(test)]` on a lone helper function ends nothing — otherwise one attribute near the top of a file would
# silence every line below it.
#
# REFUSED in production code: `Command::new`, any path to `process::Command` (a `use` included), `.spawn(`,
# `execv`/`execvp`, `fork(`. `std::process::ExitCode` and `std::process::exit` spawn nothing and pass.
#
# ⚠️ HONEST LIMIT: it reads text. A spawn reached through a macro, a re-export under another name, or a dependency
# is not seen; the workspace has no third-party dependency (`decision_zero-dependency-engine-core.md`), which is
# what keeps the last one theoretical. Code placed after a test module is treated as test code.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

# No `\b`: it is not a word boundary under every awk and grep this runs with (leaf `API.1` measured it).
SPAWN='(^|[^A-Za-z0-9_])Command::new|process::Command([^A-Za-z0-9_]|$)|use std::process::\{[^}]*Command|\.spawn\(|(^|[^A-Za-z0-9_])execvp?\(|(^|[^A-Za-z0-9_])fork\('

fail=0
note() { printf 'NO-SUBPROCESS: %s\n' "$1" >&2; fail=$((fail + 1)); }

# The production lines of $1, as "line: text": everything before a `#[cfg(test)]` that opens a module.
production() {
  awk '
    pending && /^[[:space:]]*(#\[|$)/ { held = held "\n" FNR ": " $0; next }
    pending && /^[[:space:]]*(pub[[:space:]]+)?mod[[:space:]]/ { exit }
    pending { printf "%s\n", substr(held, 2); pending = 0; held = "" }
    /^[[:space:]]*#\[cfg\(test\)\][[:space:]]*$/ { pending = 1; held = FNR ": " $0; next }
    { print FNR ": " $0 }
    END { if (pending) printf "%s\n", substr(held, 2) }' "$1"
}

scan() {
  local files file hit
  files="$(git ls-files -- 'crates/*/src/*.rs' 'crates/*/src/**/*.rs' | sort -u)"
  [ -n "$files" ] || { note "no product source under crates/*/src — an empty population is a breach, not a pass"; return; }
  while IFS= read -r file; do
    [ -f "$file" ] || continue
    checked=$((checked + 1))
    while IFS= read -r hit; do
      [ -n "$hit" ] || continue
      note "$file:${hit%%:*} spawns or names a subprocess in production code:${hit#*:}"
    done < <(production "$file" | grep -E -- "$SPAWN")
  done <<< "$files"
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/no_subprocess/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/crates/eng/src" "$work/crates/eng/tests" "$work/xtask/src" "$work/scripts"
    git -C "$work" init -q
    printf 'use std::process::ExitCode;\npub fn code() -> ExitCode { ExitCode::SUCCESS }\npub fn stop() { std::process::exit(0) }\n' > "$work/crates/eng/src/lib.rs"
    git -C "$work" add -A
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry ("" for a pass)
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    git -C "$work" add -A
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh; arm "ExitCode and process::exit spawn nothing, and pass" 0 ""
  fresh; printf 'pub fn run() { let _ = std::process::Command::new("cc").status(); }\n' >> "$work/crates/eng/src/lib.rs"
  arm "a real spawn in production code is refused, with its line" 1 "crates/eng/src/lib.rs:4 spawns"
  fresh; printf 'use std::process::{Command, Stdio};\n' > "$work/crates/eng/src/io.rs"
  arm "an import of Command is refused, though nothing calls it yet" 1 "crates/eng/src/io.rs:1"
  fresh; printf '\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn t() { let _ = std::process::Command::new("cc"); }\n}\n' >> "$work/crates/eng/src/lib.rs"
  arm "a spawn inside the test module passes" 0 ""
  fresh; printf '#[cfg(test)]\nfn helper() {}\npub fn run() { let _ = std::process::Command::new("cc"); }\n' >> "$work/crates/eng/src/lib.rs"
  arm "a cfg(test) on a lone function does not silence the lines below it" 1 "crates/eng/src/lib.rs:6 spawns"
  fresh; printf 'fn f() { let _ = std::process::Command::new("cc"); }\n' > "$work/crates/eng/tests/it.rs"
  printf 'fn f() { let _ = std::process::Command::new("cargo"); }\n' > "$work/xtask/src/main.rs"
  arm "tests/ and xtask/ drive toolchains legitimately, and are outside the population" 0 ""
  fresh; printf 'pub fn go(c: &mut Child) { let _ = c.spawn(); }\n' >> "$work/crates/eng/src/lib.rs"
  arm "a .spawn( call is refused however the command was built" 1 "spawns or names a subprocess"
  fresh; git -C "$work" rm -q -r --cached crates; rm -rf "$work/crates"
  arm "no product source at all is a breach, not a pass" 1 "an empty population is a breach"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real product spawns nothing"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "no-subprocess self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

checked=0
scan
if [ "$fail" -ne 0 ]; then
  echo "NO-SUBPROCESS: $fail breach(es) — the product computes over descriptions and executes nothing (§10.4)" >&2
  exit 1
fi
echo "no-subprocess: OK ($checked production source file(s) under crates/*/src; none spawns a process)"
exit 0
