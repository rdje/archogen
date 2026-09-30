#!/usr/bin/env bash
# scripts/wasm_binding.sh — the wasm binding checked as an artifact (leaf `API.5.3`,
# docs/decisions/decision_wasm-binding.md §7 and §8). The `wasm-binding` step of the `integration` tier.
#
# ⭐ WHAT IT PROVES. The binding's judgement is the engine API's, which the rest of the suite tests. What is new is
# the compilation to WebAssembly, the transport and the loader, so this checks exactly those, on the artifact a
# page loads:
#   1. the artifact imports nothing, and exports exactly the record's functions, its memory and the linker's two
#      globals — read by the platform's own `WebAssembly.Module`, not from the source;
#   2. for every description in the population, the artifact's response, reached through
#      crates/archogen-wasm/js/archogen.mjs as a page reaches it, is byte-identical to the host build's
#      (`cargo run -p archogen-wasm --example answers`);
#   3. for every description, the response's `exit` equals `archogen check`'s exit code for the same file;
#   4. the book's transcript of the page (docs/book/src/engine-api.md, the blocks after its
#      `wasm-page-transcript` marker) is what the page's own logic shows for that input (leaf `API.5.4`);
#   5. the page's own wiring, run against a stand-in document, fetches this artifact by its relative URL and shows
#      its default description's answer on load, and the transcript input's answer after Check.
# THE POPULATION is derived on every run: every tracked `*.eadl` outside `docs/feedback/` (the vendors' own
# reproduction inputs).
#
# CONTRACT: exit 0 all three hold · 1 a leg failed, naming the description · 2 a build or usage failure, a
# toolchain without the `wasm32-unknown-unknown` target included · 20 unavailable: no `node`, which is never a pass. `--self-test` runs the
# RED arms in target/doctrine_scratch/wasm_binding/.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

SCRATCH="$ROOT/target/doctrine_scratch/wasm_binding"
WASM="$ROOT/target/wasm32-unknown-unknown/release/archogen_wasm.wasm"
note() { printf 'wasm-binding: %s\n' "$1" >&2; }

# Compare the three sides, one line per description in the list's order: $1 the list, $2 the host build's
# responses, $3 the artifact's, $4 `archogen check`'s exit codes. Prints each disagreement; exit 1 on any.
compare() {
  awk -v native="$2" -v wasm="$3" -v cli="$4" '
    {
      path = $0
      if ((getline n < native) <= 0) { print "the host build gave no response for " path; bad++; next }
      if ((getline w < wasm) <= 0) { print "the artifact gave no response for " path; bad++; next }
      if ((getline c < cli) <= 0) { print "archogen check gave no exit code for " path; bad++; next }
      if (n != w) { print path ": the artifact'"'"'s response differs from the host build'"'"'s"; bad++ }
      exit_code = w
      if (match(exit_code, /"exit":-?[0-9]+/)) exit_code = substr(exit_code, RSTART + 7, RLENGTH - 7); else exit_code = "none"
      if (exit_code != c) { print path ": the response says exit " exit_code ", and archogen check exits " c; bad++ }
      count++
    }
    END {
      if ((getline extra < wasm) > 0) { print "the artifact gave more responses than there are descriptions"; bad++ }
      if (bad) exit 1
      printf "%d description(s)\n", count
    }' "$1"
}

CHAPTER="docs/book/src/engine-api.md"

