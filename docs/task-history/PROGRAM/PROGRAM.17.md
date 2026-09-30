- ID: `PROGRAM.17`
  Status: `done`
  Goal: check the director-mandated **live-document size-containment guide** (§18) against what this
  repository already does and adopt what is missing — the instruction covers a *partial* adoption too,
  and this is one.
  Reproduce / issue: partially adopted, with no record of the comparison. Caps exist and are
  mechanically enforced for two documents — `README.md` (`line_cap=300`, `byte_cap=16384`,
  `scripts/check_readme_stability.sh`; measured 82 lines / 4 062 bytes) and `MEMORY.md` (≤ 50 lines /
  ≤ 7168 bytes, `scripts/check_memory_architecture.sh`; measured 42 / 3 273). No budget at all is
  recorded for the rest, and they are not small: `CHANGELOG.md` 1 464 lines / 109 666 bytes and grows
  one entry per commit, `ROADMAP.md` 912 / 101 260, `DEV_NOTES.md` 687 / 54 492, `LIVE_STATUS.md`
  20 / 3 315, `KNOWLEDGE_MAP.md` 150 / 11 842 (derived). THE GAP: nothing states which live documents
  are under a size budget and which are deliberately allowed to grow — census:
  `git grep -lni 'byte_cap\|line_cap\|size cap\|byte cap\|line cap' -- '*.md' '*.sh'` → `README_POLICY`,
  `MEMORY_ARCHITECTURE`, `DOCTRINE_ENFORCEMENT`, `CHANGELOG` and two check scripts, i.e. the two
  capped documents and their doctrine, not the other five. The read-only source (431 lines,
  21 327 bytes) has not been re-read since adoption, which §18 requires even for a partial one.
  Acceptance: the guide is read at its current source revision and the comparison recorded; every
  live document is listed with its measured size and either a cap or a recorded reason it has none;
  `CHANGELOG.md` and `DEV_NOTES.md` get an explicit decision — cap, rotate into dated segments, or
  grow deliberately — instead of drifting; anything adopted is copied in under a repository-relative
  path with provenance and nothing outside this repository is written to; a mechanical check is added
  only if a cap is actually set, registered like the others and carrying RED arms.
  Priority: **medium** — nothing is unreadable today and the two documents a resuming session reads
  first are both capped and enforced. But "append-only by design" is a decision nobody has written
  down, and an unwritten decision is the one a future session quietly reverses.
  Verification: closed `2026-09-30` by its three children, each part of the acceptance met: the guide read at its
  current revision and the comparison recorded (`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s adoption note, `.17.1`); every
  live document listed with its measured size and a bound or a reason (`.17.1`); the changelog and development notes
  given an explicit decision — grow deliberately until the ruling in §8 (`.17.1`, `.17.3`); the doctrine copied in with
  provenance and nothing written outside the repository (`.17.1`); a mechanical check added because caps were set,
  registered and armed (`LIVE-SNAPSHOTS`, `.17.2`).
  Commit: `ARCHOGEN-PROGRAM-0138` (`.17.1`), `ARCHOGEN-PROGRAM-0139` (`.17.2`), `ARCHOGEN-PROGRAM-0140` (`.17.3`)
  Children: `PROGRAM.17.1`, `PROGRAM.17.2`, `PROGRAM.17.3` — decomposed `2026-09-30` after re-measuring. The guide the
  leaf means is fsmgen's adoption bridge, `docs/LIVE_DOCUMENT_SIZE_CONTAINMENT_ADOPTION_GUIDE.md` (431 lines /
  21 327 bytes at `727e0d086`, exactly the leaf's figure); the normative doctrine is fsmgen's root
  `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` at `0fa794310`. ⛔ The surfaces measured at filing have not held still:
  `LIVE_STATUS.md` 3 315 → **42 110 bytes** in 21 lines, **one table row 30 256 bytes long**; `CHANGELOG.md`
  1 464 → 3 428 lines; `MEMORY.md`'s longest line 705 bytes and `docs/TASK_TREE.md`'s 1 158 — each growing by a
  closure note per closed leaf, this session's included. The guide's stop condition binds part of this: "stop and
  request direction when a lifecycle choice would change what users can directly browse" — so rotating the
  changelog or partitioning a task tree is proposed, not done.

- ID: `PROGRAM.17.1`
  Status: `done`
  Goal: adopt the doctrine as a project-owned copy, and record the measured inventory it requires.
  Acceptance: `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` at the root — the neutral body copied verbatim with its source
  revision and digest, and a fenced archogen adoption note; every live document listed with lifecycle class,
  measured lines / bytes / longest line, today's bound and what enforces it, and its scaling term; `CHANGELOG.md`,
  `DEV_NOTES.md` and the task trees each given an explicit decision; nothing written outside this repository.
  Verification: the guide and the doctrine read at their current revisions (fsmgen `727e0d086` / `0fa794310`),
  compared with what this repository did: two of the snapshot-class documents capped and enforced (`README.md`,
  `MEMORY.md`), a third (`LIVE_STATUS.md`) uncapped and grown 12.7× since filing, no lifecycle declared for any
  history, no line-width axis anywhere. The copy's body re-extracted from the committed file and compared with
  source lines 190–529 → `cmp` identical; its sha256 `af130de4…` is the one the adoption note records. Every
  measurement in the inventory re-derived by one `wc -l`/`wc -c`/longest-line pass on `2026-09-30`. `README-STABILITY`
  → 84/300 lines, 4 203/16 384 bytes after the pointer. Nothing written outside this repository: fsmgen read with
  `git -C … rev-parse`, `wc`, `shasum` and `sed -n` only.
  Commit: `ARCHOGEN-PROGRAM-0138 (leaf PROGRAM.17.1)`

- ID: `PROGRAM.17.2`
  Status: `done`
  Goal: the guide's Phase 1 — the bounded snapshots (`LIVE_STATUS.md`, `MEMORY.md`, `docs/TASK_TREE.md`) hold current
  state only, and a checker keeps them bounded on all three axes.
  Acceptance: every closure note removed from a snapshot is first proved present in its canonical home (the leaf's
  Commit Log row, `CHANGELOG.md`); caps set from the reviewed survivor with transaction-sized headroom; a checker with
  RED arms, registered; `COMMIT.md` no longer asks for a snapshot edit that only restates history.
  Verification: see the checklist — the no-loss proof before the trim, a checker on three axes (11 arms, 6
  mutations), the pre-trim file refused.
  Commit: `ARCHOGEN-PROGRAM-0139 (leaf PROGRAM.17.2)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — measured at `2ae5559` (lines / bytes / longest line):
    ```text
    LIVE_STATUS.md       21 / 42 110 / 30 256      (3 315 bytes when PROGRAM.17 was filed)
    docs/TASK_TREE.md    73 /  7 199 /  1 158
    MEMORY.md            41 /  4 035 /    705      (a "Closed today" list, and a stale "Also open" list naming eight closed leaves)
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — two instructions asked for a snapshot edit on every commit, and the path of
    least resistance was to append what the commit did:
    ```text
    $ git grep -n "Review and update before every commit\|Update every relevant tracked doc" 2ae5559 -- LIVE_STATUS.md COMMIT.md
      2ae5559:COMMIT.md:63:3. Update every relevant tracked doc (`MEMORY.md`, `CHANGELOG.md`, `DEV_NOTES.md`,
      2ae5559:LIVE_STATUS.md:4:Review and update before every commit whenever actual closure or remaining scope changes;
    ```
    Nothing bounded any of the three on line width, and only `MEMORY.md` on lines and bytes.
  - [x] **FIX** — the three snapshots rewritten to current state: status, frontier head and blocker per row, each
    head **derived** from its tree's frontier table and that leaf's own goal; `MEMORY.md` to the guide's Phase 1 shape
    (one work unit, next action, in-flight state, blockers), with the test count and push distance given as
    commands rather than copied. **No-loss proof first:** every leaf id the three mentioned — 60, 43, 39 distinct —
    declared by its tree (0 undeclared); every `done` leaf in every tree has its Commit Log row (0 missing); the removed
    text retrievable byte-exact — `git show 2ae5559:<file>`, sha256 `4910305cff6e9c53…` (`LIVE_STATUS.md`, 42 110
    bytes), `0d2eca15c82fa3d2…` (`docs/TASK_TREE.md`, 7 199), `55d49964525f4f60…` (`MEMORY.md`, 4 035); retention: this
    repository's history. New doctrine **`LIVE-SNAPSHOTS`**: ceilings as data in the adoption note, set from the reviewed
    survivor plus one ordinary change. `COMMIT.md` and `LIVE_STATUS.md`'s header now say a snapshot changes only when
    its state does.
  - [x] **ADDRESSED (verified)** —
    ```text
    LIVE_STATUS.md       22 /  2 324 / 220    docs/TASK_TREE.md  73 / 4 469 / 189    MEMORY.md  26 / 1 770 / 120
    $ bash scripts/check_live_snapshots.sh
      live-snapshots: OK (4 snapshot(s) within their ceilings on lines, bytes and longest line)          exit=0
    $ bash scripts/check_live_snapshots.sh --self-test
      live-snapshots self-test: 11 pass / 0 fail (11 arms)
    $ git show 2ae5559:LIVE_STATUS.md > LIVE_STATUS.md; bash scripts/check_live_snapshots.sh
      LIVE-SNAPSHOTS: LIVE_STATUS.md: 42110 bytes, over its ceiling of 4096 — …
      LIVE-SNAPSHOTS: LIVE_STATUS.md: 30256 bytes on its longest line, over its ceiling of 320 — …   exit=1  (21 lines passes)
    ```
    Six mutations L-1–L-6, each restored by `cmp`, each fails its own arms — L-5 (the ceiling made exclusive) fails
    the at-the-ceiling arm. ⚠️ One arm's fixture was mis-sized by hand (43 bytes for "one over 40"); caught by the arm
    itself, rebuilt to exactly 41.
  - [x] **NO REGRESSION** — `MEMORY-ARCH` exit=0 on the rewritten pointer; `README-STABILITY` 84/300 lines; `TABLE-ARITY`
    green — one self-inflicted slip on the way, `LIVE_STATUS.md`'s header row dropped by an off-by-one slice and
    restored before any check ran on it; the doctrine driver green at the commit.
  - [x] **LOCKSTEP** — `verification.md` gains "The status pages stay short"; `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`;
    the adoption note's inventory and debt statements updated to what is now true.

- ID: `PROGRAM.17.3`
  Status: `done`
  Goal: the lifecycle decisions that change what the director browses, put to the director with evidence — the
  changelog and development notes as rolling ledgers, and the task-tree monoliths (`M1.md` 658 KB).
  Acceptance: a findings entry with measured sizes, the options the doctrine allows, a recommendation each, and what
  is lost or kept by each; no rotation or partition performed without the ruling.
  Verification: §8 of `docs/decisions/decision_findings-for-director-review.md` written from measurements taken
  `2026-09-30`: growth since `4d6d002` (`2026-09-27`, 82 commits) — `CHANGELOG.md` 1 464 → 3 459 lines, `DEV_NOTES.md`
  687 → 1 602, `M1.md` 1 864 → 6 928, `PROGRAM.md` 796 → 3 158 — and an `awk` over each tree's leaf blocks by status:
  closed leaves hold 5 328 of `M1.md`'s 6 928 lines (55 leaves) and 2 375 of `PROGRAM.md`'s 3 158 (33). Three options
  from the doctrine, what each changes for the reader, a recommendation, and the work filed as `PROGRAM.31` and
  `PROGRAM.32`, both `blocked` on the ruling; nothing rotated or sealed. On the way: the decisions index's hook for
  that record still said "six items… two needing rulings" from before §7 — corrected to eight and four.
  Commit: `ARCHOGEN-PROGRAM-0140 (leaf PROGRAM.17.3)`
