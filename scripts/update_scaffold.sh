#!/usr/bin/env bash
# scripts/update_scaffold.sh — pull the latest project-NEUTRAL spine from the bedrock
# template into THIS project, WITHOUT touching your roadmap, task-trees, decisions, or code.
#
#   scripts/update_scaffold.sh <bedrock-repo-url-or-local-path>
#
# ⛔⛔ THIS TOOL NEVER OVERWRITES. It used to, and that was wrong — measured, not feared.
#
# The premise was that the files below "never carry project content". Across the three
# projects created from this template, that premise is FALSE for most of them:
# `docs/TASK_TREE.md` diverges by 20-30 lines (it indexes the project's OWN trees),
# `TOOLBOX.md` by 27-616, `check_task_acceptance.sh` by up to 447, `check_readme_stability.sh`
# by 217. A "pull the latest spine" run silently replaced a mature project's hardened checks
# with the template's simpler ones, and `make gate` PASSED afterwards — because template
# files satisfy template checks, so the advertised safety net cannot see this class at all.
#
# It happened: 2026-09-21, a run in one project wiped its task-tree index (11 registered
# trees), its four own doctrine rows, its tool registry and its tiered commit workflow. It was
# recoverable only because nothing had been committed yet.
#
# ⭐ AND THE IMPROVEMENT OFTEN FLOWS THE OTHER WAY. `MAINTAINING.md` records that transfer
# runs BOTH ways: a mature project hardens a check and sends it back here. Overwriting that
# project from here is backwards.
#
# So: identical → nothing. Absent → seeded. Different → the incoming version is written
# into `.bedrock-incoming/<path>` and reported, and you merge deliberately. Nothing you have
# is ever modified, renamed or deleted.
# Everything else project-owned
# (CLAUDE.md, README.md, ROADMAP.md, the live-docs, the project doctrine slot, the curated
# subsystems.md, and all of docs/tasks/ + docs/decisions/ records) is deliberately left
# alone. After syncing: review `git diff`, run `make gate`, and commit.
set -euo pipefail
URL="${1:-}"
[ -n "$URL" ] || { echo "usage: scripts/update_scaffold.sh <bedrock-repo-url-or-local-path> [--merge] [--force]" >&2; exit 2; }
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

# ⛔ archogen's one change to upstream's file (bedrock `5af0c1c`, adopted by leaf `PROGRAM.26`): scratch on this
# repository's own volume, never `$TMPDIR` (leaf `PROGRAM.29`, `SCRATCH-LOCALITY`). The file is not in its own
# NEUTRAL array, so it is project-owned, and a later adoption must carry this hunk forward.
mkdir -p "$ROOT/target/doctrine_scratch"
tmp="$(mktemp -d "$ROOT/target/doctrine_scratch/update_scaffold.XXXXXX")"; trap 'rm -rf "$tmp"' EXIT
if [ -d "$URL/.git" ]; then
  cp -R "$URL" "$tmp/bedrock"
else
  git clone --depth 1 "$URL" "$tmp/bedrock" >/dev/null 2>&1 || { echo "clone failed: $URL" >&2; exit 1; }
fi

# The project-NEUTRAL spine — safe to overwrite because it never carries project content.
NEUTRAL=(
  MEMORY_ARCHITECTURE.md
  DOCTRINE_ENFORCEMENT.md
  TOOLBOX.md
  README_POLICY.md
  COMMIT.md
  AGENTS.md
  docs/TASK_TREE.md
  docs/TASK_TREE_README.md
  docs/tasks/TEMPLATE.md
  docs/decisions/TEMPLATE.md
  .githooks/pre-commit
  .githooks/commit-msg
  scripts/check_doctrines.sh
  scripts/check_memory_architecture.sh
  scripts/check_live_doc_currency.sh
  scripts/check_no_background_jobs.sh
  scripts/check_lesson_promotion.sh
  scripts/check_routing_evidence.sh
  scripts/check_gap_claims.sh
  scripts/check_table_arity.sh
  scripts/check_readme_stability.sh
  scripts/check_waiver_routing.sh
  scripts/check_task_acceptance.sh
  .doctrine/README.md
  scripts/check_docpaths.sh
  scripts/check_task_tree_ownership.sh
  knowledge-map/scripts/gen_knowledge_map.sh
  knowledge-map/scripts/check_knowledge_map.sh
  DOCTRINE_VERSION
)

