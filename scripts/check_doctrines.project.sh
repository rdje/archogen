#!/usr/bin/env bash
# scripts/check_doctrines.project.sh — THE PROJECT-SPECIFIC DOCTRINE SLOT.
#
# This is where a project instantiated from the template adds ITS OWN mechanizable
# doctrine checks — the equivalent of PGEN's "EBNF is the single source of truth",
# "regex self-hosts", "cert-coverage / shape-contract gates", etc.
#
# It runs LAST in scripts/check_doctrines.sh. Exit 0 = all project doctrines pass;
# exit nonzero (with a message on stderr) = a breach that blocks the commit.
#
# The template ships this as a passing no-op. Add checks below as your project grows;
# keep each one cheap, deterministic, and self-describing. For anything heavier than a
# few seconds, gate it in CI instead and keep this hook fast.
set -uo pipefail

ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
fail=0

# --- archogen's own doctrines -----------------------------------------------------------------
# Each entry: "ID|what it proves|script". Keep each cheap and deterministic; anything heavier
# than a few seconds belongs in a CI tier (ROADMAP.md §14.3), not in the pre-commit path.
PROJECT_DOCTRINES=(
  "FROZEN-EVALUATION|the sealed evaluation set is unmodified, complete, and unnamed outside its directory|scripts/check_frozen_evaluation.sh"
  "S0-RETIREMENT|every hard-coded S0 assumption is marked in the source, listed with an owning leaf, and the prototype has acquired no new consumers|scripts/check_s0_retirement.sh"
  "BOOK-ANCHORS|every book chapter, and every normative document under docs/semantics/, that describes behavior cites a repository path — and every path either of them cites exists|scripts/check_book_anchors.sh"
  "FEEDBACK-SELF-CONTAINED|every reported bug's directory stands alone — complete, closed, portable, and in its vendor register|scripts/check_feedback_self_contained.sh"
  "LANGUAGE-FREEZE|no construct of eadl/1 moved without a migration note — the baseline agrees with the working tree, and amending it is an explicit act|scripts/check_language_freeze.sh"
  "BOOK-COVERAGE|every workspace member is named in a book chapter that also cites a path inside it — the mirror of BOOK-ANCHORS|scripts/check_book_coverage.sh"
  "REPOSITORY-BOUNDARY|every other repository is read-only — each vendored checkout this repository pins is at its pin, with no local commit, no modified file and no created file|scripts/check_repository_boundary.sh"
  "SCRATCH-LOCALITY|nothing this repository owns puts scratch off its volume — every temporary file or directory is made under target/, and no script names the system temporary directory|scripts/check_scratch_locality.sh"
  "SOURCE-LEDGER|every external source the repository pins has a ledger entry carrying the pinned version, and every chapter or decision naming a ledgered source cites its entry|scripts/check_source_ledger.sh"
  "VERSION-REGISTER|every version the code declares — format identifiers, version constants, profile ids, the engine version — is an entry of the register at that value, and every entry still has its declaration|scripts/check_version_register.sh"
  "FEEDBACK-REGISTER|every outbound bug register agrees with its issues — each row's State and Severity, a verified row backed by a dated re-measurement — and its totals and the cross-vendor index are recounts|scripts/check_feedback_register.sh"
  "LIVE-SNAPSHOTS|every live document that shows current state stays within its ceilings on lines, bytes and longest line, and every snapshot the inventory declares is bounded|scripts/check_live_snapshots.sh"
  "STATED-ORDER|every restated order agrees with its source — frontier tables with their leaves, snapshot heads and successor lists with their trees, the changelog newest-first|scripts/check_stated_order.sh"
  "FIGURE-REGISTER|a commit may not add an unclassified figure to a live document — gated by a test that reads it, a dated record, or not a count, per docs/figures.md|scripts/check_figure_register.sh"
  "NO-SUBPROCESS|the product spawns no process and executes nothing — no production code under crates/*/src names Command, spawn, exec or fork (§10.4)|scripts/check_no_subprocess.sh"
)

for entry in "${PROJECT_DOCTRINES[@]}"; do
  id="${entry%%|*}"; rest="${entry#*|}"; what="${rest%%|*}"; script="${rest##*|}"
  if [ ! -x "$script" ]; then
    echo "PROJECT: $id — $script is missing or not executable" >&2
    fail=1
    continue
  fi
  if ! "$script"; then
    echo "PROJECT: $id breach — $what" >&2
    fail=1
  fi
done

exit $fail
