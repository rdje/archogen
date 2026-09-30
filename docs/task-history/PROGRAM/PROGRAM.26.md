- ID: `PROGRAM.26`
  Status: `done`
  Goal: make `make update-scaffold` unable to destroy project content, by adopting the upstream fix
  rather than inventing one — this repository's `scripts/update_scaffold.sh` is two minor versions
  behind `bedrock`, and the version it has `cp`s every neutral spine file straight over the project's
  copy.
  Reproduce / issue: **measured `2026-09-29` by `PROGRAM.19`'s third cleanup run**, which found
  `target/sync-backup-2026-09-21/` — four spine files matching no committed state of this repository —
  and identified them as byte-identical to `bedrock`'s `HEAD` copies. Identifying them is what exposed
  the hazard beside them.

  ```text
  census: cat DOCTRINE_VERSION                                -> bedrock-scaffold 0.8.1
          cat ../bedrock/DOCTRINE_VERSION   (read-only, §21)  -> bedrock-scaffold 0.10.0
  census: grep -n 'cp "\$tmp/bedrock/\$f"' scripts/update_scaffold.sh
          -> one unconditional `cp` per NEUTRAL file, 25 entries in the array [29, re-measured 2026-09-30], no comparison and no
             refusal; the script's own header calls them "safe to overwrite because it never carries
             project content"
  census: the array includes docs/TASK_TREE.md, COMMIT.md, DOCTRINE_ENFORCEMENT.md, TOOLBOX.md and
          .doctrine/README.md — and this repository's copies of at least the first two DO carry project
          content: docs/TASK_TREE.md's Active Task Trees table is the index a resuming session reads
          first, and COMMIT.md carries the §14.3 tier paragraph and the ⛔ no-agent-trailer ruling
          (maintainer ruling 2026-08-22, ported by BEDROCK-MAINTENANCE.2.5)
  census: git -C ../bedrock log --oneline -8 | grep -i overwrite
          -> BEDROCK-MAINTENANCE-0015 (.2.10) "update_scaffold.sh never overwrites anything"
             BEDROCK-MAINTENANCE-0016 (.2.11) "never overwrite is now auditable, and the merge is
             asked for and never applied"
             BEDROCK-MAINTENANCE-0014 (.2.9)  "a spine file that carries a project decision can now
             reach a project that predates it"
  ```

  Impact: running `make update-scaffold` today would replace this project's task-tree **index** with the
  template's empty one and its commit workflow with the neutral text, and the only thing standing
  between that and a commit is the script's closing advice to "review `git diff`". Nothing mechanical
  refuses it. The `2026-09-21` sync survived because whoever ran it kept a copy of the incoming files
  in `target/` — a hand-made mitigation, in a scratch directory that any `cargo clean` deletes, which
  is how the cleanup run found it.
  ⚠️ Added `2026-09-29` by `PROGRAM.29`: upstream's updater (`bedrock` `5af0c1c`, `:37`) still makes its scratch with a
  bare `mktemp -d`, and it is not in its own `NEUTRAL` array, so it stays project-owned after adoption and
  `SCRATCH-LOCALITY` refuses the adopted copy unless it keeps this repository's `"$ROOT/target/…"` template.
  Acceptance: `scripts/update_scaffold.sh` is replaced by **upstream's** version at a named `bedrock`
  revision (copied in, per §12's exception for read-only external sources — never depended on at build
  time, and `bedrock` is not a submodule), so the fix is adopted rather than re-derived; the adoption
  is recorded in a `docs/decisions/` record naming the revision and what it changes; a **dry run** on
  this repository is measured and its output recorded on this leaf — what it would touch, what it
  refuses to touch, and what it asks for; `DOCTRINE_VERSION` states the adopted revision truthfully
  afterwards, whether or not the rest of `0.10.0` is adopted in the same commit; the other neutral files
  are **censused rather than assumed** — each one diffed against upstream and classified as
  *identical*, *project-carrying* or *behind*, so adopting the script is not confused with adopting the
  whole spine; RED arm: a seeded project-carrying spine file survives a dry run and is named in its
  output; `make gate` green.
  Priority: **medium** — it fires only on a deliberate sync, and the last one was `2026-09-21`. Filed
  now rather than then-because the cost of being wrong is the task-tree index, and because the
  mitigation currently in use is a directory inside `target/`.
  ⚠️ **Interim mitigation, until this leaf lands:** do not run `make update-scaffold`. If a sync becomes
  necessary before then, run it on a clean tree, `git diff` every one of the 25 files before staging,
  and keep the incoming copies somewhere tracked rather than under `target/`.
  Verification: see the checklist — the census of every neutral file, two dry runs on clones (before and after), five
  new arms in the spine harness that all fail against the replaced updater, and `make gate`.
  Commit: `ARCHOGEN-PROGRAM-0150 (leaf PROGRAM.26)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the replaced updater, run by the new arms against a scratch project:
    ```text
    $ git show HEAD:scripts/update_scaffold.sh > scripts/update_scaffold.sh; bash scripts/selftest_spine.sh
      SELF-TEST: the updater modified a project-carrying file, or set nothing aside
      SELF-TEST: the updater: a dirty tree is refused, not synced — expected exit 2, got 0
      spine self-test: 34 pass / 5 fail (39 arms)                                   (restored by cmp)
    ```
    — it overwrote a project-carrying file and synced over uncommitted work.
  - [x] **ROOT CAUSE (WHY + WHERE)** — **WHERE:** `git show HEAD:scripts/update_scaffold.sh | grep -nF 'cp "$tmp/bedrock/$f" "$f"'`
    → `62:    cp "$tmp/bedrock/$f" "$f"`, unconditional, inside the loop over the `NEUTRAL` array — **29** entries by
    `sed -n '/^NEUTRAL=(/,/^)/p' | grep -vc '[()]'`, not the 25 this leaf's own census said. **WHY:** its premise,
    "never carries project content", is false for 7 of them here (the census below) — the template was written before
    projects diverged from it, and this copy predates upstream's fix (`bedrock-scaffold 0.8.1` against `0.10.0`).
    ```text
    census (ours vs upstream HEAD 5af0c1c vs the 0.8.1 base 8222e97, found by git log -S"bedrock-scaffold 0.8.1"):
      identical 21 · project-carrying (only we moved) 7 · behind: DOCTRINE_VERSION · both moved: update_scaffold.sh
      VISIBILITY.md: absent here — a SEED_ONCE file of 0.8.1 the old updater could not deliver
    $ git -C ../bedrock diff --stat 8222e97 5af0c1c   → .gitignore, CHANGELOG.md, DOCTRINE_VERSION, the maintenance tree,
      scripts/update_scaffold.sh — 5 files changed: no other spine file moved
    ```
  - [x] **FIX** — upstream's `scripts/update_scaffold.sh` at `5af0c1c`, plus one hunk (scratch under
    `target/doctrine_scratch/`), `diff` against upstream = that hunk alone (6 lines); `.bedrock-incoming/` ignored, as
    upstream ignores it; `VISIBILITY.md` seeded from `5af0c1c` (the host measured `PUBLIC` by `gh repo view`, matching
    the posture it declares); `DOCTRINE_VERSION` → `bedrock-scaffold 0.10.0`; five arms in `scripts/selftest_spine.sh`;
    `docs/decisions/decision_scaffold-updater-adopted.md`.
  - [x] **ADDRESSED (verified)** —
    ```text
    dry run 1, a clone with the adopted updater, before seeding:
      ✓ 21 already current, 1 seeded, 8 differ, 0 merged to a side file — nothing of yours was modified.   exit=0
      (seeded VISIBILITY.md; DIFFERS for the 7 project-carrying files and DOCTRINE_VERSION, each "yours is UNTOUCHED")
    dry run 2, a clone of this commit's staged state, with --merge and no terminal:
      ✓ 23 already current, 0 seeded, 7 differ, 0 merged to a side file — nothing of yours was modified.   exit=0
    $ git -C ../bedrock log --format=%h -S"bedrock-scaffold 0.10.0" -- DOCTRINE_VERSION   → 6bbbf82  (the merge base resolves)
    $ bash scripts/selftest_spine.sh   → spine self-test: 39 pass / 0 fail (39 arms)
    ```
  - [x] **NO REGRESSION** — `bash scripts/check_scratch_locality.sh` → `scratch-locality: OK (143 file(s) scanned; 4
    site(s) in scaffold-owned files reported …)`, so the adopted copy keeps the fix; `make gate` green at the commit;
    `bash scripts/check_source_ledger.sh` → `source-ledger: OK (12 entries; 11 pin(s) …)` with the entry at `0.10.0`.
  - [x] **LOCKSTEP** — the decision record and its INDEX row, the `bedrock` ledger entry, `README.md` (a pointer to
    `VISIBILITY.md`), `DOCTRINE_ENFORCEMENT.md`, the `doctrine-seams-vs-forking-a-check` card, `KNOWLEDGE_MAP.md`
    regenerated. The interim mitigation above is lifted: `make update-scaffold` is safe on a clean tree.

