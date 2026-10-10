# PROGRAM: program spine — roadmap ownership, workspace, tiers, and the book

## Metadata

- Tree ID: `PROGRAM`
- Status: `active`
- Roadmap lane: workstream F — engineering operations (`ROADMAP.md` §11, §14, §15)
- Created: `2026-09-13`
- Owner: repo-local workflow

## Goal

Own the cross-cutting engineering substrate that every milestone tree depends on: the
conversion of `ROADMAP.md` into task-trees, the Rust workspace and crate boundaries, the
tiered verification story, the `xtask` runner, the versioning/ledger discipline, and the
mdBook that is the director's window into the project.

## Non-Goals

- This tree does not implement eADL semantics, engine realization, analysis, or generation.
  Those belong to `M0`–`M7` and `S0`.
- This tree does not restate the roadmap. `ROADMAP.md` remains the direction; the trees own
  the disciplined execution of it.

## Acceptance Criteria

- Every roadmap milestone, work package, and mandatory fixture (F01–F30) is owned by a leaf
  in some tree, and the mapping is discoverable from `docs/TASK_TREE.md`.
- The workspace layout matches the responsibility boundaries in `ROADMAP.md` §4.2, with
  crates created only when a real consumer justifies the split.
- `make check` and `make gate` stay green; the tiered verification story of §14.3 is
  implemented as named, runnable commands.
- The mdBook describes what the code actually does, with no drift.

## Task Tree

- ID: `PROGRAM`
  Status: `active`
  Goal: own the program spine
  Children: `PROGRAM.1` … `PROGRAM.63`, and the sub-leaves each of them names

- ID: `PROGRAM.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.1.md`](../task-history/PROGRAM/PROGRAM.1.md); commit `ARCHOGEN-PROGRAM-0002`

- ID: `PROGRAM.1.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.1.md`](../task-history/PROGRAM/PROGRAM.1.md); commit `ARCHOGEN-PROGRAM-0002`

- ID: `PROGRAM.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.2.md`](../task-history/PROGRAM/PROGRAM.2.md); commit `ARCHOGEN-PROGRAM-0003`

- ID: `PROGRAM.2.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.2.md`](../task-history/PROGRAM/PROGRAM.2.md); commit `ARCHOGEN-PROGRAM-0021`

- ID: `PROGRAM.3`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.3.md`](../task-history/PROGRAM/PROGRAM.3.md); commit `ARCHOGEN-PROGRAM-0029`

- ID: `PROGRAM.11`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.11.md`](../task-history/PROGRAM/PROGRAM.11.md); commit `ARCHOGEN-PROGRAM-0119`

- ID: `PROGRAM.12`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.12.md`](../task-history/PROGRAM/PROGRAM.12.md); commit `ARCHOGEN-PROGRAM-0050`

- ID: `PROGRAM.13`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.13.md`](../task-history/PROGRAM/PROGRAM.13.md); commit `ARCHOGEN-PROGRAM-0136`

- ID: `PROGRAM.14`
  Status: `pending`
  Goal: make `PROGRAM.13`'s finding **mechanically** impossible to repeat — a doctrine check that a
  leaf marked `done` names a commit in its own tree's Commit Log, registered in
  `scripts/check_doctrines.project.sh` and mirrored in `DOCTRINE_ENFORCEMENT.md`.
  THE GAP: nothing gates a `done` leaf that names no commit. CENSUS:
  `git grep -ln 'Commit Log' -- scripts/ xtask/ .doctrine/` → exactly one path,
  `scripts/bootstrap.sh`, which **seeds** the section in a new tree and never reads one back;
  the thirteen registered doctrines (`scripts/check_doctrines.sh`, `make gate`) include no
  task-log check.
  Acceptance: a new `scripts/check_*.sh` exits nonzero on a seeded `done` leaf with no commit row
  and zero on the real trees after `PROGRAM.13`; it carries RED arms in `--self-test`; it is scoped
  to staged files, so an unrelated tree cannot fail a commit; `TOOLBOX.md` and
  `DOCTRINE_ENFORCEMENT.md` name it; the honest limit is stated in the script header — it proves a
  row exists and names a subject that exists in git, never that the work was done.
  Priority: **medium**, and it must land **after** `PROGRAM.13` or the gate is red on arrival.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.15`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.15.md`](../task-history/PROGRAM/PROGRAM.15.md); commit `ARCHOGEN-PROGRAM-0137`

- ID: `PROGRAM.16`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.16.md`](../task-history/PROGRAM/PROGRAM.16.md); commit `ARCHOGEN-PROGRAM-0058`

- ID: `PROGRAM.17`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.17.md`](../task-history/PROGRAM/PROGRAM.17.md); commit `ARCHOGEN-PROGRAM-0138`

- ID: `PROGRAM.17.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.17.md`](../task-history/PROGRAM/PROGRAM.17.md); commit `ARCHOGEN-PROGRAM-0138`

- ID: `PROGRAM.17.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.17.md`](../task-history/PROGRAM/PROGRAM.17.md); commit `ARCHOGEN-PROGRAM-0139`

- ID: `PROGRAM.17.3`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.17.md`](../task-history/PROGRAM/PROGRAM.17.md); commit `ARCHOGEN-PROGRAM-0140`

- ID: `PROGRAM.18`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.18.md`](../task-history/PROGRAM/PROGRAM.18.md); commit in the tree's Commit Log

- ID: `PROGRAM.18.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.18.md`](../task-history/PROGRAM/PROGRAM.18.md); commit `ARCHOGEN-PROGRAM-0120`

- ID: `PROGRAM.18.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.18.md`](../task-history/PROGRAM/PROGRAM.18.md); commit `ARCHOGEN-PROGRAM-0121`

- ID: `PROGRAM.19`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.19.md`](../task-history/PROGRAM/PROGRAM.19.md); commit `ARCHOGEN-PROGRAM-0060`

- ID: `PROGRAM.20`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.20.md`](../task-history/PROGRAM/PROGRAM.20.md); commit `ARCHOGEN-PROGRAM-0142`

- ID: `PROGRAM.20.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.20.md`](../task-history/PROGRAM/PROGRAM.20.md); commit `ARCHOGEN-PROGRAM-0142`

- ID: `PROGRAM.20.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.20.md`](../task-history/PROGRAM/PROGRAM.20.md); commit `ARCHOGEN-PROGRAM-0143`

- ID: `PROGRAM.20.3`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.20.md`](../task-history/PROGRAM/PROGRAM.20.md); commit `ARCHOGEN-PROGRAM-0144`

- ID: `PROGRAM.21`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.21.md`](../task-history/PROGRAM/PROGRAM.21.md); commit `ARCHOGEN-PROGRAM-0086`

- ID: `PROGRAM.22`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.22.md`](../task-history/PROGRAM/PROGRAM.22.md); commit `ARCHOGEN-PROGRAM-0067`

- ID: `PROGRAM.23`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.23.md`](../task-history/PROGRAM/PROGRAM.23.md); commit `ARCHOGEN-PROGRAM-0149`

- ID: `PROGRAM.10`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.10.md`](../task-history/PROGRAM/PROGRAM.10.md); commit `ARCHOGEN-PROGRAM-0481`

- ID: `PROGRAM.10.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.10.md`](../task-history/PROGRAM/PROGRAM.10.md); commit `ARCHOGEN-PROGRAM-0145`

- ID: `PROGRAM.10.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.10.md`](../task-history/PROGRAM/PROGRAM.10.md); commit `ARCHOGEN-PROGRAM-0146`

- ID: `PROGRAM.10.3`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.10.md`](../task-history/PROGRAM/PROGRAM.10.md); commit `ARCHOGEN-PROGRAM-0147`

- ID: `PROGRAM.10.4`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.10.md`](../task-history/PROGRAM/PROGRAM.10.md); commit `ARCHOGEN-PROGRAM-0148`

- ID: `PROGRAM.10.5`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.10.md`](../task-history/PROGRAM/PROGRAM.10.md); commit `ARCHOGEN-PROGRAM-0481`

- ID: `PROGRAM.10.5.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.10.md`](../task-history/PROGRAM/PROGRAM.10.md); commit `ARCHOGEN-PROGRAM-0478`

- ID: `PROGRAM.10.5.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.10.md`](../task-history/PROGRAM/PROGRAM.10.md); commit `ARCHOGEN-PROGRAM-0479`

- ID: `PROGRAM.34`
  Status: `pending`
  Goal: every repository nested inside a vendored checkout is at the commit its parent records, and the boundary
  gate can see whether it is.
  Reproduce / issue: found `2026-09-30` while preparing `M1.21`, reading `vendor/linkedspec` read-only.
  ```text
  $ git -C vendor/linkedspec status --short                       → (clean: the level REPOSITORY-BOUNDARY checks)
  $ git -C vendor/linkedspec/rgx/subs/pgen submodule status | grep -c "^+"   → 9 of 17 nested repositories off
    their recorded commit — stimuli/sv/subs/{Cores-VeeR-EL2,slang,sv-tests,verible,verilator} and
    stimuli/vhdl/subs/{Interfaces,OsvvmLibraries,PoC,ghdl}, with 35 to 13 834 changed paths each
  $ git -C …/stimuli/sv/subs/slang reflog -1 --date=iso
    97a2b64cb HEAD@{2026-09-27 15:16:02 +0200}: clone: from https://github.com/MikePopoloski/slang
  ```
  All nine were cloned between 15:16 and 15:21 on `2026-09-27`, the day `M1.19.1` adopted LinkedSpec `2ac834913`,
  and each is at its upstream's tip rather than at the commit PGEN records (`slang` at `97a2b64cb`, recorded
  `4106501b`), with an index that does not match its `HEAD`. They are PGEN's grammar test corpora; nothing
  archogen builds, measures or reports reads them (`LS-001`…`LS-007` exercise RGX and PGEN's Rust, never
  `stimuli/`). So no evidence is affected, and the repository-boundary rule is: the vendored checkout is not
  wholly at its pin, and `REPOSITORY-BOUNDARY` checks only the first level, as its header states.
  Acceptance: the nested checkouts restored to their recorded commits by the vendor's own route, which is
  consumption, not a write; `REPOSITORY-BOUNDARY` walks nested gitlinks, with a self-test arm that seeds a nested
  repository off its pin. ⚠️ **Awaiting the director:** the restore discards the working-tree state of nine
  third-party repositories. That is hard to undo and was not ours to create, so it is not done without a yes.
  Priority: **low** — no evidence depends on the corpora; the gap is in what the gate can see.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.33`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.33.md`](../task-history/PROGRAM/PROGRAM.33.md); commit `ARCHOGEN-PROGRAM-0153`

- ID: `PROGRAM.9`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.9.md`](../task-history/PROGRAM/PROGRAM.9.md); commit `ARCHOGEN-PROGRAM-0127`

- ID: `PROGRAM.9.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.9.md`](../task-history/PROGRAM/PROGRAM.9.md); commit `ARCHOGEN-PROGRAM-0127`

- ID: `PROGRAM.9.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.9.md`](../task-history/PROGRAM/PROGRAM.9.md); commit `ARCHOGEN-PROGRAM-0131`

- ID: `PROGRAM.9.3`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.9.md`](../task-history/PROGRAM/PROGRAM.9.md); commit `ARCHOGEN-PROGRAM-0132`

- ID: `PROGRAM.4`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.4.md`](../task-history/PROGRAM/PROGRAM.4.md); commit `ARCHOGEN-PROGRAM-0033`

- ID: `PROGRAM.5`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.5.md`](../task-history/PROGRAM/PROGRAM.5.md); commit `ARCHOGEN-PROGRAM-0126`

- ID: `PROGRAM.6`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.6.md`](../task-history/PROGRAM/PROGRAM.6.md); commit `ARCHOGEN-PROGRAM-0133`

- ID: `PROGRAM.6.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.6.md`](../task-history/PROGRAM/PROGRAM.6.md); commit `ARCHOGEN-PROGRAM-0133`

- ID: `PROGRAM.6.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.6.md`](../task-history/PROGRAM/PROGRAM.6.md); commit `ARCHOGEN-PROGRAM-0134`

- ID: `PROGRAM.6.3`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.6.md`](../task-history/PROGRAM/PROGRAM.6.md); commit `ARCHOGEN-PROGRAM-0135`

- ID: `PROGRAM.8`
  Status: `pending`
  Goal: resolve the cross-tree lockstep friction in `TASK-ACCEPTANCE` — the check requires a
  complete acceptance checklist from **every** staged `docs/tasks/*.md`, not from the leaf that
  owns the staged code, so propagating a blocker into a second tree in the same commit as code
  is refused.
  Acceptance: either a declared seam that lets a commit name its owning leaf, or a documented
  convention with a gate that enforces it; the fix must not reopen the cross-file evidence
  leakage the check was hardened against, and must not be a local edit to the portable check.
  Verification: `pending`
  Commit: `pending`

  ### ROUTING EVIDENCE

  - **Does it reproduce outside this project?** Yes. Nothing in the refusal is
    archogen-specific: it fires for any repository using this spine whenever one commit lands
    code owned by tree A and a documentation edit in tree B. Measured here on
    `ARCHOGEN-M0-0009`: `docs/tasks/M5.md` was staged carrying only a blocker note, and the
    check reported `docs/tasks/M5.md has no 'ROOT CAUSE' box in its acceptance checklist`
    ×3 with `=== 1 doctrine breach(es) — commit blocked ===`, while `docs/tasks/M0.md` — the
    tree that actually owns the change — carried a complete, evidenced checklist.
  - **What was measured:** the refusal is a property of the check's *file scope*, not of the
    content. Unstaging `docs/tasks/M5.md` and changing nothing else returns
    `=== all doctrines green ===`.
  - **What would make this routing wrong:** if the strictness is deliberate — i.e. if requiring
    every touched tree to justify itself is the intended cost of the cross-file hardening
    described in the check's own header. That is plausible, and it is why the acceptance above
    forbids any fix that reopens the leakage. The correct first output of this leaf may be an
    upstream report rather than a change.
  - **A third occurrence, and the one that changes the shape of the problem
    (`ARCHOGEN-PROGRAM-0021`):** a *rename* touches every tree that mentions the old name — here
    `M0`, `M1`, `M3`, `M4` and `S0`, none of which owned the change. In the earlier two cases the
    cross-tree edit was incidental and could plausibly have been deferred; for a rename it is
    **unavoidable**, because leaving the old name in five trees is the drift the change exists to
    remove. That also exposed a second-order trap: `M0` and `M1` happen to carry ticked
    checklists from their own earlier leaves, so staging them alongside code would have passed
    the gate on evidence belonging to unrelated work — precisely the incidental pass the
    box-scoping was hardened against. They were unstaged deliberately rather than relied upon.
  - **Interim convention, in use from 2026-09-13:** split the commit. Code and its owning tree
    land together; a documentation edit to another tree lands as its own docs-only commit. See
    `docs/knowledge/cross-tree-lockstep-and-commit-scope.md`.

- ID: `PROGRAM.7`
  Status: `pending`
  Goal: project-specific doctrine checks (`scripts/check_doctrines.project.sh`) that encode
  this program's own invariants — most importantly that no implementation content leaks into
  eADL fixtures, and that assurance wording cannot overstate evidence.
  Acceptance: each check fails on a seeded violation and passes on the clean tree.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.24`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.24.md`](../task-history/PROGRAM/PROGRAM.24.md); commit `ARCHOGEN-PROGRAM-0122`

- ID: `PROGRAM.25`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.25.md`](../task-history/PROGRAM/PROGRAM.25.md); commit `ARCHOGEN-PROGRAM-0083`

- ID: `PROGRAM.26`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.26.md`](../task-history/PROGRAM/PROGRAM.26.md); commit `ARCHOGEN-PROGRAM-0150`


- ID: `PROGRAM.27`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.27.md`](../task-history/PROGRAM/PROGRAM.27.md); commit `ARCHOGEN-PROGRAM-0111`

- ID: `PROGRAM.28`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.28.md`](../task-history/PROGRAM/PROGRAM.28.md); commit `ARCHOGEN-PROGRAM-0123`

- ID: `PROGRAM.29`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.29.md`](../task-history/PROGRAM/PROGRAM.29.md); commit `ARCHOGEN-PROGRAM-0124`

- ID: `PROGRAM.30`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.30.md`](../task-history/PROGRAM/PROGRAM.30.md); commit `ARCHOGEN-PROGRAM-0151`

- ID: `PROGRAM.31`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.31.md`](../task-history/PROGRAM/PROGRAM.31.md); commit `ARCHOGEN-PROGRAM-0207`

- ID: `PROGRAM.32`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.32.md`](../task-history/PROGRAM/PROGRAM.32.md); commit `ARCHOGEN-PROGRAM-0228`

- ID: `PROGRAM.32.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.32.md`](../task-history/PROGRAM/PROGRAM.32.md); commit `ARCHOGEN-PROGRAM-0224`

- ID: `PROGRAM.32.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.32.md`](../task-history/PROGRAM/PROGRAM.32.md); commit `ARCHOGEN-PROGRAM-0225`

- ID: `PROGRAM.32.3`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.32.md`](../task-history/PROGRAM/PROGRAM.32.md); commit `ARCHOGEN-PROGRAM-0226`

- ID: `PROGRAM.32.4`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.32.md`](../task-history/PROGRAM/PROGRAM.32.md); commit `ARCHOGEN-PROGRAM-0228`

- ID: `PROGRAM.35`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.35.md`](../task-history/PROGRAM/PROGRAM.35.md); commit `ARCHOGEN-PROGRAM-0197`

- ID: `PROGRAM.35.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.35.md`](../task-history/PROGRAM/PROGRAM.35.md); commit `ARCHOGEN-PROGRAM-0197`

- ID: `PROGRAM.35.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.35.md`](../task-history/PROGRAM/PROGRAM.35.md); commit `ARCHOGEN-PROGRAM-0198`


- ID: `PROGRAM.36`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.36.md`](../task-history/PROGRAM/PROGRAM.36.md); commit `ARCHOGEN-PROGRAM-0211`

- ID: `PROGRAM.37`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.37.md`](../task-history/PROGRAM/PROGRAM.37.md); commit `ARCHOGEN-PROGRAM-0218`

- ID: `PROGRAM.38`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.38.md`](../task-history/PROGRAM/PROGRAM.38.md); commit `ARCHOGEN-PROGRAM-0222`

- ID: `PROGRAM.39`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.39.md`](../task-history/PROGRAM/PROGRAM.39.md); commit `ARCHOGEN-PROGRAM-0230`

- ID: `PROGRAM.40`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.40.md`](../task-history/PROGRAM/PROGRAM.40.md); commit `ARCHOGEN-PROGRAM-0229`

- ID: `PROGRAM.41`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.41.md`](../task-history/PROGRAM/PROGRAM.41.md); commit `ARCHOGEN-PROGRAM-0234`


- ID: `PROGRAM.41.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.41.md`](../task-history/PROGRAM/PROGRAM.41.md); commit `ARCHOGEN-PROGRAM-0238`


- ID: `PROGRAM.42`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.42.md`](../task-history/PROGRAM/PROGRAM.42.md); commit `ARCHOGEN-PROGRAM-0239`


- ID: `PROGRAM.43`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.43.md`](../task-history/PROGRAM/PROGRAM.43.md); commit `ARCHOGEN-PROGRAM-0259`
- ID: `PROGRAM.44`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.44.md`](../task-history/PROGRAM/PROGRAM.44.md); commit `ARCHOGEN-PROGRAM-0278`

- ID: `PROGRAM.45`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.45.md`](../task-history/PROGRAM/PROGRAM.45.md); commit `ARCHOGEN-PROGRAM-0282`

- ID: `PROGRAM.46`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.46.md`](../task-history/PROGRAM/PROGRAM.46.md); commit `ARCHOGEN-PROGRAM-0283`

- ID: `PROGRAM.47`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0296`

- ID: `PROGRAM.47.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0296`

- ID: `PROGRAM.47.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0297`

- ID: `PROGRAM.47.3`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0298`

