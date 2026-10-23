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
# THE THREE ROUTES. A workspace with no third-party dependency (`decision_zero-dependency-engine-core.md`) reaches a
# process three ways, and each is closed by structure rather than by name (leaf `PROGRAM.105`, which measured an alias,
# a nested `use` group, a libc function declared in an `extern` block and a call to it each passing the name patterns):
#   - `std::process`, admitted only as `ExitCode` and `exit`, however it is imported: every `process::` path names one
#     of them, a `use` group under `process` holds only them, and `process` is never aliased or glob-imported;
#   - foreign functions: no `extern` block and no `#[link]` — so no `execve`, `posix_spawn` or `system` can be called,
#     declared or not; an `extern "C" fn` definition, the wasm module's export, is not a block and passes;
#   - a system call by inline assembly: no `asm!`, `global_asm!` or `naked_asm!`.
# Beside them the name patterns stay: `Command::new`, `.spawn(`, `execv`/`execvp`, `fork(`. The text read is production
# code with its comments and string and character literals removed — so a doc comment or a message may name what it
# refuses — and each `use` statement joined onto its first line, however rustfmt breaks it.
#
# ⚠️ HONEST LIMIT: it reads text, lexed lightly. A route through a macro whose expansion it cannot see, or a
# dependency, is not seen; the workspace has none, which keeps the second theoretical. A module of the product's own
# named `process` would be read as `std::process`. Code placed after a test module is treated as test code.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

# No `\b`: it is not a word boundary under every awk and grep this runs with (leaf `API.1` measured it).
SPAWN='(^|[^A-Za-z0-9_])Command::new|process::Command([^A-Za-z0-9_]|$)|use std::process::\{[^}]*Command|\.spawn\(|(^|[^A-Za-z0-9_])execvp?\(|(^|[^A-Za-z0-9_])fork\('

# An `extern` block, `extern "ABI" {`, `extern {` or `unsafe extern … {` — its ABI string read empty once stripped — or a
# `#[link]`; an `extern "C" fn` definition is not a block and passes.
FOREIGN='(^|[^A-Za-z0-9_])extern[[:space:]]*(""[[:space:]]*)?\{|#!?[[:space:]]*\[[[:space:]]*link[[:space:]]*[](]'
ASSEMBLY='(^|[^A-Za-z0-9_])(asm|global_asm|naked_asm)[[:space:]]*![[:space:]]*[[({]'
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

