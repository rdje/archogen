#!/usr/bin/env bash
# scripts/language_baseline.sh — LANGUAGE-BASELINE: the frozen-construct baseline of `eadl/1`.
#
# ⭐ WHY THIS EXISTS. `ROADMAP.md` §15 promises that "any changed behavior must be explicit", and §12
# M1's exit gate asks for a frozen compatibility baseline. A baseline somebody retypes is a baseline
# nobody trusts, so this script *derives* it: one digest per frozen construct, computed from the
# documents and descriptions themselves.
#
# WHAT IS FROZEN — enumerated at run time by the instrument, never listed here or in it:
#   • `docs/semantics/grammar.md`'s EBNF fence (the normative surface syntax)
#   • every machine-read table `docs/semantics/reference.md` carries (the executed rules)
#   • the **canonical form** of every description `docs/semantics/conformance.md` declares
#
# ⛔ Canonical form, not file bytes, and the reason is a gate that would otherwise get disabled: a
# corpus file's comment header carries its `case:`, `why:` and `rationale:` prose, so digesting bytes
# would make every documentation fix look like a change to a frozen construct. Canonical form is the
# language's own normative printer and drops comments, so a digest moves exactly when the described
# system moves — and it is the same artifact §12 M4 hashes, so the baseline and the build agree on what
# "the same description" means instead of maintaining two definitions of it.
#
# ⚠️ HONEST LIMIT, stated rather than hidden: canonical form retains no comment, so a change to a
# corpus file's *header* is invisible here. The baseline proves the described systems did not move, not
# that the files did not. The headers that carry data are pinned elsewhere — by `corpus.rs` against
# `docs/semantics/boundary/README.md`'s counts and by `reference.rs` against the reference's
# `comment-headers` table.
#
# ⛔ THIS SCRIPT WRITES THE BASELINE AND DOES NOT GATE IT. Comparing a fresh run against the tracked
# file, requiring a migration note for any movement, and refusing a note-less regeneration are leaf
# `M1.13.5`'s, which registers `scripts/check_language_freeze.sh` as a doctrine. A gate written in the
# same commit as its baseline has no prior state to differ from, so no RED arm can fire against the real
# tree — which is why the two are separate commits and not one.
#
# CONTRACT: exit code is the verdict; explains on stderr; deterministic. Scratch lives under
# `target/tmp/`, on the repository's own volume (§13 of the standing instructions), never in `/tmp`.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

OUT="docs/semantics/BASELINE.txt"
SCRATCH="target/tmp/language-baseline"

usage() {
  cat >&2 <<'TEXT'
usage: scripts/language_baseline.sh --emit | --print | --list
  --emit    rewrite docs/semantics/BASELINE.txt (an explicit act, never a side effect)
  --print   write the baseline to stdout and touch nothing
  --list    the construct ids only, one per line
TEXT
}

# Portable SHA-256 of stdin: coreutils on Linux, BSD/perl shasum on macOS. The same helper
# scripts/check_frozen_evaluation.sh uses, because the workspace carries zero dependencies
# (docs/decisions/decision_zero-dependency-engine-core.md) and so has no hasher of its own.
sha256_of_stdin() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum | cut -d' ' -f1
  elif command -v shasum >/dev/null 2>&1; then shasum -a 256 | cut -d' ' -f1
  else
    echo "LANGUAGE-BASELINE: no sha256 tool available (sha256sum or shasum)" >&2
    return 1
  fi
}

# ── the constructs, from the one instrument ────────────────────────────────────────────────────────
# The instrument is Rust because only the frontend can print a description's canonical form, and only
# the manifest's own reader knows the population. The shell's job is the part it does portably: hashing.
instruments() {
  mkdir -p "$SCRATCH" || return 1
  if ! cargo run -q -p eadl-front --example language_freeze -- --texts > "$SCRATCH/constructs.bin"; then
    echo "LANGUAGE-BASELINE: the instrument failed, so there is nothing to digest" >&2
    echo "  a construct that cannot be read is a defect, not a construct to skip" >&2
    return 1
  fi
}

# `<sha256>  <construct-id>` per construct, sorted by id so two runs are byte-comparable.
#
# ⛔ Built by appending, not by `records+="$(…)"`: command substitution strips the trailing newline, so
# a record assembled inside one loses the line break that separates it from the next, and the whole
# baseline arrives at `sort` as a single line. Measured, not anticipated — the first cut produced exactly
# that, and `sort -k2` on one line returned one line, which looked like a very long digest.
baseline() {
  local id text digest records=""
  while IFS= read -r -d '' id && IFS= read -r -d '' text; do
    digest="$(printf '%s' "$text" | sha256_of_stdin)" || return 1
    records+="$digest  $id"$'\n'
  done < "$SCRATCH/constructs.bin"
  if [ -z "$records" ]; then
    echo "LANGUAGE-BASELINE: the instrument emitted no construct at all, so a baseline written now" >&2
    echo "  would freeze nothing and look like a clean run" >&2
    return 1
  fi
  printf '%s' "$records" | LC_ALL=C sort -k2
}

header() {
  cat <<TEXT
# eadl/1 — the frozen-construct baseline. GENERATED by scripts/language_baseline.sh; do not edit.
#
# format:     <sha256 of the construct's text>  <construct id>
# constructs: docs/semantics/grammar.md's EBNF fence, every machine-read table in
#             docs/semantics/reference.md, and the canonical form of every description
#             docs/semantics/conformance.md declares — enumerated at run time, never listed,
#             because a list is a figure nothing re-derives.
# regenerate: scripts/language_baseline.sh --emit     (an explicit act, never a side effect)
# inspect:    scripts/language_baseline.sh --print
# limit:      canonical form carries no comment, so a corpus file's header is invisible here; this
#             proves the described systems did not move, not that the files did not.
# gate:       leaf M1.13.5 owns comparing this file against a fresh run and requiring a migration
#             note for any movement. This script writes the baseline and does not enforce it.
TEXT
}

case "${1:-}" in
  --emit|--print)
    instruments || exit 1
    body="$(baseline)" || exit 1
    if [ "${1}" = "--print" ]; then
      printf '%s\n%s\n' "$(header)" "$body"
      exit 0
    fi
    printf '%s\n%s\n' "$(header)" "$body" > "$OUT" || exit 1
    printf 'LANGUAGE-BASELINE: wrote %s (%s construct(s))\n' "$OUT" "$(printf '%s\n' "$body" | wc -l | tr -d ' ')"
    ;;
  --list)
    exec cargo run -q -p eadl-front --example language_freeze -- --list
    ;;
  *)
    usage
    exit 2
    ;;
esac
