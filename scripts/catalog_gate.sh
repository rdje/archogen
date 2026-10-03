#!/usr/bin/env bash
# scripts/catalog_gate.sh — the catalog's local gate: the checker run from the hooks (leaf `M2.7.4.4`;
# `docs/specs/catalog/decision_catalog-records.md` §4 and §9).
#
# ⭐ WHY THIS EXISTS. The checker, `cargo xtask catalog-check`, judges a commit's catalog against its bases; the hooks
# are where a commit is still a pending one. This script is what each hook runs, so the three hooks share one
# reading of what is judged and when:
#   - `pre-commit` (no argument): the pending commit from the index, its parents `HEAD` and, during a merge made with
#     `git merge --no-commit`, `MERGE_HEAD`. A refusal blocks the commit. With `ARCHOGEN_BLESS_CATALOG=1` the checker
#     first writes and stages the lock as §9's blessing, then judges the index it staged;
#   - `--merge` (from `pre-merge-commit`): the same judgment, advisory. Measured `2026-10-03` on git 2.54.0: that hook
#     runs before git writes `MERGE_HEAD`, so the gate sees one parent, and `post-commit` does not run for a merge's
#     own commit. The verdict is printed and the merge proceeds; a catalog merge is made with `git merge --no-commit`
#     and then `git commit`, whose `pre-commit` sees both parents (§4);
#   - `--post-commit`: the commit as made, against its true parents, which an amend's `pre-commit` cannot see — it is
#     given the replaced commit as parent. A mismatch is reported at once, loudly; the repair is to reset to the
#     commit the amend replaced. A hook after the commit cannot block, so it exits 0.
#
# WHEN THE CHECKER RUNS. With no tracked file under `catalog/` and nothing untracked there, the catalog is empty and
# nothing is locked or built, so the gate passes without building `xtask`. Otherwise the checker runs when the
# commit touches `catalog/`, `targets/`, a manifest, `Cargo.lock`, a cargo configuration, a toolchain file, or
# anything under a package directory (`crates/`, `xtask/`) — the paths §3 says are built for — or when something
# untracked sits under `catalog/`, which the checker refuses (`catalog-layout`).
#
# THE CHECKER is the repository's own: `cargo run -q -p xtask -- catalog-check …`, built from the working tree.
# `ARCHOGEN_CATALOG_CHECKER=<program>` names another, for this script's self-test alone: the local gate is advisory
# by design — premise 3's protected check is CI's replay of every commit as it was made (§9) — so an override here
# weakens nothing a claim rests on.
#
# ⚠️ HONEST LIMIT: a rebase, a cherry-pick, `am` and a revert make commits without running these hooks (§4); CI's
# replay judges them. The post-commit run reports; it cannot undo.
#
# usage: bash scripts/catalog_gate.sh [--merge | --post-commit]
#        bash scripts/catalog_gate.sh --self-test
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac

# Paths whose change the checker judges (§3, "Who runs these checks").
relevant() {
  grep -E '^(catalog/|targets/|crates/|xtask/|Cargo\.(toml|lock)$|\.cargo/config|rust-toolchain)|/(Cargo\.(toml|lock)|rust-toolchain(\.toml)?|build\.rs)$' || true
}

checker() { # the checker's arguments follow
  local root; root="$(git rev-parse --show-toplevel)"
  if [ -n "${ARCHOGEN_CATALOG_CHECKER:-}" ]; then
    "$ARCHOGEN_CATALOG_CHECKER" "$@"
  else
    (cd "$root" && cargo run -q -p xtask -- "$@")
  fi
}

gate() {
  local mode="${1:-}" tracked untracked changed rc
  tracked="$(git ls-files --cached -z -- catalog | tr -d '\0' | head -c 1)"
  untracked="$(git ls-files --others -z -- catalog | tr -d '\0' | head -c 1)"
  case "$mode" in
    "" | --merge)
      if [ -z "$tracked" ] && [ -z "$untracked" ]; then
        echo "catalog-gate: no catalog in the index and nothing untracked under catalog/: nothing to judge"
        return 0
      fi
      changed="$(git diff --cached --name-only --diff-filter=ACDMRT | relevant)"
      if [ -z "$changed" ] && [ -z "$untracked" ]; then
        echo "catalog-gate: the pending commit changes nothing the checker judges"
        return 0
      fi
      if [ "$mode" = --merge ]; then
        checker catalog-check --index; rc=$?
        if [ "$rc" -ne 0 ]; then
          echo "catalog-gate: ADVISORY — pre-merge-commit sees one parent (measured, §4); the checker's exit was $rc." >&2
          echo "catalog-gate: make a catalog merge with \`git merge --no-commit\` and then \`git commit\`." >&2
        fi
        return 0
      fi
      if [ "${ARCHOGEN_BLESS_CATALOG:-}" = 1 ]; then
        checker catalog-check --index --bless
      else
        checker catalog-check --index
      fi
      ;;
    --post-commit)
      tracked="$(git ls-tree -r --name-only HEAD -- catalog | head -c 1)"
      if [ -z "$tracked" ]; then
        return 0
      fi
      changed="$(git diff-tree --no-commit-id --name-only -r -m HEAD | relevant)"
      if [ -z "$changed" ]; then
        return 0
      fi
      checker catalog-check --commit HEAD; rc=$?
      if [ "$rc" -ne 0 ]; then
        echo "catalog-gate: MISMATCH — the commit as made, judged against its true parents, was refused (exit $rc)." >&2
        echo "catalog-gate: after an amend, the pre-commit gate saw the replaced commit as parent (§4); reset to the" >&2
        echo "catalog-gate: commit the amend replaced and make the change again. CI's replay will refuse this commit." >&2
      fi
      return 0
      ;;
    *)
      echo "usage: bash scripts/catalog_gate.sh [--merge | --post-commit | --self-test]" >&2
      return 2
      ;;
  esac
}