# Production lines as "N: text", comments and string and character literals removed (their quotes kept, so `extern
# "C"` reads `extern ""`), the state carried across lines: nested block comments, raw strings, a lifetime told apart from
# a character literal by the byte length of the character after its quote.
stripped() {
  LC_ALL=C awk '
    BEGIN { for (n = 1; n < 256; n++) ord[sprintf("%c", n)] = n; depth = 0; instr = 0; raw = -1 }
    function hashes(k,   h) { h = ""; while (k-- > 0) h = h "#"; return h }
    function ident(ch) { return ch ~ /[A-Za-z0-9_]/ }
    {
      colon = index($0, ": "); num = substr($0, 1, colon - 1); t = substr($0, colon + 2)
      out = ""; i = 1; n = length(t)
      while (i <= n) {
        c = substr(t, i, 1); c2 = substr(t, i, 2)
        if (depth > 0) {
          if (c2 == "/*") { depth++; i += 2; continue }
          if (c2 == "*/") { depth--; i += 2; out = out " "; continue }
          i++; continue
        }
        if (instr) {
          if (raw >= 0) {
            if (c == "\"" && substr(t, i + 1, raw) == hashes(raw)) { instr = 0; raw = -1; out = out "\""; i += 1; continue }
            i++; continue
          }
          if (c == "\\") { i += 2; continue }
          if (c == "\"") { instr = 0; out = out "\""; i++; continue }
          i++; continue
        }
        if (c2 == "//") break
        if (c2 == "/*") { depth = 1; i += 2; continue }
        prev = (i > 1) ? substr(t, i - 1, 1) : ""
        j = i; if (c == "b" && substr(t, i + 1, 1) == "r") j = i + 1
        if (substr(t, j, 1) == "r" && !ident(prev) && (j == i || c == "b")) {
          k = j + 1; h = 0; while (substr(t, k, 1) == "#") { h++; k++ }
          if (substr(t, k, 1) == "\"") { instr = 1; raw = h; out = out "\""; i = k + 1; continue }
        }
        if (c == "\"") { instr = 1; raw = -1; out = out "\""; i++; continue }
        if (c == "'"'"'") {
          if (substr(t, i + 1, 1) == "\\") {
            k = index(substr(t, i + 2), "'"'"'")
            if (k > 0) { out = out "'"'"''"'"'"; i = i + 2 + k; continue }
          }
          b = ord[substr(t, i + 1, 1)]; len = (b < 128) ? 1 : (b >= 240) ? 4 : (b >= 224) ? 3 : 2
          if (substr(t, i + 1 + len, 1) == "'"'"'") { out = out "'"'"''"'"'"; i = i + 2 + len; continue }
        }
        out = out c; i++
      }
      print num ": " out
    }
    END { if (depth > 0 || instr) print "0: UNTERMINATED" }'
}

# Each `use` statement onto its first line, its continuation lines left empty so every number stays the file's.
joined() {
  awk '
    function body(line) { return substr(line, index(line, ": ") + 2) }
    acc != "" {
      acc = acc " " body($0); n = n + 1; held[n] = substr($0, 1, index($0, ": ") + 1)
      if ($0 ~ /;/) { print acc; for (k = 1; k <= n; k++) print held[k]; acc = ""; n = 0 }
      next
    }
    body($0) ~ /^[[:space:]]*(pub(\([^)]*\))?[[:space:]]+)?use[[:space:]]/ && body($0) !~ /;/ { acc = $0; n = 0; next }
    { print }
    END { if (acc != "") { print acc; for (k = 1; k <= n; k++) print held[k] } }'
}

# Every `std::process` path naming more than `ExitCode` and `exit`, as "N: why".
process_paths() {
  awk '
    {
      t = $0; num = substr(t, 1, index(t, ": ") - 1); t = substr(t, index(t, ": ") + 2)
      if (t ~ /(^|[^A-Za-z0-9_])process[[:space:]]+as[[:space:]]/) { print num ": aliases `process`" ; next }
      while (match(t, /(^|[^A-Za-z0-9_])process[[:space:]]*::[[:space:]]*/)) {
        rest = substr(t, RSTART + RLENGTH)
        if (substr(rest, 1, 1) == "{") {
          close_at = index(rest, "}"); group = substr(rest, 2, close_at - 2)
          m = split(group, items, ",")
          for (k = 1; k <= m; k++) {
            item = items[k]; gsub(/^[[:space:]]+|[[:space:]]+$/, "", item)
            if (item == "") continue
            if (item !~ /^(ExitCode|exit)([[:space:]]+as[[:space:]]+[A-Za-z_][A-Za-z0-9_]*)?$/) { print num ": imports `" item "` from `process`"; break }
          }
        } else if (!match(rest, /^(ExitCode|exit)([^A-Za-z0-9_]|$)/)) {
          print num ": names `process::" substr(rest, 1, 24) "`"
        }
        t = rest
      }
    }'
}

scan() {
  local files file hit text
  files="$(git ls-files -- 'crates/*/src/*.rs' 'crates/*/src/**/*.rs' | sort -u)"
  [ -n "$files" ] || { note "no product source under crates/*/src — an empty population is a breach, not a pass"; return; }
  while IFS= read -r file; do
    [ -f "$file" ] || continue
    checked=$((checked + 1))
    text="$(production "$file" | stripped | joined)"
    if printf '%s\n' "$text" | grep -qx '0: UNTERMINATED'; then
      note "$file cannot be read: it ends inside a comment or a string, so nothing after its opening can be vouched for"
      continue
    fi
    while IFS= read -r hit; do
      [ -n "$hit" ] || continue
      note "$file:${hit%%:*} spawns or names a subprocess in production code:${hit#*:}"
    done < <(printf '%s\n' "$text" | grep -E -- "$SPAWN")
    while IFS= read -r hit; do
      [ -n "$hit" ] || continue
      note "$file:${hit%%:*} reaches std::process beyond ExitCode and exit:${hit#*:}"
    done < <(printf '%s\n' "$text" | process_paths)
    while IFS= read -r hit; do
      [ -n "$hit" ] || continue
      note "$file:${hit%%:*} declares a foreign function (an extern block or #[link]), the route to execve or posix_spawn:${hit#*:}"
    done < <(printf '%s\n' "$text" | grep -E -- "$FOREIGN")
    while IFS= read -r hit; do
      [ -n "$hit" ] || continue
      note "$file:${hit%%:*} holds inline assembly, which can make a system call:${hit#*:}"
    done < <(printf '%s\n' "$text" | grep -E -- "$ASSEMBLY")
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
  # The three routes, each closed by structure (leaf `PROGRAM.105`): every arm below passed the name patterns alone.
  fresh; printf 'use std::{process::{Command as C}};\npub fn r() { let _ = C::new("x").output(); }\n' >> "$work/crates/eng/src/lib.rs"
  arm "an aliased Command in a nested use group is refused" 1 "lib.rs:4 reaches std::process beyond ExitCode and exit"
  fresh; printf 'use std::{\n    process::{\n        Command,\n    },\n};\n' > "$work/crates/eng/src/io.rs"
  arm "a use group rustfmt splits over lines is read whole" 1 "io.rs:1 reaches std::process"
  fresh; printf 'use std::process as p;\n' > "$work/crates/eng/src/io.rs"
  arm "process aliased is refused" 1 "aliases \`process\`"
  fresh; printf 'use std::process::*;\n' > "$work/crates/eng/src/io.rs"
  arm "process glob-imported is refused" 1 "names \`process::*"
  fresh; printf 'extern "C" { fn execve(p: *const u8) -> i32; }\n' > "$work/crates/eng/src/io.rs"
  arm "an extern block declaring execve is refused" 1 "io.rs:1 declares a foreign function"
  fresh; printf 'unsafe extern "C" {\n    fn posix_spawn() -> i32;\n}\n' > "$work/crates/eng/src/io.rs"
  arm "an unsafe extern block declaring posix_spawn is refused" 1 "io.rs:1 declares a foreign function"
  fresh; printf '#[link(name = "c")]\npub struct L;\n' > "$work/crates/eng/src/io.rs"
  arm "a #[link] attribute is refused" 1 "io.rs:1 declares a foreign function"
  fresh; printf 'pub fn s() { unsafe { core::arch::asm!("ecall") } }\n' > "$work/crates/eng/src/io.rs"
  arm "inline assembly, a system call's route, is refused" 1 "io.rs:1 holds inline assembly"
  fresh; printf 'pub extern "C" fn exported() -> usize { 0 }\nuse std::process::{ExitCode, exit};\n' > "$work/crates/eng/src/io.rs"
  arm "an extern \"C\" fn definition and process::{ExitCode, exit} pass" 0 ""
  fresh; printf '%s\n' '// Command::new, extern "C" { and asm!( in a comment' \
    'pub const M: &str = "Command::new, extern \"C\" { and asm!(";' \
    'pub const R: &str = r#"process::Command "quoted" asm!("#;' \
    '/* Command::new /* nested */ still a comment, asm!( */' > "$work/crates/eng/src/io.rs"
  arm "what comments and string literals name is not code" 0 ""
  fresh; printf '%s\n' "pub fn q() { let _ = '\"'; let _ = std::process::Command::new(\"x\"); }" > "$work/crates/eng/src/io.rs"
  arm "a character literal holding a quote opens no string" 1 "io.rs:1 spawns"
  fresh; printf '%s\n' "pub fn f<'a>(x: &'a str) -> &'a str { let _ = std::process::Command::new(\"x\"); let _ = 'z'; x }" > "$work/crates/eng/src/io.rs"
  arm "a lifetime is no character literal" 1 "io.rs:1 spawns"
  fresh; printf 'pub const S: &str = "never closed;\n' > "$work/crates/eng/src/io.rs"
  arm "a file ending inside a string cannot be vouched for" 1 "io.rs cannot be read"
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
