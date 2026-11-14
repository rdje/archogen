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
  Children: `PROGRAM.1` … `PROGRAM.97`, and the sub-leaves each of them names

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
  Status: `done` — sealed in [`PROGRAM/PROGRAM.53.md`](../task-history/PROGRAM/PROGRAM.53.md); commit `ARCHOGEN-PROGRAM-0534`

- ID: `PROGRAM.54`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.54.md`](../task-history/PROGRAM/PROGRAM.54.md); commit `ARCHOGEN-PROGRAM-0533`

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
  Status: `done` — sealed in [`PROGRAM/PROGRAM.60.md`](../task-history/PROGRAM/PROGRAM.60.md); commit `ARCHOGEN-PROGRAM-0529`

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
  Status: `done` — sealed in [`PROGRAM/PROGRAM.67.md`](../task-history/PROGRAM/PROGRAM.67.md); commit `ARCHOGEN-PROGRAM-0530`

- ID: `PROGRAM.68`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.68.md`](../task-history/PROGRAM/PROGRAM.68.md); commit `ARCHOGEN-PROGRAM-0508`

- ID: `PROGRAM.69`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.69.md`](../task-history/PROGRAM/PROGRAM.69.md); commit `ARCHOGEN-PROGRAM-0510`

- ID: `PROGRAM.70`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.70.md`](../task-history/PROGRAM/PROGRAM.70.md); commit not recorded

- ID: `PROGRAM.70.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.70.md`](../task-history/PROGRAM/PROGRAM.70.md); commit `ARCHOGEN-PROGRAM-0523`

- ID: `PROGRAM.70.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.70.md`](../task-history/PROGRAM/PROGRAM.70.md); commit `ARCHOGEN-PROGRAM-0524`

- ID: `PROGRAM.71`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.71.md`](../task-history/PROGRAM/PROGRAM.71.md); commit `ARCHOGEN-PROGRAM-0971`

- ID: `PROGRAM.72`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.72.md`](../task-history/PROGRAM/PROGRAM.72.md); commit `ARCHOGEN-PROGRAM-1040`

- ID: `PROGRAM.73`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.73.md`](../task-history/PROGRAM/PROGRAM.73.md); commit `ARCHOGEN-PROGRAM-1039`

- ID: `PROGRAM.74`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.74.md`](../task-history/PROGRAM/PROGRAM.74.md); commit `ARCHOGEN-PROGRAM-0526`

- ID: `PROGRAM.75`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.75.md`](../task-history/PROGRAM/PROGRAM.75.md); commit `ARCHOGEN-PROGRAM-0537`

- ID: `PROGRAM.76`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.76.md`](../task-history/PROGRAM/PROGRAM.76.md); commit `ARCHOGEN-PROGRAM-0542`

- ID: `PROGRAM.77`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.77.md`](../task-history/PROGRAM/PROGRAM.77.md); commit `ARCHOGEN-PROGRAM-0569`

- ID: `PROGRAM.78`
  Status: `blocked` — filed `2026-10-10` by `PROGRAM.69`'s review round 26, its R3; blocked `2026-10-28` until the
    features are complete, its purpose a parallel mutation sweep (`decision_mutation-testing-after-completion.md`)
  Goal: the sealing self-test's arm "an interrupt mid-proof waits for the seal" holds whatever the machine's load, so a
  mutation sweep run in parallel reads its failure as a kill only when the mutant made it.
  Reproduce / issue: in a three-way parallel sweep of `scripts/check_task_history.sh`, the arm failed for 8 of 50
  mutants, 6 of which pass 121/121 alone (the round's untracked runner, so not durable).
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.79`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.79.md`](../task-history/PROGRAM/PROGRAM.79.md); commit `ARCHOGEN-PROGRAM-0600`

- ID: `PROGRAM.80`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.80.md`](../task-history/PROGRAM/PROGRAM.80.md); commit `ARCHOGEN-PROGRAM-0607`


- ID: `PROGRAM.81`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.81.md`](../task-history/PROGRAM/PROGRAM.81.md); commit `ARCHOGEN-PROGRAM-0610`


- ID: `PROGRAM.82`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.82.md`](../task-history/PROGRAM/PROGRAM.82.md); commit `ARCHOGEN-PROGRAM-0611`

- ID: `PROGRAM.83`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.83.md`](../task-history/PROGRAM/PROGRAM.83.md); commit `ARCHOGEN-PROGRAM-0626`

- ID: `PROGRAM.83.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.83.1.md`](../task-history/PROGRAM/PROGRAM.83.1.md); commit `ARCHOGEN-PROGRAM-0625`

- ID: `PROGRAM.83.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.83.md`](../task-history/PROGRAM/PROGRAM.83.md); commit `ARCHOGEN-PROGRAM-0626`


- ID: `PROGRAM.84`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.84.md`](../task-history/PROGRAM/PROGRAM.84.md); commit `ARCHOGEN-PROGRAM-0687`


- ID: `PROGRAM.85`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.85.md`](../task-history/PROGRAM/PROGRAM.85.md); commit `ARCHOGEN-PROGRAM-0707`

- ID: `PROGRAM.86`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.86.md`](../task-history/PROGRAM/PROGRAM.86.md); commit `ARCHOGEN-PROGRAM-0724`

- ID: `PROGRAM.87`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.87.md`](../task-history/PROGRAM/PROGRAM.87.md); commit `ARCHOGEN-PROGRAM-0726`

- ID: `PROGRAM.88`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.88.md`](../task-history/PROGRAM/PROGRAM.88.md); commit `ARCHOGEN-PROGRAM-0725`


- ID: `PROGRAM.89`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.89.md`](../task-history/PROGRAM/PROGRAM.89.md); commit `ARCHOGEN-PROGRAM-0729`

- ID: `PROGRAM.89.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.89.1.md`](../task-history/PROGRAM/PROGRAM.89.1.md); commit `ARCHOGEN-PROGRAM-0728`

- ID: `PROGRAM.89.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.89.md`](../task-history/PROGRAM/PROGRAM.89.md); commit `ARCHOGEN-PROGRAM-0729`

- ID: `PROGRAM.90`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.90.md`](../task-history/PROGRAM/PROGRAM.90.md); commit `ARCHOGEN-PROGRAM-0735`

- ID: `PROGRAM.91`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.91.md`](../task-history/PROGRAM/PROGRAM.91.md); commit `ARCHOGEN-PROGRAM-0736`

- ID: `PROGRAM.92`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.92.md`](../task-history/PROGRAM/PROGRAM.92.md); commit `ARCHOGEN-PROGRAM-0738`

- ID: `PROGRAM.93`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.93.md`](../task-history/PROGRAM/PROGRAM.93.md); commit `ARCHOGEN-PROGRAM-0778`

- ID: `PROGRAM.94`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.94.md`](../task-history/PROGRAM/PROGRAM.94.md); commit `ARCHOGEN-PROGRAM-0814`

- ID: `PROGRAM.95`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.95.md`](../task-history/PROGRAM/PROGRAM.95.md); commit `ARCHOGEN-PROGRAM-0827`

- ID: `PROGRAM.96`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.96.md`](../task-history/PROGRAM/PROGRAM.96.md); commit `ARCHOGEN-PROGRAM-0829`

- ID: `PROGRAM.97`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.97.md`](../task-history/PROGRAM/PROGRAM.97.md); commit `ARCHOGEN-PROGRAM-0834`

- ID: `PROGRAM.98`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.98.md`](../task-history/PROGRAM/PROGRAM.98.md); commit `ARCHOGEN-PROGRAM-0840`


- ID: `PROGRAM.99`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.99.md`](../task-history/PROGRAM/PROGRAM.99.md); commit `ARCHOGEN-PROGRAM-0845`

- ID: `PROGRAM.100`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.100.md`](../task-history/PROGRAM/PROGRAM.100.md); commit `ARCHOGEN-PROGRAM-0847`

- ID: `PROGRAM.101`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.101.md`](../task-history/PROGRAM/PROGRAM.101.md); commit `ARCHOGEN-PROGRAM-0846`

- ID: `PROGRAM.102`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.102.md`](../task-history/PROGRAM/PROGRAM.102.md); commit `ARCHOGEN-PROGRAM-0866`

- ID: `PROGRAM.103`
  Status: `pending` — P3: `CHANGELOG.md` becomes a curated changelog — decided `2026-10-22` under the director's delegation of `2026-10-22`: low-level technical findings are the engineer's to decide, at state-of-the-art, signoff, production grade
  Decision: Keep a Changelog 1.1.0's convention — an *Unreleased* section and one section per release, entries grouped
    Added, Changed, Deprecated, Removed, Fixed, Security, each a change a user of archogen sees (the CLI, the language, the
    profile, the book's behaviour), never one per commit. The per-commit record already lives twice — `git log`, whose
    subjects name the work unit and leaf, and each tree's Commit Log, which `COMMIT-LOG-ROWS` enforces — so a third copy
    costs every commit and informs no one; the director reads the book. Rejected: freezing it with nothing after, since
    `M7`'s release needs notes a user can read; generating it from `git log`, which writes commits, not changes.
  Goal: the live file restarted in that form, its per-commit history sealed whole under `docs/history/changelog/` as
    `HISTORY-LEDGERS` already seals; `COMMIT.md`, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s narrative home, `STATED-ORDER`'s
    changelog leg and `HISTORY-LEDGERS`' window rule moved to it, each gate's arms kept and re-armed; the book's account of
    the workflow updated. A design first, reviewed by a context that did not write it.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.104`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.104.md`](../task-history/PROGRAM/PROGRAM.104.md); commit `ARCHOGEN-PROGRAM-0865`

- ID: `PROGRAM.105`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.105.md`](../task-history/PROGRAM/PROGRAM.105.md); commit `ARCHOGEN-PROGRAM-0878`

- ID: `PROGRAM.106`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.106.md`](../task-history/PROGRAM/PROGRAM.106.md); commit `ARCHOGEN-PROGRAM-0900`

- ID: `PROGRAM.107`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.107.md`](../task-history/PROGRAM/PROGRAM.107.md); commit `ARCHOGEN-PROGRAM-0901`

- ID: `PROGRAM.108`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.108.md`](../task-history/PROGRAM/PROGRAM.108.md); commit `ARCHOGEN-PROGRAM-0908`

- ID: `PROGRAM.109`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.109.md`](../task-history/PROGRAM/PROGRAM.109.md); commit `ARCHOGEN-PROGRAM-0928`

- ID: `PROGRAM.110`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.110.md`](../task-history/PROGRAM/PROGRAM.110.md); commit `ARCHOGEN-PROGRAM-0952`

- ID: `PROGRAM.111`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.111.md`](../task-history/PROGRAM/PROGRAM.111.md); commit `ARCHOGEN-PROGRAM-0953`

- ID: `PROGRAM.112`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.112.md`](../task-history/PROGRAM/PROGRAM.112.md); commit `ARCHOGEN-PROGRAM-0959`

- ID: `PROGRAM.113`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.113.md`](../task-history/PROGRAM/PROGRAM.113.md); commit `ARCHOGEN-PROGRAM-0978`

- ID: `PROGRAM.113.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.113.md`](../task-history/PROGRAM/PROGRAM.113.md); commit `ARCHOGEN-PROGRAM-0976`

- ID: `PROGRAM.113.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.113.md`](../task-history/PROGRAM/PROGRAM.113.md); commit `ARCHOGEN-PROGRAM-0977`

- ID: `PROGRAM.113.3`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.113.md`](../task-history/PROGRAM/PROGRAM.113.md); commit `ARCHOGEN-PROGRAM-0978`

- ID: `PROGRAM.114`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.114.md`](../task-history/PROGRAM/PROGRAM.114.md); commit `ARCHOGEN-PROGRAM-0980`

- ID: `PROGRAM.115`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.115.md`](../task-history/PROGRAM/PROGRAM.115.md); commit `ARCHOGEN-PROGRAM-1036`

- ID: `PROGRAM.116`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.116.md`](../task-history/PROGRAM/PROGRAM.116.md); commit `ARCHOGEN-PROGRAM-0989`

- ID: `PROGRAM.117`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.117.md`](../task-history/PROGRAM/PROGRAM.117.md); commit not recorded

- ID: `PROGRAM.117.1`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.117.md`](../task-history/PROGRAM/PROGRAM.117.md); commit `ARCHOGEN-PROGRAM-1020`

- ID: `PROGRAM.117.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.117.md`](../task-history/PROGRAM/PROGRAM.117.md); commit `ARCHOGEN-PROGRAM-1022`

- ID: `PROGRAM.117.3`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.117.md`](../task-history/PROGRAM/PROGRAM.117.md); commit `ARCHOGEN-PROGRAM-1037`

- ID: `PROGRAM.118`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.118.md`](../task-history/PROGRAM/PROGRAM.118.md); commit `ARCHOGEN-PROGRAM-1028`

- ID: `PROGRAM.119`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.119.md`](../task-history/PROGRAM/PROGRAM.119.md); commit `ARCHOGEN-PROGRAM-1021`

- ID: `PROGRAM.120`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.120.md`](../task-history/PROGRAM/PROGRAM.120.md); commit `ARCHOGEN-PROGRAM-1068`

- ID: `PROGRAM.121`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.121.md`](../task-history/PROGRAM/PROGRAM.121.md); commit `ARCHOGEN-PROGRAM-1107`

- ID: `PROGRAM.122`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.122.md`](../task-history/PROGRAM/PROGRAM.122.md); commit `ARCHOGEN-PROGRAM-1116`

- ID: `PROGRAM.123`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.123.md`](../task-history/PROGRAM/PROGRAM.123.md); commit `ARCHOGEN-PROGRAM-1118`

- ID: `PROGRAM.124`
  Status: `done` — P1, filed `2026-11-03` by `M4.1.19`'s commit, which the gate refused for `docs/tasks/M3.md` at 3008
    lines, over its 3000, with nothing closed to seal; closed the same day.
  Reproduce / issue: `bash scripts/check_task_history.sh --seal M3` → `nothing to seal`, while `M3.md`'s Verification Log
    held rows whose leaf cell names one sealed leaf with a note, `` `M3.1.1` (step 1) ``: the seal's pattern, `LOGS`, took a
    cell that is the leaf alone, so those rows stayed live. The director ruled `2026-10-20` that the ceiling is met by
    sealing, not by relocating text.
  Built: `LOGS` reading a note in parentheses after the leaf as no second leaf, in both logs; a note holding a code span
    stays live, the seal not reading its prose; two rows in the arm's fixture, one that goes and one that stays; the
    record's amendment and `docs/TASK_TREE_SEALING.md`. Then, on the committed trees: `--seal M4`, 16 subtrees, 21 leaves
    and 46 log rows; `--seal M2`, 8 subtrees, 8 leaves and 47 rows; `--seal M3`, 71 rows; `--seal PROGRAM`, 2 subtrees, 2
    leaves and 55 rows; each `the reconstruction is byte for byte`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `bash scripts/check_task_history.sh --seal M3` at `8a767597`, before the change → `nothing to seal`, `M3.md` at 2999 of its 3000 lines.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git show 8a767597:scripts/check_task_history.sh | grep -c 'no second leaf'` → `0`: `LOGS` matched a closing backtick directly before the cell's end, so a note after it named no leaf.
  - [x] **FIX** — `scripts/check_task_history.sh`'s `LOGS`, its arm's fixture and its expected rows; `docs/decisions/decision_task-log-row-sealing.md`; `docs/TASK_TREE_SEALING.md`; the four seals.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_task_history.sh --self-test` → `174 pass / 0 fail`, the new rows among them; `wc -l docs/tasks/M3.md` → `2928`, 72 lines of room; `docs/tasks/` 785285 → 614992 bytes before this leaf's own text.
  - [x] **NO REGRESSION** — `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree; the gate's `task-history` leg holds every seal across history.
  - [x] **LOCKSTEP** — the record, the sealing guide, this leaf and the Commit Log; `MEMORY.md`.
  Commit: `ARCHOGEN-PROGRAM-1122 (leaf PROGRAM.124)`

- ID: `PROGRAM.125`
  Status: `done` — P3, filed `2026-11-04` by `M4.2.2`, which found the map had no row for its crate, and closed the same day:
    eleven crates had none in `knowledge-map/subsystems.md`, whose head says a crate's row appears when the crate does, and
    nothing asked for one.
  Built: a row saying what each is for: `archogen-api`, `archogen-catalog`, `archogen-digest`, `archogen-install`,
    `archogen-json`, `archogen-outcome`, `archogen-plan-check`, `archogen-plan-check-wasm`, `archogen-transport`,
    `archogen-wasm` and `eadl-resolve`. `BOOK-COVERAGE`, whose population is already the workspace's, also refuses a member
    with no row: it must begin a line, so a mention in another row's prose or a longer crate's name is none.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `git grep -c '^- .crates/eadl-resolve/' 64c39016 -- knowledge-map/subsystems.md` → no match, exit 1: the search, the engine's largest crate, had no row, and neither had ten others.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -c 'subsystems' 64c39016 -- scripts/check_book_coverage.sh` → no match, exit 1, and `KNOWLEDGE-MAP` holds the map to its sources alone: nothing compared those sources with the workspace.
  - [x] **FIX** — `scripts/check_book_coverage.sh` and its arms; the rows in `knowledge-map/subsystems.md` and the map regenerated; the doctrine's lines in `scripts/check_doctrines.project.sh`, `DOCTRINE_ENFORCEMENT.md` and `TOOLBOX.md`; `docs/book/src/annex-repository.md`.
  - [x] **ADDRESSED (verified)** — before the rows, `bash scripts/check_book_coverage.sh` → `BOOK-COVERAGE: 11 of 23 workspace member(s) with no row`, exit 1; after them `book-coverage: OK (23 workspace member(s), …)`, exit 0; `bash scripts/check_book_coverage.sh --self-test` → `book-coverage self-test: 12 pass / 0 fail (12 arms)`.
  - [x] **NO REGRESSION** — `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree; no Rust source changed.
  - [x] **LOCKSTEP** — the book's annex, the doctrine's register, the map, this leaf and the Commit Log.
  Commit: `ARCHOGEN-PROGRAM-1135 (leaf PROGRAM.125)`

- ID: `PROGRAM.126`
  Status: `done` — twenty-second daily artifact cleanup; the record's last run `2026-11-03T21:42:14+00:00`, over 24 h
    old at `2026-11-05T00:03Z`
  Goal: remove only idle, regenerable incremental compiler caches, after the handoff census; keep every dependency
    store, product, evidence directory and Git workspace; a light run under doctrine 22
    (`docs/decisions/decision_small-checks-per-slice.md`) — no identity audit of what stayed.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the cleanup record was over 24 hours old; WHERE, `docs/ARTIFACT_CLEANUP.md` at `e11d0c81`, its entry `2026-11-03T21:42:14+00:00`.
    - [x] FIX — `python3 -I -B target/artifact-cleanup/2026-11-05/cleanup.py --delete`: `target/debug/incremental` removed, no link in it; `target/wasm32-unknown-unknown/debug/incremental` absent, skipped; `target/release` and `target/debug/deps` hold no top-level `.bin` or `.log` file, `find -maxdepth 1` → `0`.
    - [x] ADDRESSED — `total files 30872 bytes 3821526631`, 2106 bin, 0 log, `gone: true` (`target/artifact-cleanup/2026-11-05/removed.json`); `bash scripts/handoff_census.sh`: `handoff: OK` before, twice.
    - [x] NO REGRESSION — a cache alone, regenerated by the next build; no tracked file but the record, the leaf, the resume pointer and the migration note `LANGUAGE-FREEZE` holds spent, `M1.42`'s, flipped to `applied` as the next commit must.
    - [x] LOCKSTEP — the record, `MEMORY.md` and the tree.
  Verification: the census, the removal report.
  Commit: `ARCHOGEN-PROGRAM-1143 (leaf PROGRAM.126)`

- ID: `PROGRAM.127`
  Status: `done` — `2026-11-05`: by the director's instruction, "that's critical". The machine is shared and holds 24 GB, RAM use above 88% is the danger zone and above 90% it may crash, and nothing here watched it. A watcher now stops this repository's processes before those lines and never another project's; everything `make` builds and every agent's shell command goes through it. Its review is `PROGRAM.127.1`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `2026-11-05`, the director: "You are eating all the RAM, the host is going to crash!!"; `sysctl -n vm.swapusage` → `total = 63488.00M  used = 62501.50M`, with a reader and `make focused` running at once; and `git grep -nIiE 'memory_pressure|swapusage|CARGO_BUILD_JOBS' cb7ee681 -- scripts Makefile TOOLBOX.md` → no match, exit 1: nothing read the host.
  - [x] **ROOT CAUSE (WHY + WHERE)** — no mechanism existed: `git grep -n 'verify --tier focused' cb7ee681 -- Makefile` → `cb7ee681:Makefile:24:	cargo xtask verify --tier focused`, unbounded and unwatched, and a background job's notification says when it ended, never what it did to the host. Which process held the memory was never established: another project's `rustc --crate-name pgen` was running then and later measured 8.4 GiB, and my own kill took it down too, so that incident cannot be divided up.
  - [x] **FIX** — `scripts/host_guard.py`, `scripts/selftest_host_guard.py`, `scripts/check_host_guard.sh` and its `HOST-GUARD` row; `Makefile`'s `GUARD`; `.claude/settings.json`'s hook; `scripts/handoff_census.sh`; `docs/decisions/decision_host-guard.md` and its index row; `CLAUDE.md`, compacted to hold it; `DOCTRINE_ENFORCEMENT.md`; `docs/book/src/verification.md`.
  - [x] **ADDRESSED (verified)** — `python3 scripts/selftest_host_guard.py` → `host-guard self-test: 71 passed, 0 failed, 0 not run`, exit 0: a process over its limit stopped and one under it not; a process of the same size outside the root, one in a directory that merely begins with its name and an agent working in it all untouched, what the agent started stopped; what a stopped process started stopped too, in another session and directory; a process still growing stopped within a sample or two; a command line that is no text no blindness; macOS's pressure level at start, stop and critical; a loosened limit refused; every command refused at the stop line, the host still readable, nothing riding in on a read; a heavy command refused at the start line and while a build runs, and the word in a message not taken for one; on the real host, `2026-11-05`: the watcher stopped this repository's job at `RAM use is 88%, at the 88% danger line`, with `0.06 GiB` the largest of ours, and another project's compiler ran on untouched; a heavy job waited for room and a whole `make focused` then passed under it, ours peaking at `0.77` GiB; the reading agreed with the director's, `85.6%` here and `85%` there.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, and `bash scripts/check_doctrines.sh` on the staged tree naming only the focused stamp; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, now through the guard.
  - [x] **LOCKSTEP** — the record, its index row, `CLAUDE.md`, the doctrine table, the book, the map, these leaves and the Commit Log.
  Commit: `ARCHOGEN-PROGRAM-1157 (leaf PROGRAM.127)`

- ID: `PROGRAM.127.1`
  Status: `done` — `2026-11-05`: round 1, a fresh reader told to break the guard by running it: 25 findings, 15 defects, the worst being that it could stop another project's build. Its measurements held and its decisions did not. A process is now ours only by what it shows of itself. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `5afd82de`, `target/m4257/work/exp_ours2.py`: an agent in another project with `--add-dir S`, panic line tightened in scratch root `S` → its `rustc` stopped, the agent alive; the control without `--add-dir`, both alive. `exp_watch.py W1`: `rm -rf target` → the watcher `rc 1`, and a holder over its limit alive 4 s later. `exp_hook.py`: `ps -ax\ncargo build` at the stop line → `rc=0`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'named.search(p.command)' 5afd82de -- scripts/host_guard.py` → the seed by mention, and `git grep -n 'found.add(child)' 5afd82de -- scripts/host_guard.py` → the walk, which never asked `protected`; only the result was filtered, so what an agent or a terminal started became ours by passing through it. `git grep -n 'record(state, kind="unread"' 5afd82de -- scripts/host_guard.py` → bookkeeping inside the handler that kept the watcher alive.
  - [x] **FIX** — `scripts/host_guard.py`, `scripts/selftest_host_guard.py`, `scripts/check_host_guard.sh`, `Makefile`; the record's account of what is ours and its limits, its `## Review`; `docs/reviews/decision_host-guard-reviews.md` and its index.
  - [x] **ADDRESSED (verified)** — `python3 scripts/selftest_host_guard.py` → `host-guard self-test: 166 passed, 0 failed, 0 not run`, exit 0, each finding a case: a build under an agent elsewhere naming the root, under a terminal started here, a shell reading a file here, `ROOT copy/…` and `/other/fixtures/ROOT/…` all left; an agent under a stopped shell and a small `git` left; `cursor-agent` and an agent an interpreter runs left, a `node` job and a dash-prefixed name stopped; the watcher through a removed state, a damaged peak and a closed log, still stopping what it should; a blind watcher said and a heavy command refused; a newline, `$(…)` and a backtick refused as reads, a pipe of reads and the scratch preparation let through; a renamed `vm_stat` label not read as zero; a small writer counted; a stale pid not signalled; `run` one job at a time
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, and `bash scripts/check_doctrines.sh` on the staged tree naming only the focused stamp; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log.
  Commit: `ARCHOGEN-PROGRAM-1158 (leaf PROGRAM.127.1)`