- ID: `PROGRAM.47.4`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0301`

- ID: `PROGRAM.47.5`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0302`

- ID: `PROGRAM.47.5.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0302`

- ID: `PROGRAM.47.5.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0303`

- ID: `PROGRAM.47.5.3`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0304`

- ID: `PROGRAM.47.5.4`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0305`

- ID: `PROGRAM.47.5.5`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0306`

- ID: `PROGRAM.47.5.6`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0307`

- ID: `PROGRAM.47.5.7`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0308`

- ID: `PROGRAM.47.5.8`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0310`

- ID: `PROGRAM.47.5.9`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0311`

- ID: `PROGRAM.47.5.10`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0312`

- ID: `PROGRAM.47.5.11`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0313`

- ID: `PROGRAM.47.5.12`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0314`

- ID: `PROGRAM.47.5.13`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0315`

- ID: `PROGRAM.47.5.14`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0316`

- ID: `PROGRAM.47.5.15`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0317`

- ID: `PROGRAM.47.5.16`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0318`

- ID: `PROGRAM.47.5.17`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0320`

- ID: `PROGRAM.47.5.18`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0321`

- ID: `PROGRAM.47.5.19`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0322`

- ID: `PROGRAM.47.5.20`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0323`

- ID: `PROGRAM.47.5.21`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0324`

- ID: `PROGRAM.47.5.22`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0325`

- ID: `PROGRAM.47.5.23`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0327`

- ID: `PROGRAM.47.6`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.47.md`](../task-history/PROGRAM/PROGRAM.47.md); commit `ARCHOGEN-PROGRAM-0326`

- ID: `PROGRAM.48`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.48.md`](../task-history/PROGRAM/PROGRAM.48.md); commit `ARCHOGEN-PROGRAM-0392`

- ID: `PROGRAM.49`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.49.md`](../task-history/PROGRAM/PROGRAM.49.md); commit `ARCHOGEN-PROGRAM-0400`

- ID: `PROGRAM.50`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.50.md`](../task-history/PROGRAM/PROGRAM.50.md); commit `ARCHOGEN-PROGRAM-0412`

- ID: `PROGRAM.51`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.51.md`](../task-history/PROGRAM/PROGRAM.51.md); commit `ARCHOGEN-PROGRAM-0421`

- ID: `PROGRAM.52`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.52.md`](../task-history/PROGRAM/PROGRAM.52.md); commit `ARCHOGEN-PROGRAM-0425`

- ID: `PROGRAM.52.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.52.md`](../task-history/PROGRAM/PROGRAM.52.md); commit `ARCHOGEN-PROGRAM-0426`

- ID: `PROGRAM.52.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.52.md`](../task-history/PROGRAM/PROGRAM.52.md); commit `ARCHOGEN-PROGRAM-0429`

- ID: `PROGRAM.53`
  Status: `pending`
  Goal: a ticked NO REGRESSION box that cites a test run is backed by a run made on the tree it is committed with —
  measured, not remembered.
  Reproduce / issue: found `2026-10-05` by `M3.6.2.1`. `M3.6.2`'s box read `make focused` → `tier focused: passed — 3
  passed, 0 failed`; the session's only run before its commit `ARCHOGEN-M3-0432` was at 08:08, and the file that
  broke `eadl-front`'s population test, `trust/roots.eadl`, was written at 08:27. The doctrines run no test suite, so
  nothing at commit time compares a cited run with the tree. CI runs the suite only after a push.
  Direction, to be designed in this leaf: `make focused` writes a stamp in `target/` naming the tree it ran on (the
  working tree's own tree id, through a temporary index) and its verdict; `TASK-ACCEPTANCE` refuses a ticked box that
  cites `make focused` when the staged tree has no passing stamp. Open: how unstaged changes beside the staged ones are
  handled, and a stamp's relation to a tier other than `focused`.

- ID: `PROGRAM.54`
  Status: `done` — started and closed `2026-10-10`: the project's second sample; the scaffold's census itself is
  `PROGRAM.75`'s proposal
  Goal: the handoff census (`scripts/check_no_background_jobs.sh`) tells a background job from a transient helper
  without an allowlist.
  Reproduce / issue: found `2026-10-05` by three review readers (`M3.1.1` rounds 18 and 19, `M3.6.1` round 9), each
  told to end on `handoff: OK`: the census named iTerm's `pidinfo --git-state <repo> 4 1`, whose command line names
  the checkout — the census's PROJECT-WORK arm — respawned every few seconds by the terminal (PIDs 2171, 6514, 10743;
  38355, 42120, each 4–5 s old), while a run between two respawns prints `handoff: OK`. So the verdict depends on when
  it runs. Direction, to be designed here: a process counts only if a second sample a few seconds later still holds
  it, since a job that can rewrite tracked files outlives that and a helper does not — a property, as the census's
  own header asks, not a list of names. The script is the scaffold's (`scripts/update_scaffold.sh`'s `NEUTRAL` list),
  so an edit here would be overwritten by the next update: the fix is proposed to the scaffold's owner, never written
  into another repository (`decision_repository-boundary-read-only.md`), with a project-side second sample meanwhile.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — a helper whose command line names the checkout, `bash -c 'sleep "$1"; :'
    "$PWD/handoff-census-arm" 4`, alive: `bash scripts/check_no_background_jobs.sh` → exit `1`, naming it; once it
    ended, the same run names it no more (`grep -c handoff-census-arm` → `0`) — the verdict depends on the instant.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: the scaffold census's one `ps -Ao` snapshot, `git grep -n 'SNAP=' --
    scripts/check_no_background_jobs.sh` → `scripts/check_no_background_jobs.sh:80`. WHY: one instant cannot tell a process that lives seconds from
    one that lives on, and the script is the scaffold's, so it is not this project's to change.
  - [x] **FIX** — `scripts/handoff_census.sh`: the scaffold's census, then, when it names anything, a second after
    `HANDOFF_RESAMPLE_SECONDS` (8 unless set); a process counts only when both name it by PID and command line, the
    rest reported as transient. `TOOLBOX.md`'s row. `PROGRAM.75` filed for the scaffold's owner.
  - [x] **ADDRESSED (verified)** — `bash scripts/handoff_census.sh --self-test` → *"2 pass / 0 fail (2 arms)"*: a
    process the first sample names and the second does not, reported transient and not counted; one both hold, named
    and refused. A copy without the second sample (`target/p54/mut/`, untracked, so not durable) → *"1 pass / 1 fail"*.
  - [x] **NO REGRESSION** — `bash scripts/run_self_tests.sh` → *"OK — 48 self-test(s) passed"*, the new one among
    them; `make focused` → `passed — 3 passed, 0 failed`; the doctrine gate at commit.
  - [x] **LOCKSTEP** — `TOOLBOX.md`; this leaf, `PROGRAM.75` and both logs; `docs/TASK_TREE.md`; `CHANGELOG.md`.
  Verification: `2026-10-10` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0533 (leaf PROGRAM.54)`

- ID: `PROGRAM.55`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.55.md`](../task-history/PROGRAM/PROGRAM.55.md); commit `ARCHOGEN-PROGRAM-0438`

- ID: `PROGRAM.55.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.55.md`](../task-history/PROGRAM/PROGRAM.55.md); commit `ARCHOGEN-PROGRAM-0442`

- ID: `PROGRAM.56`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.56.md`](../task-history/PROGRAM/PROGRAM.56.md); commit `ARCHOGEN-PROGRAM-0445`

- ID: `PROGRAM.57`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.57.md`](../task-history/PROGRAM/PROGRAM.57.md); commit in the tree's Commit Log

- ID: `PROGRAM.58`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.58.md`](../task-history/PROGRAM/PROGRAM.58.md); commit in the tree's Commit Log

- ID: `PROGRAM.59`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.59.md`](../task-history/PROGRAM/PROGRAM.59.md); commit in the tree's Commit Log

- ID: `PROGRAM.60`
  Status: `done` — filed `2026-10-06` by `M3.1.2.1`; started and closed `2026-10-10`
  Goal: the third reader's record, `docs/semantics/third-opinion.txt`, cannot fall behind the tracked descriptions
  unseen.
  Reproduce / issue: `bash scripts/third_opinion.sh` on `2026-10-06` → `125 document(s) — 125 agree`, then exit 1:
  four files *"agree, and not in the record"*. They were added by `M2.13` (`49638bf`, `2026-10-01`), `M2.14`
  (`d65d836`, `2026-10-01`, two files) and `M3.6.2` (`1d6415d`, `2026-10-05`), and the record was last written by
  `947cdc2` on `2026-09-30`. Each leaf's commit passed every gate.
  Root cause: the script needs LinkedSpec's `sexpr_file`, built under `.app-data/`, and exits 2 without it. So it
  runs only by hand: census `git grep -n third_opinion -- scripts xtask Makefile .github .githooks
  ':!scripts/third_opinion.sh'` → no match, so no tier, hook, doctrine or workflow invokes it. A description added after the record
  was last written goes unrecorded until somebody runs the script by hand. `M3.1.2.1` blessed the four (all
  `agree`) beside its own two files.
  Direction, to be designed here: the frozen verdict table (`verdicts.txt`) and the record share one population, so
  a check needing no vendor build can refuse a tracked description that the record lacks. The reader's verdict on a
  new file then still needs the build, but its absence from the record becomes visible at commit time.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the new test run against the record as `947cdc2` left it (`git show
    947cdc2:docs/semantics/third-opinion.txt`) → *"6 difference(s) …"*: the four this leaf names and two more since,
    `docs/semantics/kinds/deffact.eadl` and `docs/semantics/vocabulary/vocabulary.eadl`; against today's record, `bash
    scripts/third_opinion.sh` → *"127 document(s) — 127 agree … OK"*, `M3.1.2.1`'s bless having caught up by hand.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `git grep -n third_opinion -- scripts xtask Makefile .github .githooks
    crates ':!scripts/third_opinion.sh'` at `3d9a70c` → one match, `crates/eadl-front/examples/reader_shape.rs:1`, a
    comment naming the script whose shape that example prints: no tier, hook, test or workflow ran the census. WHY:
    the script checks the census and the classification together, and the classification needs LinkedSpec's build.
  - [x] **FIX** — `crates/archogen-cli/tests/verdicts.rs`: `the_third_reader_s_record_names_every_description`, the
    record's paths against the verdict table's own population, every description and only those, with no vendor
    build; the two populations one, as the direction asks. The script's header and the book's *A third reader*.
  - [x] **ADDRESSED (verified)** — `cargo test -p archogen-cli --test verdicts the_third_reader` → `1 passed` on the
    tree; against `947cdc2`'s record → `1 failed`, six descriptions named; against a record naming `docs/nowhere.eadl`
    → `1 failed`, it named — `target/p60/falsify.py`, untracked and so not durable, the record restored byte for byte.
  - [x] **NO REGRESSION** — `make focused` → `passed — 3 passed, 0 failed`; the doctrine gate at commit.
  - [x] **LOCKSTEP** — the script's header; the book's *A third reader*; this leaf and both logs; `docs/TASK_TREE.md`;
    `CHANGELOG.md`.
  Verification: `2026-10-10` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0529 (leaf PROGRAM.60)`

- ID: `PROGRAM.61`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.61.md`](../task-history/PROGRAM/PROGRAM.61.md); commit in the tree's Commit Log
- ID: `PROGRAM.62`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.62.md`](../task-history/PROGRAM/PROGRAM.62.md); commit in the tree's Commit Log

- ID: `PROGRAM.63`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.63.md`](../task-history/PROGRAM/PROGRAM.63.md); commit in the tree's Commit Log

- ID: `PROGRAM.64`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.64.md`](../task-history/PROGRAM/PROGRAM.64.md); commit `ARCHOGEN-PROGRAM-0482`

- ID: `PROGRAM.65`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.65.md`](../task-history/PROGRAM/PROGRAM.65.md); commit `ARCHOGEN-PROGRAM-0488`

- ID: `PROGRAM.66`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.66.md`](../task-history/PROGRAM/PROGRAM.66.md); commit `ARCHOGEN-PROGRAM-0490`

- ID: `PROGRAM.67`
  Status: `done` — filed `2026-10-06` by `M3.6.6.1`'s round 10; started and closed `2026-10-10`
  Goal: `cargo xtask mutate` runs each entry under a timeout that kills the test's whole process group and says
  which entry hung, so a mutation that makes a loop never end is reported, not waited on.
  Acceptance: an entry that hangs is reported within its timeout as a hang the tests reach, and the file is restored;
  a self-test arm holds it; the `extended` tier's `mutation` step finishes whatever the catalogue holds.
  Why: measured `2026-10-06`, a catalogue entry of `M3.6.6.1`'s, `gh-header-reads-past-code`, made the header's loop
  step past the end of a file forever once a redundant end test was removed: the whole-catalogue run waited 1 507.5 s
  until the test was killed by hand, and only then counted it killed. `scripts/mutation_sweep.sh` has such a timeout;
  the harness has none (`grep -n timeout xtask/src/mutation.rs` → nothing). The entry was rewritten to end, so nothing
  hangs today.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `git show b3c14de:xtask/src/mutation.rs | grep -n '.output()'` → `248`: the tests ran by
    `Command::output`, which waits without limit; `git show b3c14de:xtask/src/mutation.rs | grep -c timeout` → `0`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `git show b3c14de:xtask/src/mutation.rs | grep -n 'Command::new("cargo")'`
    → `245`, `run_one`'s call, ending in `.output()` at `250`. WHY: the harness
    assumed a mutation fails or passes; one that makes a loop endless does neither, and cargo's test binary runs in
    cargo's process group, so killing cargo alone would leave it running.
  - [x] **FIX** — `xtask/src/mutation.rs`: `run_bounded`, the command in a process group of its own, its output read
    by two threads, polled against `limit()` — `ARCHOGEN_MUTATE_TIMEOUT` seconds, 600 unless set — and past it the
    group killed with `kill -KILL -<pgid>`; `Seen::Hung`, reported as a kill *"by a hang"*; the file restored by the
    same guard as ever. Two unit tests; two catalogue entries breaking the limit and the group kill. ⚠️ A process the
    tests start with `setsid` leaves the group, and while it holds the output the run waits: stated beside the code.
  - [x] **ADDRESSED (verified)** — `cargo test -p xtask mutation::tests` → `8 passed`; `cargo xtask mutate --only
    mutate-a-hang-kills-the-command-alone mutate-a-hang-is-waited-on` → *"2 mutation(s), each killed"*, each by
    `a_run_past_its_limit_is_stopped_with_everything_it_started`. End to end, a temporary entry making a test spin
    forever, `ARCHOGEN_MUTATE_TIMEOUT=30 cargo xtask mutate --only demo-a-hang` → *"killed by a hang: the tests ran
    past the 30 s limit, their process group killed (30.1s)"*, the file restored byte for byte (`shasum` equal), no
    test process left (`pgrep` → none) — the entry removed after, its run untracked and so not durable.
  - [x] **NO REGRESSION** — `cargo test -p xtask` → `165 passed; 0 failed`; `cargo clippy -p xtask --all-targets -- -D
    warnings` → clean; `cargo fmt -p xtask -- --check` → clean; `make focused` → `passed — 3 passed, 0 failed`; the doctrine gate at commit.
  - [x] **LOCKSTEP** — the module's header; the book's harness section; `TOOLBOX.md`'s row; this leaf and both logs;
    `docs/TASK_TREE.md`; `CHANGELOG.md`.
  Verification: `2026-10-10` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0530 (leaf PROGRAM.67)`

- ID: `PROGRAM.68`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.68.md`](../task-history/PROGRAM/PROGRAM.68.md); commit `ARCHOGEN-PROGRAM-0508`

