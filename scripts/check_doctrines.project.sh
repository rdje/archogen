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