# The page transcript of chapter $1: its input into $2 and its answer into $3, from the first two fenced blocks
# after the `wasm-page-transcript` marker. Exit 1 when the chapter shows none.
page_transcript() {
  awk -v input="$2" -v answer="$3" '
    /<!-- wasm-page-transcript/ { marked = 1; next }
    marked && /^```/ {
      if (inside) { inside = 0; blocks++; if (blocks == 2) exit; next }
      inside = 1; next
    }
    marked && inside { print > (blocks == 0 ? input : answer) }
    END { if (blocks < 2) exit 1 }' "$1"
}

# Compare the page's answer $2 with the chapter's $1; print the difference and exit 1 when they differ.
compare_page() {
  if diff -u "$1" "$2" >/dev/null; then return 0; fi
  echo "the book's page transcript ($CHAPTER) is not what the page shows for its input:"
  diff -u "$1" "$2" | sed -n '3,12p'
  return 1
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  rm -rf "$work"; mkdir -p "$work"
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
  local line='{"format":"archogen-wasm-response/1","status":"ok","exit":0}'
  printf 'a.eadl\nb.eadl\n' > "$work/list"
  printf '%s\n%s\n' "$line" "${line/\"exit\":0/\"exit\":10}" > "$work/native"
  cp "$work/native" "$work/wasm"
  printf '0\n10\n' > "$work/cli"
  arm "sides that agree on every description pass" 0 "2 description(s)" compare "$work/list" "$work/native" "$work/wasm" "$work/cli"
  printf '%s\n%s\n' "$line" "${line/ok/okay}" > "$work/wasm-differs"
  arm "a response that differs from the host build's is refused, naming the description" 1 \
    "b.eadl: the artifact's response differs" compare "$work/list" "$work/native" "$work/wasm-differs" "$work/cli"
  printf '0\n1\n' > "$work/cli-differs"
  arm "an exit that differs from archogen check's is refused" 1 \
    "b.eadl: the response says exit 10, and archogen check exits 1" compare "$work/list" "$work/native" "$work/wasm" "$work/cli-differs"
  head -1 "$work/wasm" > "$work/wasm-short"
  arm "a description the artifact did not answer is refused" 1 \
    "the artifact gave no response for b.eadl" compare "$work/list" "$work/native" "$work/wasm-short" "$work/cli"
  printf 'text\n<!-- wasm-page-transcript -->\n```eadl\n(defblock a.b)\n```\n\nis answered:\n\n```text\nok (exit 0)\n```\n' \
    > "$work/chapter.md"
  arm "a chapter's page transcript is read: its input, then its answer" 0 "" page_transcript "$work/chapter.md" "$work/in" "$work/out"
  arm "the transcript's input is the first block" 0 "(defblock a.b)" cat "$work/in"
  printf 'ok (exit 0)\n' > "$work/shown"
  arm "a page that shows the chapter's answer passes" 0 "" compare_page "$work/out" "$work/shown"
  printf 'invalid-description (exit 10)\n' > "$work/shown-differs"
  arm "a page that shows something else is refused" 1 "is not what the page shows" compare_page "$work/out" "$work/shown-differs"
  printf 'no marker here\n' > "$work/unmarked.md"
  arm "a chapter that shows no page transcript is refused" 1 "" page_transcript "$work/unmarked.md" "$work/in2" "$work/out2"
  if command -v node >/dev/null 2>&1; then
    # Minimal modules written byte by byte: the magic and version, a type section with one `() -> ()`, then
    #   an import of `env.f` (section 2), or
    #   one function (section 3) exported as `x` (section 7) with an empty body (section 10).
    printf '\000asm\001\000\000\000\001\004\001\140\000\000\002\011\001\003env\001f\000\000' > "$work/imports.wasm"
    printf '\000asm\001\000\000\000\001\004\001\140\000\000\003\002\001\000\007\005\001\001x\000\000\012\004\001\002\000\013' \
      > "$work/exports.wasm"
    arm "a module that imports anything is refused" 1 "imports function \`env.f\`" \
      node "$ROOT/scripts/wasm_binding.mjs" inspect "$work/imports.wasm"
    arm "a module that exports what the record does not list, and not what it does, is refused" 1 \
      "exports function \`x\`, which the record does not list" node "$ROOT/scripts/wasm_binding.mjs" inspect "$work/exports.wasm"
  else
    arms=$((arms + 2))
    echo "SELF-TEST: the two inspect arms need node, and node is not on PATH" >&2
  fi
  rm -rf "$work"
  echo "wasm-binding self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") ;;
  *) echo "usage: bash scripts/wasm_binding.sh [--self-test]" >&2; exit 2 ;;
esac

command -v node >/dev/null 2>&1 || { note "UNAVAILABLE — node is not on PATH, so the artifact cannot be run"; exit 20; }
# The target is not optional: `rust-toolchain.toml` lists it, so a toolchain without it is a broken install.
rustup target list --installed 2>/dev/null | grep -qx wasm32-unknown-unknown ||
  { note "the wasm32-unknown-unknown target is not installed, and rust-toolchain.toml lists it"; exit 2; }

mkdir -p "$SCRATCH"
cargo build -q --release -p archogen-wasm --target wasm32-unknown-unknown ||
  { note "the artifact did not build"; exit 2; }
cargo build -q -p archogen-cli || { note "archogen did not build"; exit 2; }
git ls-files -z '*.eadl' | tr '\0' '\n' | grep -v '^docs/feedback/' > "$SCRATCH/list"
[ -s "$SCRATCH/list" ] || { note "the population is empty — nothing tracked to compare"; exit 1; }

# shellcheck disable=SC2046
node scripts/wasm_binding.mjs answer "$WASM" $(cat "$SCRATCH/list") > "$SCRATCH/wasm" || exit 1
# shellcheck disable=SC2046
cargo run -q -p archogen-wasm --example answers -- $(cat "$SCRATCH/list") > "$SCRATCH/native" ||
  { note "the host build could not answer"; exit 2; }
: > "$SCRATCH/cli"
while IFS= read -r path; do
  "$ROOT/target/debug/archogen" check "$path" >/dev/null 2>&1
  echo "$?" >> "$SCRATCH/cli"
done < "$SCRATCH/list"

if ! result="$(compare "$SCRATCH/list" "$SCRATCH/native" "$SCRATCH/wasm" "$SCRATCH/cli")"; then
  printf '%s\n' "$result" | sed 's/^/wasm-binding: /' >&2
  note "the artifact disagrees with the host build or the command line"
  exit 1
fi
page_transcript "$CHAPTER" "$SCRATCH/page-input" "$SCRATCH/page-answer" ||
  { note "$CHAPTER shows no page transcript after a wasm-page-transcript marker"; exit 1; }
node scripts/wasm_binding.mjs show "$WASM" "$SCRATCH/page-input" > "$SCRATCH/page-shown" || exit 1
if ! page="$(compare_page "$SCRATCH/page-answer" "$SCRATCH/page-shown")"; then
  printf '%s\n' "$page" | sed 's/^/wasm-binding: /' >&2
  exit 1
fi
node scripts/wasm_binding.mjs page "$WASM" "$SCRATCH/page-input" || { note "the page's wiring did not answer as its logic does"; exit 1; }
echo "wasm-binding: OK — the artifact imports nothing and exports what the record lists; $result answered byte for byte as the host build answers them, each with archogen check's exit code; the book's page transcript is what the page shows, and the page's wiring shows it ($(node --version))"
exit 0
