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
  "README-ROUTES|every destination the README sends a reader or an author to is registered, classified and bounded — derived from its links and its guards' actual hints, followed onward, each ceiling held or its debt owned by an open leaf|scripts/check_readme_routes.sh"
  "HISTORY-LEDGERS|the changelog and the development notes are rolling ledgers — every sealed segment unchanged against its index row, the index complete and append-only, the order continuous, and each live window below twice its size|scripts/check_history_ledgers.sh"
  "DECISION-INDEX|every decision record, in the folder or in a partition of it, is linked from docs/decisions/INDEX.md by its own path, and every link the index holds resolves — the partition-aware counterpart of the template's flat index check|scripts/check_decision_index.sh"
  "TASK-HISTORY|closed subtrees sealed out of the task trees stay sealed — every sealed file unchanged against its index row, the index complete and append-only, and every sealed leaf with exactly one two-line stub in its tree that links it|scripts/check_task_history.sh"
  "DECISION-HISTORY|settled sections sealed out of decision records stay sealed — every sealed file unchanged against its index row and against its record before the seal, the index complete and append-only across history, and every sealed section with exactly one stub under its own heading|scripts/check_decision_history.sh"
  "BOOK-GLOSSARY|the book's glossary is live — every acronym a chapter uses is defined there, every entry is still used, each section in order|scripts/check_book_glossary.sh"
  "WORKFLOW-TOKENS|every workflow holds a read-only token and keeps no credentials — top-level permissions granting no write scope, none granted by a job, every checkout with persist-credentials false, no trigger that runs with the base repository's token, and YAML it cannot read refused|scripts/check_workflow_tokens.sh"
  "COMMIT-LOG-ROWS|every work-unit commit — HEAD's history and the pending one — has a row in a task tree's Commit Log, beyond a measured backlog that may only shrink|scripts/check_commit_log_rows.sh"
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