- ID: `PROGRAM.127.2`
  Status: `done` — `2026-11-05`: round 2, a fresh reader over round 1's answers: 23 findings, 12 defects, narrower than round 1's. A terminal's shell with no dash still counted as ours, and "one job at a time" failed four ways. The record now says what the guard is: against accident, no sandbox. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `a6733960`, `target/m4258/work/exp_ours.py O2`: `/bin/zsh -f -il` in scratch root `S` running a build in another directory, panic line tightened there → the shell and the build gone, while `exec -a -zsh` stayed up; `exp_run.py R1`: `ps` failing under `run` → `exit 1`, a `CalledProcessError`, and `sleep 60` still up; `exp_reads.py`: `pkill -x rustc` read as a read, `rc 0`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'process.name.startswith("-") and' a6733960 -- scripts/host_guard.py` → a terminal known only by a dash, and `git grep -n 'f"--root {text}" in p.command' a6733960 -- scripts/host_guard.py` → a root known by a substring: each decided by what a process is called; `git grep -n 'table = processes()' a6733960 -- scripts/host_guard.py` → the blind path needing the very table it had lost.
  - [x] **FIX** — `scripts/host_guard.py`, `scripts/selftest_host_guard.py`, `scripts/check_host_guard.sh`; the record's account of what is ours, what is spared and its limits, its `## Review`; `docs/reviews/decision_host-guard-reviews.md` and its index.
  - [x] **ADDRESSED (verified)** — `python3 scripts/selftest_host_guard.py` → `host-guard self-test: 232 passed, 0 failed, 0 not run`, exit 0, each finding a case: four real shells with no dash left, and a build elsewhere under one; another root whose name begins the same left, its job and its watcher; a helper's children left; `rm` going on when its shell stops; a small `git` passed over; one job at a time with its lock removed, an environment forged and its `run` killed; a `run` that goes blind ending its job; a `run` from another directory counted; unreadable input refused in ten shapes; `pkill -x rustc` refused, a pattern naming this root allowed; a `ps` answering nothing or cut short read as blindness; a future `sampled` not fresh; a replaced log told
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, and `bash scripts/check_doctrines.sh` on the staged tree naming only the focused stamp; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log.
  Commit: `ARCHOGEN-PROGRAM-1159 (leaf PROGRAM.127.2)`

- ID: `PROGRAM.127.3`
  Status: `done` — `2026-11-05`: round 3, a fresh reader on ordinary work: 20 findings, 5 defects. A job an agent's shell execs escaped every limit, and `pkill` passed behind the ordinary `cd ROOT &&`. A helper is now passed over only while small. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `ff41eab2`, `target/m4259/work/e_a.py`: a fake agent's `/bin/bash -c 'x'` execing a 60 MiB holder, 30 MiB limit in scratch root `S` → `ours` empty, the holder alive; `H("cd S && pkill -x rustc")` → `exit 0`, against `H("pkill -x rustc")` → `exit 2`; `H("ps -o rss= -p 1")` at a stop line → `exit 2`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'return at.name not in SHELLS' ff41eab2 -- scripts/host_guard.py` → a helper known only by not being a shell, which a shell that has execed its command is not; `git grep -n 'str(root) not in command' ff41eab2 -- scripts/host_guard.py` → the path looked for in the whole command, not the stage; `git grep -n 'WRITES_OR_SPENDS = ' ff41eab2 -- scripts/host_guard.py` → one list for every tool.
  - [x] **FIX** — `scripts/host_guard.py`, `scripts/selftest_host_guard.py`, `scripts/check_host_guard.sh`; the record's account of what is spared, passed over and refused, and its limits, its `## Review`; `docs/reviews/decision_host-guard-reviews.md` and its index.
  - [x] **ADDRESSED (verified)** — `python3 scripts/selftest_host_guard.py` → `host-guard self-test: 318 passed, 0 failed, 0 not run`, exit 0, each finding a case: a job an agent's shell execed seen as ours and stopped past its limit, the agent left; seven real shells left, flags with values among them; `SCREEN`, an editor and a REPL left; `cd ROOT && pkill -x rustc`, `kill $(pgrep -x rustc)` and `pgrep -x rustc | xargs kill` refused, a search for the word let through; `ps -o`, `top -o`, `git show`, `sed -n`, `find` and the census read as reads; `cargo --version` and `make help` not heavy; growth over a window and from afresh; what a job leaves behind holding no turn; a `run` saying why its own job stopped; two sessions told each for itself; a linked worktree its own root
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, and `bash scripts/check_doctrines.sh` on the staged tree naming only the focused stamp; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log.
  Commit: `ARCHOGEN-PROGRAM-1160 (leaf PROGRAM.127.3)`

- ID: `PROGRAM.127.4`
  Status: `done` — `2026-11-05`: round 4, a fresh reader on ordinary work: 16 findings, 6 defects, no longer converging. Most came from lists kept by hand, so they are gone: a host's line goes by what a process holds, a disk's by what it writes, and the hook refuses only what it cannot read, stopping by a name, and a heavy command with no room. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `36b2eebf`, `target/m4260/work/exp_hook.py`: `kill "$(pgrep -x rustc)"` → hook `exit 0`, against `kill $(pgrep -x rustc)` → `exit 2`; `exp_cat.py`, a disk floor: `cat FIFO > data` still growing, 2 to 10 MiB, its shell stopped; `exp_make.py`: two `make`s with `--wait 32`, both `exit 75`, neither having run.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'SPARED_WHEN_SMALL = ' 36b2eebf -- scripts/host_guard.py` and `git grep -n 'READERS = ' 36b2eebf -- scripts/host_guard.py` → two lists of names, kept apart, deciding what may run and what may be stopped; each round's findings fell on their edges or on the gap between them, and each answer lengthened them.
  - [x] **FIX** — `scripts/host_guard.py`, rewritten without those lists; `scripts/selftest_host_guard.py`; the record's account of what is stopped and refused, and its limits, its `## Review`; `docs/reviews/decision_host-guard-reviews.md` and its index.
  - [x] **ADDRESSED (verified)** — `python3 scripts/selftest_host_guard.py` → `host-guard self-test: 270 passed, 0 failed, 0 not run`, exit 0, each finding a case: `kill "$(pgrep -x rustc)"`, backticks, a loop, a variable and a pipe refused, `kill "$(cat job.pid)"` and a search naming this root let through; at the danger line a small `git`, `rm`, `vim`, `sed`, `find` and their shell left, every build stopped, what holds memory stopped, and the largest when only together they hold it; at a disk floor a small writer and every build stopped, what is not writing left; the watcher stopping what it should with no write possible; two `make`s taking turns; a worktree inside another checkout its own root; a session that lost its place told; a `run` saying why with its log replaced; a light command and `sleep` run at every line, a heavy one refused
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, and `bash scripts/check_doctrines.sh` on the staged tree naming only the focused stamp; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log.
  Commit: `ARCHOGEN-PROGRAM-1161 (leaf PROGRAM.127.4)`

- ID: `PROGRAM.127.5`
  Status: `done` — `2026-11-05`: round 5, a fresh reader over round 4's rewrite: 21 findings, 13 defects, its twenty and the session's refused first command, rising after 15, 12, 5 and 6. Answered by method again: a job is what is judged and stopped; each line judges alone and they add up; the hook reads a command by the shell's grammar, held to the real shell, and tells a stop without refusing anything. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `77e0bf88`, `target/p1275/work/`: `loop.py`, a script around a build at the danger line → `stop events: 11`, `steps 12`, `run alive: True`, and `kill PID` through the hook `rc=2` six times of six; `hk2.py`, a 2 GiB job at 86% with 90 GiB free → `[] not_ours = True`; `hk.py` → `(pkill -x rustc)`, `$(pkill -x rustc)`, `bash -c 'pkill -x rustc'` each `rc 0`, and `grep -n "kill\|pgrep" f`, `python3 scripts/host_guard.py run --wait 1800 -- bash -c 'cd x && cargo test'` each `rc 2`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'builds = \[p for p in mine if p.name in BUILDS\]' 77e0bf88 -- scripts/host_guard.py` → line 497: a line chose processes, so a stopped build left the script that started it; `git grep -n 'if danger or disk or total < worth' 77e0bf88 -- scripts/host_guard.py` → line 533: one verdict with early returns, so a disk's reason silenced the RAM line; `git grep -n 'HEAVY = re.compile\|KILL_BY_NAME = re.compile\|KILL_OF_A_SEARCH = re.compile' 77e0bf88 -- scripts/host_guard.py` → lines 739, 746, 747: patterns over a shell's text, with both edges found again; `git grep -n 'this command was not run' 77e0bf88 -- scripts/host_guard.py` → line 940, the fourth refusal; `git grep -n 'str(event.get("at", "")) >= since' 77e0bf88 -- scripts/host_guard.py` → line 717, a time compared as text, so a hand-written `at` of `hook-proof` was recent for good.
  - [x] **FIX** — `scripts/host_guard.py`: `Job` and `jobs_of`, `disk_line`, `danger_line`, `ceiling_line`, `stop_line` and `judged`, `Motion` and `Memory`, `ShellReader` and `commands_of`, `is_heavy`, `kills_by_name`, `additionalContext` in `hook`, a watcher taking up its new self; `scripts/selftest_host_guard.py`; `.gitignore`; the record, the review history and its index, `docs/book/src/verification.md`; the stale line blanked in the real log at its own length, so that no session's place moved.
  - [x] **ADDRESSED (verified)** — `python3 scripts/selftest_host_guard.py` → `host-guard self-test: 386 passed, 0 failed, 0 not run`, exit 0, no `scripts/__pycache__`; among them the real shell against the reader over 206 forms, and every one of 288 states of the host against one set of jobs. The same rigs on the answer: `loop.py` → `stop events: 1`, `steps ... 1`, `run alive: False`, `kill` through the hook `rc=0` six times of six; `lines.py` A → the job stopped with a disk reason beside it; `mk.py` F → a real waiting `make` `up = True`; `disk.py` 3 → what wrote and sat idle `up = True`; the session's first command no longer refused. In this session, a marked stop in the real log → the next command ran, and `PreToolUse:Bash hook additional context: host-guard: ... stopped nothing` came beside its result, the line then blanked.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard; every case of round 4's kept but those a finding replaced, which say so in the review history.
  - [x] **LOCKSTEP** — the record, the review history and its index, the book, these leaves and the Commit Log.
  Commit: `ARCHOGEN-PROGRAM-1163 (leaf PROGRAM.127.5)`

