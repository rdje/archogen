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
  Status: `pending` — round 8, a fresh reader over round 7's answer. It closes on a round that finds no defect, and a form written to go round the guard is a stated limit, not a defect.

- ID: `PROGRAM.128`
  Status: `done` — `2026-11-05`: by the director's word, "If you ever need new CPU, DSP, MCU, BOARD, ISA or in general digital chip PDF just look here", naming `chipdoc`'s `AGENTS.md`. The record, its index row, the book's ledger and a `TOOLBOX.md` row now start any chip document's need there. Reading it corrected three things of ours: the record and the ledger still said `REQ-007` was `requested`, fulfilled `2026-09-30`; the ledger said the corpus was off this volume; and the record said what is adopted is copied here, where most of the corpus may not be republished and this repository is public, so a fact and its citation are copied and never a document. A documents-only leaf.
  Verification: `python3 -I -B "$ARCHOGEN_CHIPDOC_ROOT/scripts/find_docs.py" --text FE310 --retrieve` → a path, `pages: 20`, a `sha256:` and a `cite:` line; `--peer archogen --count` → `27`; `git -C "$ARCHOGEN_CHIPDOC_ROOT" log -1 --format=%h` → `226cfb0`; `grep -n 'REQ-007' "$ARCHOGEN_CHIPDOC_ROOT/catalog/REQUESTS.md"` → `**fulfilled** 2026-09-30`; `stat -f .` from each checkout → the same file system id, `100001c0000001a`; `awk` over that ledger for the rows naming archogen → `REQ-006` and `REQ-007` alone; `bash scripts/check_source_ledger.sh` and `bash scripts/check_doctrines.sh` on the staged tree.
  Commit: `ARCHOGEN-PROGRAM-1162 (leaf PROGRAM.128)`

- ID: `PROGRAM.129`
  Status: `done` — `2026-11-06`: by the director's word: "Regarding RAM, Just observe the trend, it shouldn't cross 90% and still increasing. don't stress out. If it's cross 90% and keep increasing, then you might say, well, I think we might have a situation here. other than that, just chill." The record's `How to apply` and the book's section say so, and a reader's brief from now on. The guard's lines stand as they are. A documents-only leaf.
  Verification: `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, its lines untouched; `git diff --cached --stat -- scripts` → empty.
  Commit: `ARCHOGEN-PROGRAM-1169 (leaf PROGRAM.129)`

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
| `PROGRAM.129` | `ARCHOGEN-PROGRAM-1169 (leaf PROGRAM.129)` | **the director's word on RAM**: observe the trend, and say something only when it is over 90% and still rising; the guard's lines stand as they are |
| `PROGRAM.128` | `ARCHOGEN-PROGRAM-1162 (leaf PROGRAM.128)` | **a chip's document starts at `chipdoc`'s `AGENTS.md`**, the director's word: the record, its index row, the ledger and `TOOLBOX.md`; `REQ-007` fulfilled, the corpus on this volume, and a document never copied into this public repository, each corrected |

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