- ID: `PROGRAM.69`
  Status: `active` — the tool, its gate and the seal committed `2026-10-10` (`ARCHOGEN-PROGRAM-0511`); its review open,
  round 11's defects and arm gaps answered (`ARCHOGEN-PROGRAM-0531`), round 12 next
  Goal: a closed subtree below an open top-level subtree is sealed too, so a tree whose top-level subtree stays open
  for long — on a blocked leaf, or a long feature — does not keep its finished leaves live.
  Reproduce / issue: `M3.6.6.2.1`'s commit was refused by `README-ROUTES`, *"docs/tasks/: 823457 bytes in total, over
  its ceiling of 819200"*. The seal releases only a top-level subtree whose every leaf is `done`
  (`decision_task-tree-sealing.md`, "The unit is a closed subtree"), and on `2026-10-10` the only ones left were
  `PROGRAM.66` and `.68`, sealed by that commit to bring the folder to 815 443 bytes before this leaf was written. Measured the same day by a census
  of every leaf's span: 271 393 bytes of `done` leaves sit under top-level subtrees with an open leaf — `M1` 84 363,
  `M2` 104 992, `M3` 82 038 — and 258 649 of them in 28 subtrees below the top level whose every leaf is `done`, which
  a finer unit would seal — *corrected the same day by the review's R2-3 and R3-1: those were character counts of a
  working tree; in bytes at `fea69ad`, by `bash scripts/check_task_history.sh --census fea69ad`, 274 483, `M1` 85 113,
  `M2` 105 946, `M3` 83 424, and 261 610 in the 28; and, by R4-4, the 823 457 and 815 443 above were measured on the
  working tree, `M3.6.6.2.1`'s edits staged, and reproduce at no commit* — `M3.6.1`, `M3.6.2`, `M3.6.4`, `M3.6.6.1` among them, under `M3.6`, which stays open while
  `M3.6.5` waits on the director. Each sub-leaf of `M3.6.6.2` adds kilobytes to `M3.md`, so the next commit crosses
  the ceiling again.
  Direction, to be designed here: the unit becomes the outermost closed subtree — a subtree at any depth whose every
  leaf is `done` and whose parent is open or the tree itself — sealed to `docs/task-history/<TREE>/<SUBTREE>.md`; a
  sealed file still never changes, since its subtree is closed, and when its parent later closes, the parent's
  remaining leaves seal to the parent's own file beside it. The record, the tool, the gate's legs 4–6 and its
  self-test change together, under a review by a context that did not write them, as the record's own was; the option
  the director ruled, (C), is "closed leaves sealed out of the task trees", which this keeps.
  Acceptance: a seal of the real trees releases what the census measures, each reconstruction byte for byte; the gate
  refuses a live leaf inside a sealed subtree at any depth, a sealed file that changes, and a nested seal whose parent
  is closed; every arm fails first; the record amended and reviewed.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `M3.6.6.2.1`'s first commit attempt, the doctrine gate → *"README-ROUTES: docs/tasks/:
    823457 bytes in total, over its ceiling of 819200"*. `bash scripts/check_task_history.sh --census fea69ad` →
    *"census: fea69ad — 274483 byte(s) of `done` leaves under open top-level subtrees, 261610 of them in 28 closed
    subtree(s) below the top level"*: `M1` 85 113, `M2` 105 946, `M3` 83 424, which no seal could take.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `git show fea69ad:scripts/check_task_history.sh | grep -n 'return
    ".".join(parts\[:2\])\|key = subtree(lid, tree_name)'` → `122`, `subtree` fixing a unit at two components, and
    `324`, the seal grouping every leaf by it; leg 6 at `277` and leg 5's whole-subtree rule at `312` judged by the same
    key. WHY it bites now: the unit assumed a top-level subtree closes soon after its parts, and `M3.6` stays open while
    `M3.6.5` waits on the director, as `M2.7` does on `M2.7.4.5` and `M1.29` on `M1.29.4`.
  - [x] **FIX** — `scripts/check_task_history.sh`: `under` and `units`, each live leaf's outermost closed subtree, a
    top-level key always a candidate and a deeper one only as a leaf of the tree; the seal groups by `units`, refuses a
    tree holding a leaf not under its name before it writes, and on a rollback removes the folders it made; leg 4 by
    `under`; leg 5 rebuilds each file's bytes from its leaves' spans in the tree before the seal, requires each sealed
    leaf's outermost closed subtree there to be its file's; leg 6 refuses
    a live leaf under any sealed key, at any depth, in any tree file; the index's layout, one table per tree with its
    rows under its header, checked; `--census <COMMIT>`. The record amended sentence by sentence; its review reopened
    (`ARCHOGEN-PROGRAM-0510`) and rounds 2–5 and the sweep appended; `DOCTRINE_ENFORCEMENT.md`'s row, `TOOLBOX.md`, the
    book's annex, the index's header. After round 6: a row twice in its tree's table and a leaf named twice in a tree
    refused by name, the seal refusing the second before it writes; a file the gate cannot read, met after the seal
    wrote, rolled back as any refusal is; the check of a stub's commit, added after round 5, withdrawn, and the two
    first-seal stubs it had "corrected" restored. `PROGRAM.70` filed for `docs/reviews/`' file ceiling.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_task_history.sh --self-test` → *"66 pass / 0 fail (66 arms)"*.
    `python3 target/m369/mutate.py` → twenty named mutations of the tool, each `killed` by the arm for its rule;
    `target/review-p69-r3/mutate2.py`, round 3's twenty → 16 killed, the four left equivalent as rounds 4 and 5
    argued — both runners untracked, so these counts are not durable. The sweep of the core, `target/m369/sweep.py`,
    untracked and not durable → its third run, on round 5's tool, *"196 mutant(s) … 182 killed, 14
    not"*, each of the 14 reasoned in the review history. The real seal: `--seal M1` → *"sealed 7 subtree(s), 7
    leaves"*, `M2` → *"10 subtree(s), 27 leaves"*, `M3` → *"11 subtree(s), 12 leaves"*, `PROGRAM` → *"nothing to
    seal"*, each *"the reconstruction is byte for byte"*; the seal takes 255 871 bytes out of `docs/tasks/`, which held
    818 985 at `72bd446`; the 28 files hold
    261 582 bytes, the census's 261 610 less the blank line each subtree leaves in its tree. Rounds 2 to 5 re-derived the
    seal with code of their own, byte for byte `fea69ad`'s leaves and exactly the outermost closed set there; round 4
    fuzzed 840 seals against an oracle and 890 hand seals, the gate accepting exactly the tool's — its runner
    untracked, so those counts are not durable. After round 7: a sealed file holding no leaf refused; every write of a
    seal and its proof in one guard, so any exception that stops the run rolls the seal back first — a kill signal
    excepted, a stated limit (R8-2); a column-0 line below the top
    level refused, by an arm; `--self-test` → *"69 pass / 0 fail (69 arms)"*. After round 8: a refused
    seal never reported sealed, the separator and an empty table checked → *"71 pass / 0 fail (71 arms)"*, each new rule
    failed by a mutation. After round 9: the tree and the index written whole or not at all; the rollback undoing what
    was written alone, each step on its own, naming what it could not undo; a sealed file's path already taken refused
    before any write — the file-size and taken-path arms failing first on `7ad8e6b`'s tool, then `--self-test` →
    *"77 pass / 0 fail (77 arms)"*; `python3 target/p69r9/mutate.py` → nine mutations of round 9's rules, each `killed`,
    its runner untracked, so not durable. After round 10: each write noted before it is made; any entry at a sealed
    file's path refused, a unit the index rows skipped; a linked tree or index refused — the tree, index and create
    interrupt arms, the taken-file and the linked-tree arms failing first beside `0a7ec7f`'s core, the
    just-before-creation arm holding the existence guard (corrected by R11-D2), on the working tree and so not
    durable, then `--self-test` → *"85 pass / 0 fail (85 arms)"*; `python3 target/p69r10/mutate.py` → eleven mutations
    of round 10's rules, each `killed`, its runner untracked, so not durable. After round 11: a stop recorded and
    answered by a rollback that runs to its end; a linked history folder refused — `target/p69r11/hybrid.sh`, these
    arms beside `b87adbc`'s core → *"82 pass / 9 fail (91 arms)"*, the seven stop arms and the linked folder failing,
    then `--self-test` → *"90 pass / 0 fail (90 arms)"*; `python3 target/p69r11/mutate.py` → six mutations of round
    11's rules, each `killed`; both runners untracked, so not durable.
  - [x] **NO REGRESSION** — `bash scripts/check_task_history.sh` at `e624001` → *"OK (170 sealed file(s) … every sealed
    leaf proven against its tree before its seal)"*: the 142 seals before this change judged by every generalised leg, the byte
    rebuild included. `bash scripts/run_self_tests.sh` → *"OK — 47 self-test(s) passed"*; the
    gates that read what this change touched, each re-run → `stated-order` *"10 pass / 0 fail"*, `review-history` *"65
    pass / 0 fail"*, `readme-routes` *"20 pass / 0 fail"*, `handoff-ledger` *"14 pass / 0 fail"*.
    `handoff-ledger` reads the sealed quotes; `review-history` holds the reopened history; the doctrine gate at commit.
  - [x] **LOCKSTEP** — `docs/decisions/decision_task-tree-sealing.md`, its review history and `docs/reviews/INDEX.md`'s
    row, `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`, `docs/book/src/annex-repository.md`, `docs/task-history/INDEX.md`;
    this leaf and both logs, `PROGRAM`'s changelog; the frontier, `docs/TASK_TREE.md`, `LIVE_STATUS.md`
    and `MEMORY.md`; `CHANGELOG.md`.
  Verification: `2026-10-10` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0510 (leaf PROGRAM.69)`, the review reopened; `ARCHOGEN-PROGRAM-0511 (leaf PROGRAM.69)`;
  `ARCHOGEN-PROGRAM-0514 (leaf PROGRAM.69)`, round 7 answered; `ARCHOGEN-PROGRAM-0517 (leaf PROGRAM.69)`, round 8;
  `ARCHOGEN-PROGRAM-0521 (leaf PROGRAM.69)`, round 9; `ARCHOGEN-PROGRAM-0525 (leaf PROGRAM.69)`, round 10;
  `ARCHOGEN-PROGRAM-0531 (leaf PROGRAM.69)`, round 11

- ID: `PROGRAM.70`
  Status: `done` — started `2026-10-10`, decomposed into `.1`, the bytes, and `.2`, the count, closed the same day with `.2`
  Goal: a new design's review history always has room in `docs/reviews/`.
  Reproduce / issue: `PROGRAM.69` opened `docs/reviews/decision_task-tree-sealing-amendment-reviews.md`, and
  `bash scripts/check_readme_routes.sh` → *"docs/reviews/: 17 tracked files, over its ceiling of 16"*. The folder holds
  `INDEX.md` and fifteen histories, two of them stubs of archived histories (`bash scripts/check_review_history.sh
  --seal` leaves one at each history's path, so a citation still resolves), and an archive never lowers the count.
  So the next design whose review needs a history of its own — `M3.6.6.4`'s, `M3.7`'s — is refused, and `PROGRAM.69`
  reopened a closed history instead (`ARCHOGEN-PROGRAM-0510`).
  Direction, to be decided here with a measurement: stubs that leave the folder, their citations repointed; the count
  ceiling raised by a decision record, with the rate measured; or one index line standing for each archived history.
  Measured `2026-10-10` at `0a7ec7f`, `git ls-files docs/reviews | wc -l` → `16` and `… | xargs cat | wc -c` →
  `391513`, of ceilings 16 and 393 216: the bytes block a new history first, about 1.7 KB left, less than one round. Of
  the 15 histories, 2 are archived stubs and 11 more are closed — `docs/reviews/INDEX.md`'s rows — and only 2 open. A
  stub cannot leave: sealed task-history files cite both archived stubs (`git grep -l` → `docs/task-history/M2/M2.7.1.md`,
  `PROGRAM/PROGRAM.36.md`, `PROGRAM/PROGRAM.50.md`), and a sealed file never changes, so the first and third directions
  would break citations nothing may repoint. Decided: the bytes come back by archiving every closed history, which
  `PROGRAM.55`'s archive exists for (`.1`); the count stops counting what is no history — a stub, three lines
  `REVIEW-HISTORY` proves exact on every commit — so `README-ROUTES` counts the histories a reader may open (`.2`).

- ID: `PROGRAM.70.1`
  Status: `done` — started and closed `2026-10-10`, in two commits
  Goal: every closed review history archived out of `docs/reviews/`, byte for byte, behind its stub.
  Acceptance: `bash scripts/check_review_history.sh --seal <FILE>` for each history whose row reads closed; the gate
  and `README-ROUTES` pass; the folder's bytes measured before and after.
  Step 1, `2026-10-10` (`ARCHOGEN-PROGRAM-0522`): seven archived, each *"read back byte for byte"*; the three rows
  reading *"closed `<date>`: …"* put in the archive's form, *"closed: `<date>`, …"* — the tool archives a row whose
  status begins `closed: `, read at `HEAD`, so their archiving is step 2's; `decision_runtime-analysis-variant-reviews.md`
  refused and kept live — *"08a707bbbd3b changed … while its review's row already read closed"*: `ARCHOGEN-M2-0344`,
  already on `origin/main`, reopened its review in the commit that appended the round, and pushed history is not
  rewritten (the archive's own limit). No link anchors into an archived history (`git grep` → none).
  Step 2 (`ARCHOGEN-PROGRAM-0523`): the three, their rows closed at `HEAD`, archived, each *"read back byte for byte"*.
  Verification: `2026-10-10` — `git ls-files docs/reviews | xargs cat | wc -c` → `391513` at `0a7ec7f`, `262852`
  after step 1, `78575` after step 2, 16 files throughout; `bash scripts/check_review_history.sh` and
  `bash scripts/check_readme_routes.sh` at each commit, in the doctrine gate → green.
  Commit: `ARCHOGEN-PROGRAM-0523 (leaf PROGRAM.70.1)`, with step 1's `ARCHOGEN-PROGRAM-0522`

- ID: `PROGRAM.70.2`
  Status: `done` — started and closed `2026-10-10`; `PROGRAM.70` closes with it
  Goal: `README-ROUTES` counts the histories in `docs/reviews/`, not the stubs `REVIEW-HISTORY` proves.
  Acceptance: an exact archive stub is left out of a directory's file count and of nothing else, and a file that only
  looks like one is counted; an arm for each, failing first; the decision record's dated paragraph, `README_POLICY.md`'s
  row and the book's account changed with it.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `git ls-files docs/reviews | wc -l` at `2493bf8` → `16`, of a ceiling of 16, 12 of them
    stubs: a new design's history is refused, *"docs/reviews/: 17 tracked files, over its ceiling of 16"*, as
    `PROGRAM.69` met it. The first new arm, an exact stub beside four files, run on `2493bf8`'s script → *"expected
    exit 0, got 1 … docs/tasks/: 5 tracked files, over its ceiling of 4"*, `22 pass / 1 fail`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `git show 2493bf8:scripts/check_readme_routes.sh | grep -nF 'n=$((n +
    1))'` → `247`, the directory branch counting every tracked file beneath. WHY: the 16-file ceiling (`PROGRAM.36`) bounded the histories a reader faces, before
    an archive left stubs that can never leave — sealed task-history files cite them — so every history ever reviewed
    would count against it for good.
  - [x] **FIX** — `scripts/check_readme_routes.sh`: `is_archive_stub`, `REVIEW-HISTORY`'s stub exactly — three lines and
    a final newline, the second blank, the third the archive line naming the file itself, its archive tracked — left
    out of a directory's file count alone. `README_POLICY.md`'s adoption note; the ceiling record's dated paragraph and
    How to apply; `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s two rows, re-measured; the book's annex.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_readme_routes.sh --self-test` → *"28 pass / 0 fail (28 arms)"*:
    an exact stub left out; a look-alike counted — no archive, a fourth line, text after the last newline, a second
    line not blank, another file named, another archive linked; a stub's bytes in the total. `python3
    target/p70/mutate.py` → seven mutations of the rule, each `killed` — its runner untracked, so not durable. `bash
    scripts/check_readme_routes.sh` → *"OK (24 destination(s) governed)"*, the folder 4 files against 16.
  - [x] **NO REGRESSION** — `bash scripts/run_self_tests.sh` → *"OK — 47 self-test(s) passed"*; `make focused` →
    `passed — 3 passed, 0 failed`; the doctrine gate at commit.
  - [x] **LOCKSTEP** — the policy's note, the ceiling record, the size inventory, the book's annex; this leaf,
    `PROGRAM.70`'s and both logs; the frontier, `docs/TASK_TREE.md`, `MEMORY.md` and `LIVE_STATUS.md`; `CHANGELOG.md`.
  Verification: `2026-10-10` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0524 (leaf PROGRAM.70.2)`

- ID: `PROGRAM.71`
  Status: `pending` — filed `2026-10-10` by `PROGRAM.69`'s review round 7, its P1
  Goal: the history's index is append-only in order as well as in presence, as its record says ("Rows are only ever
  appended").
  Reproduce / issue: `scripts/check_task_history.sh`'s leg 3 compares each committed index's rows with the current
  index's as sets, so a row inserted above committed rows passes; measured by the round's own runner, `exp/e2`, untracked
  and so not durable → *"OK"*, and a
  mutation of `add_rows` that puts new rows first passes every arm. Present at `fea69ad`, before `PROGRAM.69`.
  Direction: leg 3 also requires each committed index's rows to stand, in their order, as a prefix of the current
  table's — or the record and the gate's line say "present and unchanged" rather than "append-only"; an arm either way.

- ID: `PROGRAM.72`
  Status: `pending` — filed `2026-10-10` by `PROGRAM.69`'s review round 8, its P1
  Goal: a tree file in a sub-folder of `docs/tasks/` is judged as any tree file is, or is no tree file.
  Reproduce / issue: `scripts/check_task_history.sh`'s leg 6, its foreign-name and named-twice checks read
  `docs/tasks/*.md` alone, while `TASK-ACCEPTANCE` takes an owning leaf from any staged `docs/tasks/**/*.md`
  (`scripts/check_task_acceptance.sh`); the round's probe, its own untracked runner and so not durable: a live
  `M1.12.1.9` in `docs/tasks/m1/M1.md` passes the gate, at `docs/tasks/M1x.md` refused. Present at `fea69ad`.
  Direction: the gate walks `docs/tasks/` whole, `TEMPLATE.md` excepted — or `TASK-ACCEPTANCE` takes owners from
  `docs/tasks/*.md` alone; an arm either way.

- ID: `PROGRAM.73`
  Status: `pending` — filed `2026-10-10` by `PROGRAM.69`'s review round 9, its P1
  Goal: the history gate names every failure it meets as a breach, never a traceback.
  Reproduce / issue: `scripts/check_task_history.sh`'s top level catches `Unreadable` and `OSError` alone, so any other
  exception in the gate's own run ends in a traceback, exit 1 — git printing bytes that are not UTF-8 for
  `rev-parse --is-shallow-repository`, a `UnicodeDecodeError` at `shallow.decode()`; the round's probe, a fake `git` on
  the path, its own untracked runner and so not durable; the self-test's arm for a seal stopped that way (R9-2) expects
  the traceback today. Present at `fea69ad`.
  Direction: the top level names any exception as a breach, and that arm expects the named breach; an arm for the gate.