# ⛔ SEEDED ONCE, NEVER OVERWRITTEN — a spine file that carries a PROJECT DECISION.
#
# `NEUTRAL` above is blind-overwrite, which is correct for a file that never holds project
# content. A file that a project is MEANT to edit cannot be in it: re-syncing would silently
# revert that project's decision, and the more deliberate the decision, the worse the loss.
#
# But such a file still has to REACH a project that predates it, or a new spine rule lands
# only in projects created afterwards — which is exactly the gap this category was added to
# close (`BEDROCK-MAINTENANCE.2.9` follow-up): `VISIBILITY.md` carries a declared posture a
# project may change, so it is copied when ABSENT and left alone when present.
SEED_ONCE=(
  VISIBILITY.md
)

# ⛔ Refuse on a dirty tree, and this is the load-bearing guard rather than a courtesy.
# Committed content survives anything this tool could do. UNCOMMITTED content survives
# nothing: it is in no object, no reflog, no stash — only in the working file, so a single
# write ends it. That is not hypothetical (`BEDROCK-MAINTENANCE.2.10`). `--force` exists for
# someone who has read this sentence and accepts it.
MERGE=0; FORCE=0
for a in "$@"; do
  case "$a" in
    --merge) MERGE=1 ;;
    --force) FORCE=1 ;;
  esac
done

if [ "$FORCE" != "1" ] && [ -n "$(git status --porcelain)" ]; then
  echo "REFUSED: the working tree is dirty." >&2
  echo "  Commit or stash first." >&2
  echo "  ⛔ WHY THIS REFUSES RATHER THAN WARNS: committed content is always recoverable." >&2
  echo "     UNCOMMITTED content is recoverable by NOTHING — not git checkout, not the" >&2
  echo "     reflog, not a dangling blob, not fsck. It exists only in the working file." >&2
  echo "     This tool writes files. Meeting your uncommitted work is the one failure it" >&2
  echo "     cannot let you undo, so it declines to be in the room with it." >&2
  echo "  Deliberate override: scripts/update_scaffold.sh <url> --force" >&2
  echo "  (--force skips THIS check only. Nothing can make this tool overwrite a file.)" >&2
  exit 2
fi

n=0; s=0; d=0; m=0
SEEDED=()

# ⛔⛔ ASK, MERGE INTO A SIDE FILE, NEVER INTO YOURS.
#
# Maintainer instruction, 2026-09-21: *"shall not update files that are different. Worst
# case it shall ask to merge, never overwrite, never."* So this is the most this tool will
# ever do to a file you have changed: ask, compute a THREE-WAY merge, and write the result
# to `.bedrock-incoming/<path>.merged`. Your file is not read-modified-written, not renamed,
# not deleted. Taking the merge is a copy YOU run, after reading it.
#
# ⭐ The merge is a real three-way, not a guess: the common ancestor is the template version
# this project last synced from, resolved from its own recorded DOCTRINE_VERSION via the
# commit that introduced that version upstream. Without a base, a "merge" of two files is
# just a diff with opinions, so when the base cannot be resolved this says so and offers the
# diff instead.
offer_merge() {
  local f="$1" theirs="$2" ver base_rev bt ot rc
  ver="$(cat DOCTRINE_VERSION 2>/dev/null || true)"
  base_rev="$(cd "$tmp/bedrock" && git log --format=%h -S"$ver" -- DOCTRINE_VERSION 2>/dev/null | tail -1)"

  if [ -z "$ver" ] || [ -z "$base_rev" ]; then
    echo "           no common ancestor resolvable (recorded version: ${ver:-none}) — a two-way"
    echo "           merge would be guesswork. Compare instead:  diff $f .bedrock-incoming/$f"
    return 0
  fi

  printf '           merge theirs into a COPY of yours, 3-way from %s? [y/N] ' "$base_rev"
  local reply=""
  read -r reply </dev/tty 2>/dev/null || reply=""
  case "$reply" in [yY]*) ;; *) echo "           skipped — nothing written"; return 0 ;; esac

  bt="$tmp/base.$$"; ot="$tmp/ours.$$"
  if ! (cd "$tmp/bedrock" && git show "$base_rev:$f" 2>/dev/null) > "$bt" || [ ! -s "$bt" ]; then
    echo "           $f did not exist at $base_rev — no base, so no merge. Compare instead."
    rm -f "$bt"; return 0
  fi
  cp "$f" "$ot"
  git merge-file -p --diff3 "$ot" "$bt" "$theirs" > ".bedrock-incoming/$f.merged"; rc=$?
  rm -f "$bt" "$ot"
  if [ "$rc" -eq 0 ]; then
    echo "           ✓ merged cleanly → .bedrock-incoming/$f.merged (yours is still untouched)"
  else
    echo "           ⚠ merged with $rc conflict(s) → .bedrock-incoming/$f.merged — resolve the"
    echo "             <<<<<<< markers there. Yours is still untouched."
  fi
  echo "             Take it only after reading it:  cp .bedrock-incoming/$f.merged $f"
  m=$((m+1))
}