self_test() {
  local root scratch repo stub log arms=0 ok=0
  root="$(git rev-parse --show-toplevel)"
  scratch="$root/target/doctrine_scratch/catalog_gate"
  rm -rf "$scratch"; mkdir -p "$scratch"
  repo="$scratch/repo"; stub="$scratch/stub.sh"; log="$scratch/stub.log"
  # The stub checker records its arguments and exits as the file `verdict` says.
  printf '#!/bin/sh\nprintf "%%s\\n" "$*" >> "%s"\nexit "$(cat "%s/verdict" 2>/dev/null || echo 0)"\n' "$log" "$scratch" > "$stub"
  chmod +x "$stub"
  g() { git -C "$repo" -c user.name=t -c user.email=t@t -c commit.gpgsign=false "$@"; }
  mkdir -p "$repo/catalog/experimental" "$repo/docs"
  g init -q
  printf 'doc\n' > "$repo/docs/a.md"; g add -A; g commit -q -m base
  arm() { # $1 = name, $2 = expected rc, $3 = the stub's arguments the log must hold, or `-` for no run, then the mode
    local name="$1" want="$2" must="$3" rc out; shift 3
    arms=$((arms + 1)); : > "$log"
    out="$(cd "$repo" && ARCHOGEN_CATALOG_CHECKER="$stub" bash "$SELF" "$@" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2; return
    fi
    if [ "$must" = - ]; then
      if [ -s "$log" ]; then echo "SELF-TEST: $name — the checker ran: $(cat "$log")" >&2; return; fi
    elif ! grep -qF -- "$must" "$log"; then
      echo "SELF-TEST: $name — the checker did not run as \`$must\`: $(cat "$log")" >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  echo 0 > "$scratch/verdict"
  printf 'doc\n' >> "$repo/docs/a.md"; g add -A
  arm "no catalog and nothing untracked under it: nothing to judge, no build" 0 -
  printf '(catalog-record stray)\n' > "$repo/catalog/experimental/stray.catalog"
  arm "something untracked under catalog/: the checker judges the index" 0 "catalog-check --index"
  g add -A
  arm "a record staged: the checker judges the index" 0 "catalog-check --index"
  echo 1 > "$scratch/verdict"
  arm "the checker's refusal blocks the commit" 1 "catalog-check --index"
  arm "from pre-merge-commit the verdict is advisory" 0 "catalog-check --index" --merge
  echo 0 > "$scratch/verdict"
  : > "$log"
  arms=$((arms + 1))
  if (cd "$repo" && ARCHOGEN_CATALOG_CHECKER="$stub" ARCHOGEN_BLESS_CATALOG=1 bash "$SELF" >/dev/null 2>&1) && grep -qF -- "catalog-check --index --bless" "$log"; then
    ok=$((ok + 1)); echo "  ✅ ARCHOGEN_BLESS_CATALOG=1 asks the checker to bless"
  else
    echo "SELF-TEST: blessing — the checker was not asked to bless: $(cat "$log")" >&2
  fi
  g commit -q -m record
  printf 'doc\n' >> "$repo/docs/a.md"; g add -A; g commit -q -m doc
  arm "post-commit after a commit that changes nothing judged: no run" 0 - --post-commit
  printf '(catalog-record stray2)\n' > "$repo/catalog/experimental/stray2.catalog"; g add -A; g commit -q -m record2
  echo 1 > "$scratch/verdict"
  arm "post-commit reports a mismatch and cannot block" 0 "catalog-check --commit HEAD" --post-commit
  g rm -q -r --cached catalog >/dev/null; g commit -q -m "catalog gone"
  rm -rf "$repo/catalog"
  arm "post-commit with no catalog at HEAD: no run" 0 - --post-commit
  echo "catalog-gate self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test ;;
  *) gate "${1:-}" ;;
esac