- ID: `PROGRAM.74`
  Status: `done` — filed, started and closed `2026-10-10`
  Goal: the book has room to grow with the code it describes.
  Reproduce / issue: `PROGRAM.70.2`'s first commit attempt, the doctrine gate → *"README-ROUTES: docs/book/: 458863 bytes
  in total, over its ceiling of 458752"*, a two-sentence addition to the annex; replacing a sentence instead, and the
  index regenerated, left `git ls-files docs/book | xargs cat | wc -c` → `458517`, 235 bytes of room. `M3.6.6.3`'s chapter took most of the rest
  the same day, and `M3.6.6.4`, `M3.3` and `M3.5` each owe the book their account.
  Direction: the measurement, and the book's layering (`docs/decisions/decision_book-in-layers.md`) — chapters and
  annexes — weighed against a dated raise of its total in that record, as the reviews folder's was.
  Decided `2026-10-10`: a raise, to 589 824 bytes, by the record's dated paragraph. Measured at `bfabefa`, `git
  ls-files docs/book | xargs cat | wc -c` → `458750`, two below the ceiling; `git ls-tree -r -l a274131 docs/book`
  summed → `395389`, the first book commit after the last raise; `git log a274131..bfabefa -- docs/book/src` → 38
  commits: 63 361 bytes in a week, about 1.7 KB a commit, as the record foresaw. Annexes move text, not remove it, so
  no layering frees room; the index, generated, is 44 785 bytes of it. The record asks the next raise to weigh a split.
  Verification: `2026-10-10` — `bash scripts/check_readme_routes.sh` → *"OK (24 destination(s) governed)"*, and its
  self-test → *"28 pass / 0 fail"*, the policy's cell and its decision's maximum agreeing; the doctrine gate at commit.
  Commit: `ARCHOGEN-PROGRAM-0526 (leaf PROGRAM.74)`

- ID: `PROGRAM.75`
  Status: `pending` — filed `2026-10-10` by `PROGRAM.54`
  Goal: the scaffold's handoff census judges by a property that tells a background job from a respawned helper.
  Reproduce / issue: `PROGRAM.54`'s reproduction — `scripts/check_no_background_jobs.sh`, the scaffold's, names a
  helper alive for seconds, and its verdict depends on the instant it runs; archogen samples twice in a wrapper of its
  own, which the census's next update cannot undo, but every other project the scaffold serves meets the same.
  Direction: an outbound report to the scaffold's owner, `bedrock`, as a tracker under `docs/feedback/bedrock/`
  shaped as the LinkedSpec one is — a self-contained issue whose `repro.sh` exits 0 while the census names a helper of
  a few seconds' life — proposing the second sample; nothing written into `bedrock` itself
  (`decision_repository-boundary-read-only.md`).

## Roadmap coverage map

Every roadmap unit has exactly one owning tree. This table is the answer to "where does
roadmap item X live?".

| Roadmap unit | Owning tree | Note |
| --- | --- | --- |
| §12 M0 — charter, boundary, target, examples | [`M0`](M0.md) | queue packages 1, 2, 4 |
| §12 S0 — early executable generation | [`S0`](S0.md) | queue package 3 |
| §12 M1 — eADL description foundation | [`M1`](M1.md) | queue packages 5, 6 |
| §12 M2 — one engine realization + controls | [`M2`](M2.md) | queue packages 7, 8, 9 |
| §12 M3 — joint resolver + checked plan | [`M3`](M3.md) | queue package 10 |
| §12 M4 — generated system + simulator | [`M4`](M4.md) | queue package 11 |
| §12 M5 — physical execution | [`M5`](M5.md) | board/emulator evidence |
| §12 M6 — reuse and extension | [`M6`](M6.md) | catalog reuse measurement |
| §12 M7 — first supported release | [`M7`](M7.md) | release packaging |
| §10.4 programmatic interface — engine API, wasm binding, MCP server | [`API`](API.md) | added by director ruling `2026-09-28`; §10.2's CLI stays the human interface and becomes a consumer of the same contract |
| §11 workstream F — engineering operations | `PROGRAM` | this tree |
| §14 agent workflow, review, CI tiers | `PROGRAM` | `PROGRAM.3` |
| §15 versioning and change management | `PROGRAM` | `PROGRAM.5`, `PROGRAM.6` |
| §16 reuse/optimization measurement | [`M6`](M6.md) | baseline recorded at M6 |
| §17 risk decisions and stop/rework criteria | `PROGRAM` | routed per trigger to its tree |
| §19 prior art and source ledger | `PROGRAM` | `PROGRAM.5` |
| §20 definition of completion | [`M7`](M7.md) | release exit gate |

## Fixture ownership map (F01–F30)

| Fixture | First gate | Owning tree | Leaf |
| --- | --- | --- | --- |
| F01 valid sub-HW/sub-OS imports | M1 | [`M1`](M1.md) | `M1.4` |
| F02 circular imports / conflicting exports | M1 | [`M1`](M1.md) | `M1.4` |
| F03 zero clock frequency / incompatible units | M1 | [`M1`](M1.md) | `M1.3`, `M1.28` |
| F04 relevant capability undescribed | M1 | [`M1`](M1.md) | `M1.5` |
| F05 irrelevant capability undescribed | M1 | [`M1`](M1.md) | `M1.5` |
| F06 contradictory offered/absent declarations | M1 | [`M1`](M1.md) | `M1.5` |
| F07 invalid functional refinement | M1 | [`M1`](M1.md) | `M1.6` |
| F08 two exclusive requests, one resource | M3 | [`M3`](M3.md) | `M3.3` |
| F09 conflicting ownership | M3 | [`M3`](M3.md) | `M3.3` |
| F10 unsupported implementation path | M3 | [`M3`](M3.md) | `M3.4` |
| F11 solver resource limit | M3 | [`M3`](M3.md) | `M3.4` |
| F12 corrupted plan / changed requirement | M3 | [`M3`](M3.md) | `M3.5` |
| F13 counter rollover, ambiguous horizon | M4 | [`M4`](M4.md) | `M4.5` |
| F14 deadline already expired | M4 | [`M4`](M4.md) | `M4.5` |
| F15 interrupt pending while masked | M4 | [`M4`](M4.md) | `M4.5` |
| F16 context preservation under preemption | M4/M5 | [`M4`](M4.md) | `M4.6`, `M5.3` |
| F17 unknown interference, refuse assurance | M2/M4 | [`M2`](M2.md) | `M2.6` |
| F18 scheduling positive/negative fixtures | M2 | [`M2`](M2.md) | `M2.3` |
| F19 cost changed, old bound retained | M4 | [`M4`](M4.md) | `M4.8` |
| F20 linked image exceeds or overlaps RAM | M4 | [`M4`](M4.md) | `M4.4` |
| F21 stack observation sold as a bound | M4 | [`M4`](M4.md) | `M4.8` |
| F22 replay identity mismatch | M4 | [`M4`](M4.md) | `M4.7` |
| F23 intentional shared misconception | M5 | [`M5`](M5.md) | `M5.4` |
| F24 new composite functional kind | M6 | [`M6`](M6.md) | `M6.3` |
| F25 locked rebuild after catalog update | M6 | [`M6`](M6.md) | `M6.4` |
| F26 missed deadline / stack guard / trap | M4/M5 | [`M4`](M4.md) | `M4.6` |
| F27 boundary classification corpus | M0/M1 | [`M0`](M0.md) | `M0.3` |
| F28 small description to executable | S0 | [`S0`](S0.md) | `S0.4` |
| F29 repeated preemption cost ledger | M2 | [`M2`](M2.md) | `M2.5` |
| F30 trust-dependency drift gate | M3/M4 | [`M3`](M3.md) | `M3.6`; acceptance and its costs `M2.7.6.5`, `M3.6.5`, `M4.7`, `M4.8` |

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `PROGRAM.69` | `active` | its review open: round 12 next, on the committed tool, its seal already in place |
| 2 | `PROGRAM.34` | `pending` | **low, awaiting the director** — nine repositories nested in `vendor/linkedspec` are off their recorded commits since the `2026-09-27` adoption, and `REPOSITORY-BOUNDARY` sees only the first level; the restore discards third-party working trees, so it waits for a yes |

The second row waits on the director's yes. The pending leaves beside them —
`PROGRAM.53`, `.71`, `.72`, `.73` and `.75` — are filed and owned. Every closed
leaf's outcome is its row in the Commit Log below, and its full record is sealed under `docs/task-history/PROGRAM/`.

## Decisions

- `2026-09-13`: one tree per roadmap milestone, plus this `PROGRAM` tree for the
  cross-cutting substrate. Rationale: a milestone is the roadmap's own unit of exit-gate
  evidence, so a tree per milestone makes the frontier and the gate the same object.
- `2026-09-13`: the engine carries no external Rust dependencies
  (`docs/decisions/decision_zero-dependency-engine-core.md`). §4.4 makes every dependency
  shared between generator and checker a reviewable trust event, §10.3 requires locked
  offline builds, and §5.5 makes diagnostic wording part of the user contract.
- `2026-09-13`: the §10.2 command surface is declared **once, as data**
  (`crates/archogen-cli/src/spec.rs`); help text is rendered from it and the parser validates
  against it, so documented and accepted options cannot diverge.
- `2026-09-13`: `docs/book/src/` is NOT a code path here. The neutral `TASK-ACCEPTANCE`
  default treats any `src/` segment as Rust source; the project seam
  `.doctrine/code_paths.txt` states this repository's real shape instead of editing the
  portable check. Found by the gate refusing this tree's own first commit (`PROGRAM.1.1`).
- `2026-09-13`: crates are created when a consumer needs them, per `ROADMAP.md` §4.2, not
  up front as eleven empty shells. The responsibility names in §4.2 are the naming
  convention for when each split happens.

## Open Questions

- **What is the push cadence?** The layer-A template provides the field — `MEMORY_ARCHITECTURE.md:193`,
  `(ahead of origin: <N>; push at ~<threshold>)` — and `MEMORY.md` has never filled it in. Nothing
  mechanical reads it: `grep -rniE 'rev-list --count|origin/main|push.threshold|ahead of origin'
  scripts/*.sh xtask/src/main.rs` → no match. The doctrine is qualitative only: "**Push regularly** —
  the remote is your crash insurance; an unpushed commit dies with the machine" (`:239`), "The single
  point of failure is **not committing / not pushing**" (`:432`). The only numeric threshold anywhere
  is the operator's *batch* rule (BWFSC, default 100 slices) — and the PNT loop has no fixed BWFSC by
  definition, so under PNT that trigger can never fire. That is the structural reason the branch went
  unpushed from `origin/main`'s `32e6b14` (`2026-09-13`) at ≈4.9 commits/day — `git rev-list --count
  origin/main..HEAD` for the live number, which is deliberately not written down because it moves
  with every commit.
  **Ruled `2026-09-28`.** The number is [[decision_push-cadence]]'s **Threshold** field, its one copy, and
  `bash scripts/push_cadence.sh` reports the live distance against it (`PROGRAM.23`). The recommendation
  put to the director was `25` commits or `7` days, whichever came first; the director chose a commit
  count, and the consequence at the measured rate is recorded in the decision rather than argued again
  here. An ungated threshold is the unenforced-prose class
  `docs/knowledge/a-rule-only-in-the-prompt-is-enforced-nowhere.md` describes, which is why the check exists.

## Blockers

- ~~The branch cannot be pushed under `COMMIT.md`'s own precondition~~ — **cleared `2026-09-30` by
  `PROGRAM.10.1`**: the emulator step is quarantined under `M2.8`, so `make integration` reads `incomplete`,
  which step 2 permits a push past after reading what it names. `M2.8.3.4` removed the cause the same day: the
  step passes, and so does the tier (`make integration` → `12 passed, 0 quarantined` on `2026-10-06`).
- `PROGRAM.31` and `PROGRAM.32` wait on the director's ruling on the findings record's §8.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-13` | `PROGRAM.1` | `scripts/check_doctrines.sh` | `13/13 green` |
| `2026-09-13` | `PROGRAM.1.1` | `scripts/check_doctrines.sh` staged | `red → green` |
| `2026-09-13` | `PROGRAM.2` | `make check` + `make gate` + `mdbook build` | `28 tests pass; 13/13 green` |
| `2026-09-27` | `PROGRAM.12` | `git check-ignore -v`, `git status --porcelain`, `git ls-files .qwen`, `make gate` | `ignore matches at .gitignore:31; status carries no untracked row; 0 tracked paths ignored; 13/13 green` |
| `2026-09-27` | `PROGRAM.16` | the policy copied and the copy **diff-verified** against its read-only source rather than read; the §7 adoption sweep run against this repository; all three entrypoints re-grepped afterwards; tiers and the gate re-run | body `diff -q` identical, digest `9f99df25209c43af` on both sides; 406 lines / 27 263 bytes; sweep found 18 checks / 8 with RED arms / **10 without** → `PROGRAM.18`; `AGENTS.md` line 11 carries an explicit list, so it was edited after the leaf's first draft claimed otherwise; 13 doctrines green; `make focused` exit `0`, 421 passed / 0 failed |
| `2026-09-27` | `PROGRAM.19` | the artifact inventory measured before and after; a residue census over every deleted path; the retained vendor build, one instrument self-test, the focused tier and the doctrine gate all re-run afterwards | ≈1.4 GB released: `.app-data` 3.5 GB → 2.2 GB, `target` 815 MB → 799 MB (then 816 MB once the suite recreated its scratch); all 7 deleted paths `gone`; `bins` → both binaries resolve; `reference` → the documented two-form result; `LS-002 --self-test` → `9/9`; 13 doctrines green; `make focused` exit `0`, 421 passed / 0 failed. ⛔ The first post-cleanup tier run **failed** and was reproduced, not dismissed → `S0.7` |
| `2026-09-28` | `PROGRAM.19` (second run) | the trigger read off `docs/ARTIFACT_CLEANUP.md` rather than assumed; a full inventory taken before any deletion; a residue census over every deleted path; `git grep` for each retention candidate's consumers; the focused tier re-run **cold**, with the scratch it consumes already deleted | ≈18 MB released: `target` 873 MB → 855 MB, `.bin` files 715 → 693, all 7 deleted paths `gone`; **2.2 GB retained on evidence** — `pgen-generated-before-remeasure` is read by `LS-004`'s `remeasure.sh:225-227`, and `target/debug/incremental` holds at most four generations per crate across 121 directories, which is cargo's retention and not residue; `make focused` → `passed — 3 / 0 / 0` cold; 13 doctrines green; `git status --porcelain` empty after the deletions |
| `2026-09-28` | `PROGRAM.21` | **no code changed — this is the finding's severity re-measured, not its fix.** Three instruments over the commit that had just landed: a `grep -c` census of ticked ROOT CAUSE boxes in `docs/tasks/M1.md`; the check's **own awk**, extracted from `scripts/check_task_acceptance.sh` and run over the file with `kw="root.?cause"`, printing the line it captures and whether that box is ticked; and a **mutation** — `M1.13.1`'s ROOT CAUSE box unticked in a scratch copy under `target/tmp/m113/`, with the same awk re-run over both files and the captures compared | the census returns **31** ticked boxes where this leaf recorded 24; the awk captures **line 53, ticked=1**, which is leaf `M1.1`'s box from `2026-09-13`, while `M1.13.1`'s sits near line 1470; and the mutation gives an **identical capture for both files**, so `M1.13.1`'s boxes provably cannot affect the verdict for commit `cd355ef` — five Rust files staged, `task-acceptance: OK`, `exit=0`. ⛔ The severity claim on this leaf is therefore superseded from *latent* to **active**: the gate read a thirteen-day-old checklist belonging to a different leaf and reported that it had checked the staged change |
| `2026-09-29` | `PROGRAM.25` | **docs-only, and the point of the run was to falsify the finding before filing it.** `command -v pdftotext pdftk mutool qpdf gs`; `pdftotext -v`; `pdftotext -f 1 -l 6` on the FE310-G002 datasheet to stdout; both semulith censuses (`git grep` excluding `vendor/`, and a filesystem walk of `vendor/`); `.gitmodules`; and a resolution check on every citation the new record makes | ⛔ **The finding as raised was false and the measurement is what caught it**: `pdftotext` resolves to `/opt/homebrew/bin/pdftotext` (Xpdf **4.06**), and the datasheet's first page extracts, so chipdoc's board PDFs are readable here and §3.2's `board-first` rows are **not** blocked on tooling — `read_file`'s bridge cannot see the host `PATH`, and its "not installed" message describes the bridge. A blocker on tree `M5` was drafted on that false premise and is **not** filed; what is filed is the route, in `TOOLBOX.md` and beside the chipdoc record's inventory. Semulith census: **0** archogen tracked files name it, **40** files inside the vendored submodule do, and `.gitmodules` confirms `vendor/linkedspec` is a submodule — so `git grep` alone returns a false negative and the census needs both halves. All four citations resolve; `make gate` → `13/13 green` |
| `2026-09-29` | `PROGRAM.21` | the check rewritten leaf-scoped, then **its own RED arms run before anything was claimed** — and the arms' first oracle found unsound, so three mutations were run over the finished script (`bash -n` first, then `--self-test`); both historical commits **replayed as fixtures** built from `git show <commit>:<path>` in throwaway repos, each run pristine and then with the committing leaf's ROOT CAUSE box unticked, against `HEAD`'s check and the new one side by side; `make focused`; `make gate`; the knowledge map regenerated | ⛔ **The arms' first run scored `4 pass / 5 fail` and the four passes were on `exit 127`** — the check was never found, because `$0` was relative and every arm `cd`s into a throwaway repo, and the oracle was `rc -ne 0`. Mutation B2 reproduces that false green deliberately (`4 pass / 5 fail`, arms 1/3/4/5 ✅ on 127) while mutation B, with the exact oracle, gives `0 pass / 9 fail`. Restored: `diff -q` silent, `9 pass / 0 fail`, `exit=0`. Mutation A (restore the first-leaf fallback) → `8 pass / 1 fail`, arm 4 only, so the refusal is load-bearing. **Replay, the finding:** `cd355ef` and `3a6bbb9` both give OLD `exit=0 OK` **pristine and mutated** — byte-identical, so neither commit's own boxes could affect the verdict — while NEW gives `exit=0` pristine naming `M1.13.1`/`M1.13.2` and `exit=1` mutated naming the same leaf. Root cause confirmed at `HEAD`: `if (inbox) exit` at line 111, **32** ticked ROOT CAUSE boxes in `docs/tasks/M1.md`, and that awk captures **line 53** — leaf `M1.1`, `2026-09-13`. `make focused` → `passed — 3 / 0 / 0`; `make gate` → `13 doctrines green`; ⚠️ the draft had also silently **widened `DEFAULT_SIG`** with a token that does not exist (`\bspindb\b`) and mis-dated a historical comment — both caught by diffing the preserved blocks against `HEAD` and restored byte-identical |
| `2026-09-29` | `PROGRAM.20` | **docs only, no code staged — a sixth shape of the class recorded, and its three copies corrected.** A census over the frontier surfaces rather than the two lines spotted by reading: an `awk` over `docs/TASK_TREE.md`'s row heads against each `docs/tasks/<TREE>.md` `## Current Frontier` order-1 row; `git ls-files '*.md'` piped to `grep -n -o` for successor clauses; `git show --stat 7c59b0b -- docs/TASK_TREE.md` for the copy the closing commit never touched. Recovery post-conditions after the force-quit measured rather than assumed: `git status --short` empty, `git rev-parse HEAD` = `91f329d`, `wc -c git_message_brief.txt` = `0` and untracked, `git config core.hooksPath` = `.githooks`. Then the baseline re-derived rather than carried: `cargo test --all` → **492 passed, 0 failed, 37 suites**; `make focused` → `passed — 3 / 0 / 0`; `scripts/check_doctrines.sh` → `13/13 green`; `scripts/check_no_background_jobs.sh` → `handoff: OK` | ⛔ The class fired a **sixth** time and the shape is new: not a figure but an ordered **sequence**. `docs/TASK_TREE.md`'s `PROGRAM` row named `PROGRAM.21` as the frontier one commit after that leaf's status became `done` — `7c59b0b` staged **0** lines of the index that restates it, the same evidence shape as the `S0` instance above — and both that row and `LIVE_STATUS.md`'s `M1` row named `.13.4` as its own successor, because a closure rewrote the frontier sentence and left the old clause behind. **10 of 12** index rows agreed with their tree file before the correction and **11 of 12** after; `M5`'s `DISAGREE` is the census pattern reading the blocker its cell names, recorded rather than dropped. Nothing in the tree compares an index row against the tree it indexes, and this leaf's acceptance enumerated figure-shaped text only — so it is widened on the leaf rather than left to miss the shape that just fired |
| `2026-09-29` | `PROGRAM.19` (third run) | the trigger read off the record's own commit timestamp rather than its date-only line; a full inventory before any deletion; a residue census over the deleted paths; the unexpected item **identified against a read-only sibling repository** before deletion; the focused tier, the whole suite and the doctrine gate all re-run **cold** with the scratch they consume already deleted | ≈11 MB released: `target` 957 MB → 949 MB, then 965 MB once verification recreated the scratch; all five sampled paths `gone`. `target/sync-backup-2026-09-21` matched **no** committed state here (`DIFFER` on 4 of 4 against both `a4cbab5^:` and `a4cbab5:`) and was proven **byte-identical to `bedrock`'s `HEAD`** copies (`MATCH` on 4 of 4), so it was the incoming scaffold and not a backup — deleted on that evidence, and identifying it exposed `PROGRAM.26`. **2.2 GB retained on evidence**, with two new retentions: `build/riscv-virt.dtb` needs the pinned emulator to regenerate and `M2.8.2` compares against it, and `target/s0-demo/base` is cited by a closed leaf and needs a full `archogen build`. `make focused` → `passed — 3 / 0 / 0`; `cargo test --all` → **492 passed, 0 failed**; 13 doctrines green, with `target/doctrine_scratch` recreated by the run |
| `2026-09-29` | `PROGRAM.20` | **docs only, no code staged — two more instances of the older shapes recorded, and the mechanism that found them is a fourth one this leaf's acceptance does not name.** The per-verdict distribution measured from the cases themselves rather than from the chapter; `git log -S 'holds 25 worked cases'` for the commit that wrote the figure; `git ls-tree --name-only 6df022f docs/semantics/cases/ | wc -l` for whether it was true then; `git rev-list --count 538fe3b..HEAD` for how long it had been false | the chapter said **25** and tabulated 5 / 10 / 5 / 4 / 1, against a measured 11 / 7 / 6 / 4 / 2 over 30 cases: the sentence false for **71 commits** since `538fe3b` (`M1.9`, which added four cases and touched neither figure) and **three of five table rows** wrong, with the table's own sum contradicting the sentence above it. ⛔ Neither existing sweep pattern sees the table — a bare count in a cell is neither `N of M` nor a digit followed by a size noun. ⭐ The mechanism that found them is cheaper than any sweep: **census the surfaces a change moves, at the commit that moves them** — a change to a population knows which population it touched, while a sweep has to guess what a figure is about. Both figures corrected to the measurement **and** gated in `ARCHOGEN-M1-0094`, so they now classify as *gated* rather than *unregistered* |
| `2026-09-29` | `PROGRAM.20` | **docs only, no code staged — the seventh shape given a measured false instance rather than a hypothetical one.** The fixture reconstructed both ways from `s0_reader.rs`'s own writing code (`text.trim_end().strip_suffix(')')` against the same text with the trailing newline kept), each written to `build/s0/malformed.eadl` and checked with the real binary; `git show HEAD~1:examples/s0-heartbeat/system.eadl` for the pre-retrofit description, so this leaf's own change is excluded as the cause | the `trim_end()` fixture gives `--> build/s0/malformed.eadl:40:36` with the last real line rendered, and the newline-preserving one gives `41:1` over an empty line — byte-for-byte what `docs/book/src/s0.md` showed. So the block was rendered from a fixture shape **no tracked command writes** and was false before anything moved; the same chapter's `--locked` block was abridging its hint's last sentence with no marker saying so. Both re-rendered by `M1.13.4.2`. ⭐ The instance widens the design work: a moved figure is the cheap half, and the expensive half is that a transcript quotes a *run* whose input (`build/s0/malformed.eadl`) exists nowhere tracked, so neither a reader nor a gate can reproduce the output — six of the fourteen line-numbered transcript lines re-rendered from a tracked command over a tracked file, and this one needed the fixture reconstructed from the test that writes it |
| `2026-09-29` | `PROGRAM.20` | **docs only, no code staged — an eighth shape recorded and its artifact corrected: `CHANGELOG.md`'s own ordering rule.** Found by the fourth mechanism (census the surface a change moves) while inserting an entry at the top. Four censuses rather than a read: `git show <c> -- CHANGELOG.md \| grep -m1 '^@@'` over the five suspect commits for the anchor; `git log -S` for the commit that placed the entry; `git rev-list --count b88812d..HEAD` for its age; and a `python3` inversion count over every id line in the file. The correction was verified as a **pure move** two ways rather than by eye — `collections.Counter` equality over the line multiset in the script that performed it, and the inversion census re-run over the whole file | `ARCHOGEN-M1-0092`'s entry sat **above six newer ones**, so the surface the director reads for "what just happened" opened with work seven commits old. Root cause is a mechanism and not a slip: five consecutive commits (`fc3655c`, `d0692f2`, `145baab`, `2ae744a`, `6a31da5`) all inserted at `@@ -52,6 +52,N @@`, the same anchor, because the insertion point was "just after the first entry" and the file looked identical each time — a wrong anchor that produces a plausible file is self-concealing. **74** ids, **1** out-of-order pair before the correction and **0** after, so no second misplacement was left behind; the order had been false for the **7** commits since `b88812d`. ⛔ What the instance adds is not the sequence but the **population**: `CHANGELOG.md` is a fourth live surface, named in the acceptance only as a place a figure may be legitimately *registered as a record*, and the fifth shape was the first falsification of the enumerated three. The acceptance is widened again on the leaf — every live surface the commit stages, derived from the staged diff, because a list is the thing that keeps being wrong |
| `2026-09-29` | `PROGRAM.27` (filed) | **docs only in this tree — a registered doctrine gate reproduced failing to fail.** The reproduction is on the leaf and was taken on the real tree, not on a fixture: the pending note moved out of `docs/semantics/migrations/` entirely, the baseline left amended at 72 constructs, and `scripts/check_language_freeze.sh` re-run. Then the two causes located in the check's own source: the notes population grep and `names_construct`'s `*all*` case, each read against `docs/semantics/migrations/README.md`'s form template. Restored and proven: `diff -q` silent on the note and `language-freeze: OK (72 …)` with it back in place | `language-freeze: OK` with **no note at all** and an amended baseline — the exact waiver leg B was written to close, `--emit` and green. `grep -rl '^- status:[[:space:]]*pending' docs/semantics/migrations` → `README.md`, whose form template carries `- status: pending \| applied`; `grep -n '^- constructs:'` on it → `36:- constructs: <construct id>, <construct id>   — or: all`, which the `*all*) return 0 ;;` substring case reads as covering every construct. ⛔ Nine RED arms and none of them could see it: all nine point `LANGUAGE_FREEZE_NOTES` at `$work/notes`, a scratch directory holding only the fixture note, so the arms proved the mechanism and never the deployed population — `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md` one level up. Filed at priority **high** and sequenced ahead of `PROGRAM.11`: a gate that cannot fail is worse than no gate, because the registry is what a reader consults to find out what is enforced. Two of the migration README's workflow steps are false as written and are part of the same fix, measured in all three note states (`pending` before `--emit` → leg A red, which the README calls green; `pending` after → green; `applied` after → green only because leg B is inert) |
| `2026-09-29` | `PROGRAM.20` | **docs only, no code staged — two instances of the seventh shape routed in by `M1.31`, and the population they belong to measured.** Every `error[…]` block in `docs/book/src/*.md` classified by its first `-->`; the tracked ones re-run through `archogen check` and compared verbatim | 20 blocks: **5** over a tracked input (4 verbatim; `checking.md:93` wraps a hint and drops the verdict line), **7** over an untracked input, **8** with no `-->` at all — including `checking.md:115`, which drops the location, source and marker lines the command prints. 15 of 20 cannot be checked, and a population keyed on `-->` cannot see 8 of them |
| `2026-09-29` | `PROGRAM.27` | the false green reproduced on `docs/semantics/migrations/` itself with `7f46f4d`'s baseline as `HEAD`'s side; four real-directory cases after the fix, and the real tree before and after the flip; `--self-test` (16 arms); six mutations P-A–P-F with `cmp` restoration; every gate's `--self-test`; `make focused`; `cargo test --all`; the doctrine driver; `mdbook build` | **rc 0 → rc 1** with no note; the four cases OK / refused / OK / refused as specified; leg C fired on `M1.28.2`'s note on its first real run; 16 / 0; P-E shows the deployed-directory arm red under the shipped logic, P-F shows the two formerly vacuous arms refused *for another reason*; ten gates' self-tests all green; **575 passed / 0 failed**; 13 doctrines green |
| `2026-09-29` | `PROGRAM.20` | **docs only — the book population re-measured, keyed on the `$ archogen check` line.** Every `error[…]` block classified by its command line (or its first `-->` when it has none), the tracked ones re-run through the command | 20 blocks: **9** tracked (7 verbatim), **6** untracked, **5** location-free — 11 uncheckable, was 15; the movement is `M1.31`'s and `M1.29.2`'s re-rendered transcripts |
| `2026-09-29` | `PROGRAM.11` | `git grep` for the rule in the entrypoints and for any vendor doctrine at `b9f6e22`; the real checkout's pin, local-only commits (tags excluded) and status; the naive census on one nested checkout; the new check and its `--self-test`; five mutations R-1–R-5 with `cmp` restoration; the doctrine driver | the rule absent from both entrypoints and no doctrine looking at `vendor/` → both fixed; the real checkout clean (**0** local-only commits), the naive census **4 040** on a nested one; **9 / 0** arms; every mutation fails exactly its arm, one arm found vacuous first and re-seeded; all doctrines green |
| `2026-09-29` | `PROGRAM.18.1` | the census of armed scripts at `61e5f09`; `FROZEN-EVALUATION --self-test`; seven mutations F-1–F-7 with `cmp` restoration; the real tree; the doctrine driver | `check_task_acceptance.sh` already armed, seven registered controls not; **9 / 0** arms on synthetic cases; every mutation fails its own arm (F-5 three, the untracked arm among them); all doctrines green; `PROGRAM.29` filed for six `mktemp` sites in `/tmp` |
| `2026-09-29` | `PROGRAM.18.2` | `bash scripts/selftest_spine.sh`; the `git grep` of the two self-locating roots; nine mutations S-1–S-9 on the real scripts with `cmp` restoration; the handoff tool after the run | **34 / 34** after one clean arm caught a stub-regex gap (first run 33 / 1); every mutation fails exactly its own arm; `handoff: OK`; `PROGRAM.18` closed |
| `2026-09-29` | `PROGRAM.24` | the leaf's census; a strong-shape census over all nine members; the new check before and after the chapter fixes; `--self-test`; four mutations C-1–C-4; `cargo test --all --no-fail-fast`; the doctrine driver | `rt-analysis` named nowhere, and `xtask` named without a path — **2 of 9**, where a first census over-reported 5 by missing directory citations; **8 / 8** arms after one unfailable arm was re-staged; every mutation fires; **602 / 0** |
| `2026-09-29` | `PROGRAM.28` | `git grep` for any runner of `--self-test` at `96636ac`; the new runner and its `--self-test`; four mutations U-1–U-4; `cargo xtask verify --tier integration` and the three other tiers; `cargo test -p xtask` | nothing ran them → **14** discovered and passing in 42 s; 6 / 6 arms; every mutation fires; integration 7 passed / 1 failed (emulator, `M2.8`), the book's transcript re-rendered from it |
| `2026-09-29` | `PROGRAM.29` | a logging `mktemp` on `PATH` over the driver and the self-test runner, before and after; `git grep -c mktemp 7bf85ba` over every tracked script; `SCRATCH-LOCALITY` and its 16 arms; nine mutations S-1–S-8b; every changed script re-run against its before-state; the LS-001 precondition falsified; `cargo test --all`; a residue census | **26 of 26** calls off the volume before; **18 sites in 15 files** where the filing census had 8 in 6; after, every project-owned call under `target/`, 0 of 58 paths left; eight mutations fire, S-8 survives for a stated reason; two self-tests that the move broke — one gone partly vacuous — fixed at the cause; 602 / 0 |
| `2026-09-29` | `PROGRAM.5` | `git grep` for URLs and ledger links at `cb8180d`; the ledger gate on the real tree and its 20 arms; fifteen mutations M-1–M-15; two falsifications on the real tree (the QEMU pin moved, a citation deleted); `mdbook build` and each entry's anchor in the HTML; `BOOK-ANCHORS` | 0 URLs and 0 ledger links before; 11 entries, 7 pins carried, 21 citations in 12 files after; every mutation fails its own arm; both real-tree falsifications refused at the named entry and line; the Rust channel, mdBook and CI actions found unpinned → `PROGRAM.30` |
| `2026-09-29` | `PROGRAM.9.1` | the extended tier at `39aa2fc` against `cargo +nightly miri --version`; every one of 34 test targets under Miri alone (600 s cap); the tier end to end; F-1 arm without UB, F-2 stale exclusion, F-3 missing component; the probe-form test and its mutation; the home cache after the run | Miri reported unavailable while installed → found on `nightly`; 541 passed, 4 ignored, 0 failed; five corpus walks over 300 s left out on measurement; `miri` ✅ in 851 s; all three falsifications refused; sysroot under `target/` |
| `2026-09-29` | `PROGRAM.9.2` | the harness's first runs (smoke, and 5 seeds × 20 000 with overflow checks); the step on a fixed and a fresh seed; seeded defects F-A–F-D and F-C without overflow checks; `cargo test -p xtask` | found `M1.36` on the first run (and `M1.34`, `M1.35` while designing it) — all three fixed first; then 8 properties and 6 arms green on every seed; every seeded defect fails its own property or arm; F-D's restore nearly skipped (untracked file) |
| `2026-09-30` | `PROGRAM.9.3` / `PROGRAM.9` | `cargo xtask mutate` twice (the second naming each kill's tests); harness arms H-1–H-4 through temporary entries; `cargo test -p xtask`; the `extended` tier end to end | 8 of 8 as expected, each kill by the test written for its defect, the `lcm`→`max` blind spot surviving the harmonic corpus; every arm refused as designed, `crates/` clean after each; 14 / 0; **`extended` passed for the first time** |
| `2026-09-30` | `PROGRAM.6.1` | the register gate on the real tree and its 12 arms; seven mutations V-1–V-7; `FORMAT` bumped on the real code; the book build | no register before; 7 entries and 7 declared versions after — the seventh (`idealized-zero-overhead/1`) found by the gate, missed by the planning census; every mutation fires; the bump refused at its entry |
| `2026-09-30` | `PROGRAM.6.2` | the frozen table blessed and checked; its exit codes against `M1.36`'s binary census; the D ≤ T mutation; a new description; a stale row; the catalog entry through `cargo xtask mutate` | 120 verdicts frozen, identical to the binary census; 20 moved and named under the mutation; the new and the vanished each named; the catalog entry killed by the verdict test |
| `2026-09-30` | `PROGRAM.6.3` / `PROGRAM.6` | both goldens blessed and checked; the lexer's own test; F-1–F-4 with restores; the two new catalog entries and the whole catalog; `cargo test --all` | 20 provenance key paths, re-derived from the renderer; the cost contract frozen; every falsification refused, blessing never overwrites; 11 of 11 mutations as expected; 620 / 0 |
| `2026-09-13` | `PROGRAM.2.1` | backfilled `2026-09-30` (`PROGRAM.13`), transcribed from the leaf's own checklist: after the change, the scoped census git grep -c -i 'osgen' -- . ':(exclude)Cargo.lock' ':(exclude)docs/tasks/PROGRAM.md' \ ':(exclude)ROADMAP.md' ':(exclude)CHANGELOG.md' ':(exclude)MEMORY.md' returns **no output**, `rc=1`. The four … | `done` at `af4e6dc`; the full evidence is the leaf's acceptance checklist |
| `2026-09-13` | `PROGRAM.3` | backfilled `2026-09-30` (`PROGRAM.13`), transcribed from the leaf's own checklist: five named commands, and the honest picture they produce: `cargo xtask verify --tier focused` → `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built`, `exit=0`; `--tier integration` → `incomplete — 5 passed, 0 failed, 1` … | `done` at `f7175e8`; the full evidence is the leaf's acceptance checklist |
| `2026-09-28` | `PROGRAM.22` | backfilled `2026-09-30` (`PROGRAM.13`), transcribed from the leaf's own checklist: `git status --short` after the change → empty, with `build/riscv-virt.dtb` and `build/riscv-virt.dts` still present on disk (`git check-ignore -v build/riscv-virt.dtb` names the new rule); `make gate` → `=== all doctrines green ===` over … | `done` at `f199e1f`; the full evidence is the leaf's acceptance checklist |
| `2026-09-13` | `PROGRAM.4` | backfilled `2026-09-30` (`PROGRAM.13`), transcribed from the leaf's own checklist: `SUMMARY.md` now carries five parts that mirror the programme (what eADL describes · writing a description · what the engine may claim · generating and running a system · using the toolchain), and `presence.md` names … | `done` at `2733efe`; the full evidence is the leaf's acceptance checklist |
| `2026-09-30` | `PROGRAM.13` | the unlogged-leaf census by leaf ID over every tree, before and after; each derived commit against the commit its leaf records | 22 rows owed across `BOOTSTRAP`, `M1`, `M2`, `PROGRAM` → 0; every commit derived from `git log` matched its leaf's own record |
| `2026-09-30` | `PROGRAM.15` | the register gate's first run on the real tree; its 15 arms; eight mutations R-1–R-8 | the cross-vendor index said 5 open / 2 blockers against a closed register — corrected; 15 / 0; every mutation fires |
| `2026-09-30` | `PROGRAM.17.1` | the guide and doctrine at their current fsmgen revisions; a three-axis measurement of every live document; the copied body against its source | the copy byte-identical (`cmp`); `LIVE_STATUS.md` 42 110 bytes with a 30 256-byte row, `CHANGELOG.md` 3 428 lines — both recorded as debt or a decision |
| `2026-09-30` | `PROGRAM.17.2` | the no-loss proof (ids declared, commit rows present, pre-trim digests); the checker and its 11 arms; six mutations; the pre-trim `LIVE_STATUS.md` put back | 42 110 → 2 324 bytes and 30 256 → 220 on the longest line; every arm and mutation as designed; the old file refused on bytes and width, not on lines |
| `2026-09-30` | `PROGRAM.17.3` / `PROGRAM.17` | growth since `2026-09-27` from `git show 4d6d002:<file>`; closed-leaf share of `M1.md` and `PROGRAM.md` by an `awk` over leaf blocks | 77% of `M1.md` and 75% of `PROGRAM.md` are closed leaves; §8 written, `PROGRAM.31`/`.32` filed blocked; `PROGRAM.17` closed |
| `2026-09-30` | `PROGRAM.20.1` | the gate's first run; its 10 arms; seven mutations; each real snapshot's head moved | `M2.9`'s status contradiction found and corrected in its own commit; every arm and mutation as designed; all three snapshots genuinely checked |
| `2026-09-30` | `PROGRAM.20.2` | the transcript test before and after; the backlog grown and made stale; the renderer mutation through the catalog | 5 of 12 checkable transcripts differed, two contradicting their own prose; all 12 exact after re-rendering; both ratchet directions refused; the mutation killed by the book's own transcripts |
| `2026-09-30` | `PROGRAM.20.3` / `PROGRAM.20` | a census of figure-shaped text in the live surfaces; the gate's 14 arms; eight mutations; the `S0.8` shape staged on the real tree | 96 phrases over 28 files as backlog; the added figure refused; the register's own first row refused as false; `PROGRAM.20` closed |
| `2026-09-30` | `PROGRAM.10.1` | the emulator check against HEAD's script and the new one; its 7 arms and 3 mutations; the runner's judgement table and 2 catalogued mutations; the whole `integration` tier | HEAD exit `1`, now `20`; every arm and mutation as designed; the tier `incomplete` with the emulator quarantined under `M2.8`, a failing step now naming its cause |
| `2026-09-30` | `PROGRAM.10.2` | every self-test under a simulated bare runner; the spine harness's identity; the runner's two new arms and one mutation | 23 of 23 pass bare — the suspected defect is not one (`repo()` sets a local identity); the bare environment kept, and shown to fire |
| `2026-09-30` | `PROGRAM.10.3` | `--provisioned` both ways on a PATH without QEMU; the policy script's 7 arms and four mutations; the real job run locally | `1` provisioned, `20` not; one mutation equivalent under `pipefail`, the rest killed; the real run found its own arms clobbering its log — fixed, then exit `0` with the gap annotated |
| `2026-09-30` | `PROGRAM.10.4` | the pinned tools installed here from verified downloads, QEMU built from source; the provisioner's arms, three mutations and a real refused digest; the job rehearsed from a fresh checkout | QEMU `11.1.1` built and offering `virt`; every arm and mutation as designed; the rehearsal passes, `incomplete` with the emulator annotated — the runner's own userland still unobserved (`.10.5`) |
| `2026-09-30` | `PROGRAM.23` | the threshold's copies before and after; the check's 10 arms and three mutations; the real distance | 8 copies outside the record, now 0; every arm and mutation as designed; a push not yet due |
| `2026-09-30` | `PROGRAM.26` | a census of every neutral file against upstream and its base; two dry runs on clones; five spine arms against the adopted and the replaced updater; `make gate` | 21 identical, 7 project-carrying, only the updater and the version behind; nothing of ours modified by either run; the arms pass on the adopted updater and all five fail on the old one |
| `2026-09-30` | `PROGRAM.30` | the moving refs at HEAD; the installed toolchains; the pins held by the ledger; the book builder's arms and mutations; the tier and the suite on `1.95.0` and `1.98.0` | three moving refs, and a newer compiler waiting in CI; all pinned, 9 pins held; the tier unchanged; both toolchains pass the same suite |
| `2026-09-30` | `PROGRAM.33` | the backlog re-keyed by content; five arms; two catalogued mutations | the edit that broke `S0.8`'s run now moves nothing; a changed, new, fixed or copied block each caught; both mutations killed |
| `2026-09-30` | `PROGRAM.19` (fourth run) | the trigger off the record's commit timestamp; a full inventory; each scratch name censused against the tracked tree and its leaf's status read; a residue census; the provisioner, the focused tier, the whole suite and the gate, cold | 6.3 GB → 4.1 GB; ten of ten paths `gone`; both CI tools `already in place`; `passed — 3 passed`; 742 passed / 0 failed over 62 suites; all green — rows added by `ARCHOGEN-PROGRAM-0197`, since `0196` omitted them |
| `2026-09-30` | `PROGRAM.35.1` | the body `cmp`'d and digested against fsmgen's; the page reviewed against the contract; 19 links resolved; `LIVE-SNAPSHOTS` one below the survivor on each axis, at equality and at the ceilings, on the real tree | body identical, `77a1e934…`; one duplication removed; 84 / 4 173 / 125 → ceilings 110 / 6 144 / 200; rc 1, 1, 0, 0 |
| `2026-09-30` | `PROGRAM.35.2` | the derived population on the real tree; six mutations of the real registry; the gate's 16 arms; every self-test bare; scratch locality; the project doctrines | 18 destinations governed, two debt owners; each mutation refused with one named breach, and restored green; 16 of 16; 32 self-tests passed; OK; OK |
| `2026-09-30` | `PROGRAM.31` | the seal with its own proof; an independent rebuild in Python against the files saved before; the ordering defect traced to the initial commit and its move proved; 13 RED arms; the routes and the other gates | both ledgers byte for byte, twice; the template block carried verbatim; 13 of 13; 19 destinations, debt only `PROGRAM.32` |
| `2026-09-30` | `PROGRAM.36` | the breach reproduced with the ceiling restored; the move's digest against `HEAD`'s; `README-ROUTES`; the doctrine enforcer; the book built | rc=1, then rc=0 with 20 destinations governed; digests equal; all green; rc=0 |
| `2026-09-30` | `PROGRAM.37` | the move against `HEAD`'s tail; review sections measured by section; `README-ROUTES`; the doctrine enforcer | identical; 13 482 bytes moved, the rest summaries; rc=0, 318 625 bytes; all green |
| `2026-09-30` | `PROGRAM.38` | the folder at `HEAD` and at `38d8634`; the independent review of the raise; what grew, by file; `README-ROUTES`; the doctrine enforcer | 324 165 over 28, against 250 243 over 23; 18 findings, 4 defects, all answered; the reviewed designs from 72 269 to 136 093 bytes; rc=0; all green |
| `2026-09-30` | `PROGRAM.32.1` | a read-only audit of every script that reads `docs/tasks/`, each against a sealing commit; closed subtrees measured in `M1` and `PROGRAM` | the four held checks pass it; `TASK-ACCEPTANCE`, `README-ROUTES` and `LESSON-PROMOTION` shaped the design; 37 of 39 and 32 of 39 subtrees closed |
| `2026-09-30` | `PROGRAM.32.2` | both self-tests; a dry run sealing copies of `M1` and `PROGRAM`, then the gate; a code change owned by a sealed leaf; two mutations; every self-test; the enforcer | 14 of 14 and 10 of 10; byte-for-byte reconstruction, 69 files and 112 stubs; refused as sealed; each mutation red; 35 of 35; all green |
| `2026-09-30` | `PROGRAM.32.3` | the seal of `M1` and `PROGRAM` with its proof; an independent re-derivation of both trees from `HEAD`; `shasum` over 69 files; `README-ROUTES`; the enforcer | byte for byte, 69 files and 112 stubs; equal; 0 mismatched; 21 destinations, no debt; all green |
| `2026-09-30` | `PROGRAM.32.4` | an independent review of the tool and the seal; the tool rebuilt; 25 arms; two mutations; the real history through every leg; the figures re-measured at their commits | the seal accepted, 10 findings, all answered; 25 of 25; each mutation red; 69 files and 112 stubs proven; corrected |
| `2026-09-30` | `PROGRAM.40` | a committed forgery in a scratch repository; the self-test; a mutation; the real ledgers; the enforcer and every self-test | refused; 15 of 15; the arm red when mutated; OK; all green, 35 of 35 |
| `2026-09-30` | `PROGRAM.39` | the folder measured; the records moved and every path rewritten; `DECISION-INDEX` and `README-ROUTES` with their self-tests; the crate's tests; every self-test; the enforcer | 358 185 before; no stale path left; 5 of 5 and 20 of 20; 34 passed; 36 of 36; all green |
| `2026-09-30` | `PROGRAM.41` | the folder measured at `b8448c6`; the seal with its reconstruction proof; the gate and its self-test; four mutations on a copy; `README-ROUTES` with the new row; every self-test; the enforcer | 376 904 before, 368 643 after; `02 04 08 10, 9745 bytes`; `20 pass / 0 fail`; each mutation red on its own arm, the proof reached by none; all green |
| `2026-09-30` | `PROGRAM.41.1` | the gate rewritten against the review's D1–D18; its self-test; a matrix of 42 mutations on a copy; the real history; every self-test; the enforcer | `54 pass / 0 fail`; 41 of 42 red, the 42nd unreachable apart from the rows leg; `decision-history: OK (4 sealed section(s) …)`; 37 self-tests passed; all green |
| `2026-09-30` | `PROGRAM.42` | the `-s ours` merge built on both gates in scratch repositories, before and after the fix; both self-tests; four mutations on copies; both gates on the real trees; every self-test; the enforcer | both passed the merge before and refuse it after; `27 pass / 0 fail`, `17 pass / 0 fail`; each mutation red on its own arm; `task-history: OK`, `history-ledgers: OK`; 37 self-tests passed; all green |
| `2026-10-03` | `PROGRAM.49` | the folder measured before and after; the refusal reproduced by the enforcer; the enforcer after the raise | `264219` over `262144`; all green |
| `2026-10-03` | `PROGRAM.50` | the moved block's sha256 before and after; the folder measured before and after; the enforcer | `5785eb49…57cc9` both; 395 564 → 352584; all green |
| `2026-10-03` | `PROGRAM.51` | the pre-`M2.22` file through the gate's method; `rustfmt --check` on stdin; ten RED arms; the tree; every gate's self-test; the enforcer | differs, `cmp rc=1`; `--check` exits 0 over a diff; 10 / 10 arms; 159 files canonical; 44 self-tests passed |
| `2026-10-05` | `PROGRAM.52.1` | twelve RED arms, one of which first failed and found a quote outside every block unchecked; the tree; every gate's self-test; the README routes after compacting two rows; the enforcer | 12 / 12 arms; 0 hand-offs, 0 ledgers, clean; all self-tests passed; `DOCTRINE_ENFORCEMENT.md` 36 537 bytes under 36 864; all green |
| `2026-10-05` | `PROGRAM.52.2` | the two ledgers through `HANDOFF-LEDGER`; the prose census of each record against its ledger; each rewritten leaf against its previous text; the enforcer | 46 hand-offs, each quoted; no hand-off left in prose; one dropped item restored; all green |
| `2026-10-05` | `PROGRAM.55` | the self-test; three legs broken in turn; the seal on the closed catalog history and `cmp` against `HEAD`; both folders measured; every gate's self-test; the focused tier; the enforcer | 38 / 38 arms; each broken leg failed its arms; byte for byte; `docs/reviews/` 317 648 bytes; all green |
| `2026-10-05` | `PROGRAM.55.1` | the review's constructions armed; the self-test; a mutation matrix, one per refusal and leg; the real repository; every gate's self-test; the focused tier; the enforcer | 65 / 65 arms; 38 of 38 killed; OK; all green |
| `2026-10-05` | `PROGRAM.56` | both folders measured before and after; every reference to the old path searched; the routes; the focused tier; the enforcer | decisions 393 061 → 349 217, specs 253 792 → 300 288; 0 stale references; all green |
| `2026-10-06` | `PROGRAM.57` | the trigger off the record's commit; a full inventory; each name looked up for an owner, a citation and its leaf's status; a residue census; the adopted policies' sources re-hashed; the provisioner, the focused tier, the whole suite and the gate, cold | 11 GB → 1.6 GB; twelve of twelve paths `gone`; 1 454 953 object files in `deps` → `PROGRAM.58`; sources unchanged; `already in place`; `passed — 3 passed`; 1 218 passed / 0 failed over 91 suites; all green |
| `2026-10-06` | `PROGRAM.58` | the leak reproduced in a scratch crate under each mode; the `OSO` entries read with `nm -ap`; Cargo's and rustc's pages quoted; before and after on the real `target/debug`; the way back; the integration tier, Miri's arm, then the focused tier and the gate on the committed tree | +6 098 objects per relink → 0; `at ./src/main.rs:1:52` restored by the override; `passed — 12 passed, 0 quarantined`; the arm fired; all green |
| `2026-10-06` | `PROGRAM.59` | two `git grep` censuses over the live documents, each hit read in context; the lifting commit's stat; `git log -L` on the book's lines; the censuses re-run; the book, the focused tier and the gate | four stale present-tense claims corrected, the dated history left; every remaining hit true; all green |
| `2026-10-06` | `PROGRAM.61` | the new catalog test before and after the fix; the entry alone; the whole catalog; the focused tier and the suite | FAILED naming the one entry, then ok; killed; 152 as expected in 177 s; 1237 passed, 0 failed |
| `2026-10-06` | `PROGRAM.62` | every closure a removed paragraph narrates looked up in its tree's Commit Log; the folder measured before and after; the stated order; the gate | 31 of 31 with a row; 819 056 → 786 004 bytes; OK; all green |
| `2026-10-06` | `PROGRAM.63` | the ledger's self-test before and after; a trial seal of `M3.1`; the routes and their self-test; the decision index; the focused tier | 12 / 2 then 14 / 0; OK; OK, 20 / 0; OK; passed |
| `2026-10-06` | `PROGRAM.10.5.1` | the catalog tests with no identity outside the scratch repository, before and after; the same in the ordinary environment; the new entry alone; the whole suite with no identity; the focused tier | 2 passed, 6 failed, then 9 passed; 9 passed; killed by the new test; 1274 passed, 0 failed; passed |
| `2026-10-06` | `PROGRAM.10.5.2` | the test's two sources on an ELF target, before and after, and on the host; the blob in what they build; the test; the whole suite; the focused tier | `Size expression must be absolute.`, then `rc=0` on both formats; 1 in each `.rlib`; ok; 1274 passed, 0 failed; passed |
| `2026-10-06` | `PROGRAM.10.5` | the first and second CI runs, each job's log fetched by the API; the two assumptions | `focused` and `integration` failed at `tests`, 7 of 106, `enforce` passed; then every job `success`, `tier integration: passed — 12 passed`; both held |
| `2026-10-06` | `PROGRAM.64` | the catalog tests in the rehearsal's checkout of `a3c0cbd`, the old job environment and the new; the same on `e2baf65`; the job home afterwards; a whole rehearsal of `a3c0cbd` | 8 passed, then 2 passed, 6 failed; 9 passed; `.gitconfig` alone; 57 passed, 49 failed: the six, and 43 (38 `trust`, 5 `catalog_build`) `PROGRAM.65`'s |
| `2026-10-06` | `PROGRAM.65` | a whole rehearsal of `HEAD` and of `a3c0cbd` beside the repository; the directory afterwards; the scratch gate | passed, 12 of 12; the runner's six and no other; gone; OK |
| `2026-10-06` | `PROGRAM.66` | the gate's self-test, and with each protection removed; the real tree before and after the script was staged; the doctrine gate | 7 / 0, then 5 / 2 and 6 / 1; refused on itself, then OK; all green |
| `2026-10-10` | `PROGRAM.68` | the trigger off the record's commit; a `git grep -F` census of every `target` and `target/tmp` entry; a residue census; the provisioner, `make focused`, `cargo test --all -q` and the doctrine driver, cold; the three adopted policies' sources | ≈4.0 GB released, `target` 5.6 GB → 1.6 GB, `.bin` 2 648 → 29; twelve of twelve sampled paths `gone`; `m3634` present though its leaf said removed, now true; focused passed 3/0; **1 304 passed, 0 failed** over 97 suites; all doctrines green; no policy source changed |
| `2026-10-10` | `PROGRAM.69` | the census at `fea69ad`; the self-test; twenty named mutations and round 3's twenty, and a sweep of the core in three runs, runners untracked; the seal of `M1`, `M2`, `M3`, `PROGRAM`; the gate over all 170 files; every gate's self-test; review rounds 2–6 | 274 483 bytes, 261 610 in 28 subtrees; 66 arms; 20 killed, 16 and 4 equivalent; 182 of 196 on round 5's tool, the 14 reasoned; 46 leaves, 255 871 bytes out of `docs/tasks/`; OK, the 142 older seals included; 47 passed; 5, 2, 3, 2, 5 defects, each answered, the review open |
| `2026-10-10` | `PROGRAM.69` (rounds 7, 8) | the self-test, each new arm's rule mutated; the gate over 170 files; the doctrines | 69, then 71 arms, every new rule's mutation killed; OK; all green |
| `2026-10-10` | `PROGRAM.69` (round 9) | the self-test, the new arms failing first on `7ad8e6b`'s tool; nine mutations of round 9's rules; the gate over 172 files; the doctrines | 74 pass / 2 fail — the working tree's first new arms beside `7ad8e6b`'s core, not durable — then 77 / 0; each killed by an untracked runner, not durable; OK; all green |
| `2026-10-10` | `PROGRAM.70.2` | the routes self-test, its stub arm failing first; seven mutations of the rule; the real routes; every self-test; focused | 22 / 1, then 28 / 0; each killed; OK, 4 files of 16; 47 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 10) | the self-test, the new arms failing first beside `0a7ec7f`'s core; eleven mutations of round 10's rules; the gate over 172 files; every self-test; the doctrines | 79 pass / 5 fail, the working tree's and not durable, then 85 / 0; each killed by an untracked runner, not durable; OK; 47 passed; all green |
| `2026-10-10` | `PROGRAM.60` | the census test on the tree, on `947cdc2`'s record, on a record naming a path that is gone; focused | 1 passed; 1 failed, six named; 1 failed, it named; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.67` | the module's tests; two catalogued mutations; a temporary hanging entry under a 30 s limit; the crate's tests, clippy, fmt; focused | 8 passed; each killed; reported as a hang in 30.1 s, the file restored, untracked and not durable; 165 passed, clean, clean; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 11) | the new arms beside `b87adbc`'s core; the self-test; six mutations of round 11's rules; the gate over 172 files; every self-test; the doctrines | 82 pass / 9 fail, the working tree's and not durable; 90 / 0; each killed by an untracked runner, not durable; OK; 47 passed; all green |
| `2026-10-10` | `PROGRAM.54` | the scaffold's census with a helper alive and gone; the wrapper's self-test; a copy without the second sample; every self-test; focused | exit 1 then not named; 2 / 0; 1 / 1, untracked; 48 passed; `passed — 3 passed, 0 failed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `PROGRAM.1` | `ARCHOGEN-PROGRAM-0002 (leaf PROGRAM.1)` | roadmap seeded into ten trees |
| `PROGRAM.1.1` | `ARCHOGEN-PROGRAM-0002 (leaf PROGRAM.1)` | code-path seam, same commit |
| `PROGRAM.2` | `ARCHOGEN-PROGRAM-0003 (leaf PROGRAM.2)` | `archogen` CLI shell, §5.5 exit codes |
| `PROGRAM.12` | `ARCHOGEN-PROGRAM-0050 (leaf PROGRAM.12)` | harness-local scratch ignored; the stale `.gitignore` pointer to a nonexistent shared `SETUP.md` corrected; `PROGRAM.13`/`.14` logged from the census |
| `PROGRAM.16` | `ARCHOGEN-PROGRAM-0058 (leaf PROGRAM.16)` | the claim-verification policy adopted as `docs/CLAIM_VERIFICATION.md` — copied verbatim and diff-verified, restated locally per its own §7.6, registered in all three entrypoints; its §7.4 sweep found ten controls with no RED arm → `PROGRAM.18` |
| `PROGRAM.19` | `ARCHOGEN-PROGRAM-0060 (leaf PROGRAM.19)` | ≈1.4 GB of regenerable artifacts released and `docs/ARTIFACT_CLEANUP.md` started, so "is a cleanup due?" is answerable; the cleanup's own verification exposed `S0.7`; one unexpected item investigated and flagged rather than deleted |
| `PROGRAM.19` | `ARCHOGEN-PROGRAM-0074 (leaf PROGRAM.19, second run)` | ≈18 MB more released, and **2.2 GB retained on evidence**: a frozen instrument's backup, which `remeasure.sh` treats an existing copy of as a reason to keep it, and cargo's own incremental cache. Runs append to the standing owner rather than becoming one leaf per day |
| `PROGRAM.21` | `ARCHOGEN-PROGRAM-0081 (leaf PROGRAM.21)` | **filed, not fixed** — the severity re-measured from *latent* to **active** on commit `cd355ef`: five Rust files staged, `task-acceptance: OK`, and the box it read was leaf `M1.1`'s from `2026-09-13`. A mutation proves `M1.13.1`'s own boxes could not have changed the verdict. No code in this commit, so the gate stays unsound and the fix is still owed |
| `PROGRAM.25` | `ARCHOGEN-PROGRAM-0083 (leaf PROGRAM.25)` | two findings raised in conversation and owned by nothing are now tracked — census, both halves, because `git grep` alone is a false negative here: `git grep -il semulith -- ':!vendor' \| wc -l` → **0** archogen tracked files, `grep -ril semulith vendor/ \| wc -l` → **40** files inside the submodule, which `.gitmodules` confirms `git grep` skips. And one of the two findings was **false as raised**: `pdftotext` is on the host (`/opt/homebrew/bin`, Xpdf 4.06) and chipdoc's board datasheets extract fine, so `read_file`'s "not installed" is a bridge limitation and §3.2's `board-first` rows are not tooling-blocked. The route is a `TOOLBOX.md` row and a note beside the chipdoc inventory; an `M5` blocker drafted on the false premise is **not** filed. `docs/decisions/reference_sibling-project-semulith.md` records that `../semulith` exists, is read-only, and names archogen as its consumer — a pointer with no analysis, which is what was declined |
| `PROGRAM.21` | `ARCHOGEN-PROGRAM-0086 (leaf PROGRAM.21)` | **fixed, not filed**: `TASK-ACCEPTANCE` is leaf-scoped. The owner comes from `TASK_ACCEPTANCE_LEAF` or the `(leaf <ID>)` token in the pending message's subject through the new `.doctrine/commit_message_file` seam, and the check **refuses** when it cannot tell — never falling back to the first checklist, because that fallback *was* the defect. The staged-paths signal was priced first and is dead: **1 of 7** real code commits. Both historical commits replayed as fixtures, pristine and mutated, old check against new: the old verdict is byte-identical either way, the new one moves and names the right leaf. Nine `--self-test` arms; ⛔ their first oracle scored **4 passes on `exit 127`** and the false green is reproduced deliberately as mutation B2, promoted into `verify-the-mutation-applied`. This commit is the new check's first real exercise — it gated itself |
| `PROGRAM.20` | `ARCHOGEN-PROGRAM-0088 (docs)` | **an instance recorded, not the register built** — three stale frontier copies corrected, in `docs/TASK_TREE.md` (the `PROGRAM` head, which named a `done` leaf, and the `M1` successor clause) and `LIVE_STATUS.md` (the `M1` successor clause), and the sixth shape filed on the leaf with its census and its acceptance widened: an ordered sequence, which a register scoped to figure-shaped text could not have seen |
| `PROGRAM.19` | `ARCHOGEN-PROGRAM-0090 (leaf PROGRAM.19, third run)` | ≈11 MB more released and **2.2 GB retained on evidence**. The run's finding is not a deletion but an identification: a directory that matched no committed state here was proven byte-identical to the upstream scaffold's, so it was deleted on evidence — and reading it exposed that this repository's `update_scaffold.sh` is two minor versions behind a fix upstream made to exactly that hazard → `PROGRAM.26` |
| `PROGRAM.20` | `ARCHOGEN-PROGRAM-0093 (leaf PROGRAM.20)` | **an instance recorded, not the register built** — the seventh shape now has a live false instance: a book transcript rendered from a fixture shape no tracked command writes, false before anything moved, plus a second block abridging its output with no marker. The design work is widened on the leaf, because a transcript quotes a run and this one's input exists nowhere a reader could rebuild |
| `PROGRAM.20` | `ARCHOGEN-PROGRAM-0095 (leaf PROGRAM.20)` | **two more instances recorded, and a fourth finding mechanism** — a corpus size false for 71 commits and a per-verdict table wrong in three of five rows, both in `docs/book/src/checking.md`, both found by censusing the surfaces a change moves rather than by sweeping for figure-shaped text, which is the mechanism this leaf's acceptance does not yet name. Both gated in the commit that found them |
| `PROGRAM.20` | `ARCHOGEN-PROGRAM-0101 (leaf PROGRAM.20)` | **an instance recorded and its artifact corrected, not the register built** — an eighth shape: `CHANGELOG.md`'s own "Newest first" rule, broken for **7** commits by five consecutive inserts at one wrong anchor, and invisible because each produced a plausible-looking file. Corrected as a **pure move** verified by line-multiset equality and by an inversion census over all **74** ids (1 → 0). ⛔ The instance falsifies the acceptance's *population* for the second time — `CHANGELOG.md` is none of the three live surfaces it enumerates — so the population becomes every live surface the commit stages |
| `PROGRAM.27` | filed by `ARCHOGEN-M1-0105 (leaf M1.28.2)` | **a registered doctrine gate that cannot fail, reproduced rather than suspected.** `LANGUAGE-FREEZE`'s explicitness leg — the one `M1.13.5` called "the one that matters", because it is what stops `--emit` being the waiver — prints `OK` on the real tree with an amended baseline and no migration note at all, because the notes population grep matches the directory's own README and `names_construct`'s `*all*` substring case reads that README's template as covering every construct. Filed by the first leaf to run the workflow, at priority high and ahead of `PROGRAM.11`; two of the README's workflow steps are false as written and are inside the same fix |
| `PROGRAM.20` | `ARCHOGEN-PROGRAM-0110 (leaf PROGRAM.20)` | **instances recorded, the register not built** — two abridged `checking.md` transcripts routed in by `M1.31`, and the first whole-population measurement of the book's rendered diagnostics: 15 of 20 blocks cannot be re-run, and 8 carry no `-->` for a population to key on |
| `PROGRAM.27` | `ARCHOGEN-PROGRAM-0111 (leaf PROGRAM.27)` | **`LANGUAGE-FREEZE` can fail on the real tree.** A pending note is a line exactly `- status: pending`, `constructs:` a list compared exactly, and a new leg refuses a note `HEAD` already carries as pending — so a note is a permission for one commit, and `M1.28.2`'s, open for five, is flipped. Two vacuous arms found and fixed (unsorted fixtures), every refusing arm now names its subject, and the classifier names the file it parsed. `PROGRAM.28` filed: no tier runs any gate's `--self-test` |
| `PROGRAM.20` | `ARCHOGEN-PROGRAM-0113 (leaf PROGRAM.20)` | **re-measurement recorded, and `M1.29.2`'s migration note flipped to `applied`** — the second commit of the two-commit lifecycle, which the freeze gate's spent-note leg refused the tree until it happened |
| `PROGRAM.11` | `ARCHOGEN-PROGRAM-0119 (leaf PROGRAM.11)` | **every other repository is read-only, stated and gated.** The rule in `CLAUDE.md` (both directions), and `REPOSITORY-BOUNDARY` on every commit: each vendored checkout this repository pins is at its pin with nothing committed, modified or created in it. Scoped on measurement to the pins this repository owns — the vendor's documented bootstrap legitimately dirties its nested checkouts — and with tags excluded from "local-only", because the naive census counted 4 040 phantom commits |
| `PROGRAM.18` → `PROGRAM.18.1` | `ARCHOGEN-PROGRAM-0120 (leaf PROGRAM.18.1)` | **`FROZEN-EVALUATION` is armed**: nine arms in `--self-test`, each seeding one breach in a scratch repository with synthetic case names and naming what it refuses. `PROGRAM.18` decomposed by ownership on a re-run census — the six scaffold-owned gates are armed from outside in `.18.2`, because an arm written into them is erased by the scaffold and cannot be sent upstream; `PROGRAM.29` filed |
| `PROGRAM.18.2` → `PROGRAM.18` | `ARCHOGEN-PROGRAM-0121 (leaf PROGRAM.18.2)` | **every registered control now has repeatable RED arms.** The six scaffold-owned universal gates, both drivers and the handoff tool are armed **from outside** by `scripts/selftest_spine.sh` — each run unmodified in a scratch repository with one seeded breach, 34 arms, nine mutations each failing its own arm — so the arms survive a scaffold sync that would erase them in place. `PROGRAM.18` closed; tier registration is `PROGRAM.28`'s |
| `PROGRAM.24` | `ARCHOGEN-PROGRAM-0122 (leaf PROGRAM.24)` | **the book is checked from the code's side too.** `BOOK-COVERAGE`: every workspace member, derived from `Cargo.toml`, is named in a chapter beside a path into it — a name alone does not count. Two members failed and are fixed: `rt-analysis` gains "Where it lives" in `analysis.md`, `xtask` its file in `verification.md` |
| `PROGRAM.28` | `ARCHOGEN-PROGRAM-0123 (leaf PROGRAM.28)` | **every gate's RED arms now run in a tier and in CI.** `scripts/run_self_tests.sh` discovers every armed gate by census, plus the outside harness, and fails on any failed arm; a `self-tests` step in `integration`. The book's stale tier transcript re-rendered from a real run |
| `PROGRAM.29` | `ARCHOGEN-PROGRAM-0124 (leaf PROGRAM.29)` | **scratch stays on this volume, and a gate says so.** 14 project-owned sites moved under `target/`; `SCRATCH-LOCALITY` reads every tracked script and Rust source; the scaffold's four sites recorded for its owner, not sent. The root manifest excludes `target`, so a Cargo fixture there is standalone |
| `PROGRAM.5` | `ARCHOGEN-PROGRAM-0126 (leaf PROGRAM.5)` | **§15's ledger exists and is checked.** A book chapter of 11 external sources at the versions they are pinned to; `SOURCE-LEDGER` derives every pin and every naming document. `PROGRAM.30` filed for the unpinned toolchain |
| `PROGRAM.9` → `PROGRAM.9.1` | `ARCHOGEN-PROGRAM-0127 (leaf PROGRAM.9.1)` | **the `miri` step runs, and proves it can fail first.** Found on `nightly` (it had been reported unavailable while installed); a seeded dangling-pointer read refused before every run; every test target timed under Miri, five corpus walks left out on measured cost; sysroot under `target/`. `PROGRAM.9` decomposed into `.9.1`–`.9.3` |
| `PROGRAM.9.2` | `ARCHOGEN-PROGRAM-0131 (leaf PROGRAM.9.2)` | **the `extended` tier fuzzes the reader and the exact arithmetic.** A dependency-free seeded harness, six known-false arms it must refute, eight properties; a fixed and a fresh seed, overflow checks on. It found three engine defects before its own commit (`M1.34`–`M1.36`) |
| `PROGRAM.9.3` → `PROGRAM.9` | `ARCHOGEN-PROGRAM-0132 (leaf PROGRAM.9.3)` | **the mutation controls are a catalog run on every `extended` tier**, and **`extended` passes** for the first time. `cargo xtask mutate`: eight entries, each checked to apply once, name its killing tests and restore byte for byte; S0.4's blind spot reproduced by command. `PROGRAM.9` closed |
| `PROGRAM.6` → `PROGRAM.6.1` | `ARCHOGEN-PROGRAM-0133 (leaf PROGRAM.6.1)` | **everything versioned is in one register, derived from the code.** Seven surfaces with what changes and pins each; F25's home named (`M6.4`); `VERSION-REGISTER` refuses an unannounced format, bump or stale entry. `PROGRAM.6` decomposed |
| `PROGRAM.6.2` | `ARCHOGEN-PROGRAM-0134 (leaf PROGRAM.6.2)` | **every description keeps its frozen verdict.** The census three engine fixes ran by hand is a test: 120 descriptions, exit code and diagnostic codes; a moved verdict fails unless the table is regenerated in the same change |
| `PROGRAM.6.3` → `PROGRAM.6` | `ARCHOGEN-PROGRAM-0135 (leaf PROGRAM.6.3)` | **each evidence format is held to its identifier** — a golden per identifier, never rewritten: the provenance's shape and the cost contract's text. `PROGRAM.6` closed: the register, frozen verdicts and format goldens |
| `PROGRAM.2.1` | `ARCHOGEN-PROGRAM-0021 (leaf PROGRAM.2.1)` | rename the command osgen -> archogen — backfilled `2026-09-30` (`PROGRAM.13`) from `git log`, `af4e6dc` `2026-09-13` |
| `PROGRAM.3` | `ARCHOGEN-PROGRAM-0029 (leaf PROGRAM.3)` | the five verification tiers become five commands — backfilled `2026-09-30` (`PROGRAM.13`) from `git log`, `f7175e8` `2026-09-13` |
| `PROGRAM.22` | `ARCHOGEN-PROGRAM-0067 (leaf PROGRAM.22)` | the roadmap's own example dirtied the tree — backfilled `2026-09-30` (`PROGRAM.13`) from `git log`, `f199e1f` `2026-09-28` |
| `PROGRAM.4` | `ARCHOGEN-PROGRAM-0033 (leaf PROGRAM.4)` | the book gets a shape, and its citations get checked — backfilled `2026-09-30` (`PROGRAM.13`) from `git log`, `2733efe` `2026-09-13` |
| `PROGRAM.13` | `ARCHOGEN-PROGRAM-0136 (leaf PROGRAM.13)` | **every closed leaf names its commit and its checks in its own tree** — 22 rows backfilled from `git log` and the leaves' own checklists; `BOOTSTRAP` gains a Verification Log |
| `PROGRAM.15` | `ARCHOGEN-PROGRAM-0137 (leaf PROGRAM.15)` | **a bug register says what its issues say, and its totals are recounts** — `FEEDBACK-REGISTER`; its first run found the cross-vendor index three days stale |
| `PROGRAM.17` → `PROGRAM.17.1` | `ARCHOGEN-PROGRAM-0138 (leaf PROGRAM.17.1)` | **the live-document size-containment doctrine is adopted, with a measured inventory** — a project-owned copy, an adoption note per surface, and the snapshots' chronology recorded as debt. `PROGRAM.17` decomposed |
| `PROGRAM.17.2` | `ARCHOGEN-PROGRAM-0139 (leaf PROGRAM.17.2)` | **the snapshots hold current state, and a checker bounds them on lines, bytes and longest line** — `LIVE_STATUS.md` 42 110 → 2 324 bytes after a no-loss proof; `LIVE-SNAPSHOTS`; `COMMIT.md` stops asking for history in a snapshot |
| `PROGRAM.17.3` → `PROGRAM.17` | `ARCHOGEN-PROGRAM-0140 (leaf PROGRAM.17.3)` | **the history lifecycle choices go to the director** — findings §8 with measurements and a recommendation; `PROGRAM.31`/`.32` filed blocked; `PROGRAM.17` closed |
| `PROGRAM.20` → `PROGRAM.20.1` | `ARCHOGEN-PROGRAM-0142 (leaf PROGRAM.20.1)` | **a restated order is a verified copy** — `STATED-ORDER` checks frontier rows, snapshot heads, successor lists and the changelog's order; its first run found `M2.9` stated two ways. `PROGRAM.20` decomposed |
| `PROGRAM.20.2` | `ARCHOGEN-PROGRAM-0143 (leaf PROGRAM.20.2)` | **every diagnostic the book shows is a real run** — a test re-runs each and requires it exactly; five blocks re-rendered, two of which contradicted their own prose |
| `PROGRAM.20.3` → `PROGRAM.20` | `ARCHOGEN-PROGRAM-0144 (leaf PROGRAM.20.3)` | **a figure added to a live document says what keeps it true** — `FIGURE-REGISTER`, a ratchet with a classifying register. `PROGRAM.20` closed: orders, transcripts and figures each have an instrument |
| `PROGRAM.10.1` | `ARCHOGEN-PROGRAM-0145 (leaf PROGRAM.10.1)` | **the emulator step is quarantined, not failed** — §14.3's quarantine as a runner row with its four fields; a failing step shows both streams |
| `PROGRAM.10.2` | `ARCHOGEN-PROGRAM-0146 (leaf PROGRAM.10.2)` | **the self-tests run as a bare runner would** — the suspected identity defect falsified; the instrument kept |
| `PROGRAM.10.3` | `ARCHOGEN-PROGRAM-0147 (leaf PROGRAM.10.3)` | **the blocking policy for `incomplete`** — recorded, and carried out by `--provisioned` and `scripts/ci_integration.sh` |
| `PROGRAM.10.4` | `ARCHOGEN-PROGRAM-0148 (leaf PROGRAM.10.4)` | **the `integration` job** — its tools built and installed at their pins from digest-checked downloads; rehearsed from a fresh checkout |
| `PROGRAM.23` | `ARCHOGEN-PROGRAM-0149 (leaf PROGRAM.23)` | **the push cadence is a report, not prose** — one copy of the threshold, the distance on demand, never a gate |
| `PROGRAM.26` | `ARCHOGEN-PROGRAM-0150 (leaf PROGRAM.26)` | **the scaffold updater never overwrites** — upstream's at `bedrock` `5af0c1c` plus one hunk; the spine at `0.10.0` |
| `PROGRAM.30` | `ARCHOGEN-PROGRAM-0151 (leaf PROGRAM.30)` | **the build environment is named, not dated** — `rustc 1.95.0`, actions by commit, the book's mdBook checked |
| `PROGRAM.33` | `ARCHOGEN-PROGRAM-0153 (leaf PROGRAM.33)` | **the transcript backlog names what a block is** — not the line it sits on |
| `PROGRAM.19` | `ARCHOGEN-PROGRAM-0196 (leaf PROGRAM.19, fourth run)` | **≈2.2 GB released** — the QEMU build tree the provisioner leaves after installing, the CI rehearsal's checkout, the doctrine scratch and closed leaves' probe files; every deletion looked up first, one active leaf's scratch kept |
| `PROGRAM.35.1` | `ARCHOGEN-PROGRAM-0197 (leaf PROGRAM.35.1)` | **fsmgen's README policy adopted, and the caps derived from the page** — 110 lines and 6 144 bytes held by `LIVE-SNAPSHOTS`, where the template's 300 / 16 384 had never been fitted |
| `PROGRAM.35.2` → `PROGRAM.35` | `ARCHOGEN-PROGRAM-0198 (leaf PROGRAM.35.2)` | **every route out of the README registered and bounded, or its debt owned** — `README-ROUTES` derives the routes from the page's links and its guards' actual hints; `PROGRAM.35` closed |
| `PROGRAM.31` | `ARCHOGEN-PROGRAM-0207 (leaf PROGRAM.31)` | **the changelog and the development notes are rolling ledgers** — sealed by entry count into digest-checked segments under `docs/history/`, as ruled on §8 by delegation |
| `PROGRAM.36` | `ARCHOGEN-PROGRAM-0211 (leaf PROGRAM.36)` | **the review histories get a home of their own** — `docs/reviews/`, registered as the decisions folder's overflow with its own ceilings; the debt `ARCHOGEN-M2-0210` recorded is paid |
| `PROGRAM.37` | `ARCHOGEN-PROGRAM-0218 (leaf PROGRAM.37)` | **the runtime variant's review history joins the others** in `docs/reviews/`; the debt `ARCHOGEN-M2-0217` recorded is paid |
| `PROGRAM.38` | `ARCHOGEN-PROGRAM-0222 (leaf PROGRAM.38)` | **the decisions folder's ceiling raised once**, to 40 files and 384 KiB, as the director's reviewed exception; `PROGRAM.39` owns the partition that replaces it |
| `PROGRAM.32.1` | `ARCHOGEN-PROGRAM-0224 (leaf PROGRAM.32.1)` | **the sealing design** — `docs/decisions/decision_task-tree-sealing.md`; the tool next |
| `PROGRAM.32.2` | `ARCHOGEN-PROGRAM-0225 (leaf PROGRAM.32.2)` | **the sealing tool and its gate** — `scripts/check_task_history.sh`, registered as `TASK-HISTORY`; `TASK-ACCEPTANCE` names a sealed owner as closed |
| `PROGRAM.32.3` | `ARCHOGEN-PROGRAM-0226 (leaf PROGRAM.32.3)` | **`M1` and `PROGRAM` sealed**: 112 leaves in 69 files under `docs/task-history/`; `docs/tasks/` bounded, its debt paid |
| `PROGRAM.32.4` | `ARCHOGEN-PROGRAM-0228 (leaf PROGRAM.32.4)` | **the sealing tool hardened** after its review: history-wide immutability, provenance, fail-closed slicing, no reopening; `PROGRAM.32` closed |
| `PROGRAM.40` | `ARCHOGEN-PROGRAM-0229 (leaf PROGRAM.40)` | **the history ledgers checked across history**, so CI catches a committed forgery as the hook does |
| `PROGRAM.39` | `ARCHOGEN-PROGRAM-0230 (leaf PROGRAM.39)` | **the decisions folder partitioned**: the catalog design in `docs/decisions/catalog/`, `DECISION-INDEX`, partition-aware `README-ROUTES` and its cap table |
| `PROGRAM.41` | `ARCHOGEN-PROGRAM-0234 (leaf PROGRAM.41)` | **settled sections sealed out of the decisions folder**: the findings register's §2, §4, §8 and §10 in `docs/decision-history/`, each heading kept above its stub; `DECISION-HISTORY`; opened by `ARCHOGEN-PROGRAM-0232` |
| `PROGRAM.41.1` | `ARCHOGEN-PROGRAM-0236 (leaf PROGRAM.41.1)` | the review filed as work: `PROGRAM.41` reopened, `PROGRAM.41.1` and `PROGRAM.42` filed |
| `PROGRAM.41.1` | `ARCHOGEN-PROGRAM-0238 (leaf PROGRAM.41.1)` | **`DECISION-HISTORY` hardened**: `--full-history`, CommonMark fences, rollback on any refusal, shallow clones refused, 54 arms each proven by a mutation; `PROGRAM.41` closed again |
| `PROGRAM.42` | `ARCHOGEN-PROGRAM-0239 (leaf PROGRAM.42)` | **`TASK-HISTORY` and `HISTORY-LEDGERS` read history whole**: `--full-history`, shallow clones and failed reads refused, the merge and a shallow clone as RED arms; the sealed folders kept from line-ending conversion |
| `PROGRAM.43` | `ARCHOGEN-PROGRAM-0259 (leaf PROGRAM.43)` | **the accepted designs moved to `docs/specs/`**, on the director's ruling: the decisions folder from 388 884 to 205 397 bytes, no ceiling raised |
| `PROGRAM.44` | `ARCHOGEN-PROGRAM-0278 (leaf PROGRAM.44)` | **the book opens with a tour**: one description to its running program and on to a board, honest about today, its copies held to their files |
| `PROGRAM.45` | `ARCHOGEN-PROGRAM-0282 (leaf PROGRAM.45)` | **the tour says how you interact with a system**: inputs, outputs, inspection; no login, shell or filesystem yet, with the roadmap's reasons; `PROGRAM.46` filed |
| `PROGRAM.46` | `ARCHOGEN-PROGRAM-0283 (leaf PROGRAM.46)` | **`COMMIT-LOG-ROWS`**: every work-unit commit has its Commit Log row, the pending one included; a backlog of 18 that may only shrink |
| `PROGRAM.47.1` | `ARCHOGEN-PROGRAM-0296 (leaf PROGRAM.47.1)` | **the book in layers, ruled and recorded** — plain words first, a one-minute summary, the precise rules; a live glossary, annexes, a generated index |
| `PROGRAM.47.2` | `ARCHOGEN-PROGRAM-0297 (leaf PROGRAM.47.2)` | **the glossary, kept live** — every acronym the book uses, defined; `BOOK-GLOSSARY` |
| `PROGRAM.47.3` | `ARCHOGEN-PROGRAM-0298 (leaf PROGRAM.47.3)` | **the index, generated** — every glossary word and every section, linked; `BOOK-INDEX` |
| `PROGRAM.47.2` | `ARCHOGEN-PROGRAM-0300 (leaf PROGRAM.47.2)` | **correction**: eADL spelled out, Extended Architecture Description Language, as `M1.13.3` recorded; the census that missed it was too narrow |
| `PROGRAM.47.4` | `ARCHOGEN-PROGRAM-0301 (leaf PROGRAM.47.4)` | **the runtime chapter in layers**, its mechanics and history in Annex A |
| `PROGRAM.47.5.1` | `ARCHOGEN-PROGRAM-0302 (leaf PROGRAM.47.5.1)` | **the introduction in layers** |
| `PROGRAM.47.5.2` | `ARCHOGEN-PROGRAM-0303 (leaf PROGRAM.47.5.2)` | **the tour**: the one-minute summary for engineers |
| `PROGRAM.47.5.3` | `ARCHOGEN-PROGRAM-0304 (leaf PROGRAM.47.5.3)` | ***Reading a description* in layers** |
| `PROGRAM.47.5.4` | `ARCHOGEN-PROGRAM-0305 (leaf PROGRAM.47.5.4)` | ***Describing a workload* in layers** |
| `PROGRAM.47.5.5` | `ARCHOGEN-PROGRAM-0306 (leaf PROGRAM.47.5.5)` | ***Checking a description* in layers** |
| `PROGRAM.47.5.6` | `ARCHOGEN-PROGRAM-0307 (leaf PROGRAM.47.5.6)` | ***What a report may claim* in layers** |
| `PROGRAM.47.5.7` | `ARCHOGEN-PROGRAM-0308 (leaf PROGRAM.47.5.7)` | ***What the scheduling checker establishes* in layers**, a two-task example worked in words |
| `PROGRAM.47.5.8` | `ARCHOGEN-PROGRAM-0310 (leaf PROGRAM.47.5.8)` | ***The catalog* in layers** |
| `PROGRAM.47.5.9` | `ARCHOGEN-PROGRAM-0311 (leaf PROGRAM.47.5.9)` | ***Where generated systems run* in layers**, and which boards, from a ledgered datasheet |
| `PROGRAM.47.5.10` | `ARCHOGEN-PROGRAM-0312 (leaf PROGRAM.47.5.10)` | ***The `archogen` command line* in layers** |
| `PROGRAM.47.5.11` | `ARCHOGEN-PROGRAM-0313 (leaf PROGRAM.47.5.11)` | ***The engine API* in layers** |
| `PROGRAM.47.5.12` | `ARCHOGEN-PROGRAM-0314 (leaf PROGRAM.47.5.12)` | ***Verifying the toolchain* in layers**, its repository checks moved to Annex B |
| `PROGRAM.47.5.13` | `ARCHOGEN-PROGRAM-0315 (leaf PROGRAM.47.5.13)` | ***The boundary* in layers** |
| `PROGRAM.47.5.14` | `ARCHOGEN-PROGRAM-0316 (leaf PROGRAM.47.5.14)` | ***The supported profile* in layers** |
| `PROGRAM.47.5.15` | `ARCHOGEN-PROGRAM-0317 (leaf PROGRAM.47.5.15)` | ***The use cases* in layers** |
| `PROGRAM.47.5.16` | `ARCHOGEN-PROGRAM-0318 (leaf PROGRAM.47.5.16)` | ***Kinds and schemas* in layers** |
| `PROGRAM.47.5.17` | `ARCHOGEN-PROGRAM-0320 (leaf PROGRAM.47.5.17)` | ***Quantities and units* in layers** |
| `PROGRAM.47.5.18` | `ARCHOGEN-PROGRAM-0321 (leaf PROGRAM.47.5.18)` | ***Modules and composition* in layers** |
| `PROGRAM.47.5.19` | `ARCHOGEN-PROGRAM-0322 (leaf PROGRAM.47.5.19)` | ***Presence, absence, and relevance* in layers** |
| `PROGRAM.47.5.20` | `ARCHOGEN-PROGRAM-0323 (leaf PROGRAM.47.5.20)` | ***Refinement* in layers** |
| `PROGRAM.47.5.21` | `ARCHOGEN-PROGRAM-0324 (leaf PROGRAM.47.5.21)` | ***What is versioned* in layers**, its entries kept where the register gate reads them |
| `PROGRAM.47.5.22` | `ARCHOGEN-PROGRAM-0325 (leaf PROGRAM.47.5.22)` | ***The S0 early generation path* in layers** |
| `PROGRAM.47.6` | `ARCHOGEN-PROGRAM-0326 (leaf PROGRAM.47.6)` | **the index without the layer headings**: 342 entries to 270, four words that named no topic gone |
| `PROGRAM.47.5.23` | `ARCHOGEN-PROGRAM-0327 (leaf PROGRAM.47.5.23)` | ***What this project relies on from outside* in layers**; `PROGRAM.47.5` and `PROGRAM.47` closed, every chapter in layers |
| `PROGRAM.48` | `ARCHOGEN-PROGRAM-0392 (leaf PROGRAM.48)` | the book's total-bytes ceiling raised to 448 KiB with its measurement, in the decision that owns it |
| `PROGRAM.49` | `ARCHOGEN-PROGRAM-0400 (leaf PROGRAM.49)` | `docs/reviews/`' total-bytes ceiling raised to 384 KiB with its measurement, in a decision record of its own |
| `PROGRAM.50` | `ARCHOGEN-PROGRAM-0412 (leaf PROGRAM.50)` | the fault contract's review rounds R3–R15 moved byte for byte from `decision_runtime-contract-gaps.md` to `docs/reviews/`, `docs/decisions/` back under its ceiling |
| `PROGRAM.51` | `ARCHOGEN-PROGRAM-0421 (leaf PROGRAM.51)` | `RUST-FORMAT`: every staged Rust source in canonical format, read from the index, at commit time |
| `PROGRAM` | `ARCHOGEN-PROGRAM-0331 (leaf PROGRAM)` | **`PROGRAM.47` sealed**, its 30 closed leaves into `docs/task-history/PROGRAM/`: `docs/tasks/` had grown 1 653 bytes over its 819 200-byte ceiling with `API.6.3`'s leaf |
| `PROGRAM.52` | `ARCHOGEN-PROGRAM-0425 (leaf PROGRAM.52)` | **step 1 — design reviews made executable**: the method record; `M3.1.1.1` filed; `M3.6.2` begun as the instrument; closed leaves sealed |
| `PROGRAM.52.1` | `ARCHOGEN-PROGRAM-0426 (leaf PROGRAM.52.1)` | `HANDOFF-LEDGER`: a design record's hand-offs in a machine-read ledger, each quoted word for word by its leaf |
| `PROGRAM.52.2` | `ARCHOGEN-PROGRAM-0429 (leaf PROGRAM.52.2)` | **step 1 — the trust record's ledger**: 24 hand-offs, each quoted word for word by its leaf; the receiving leaves rewritten around their quotes |
| `PROGRAM.52.2` | `ARCHOGEN-PROGRAM-0430 (leaf PROGRAM.52.2)` | **step 2 — the substitutability record's ledger**: 22 hand-offs in its new §11, each quoted by its leaf; `PROGRAM.52` closed |
| `PROGRAM.55` | `ARCHOGEN-PROGRAM-0438 (leaf PROGRAM.55)` | **the review-history archive**: `REVIEW-HISTORY`, `--seal <FILE>`, 38 arms; the closed catalog history archived behind a stub; `docs/reviews/` back under its ceiling |
| `PROGRAM.55.1` | `ARCHOGEN-PROGRAM-0442 (leaf PROGRAM.55.1)` | **`REVIEW-HISTORY` hardened after its review**: 17 findings answered; the review's row read from the table; merged rounds, staging, frozen histories, stub shapes; 65 arms; 38 of 38 killed |
| `PROGRAM.56` | `ARCHOGEN-PROGRAM-0445 (leaf PROGRAM.56)` | **the trust design moves to `docs/specs/trust/`**, by the ruling of `2026-10-01`; `docs/specs/`' ceiling raised to 384 KiB by a decision record; `docs/decisions/` back to 349 217 bytes |
| `PROGRAM` | `ARCHOGEN-PROGRAM-0450 (leaf PROGRAM)` | **`PROGRAM.55` and `PROGRAM.56` sealed**, their 3 closed leaves into `docs/task-history/PROGRAM/`: `docs/tasks/` had grown 1 324 bytes over its 819 200-byte ceiling with `M3.1.1`'s step 27 |
| `PROGRAM.57` | `ARCHOGEN-PROGRAM-0457 (leaf PROGRAM.57)` | **the fifth artifact cleanup, ≈9.4 GB released**: `target/debug` deleted whole because its `deps` held 1 454 953 leaked object files, which is filed as `PROGRAM.58`; closed reviews' scratch removed; cited mutation scripts kept |
| `PROGRAM.58` | `ARCHOGEN-PROGRAM-0458 (leaf PROGRAM.58)` | **a macOS build stops leaving its object files behind**: `[profile.dev] split-debuginfo = "off"`, measured +6 098 objects per relink → 0; the way back to backtrace lines in `TOOLBOX.md`; Cargo's default recorded in the ledger |
| `PROGRAM.59` | `ARCHOGEN-PROGRAM-0459 (leaf PROGRAM.59)` | **no live document says the emulator is still quarantined**: `COMMIT.md` step 2, the book, a decision's "Today" and this tree's Blockers corrected six days after `M2.8.3.4` lifted the quarantine |
| `PROGRAM` | `ARCHOGEN-PROGRAM-0460 (leaf PROGRAM)` | **`PROGRAM.57`, `PROGRAM.58` and `PROGRAM.59` sealed**, their 3 closed leaves into `docs/task-history/PROGRAM/`: `docs/tasks/` would have grown 1 331 bytes over its 819 200-byte ceiling with `M3.1.2`'s decomposition |
| `PROGRAM.62` | `ARCHOGEN-PROGRAM-0464 (leaf PROGRAM.62)` | **the frontiers hold the frontier**: `M1`'s and `PROGRAM`'s closure narratives removed, each closure's row checked present in its Commit Log; `docs/tasks/` 819 056 → 786 004 bytes |
| `PROGRAM.61` | `ARCHOGEN-PROGRAM-0465 (leaf PROGRAM.61)` | **the mutation catalog runs whole again**: one entry repointed after `API.4.2` moved its line, and `cargo test` now refuses an entry whose text its file no longer holds once |
| `PROGRAM.63` | `ARCHOGEN-PROGRAM-0472 (leaf PROGRAM.63)` | **a subtree carrying hand-offs can be sealed**: `HANDOFF-LEDGER` reads a sealed leaf's quotes past its stub; `docs/task-history/`' line ceiling raised to the trees' 3 072 by a decision record |
| `PROGRAM.10.5.1` | `ARCHOGEN-PROGRAM-0478 (leaf PROGRAM.10.5.1)` | **the catalog gate reads its pending date where §4 says**: `git var` in the hook's own environment, not the history readers' allowlist; the scratch repositories carry their own identity, which the first CI run found missing |
| `PROGRAM.10.5.2` | `ARCHOGEN-PROGRAM-0479 (leaf PROGRAM.10.5.2)` | **the `.incbin` fixture assembles on ELF**: `.previous`, not `.text`, so a naked function's `.size` is measured in its own section |
| `PROGRAM.10.5.1` | `ARCHOGEN-PROGRAM-0480 (leaf PROGRAM.10.5.1)` | **a reproduction's description corrected**: the leaf said the identity-less runs used the machine's `CARGO_HOME` and `RUSTUP_HOME`; bash had expanded them inside the empty home, where rustup installed the same pin |
| `PROGRAM.10.5` | `ARCHOGEN-PROGRAM-0481 (leaf PROGRAM.10.5)` | **CI green on the runner**: the first run's two test defects fixed, the second run `success` on every job; `PROGRAM.10` closed with it |
| `PROGRAM.64` | `ARCHOGEN-PROGRAM-0482 (leaf PROGRAM.64)` | **the rehearsal's job has a home of its own**, naming no git identity, so a tool that clears its environment and keeps `HOME` sees what the runner gives; `PROGRAM.65` filed for its checkout's place |
| `PROGRAM.65` | `ARCHOGEN-PROGRAM-0488 (leaf PROGRAM.65)` | **a whole CI rehearsal passes again**: its checkout beside the repository, by the director's ruling, no cargo configuration above it; `.archogen-data/` ignored, a store |
| `PROGRAM` | `ARCHOGEN-PROGRAM-0489 (leaf PROGRAM)` | **`PROGRAM.10` and `.61`–`.65` sealed**, 13 closed leaves into `docs/task-history/PROGRAM/`, when `README-ROUTES` refused `docs/tasks/` at 820 957 bytes over its 819 200; `docs/tasks/` 817 470 → 752 861 bytes, measured from `HEAD` and the tree |
| `PROGRAM.66` | `ARCHOGEN-PROGRAM-0490 (leaf PROGRAM.66)` | **a commit holds what was run**: `UNTRACKED-CODE` refuses a commit while a file in a code path is untracked, after `0485` recorded a tree that did not build |
| `PROGRAM.68` | `ARCHOGEN-PROGRAM-0508 (leaf PROGRAM.68)` | **the sixth artifact cleanup, ≈4.0 GB released**: the incremental caches, the closed generated-sources review's loose logs and sweep scratch, uncited probe output and test scratch; cited evidence kept; `PROGRAM.58`'s leak not back |
| `PROGRAM.69` | `ARCHOGEN-PROGRAM-0510 (leaf PROGRAM.69)` | **the sealing record's review reopened** for its amendment, so the amendment's rounds append to its history: a new history would be `docs/reviews/`' seventeenth file, past its ceiling of sixteen, and a closed one is frozen |
| `PROGRAM.69` | `ARCHOGEN-PROGRAM-0511 (leaf PROGRAM.69)` | **a closed subtree below an open one sealed too**, its review open: the outermost closed subtree the unit; leg 5 byte for byte and outermost; leg 6 at any depth and in any tree; one row a file, one leaf a name; the index's layout; `--census`; 46 leaves of `M1`, `M2`, `M3` sealed, 255 871 bytes out of `docs/tasks/`; round 6 answered |
| `PROGRAM.69` | `ARCHOGEN-PROGRAM-0514 (leaf PROGRAM.69)` | **review round 7 answered**: a sealed file holding no leaf refused; every write of a seal in one guard, rolled back whatever stops it; a column-0 line below the top level armed; the texts corrected; `PROGRAM.71` filed; 69 arms |
| `PROGRAM.69` | `ARCHOGEN-PROGRAM-0517 (leaf PROGRAM.69)` | **review round 8 answered**: the rollback's claim narrowed to what it catches, a kill signal a stated limit; no seal refused yet reported sealed; the separator and an empty table checked; a directory for the unreadable-file arm; `PROGRAM.72` filed; 71 arms |
| `PROGRAM.69` | `ARCHOGEN-PROGRAM-0521 (leaf PROGRAM.69)` | **review round 9 answered**: the tree and the index written whole or not at all; the rollback undoing what was written alone, each step on its own, naming what it could not undo; a taken path refused before any write; arms for a full disk, an interrupt, undecodable git output, a first seal's folder and a table's blank line; `PROGRAM.73` filed; 77 arms |
| `PROGRAM.70.1` | `ARCHOGEN-PROGRAM-0522 (leaf PROGRAM.70.1)` | **seven closed review histories archived** behind their stubs; three closed rows put in the archive's form; the runtime variant's history kept live, edited by a pushed commit while its row read closed; `docs/reviews/` 391 513 → 262 852 bytes |
| `PROGRAM.70.1` | `ARCHOGEN-PROGRAM-0523 (leaf PROGRAM.70.1)` | **the three closed rows' histories archived**, `.1` closed: every closed review history archived but the one a pushed commit edited while closed; `docs/reviews/` 262 852 → 78 575 bytes, 16 files |
| `PROGRAM.70.2` | `ARCHOGEN-PROGRAM-0524 (leaf PROGRAM.70.2)` | **an archive stub is no file of its folder's count**, `PROGRAM.70` closed: `README-ROUTES` counts the histories in `docs/reviews/`, 4 of 16, its 12 stubs aside; seven look-alike and stub arms |
| `PROGRAM.69` | `ARCHOGEN-PROGRAM-0525 (leaf PROGRAM.69)` | **review round 10 answered**: each write noted before it is made, so an interrupt just after one is undone; any entry at a sealed file's path refused; a linked tree or index refused; every rollback step armed; the book's claim and round 9's counts corrected; 85 arms |
| `PROGRAM.74` | `ARCHOGEN-PROGRAM-0526 (leaf PROGRAM.74)` | **the book's total raised to 589 824 bytes**, by `decision_book-in-layers.md`'s dated paragraph: 458 750 of 458 752 measured, 63 361 bytes in a week over 38 commits |
| `PROGRAM.60` | `ARCHOGEN-PROGRAM-0529 (leaf PROGRAM.60)` | **the third reader's record cannot fall behind unseen**: a test holds it to every description, needing no vendor build |
| `PROGRAM.67` | `ARCHOGEN-PROGRAM-0530 (leaf PROGRAM.67)` | **a mutation that makes a test hang is reported, not waited on**: each entry's tests under a limit in a process group of their own, killed whole past it |
| `PROGRAM.69` | `ARCHOGEN-PROGRAM-0531 (leaf PROGRAM.69)` | **review round 11 answered**: a stop — SIGINT, SIGTERM, SIGHUP — recorded and answered by a rollback that runs to its end, only what no handler sees leaving writes; a linked history folder refused; arms for a linked index and another writer's file; 90 arms |
| `PROGRAM.54` | `ARCHOGEN-PROGRAM-0533 (leaf PROGRAM.54)` | **the handoff census sampled twice**: a process counts only when a second sample still holds it; `PROGRAM.75` filed to propose it upstream |

## Changelog

- `2026-09-13`: Created task tree; seeded the milestone trees from `ROADMAP.md` revision 2.0.
- `2026-09-30`: `PROGRAM.35` filed and decomposed into `.35.1` and `.35.2`: the README policy the standing instruction names had never been adopted at its revision, and the caps never fitted.
- `2026-09-30`: `PROGRAM.41` reopened by its independent review; `PROGRAM.41.1` and `PROGRAM.42` filed.
- `2026-10-03`: `PROGRAM.51` filed and closed — `RUST-FORMAT`, `COMMIT.md` step 2's format step held at commit time.
- `2026-10-05`: `PROGRAM.52` filed — design reviews made executable (`decision_executable-design-reviews.md`).
- `2026-10-05`: `PROGRAM.52.1` done — `HANDOFF-LEDGER`; `PROGRAM.52.2` filed for the two records' ledgers.
- `2026-10-05`: `PROGRAM.52` closed — both design records hold their hand-offs in ledgers `HANDOFF-LEDGER` checks.
- `2026-10-06`: `PROGRAM.57` filed and closed — the fifth artifact cleanup, the first since `PROGRAM.19`'s seal; `PROGRAM.58` filed for the object files a macOS build leaves in `target/debug/deps`.
- `2026-10-06`: `PROGRAM.58` closed — the dev profile keeps no object files for their debug information, so `target/debug/deps` no longer grows with every build on macOS.
- `2026-10-06`: `PROGRAM.59` filed and closed — four present-tense claims that the emulator step is quarantined, stale since `2026-09-30`, corrected.
- `2026-10-06`: `PROGRAM.60` filed by `M3.1.2.1`: the third reader's record fell four files behind; it runs only by hand.
- `2026-10-06`: `PROGRAM.61` filed by `M3.1.2.2`: one mutation entry broken since `API.4.2`, so the catalog cannot run whole.
- `2026-10-06`: `PROGRAM.62` filed and closed — `M1`'s and `PROGRAM`'s Current Frontier sections cut to the frontier; 33 KB of closure narratives, each already a Commit Log row.
- `2026-10-06`: `PROGRAM.61` closed — the catalog's broken entry repointed; every entry checked by `cargo test`.
- `2026-10-06`: `PROGRAM.63` filed and closed — sealing `M3.1` met two gate gaps, the ledger's stub shadowing and the history's line ceiling; both fixed.
- `2026-10-10`: `PROGRAM.68` filed and closed — the sixth artifact cleanup; nothing unexpected but a record that called a present directory removed.
- `2026-10-10`: `PROGRAM.66` and `PROGRAM.68` sealed by `M3.6.6.2.1`, whose commit took `docs/tasks/` past its ceiling; `PROGRAM.69` filed for the finished leaves under open top-level subtrees.
- `2026-10-10`: `PROGRAM.69`'s tool and seal committed — a closed subtree below an open one sealed too — its review open after rounds 2 to 6 and a mutation sweep; `PROGRAM.70` filed for `docs/reviews/`' file ceiling.
- `2026-10-10`: `PROGRAM.71` filed by `PROGRAM.69`'s review round 7 — the index's order; `PROGRAM.72` by round 8 — tree files in sub-folders. Both pre-existing, owned.
- `2026-10-10`: `PROGRAM.73` filed by `PROGRAM.69`'s review round 9 — the history gate's traceback on an unforeseen exception. Pre-existing, owned.
- `2026-10-10`: `PROGRAM.70.1` done — ten closed review histories archived; `docs/reviews/` 391 513 → 78 575 bytes.
- `2026-10-10`: `PROGRAM.70` done — every closed review history archived and the stubs out of the count; `M3.6.6.4`'s review has room.
- `2026-10-10`: `PROGRAM.74` filed by `PROGRAM.70.2` — the book at its total ceiling, 235 bytes left.
- `2026-10-10`: `PROGRAM.74` done — the book's total raised to 589 824 bytes, measured.
- `2026-10-10`: `PROGRAM.60` done — the third reader's record held to every description by the tests.
- `2026-10-10`: `PROGRAM.67` done — the mutation harness bounds each entry's tests and kills a hang's whole process group.
- `2026-10-10`: `PROGRAM.54` done — the handoff census sampled twice; `PROGRAM.75` filed, the proposal to the scaffold's owner.