- ID: `PROGRAM.127.6`
  Status: `done` — `2026-11-05`: round 6, a fresh reader over round 5's answer: 13 findings, 2 defects, the fewest yet, after 15, 12, 5, 6 and 13; the pause lifted by the director's word, "Go in full PNT mode". A floor's way out was stopped for a fall of a single MiB by anyone, and "left alone, whatever it is" was not true of what starts a build. Answered: a volume has fallen only by more than a shared volume costs anyway, and only the one that is short; what a job takes with it said, an editor and a `run` among it; the shell reader given a case's branches, arithmetic, arrays and `[[ ]]`, and held to each `bash` here; each agent told for itself. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `9729be3b`, `target/p1276/work/`: `rig4.py`, `rig7.py`, `rig9.py`, a script of `rm` under a floor → 9 of 9 ended rc -15 within 8.3 s, `written=[]`; `jobs_rig2.py`, a `vim` whose `:!` runs a build at the danger line → `vim alive False`, against the record's "left alone"; `hook_forms.py` → exit 0 for `arch="$(case "$(uname -m)" in arm64) echo aarch64;; *) echo x86;; esac)"; pkill -x rustc`, `bash -euo pipefail -c 'pkill -x rustc'` and `trap 'pkill -x rustc' EXIT`, exit 2 for `tools=(pkill killall); ...` and for `rustc --explain E0308` at a start line of 0.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'motion.fell = any' 9729be3b -- scripts/host_guard.py` → line 691, a fall of `WRITING_BYTES` on either volume; `git grep -n 'if words\[:1\] == \["case"\]' 9729be3b -- scripts/host_guard.py` → line 1018, a case tracked by a count of `)`, so a bare one in a substitution lost the rest; `git grep -n 'def told_script' 9729be3b -- scripts/host_guard.py` → line 915, stopping at the first plain word; `git grep -n 'session = str(given.get' 9729be3b -- scripts/host_guard.py` → line 1467, one place per session id, which a subagent shares; `git grep -n 'print("a heavy job may start"' 9729be3b -- scripts/host_guard.py` → line 1541, the host alone, with `run`'s other reason unread.
  - [x] **FIX** — `scripts/host_guard.py`: `FELL_BYTES`, `FELL_WINDOW_SECONDS` and `Memory.moved`, `disk_line` reading the volume that is short; `shell_told`, `runs_its_input`, `ShellReader.case`, `condition`, `array`, `arithmetic_end`, and here-documents read at whatever level the next newline falls; `only_asks` and `CARGO_ONLY_ASKS`; `agent_id` beside `session_id`; `one_at_a_time` in `check` and `status`. `scripts/selftest_host_guard.py`: the oracle against each `bash`, holding its log until the shell's whole family has ended. The record, the review history and its index, `docs/book/src/verification.md`.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 504 passed, 0 failed, 0 not run`, exit 0, no `scripts/__pycache__`: among them the real shell against the reader over 239 forms on `/bin/bash` 3.2 and 243 on bash 5.3, and a real script of `rm` and `du` under a floor to its end. Measured for the answers: `target/p1276/mine/logical.py` → a slow writer's `ri_logical_writes` 0.3 MiB of 10 and a large `rm -rf` 12 MiB in 0.84 s, against 0.000 for `ri_diskio_byteswritten`, so that figure stays; `target/p1276/mine/race2.py` → `cargo missing 0 of 300` on each `bash`, against 6 of 150 before and 33 of 150 for the first fix tried.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard; every case of round 5's kept but `archogen-plan-check --help`, now light by G6-9, which says so in the review history.
  - [x] **LOCKSTEP** — the record, the review history and its index, the book, these leaves and the Commit Log; `MEMORY.md`'s next action, and the pause gone from it.
  Commit: `ARCHOGEN-PROGRAM-1165 (leaf PROGRAM.127.6)`

- ID: `PROGRAM.127.7`
  Status: `done` — `2026-11-05`: round 7, a fresh reader over round 6's answer: 13 findings, 1 defect and that small, after 15, 12, 5, 6, 13 and 2. Its fuzz of 800 composed forms on both `bash`es missed nothing, and it found nothing else missing from what is stated of a job going whole. `make` that only asked was read as heavy; the reader's rare misses are read now, each held to both `bash`es; a `kill` by a pid passes across lines and by any spelling of it; the root is named exactly; what is told says what it leaves out; the constants beside `Limits` have marks. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`. And a commit whose hook held 0.41 GiB at pressure level 2 was stopped, nothing lost, this session: the record now says a commit goes through `run` on a full host.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `a64d9c7b`, `target/p1277/work/`: `printf '{"tool_input":{"command":"make --version"}}' | python3 -I -B scripts/host_guard.py hook --root <scratch> --limits '{"start_used_percent":0}'` → `refused, this is a heavy command`, rc=2; `diff.py forms3.py` → `[[ a == a ]]>log; cargo build`, both `bash`es ran `cargo`, read nothing; `hf1.py` → `[[ $line =~ ^([[:space:]]*)#(.*)$ ]] && pkill -x rustc`, rc=0; `ps -fp "$PID" && kill "$PID"`, rc=2.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'return not make_is_guarded\|elif newlines and c == "#"\|text in word' a64d9c7b -- scripts/host_guard.py` → `make` returning before `only_asks` was reached; a `#` taken for a comment wherever it stands, so a regular expression lost the rest; and the root named by a substring, at `kills_by_name`. `git grep -n 'pipeline = next(self.pipelines)' a64d9c7b -- scripts/host_guard.py` → a newline ending a pipeline after its `|`.
  - [x] **FIX** — `scripts/host_guard.py`: `make_only_asks`, `shell_only_parses`, `-s` in `shell_told`, `/dev/stdin`, a `#` only where a word could begin, a delimiter closing on a substitution's `)`, `A_DESCRIPTOR_VARIABLE`, `coproc`, `env -S`, `Command.own_from`, `names_this_root`, how many came before what is told. `scripts/selftest_host_guard.py`; `scripts/check_host_guard.sh`, marks for the constants beside `Limits`; the record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 563 passed, 0 failed, 0 not run`, exit 0, no `scripts/__pycache__`: among them the real shell against the reader over 252 forms on `/bin/bash` 3.2 and 258 on bash 5.3, each of this round's forms by name.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1168 (leaf PROGRAM.127.7)`

- ID: `PROGRAM.127.8`
  Status: `done` — `2026-11-06`: round 8, a fresh reader over round 7's answer: 14 findings, 1 defect, after 15, 12, 5, 6, 13, 2 and 1. `kill` of a process group found by name passed; `kill` is now read as it reads its words. Beside it `Monitor` is checked; a checkout's path may hold a space, and a root a space goes on is another; a guard saved broken ends nothing, and a root that is gone stays gone; the self-test waits on what it waits for, and `integration` runs it. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `e3232b6f`, `target/p1278/work/`: `rig_d.py` → `kill -- -$(pgrep -x rustc)` rc=0, `kill -TERM -$(pgrep -f myjob)` rc=0, `PGID=$(pgrep -f myjob); kill -- -$PGID` rc=0, against `kill -9 $(pgrep -x rustc)` rc=2; `rig_b.py`, a checkout whose path holds a space → `protected False`, its watcher counted among ours and signalled; `rig_f.py` → `saved with a syntax error: watcher alive: False rc 1`; `selftest1.log`, at a load near 30 → `556 passed, 7 failed`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'if not word.startswith("-")\]\|GUARD_PROCESS = re.compile\|if entry.get("matcher") == "Bash"' e3232b6f -- scripts/host_guard.py scripts/check_host_guard.sh` → every word beginning `-` dropped from `kill`'s targets; `\S*` before the script's name, which a space ends; a matcher missing `Monitor`. `git grep -n 'time.sleep(0.8)\|time.sleep(1.2)' e3232b6f -- scripts/selftest_host_guard.py` → fixed waits for a job to begin.
  - [x] **FIX** — `scripts/host_guard.py`: `kill_targets`, `pipeline_names_this_root`, `GUARD_PROCESS` by this interpreter's and this file's own paths, `names_root`, `make_targets` and clusters, `CARGO_ALIASES`, `code_reads`, a root gone ends its watcher. `scripts/selftest_host_guard.py`: `until`, `case_a_guard_saved_broken_ends_nothing`, `case_a_root_that_is_gone_stays_gone`. `scripts/check_host_guard.sh`: `--self-test`, the matcher. `.claude/settings.json`: `Bash|Monitor`. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 606 passed, 0 failed, 0 not run`, exit 0, no `scripts/__pycache__`; among them each of `rig_d.py`'s forms refused, and a file saved broken leaving the watcher up and saying so.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1171 (leaf PROGRAM.127.8)`

- ID: `PROGRAM.127.9`
  Status: `done` — `2026-11-06`: round 9, a fresh reader over round 8's answer: 10 findings, 1 defect, after 15, 12, 5, 6, 13, 2, 1 and 1, for the third round running in the last round's own answer. "A guard saved broken ends nothing" held only for a syntax error. A changed guard now proves it runs, in a process of its own, before the watcher takes it up. Beside it a `for` needs no `;` before its body; `-P 1` and `-v` are no way to name what is ours, and `kill -0` stops nothing; what the host does not let the self-test run is incomplete in `integration`. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `298ec47e`, `target/p1279/work/`: `saved.py` → `nameerror`, `badregex`, `importerror`, `main-raises` and `watch-raises` each `ENDED after 2.2 s, exit 1; log: ''`; `brokenrun.py` → a job under `run` ended 14.1 s after the save, rc=75; `hookreal.py` → `for ((i=0;i<1;i++)) do pkill -x rustc; done` and `pkill -P 1 -x rustc` each exit 0; `gate/` → `check_host_guard.sh --self-test` rc=0 over a stub of `33 not run`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'def code_reads\|compile(CODE.read_bytes\|\[ "\$status" -eq 20 \] && exit 0' 298ec47e -- scripts/host_guard.py scripts/check_host_guard.sh` → a compile, which says nothing of a name or an import; and 20 turned into 0. `git grep -n 'if words\[0\] in NOT_A_COMMAND' 298ec47e -- scripts/host_guard.py` → a `for` header's words dropped whole, the body's first command among them.
  - [x] **FIX** — `scripts/host_guard.py`: `code_runs` and `prove`, the root's inode, a `run`'s root and flag always written, a `for` header ended by its body, `inverted`, `by_pid`'s parent of 1, `kill -0`, `make -j "$N"`. `scripts/selftest_host_guard.py`: a guard saved broken by every way of being so, a root remade, a stop of its own told from another session's. `scripts/check_host_guard.sh` and `scripts/run_self_tests.sh`: 20 carried up as incomplete. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 625 passed, 0 failed, 0 not run`, exit 0, no `scripts/__pycache__`; among them each of `saved.py`'s ways of being broken leaving the watcher up and saying so; `bash scripts/run_self_tests.sh --self-test`, exit 0.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1176 (leaf PROGRAM.127.9)`

- ID: `PROGRAM.127.10`
  Status: `done` — `2026-11-06`: round 10, a fresh reader over round 9's answer: 12 findings, 3 defects, after 15, 12, 5, 6, 13, 2, 1, 1 and 1, for the fourth round running in the last round's own answer. So this one is answered by method: what a changed guard must show is no longer a list of names, since the compiled file is walked and the watcher's own loop run. The proof stands beside a watcher that goes on reading, asks only of the file as saved and asks again. A root is named as resolved, `make` asks before any recipe, and a self-test waits for room before saying what it could not run. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `4aef58ad`, `target/p12710/work/`: `rig_broken.py` → `hold_lock`, `write_note`, `code_stamp`, `watch` each `prove rc=0 | watcher ENDED rc=1 | said 'does not run': False`; `rig_relative.py` → `--root sub` `FileNotFoundError: '/sub'`, `--root .` read `/` as its root; `rig_last.py make` → guard emptied `rc 0; cargo ran: NOTHING`; `rig_saves.py broken-then-good` → `took up the good file: False`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'def prove\|judged(reading, mine, limits, None\|arguments=list(argv)\|code = code_stamp()\|timeout=60, check=False' 4aef58ad -- scripts/host_guard.py` → a proof that called the names it was written to call, and blocked the watcher for it; a stamp taken after it; a watcher that left its directory with its first arguments. `git grep -n 'judge(Some(20), false), Judgement::Undeclared' 4aef58ad -- xtask/src/main.rs` → a 20 no row names is a failure.
  - [x] **FIX** — `scripts/host_guard.py`: `names_not_found`, `prove` over the watcher's own loop, `Saved` and `follow_the_file`, a root named as resolved, a watcher's last word, `a_loop_header`, `by_pid` over a cluster and never inverted, a stopped line's parent. `Makefile`: `prove` before any recipe, held by `scripts/check_host_guard.sh`. `scripts/run_self_tests.sh`: 21 counted apart, arms for it and for 20. `scripts/selftest_host_guard.py`: five new cases, every function's name, room waited for. The record, the review history and its index, the book.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 748 passed, 0 failed, 0 not run`, exit 0, no `scripts/__pycache__`; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 13 pass / 0 fail (13 arms)`, exit 0; the reader's rigs after the fix: `rig_broken.py` → each `prove rc=1 | watcher alive | said 'does not run': True`; `rig_last.py make` → `make focused, guard emptied: rc 2; cargo ran: NOTHING`; `rig_relative.py` → `--root sub: after the save, alive True`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, the book's guard section, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1178 (leaf PROGRAM.127.10)`

- ID: `PROGRAM.127.11`
  Status: `done` — `2026-11-06`: round 11, a fresh reader over round 10's answer: 8 findings, 2 defects, after 15, 12, 5, 6, 13, 2, 1, 1, 1 and 3, for the fifth round running in the last round's answer, and four of the eight in `make`'s new `prove`. So that comes out rather than being held up: `make` looks at the guard's last line and runs nothing. A proof goes in as a watcher does, its round's errors are its answer, no line of a round ends a watcher, and what no proof sees is said as it is. `pkill -P` and `-F` are read as `kill`'s target is. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `fe331420`, `target/p12711/work/`: `e1/run.sh … with 10` → 9 of 10 `make exit 143`, log `"name": "make, running bash python3"`; `rigs/save.py saved_field …` → `prove … 'host-guard: this file runs', exit 0`, `watcher … gone, exit 1`; `rigs/save.py warn …` → log `as saved does not run (host-guard: this file runs)`; `hook/forms.py` → `pkill -P "$(pgrep -x make)"` exit 0; `e7/` → a commented `ifneq`, `$(warning` and `ifeq` each `host-guard: OK`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'host_guard.py prove 2>/dev/null\|watch(scope, limits, True)\|stderr=subprocess.STDOUT\|def by_pid' fe331420 -- Makefile scripts/host_guard.py` → a question of the host's state asked at `make`'s every parse, beneath no `run`; a proof past `main` and with no arguments; two channels read as one; a pid `pkill` goes by taken for a number.
  - [x] **FIX** — `Makefile`: the file's last line, held whole by `scripts/check_host_guard.sh`. `scripts/host_guard.py`: `prove` by `main`, its round's `unread` its answer; a round's whole body under its `except`; the proof's two channels apart; `goes_by_pid`, read as `kill`'s target. `scripts/selftest_host_guard.py`: `case_make_looks_for_the_whole_guard`, `case_what_it_says_beside_the_answer_is_no_failure`, a saved field and a line on the way in, an error in a round said and one before it ending, the forms, `wait_for_room`, what goes again and how often. The record, the review history and its index, the book.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 805 passed, 0 failed, 0 not run`, exit 0, no `scripts/__pycache__`; the reader's `rigs/save.py saved_field` and `main_branch` shapes among the saves it refuses, each leaving the watcher up and saying so.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, the book's guard section, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1182 (leaf PROGRAM.127.11)`

- ID: `PROGRAM.127.12`
  Status: `done` — `2026-11-06`: round 12, a fresh reader over round 11's answer: 12 findings, 3 defects, for the sixth round running in the last round's answer, and most of the last four rounds' findings in a watcher replacing itself. So that comes out, as round 4 took out the lists that kept failing: a watcher leaves when its file changes, and the next command or `run` starts one from the file. `prove`, `names_not_found`, `Saved` and `follow_the_file` are gone. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `fdb02218`, `target/p12712/work/`: `rig_a.py a1` → `repaired: taken up within 25 s: False`, `watcher: running, and BLIND`; `rig_h.py 25` → `25 make gate at the danger line: 22 waiting, 3 stopped`; `rig_c2.py` → one case that always fails, `0 failed, 1 not run`, `exit 20`; `rig_f.py` → `reading fails, job gone: 25 s on the watcher is still there, holding the lock`; `mk/` → a trailing blank line `is not whole`, with `host-guard: OK`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'follow_the_file(root, limits, saved, arguments)\|return saw_run\|while not host_clear():' fdb02218 -- scripts/host_guard.py scripts/selftest_host_guard.py` → a change followed inside a round that may raise; a `make` no build only once its `run` shows; room as one clear look.
  - [x] **FIX** — `scripts/host_guard.py`: `prove`, `names_not_found`, `Saved`, `end_proof` and `follow_the_file` gone, a watcher leaving when its file changes, `run` starting one while it waits, `only_waiting`, a blind round holding on to nothing, `goes_by_pid`, `A_PLAIN_TARGET`, `ps`'s flags. `scripts/check_host_guard.sh`, `scripts/run_self_tests.sh`. `scripts/selftest_host_guard.py`: the cases for what went, `case_a_guard_changed_holds_from_the_next_command`, `case_a_job_under_run_outlasts_a_changed_guard`, room, the count. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 645 passed, 0 failed, 0 not run`, exit 0, no `scripts/__pycache__`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1185 (leaf PROGRAM.127.12)`

- ID: `PROGRAM.127.13`
  Status: `done` — `2026-11-06`: round 13, a fresh reader over round 12's answer: 11 findings, 5 defects. Round 12's leaving opened a moment with nothing watching. It is closed: a watcher takes itself up from the file as it stands, unless a save can be seen to have broken it, with no proof. The sentences on a broken guard, two minutes and blindness now say what happens. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `ee72c8e0`, `target/p12713/work/`: `e8_clean.py` → `save, then the next command at once: hook rc 0`, then `T+30: job alive True | watcher: not running`; `e1_saves.py` → guard emptied: `watcher left after 0.93 s`, `hook on 'pkill -x rustc': rc 0`; `e2_broken.py` → `run` ended at T+122; `e4_blind.py` → `status`: `BLIND`, `check`: `clear`, hook on `cargo build` rc 2.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'if code_stamp() not in (None, running)' ee72c8e0 -- scripts/host_guard.py` → a watcher that leaves, with nothing to bring the next but a command that may already have looked.
  - [x] **FIX** — `scripts/host_guard.py`: `file_is_whole`, a watcher taking itself up, `ensure`'s `PATH`, `run` starting one as it waits and with its job, `kill -1`, a plain pid to `ps`. `scripts/run_self_tests.sh`. `scripts/selftest_host_guard.py`: `case_a_guard_changed_holds_from_then`, the forms. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 657 passed, 0 failed, 0 not run`, exit 0, no `scripts/__pycache__`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1189 (leaf PROGRAM.127.13)`

- ID: `PROGRAM.127.14`
  Status: `done` — `2026-11-06`: round 14, a fresh reader over round 13's answer: 10 findings, 2 defects, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3 and 5. A commit now holds what "the commit holds" had claimed, every name the guard reads bound; a second, broken save in the instant of a taking up is a stated limit, closing it being the proof five rounds took out. A table searched by `-U 501` stays caught and `ps "$PID"` is a pid; a gate that waited and died says why; a taking up says so. Found in answering: the host's real watcher had run an image from before `prove` came out, since `08:21`, and `status` now says when one runs an older file. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `1b85fab1`, `target/p12714/work/`: `commit_holds.sh` → `host-guard: OK`, rc 0, over `def hold_lock_(`, and that copy's watcher `NameError`; `twosaves.py 5` → 4 or 5 of 5 watchers ended up to 40 ms; `corpus.py` → `ps -U 501 … | xargs kill` rc 0, `ps "$PID" && kill "$PID"` rc 2; `gates.sh` → a gate that waited and died: `✗ check_waited.sh 0s its RED arms did not all pass:` and no reason; `repair.py` → `unread: … this watcher goes on as it was`, 4 s after a repair was taken up.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'steps: &\[FMT, CLIPPY, TESTS\]\|SELF_TESTS,' 1b85fab1 -- xtask/src/main.rs` → a commit's tier runs no self-test, and `git grep -c 'symtable' 1b85fab1 -- scripts/check_host_guard.sh` finds nothing, so a commit held that the file parsed; `git grep -n 'pg\]\\w\*|--pid' 1b85fab1 -- scripts/host_guard.py` → a number taken as a pid wherever it stood; `git grep -n "grep -v 'waiting for room'" 1b85fab1 -- scripts/run_self_tests.sh` → the second `grep` read what the first had taken out.
  - [x] **FIX** — `scripts/check_host_guard.sh`: every name read is bound. `scripts/host_guard.py`: what a flag of `ps` takes is that flag's, a variable where a pid goes is a pid, `taken-up` recorded, `watch.code` and `status` saying when a watcher runs an older file. `scripts/run_self_tests.sh`: the waiting lines out first, an arm. `scripts/selftest_host_guard.py`: the 15 names, the corpus's forms, `taken-up`. The record, the review history and its index, `docs/book/src/verification.md`.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 702 passed, 0 failed, 0 not run`, exit 0; `bash scripts/run_self_tests.sh --self-test`, `run-self-tests self-test: 14 pass / 0 fail (14 arms)`, exit 0.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, the book, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1194 (leaf PROGRAM.127.14)`

- ID: `PROGRAM.127.15`
  Status: `done` — `2026-11-06`: round 15, a fresh reader over round 14's answer: 11 findings, 4 defects, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5 and 2. The count rose, and the first two were sentences of mine about what a commit holds, so this answers by method: a commit holds the file as staged, compiled, every plain name bound, and a self-test passed on those very bytes by a stamp. What names a watcher outlasts a clean, whatever its round does. `ps` with what selects more than a pid is a search. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `2d866272`, `target/p12715/work/`: `names.py` → `[module-level return] compile rc=1 check rc=0: host-guard: OK`, and 8 of 8 attribute and keyword forms rc 0; `job.bytess` in `danger_line` → job ALIVE at 6 s; `ensure; rm -rf ROOT/target; status` → "it runs an older host_guard.py"; a raising round after a clean → `not running`, `stop` leaves it up; `ps -ef -p 1 | wc -l` → 814 lines, through the real hook rc 0.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'compile(\|git show\|host-guard-self-test' 2d866272 -- scripts/check_host_guard.sh` → none of the three: a check that parsed the working file, and a sentence that said it compiled what lands; `git grep -n 'watch.pid' 2d866272 -- scripts/host_guard.py` → rewritten inside the round's `try` alone.
  - [x] **FIX** — `scripts/check_host_guard.sh`: the file as staged, `compile`, a stamp asked of a commit changing the guard. `scripts/selftest_host_guard.py`: the stamp, `case_a_commit_holds_what_is_staged` on a disposable repository under `target/`, `case_a_clean_leaves_a_watcher_named`, the forms. `scripts/host_guard.py`: what names a watcher outside its round, the image in `watch.code`, what selects more than a pid. `scripts/run_self_tests.sh`: the exit and the last lines, two arms. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 732 passed, 0 failed, 0 not run`, exit 0; `bash scripts/run_self_tests.sh --self-test`, `run-self-tests self-test: 16 pass / 0 fail (16 arms)`, exit 0.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, on the staged tree, its stamp matching; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1197 (leaf PROGRAM.127.15)`

- ID: `PROGRAM.127.16`
  Status: `done` — `2026-11-06`: round 16, a fresh reader over round 15's answer: 11 findings, 4 defects, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5, 2 and 4, each in round 15's stamp or what it reads. A stamp is of a whole run, of the bytes that run began on, in the repository they lie in; all the check reads is read as staged, and what the index lost is gone; it compiles the bytes. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's fixtures on `7871ed4f`, `target/p12716/work/`: `f5` → `selftest case_what_counts_as_heavy`, 153 passed, then `git commit` rc 0 with `job.bytess` in HEAD; `f6` → a save at 14:27:46 stamped, rc 0; `f3` → `{"hooks":{}}` staged, the tree restored, rc 0; a copy's import → sha run `9f37d6f2`, sha it would stamp `a49df506`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'stamp_of(ROOT' 7871ed4f -- scripts/selftest_host_guard.py` → the stamp took the repository's files as they stood at the end, whatever ran; `git grep -n 'root / "Makefile"\|settings.read_text\|root / "CLAUDE.md"' 7871ed4f -- scripts/check_host_guard.sh` → the rest read from the tree.
  - [x] **FIX** — `scripts/selftest_host_guard.py`: `leave_the_stamp`, its case, what is staged of the rest. `scripts/check_host_guard.sh`: `as_staged` of everything it reads, what the index lost, `compile` of the bytes, the same place asked of the filesystem. `scripts/host_guard.py`: a pid's spellings, `x` no selector. `scripts/run_self_tests.sh`: an incomplete gate's last lines, an arm. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 748 passed, 0 failed, 0 not run`, exit 0; `bash scripts/run_self_tests.sh --self-test`, `run-self-tests self-test: 17 pass / 0 fail (17 arms)`, exit 0.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, on the staged tree, its stamp matching; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1199 (leaf PROGRAM.127.16)`

- ID: `PROGRAM.127.17`
  Status: `done` — `2026-11-06`: round 17, a fresh reader over round 16's answer: 8 findings, 1 defect, mild, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5, 2, 4 and 4. A script whose name merely ends in this file's is a job like any, so `check` and `status` say what `run` would beside it. `-d` selects and `-M` takes no value. What a commit holds says where the check itself stands. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rig on `86b587c4`, `target/p12717/work/`: `S/my_host_guard.py` sleeping under `run --root S` → `check` says `clear`, `run --wait 0` refuses, rc 75, hook on `cargo build` rc 0; `ps -dp 1 | wc -l` → 91, hook rc 0; `ps -M "$PID"; kill "$PID"` → hook rc 2; `imp.py` → a copy whose repository has no `scripts/` raises `FileNotFoundError`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'S\*host_guard' 86b587c4 -- scripts/host_guard.py` → any name ending in this file's matched, the self-test's among them.
  - [x] **FIX** — `scripts/host_guard.py`: this file's name by a path's last part, `-d`, `-M`. `scripts/selftest_host_guard.py`: `case_a_job_named_like_the_guard_is_a_job`, the forms, `leave_the_stamp`'s `try`. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 756 passed, 0 failed, 0 not run`, exit 0, its stamp matching.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, on the staged tree; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1202 (leaf PROGRAM.127.17)`

- ID: `PROGRAM.127.18`
  Status: `done` — `2026-11-06`: round 18, a fresh reader over round 17's answer: 7 findings, 2 defects, one mild, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5, 2, 4, 4 and 1. Round 17 took from the self-test what spared it from a line. It is spared again by its own name, and is a job only to what `check` and `status` say. Dashless `ps` is read as dashed. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `ecd22a0b`, `target/p12718/work/`: `g.protected(Process(1,0,"python3 -I -B scripts/selftest_host_guard.py",…))` → `False`; `e1/driver.py … '{"panic_used_percent":0}' run case_what_holds_little_is_left_alone` → `rc=143`, no summary; `psforms2.py` → `ps U 501`, 519 lines, passes.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'GUARD_PROCESS.search' ecd22a0b -- scripts/host_guard.py` → one reading of a name served what spares a process and what tells a job, and round 17 moved it for the second alone.
  - [x] **FIX** — `scripts/host_guard.py`: `SELF_TEST_PROCESS`, dashless `ps`. `scripts/selftest_host_guard.py`: `case_the_self_test_is_spared_and_a_job`, `jobs_of`, the forms. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 764 passed, 0 failed, 0 not run`, exit 0, its stamp matching.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, on the staged tree; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1206 (leaf PROGRAM.127.18)`

- ID: `PROGRAM.127.19`
  Status: `done` — `2026-11-06`: round 19, a fresh reader over round 18's answer: 4 findings, 2 defects, both mild, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5, 2, 4, 4, 1 and 2. The self-test is spared by its own path, a space in it or not; what a floor counts of a process newly ours is said. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `e9818964`, `target/p12719/work/`: `rig_spared.py`, `sp ace` → `ours: 1 process(es)`, stopped, `rc=-15`; `rig_arrives.py` → a process that writes nothing now, stopped 0.1 s after it came here.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'SELF_TEST_PROCESS = ' e9818964 -- scripts/host_guard.py` → a pattern with no space in it, where `GUARD_PROCESS` carries its own path; `git grep -n 'self.written.get(p.pid, 0)' e9818964 -- scripts/host_guard.py` → a first sight counts its whole life.
  - [x] **FIX** — `scripts/host_guard.py`: `SELF_TEST_PROCESS` by its path, a comment. `scripts/selftest_host_guard.py`: the case. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 765 passed, 0 failed, 0 not run`, exit 0, its stamp matching.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, on the staged tree; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1211 (leaf PROGRAM.127.19)`

- ID: `PROGRAM.127.20`
  Status: `done` — `2026-11-06`: round 20, a fresh reader over round 19's answer: 10 findings, 2 defects. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `python3 -I -B target/p12720/work/e31/rig.py target/p12720/work/e31 420` → `make ended rc=-15` at 10, 137 and 238 s; `python3 -I -B target/p12720/work/e35/rig.py …` → `717 samples, 5 with the queued make read as a build`; `bash target/p12720/work/e3/rig.sh …` → another root's `run` stopped, `rc=143`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'process.name in READS_THE_HOST' ee53e991 -- scripts/host_guard.py` → `(sysctl)` is no name of a tool; `git grep -n 'def names_root' ee53e991 -- scripts/host_guard.py` → the whole command line searched.
  - [x] **FIX** — `scripts/host_guard.py`: `unread_name`, `reads_the_host`, `names_root`, `words_read`, `what_a_run_runs`, `GUARD_NAME`, the number's spelling, `lsof` and `top`, `env -`. `scripts/check_host_guard.sh`: what make and the limits can be told. `scripts/selftest_host_guard.py`: five cases. The record's sentences and limits.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_host_guard.sh --self-test` → `host-guard self-test: 808 passed, 0 failed, 0 not run`, the stamp equal to what is staged: `case_what_ps_cannot_read_says_no_job`, `case_a_job_naming_a_root_is_not_that_root_s`, `case_a_kill_is_read_through_what_hides_it`, `case_a_path_holding_a_space_is_one_path`, `case_a_commit_holds_what_make_and_the_limits_can_be_told`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1213 (leaf PROGRAM.127.20)`

- ID: `PROGRAM.127.21`
  Status: `done` — `2026-11-06`: round 21, a fresh reader over round 20's answer: 10 findings, 3 defects, the count up from 2, so answered by method. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `python3 -I -B target/p12721/work/hg.py` → `HOST-GUARD rc=0` for `focused: GUARD := true` and five others, each `make focused` exiting 0 with nothing compiled; `python3 -I -B target/p12721/work/make_stop_rig.py target/p12721/work/q3 400 0.15` → `makes started: 400; stopped by the watcher: 5`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'found.get(name) != recipe' 640b8c3f -- scripts/check_host_guard.sh` → a reading of the Makefile's text, where make's own reading decides; `git grep -n 'def reads_the_host' 640b8c3f -- scripts/host_guard.py` → a list of what a bracket may be.
  - [x] **FIX** — `scripts/check_host_guard.sh`: `make -n` and `make -pn` on the staged Makefile, the file loaded and asked its limits, a hook's type. `scripts/host_guard.py`: `says_no_job`, `what_a_run_runs` as `main` reads, `is_number`, `words_read` past a flag's value, `arch`, `xcrun` and `script`. `scripts/selftest_host_guard.py`: each form. The record's sentences and limits.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_host_guard.sh --self-test` → `host-guard self-test: 843 passed, 0 failed, 0 not run`, the stamp equal to what is staged; `python3 -I -B target/p12721/work/hg.py` → `HOST-GUARD rc=1` for each form that left `make focused` compiling nothing, `target_specific_guard`, `pattern_specific_guard`, `shell_true_after_whole`, `ignore_special`, `makeflags_i`, `two_targets_after`, `brace_eval` and `settings_type_prompt` among them.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1216 (leaf PROGRAM.127.21)`

- ID: `PROGRAM.127.22`
  Status: `done` — `2026-11-07`: round 22, a fresh reader over round 21's answer: 11 findings, 3 defects, 3 after 3, so answered by removing what kept failing. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `target/p12722/work/mk/try.sh a 'focused: SHELL := /usr/bin/true'` → `HOST-GUARD rc=0`, `make focused rc=0`, `ran.txt: ABSENT`; `target/p12722/work/mk/load.py`, a line set in `tightened` → `HOST-GUARD rc=0`, `status` saying 99.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'make_says' 15177496 -- scripts/check_host_guard.sh` → a tool asked of a text it can be told anything by; `git grep -n 'module.Limits()' 15177496 -- scripts/check_host_guard.sh` → the class asked, where `main` takes `tightened`.
  - [x] **FIX** — `scripts/check_host_guard.sh`: `MAKEFILE_SHA256` in place of asking `make`, `GNUmakefile` and `makefile` refused, `tightened({})` asked, any value at a call of `Limits` refused. `scripts/host_guard.py`: `taskpolicy`, `sandbox-exec`, `xcrun`'s asking flags, `PYTHON_FLAGS_WITH_VALUES`. `scripts/selftest_host_guard.py`: each form. The record's sentences and limits.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_host_guard.sh --self-test` → `host-guard self-test: 858 passed, 0 failed, 0 not run`, the stamp equal to what is staged: `case_a_commit_holds_what_make_and_the_limits_can_be_told`, a target's own `SHELL`, a `BASH_ENV`, a `$(shell)` at parse that did not run, a comment, a `GNUmakefile`, a Makefile moved with its digest passing, and what `tightened` gives.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1218 (leaf PROGRAM.127.22)`

- ID: `PROGRAM.127.23`
  Status: `done` — `2026-11-07`: round 23, a fresh reader over round 22's answer: 7 findings, 3 defects, 3 after 3 and 3, each narrow, so answered by method. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `bash target/p12723/work/xcrun_real.sh` → `xcrun -k stubtool arg => RAN stubtool arg`, and the hook passing `xcrun -k kill -9 -1`; `python3 -I -B target/p12723/work/rig_flake.py target/p12723/work/r3 25` → 1 of 25 for each race.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'What these ask of it runs nothing' 893de281 -- scripts/host_guard.py` → round 22's own exception, from what a flag was assumed to do; `git grep -n 'os.listdir(root)' 893de281 -- scripts/check_host_guard.sh` → the tree read at a commit.
  - [x] **FIX** — `scripts/host_guard.py`: the exception gone, `taskpolicy`'s flags, both patterns, `-W` and `-X` Python's alone. `scripts/check_host_guard.sh`: the index read. `scripts/selftest_host_guard.py`: `jobs_of`, `status` waited for, each form. The record's sentences and limits.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_host_guard.sh --self-test` → `host-guard self-test: 870 passed, 0 failed, 0 not run`, the stamp equal to what is staged: `xcrun -k kill -9 -1` and `xcrun -f pkill` refused, `taskpolicy`'s four flags read, a `GNUmakefile` staged and gone from the tree refused, one in the tree alone refusing nothing, a guard after `-W ignore` the guard, `perl -W codex.pl` its script.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1222 (leaf PROGRAM.127.23)`

- ID: `PROGRAM.127.24`
  Status: `done` — `2026-11-07`: round 24, a fresh reader over round 23's answer: 6 findings, 3 defects, each narrow. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `bash target/p12724/work/nested.sh` → `nested run --wait 0: STARTED-BESIDE-THIS-ROOTS-BUILD`; `bash target/p12724/work/makegate.sh` → `run` alone `rc=0`, `make gate` `rc=143`; `python3 -I -B target/p12724/work/zero.py` → the hook `rc=0` on `kill -0 -s KILL -1`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n "this root's lines and turn hold it" 612fd4ac -- docs/decisions/decision_host-guard.md` → a sentence the code never held; `git grep -n 'command.words\[1\] == "-0"' 612fd4ac -- scripts/host_guard.py` → an exception read from one flag.
  - [x] **FIX** — `scripts/host_guard.py`: a later flag undoes `kill -0` and `bash -n`. `scripts/check_host_guard.sh`: local settings. `scripts/selftest_host_guard.py`: each form. The record's sentences and limits.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_host_guard.sh --self-test` → `host-guard self-test: 878 passed, 0 failed, 0 not run`, the stamp equal to what is staged: `kill -0 -s KILL -1` and `bash -n +n -c 'pkill y'` refused, `kill -0 -1` and `bash -n -c 'pkill y'` not; local settings turning every hook off refused, and any others not.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1226 (leaf PROGRAM.127.24)`

- ID: `PROGRAM.127.25`
  Status: `done` — `2026-11-07`: round 25, a fresh reader over round 24's answer: 8 findings, 5 defects, after 3, 3, 3 and 3, so answered by method. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `python3 -I -B target/p12725/work/hook_forms.py "$PWD" target/p12725/work/forms1.txt` → `kill -0 -sKILL -1` and its like passed; `target/p12725/work/forms3.txt` → `pkill -P 123 -P 1 -x rustc` and its like passed.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'A later flag is what it sends' 9dda0da8 -- scripts/host_guard.py` → an exception read by a list of what undoes it, which is never whole; and two sentences of round 24's said of more than they hold.
  - [x] **FIX** — `scripts/host_guard.py`: each exception an exact form. `scripts/check_host_guard.sh`: the local settings. `scripts/run_self_tests.sh`: a gate's input. `scripts/selftest_host_guard.py`: each form, a `make` at each line. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 898 passed, 0 failed, 0 not run`, on these bytes, the stamp holding them; among them each exact form's row, a small `make` left at the stop line and stopped at danger, and local settings refused only where they turn the hooks off.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1231 (leaf PROGRAM.127.25)`

- ID: `PROGRAM.127.26`
  Status: `done` — `2026-11-07`: round 26, a fresh reader over round 25's answer: 7 findings, 2 defects, after 3, 3, 3, 3 and 5, each a false sentence, mild, neither in ordinary work on this host. Nothing failed open in ordinary work, and round 25's exact forms held. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `python3 -I -B target/p12726/work/spared.py` → `bash -lc cd /x/codex && cargo build` spared as `codex`; `target/p12726/work/once.py` → `python3 prog /x/claude` left at a line that stops every job.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'if word in ("-m", "-c")' b8160d0b -- scripts/host_guard.py` → a search that ran on past what runs, with two exact flags to stop it, the shape round 25 answered in the reader's exceptions.
  - [x] **FIX** — `scripts/host_guard.py`: `programs_named`, the hook's reading of `kills_by_name`, `unseen`. `scripts/check_host_guard.sh`: what stands read as what is staged. `scripts/selftest_host_guard.py`: rows for each. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 905 passed, 0 failed, 0 not run`, on these bytes, the stamp holding them; among them each form of `G26-1` a row, a too-deep command through a `run` refused, bytes that are no text in local settings, and a line of 200,000 `[` read past by the hook and `status`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1236 (leaf PROGRAM.127.26)`

- ID: `PROGRAM.127.27`
  Status: `done` — `2026-11-10`: round 27, read `2026-11-07` by a fresh reader over round 26's answer: 9 findings, 2 defects, after 3, 3, 3, 3, 5 and 2, both mild, neither failing open. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`, with what answering found in the first answer before it landed.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs: `python3 -I -B target/p12727/work/wt_rig.py target/p12727/work/fix 3 30000` → `git worktree remove --force` rc=255, `Directory not empty`, 3 of 3; `target/p12727/work/agents.py` → `python3 -I -B rig.py /tmp/kimi_data` read as an agent. Here, the new rows on `c2840c8f`'s scripts: `python3 -I -B target/p12727/land/red/scripts/selftest_host_guard.py case_a_removal_of_a_root_goes_through_its_watcher case_a_clean_leaves_a_watcher_named` → `7 passed, 4 failed, 0 not run`. And what the first whole run found, older than the round: `python3 -I -B target/p12727/land/remade_rig.py "$PWD" target/p12727/land/red/scripts/host_guard.py target/p12727/land/red/scripts/host_guard.py` → `watcher: not running` beside a live watcher, its pid note naming one that is gone.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'state.mkdir(parents=True, exist_ok=True)' 142a22c1 -- scripts/host_guard.py` → what names a watcher making its own directory, written so that `cargo clean` could not silence it, and a removal told from a clean by nothing it can read; `git grep -n 'A word after a flag may be that flag.s value' 142a22c1 -- scripts/host_guard.py` → a flag's value guessed by its shape, where a table says.
  - [x] **FIX** — `scripts/host_guard.py`: `FLAGS_WITH_VALUES`, `programs_named`, `WATCHING`, `Cleared`, `state_dir`, `write_notes`, `names_it`, a watcher's look at its root first in its round, `watcher_asked_for`, `status`, `stop`, what reads a log line, `MAKE_ASKS_IN_A_CLUSTER`. `scripts/check_host_guard.sh`: a too-deep local file. `scripts/selftest_host_guard.py`: `case_a_removal_of_a_root_goes_through_its_watcher`, listed, and rows for each. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 925 passed, 0 failed, 0 not run`, on these bytes, the stamp holding them, after a first whole run of `922 passed, 2 failed, 0 not run` that the last two fixes answer; among them each form of G27-1 a row, a watcher's notes making no directory and a stop making one, its directory staying away over its own rounds and its root then removed, `status` and `stop` right the first time after a clean, another's pid and a missing note written again, a `run` ending with 7 beside a line of the wrong type, too-deep local settings refusing nothing, and `gmake -Onone all` heavy.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1241 (leaf PROGRAM.127.27)`

- ID: `PROGRAM.127.28`
  Status: `done` — `2026-11-10`: round 28, a fresh reader over round 27's answer: 4 findings, 1 defect, after 3, 3, 3, 3, 5, 2 and 2, mild, with two gaps and one of wording, answered by method. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`; its report is `target/p12728/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `135b8427`: `python3 -I -B target/p12728/work/l7_spared_live.py` → `spared   perl -I lib/claude tool.pl` and `STOPPED  perl -I lib codex.pl`; `target/p12728/work/d5_deep_line.py 100000` → `status: rc 1; RecursionError`. Here, the new rows on `e3a64bf8`'s scripts: `python3 -I -B target/p12728/land/red/scripts/selftest_host_guard.py case_agents case_a_damaged_log case_a_taking_up_after_a_clean_is_said_where_a_directory_stands` → `48 passed, 44 failed, 0 not run`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'takes_values = PYTHON_FLAGS_WITH_VALUES if' 135b8427 -- scripts/host_guard.py` → `script` skipping a flag's value for Python alone, beside `programs_named` holding the table: two readings of one question, each knowing a part. `git grep -n 'def said_of' 135b8427 -- scripts/host_guard.py` → a line said as it was parsed, each of three readers guarding against it for itself.
  - [x] **FIX** — `scripts/host_guard.py`: `given` and `TEXT_FLAGS`, `script` by it, `programs_named` gone, `FLAGS_WITH_VALUES`, `SHELL_FLAGS_WITH_VALUES`; `event_of`, `events_of`, `names_a_stop`, `said_of`, and `unseen`, `run` and `status` by them; `TAKEN_UP`, a watcher's start and its taking up. `scripts/selftest_host_guard.py`: rows for each, `case_a_taking_up_after_a_clean_is_said_where_a_directory_stands`, and a watcher held still where its note is written over. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 980 passed, 0 failed, 0 not run`, on these bytes, the stamp holding them; among them each of its reader's forms a row both ways, 28 a flag's value or a program's text and 18 an agent behind a flag, a line nested 100,000 deep costing the hook, `status` and a `run` nothing, and a taking up after a clean making no directory and said once one stands. Its reader's rigs on these bytes: `l7_spared_live.py` → `STOPPED  perl -I lib/claude tool.pl` and `spared   perl -I lib codex.pl`; `d5_deep_line.py 100000` → each `rc 0`, a job's 7 read 7.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1244 (leaf PROGRAM.127.28)`

- ID: `PROGRAM.127.29`
  Status: `done` — `2026-11-10`: round 29, a fresh reader over round 28's answer: 7 findings, 1 defect, after 3, 3, 3, 3, 5, 2, 2 and 1, mild and in no process it read, with three gaps and three of wording. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`; its report is `target/p12729/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rig on `e572a5c6`: `python3 -I -B target/p12729/work/rig_spared.py` → `bash -posix job.sh  the script itself still runs: True`. Here, the new rows on that commit's guard: `python3 -I -B target/p12729/land/red/scripts/selftest_host_guard.py case_agents case_a_terminal_vouches_for_nothing case_killing_by_name case_what_counts_as_heavy case_a_stale_pid_names_nobody case_a_damaged_log case_a_taking_up_after_a_clean_is_said_where_a_directory_stands` → `460 passed, 26 failed, 0 not run`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'long = word.startswith("--")' e572a5c6 -- scripts/host_guard.py` → a shell's flag taken for long only by two dashes, where `bash` takes its long flags with one; and `git grep -n 'at += 2 if word in takes_values else 1' e572a5c6 -- scripts/host_guard.py` → a flag's value read of a whole word alone, where an interpreter reads a cluster letter by letter.
  - [x] **FIX** — `scripts/host_guard.py`: `BASH_LONG_FLAGS` and `shell_told`; `CLUSTERS`, `NODE_TEXT_FLAGS` and `given`, `TEXT_FLAGS` gone and `--title` out of `FLAGS_WITH_VALUES`; `watcher_pid`; what a watcher holds unsaid; `plain`. `scripts/selftest_host_guard.py`: rows for each. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 1032 passed, 0 failed, 0 not run`, on these bytes, the stamp holding them; among them a job, a terminal, an agent, a kill and a build behind a long flag with one dash, a cluster's value both ways, a watcher the kernel names no file of, a broken save after a clean said once a directory stands, and a number no machine's word holds. Its reader's rig on these bytes: `rig_spared.py` → `bash -posix job.sh  the script itself still runs: False`, as `bash --posix job.sh`, and `perl -wI lib/claude job.pl` stopped as `perl -I lib/claude job.pl` is.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1248 (leaf PROGRAM.127.29)`

- ID: `PROGRAM.127.30`
  Status: `done` — `2026-11-10`: round 30, a fresh reader over round 29's answer: 6 findings, 2 defects, after 1, both mild, with three gaps and one of wording. One gap is grave if met and came of round 29's answer: a note holding a number that is no one process had `stop` signal a group. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`; its report is `target/p12730/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rig on `15c25ffd`: `python3 -I -B target/p12730/work/shells.py` → `26 form-and-bash pairs read otherwise than that bash ran them`. Here, the new rows on that commit's guard, which take down what it would signal and send none of it: `python3 -I -B target/p12730/land/red/scripts/selftest_host_guard.py case_agents case_a_terminal_vouches_for_nothing case_killing_by_name case_what_counts_as_heavy case_a_stale_pid_names_nobody case_a_taking_up_after_a_clean_is_said_where_a_directory_stands case_the_reader_sees_what_the_shell_runs` → `515 passed, 49 failed, 0 not run`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'word\[-1\] in "oO"' 15c25ffd -- scripts/host_guard.py` → a cluster's `o` given a word only where it is the cluster's last letter, where `bash` gives each one a word wherever it stands; and `git grep -n 'os.environ\[TAKEN_UP\] = "said"' 15c25ffd -- scripts/host_guard.py` → the image a watcher becomes told only whether its own taking up was said, so what it held unsaid before died with the image it left.
  - [x] **FIX** — `scripts/host_guard.py`: `shell_told` in two passes, `BASH_LONG_FLAGS`, `PARSES_ONLY` and `shell_only_parses` by the shell's name; `handed_on` and what `TAKEN_UP` carries; `Darwin.arguments`, `watches`, `watcher_pid` and no signal to a number below 2; `CLUSTERS` and `FLAGS_WITH_VALUES`. `scripts/selftest_host_guard.py`: rows for each. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `1117 passed, 0 failed, 0 not run`, exit 0, on the final bytes, the seven cases this round's rows are in among them, 1032 before; and the reader's rig again, `python3 -I -B target/p12730/work/shells.py` → `3 form-and-bash pairs read otherwise than that bash ran them`, each a flag that runs no script, `-version` twice and `-wordexp`, read as running one.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1250 (leaf PROGRAM.127.30)`

- ID: `PROGRAM.127.31`
  Status: `done` — `2026-11-10`: round 31, a fresh reader over round 30's answer: 6 findings, 3 defects, after 2, with three gaps. One defect is in ordinary work and older: a terminal whose flag's value holds a space, VS Code's, was counted and stopped. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`; its report is `target/p12731/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rig on `21b5e7d9`: `python3 -I -B target/p12731/work/terminal_with_a_build.py` → `/Applications/Visual Studio Code.app: terminal False; ours [101, 100]; the job stopped is `bash, running cargo`; signalled [100, 101]`. Here, the new rows on that commit's guard: `python3 -I -B target/p12731/land/red/scripts/selftest_host_guard.py case_a_terminal_vouches_for_nothing case_the_reader_sees_what_the_shell_runs case_killing_by_name case_what_counts_as_heavy case_a_stale_pid_names_nobody case_what_is_handed_on_is_a_watcher_s_own case_a_watcher_whose_interpreter_is_gone_goes_on case_a_taking_up_after_a_clean_is_said_where_a_directory_stands case_agents` → `628 passed, 19 failed, 0 not run`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'return shell_told(process.arguments.split()) == (None, None)' 21b5e7d9 -- scripts/host_guard.py` → a terminal judged by `ps`'s one line split on spaces, where an argument may hold one; and `git grep -n 'PARSES_ONLY = ' 21b5e7d9 -- scripts/host_guard.py` → round 30's list, taken from shells that ran nothing under `-n` alone and not under `-n` after a `-c` that takes the next word.
  - [x] **FIX** — `scripts/host_guard.py`: `Darwin.arguments` by the measured padding and one buffer, `Process.argv` and `words`, read by `given`, `spawns` and `is_terminal_shell`; `watches` for a root and `watcher_pid` by arguments alone; `PARSES_ONLY`; `-wordexp` in `shell_told`; `handed_on`, `ensure` and the image that says it took a file up, an image that cannot start among it. `scripts/selftest_host_guard.py`: rows for each, two cases new. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `1164 passed, 0 failed, 0 not run`, exit 0, on the final bytes, the two new cases and this round's rows among them, 1117 before; and the reader's rig again, `python3 -I -B target/p12731/work/terminal_with_a_build.py` → `/Applications/Visual Studio Code.app: terminal True; ours [101]; the job stopped is `cargo`; signalled [101]`, the terminal spared and the build alone stopped. The live watcher took the file up and is known by its arguments, `watcher_pid` naming it.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1254 (leaf PROGRAM.127.31)`

- ID: `PROGRAM.127.32`
  Status: `done` — `2026-11-10`: round 32, a fresh reader over round 31's answer: 3 findings, 2 defects, after 3, with one gap. One defect is met in ordinary work and older: a process holding U+2028 or U+2029 in an argument blinded every guard while it lived. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`; its report is `target/p12732/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the real watcher's own log, its reader's command at `2026-11-10T07:30:18`: `python3 scripts/host_guard.py status` → `unread: invalid literal for int() with base 10: 'rustc",'`. Here, the new rows on `a041fe7e`'s guard, which start no such process: `python3 -I -B target/p12732/land/red/scripts/selftest_host_guard.py case_a_command_line_that_is_no_text case_a_terminal_vouches_for_nothing case_a_stale_pid_names_nobody case_a_root_spelt_relatively_stays_that_root` → `83 passed, 7 failed, 0 not run`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'for line in listed.stdout.splitlines():' a041fe7e -- scripts/host_guard.py` → the table cut as text, whose lines end at more than a line feed, where `ps` ends a row at a line feed alone; and `git grep -n 'return canonical(Path(told\[told.index("--root") + 1\])) == canonical(root)' a041fe7e -- scripts/host_guard.py` → a watcher's own spelling of its root read from where the asker stands.
  - [x] **FIX** — `scripts/host_guard.py`: `processes` cuts the table as bytes at line feeds and passes over what is no row; `Process.name` by the file where the first argument is empty; `watches` taking a root given relatively as naming no other. `scripts/selftest_host_guard.py`: rows for each. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `1171 passed, 0 failed, 0 not run`, exit 0, on the final bytes, this round's rows among them, 1164 before; and its reader's rig again, with a real process holding each separator, the live watcher having taken the file up: `python3 -I -B target/p12732/work/lines_rig.py` → `LS U+2028 … processes(): read, 595 processes` and `PS U+2029 … processes(): read, 594 processes`, where each raised.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1257 (leaf PROGRAM.127.32)`

- ID: `PROGRAM.127.33`
  Status: `done` — `2026-11-10`: round 33, a fresh reader over round 32's answer: 5 findings, 1 defect, after 2, with three gaps and one of wording. The defect fails open and was written to, not met in ordinary work: a command line of 252 bytes held every guard for days. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`; its report is `target/p12733/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — its reader's own run on `fa4a066a`, rows handed to the guard in-process and no such process started (`target/p12733/report.md`): `python3 -I -B target/p12733/work/rows.py` → `22 pairs, a command of 144 bytes: `ours` took 2.76 s`, the time doubling with each pair. Here, the new rows on `22b37422`'s guard, the same file: `python3 -I -B target/p12733/land/red/scripts/selftest_host_guard.py case_the_self_test_is_spared_and_a_job case_a_path_holding_a_space_is_one_path case_a_terminal_vouches_for_nothing case_a_command_line_that_is_no_text case_a_root_spelt_relatively_stays_that_root case_a_stale_pid_names_nobody` → `106 passed, 12 failed, 0 not run`, 22 pairs taking 7 s there.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'GUARD_PROCESS = re.compile' 22b37422 -- scripts/host_guard.py` → line 474: a guard known by a pattern over `ps`'s one line, whose flags read `-W -a` as a flag with its value and as two flags, where the record had already rejected a pattern over a command's text for the hook and three rounds had mended this one (G17-1, G19-1, G23-5); and `git grep -n 'return not os.path.isabs(named) or' 22b37422 -- scripts/host_guard.py` → line 1149: round 32's answer taking a root given relatively for any root's.
  - [x] **FIX** — `scripts/host_guard.py`: `interpreter_reads`, the one reader of an interpreter's own words, and `guard_told`, by which `protected`, `is_run_of`, `only_waiting` and a `run`'s census know a guard, the two patterns gone; `Process.told`, the word a process goes by first; `processes` taking a row of blanks; `watches` holding a watcher to a root it names whole, and `main` writing it so before a watcher does anything else; `canonical` folding a name's Unicode form. `scripts/selftest_host_guard.py`: rows for each. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `1184 passed, 0 failed, 0 not run`, exit 0, on the final bytes, this round's rows among them, 1171 before; and its reader's rigs again on the landed guard, which the live watcher had taken up: `python3 -I -B target/p12733/work/rows.py` → `22 pairs, a command of 144 bytes: `ours` took 0.00 s` and `the rows of 99981 and 99982 are read`; `python3 -I -B target/p12733/work/nameless.py` → `nameless, next word under /System/ … protected: False ours: True` and `stopped, status -15`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree. One arm of round 32's held only while the self-test's own pid was below the pids it made up, and is held as a set.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1259 (leaf PROGRAM.127.33)`

- ID: `PROGRAM.127.34`
  Status: `done` — `2026-11-10`: round 34, a fresh reader over round 33's answer: 4 findings, 2 defects, after 1, with two gaps. Both defects are in round 33's own answers, and neither is met in ordinary work. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`; its report is `target/p12734/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — its reader's own run on `2230fff7`, rows handed to the guard in-process and no such process started (`target/p12734/report.md`): `python3 -I -B target/p12734/work/cost_cluster.py 100000 200000 400000` → one reading 0.062, 0.255 and 1.035 s, the square of a cluster's length. Here, the new rows on `da5e6101`'s guard, the same file: `python3 -I -B target/p12734/land/red/scripts/selftest_host_guard.py case_a_command_line_that_is_no_text case_a_stale_pid_names_nobody case_a_root_renamed_in_case_is_still_watched case_a_kill_is_read_through_what_hides_it` → `113 passed, 6 failed, 0 not run`, a cluster of 400,000 letters taking 5 s there.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'rest = word\[place + 1 :\]' da5e6101 -- scripts/host_guard.py` → line 302: the rest of a word cut at each letter of a cluster, in the reader round 33 gave every row; and `git grep -n 'folded(spelt) == folded(str(resolved))' da5e6101 -- scripts/host_guard.py` → line 142: a root's spelling held by comparing names as this language folds them, where the volume's folding is its own and one directory has more than one path.
  - [x] **FIX** — `scripts/host_guard.py`: `interpreter_reads` cutting the rest of a word once; `canonical` taking the kernel's spelling of the directory it opens, held by what both open; `watch` spelling its root at each round; `what_a_run_runs` reading an interpreter's flags by `interpreter_reads`. `scripts/selftest_host_guard.py`: rows for each, and `case_a_root_renamed_in_case_is_still_watched`. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `1193 passed, 0 failed, 0 not run`, exit 0, on the final bytes, this round's rows among them, 1184 before; and its reader's rig again on the landed guard, which the live watcher had taken up: `python3 -I -B target/p12734/work/cost_cluster.py 100000 200000 400000` → one reading 0.005, 0.011 and 0.022 s.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1263 (leaf PROGRAM.127.34)`

- ID: `PROGRAM.127.35`
  Status: `done` — `2026-11-10`: round 35, a fresh reader over round 34's answer: 6 findings, 1 defect, after 2, with four gaps and one of wording. The defect fails open and was written to, not met in ordinary work: a search outside a guard's `run` reached no `kill` inside it behind a wrapper. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`; its report is `target/p12735/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — its reader's own run on `42829455`, the real hook fed JSON for a scratch root (`target/p12735/report.md`): `python3 -I -B target/p12735/work/a_real_hook.py` → `exit 2: pgrep -x rustc | xargs kill`, and `exit 0: pgrep -x rustc | python3 scripts/host_guard.py run --wait 1800 -- xargs kill`. Here, the new rows on `0504839a`'s guard, the same file: `python3 -I -B target/p12735/land/red/scripts/selftest_host_guard.py case_a_kill_is_read_through_what_hides_it case_a_root_renamed_in_case_is_still_watched case_what_no_root_can_be` → `76 passed, 11 failed, 0 not run`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'read_through.append(Command(told, command.pipeline, command.fed, command.own_from) if told else command)' 0504839a -- scripts/host_guard.py` → the hook putting a `run`'s job in unread, so a wrapper hid the `kill` from a search outside the `run`, while it read the job as a shell would only alone; and `git grep -n 'return buffer.raw\[152:152 + 1024\]' 0504839a -- scripts/host_guard.py` → a working directory taken as whatever path the kernel gives, which for a deep directory is its tail.
  - [x] **FIX** — `scripts/host_guard.py`: `read_through_runs`, a `run`'s job read as a shell reads it in the run's place, and `kills_by_name` reading through it; `is_run_of` by the directory a `run` names; `Darwin.cwd` held to the device and inode the kernel gives; `main` refusing a root of `/`. `scripts/selftest_host_guard.py`: rows for each, `case_what_no_root_can_be`, and the blind `run`'s row asking first whether the host refused it. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `1209 passed, 0 failed, 0 not run`, exit 0, on the final bytes, this round's rows among them, 1193 before; and a copy of its reader's rig, pointed at the landed guard: `python3 -I -B target/p12735/land/a_read_through.py scripts` → no `MISS` line of 13, the six it found among them, and `kill 12345` still passing.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1266 (leaf PROGRAM.127.35)`

- ID: `PROGRAM.127.36`
  Status: `done` — `2026-11-10`: round 36, a fresh reader over round 35's answer: 5 findings, 2 defects, after 1, with three gaps. Both defects fail open in round 35's reading of a `run`, written to and not met in ordinary work. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`; its report is `target/p12736/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — its reader's own run on `83763a68`, the real hook fed JSON for a scratch root (`target/p12736/report.md`): `hookin/b_after_env_run.json`, `python3 scripts/host_guard.py run -- env; kill $(pgrep -x rustc)` → exit 0, and `f_heredoc_run.json`, a here-document fed to a `run` of `bash` holding `pkill -x rustc` → exit 0. Here, the new rows on `58b35022`'s guard, the same file: `python3 -I -B target/p12736/land/red/scripts/selftest_host_guard.py case_a_kill_is_read_through_what_hides_it case_what_no_root_can_be` → `83 passed, 12 failed, 0 not run`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'job = read_through_runs(commands_of(shlex.join(told)), depth + 1)' 58b35022 -- scripts/host_guard.py` → round 35's read-through, which carried each command's `own_from` as it stood while the list it indexed moved; `git grep -n 'words = unwrapped(words)\[0\]' 58b35022 -- scripts/host_guard.py` → `runs_its_input` asking a `run`'s own words; and `git grep -n 'fd = os.open(str(resolved), os.O_RDONLY)' 58b35022 -- scripts/host_guard.py` → an `open` that waits on a pipe.
  - [x] **FIX** — `scripts/host_guard.py`: `read_through_runs` mapping where each command stood to where what replaced it begins; `runs_its_input` asking a `run`'s job; `canonical` opening without waiting; `is_run_of` opening another spelling only where it differs in case or form; `Darwin.cwd_read`, `is_that_directory` and `Process.stands_where_it_says`, asked in `ours` of what its path would make ours alone; `main` refusing a volume's mount. `scripts/selftest_host_guard.py`: rows for each. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `1222 passed, 0 failed, 0 not run`, exit 0, on the final bytes, this round's rows among them, 1209 before.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1270 (leaf PROGRAM.127.36)`

- ID: `PROGRAM.127.37`
  Status: `done` — `2026-11-13`: round 37, a fresh reader over round 36's answer: 7 findings, 4 defects, after 2, with three gaps. Two defects fail open in the reader, one met in an ordinary spelling, `bash -c "kill $(pgrep -x rustc)"`, and older than round 36; two are mild false sentences. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.part-2.md`; its report is `target/p12737/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — its reader's own runs on `d4561e49`, the real hook fed JSON for a scratch root (`target/p12737/report.md`): `bash -c "kill $(pgrep -x rustc)"`, `eval kill $(pgrep -x rustc)` and five forms more → exit 0, `pgrep -x cargo | xargs -I{} sh -c 'kill $(pgrep -P {})'` → exit 0, two `run`s sharing a pipeline's id → exit 0. Here, the new rows on `40bc4e64`'s guard, the same file: `python3 -I -B target/p12737/land/red/scripts/selftest_host_guard.py case_a_kill_is_read_through_what_hides_it case_what_no_root_can_be` → `106 passed, 18 failed, 0 not run`; the reader's 29 forms, `python3 -I -B target/p12737/land/probe.py scripts/host_guard.py` on that guard → `wrong 17 of 29`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'ShellReader(script, self.found, self.pipelines).commands()' 40bc4e64 -- scripts/host_guard.py` → line 1877: a shell's script read again with nothing saying what its placeholders stand for or what fed it, as `eval`'s and `trap`'s beside it; `git grep -n -- '-1 - len(out) - len(others) - 1_000_000' 40bc4e64 -- scripts/host_guard.py` → line 2023: a pipeline's id worked out of where it stood; `git grep -n 'elif operator == "<<<" and runs_its_input(words):' 40bc4e64 -- scripts/host_guard.py` → line 1561: a here-string judged by the words before it; `git grep -n 'fd = os.open(str(root), os.O_RDONLY)$' 40bc4e64 -- scripts/host_guard.py` → line 1044: the lock's open waiting on a pipe.
  - [x] **FIX** — `scripts/host_guard.py`: `ShellReader` taking a placeholder for the substitutions read where its text was a word (`PLACEHOLDERS`, `outer_from`, `reread`), carrying what feeds a shell's text (`fed`), judging a here-string by its whole command and a group's redirection as a shell's input (`feeds_a_group`), reading a command again after its here-document's substitutions; `read_through_runs` drawing further pipelines from one count and taking a placeholder for the run's substitutions; `hold_lock` opening the root as a directory without waiting, `watch` and `watcher_pid` refusing one that is none. `scripts/selftest_host_guard.py`: rows for each. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `1250 passed, 0 failed, 0 not run`, exit 0, on the final bytes, this round's rows among them, 1222 before; `python3 -I -B target/p12737/land/probe.py scripts/host_guard.py` → `wrong 0 of 29`. A first fix took every placeholder for one standing outside and refused a round-36 row, `kill $(cat job.pid)` after a search inside a job's shell text: a command is now told by the placeholders it took as text, never by what its words hold.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1276 (leaf PROGRAM.127.37)`

- ID: `PROGRAM.127.38`
  Status: `done` — `2026-11-14`: round 38, a fresh reader over round 37's answer: 7 findings, 4 defects, after 4, with two gaps and one of wording. Three defects fail open in round 37's own reading, one met in an ordinary spelling, `kill '$(pgrep x)'` in a shell's text; one is a mild false sentence. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.part-2.md`; its report is `target/p12738/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — its reader's own runs on `d939600a`, the real hook fed JSON for a scratch root (`target/p12738/report.md`): `bash -c "kill '$(pgrep -x rustc)'"`, `eval`, a here-string, a backquote and a here-document inside the text → exit 0; a `run` inside a `run` → exit 0. Here, the new rows on `660dfa24`'s guard, the same file: `python3 -I -B target/p12738/land/red/scripts/selftest_host_guard.py case_a_kill_is_read_through_what_hides_it case_what_no_root_can_be` → `126 passed, 13 failed, 0 not run`; the reader's forms, `python3 -I -B target/p12738/land/probe.py scripts/host_guard.py` on that guard → `wrong 9 of 15`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'parts.append(text\[self.at + 1 : end\])' 660dfa24 -- scripts/host_guard.py` → lines 1770 and 1852: text between single quotes taken as it stands with no placeholder counted; `git grep -n 'out.append(Command(each.words, pipeline, each.fed or command.fed, own_from))' 660dfa24 -- scripts/host_guard.py` → line 2095: a job's command rebuilt without what it stood for; `git grep -n 'if root == Path("/") or os.path.ismount(root):' 660dfa24 -- scripts/host_guard.py` → line 2506: a root judged by `ismount` alone, no word for `/`'s group or a root that is no directory.
  - [x] **FIX** — `scripts/host_guard.py`: `ShellReader.word` and `dollar` counting a placeholder taken as it stands, between single quotes and in `$'…'`; a reread's start carried to a backquote's and a here-document's readers and a here-string's; `read_through_runs` carrying what a nested run's job stands for; `main` refusing a volume of `/`'s group and a root that stands and is no directory. `scripts/selftest_host_guard.py`: rows for each. The record's sentences, a stated limit and a cost among them.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `1265 passed, 0 failed, 0 not run`, exit 0, on the final bytes, this round's rows among them, 1250 before; `python3 -I -B target/p12738/land/probe.py scripts/host_guard.py` → `wrong 0 of 15`, and round 37's 29 forms → `wrong 0 of 29`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1280 (leaf PROGRAM.127.38)`

- ID: `PROGRAM.127.39`
  Status: `done` — `2026-11-14`: round 39, a fresh reader over round 38's answer: 3 findings, 2 defects, after 4, with a gap. Both defects fail open in the reader of a kill, one in round 38's own answer, the other older and met in an ordinary spelling, `bash <<E` of `kill '$(pgrep x)'`. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.part-2.md`; its report is `target/p12739/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — its reader's own runs on `01822044`, the real hook fed JSON for a scratch root (`target/p12739/report.md`): ``bash -c "kill \`echo $(pgrep -x rustc)\`"`` and five more forms of text read again, and `bash <<E` of `kill '$(pgrep -x rustc)'` and two more, each → exit 0. Here, the new rows on `a1068afe`'s guard, the same file: `python3 -I -B target/p12739/land/red/scripts/selftest_host_guard.py case_a_kill_is_read_through_what_hides_it case_what_no_root_can_be` → `142 passed, 15 failed, 0 not run`; the reader's forms, `python3 -I -B target/p12739/land/probe.py scripts/host_guard.py` on that guard → `wrong 10 of 16`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'ShellReader(inside, self.found, self.pipelines).expansions(\[\], None)' a1068afe -- scripts/host_guard.py` → line 1751: a reader built for an expansion, its count dropped and no start given, as the backquote's beside it; `git grep -n 'if runs_its_input(fed) or feeds_a_group(fed):' a1068afe -- scripts/host_guard.py` → line 1918: a here-document fed to a shell read as written, quoted or not; `git grep -n 'if root.exists() and not root.is_dir():' a1068afe -- scripts/host_guard.py` → line 2518: a root that stands judged, and none for one that is gone.
  - [x] **FIX** — `scripts/host_guard.py`: `read_expansions` and `backticks` handing back what their readers counted and giving them the reread's start; `dollar`'s `${…}` counting its single quotes; `read_here_docs` and `ShellReader.as_fed` expanding an unquoted here-document before its shell reads it; `main` refusing a gone root for every verb but the hook. `scripts/selftest_host_guard.py`: rows for each. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `1283 passed, 0 failed, 0 not run`, exit 0, on the final bytes, this round's rows among them, 1265 before, round 36's gone-root row now asking the refusal every verb gives; `python3 -I -B target/p12739/land/probe.py scripts/host_guard.py` → `wrong 0 of 16`, and rounds 37's and 38's forms → `wrong 0 of 29`, `wrong 0 of 15`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1283 (leaf PROGRAM.127.39)`

- ID: `PROGRAM.127.40`
  Status: `done` — `2026-11-14`: round 40, a fresh reader over round 39's answer: 4 findings, 2 defects, after 2, with a gap and one of wording. One defect fails open in the reader, written to, a here-document ended where the shell does not end it; the other is a self-test row. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.part-2.md`; its report is `target/p12740/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — its reader's own runs on `c58143f0`, the real hook fed JSON for a scratch root (`target/p12740/report.md`): `cat <<E` ended where bash does not end it with a `pkill` after it, a delimiter joined over a line and one holding a substitution, `env kill $\⏎(pgrep -x rustc)`, each → exit 0, and its self-test run `1282 passed, 1 failed` on the host's own refusal. Here, the new rows on `c1f26cd5`'s guard, the same file: `python3 -I -B target/p12740/land/red/scripts/selftest_host_guard.py case_a_kill_is_read_through_what_hides_it` → `141 passed, 13 failed, 0 not run`; the reader's forms, `python3 -I -B target/p12740/land/probe.py scripts/host_guard.py` on that guard → `wrong 13 of 20`, one an over-refusal of body text.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'self.text, self.at, self.found, self.pipelines = text, 0, found, pipelines' c1f26cd5 -- scripts/host_guard.py` → line 1513: the text read with each backslash before a line feed still in it, where the shell takes it out before any token; `git grep -n '                target, quoted = self.word()' c1f26cd5 -- scripts/host_guard.py` → line 1602: a here-document's delimiter read as a word, expanded; `git grep -n 'check("one heavy job at a time", done.returncode == 2' c1f26cd5 -- scripts/selftest_host_guard.py` → line 2443: a row that took the host's refusal for a failure.
  - [x] **FIX** — `scripts/host_guard.py`: `joined`, a backslash before a line feed taken out as the shell takes it out, an unquoted here-document's continued lines joined before its end is compared; `read_delimiter`, a delimiter taken as written; `heavy_refused` saying a root is gone. `scripts/selftest_host_guard.py`: rows for each, and `case_hook`'s row reading the host's refusal as a case taken away. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `1299 passed, 0 failed, 0 not run`, exit 0, on the final bytes, this round's rows among them, 1283 before; `python3 -I -B target/p12740/land/probe.py scripts/host_guard.py` → `wrong 0 of 20`, and rounds 37's, 38's and 39's forms → `wrong 0 of 29`, `wrong 0 of 15`, `wrong 0 of 16`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1286 (leaf PROGRAM.127.40)`

- ID: `PROGRAM.127.41`
  Status: `pending` — round 41, a fresh reader over round 40's answer. It closes on a round that finds no defect, and a form written to go round the guard, or a reader with no line for a form, is a stated limit, not a defect.

- ID: `PROGRAM.128`
  Status: `done` — `2026-11-05`: by the director's word, "If you ever need new CPU, DSP, MCU, BOARD, ISA or in general digital chip PDF just look here", naming `chipdoc`'s `AGENTS.md`. The record, its index row, the book's ledger and a `TOOLBOX.md` row now start any chip document's need there. Reading it corrected three things of ours: the record and the ledger still said `REQ-007` was `requested`, fulfilled `2026-09-30`; the ledger said the corpus was off this volume; and the record said what is adopted is copied here, where most of the corpus may not be republished and this repository is public, so a fact and its citation are copied and never a document. A documents-only leaf.
  Verification: `python3 -I -B "$ARCHOGEN_CHIPDOC_ROOT/scripts/find_docs.py" --text FE310 --retrieve` → a path, `pages: 20`, a `sha256:` and a `cite:` line; `--peer archogen --count` → `27`; `git -C "$ARCHOGEN_CHIPDOC_ROOT" log -1 --format=%h` → `226cfb0`; `grep -n 'REQ-007' "$ARCHOGEN_CHIPDOC_ROOT/catalog/REQUESTS.md"` → `**fulfilled** 2026-09-30`; `stat -f .` from each checkout → the same file system id, `100001c0000001a`; `awk` over that ledger for the rows naming archogen → `REQ-006` and `REQ-007` alone; `bash scripts/check_source_ledger.sh` and `bash scripts/check_doctrines.sh` on the staged tree.
  Commit: `ARCHOGEN-PROGRAM-1162 (leaf PROGRAM.128)`

- ID: `PROGRAM.129`
  Status: `done` — `2026-11-06`: by the director's word: "Regarding RAM, Just observe the trend, it shouldn't cross 90% and still increasing. don't stress out. If it's cross 90% and keep increasing, then you might say, well, I think we might have a situation here. other than that, just chill." The record's `How to apply` and the book's section say so, and a reader's brief from now on. The guard's lines stand as they are. A documents-only leaf.
  Verification: `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, its lines untouched; `git diff --cached --stat -- scripts` → empty.
  Commit: `ARCHOGEN-PROGRAM-1169 (leaf PROGRAM.129)`

- ID: `PROGRAM.130`
  Status: `done` — twenty-third daily artifact cleanup; the record's last run `2026-11-05T00:05:20+00:00`, over 24 h old at `2026-11-06T01:38Z`
  Goal: remove only idle, regenerable incremental compiler caches, after the handoff census; keep every dependency store, product, evidence directory and Git workspace; a light run under doctrine 22 (`docs/decisions/decision_small-checks-per-slice.md`), no identity audit of what stayed.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the cleanup record was over 24 hours old; WHERE, `docs/ARTIFACT_CLEANUP.md` at `9faf9822`, its entry `2026-11-05T00:05:20+00:00`.
    - [x] FIX — `python3 -I -B target/artifact-cleanup/2026-11-06/cleanup.py --delete`: `target/debug/incremental` removed, no link in it; `target/wasm32-unknown-unknown/debug/incremental` absent, skipped; `target/release` and `target/debug/deps` hold no top-level `.bin` or `.log` file, `find -maxdepth 1` → `0`.
    - [x] ADDRESSED — `total files 19276 bytes 2779347408`, 1654 bin, 0 log, `gone: true` (`target/artifact-cleanup/2026-11-06/removed.json`); `bash scripts/handoff_census.sh`: `handoff: OK` before.
    - [x] NO REGRESSION — a cache alone, regenerated by the next build; no tracked file but the record, the leaf and the resume pointer.
    - [x] LOCKSTEP — the record, `MEMORY.md` and the tree.
  Verification: the census, the removal report.
  Commit: `ARCHOGEN-PROGRAM-1175 (leaf PROGRAM.130)`

- ID: `PROGRAM.131`
  Status: `done` — `2026-11-07`, the live task trees over their ceiling, `README-ROUTES` refusing a commit.
  Goal: bring `docs/tasks/` under its ceiling by the project's own sealing, nothing summarised and nothing lost.
  Acceptance:
    - [x] ROOT CAUSE — WHY: 53 closed leaves in `docs/tasks/M4.md`, the review rounds' bodies among them, had grown it to 213,819 bytes. WHERE: `bash scripts/check_readme_routes.sh` on the guard's round 23 → `docs/tasks/: 820406 bytes in total, over its ceiling of 819200`.
    - [x] FIX — `bash scripts/check_task_history.sh --seal M4`, on the committed tree: 44 closed subtrees, 53 leaves, into `docs/task-history/M4/`, each with its row in the index and a stub in the tree.
    - [x] ADDRESSED — `task-history: OK (537 sealed file(s), each against its row and its sealing commit`, exit 0; `docs/tasks/M4.md` 66,644 bytes, `docs/tasks/` 670,865 in all.
    - [x] NO REGRESSION — `bash scripts/check_handoff_ledger.sh` → `handoff-ledger: OK`, exit 0, each hand-off's leaf still open; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
    - [x] LOCKSTEP — the tree, the index and the Commit Log.
  Verification: the seal's own proof, the gate.
  Commit: `ARCHOGEN-PROGRAM-1221 (leaf PROGRAM.131)`

- ID: `PROGRAM.132`
  Status: `done` — twenty-fourth daily artifact cleanup; the record's last run `2026-11-06T01:39:13+00:00`, over 24 h old at `2026-11-07T03:24Z`
  Goal: remove only idle, regenerable incremental compiler caches; keep every dependency store, product, evidence directory and Git workspace; a light run under doctrine 22 (`docs/decisions/decision_small-checks-per-slice.md`), no identity audit of what stayed.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the cleanup record was over 24 hours old; WHERE, `docs/ARTIFACT_CLEANUP.md` at `36e1f1f5`, its entry `2026-11-06T01:39:13+00:00`.
    - [x] FIX — `python3 -I -B target/artifact-cleanup/2026-11-07/cleanup.py --delete`: `target/debug/incremental` removed, no link in it; `target/wasm32-unknown-unknown/debug/incremental` absent, skipped; `target/release` and `target/debug/deps` hold no top-level `.bin` or `.log` file, `find -maxdepth 1` → `0`.
    - [x] ADDRESSED — `total files 26128 bytes 3321402601`, 1752 bin, 0 log, `gone: true` (`target/artifact-cleanup/2026-11-07/removed.json`). `bash scripts/handoff_census.sh` named a reader's rig, a review running, so idleness was shown of the cache itself: `lsof +D target/debug/incremental` → `0` lines.
    - [x] NO REGRESSION — a cache alone, regenerated by the next build; no tracked file but the record, the leaf and the resume pointer.
    - [x] LOCKSTEP — the record, `MEMORY.md` and the tree.
  Verification: the removal report.
  Commit: `ARCHOGEN-PROGRAM-1224 (leaf PROGRAM.132)`

- ID: `PROGRAM.133`
  Status: `done` — twenty-fifth daily artifact cleanup; the record's last run `2026-11-07T03:24:12+00:00`, over 24 h old at `2026-11-10T00:03Z`
  Goal: remove only idle, regenerable incremental compiler caches, after the handoff census; keep every dependency store, product, evidence directory and Git workspace; a light run under doctrine 22 (`docs/decisions/decision_small-checks-per-slice.md`), no identity audit of what stayed.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the cleanup record was over 24 hours old; WHERE, `docs/ARTIFACT_CLEANUP.md` at `135b8427`, its entry `2026-11-07T03:24:12+00:00`.
    - [x] FIX — `python3 -I -B target/artifact-cleanup/2026-11-10/cleanup.py --delete`: `target/debug/incremental` removed, no link in it; `target/wasm32-unknown-unknown/debug/incremental` absent, skipped; `target/release` and `target/debug/deps` hold no top-level `.bin` or `.log` file, `find -maxdepth 1` → `0`.
    - [x] ADDRESSED — `total files 17310 bytes 2194445572`, 1152 bin, 0 log, `gone: true` (`target/artifact-cleanup/2026-11-10/removed.json`); `bash scripts/handoff_census.sh` → `handoff: OK` before, and `lsof +D target/debug/incremental` → `0` lines.
    - [x] NO REGRESSION — a cache alone, regenerated by the next build; no tracked file but the record, the leaf and the resume pointer.
    - [x] LOCKSTEP — the record, `MEMORY.md` and the tree.
  Verification: the census, the removal report.
  Commit: `ARCHOGEN-PROGRAM-1242 (leaf PROGRAM.133)`

- ID: `PROGRAM.134`
  Status: `done` — `2026-11-10`, the live task trees over their ceiling again, `README-ROUTES` refusing the guard's round 35.
  Goal: bring `docs/tasks/` under its ceiling by the project's own sealing, nothing summarised and nothing lost.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the review rounds' closed leaves in `docs/tasks/M4.md`, format `/2`'s six and the application inputs' seven among them, had grown it to 155,320 bytes since `PROGRAM.131` sealed it. WHERE: `bash scripts/check_doctrines.sh` on the guard's round 35, staged → `README-ROUTES: docs/tasks/: 819894 bytes in total, over its ceiling of 819200`.
    - [x] FIX — `bash scripts/check_task_history.sh --seal M4`, on the committed tree, that leaf set aside: 16 closed subtrees, 28 leaves, into `docs/task-history/M4/`, 27 log rows into `rows-0004.md`, each with its row in the index and a stub in the tree.
    - [x] ADDRESSED — `task-history: OK (553 sealed file(s), each against its row and its sealing commit`, exit 0, "the reconstruction is byte for byte"; `docs/tasks/M4.md` 82,662 bytes, `docs/tasks/` 743,666 in all; `bash scripts/check_readme_routes.sh` → `readme-routes: OK (25 destination(s) governed)`, exit 0.
    - [x] NO REGRESSION — `bash scripts/check_handoff_ledger.sh` → `handoff-ledger: OK`, exit 0, each hand-off's leaf still open; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
    - [x] LOCKSTEP — the tree, the index and the Commit Log.
  Verification: the seal's own proof, the gate.
  Commit: `ARCHOGEN-PROGRAM-1265 (leaf PROGRAM.134)`

- ID: `PROGRAM.135`
  Status: `done` — `2026-11-10`, an open review history past its file's ceiling, `README-ROUTES` refusing the guard's round 36.
  Goal: let an open review history outgrow one file with nothing of it edited and its counts still judged whole, raising no ceiling.
  Acceptance:
    - [x] ROOT CAUSE — WHY: a review history only grows while its review is open, and the archive takes a history only once it closes (`REVIEW-HISTORY`), so an open review of many rounds had nowhere to go: the guard's, at 36 rounds, reached 133,177 bytes. WHERE: `bash scripts/check_doctrines.sh` on the guard's round 36, staged → `README-ROUTES: docs/reviews/: 133177 bytes in its largest file, over its ceiling of 131072`; and `git grep -n 'h, r = read(hist), read(rec)' 58b35022 -- scripts/check_review_counts.sh` → the check reading one file per history.
    - [x] FIX — `scripts/check_review_counts.sh`: a history is read with its parts, `<stem>.part-2.md` on, each after the one before it; three arms. `docs/reviews/INDEX.md` says how a history continues, and `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s row for the folder.
    - [x] ADDRESSED — `bash scripts/check_review_counts.sh --self-test` → `review-counts self-test: 159 pass / 0 fail (159 arms)`, 156 before; its three new arms on the committed check's reading → `157 pass / 2 fail`, the two that need a part read.
    - [x] NO REGRESSION — `bash scripts/check_review_counts.sh` → `review-counts: OK`, exit 0, on this tree; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
    - [x] LOCKSTEP — the check, the index, the containment table, the tree and the Commit Log. No ceiling moves.
  Verification: the check's own self-test, the gate.
  Commit: `ARCHOGEN-PROGRAM-1269 (leaf PROGRAM.135)`

- ID: `PROGRAM.136`
  Status: `done` — twenty-sixth daily artifact cleanup; the record's last run `2026-11-10T01:06:41+00:00`, over 24 h old at `2026-11-13`
  Goal: remove only idle, regenerable incremental compiler caches, after the handoff census; keep every dependency store, product, evidence directory, readers' rigs and Git workspace; a light run under doctrine 22 (`docs/decisions/decision_small-checks-per-slice.md`), no identity audit of what stayed.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the cleanup record was over 24 hours old; WHERE, `docs/ARTIFACT_CLEANUP.md` at `a91de722`, its entry `2026-11-10T01:06:41+00:00`.
    - [x] FIX — `python3 -I -B target/artifact-cleanup/2026-11-13/cleanup.py --delete`: `target/debug/incremental` removed, no link in it; `target/wasm32-unknown-unknown/debug/incremental` absent, skipped; `target/release` and `target/debug/deps` hold no top-level `.bin` or `.log` file, `find -maxdepth 1` → `0`.
    - [x] ADDRESSED — `total files 24497 bytes 3218773639`, 1632 bin, 0 log, `gone: true` (`target/artifact-cleanup/2026-11-13/removed.json`); `bash scripts/handoff_census.sh` → `handoff: OK` before, and `lsof +D target/debug/incremental` → `0` lines. Two `xtask` processes then running were another project's, by their working directory, and were left alone.
    - [x] NO REGRESSION — a cache alone, regenerated by the next build; no tracked file but the record, the leaf and the resume pointer.
    - [x] LOCKSTEP — the record, `MEMORY.md` and the tree.
  Verification: the census, the removal report.
  Commit: `ARCHOGEN-PROGRAM-1272 (leaf PROGRAM.136)`

- ID: `PROGRAM.137`
  Status: `done` — `2026-11-13`; filed at session start: fsmgen's `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` changed its neutral body at `58d5edef7` (`2026-11-04`, "remove all book content size limits"), after the `0fa794310` adopted here; the adoption note admits a later revision only by an owned review.
  Goal: review that revision against this repository's own rules and adopt it verbatim, the note's and the ledger's provenance moved with it, or record why not.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the source moved and the copy did not; WHERE, `git -C <fsmgen> log 0fa794310..HEAD -- LIVE_DOCUMENT_SIZE_CONTAINMENT.md` → `58d5edef7` and `c83948125`, the second reviewed by `PROGRAM.57` as fsmgen's local part alone; the body here hashed `af130de4…7aa2`, the source's `f5fbff05…1e4b`. The other three sources are unchanged: fsmgen's README policy body `77a1e934…` and adoption guide `8f77fa39…` as adopted, and pgen's claim-verification policy 285 lines, 18 166 bytes, last changed `178251cce`, before its adoption.
    - [x] FIX — `python3 -I -B target/p137/adopt.py .`: the body replaced by the source's lines 191–549 at `58d5edef7`, read by `git show` and nothing written there; the adoption note and the ledger's `fsmgen` entry name the revision, the span and the digest. The revision adds fsmgen's user-facing book exception, which agrees with this repository's own ruling of `2026-10-18` that the book's growth is unbounded (the inventory's `docs/book/src/` row, `README_POLICY.md`'s `docs/book/` row): no limit here moves.
    - [x] ADDRESSED — `diff` of the body against `git show 58d5edef7:LIVE_DOCUMENT_SIZE_CONTAINMENT.md` lines 191–549 → identical; its sha256 `f5fbff0506537e23ca602e0006588ff1e76bfe4b4c056628d527ef8752261e4b`, 359 lines, 21 415 bytes.
    - [x] NO REGRESSION — `bash scripts/check_doctrines.sh` on the staged tree → exit 0. No code changed.
    - [x] LOCKSTEP — the doctrine's copy and note, the ledger, the tree and the Commit Log.
  Verification: the diff against the source, the gate.
  Commit: `ARCHOGEN-PROGRAM-1273 (leaf PROGRAM.137)`

- ID: `PROGRAM.138`
  Status: `done` — the twenty-seventh artifact cleanup, asked by the director on `2026-11-14`: "ARCHOGEN weights 183.9 GB with \"target 176.5 GB mixed · 3469 logs\", please clean artefacts that can be safely removed".
  Goal: remove from `target/` what is regenerable and idle, every nested cargo target directory in a leaf's or a reader's folder and every directory a test run left in the scratch `TMPDIR`, keeping the main build, every store, the guard's state and every report, brief, rig and log the trees cite as evidence.
  Acceptance:
    - [x] ROOT CAUSE — WHY: each reader and leaf since `M3.5` built in a cargo target directory of its own and never removed it, and every test run leaves its fixtures in the scratch `TMPDIR`, `target/tmp`, which only the incremental cache's cleanups passed over; WHERE, `du -sk target` → 180.0 GB, of which `target/tmp` 64.1 GB, 31,268 entries, and nested cargo target directories 63.0 GB by `CACHEDIR.TAG` and 36.1 GB more by `.rustc_info.json` alone.
    - [x] FIX — `python3 -I -B target/artifact-cleanup/2026-11-14/cleanup.py --delete`, then `cleanup2.py --delete`: every nested cargo target directory in a leaf's or a reader's folder, every directory a test run left in `target/tmp` and in `target/trust-tests`, each checked open by no process of this user (`lsof`), its links unlinked and never followed. The main build, the per-target directories, `miri`, the stores, the guard's state and every report, brief, rig and log beside those directories stay.
    - [x] ADDRESSED — the first pass `31076` directories, `800670` files, `136452411392` bytes, the second `3397`, `454269`, `40638951424`, each `gone: true`, none refused (`target/artifact-cleanup/2026-11-14/removed.json`, `removed2.json`); `du -sk target` → 16.5 GB, 180.0 before; the volume's free space 2408.8 GB, 2248.3 before; `bash scripts/handoff_census.sh` → `handoff: OK` before.
    - [x] NO REGRESSION — build output and test scratch alone, each made again by the next build or test; no tracked file but the record, the leaf and the resume pointer. 5,366 logs stay, readers' and leaves' evidence and run logs.
    - [x] LOCKSTEP — the record, `MEMORY.md` and the tree.
  Verification: the census, the two removal reports, the sizes before and after.
  Commit: `ARCHOGEN-PROGRAM-1279 (leaf PROGRAM.138)`

- ID: `PROGRAM.139`
  Status: `done` — `2026-11-14`, the live task trees over their ceiling again, `README-ROUTES` refusing format `/2`'s round 12.
  Goal: bring `docs/tasks/` under its ceiling by the project's own sealing, nothing summarised and nothing lost.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the review rounds' closed leaves in `docs/tasks/M4.md`, format `/2`'s rounds 7 to 11 and the application inputs' 8 to 12, had grown it since `PROGRAM.134` sealed it. WHERE: `bash scripts/check_doctrines.sh` on format `/2`'s round 12, staged → `README-ROUTES: docs/tasks/: 820961 bytes in total, over its ceiling of 819200`.
    - [x] FIX — `bash scripts/check_task_history.sh --seal M4`, on the committed tree, that round's work set aside: 10 closed subtrees, 10 leaves, into `docs/task-history/M4/`, 10 log rows into `rows-0005.md`, each with its row in the index and a stub in the tree.
    - [x] ADDRESSED — `task-history: OK (563 sealed file(s), each against its row and its sealing commit`, "the reconstruction is byte for byte"; `docs/tasks/` 768,673 bytes in all.
    - [x] NO REGRESSION — `bash scripts/check_doctrines.sh` on the staged tree → exit 0. No code changed.
    - [x] LOCKSTEP — the tree, the index and the Commit Log.
  Verification: the seal's own proof, the gate.
  Commit: `ARCHOGEN-PROGRAM-1287 (leaf PROGRAM.139)`

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
| 1 | `PROGRAM.34` | `pending` | **low, awaiting the director** — nine repositories nested in `vendor/linkedspec` are off their recorded commits since the `2026-09-27` adoption, and `REPOSITORY-BOUNDARY` sees only the first level; the restore discards third-party working trees, so it waits for a yes |

The PROGRAM.34 row waits on the director's yes. The other open leaf, `PROGRAM.78`, is blocked until the
features are complete, by the ruling against mutation testing. Every closed
leaf's outcome is its row in the Commit Log below, and a sealed leaf's full record is under `docs/task-history/PROGRAM/`.

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
- `2026-10-23` (`PROGRAM.107`, the director's go): the book is the contract and rustdoc a view of the code. The
  crates a consumer compiles against, listed once in `docs/book/api.crates`, are documented by rustdoc with the book,
  under `api/`, never committed; each denies missing docs, and a record's format is specified in the book alone.

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
| `2026-09-30` | `PROGRAM.9.3` / `PROGRAM.9` | `cargo xtask mutate` twice (the second naming each kill's tests); harness arms H-1–H-4 through temporary entries; `cargo test -p xtask`; the `extended` tier end to end | 8 of 8 as expected, each kill by the test written for its defect, the `lcm`→`max` blind spot surviving the harmonic corpus; every arm refused as designed, `crates/` clean after each; 14 / 0; **`extended` passed for the first time** |
| `2026-09-30` | `PROGRAM.6.3` / `PROGRAM.6` | both goldens blessed and checked; the lexer's own test; F-1–F-4 with restores; the two new catalog entries and the whole catalog; `cargo test --all` | 20 provenance key paths, re-derived from the renderer; the cost contract frozen; every falsification refused, blessing never overwrites; 11 of 11 mutations as expected; 620 / 0 |
| `2026-09-30` | `PROGRAM.17.3` / `PROGRAM.17` | growth since `2026-09-27` from `git show 4d6d002:<file>`; closed-leaf share of `M1.md` and `PROGRAM.md` by an `awk` over leaf blocks | 77% of `M1.md` and 75% of `PROGRAM.md` are closed leaves; §8 written, `PROGRAM.31`/`.32` filed blocked; `PROGRAM.17` closed |
| `2026-09-30` | `PROGRAM.20.3` / `PROGRAM.20` | a census of figure-shaped text in the live surfaces; the gate's 14 arms; eight mutations; the `S0.8` shape staged on the real tree | 96 phrases over 28 files as backlog; the added figure refused; the register's own first row refused as false; `PROGRAM.20` closed |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `PROGRAM.18` → `PROGRAM.18.1` | `ARCHOGEN-PROGRAM-0120 (leaf PROGRAM.18.1)` | **`FROZEN-EVALUATION` is armed**: nine arms in `--self-test`, each seeding one breach in a scratch repository with synthetic case names and naming what it refuses. `PROGRAM.18` decomposed by ownership on a re-run census — the six scaffold-owned gates are armed from outside in `.18.2`, because an arm written into them is erased by the scaffold and cannot be sent upstream; `PROGRAM.29` filed |
| `PROGRAM.18.2` → `PROGRAM.18` | `ARCHOGEN-PROGRAM-0121 (leaf PROGRAM.18.2)` | **every registered control now has repeatable RED arms.** The six scaffold-owned universal gates, both drivers and the handoff tool are armed **from outside** by `scripts/selftest_spine.sh` — each run unmodified in a scratch repository with one seeded breach, 34 arms, nine mutations each failing its own arm — so the arms survive a scaffold sync that would erase them in place. `PROGRAM.18` closed; tier registration is `PROGRAM.28`'s |
| `PROGRAM.9` → `PROGRAM.9.1` | `ARCHOGEN-PROGRAM-0127 (leaf PROGRAM.9.1)` | **the `miri` step runs, and proves it can fail first.** Found on `nightly` (it had been reported unavailable while installed); a seeded dangling-pointer read refused before every run; every test target timed under Miri, five corpus walks left out on measured cost; sysroot under `target/`. `PROGRAM.9` decomposed into `.9.1`–`.9.3` |
| `PROGRAM.9.3` → `PROGRAM.9` | `ARCHOGEN-PROGRAM-0132 (leaf PROGRAM.9.3)` | **the mutation controls are a catalog run on every `extended` tier**, and **`extended` passes** for the first time. `cargo xtask mutate`: eight entries, each checked to apply once, name its killing tests and restore byte for byte; S0.4's blind spot reproduced by command. `PROGRAM.9` closed |
| `PROGRAM.6` → `PROGRAM.6.1` | `ARCHOGEN-PROGRAM-0133 (leaf PROGRAM.6.1)` | **everything versioned is in one register, derived from the code.** Seven surfaces with what changes and pins each; F25's home named (`M6.4`); `VERSION-REGISTER` refuses an unannounced format, bump or stale entry. `PROGRAM.6` decomposed |
| `PROGRAM.6.3` → `PROGRAM.6` | `ARCHOGEN-PROGRAM-0135 (leaf PROGRAM.6.3)` | **each evidence format is held to its identifier** — a golden per identifier, never rewritten: the provenance's shape and the cost contract's text. `PROGRAM.6` closed: the register, frozen verdicts and format goldens |
| `PROGRAM.17` → `PROGRAM.17.1` | `ARCHOGEN-PROGRAM-0138 (leaf PROGRAM.17.1)` | **the live-document size-containment doctrine is adopted, with a measured inventory** — a project-owned copy, an adoption note per surface, and the snapshots' chronology recorded as debt. `PROGRAM.17` decomposed |
| `PROGRAM.17.3` → `PROGRAM.17` | `ARCHOGEN-PROGRAM-0140 (leaf PROGRAM.17.3)` | **the history lifecycle choices go to the director** — findings §8 with measurements and a recommendation; `PROGRAM.31`/`.32` filed blocked; `PROGRAM.17` closed |
| `PROGRAM.20` → `PROGRAM.20.1` | `ARCHOGEN-PROGRAM-0142 (leaf PROGRAM.20.1)` | **a restated order is a verified copy** — `STATED-ORDER` checks frontier rows, snapshot heads, successor lists and the changelog's order; its first run found `M2.9` stated two ways. `PROGRAM.20` decomposed |
| `PROGRAM.20.3` → `PROGRAM.20` | `ARCHOGEN-PROGRAM-0144 (leaf PROGRAM.20.3)` | **a figure added to a live document says what keeps it true** — `FIGURE-REGISTER`, a ratchet with a classifying register. `PROGRAM.20` closed: orders, transcripts and figures each have an instrument |
| `PROGRAM.35.2` → `PROGRAM.35` | `ARCHOGEN-PROGRAM-0198 (leaf PROGRAM.35.2)` | **every route out of the README registered and bounded, or its debt owned** — `README-ROUTES` derives the routes from the page's links and its guards' actual hints; `PROGRAM.35` closed |
| `PROGRAM` | `ARCHOGEN-PROGRAM-0331 (leaf PROGRAM)` | **`PROGRAM.47` sealed**, its 30 closed leaves into `docs/task-history/PROGRAM/`: `docs/tasks/` had grown 1 653 bytes over its 819 200-byte ceiling with `API.6.3`'s leaf |
| `PROGRAM` | `ARCHOGEN-PROGRAM-0450 (leaf PROGRAM)` | **`PROGRAM.55` and `PROGRAM.56` sealed**, their 3 closed leaves into `docs/task-history/PROGRAM/`: `docs/tasks/` had grown 1 324 bytes over its 819 200-byte ceiling with `M3.1.1`'s step 27 |
| `PROGRAM` | `ARCHOGEN-PROGRAM-0460 (leaf PROGRAM)` | **`PROGRAM.57`, `PROGRAM.58` and `PROGRAM.59` sealed**, their 3 closed leaves into `docs/task-history/PROGRAM/`: `docs/tasks/` would have grown 1 331 bytes over its 819 200-byte ceiling with `M3.1.2`'s decomposition |
| `PROGRAM` | `ARCHOGEN-PROGRAM-0489 (leaf PROGRAM)` | **`PROGRAM.10` and `.61`–`.65` sealed**, 13 closed leaves into `docs/task-history/PROGRAM/`, when `README-ROUTES` refused `docs/tasks/` at 820 957 bytes over its 819 200; `docs/tasks/` 817 470 → 752 861 bytes, measured from `HEAD` and the tree |
| `PROGRAM.124` | `ARCHOGEN-PROGRAM-1122 (leaf PROGRAM.124)` | **a log row's note no bar to its seal**: `LOGS`, its arm, the record; 219 log rows and 31 leaves sealed across `M4`, `M2`, `M3` and `PROGRAM`, `docs/tasks/` 785285 → 614992 bytes |
| `PROGRAM.125` | `ARCHOGEN-PROGRAM-1135 (leaf PROGRAM.125)` | **every crate a row in the map**: eleven rows, and `BOOK-COVERAGE`'s second leg with four arms |
| `PROGRAM.126` | `ARCHOGEN-PROGRAM-1143 (leaf PROGRAM.126)` | **twenty-second daily artifact cleanup**: the main debug incremental cache, 30872 files / 3821526631 bytes, removed after `handoff: OK` |
| `PROGRAM.127` | `ARCHOGEN-PROGRAM-1157 (leaf PROGRAM.127)` | **the host guard**: the machine is shared and holds 24 GB, so a watcher stops this repository's processes before the director's RAM lines, never another project's; `make` and every agent's shell command go through it |
| `PROGRAM.127.1` | `ARCHOGEN-PROGRAM-1158 (leaf PROGRAM.127.1)` | **the host guard's review, round 1 answered**: a process is ours only by what it shows of itself, so another project's build is left alone; the watcher survives its own trouble and says when it is blind; nothing rides in on a read |
| `PROGRAM.127.2` | `ARCHOGEN-PROGRAM-1159 (leaf PROGRAM.127.2)` | **the host guard's review, round 2 answered**: a terminal known by being told no command; a root exactly itself; one job at a time by what the process table shows; nothing by a name across the host; said to be a guard against accident, no sandbox |
| `PROGRAM.127.3` | `ARCHOGEN-PROGRAM-1160 (leaf PROGRAM.127.3)` | **the host guard's review, round 3 answered**: a helper passed over only while small, so a job an agent's shell execs is held; `pkill` judged by its own stage; a read is a read, each tool's writing options its own; growth over a window; everyday commands no longer refused |
| `PROGRAM.127.4` | `ARCHOGEN-PROGRAM-1161 (leaf PROGRAM.127.4)` | **the host guard's review, round 4 answered by method**: the lists of names are gone; a host's line goes by what a process holds and a disk's by what it writes; the hook refuses only what it cannot read, stopping by a name, and a heavy command with no room |
| `PROGRAM.127.5` | `ARCHOGEN-PROGRAM-1163 (leaf PROGRAM.127.5)` | **the host guard's review, round 5 answered by method**: a job is what is judged and stopped; each line judges alone and they add up; the hook reads a command by the shell's grammar, held to the real shell, and tells a stop without refusing anything |
| `PROGRAM.127.6` | `ARCHOGEN-PROGRAM-1164 (leaf PROGRAM.127.6)` | **paused at the director's word**, at a clean tree, before round 6 began: the resume pointer and the leaf say to wait |
| `PROGRAM.127.6` | `ARCHOGEN-PROGRAM-1165 (leaf PROGRAM.127.6)` | **the host guard's review, round 6 answered**: a volume has fallen only by more than a shared volume costs anyway; what a job takes with it said; the shell reader given a case's branches, arithmetic, arrays and `[[ ]]`, held to each `bash` here; each agent told for itself |
| `PROGRAM.127.7` | `ARCHOGEN-PROGRAM-1168 (leaf PROGRAM.127.7)` | **the host guard's review, round 7 answered**: `make` that only asks is no build; the reader's rare misses read, each held to both `bash`es; a `kill` by a pid passes by any spelling; the root named exactly; what is told says what it leaves out |
| `PROGRAM.127.36` | `ARCHOGEN-PROGRAM-1270 (leaf PROGRAM.127.36)` | **the host guard's review, round 36**: each command keeps its own substitutions through a `run`'s read-through, and a `run`'s input is its job's; the root opened without waiting; a working directory asked of what it would make ours alone; a volume's mount no root. |
| `PROGRAM.135` | `ARCHOGEN-PROGRAM-1269 (leaf PROGRAM.135)` | **an open review history continues in parts**, `<file>.part-2.md` on, read by `REVIEW-COUNTS` as one, so a review of many rounds outgrows its file's ceiling with nothing edited and no ceiling raised. |
| `PROGRAM.127.35` | `ARCHOGEN-PROGRAM-1266 (leaf PROGRAM.127.35)` | **the host guard's review, round 35**: a guard's `run`'s job is read in the run's place, so a search outside it reaches a `kill` inside it behind a wrapper; a `run` is a root's by the directory it names; a working directory the kernel cannot spell is none; `/` is no root. |
| `PROGRAM.134` | `ARCHOGEN-PROGRAM-1265 (leaf PROGRAM.134)` | **the M4 tree sealed again**, 16 closed subtrees and 28 leaves, the review rounds' among them, so the live task trees stand under their ceiling. |
| `PROGRAM.127.34` | `ARCHOGEN-PROGRAM-1263 (leaf PROGRAM.127.34)` | **the host guard's review, round 34**: a cluster of flags costs its length to read; a root is the directory the kernel names, by what both spellings open, and a watcher spells it at each round; the hook reads a `run` by the guard's own reader of an interpreter's flags. |
| `PROGRAM.127.33` | `ARCHOGEN-PROGRAM-1259 (leaf PROGRAM.127.33)` | **the host guard's review, round 33**: a guard is known by its words, each read once, and by no pattern, so a command line costs its length to read; a row of blanks is a row; an empty first word is never read past; a watcher writes its root whole; a root's Unicode form is the volume's. |
| `PROGRAM.127.32` | `ARCHOGEN-PROGRAM-1257 (leaf PROGRAM.127.32)` | **the host guard's review, round 32**: a row of the process table ends at a line feed and nowhere else, so no argument blinds a guard; a watcher given its root relatively is known from any directory; a nameless process goes by its file. |
| `PROGRAM.127.31` | `ARCHOGEN-PROGRAM-1254 (leaf PROGRAM.127.31)` | **the host guard's review, round 31**: a process's arguments are the kernel's own, each whole, so a terminal whose flag's value holds a space is spared; a watcher is known by its arguments, for its root; `-n` is held of no shell whose `-c` takes the next word; a taking up is said by the image that came of it. |
| `PROGRAM.127.30` | `ARCHOGEN-PROGRAM-1250 (leaf PROGRAM.127.30)` | **the host guard's review, round 30**: a shell's flags are read in `bash`'s two passes, each `o` of a cluster taking a word; what a watcher holds unsaid goes with it to the image it becomes; a number that is no one process is no watcher and is never signalled. |
| `PROGRAM.127.29` | `ARCHOGEN-PROGRAM-1248 (leaf PROGRAM.127.29)` | **the host guard's review, round 29**: `bash`'s long flags are read with one dash as with two; a cluster is read letter by letter, as its interpreter reads one; a watcher whose interpreter's file is gone is still the watcher; a broken save after a clean is said once a directory stands. |
| `PROGRAM.127.28` | `ARCHOGEN-PROGRAM-1244 (leaf PROGRAM.127.28)` | **the host guard's review, round 28, answered by method**: what an interpreter runs is read once, by its own flags, so a flag's value is never a program and a program's text holds no name; the log's lines are read by one reader, by the shape the guard writes them in; a taking up after a clean makes no directory and is said once one stands. |
| `PROGRAM.127.27` | `ARCHOGEN-PROGRAM-1241 (leaf PROGRAM.127.27)` | **the host guard's review, round 27**: the search for an agent's program holds an exact table of flags with values, each run here; a watcher makes its directory as it starts and then only for a stop, so a removal of its root goes through, and `status` and `stop` are right the first time after a clean; its notes are written again wherever they do not name it, and none into a root remade as another's; a line of the wrong type costs a job's status no more. |
| `PROGRAM.127.26` | `ARCHOGEN-PROGRAM-1236 (leaf PROGRAM.127.26)` | **the host guard's review, round 26**: the search for an agent's program ends at what runs; a `make`'s recipe said as it is read; what cannot be read is refused by the hook itself; bytes that are no text hide nothing; a bad line costs nothing, however deep. |
| `PROGRAM.127.25` | `ARCHOGEN-PROGRAM-1231 (leaf PROGRAM.127.25)` | **the host guard's review, round 25, answered by method**: each exception of the reader's holds of an exact form; two sentences say the line and the file they hold of; a gate's input is no other gate's list. |
| `PROGRAM.127.24` | `ARCHOGEN-PROGRAM-1226 (leaf PROGRAM.127.24)` | **the host guard's review, round 24**: a nested checkout's turn and a `make` at a host's line said as they are; a later flag undoing `kill -0` and `bash -n`; local settings that turn the hook off refused |
| `PROGRAM.127.23` | `ARCHOGEN-PROGRAM-1222 (leaf PROGRAM.127.23)` | **the host guard's review, round 23, answered by method**: an exception that failed open gone; a `GNUmakefile` read from the index; the self-test's two races waited out; the kill reader said to be a courtesy |
| `PROGRAM.127.22` | `ARCHOGEN-PROGRAM-1218 (leaf PROGRAM.127.22)` | **the host guard's review, round 22, answered by removing what kept failing**: the Makefile held whole by a digest, `make` no longer asked; `tightened` asked of its lines; more wrappers read |
| `PROGRAM.127.21` | `ARCHOGEN-PROGRAM-1216 (leaf PROGRAM.127.21)` | **the host guard's review, round 21, answered by method**: `make` asked what each recipe would run, the file asked what each limit is; what `ps` brackets is a moment; a `run` read as `main` reads it; numbers as the tools read them |
| `PROGRAM.127.20` | `ARCHOGEN-PROGRAM-1213 (leaf PROGRAM.127.20)` | **the host guard's review, round 20**: a `make` waits on through what `ps` brackets; a `run` is of the root it says before its `--`; the reader through the guard's `run` and `env -`, and numbers however spelt; what make and the limits can be told; a path holding a space is one path |
| `PROGRAM.127.19` | `ARCHOGEN-PROGRAM-1211 (leaf PROGRAM.127.19)` | **the host guard's review, round 19 answered**: the self-test spared by its own path; what a floor counts of a process newly ours said |
| `PROGRAM.127.18` | `ARCHOGEN-PROGRAM-1206 (leaf PROGRAM.127.18)` | **the host guard's review, round 18 answered**: the self-test spared by its own name and a job only to what `check` says; dashless `ps` read as dashed |
| `PROGRAM.127.17` | `ARCHOGEN-PROGRAM-1202 (leaf PROGRAM.127.17)` | **the host guard's review, round 17 answered**: a script named like the guard is a job; `-d` selects, `-M` takes no value |
| `PROGRAM.127.16` | `ARCHOGEN-PROGRAM-1199 (leaf PROGRAM.127.16)` | **the host guard's review, round 16 answered**: a stamp is of a whole run of the bytes it began on; all the check reads is read as staged |
| `PROGRAM.127.15` | `ARCHOGEN-PROGRAM-1197 (leaf PROGRAM.127.15)` | **the host guard's review, round 15 answered, by method**: a commit holds the file as staged, compiled, and a self-test passed on those bytes; what names a watcher outlasts a clean; `ps -ef -p 1` is a search |
| `PROGRAM.127.14` | `ARCHOGEN-PROGRAM-1194 (leaf PROGRAM.127.14)` | **the host guard's review, round 14 answered**: a commit holds that every name the guard reads is bound; a second, broken save in the instant of a taking up is stated; `-U 501` stays caught and `ps "$PID"` is a pid |
| `PROGRAM.127.13` | `ARCHOGEN-PROGRAM-1189 (leaf PROGRAM.127.13)` | **the host guard's review, round 13 answered**: a watcher takes itself up from the file as it stands, unless a save can be seen to have broken it, with no proof and no moment unwatched |
| `PROGRAM.127.12` | `ARCHOGEN-PROGRAM-1185 (leaf PROGRAM.127.12)` | **the host guard's review, round 12 answered by removing what kept failing**: a watcher no longer takes up its own change, and leaves; the next command or `run` starts one from the file |
| `PROGRAM.127.11` | `ARCHOGEN-PROGRAM-1182 (leaf PROGRAM.127.11)` | **the host guard's review, round 11 answered**: `make` runs no proof, and looks at the guard's last line; a proof goes in as a watcher does; no line of a round ends a watcher; what a `pkill` goes by read as `kill`'s target |
| `PROGRAM.127.10` | `ARCHOGEN-PROGRAM-1178 (leaf PROGRAM.127.10)` | **the host guard's review, round 10 answered**: the compiled file walked and the watcher's own loop run, in a proof that stands beside a watcher still reading; a root named as resolved; `make` asks before any recipe; a self-test waits for room |
| `PROGRAM.127.9` | `ARCHOGEN-PROGRAM-1176 (leaf PROGRAM.127.9)` | **the host guard's review, round 9 answered**: a changed guard proves it runs before the watcher takes it up; a `for` needs no `;` before its body; `-P 1` and `-v` name nothing; an incomplete self-test carried up |
| `PROGRAM.132` | `ARCHOGEN-PROGRAM-1224 (leaf PROGRAM.132)` | **the twenty-fourth artifact cleanup**: only the idle incremental caches, 3.3 GB |
| `PROGRAM.133` | `ARCHOGEN-PROGRAM-1242 (leaf PROGRAM.133)` | **the twenty-fifth artifact cleanup**: only the idle incremental cache, 2.2 GB |
| `PROGRAM.131` | `ARCHOGEN-PROGRAM-1221 (leaf PROGRAM.131)` | **the M4 tree sealed**: 44 closed subtrees, 53 leaves, `docs/tasks/` under its ceiling |
| `PROGRAM.130` | `ARCHOGEN-PROGRAM-1175 (leaf PROGRAM.130)` | **the twenty-third artifact cleanup**: only the idle incremental caches, 2.8 GB |
| `PROGRAM.127.8` | `ARCHOGEN-PROGRAM-1171 (leaf PROGRAM.127.8)` | **the host guard's review, round 8 answered**: `kill` read as it reads its words, so a process group found by name is refused; `Monitor` checked; a space in a checkout's path; a guard saved broken ends nothing |
| `PROGRAM.129` | `ARCHOGEN-PROGRAM-1169 (leaf PROGRAM.129)` | **the director's word on RAM**: observe the trend, and say something only when it is over 90% and still rising; the guard's lines stand as they are |
| `PROGRAM.128` | `ARCHOGEN-PROGRAM-1162 (leaf PROGRAM.128)` | **a chip's document starts at `chipdoc`'s `AGENTS.md`**, the director's word: the record, its index row, the ledger and `TOOLBOX.md`; `REQ-007` fulfilled, the corpus on this volume, and a document never copied into this public repository, each corrected |
| `PROGRAM.136` | `ARCHOGEN-PROGRAM-1272 (leaf PROGRAM.136)` | **the twenty-sixth artifact cleanup**: only the idle incremental cache, 3.2 GB |
| `PROGRAM.137` | `ARCHOGEN-PROGRAM-1273 (leaf PROGRAM.137)` | **fsmgen's containment doctrine at `58d5edef7` adopted**: the body verbatim, its book exception agreeing with the ruling of `2026-10-18`; no limit here moves |
| `PROGRAM.127.37` | `ARCHOGEN-PROGRAM-1276 (leaf PROGRAM.127.37)` | **the host guard's review, round 37**: a placeholder read again stands for its substitution, and what feeds a shell's text feeds its commands; a run's further pipelines drawn from one count; the lock opened as a directory without waiting; here-strings and groups' redirections read; the mount sentence made true |
| `PROGRAM.138` | `ARCHOGEN-PROGRAM-1279 (leaf PROGRAM.138)` | **the twenty-seventh artifact cleanup, the director's**: 164.9 GB of nested cargo build output and test scratch, `target/` 180.0 GB to 16.5 GB, every piece of evidence kept |
| `PROGRAM.127.38` | `ARCHOGEN-PROGRAM-1280 (leaf PROGRAM.127.38)` | **the host guard's review, round 38**: a placeholder counted wherever text is taken as it stands; a nested run's job standing for the outer run's words; a group fed a here-document a stated limit; a volume of `/`'s group and a root that is no directory refused by every verb |
| `PROGRAM.127.39` | `ARCHOGEN-PROGRAM-1283 (leaf PROGRAM.127.39)` | **the host guard's review, round 39**: each reader a word builds hands back its count and stands where the text was a word; an unquoted here-document expanded before its shell reads it; a gone root said alike by every verb but the hook |
| `PROGRAM.127.40` | `ARCHOGEN-PROGRAM-1286 (leaf PROGRAM.127.40)` | **the host guard's review, round 40**: a backslash before a line feed taken out as the shell takes it out; a here-document's delimiter taken as written and its end found as the shell finds it; a self-test row reading the host's refusal; a gone root's heavy refusal saying so |
| `PROGRAM.139` | `ARCHOGEN-PROGRAM-1287 (leaf PROGRAM.139)` | **the M4 tree sealed again**: 10 closed subtrees and 10 leaves, so `docs/tasks/` is under its ceiling |

## Changelog

- `2026-10-11`: `PROGRAM.76` closed after independent round 15 found zero defects or actionable arm gaps.

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
- `2026-10-10`: `PROGRAM.53` done — a cited focused run backs the tree committed, by a stamp the gate reads.
- `2026-10-10`: `PROGRAM.75` done — the handoff census's second sample proposed to bedrock, `BR-001`.
- `2026-10-10`: `PROGRAM.76` done — the sealed evaluation set's text out of the tree, the sealing commit its custody.
- `2026-10-10`: `PROGRAM.76` reopened by its review round 1; its review open since.
- `2026-10-10`: `PROGRAM.77` filed and fixed — the mutation runner's group kill through kill(2), after CI lost its runner; it closes on the runner's green.
- `2026-10-10`: `PROGRAM.77` done — CI green on the runner again, the restoring push's `rust` run in 2 and 4 minutes.
- `2026-10-10`: `PROGRAM.78` filed by `PROGRAM.69`'s review round 26 — the sealing self-test's interrupt arm timing-dependent under a parallel sweep. Pre-existing, owned; this line added by round 27's answer.
- `2026-10-11`: `PROGRAM.79` filed and done — `docs/specs/` raised to 524 288 bytes, measured, as three trust designs moved in.

- `2026-10-11`: `PROGRAM.80` filed by the confirmatory self-tests: the handoff census fails open when `ps` is denied; high priority, owned, the wrapper repair before another hand-off.

- `2026-10-11`: `PROGRAM.80` done — failed or missing actual OS census reads now refuse; the project handoff command is the guarded wrapper, the held native file unchanged.

- `2026-10-18`: `PROGRAM.87` restores complete headed logs without changing prior record text; native history/rendering and original input restoration verified.

- `2026-11-04`: `PROGRAM.125` filed by `M4.2.2` and done: every workspace member a row in the knowledge map, and a gate that asks for one.

- `2026-11-05`: `PROGRAM.127` done: the host guard, by the director's instruction; its review `PROGRAM.127.1` filed.

- `2026-11-05`: `PROGRAM.127.1` done: the guard's round 1, twenty-five findings answered; `PROGRAM.127.2` filed for round 2.

- `2026-11-05`: `PROGRAM.127.2` done: the guard's round 2, twenty-three findings answered; `PROGRAM.127.3` filed for round 3.

- `2026-11-05`: `PROGRAM.127.3` done: the guard's round 3, twenty findings answered; `PROGRAM.127.4` filed for round 4.

- `2026-11-05`: `PROGRAM.127.4` done: the guard's round 4, sixteen findings answered by removing its lists; `PROGRAM.127.5` filed for round 5.

- `2026-11-05`: `PROGRAM.128` done: a chip's document starts at `chipdoc`'s `AGENTS.md`, by the director's word; the pause lifted the same day, `PROGRAM.127.5` at work.

- `2026-11-05`: `PROGRAM.127.5` done: the guard's round 5, twenty-one findings answered by a job as the unit and the shell's grammar; `PROGRAM.127.6` filed for round 6.

- `2026-11-05`: `PROGRAM.127.6` done: the guard's round 6, thirteen findings answered, two of them defects; `PROGRAM.127.7` filed for round 7.

- `2026-11-05`: `PROGRAM.127.7` done: the guard's round 7, thirteen findings answered, one a defect; `PROGRAM.127.8` filed for round 8.

- `2026-11-06`: `PROGRAM.129` done: the director's word on RAM, in the record and the book.

- `2026-11-06`: `PROGRAM.127.8` done: the guard's round 8, fourteen findings answered, one a defect; `PROGRAM.127.9` filed for round 9.

- `2026-11-06`: `PROGRAM.130` done: the twenty-third artifact cleanup.

- `2026-11-06`: `PROGRAM.127.9` done: the guard's round 9, ten findings answered, one a defect; `PROGRAM.127.10` filed for round 10.
- `2026-11-06`: `PROGRAM.127.10` done: the guard's round 10, twelve findings answered, three defects; `PROGRAM.127.11` filed for round 11.
- `2026-11-06`: `PROGRAM.127.11` done: the guard's round 11, eight findings answered, two defects; `PROGRAM.127.12` filed for round 12.
- `2026-11-06`: `PROGRAM.127.12` done: the guard's round 12, twelve findings answered, three defects, by removing what kept failing; `PROGRAM.127.13` filed for round 13.
- `2026-11-06`: `PROGRAM.127.13` done: the guard's round 13, eleven findings answered, five defects; `PROGRAM.127.14` filed for round 14.
- `2026-11-06`: `PROGRAM.127.14` done: the guard's round 14, ten findings answered, two defects; `PROGRAM.127.15` filed for round 15.
- `2026-11-06`: `PROGRAM.127.15` done: the guard's round 15, eleven findings answered, four defects, by method; `PROGRAM.127.16` filed for round 16.
- `2026-11-06`: `PROGRAM.127.16` done: the guard's round 16, eleven findings answered, four defects; `PROGRAM.127.17` filed for round 17.
- `2026-11-06`: `PROGRAM.127.17` done: the guard's round 17, eight findings answered, one defect; `PROGRAM.127.18` filed for round 18.
- `2026-11-06`: `PROGRAM.127.18` done: the guard's round 18, seven findings answered, two defects; `PROGRAM.127.19` filed for round 19.
- `2026-11-06`: `PROGRAM.127.19` done: the guard's round 19, four findings answered, two defects, both mild; `PROGRAM.127.20` filed for round 20.
- `2026-11-06`: `PROGRAM.127.20` done: the guard's round 20, ten findings answered, two defects; `PROGRAM.127.21` filed for round 21.
- `2026-11-06`: `PROGRAM.127.21` done: the guard's round 21, ten findings answered, three defects, by method; `PROGRAM.127.22` filed for round 22.
- `2026-11-07`: `PROGRAM.127.22` done: the guard's round 22, eleven findings answered, three defects, by method; `PROGRAM.127.23` filed for round 23.
- `2026-11-07`: `PROGRAM.131` done: the M4 tree sealed, 53 leaves, `docs/tasks/` under its ceiling.
- `2026-11-07`: `PROGRAM.127.23` done: the guard's round 23, seven findings answered, three defects, by method; `PROGRAM.127.24` filed for round 24.
- `2026-11-07`: `PROGRAM.132` done: the twenty-fourth artifact cleanup.
- `2026-11-07`: `PROGRAM.127.24` done: the guard's round 24, six findings answered, three defects; `PROGRAM.127.25` filed for round 25.
- `2026-11-07`: `PROGRAM.127.25` done: the guard's round 25, eight findings answered, five defects, by method; `PROGRAM.127.26` filed for round 26.
- `2026-11-07`: `PROGRAM.127.26` done: the guard's round 26, seven findings answered, two defects; `PROGRAM.127.27` filed for round 27.
- `2026-11-07`: `PROGRAM.127.27`'s reader has reported, and its answers are prepared and not landed: report `target/p12727/report.md`, patch `target/p12727/mine/final.diff`, which still wants the whole self-test and its evidence. Two defects, both mild: `programs_named` reads a bare script after any flag as that flag's value, sparing `python3 -u tool.py /x/codex`, and a watcher remakes its directory under a removal of its root, failing `git worktree remove --force`. Beside them: a log line of the wrong type costing `run` its status; a job elsewhere whose `run` is killed outright; too-deep local settings; `gmake -Onone`; three sentences.
- `2026-11-10`: `PROGRAM.127.27` done: the guard's round 27, nine findings answered, two defects, the prepared answer corrected before it landed; `PROGRAM.127.28` filed for round 28.
- `2026-11-10`: `PROGRAM.133` done: the twenty-fifth artifact cleanup.
- `2026-11-10`: `PROGRAM.127.28` done: the guard's round 28, four findings answered, one defect, by method; `PROGRAM.127.29` filed for round 29.
- `2026-11-10`: `PROGRAM.127.29` done: the guard's round 29, seven findings answered, one defect; `PROGRAM.127.30` filed for round 30.
- `2026-11-10`: two of `M4`'s readers have reported, each saved and not yet answered: `M4.12.1.4`, the application inputs' round 4, not acceptable, 8 findings, 4 defects (`target/m4328/report.md`); `M4.1.7.2.3`, format `/2`'s round 3, not acceptable, 11 findings, 9 defects (`target/m4329/report.md`). They share two findings, a `binary`'s meaning and what the initialisation calls, so they are answered together.
- `2026-11-10`: `PROGRAM.127.30` done: the guard's round 30, six findings answered, two defects; `PROGRAM.127.31` filed for round 31. It went ahead of format `/2`'s answer for its third finding, the host's safety.
- `2026-11-10`: `PROGRAM.127.31` done: the guard's round 31, six findings answered, three defects, one in ordinary work; `PROGRAM.127.32` filed for round 32.
- `2026-11-10`: `PROGRAM.127.32` done: the guard's round 32, three findings answered, two defects, one met in ordinary work; `PROGRAM.127.33` filed for round 33.
- `2026-11-10`: `PROGRAM.127.33` done: the guard's round 33, five findings answered, one defect, written to and failing open; `PROGRAM.127.34` filed for round 34.
- `2026-11-10`: `PROGRAM.127.34` done: the guard's round 34, four findings answered, two defects, both in round 33's own answers; `PROGRAM.127.35` filed for round 35.
- `2026-11-10`: `PROGRAM.134` done: the M4 tree sealed again, 16 closed subtrees, so the live task trees stand under their ceiling.
- `2026-11-10`: `PROGRAM.127.35` done: the guard's round 35, six findings answered, one defect, in the hook's reading of a guard's `run`; `PROGRAM.127.36` filed for round 36.
- `2026-11-10`: `PROGRAM.135` done: an open review history continues in parts, read as one by `REVIEW-COUNTS`.
- `2026-11-10`: `PROGRAM.127.36` done: the guard's round 36, five findings answered, two defects, in round 35's reading of a `run`; `PROGRAM.127.37` filed for round 37.
- `2026-11-13`: `PROGRAM.136` done: the twenty-sixth artifact cleanup, the idle incremental cache alone; `PROGRAM.137` filed, fsmgen's containment revision `58d5edef7` to review.
- `2026-11-13`: `PROGRAM.137` done: fsmgen's containment revision `58d5edef7` adopted verbatim; the other three policy sources unchanged.
- `2026-11-13`: `PROGRAM.127.37` done: the guard's round 37, seven findings answered, four defects, two failing open in the reader; `PROGRAM.127.38` filed for round 38.
- `2026-11-14`: `PROGRAM.138` done: the twenty-seventh artifact cleanup, asked by the director: `target/` from 180.0 GB to 16.5 GB, build output and test scratch alone.
- `2026-11-14`: `PROGRAM.127.38` done: the guard's round 38, seven findings answered, four defects, three failing open in round 37's reading; `PROGRAM.127.39` filed for round 39.
- `2026-11-14`: `PROGRAM.127.39` done: the guard's round 39, three findings answered, two defects failing open in the reader of a kill; `PROGRAM.127.40` filed for round 40.
- `2026-11-14`: `PROGRAM.127.40` done: the guard's round 40, four findings answered, two defects, one failing open in the reader; `PROGRAM.127.41` filed for round 41.
- `2026-11-14`: `PROGRAM.139` done: the M4 tree sealed again, ten closed leaves of two reviews, `docs/tasks/` under its ceiling.