apply_one() { # $1 = path, $2 = "seed-only" | "offer"
  local f="$1" mode="$2" src="$tmp/bedrock/$1"
  [ -f "$src" ] || return 0
  if [ ! -f "$f" ]; then
    mkdir -p "$(dirname "$f")"
    cp "$src" "$f"
    SEEDED+=("$f")
    echo "  seeded   $f (new)"
    s=$((s+1))
    return 0
  fi
  if cmp -s "$src" "$f"; then
    n=$((n+1))
    return 0
  fi
  if [ "$mode" = "seed-only" ]; then
    echo "  kept     $f (yours — it carries a decision this project owns)"
    return 0
  fi
  # ⛔ The incoming copy goes into ONE directory, never beside the original. A
  #    `<file>.bedrock-new` sitting next to `<file>` is one `git add -A` away from being
  #    committed as if it were project content, and one careless glance away from being
  #    mistaken for the real file. `.bedrock-incoming/` is a single thing to read and a
  #    single thing to delete.
  mkdir -p ".bedrock-incoming/$(dirname "$f")"
  cp "$src" ".bedrock-incoming/$f"
  echo "  DIFFERS  $f — yours is UNTOUCHED; theirs is .bedrock-incoming/$f"
  d=$((d+1))
  [ "$MERGE" = "1" ] && offer_merge "$f" "$src"
  return 0
}

for f in "${SEED_ONCE[@]}"; do apply_one "$f" "seed-only"; done
for f in "${NEUTRAL[@]}"; do apply_one "$f" "offer"; done
# ⛔ chmod ONLY what was seeded. This used to be a blanket
#    `chmod +x scripts/*.sh knowledge-map/scripts/*.sh .githooks/*`, which changes the mode of
#    every script in the project including ones this run never looked at. A mode change is a
#    change: it shows up in `git status`, it lands in a commit, and it is exactly the class of
#    "touched a file I did not ask you to touch" this tool exists to not do.
for f in "${SEEDED[@]}"; do
  case "$f" in *.sh|.githooks/*) chmod +x "$f" 2>/dev/null || true ;; esac
done

echo "✓ $n already current, $s seeded, $d differ, $m merged to a side file — nothing of yours was modified."
if [ "$d" -gt 0 ]; then
  echo "  ⚠️  NOTHING of yours was touched. The template's versions are in .bedrock-incoming/."
  echo "      Review them, take only what you want, then:  rm -rf .bedrock-incoming"
  echo "        diff -ru . .bedrock-incoming 2>/dev/null | less     # or file by file"
  echo "      Improvement often flows project -> bedrock, so the template's version is NOT"
  echo "      automatically the better one (MAINTAINING.md: transfer runs both ways)."
fi
echo "  Review 'git status', run 'make gate', then commit."
