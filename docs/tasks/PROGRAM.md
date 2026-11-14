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
  Status: `done` — sealed in [`PROGRAM/PROGRAM.124.md`](../task-history/PROGRAM/PROGRAM.124.md); commit `ARCHOGEN-PROGRAM-1122`

- ID: `PROGRAM.125`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.125.md`](../task-history/PROGRAM/PROGRAM.125.md); commit `ARCHOGEN-PROGRAM-1135`

- ID: `PROGRAM.126`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.126.md`](../task-history/PROGRAM/PROGRAM.126.md); commit `ARCHOGEN-PROGRAM-1143`

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
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.1.md`](../task-history/PROGRAM/PROGRAM.127.1.md); commit `ARCHOGEN-PROGRAM-1158`

- ID: `PROGRAM.127.2`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.2.md`](../task-history/PROGRAM/PROGRAM.127.2.md); commit `ARCHOGEN-PROGRAM-1159`

- ID: `PROGRAM.127.3`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.3.md`](../task-history/PROGRAM/PROGRAM.127.3.md); commit `ARCHOGEN-PROGRAM-1160`

- ID: `PROGRAM.127.4`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.4.md`](../task-history/PROGRAM/PROGRAM.127.4.md); commit `ARCHOGEN-PROGRAM-1161`

- ID: `PROGRAM.127.5`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.5.md`](../task-history/PROGRAM/PROGRAM.127.5.md); commit `ARCHOGEN-PROGRAM-1163`

- ID: `PROGRAM.127.6`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.6.md`](../task-history/PROGRAM/PROGRAM.127.6.md); commit `ARCHOGEN-PROGRAM-1165`

- ID: `PROGRAM.127.7`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.7.md`](../task-history/PROGRAM/PROGRAM.127.7.md); commit `ARCHOGEN-PROGRAM-1168`

- ID: `PROGRAM.127.8`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.8.md`](../task-history/PROGRAM/PROGRAM.127.8.md); commit `ARCHOGEN-PROGRAM-1171`

- ID: `PROGRAM.127.9`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.9.md`](../task-history/PROGRAM/PROGRAM.127.9.md); commit `ARCHOGEN-PROGRAM-1176`

- ID: `PROGRAM.127.10`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.10.md`](../task-history/PROGRAM/PROGRAM.127.10.md); commit `ARCHOGEN-PROGRAM-1178`

- ID: `PROGRAM.127.11`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.11.md`](../task-history/PROGRAM/PROGRAM.127.11.md); commit `ARCHOGEN-PROGRAM-1182`

- ID: `PROGRAM.127.12`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.12.md`](../task-history/PROGRAM/PROGRAM.127.12.md); commit `ARCHOGEN-PROGRAM-1185`

- ID: `PROGRAM.127.13`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.13.md`](../task-history/PROGRAM/PROGRAM.127.13.md); commit `ARCHOGEN-PROGRAM-1189`

- ID: `PROGRAM.127.14`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.14.md`](../task-history/PROGRAM/PROGRAM.127.14.md); commit `ARCHOGEN-PROGRAM-1194`

- ID: `PROGRAM.127.15`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.15.md`](../task-history/PROGRAM/PROGRAM.127.15.md); commit `ARCHOGEN-PROGRAM-1197`

- ID: `PROGRAM.127.16`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.16.md`](../task-history/PROGRAM/PROGRAM.127.16.md); commit `ARCHOGEN-PROGRAM-1199`

- ID: `PROGRAM.127.17`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.17.md`](../task-history/PROGRAM/PROGRAM.127.17.md); commit `ARCHOGEN-PROGRAM-1202`

- ID: `PROGRAM.127.18`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.18.md`](../task-history/PROGRAM/PROGRAM.127.18.md); commit `ARCHOGEN-PROGRAM-1206`

- ID: `PROGRAM.127.19`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.19.md`](../task-history/PROGRAM/PROGRAM.127.19.md); commit `ARCHOGEN-PROGRAM-1211`

- ID: `PROGRAM.127.20`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.20.md`](../task-history/PROGRAM/PROGRAM.127.20.md); commit `ARCHOGEN-PROGRAM-1213`

- ID: `PROGRAM.127.21`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.21.md`](../task-history/PROGRAM/PROGRAM.127.21.md); commit `ARCHOGEN-PROGRAM-1216`

- ID: `PROGRAM.127.22`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.22.md`](../task-history/PROGRAM/PROGRAM.127.22.md); commit `ARCHOGEN-PROGRAM-1218`

- ID: `PROGRAM.127.23`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.23.md`](../task-history/PROGRAM/PROGRAM.127.23.md); commit `ARCHOGEN-PROGRAM-1222`

- ID: `PROGRAM.127.24`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.24.md`](../task-history/PROGRAM/PROGRAM.127.24.md); commit `ARCHOGEN-PROGRAM-1226`

- ID: `PROGRAM.127.25`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.25.md`](../task-history/PROGRAM/PROGRAM.127.25.md); commit `ARCHOGEN-PROGRAM-1231`

- ID: `PROGRAM.127.26`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.26.md`](../task-history/PROGRAM/PROGRAM.127.26.md); commit `ARCHOGEN-PROGRAM-1236`

- ID: `PROGRAM.127.27`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.27.md`](../task-history/PROGRAM/PROGRAM.127.27.md); commit `ARCHOGEN-PROGRAM-1241`

- ID: `PROGRAM.127.28`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.28.md`](../task-history/PROGRAM/PROGRAM.127.28.md); commit `ARCHOGEN-PROGRAM-1244`

- ID: `PROGRAM.127.29`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.29.md`](../task-history/PROGRAM/PROGRAM.127.29.md); commit `ARCHOGEN-PROGRAM-1248`

- ID: `PROGRAM.127.30`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.30.md`](../task-history/PROGRAM/PROGRAM.127.30.md); commit `ARCHOGEN-PROGRAM-1250`

- ID: `PROGRAM.127.31`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.31.md`](../task-history/PROGRAM/PROGRAM.127.31.md); commit `ARCHOGEN-PROGRAM-1254`

- ID: `PROGRAM.127.32`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.32.md`](../task-history/PROGRAM/PROGRAM.127.32.md); commit `ARCHOGEN-PROGRAM-1257`

- ID: `PROGRAM.127.33`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.33.md`](../task-history/PROGRAM/PROGRAM.127.33.md); commit `ARCHOGEN-PROGRAM-1259`

- ID: `PROGRAM.127.34`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.34.md`](../task-history/PROGRAM/PROGRAM.127.34.md); commit `ARCHOGEN-PROGRAM-1263`

- ID: `PROGRAM.127.35`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.35.md`](../task-history/PROGRAM/PROGRAM.127.35.md); commit `ARCHOGEN-PROGRAM-1266`

- ID: `PROGRAM.127.36`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.36.md`](../task-history/PROGRAM/PROGRAM.127.36.md); commit `ARCHOGEN-PROGRAM-1270`

- ID: `PROGRAM.127.37`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.37.md`](../task-history/PROGRAM/PROGRAM.127.37.md); commit `ARCHOGEN-PROGRAM-1276`

- ID: `PROGRAM.127.38`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.38.md`](../task-history/PROGRAM/PROGRAM.127.38.md); commit `ARCHOGEN-PROGRAM-1280`

- ID: `PROGRAM.127.39`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.39.md`](../task-history/PROGRAM/PROGRAM.127.39.md); commit `ARCHOGEN-PROGRAM-1283`

- ID: `PROGRAM.127.40`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.40.md`](../task-history/PROGRAM/PROGRAM.127.40.md); commit `ARCHOGEN-PROGRAM-1286`

- ID: `PROGRAM.127.41`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.41.md`](../task-history/PROGRAM/PROGRAM.127.41.md); commit `ARCHOGEN-PROGRAM-1290`

- ID: `PROGRAM.127.42`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.42.md`](../task-history/PROGRAM/PROGRAM.127.42.md); commit `ARCHOGEN-PROGRAM-1293`

- ID: `PROGRAM.127.43`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.43.md`](../task-history/PROGRAM/PROGRAM.127.43.md); commit `ARCHOGEN-PROGRAM-1297`

- ID: `PROGRAM.127.44`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.127.44.md`](../task-history/PROGRAM/PROGRAM.127.44.md); commit `ARCHOGEN-PROGRAM-1300`

- ID: `PROGRAM.127.45`
  Status: `done` — `2026-11-14`: round 45, a fresh reader over round 44's answer: 5 findings, 4 defects, after 7, two in round 44's answers, each failing open on forms written to; and 3 more of this round's own reruns of the earlier rounds' rigs, 2 of them defects older than round 44. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.part-2.md`; its report is `target/p12745/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — its reader's own runs on `57eac8a2`, the real hook for a scratch root (`target/p12745/report.md`): each form exit 0 with a shell running the stub, and its fuzzers 10 misses of about 3,600. This round's reruns: `python3 -I -B target/p12743/land/rig43/fuzz.py 453 600` → a `pkill` bash 3.2 ran, unread; `python3 -I -B target/p12745/land/rig45/fuzz_gather.py 6 3000` → `1 missed`, `cat <()##<() ; pkill -x rustc`. Here, this round's rows on that guard: `python3 -I -B target/p12745/land/red/scripts/selftest_host_guard.py case_the_reader_sees_what_the_shell_runs case_killing_by_name` → `307 passed, 6 failed, 0 not run` (`target/p12745/land/red.log`).
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n -F 'differs = "<<" in plain or' 57eac8a2 -- scripts/host_guard.py` → `1534`, `$[` no marker; `git grep -n -F 'elif two == "${" and self.older and not in_quotes and' 57eac8a2 -- scripts/host_guard.py` → `2230`, 3.2's `${…}` ended so outside double quotes alone; `git grep -n -F 'end = self.arithmetic_end(self.at + 3) if three == "$((" else None' 57eac8a2 -- scripts/host_guard.py` → `2177`, `$$` read as `$` and what follows; `git grep -n -F 'if told and kills_by_name(read_as(shlex.join(told), which), root, which):' 57eac8a2 -- scripts/host_guard.py` → `2808`, a run's job read once; `git grep -n -F 'elif two == "$(" and self.older and (close := self.gathered(self.at + 2, comment_first=not in_quotes))' 57eac8a2 -- scripts/host_guard.py` → `2192`, 3.2's `$(` between double quotes gathered by its expansion's rule; `git grep -n -F 'def braced(at: int) -> int | None:' 57eac8a2 -- scripts/host_guard.py` → `1558`, `gathering`'s `${…}` and, to line 1587, its nested `$(` inside a double-quoted string; `git grep -n -F '            elif c == "#":' 57eac8a2 -- scripts/host_guard.py` → `1889`, a `#` after `<(…)` taken as a comment.
  - [x] **FIX** — `scripts/host_guard.py`: `$$` one token; `$[` among `each_bash`'s markers; 3.2's `${…}` between double quotes too; `kills_by_name` reading the first run's job as each bash; 3.2's `$(` gathered by the parser's rule and `read_older` reading what its expansion takes too; `gathering`'s `older`, a nested `$(` with no comment and `braced` as each bash ends a `${…}`; `glued`. `scripts/selftest_host_guard.py`: rows for each. The record's sentences, G45-4 and G45-5 said.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/host_guard.py run --wait 1800 -- python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 1313 passed, 0 failed, 0 not run`, run exit 0, on the final bytes, `cmp`-equal to `target/p12745/land/draft/host_guard.py`, this round's rows among them (`target/p12745/land/selftest-full.log`); its two cases alone → `313 passed, 0 failed, 0 not run` (`target/p12745/land/green.log`); the reader's forms through the real hook → each exit 2 (`target/p12745/land/rig45/hook_forms.py` on `forms.txt`, `forms2.txt`, `forms3.txt`: 11 refused, 0 passed); its rigs `brackets.py`, `brackets2.py`, `dollars.py` → `0 missed`; `fuzz2.py` and `fuzz_brace.py` seeds 451 and 452 → `0 missed` of 6,000; this round's `fuzz_gather.py` seeds 4 to 13 → `0 missed` of 27,000 but seed 6's `<(…)#`, answered.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; the earlier rounds' rigs on this guard: round 44's 122 forms → no kill passed but `kill -0 "$PID"`, a probe of one pid, its 20 ordinary forms passing, and its fuzzers seed 453 → `0 missed`; round 43's `fuzz2.py` → `0 missed`; round 42's tables → no row open, its fuzzers `0 missed`; round 41's 49 forms → none passed with a kill run, its 20 ordinary forms passing, its here-document fuzzers → `0 fail open` of 1,200; the probes of rounds 37 to 39 → `wrong 0`; `make focused` → `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1304 (leaf PROGRAM.127.45)`

- ID: `PROGRAM.127.46`
  Status: `done` — `2026-11-14`: round 46, a fresh reader over round 45's answer: 6 findings, 4 defects, after 6, three false sentences on forms written to and one in ordinary work, the self-test's own, with a gap and one of wording. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.part-2.md`; its report is `target/p12746/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — its reader's own runs on `44a79376`, the real hook for a scratch root (`target/p12746/report.md`): `hookrun.py h1.txt` and `h2.txt`, each run form `exit 0` beside its control `exit 2`; `selftest1.log`, `1312 passed, 1 failed, 0 not run` on a host at 70%. Here, this round's rows on that guard: `python3 -I -B target/p12746/land/red/scripts/selftest_host_guard.py case_the_reader_sees_what_the_shell_runs case_killing_by_name` → `310 passed, 6 failed, 0 not run` (`target/p12746/land/red.log`).
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n -F 'job = read_through_runs(read_as(shlex.join(told), which), depth + 1, fresh, which)' 44a79376 -- scripts/host_guard.py` → `2816`, the job read as `which` alone in the run's place; `git grep -n -F 'ShellReader(text, found, itertools.count(), older=bool(which), doubled=BOTH_TO_DEPTH).commands()' 44a79376 -- scripts/host_guard.py` → `2682`, a job never doubling; `git grep -n -F "at = double_quoted(text, at + 1)[1]" 44a79376 -- scripts/host_guard.py` → `2450`, in `older_brace_end`, 3.2's `${…}` nesting nothing in a double-quoted string, and `2601`, `substitution_end`'s probing scan, which this round leaves; `git grep -n -F 'return subprocess.run([*PY, str(GUARD), verb, *flags, *rest]' 44a79376 -- scripts/selftest_host_guard.py` → `129`, a `run` refused for the host's line read as any status.
  - [x] **FIX** — `scripts/host_guard.py`: `job_bashes`, `read_as`'s doubling, `read_through_runs` and `kills_by_name` reading the first run's job as each bash in the run's place too; `older_brace_end` nesting a substitution in a double-quoted string. `scripts/selftest_host_guard.py`: `guard()` marking the host lately unclear on a `run` refused for its line; rows for each. The record's sentences, G46-3 and G46-6 said.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/host_guard.py run --wait 1800 -- python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 1316 passed, 0 failed, 0 not run`, run exit 0, on the final bytes, `cmp`-equal to `target/p12746/land/draft/host_guard.py`, this round's rows among them (`target/p12746/land/selftest-full.log`); its two cases alone → `316 passed, 0 failed, 0 not run` (`target/p12746/land/green.log`); the reader's forms through the real hook (`target/p12746/land/rig46/hookrun.py` on `h1.txt`, `h2.txt`) → each `exit 2`; `rig.py` on `c2.txt`, `c4.txt`, `c5.txt`, `c7.txt`, `c8.txt` → every form a bash runs a kill in refused; its fuzzer seeds 461 and 462 → `0 missed` of 10,000; its corpus → `344 texts in 0.70 s; refused as a kill 0`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; the earlier rounds' rigs on this guard: round 45's forms through the hook → 11 refused, its `fuzz2.py`, `fuzz_brace.py` and `fuzz_gather.py` seeds 463 and 464 → `0 missed` of 12,000; round 44's 122 forms → no kill passed but `kill -0 "$PID"`, a probe of one pid, and its fuzzers seed 465 → `0 missed`; round 43's `fuzz2.py` → `0 missed`; round 42's tables → no row open, its fuzzers `0 missed`; round 41's 49 forms → none passed with a kill run, its 20 ordinary forms passing, its here-document fuzzers → `0 fail open` of 1,200; the probes of rounds 37 to 39 → `wrong 0`; `make focused` → `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1309 (leaf PROGRAM.127.46)`

- ID: `PROGRAM.127.47`
  Status: `done` — `2026-11-14`: round 47, a fresh reader over round 46's answer: 4 findings, 2 defects, after 4, both in round 46's answers, one in the self-test and one a false sentence on a form written to, with two gaps on forms written to, stated. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.part-2.md`; its report is `target/p12747/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — its reader's own runs on `29cdf880` (`target/p12747/report.md`): `g464_rig.py`, `not_run: ['a check that FAILS on a clear host, right after a `run` refused for its turn (the host filled meanwhile)']` on a clear host; `hookrun.py hook_forms.txt`, the two nested runs `exit 0` beside the control's `exit 2`. Here, this round's rows on that guard: `python3 -I -B target/p12747/land/red/scripts/selftest_host_guard.py case_killing_by_name` → `188 passed, 2 failed, 0 not run` (`target/p12747/land/red.log`).
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n -F 'done.returncode == g.UNCLEAR and "wait for the host to have room" in done.stderr:' 29cdf880 -- scripts/selftest_host_guard.py` → `130`, a hint every refusal prints; `git grep -n -F 'if any(kills_by_name(read_as(shlex.join(told), int(older), doubled), root, int(older), False) for older in bashes):' 29cdf880 -- scripts/host_guard.py` → `2886`, and `git grep -n -F 'job = read_through_runs(read_as(shlex.join(told), which, doubled if depth == 0 else BOTH_TO_DEPTH), depth + 1, fresh, which)' 29cdf880 -- scripts/host_guard.py` → `2848`: past the first run, a job read once and nothing in it twice.
  - [x] **FIX** — `scripts/host_guard.py`: `kills_by_name` passing the first run's reading to the runs inside a job read once, `read_through_runs` its doubling, `job_bashes`' words. `scripts/selftest_host_guard.py`: `HOST_LINES`, `guard()` marking the host on its own line alone; rows for the nested runs. The record's sentences, G47-3 and G47-4 stated.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/host_guard.py run --wait 1800 -- python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 1318 passed, 0 failed, 0 not run`, run exit 0, on the final bytes, `cmp`-equal to `target/p12747/land/draft/host_guard.py`, this round's rows among them (`target/p12747/land/selftest-full.log`); its two cases alone → `318 passed, 0 failed, 0 not run` (`target/p12747/land/green.log`); the reader's rigs on this guard (`target/p12747/land/rig47/`): `g464_rig.py` → `last_unclear moved: False`, the deliberate failure a failure, `not_run: []`; `hookrun.py hook_forms.txt` → the control and both nested runs `exit 2`, the two gaps' forms passing as the record states.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; the earlier rounds' rigs on this guard: round 46's case files → `0 missed`, its fuzzer seeds 471 and 472 → `0 missed` of 10,000, its corpus → `344 texts in 0.72 s; refused as a kill 0`; round 45's forms through the hook → 11 refused, its fuzzers seed 473 → `0 missed`; round 44's 122 forms → no kill passed but `kill -0 "$PID"`, a probe of one pid, its fuzzers seed 474 → `0 missed`; round 43's `fuzz2.py` → `0 missed`; round 42's tables → no row open, its fuzzers `0 missed`; round 41's 49 forms → none passed with a kill run, its 20 ordinary forms passing, its here-document fuzzers → `0 fail open`; the probes of rounds 37 to 39 → `wrong 0`; `make focused` → `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1312 (leaf PROGRAM.127.47)`

- ID: `PROGRAM.127.48`
  Status: `done` — `2026-11-14`: round 48, a fresh reader over round 47's answer: 5 findings, 3 defects, after 2, none in ordinary work and none failing open, one in the self-test and two false sentences, with two gaps on forms written to, stated. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.part-2.md`; its report is `target/p12748/report.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — its reader's own runs on `3b6ffee4` (`target/p12748/report.md`): `python3 -I -B target/p12748/work/hostlines.py` → `matched HOST_LINES: ['GiB, over ']`, `not_run: ['a check that is false on a clear host (the host filled meanwhile)']` on a clear host; `python3 -I -B target/p12748/work/runrefused.py` → `run_refused says the host took it: True` of a refusal for its turn; `innermost2.py` → the innermost text read 4, 12, 32, 80, 192 and 448 times.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n -F 'any(line in done.stderr for line in HOST_LINES)' 3b6ffee4 -- scripts/selftest_host_guard.py` → `132`, the whole of what `run` said read, its stops among it; `git grep -n -F 'if job.poll() == g.UNCLEAR:' 3b6ffee4 -- scripts/selftest_host_guard.py` → `154`, any exit 75 the host's.
  - [x] **FIX** — `scripts/selftest_host_guard.py`: `refused_for_the_host`, the "not started" line alone; `run_refused` taking an exit 75 for the host's only on a host lately unclear. The record's sentences, G48-2 and G48-3 said, G48-4 and G48-5 stated.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/host_guard.py run --wait 1800 -- python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 1318 passed, 0 failed, 0 not run`, run exit 0, on the final bytes, `cmp`-equal to `target/p12748/land/draft/selftest_host_guard.py` (`target/p12748/land/selftest-full.log`); the reader's rigs on this self-test (`target/p12748/land/rig48/`): `hostlines.py` → `last_unclear set: False`, the deliberate failure a failure, `not_run: []`; `runrefused.py` → `not_run: []`, a refusal for its turn no longer the host's.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `scripts/host_guard.py` unchanged this round, so the earlier rounds' rigs stand as round 47 ran them; `make focused` → `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action. The book's section says nothing this round moves.
  Commit: `ARCHOGEN-PROGRAM-1316 (leaf PROGRAM.127.48)`

- ID: `PROGRAM.127.49`
  Status: `pending` — round 49, a fresh reader over round 48's answer. It closes on a round that finds no defect, and a form written to go round the guard, or a reader with no line for a form, is a stated limit, not a defect.

- ID: `PROGRAM.128`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.128.md`](../task-history/PROGRAM/PROGRAM.128.md); commit `ARCHOGEN-PROGRAM-1162`

- ID: `PROGRAM.129`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.129.md`](../task-history/PROGRAM/PROGRAM.129.md); commit `ARCHOGEN-PROGRAM-1169`

- ID: `PROGRAM.130`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.130.md`](../task-history/PROGRAM/PROGRAM.130.md); commit `ARCHOGEN-PROGRAM-1175`

- ID: `PROGRAM.131`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.131.md`](../task-history/PROGRAM/PROGRAM.131.md); commit `ARCHOGEN-PROGRAM-1221`

- ID: `PROGRAM.132`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.132.md`](../task-history/PROGRAM/PROGRAM.132.md); commit `ARCHOGEN-PROGRAM-1224`

- ID: `PROGRAM.133`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.133.md`](../task-history/PROGRAM/PROGRAM.133.md); commit `ARCHOGEN-PROGRAM-1242`

- ID: `PROGRAM.134`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.134.md`](../task-history/PROGRAM/PROGRAM.134.md); commit `ARCHOGEN-PROGRAM-1265`

- ID: `PROGRAM.135`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.135.md`](../task-history/PROGRAM/PROGRAM.135.md); commit `ARCHOGEN-PROGRAM-1269`

- ID: `PROGRAM.136`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.136.md`](../task-history/PROGRAM/PROGRAM.136.md); commit `ARCHOGEN-PROGRAM-1272`

- ID: `PROGRAM.137`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.137.md`](../task-history/PROGRAM/PROGRAM.137.md); commit `ARCHOGEN-PROGRAM-1273`

- ID: `PROGRAM.138`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.138.md`](../task-history/PROGRAM/PROGRAM.138.md); commit `ARCHOGEN-PROGRAM-1279`

- ID: `PROGRAM.139`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.139.md`](../task-history/PROGRAM/PROGRAM.139.md); commit `ARCHOGEN-PROGRAM-1287`

- ID: `PROGRAM.140`
  Status: `done` — sealed in [`PROGRAM/PROGRAM.140.md`](../task-history/PROGRAM/PROGRAM.140.md); commit `ARCHOGEN-PROGRAM-1296`

- ID: `PROGRAM.141`
  Status: `done` — `2026-11-14`, the live task trees over their ceiling again, `README-ROUTES` refusing the application inputs' round 17.
  Goal: bring `docs/tasks/` under its ceiling by the project's own sealing, nothing summarised and nothing lost.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the guard's review rounds' closed leaves in `docs/tasks/PROGRAM.md`, `PROGRAM.127.1` to `PROGRAM.127.44` among sixty, never sealed, and three closed in `M4.md` since `PROGRAM.140`. WHERE: `bash scripts/check_doctrines.sh` on the application inputs' round 17, staged → `README-ROUTES: docs/tasks/: 821042 bytes in total, over its ceiling of 819200`.
    - [x] FIX — `bash scripts/check_task_history.sh --seal PROGRAM` and `--seal M4`, on the committed tree, that round's work set aside: 60 closed subtrees and 60 leaves of `PROGRAM`, 3 of `M4`, each with its row in the index and a stub in the tree.
    - [x] ADDRESSED — `task-history: OK (632 sealed file(s), each against its row and its sealing commit`, "the reconstruction is byte for byte"; `docs/tasks/` 632,963 bytes in all before this leaf.
    - [x] NO REGRESSION — `bash scripts/check_doctrines.sh` on the staged tree → exit 0. No code changed.
    - [x] LOCKSTEP — the trees, the index and the Commit Log.
  Verification: the seal's own proof, the gate.
  Commit: `ARCHOGEN-PROGRAM-1302 (leaf PROGRAM.141)`

- ID: `PROGRAM.142`
  Status: `done` — `2026-11-14`, `docs/tasks/M3.md` over its line ceiling, `README-ROUTES` refusing `M3.4.17.3`.
  Goal: bring `docs/tasks/M3.md` under its ceiling by the project's own sealing, nothing summarised and nothing lost.
  Acceptance:
    - [x] ROOT CAUSE — WHY: three leaves closed in `docs/tasks/M3.md` since its last seal, `M3.4.16`, `M3.4.17.1` and `M3.4.17.2`, never sealed. WHERE: `bash scripts/check_doctrines.sh` on `M3.4.17.3`, staged → `README-ROUTES: docs/tasks/: 3011 lines in its largest file, over its ceiling of 3000`.
    - [x] FIX — `bash scripts/check_task_history.sh --seal M3`, on the committed tree, that leaf's work set aside: 3 closed subtrees and 3 leaves, each with its row in the index and a stub in the tree, and 3 log rows in `rows-0050.md`.
    - [x] ADDRESSED — `task-history: docs/tasks/M3.md — sealed 3 subtree(s), 3 leaves: M3.4.17.1 M3.4.17.2 M3.4.16; 3 log row(s) in rows-0050.md; the reconstruction is byte for byte`; `docs/tasks/M3.md` 2,967 lines, from 2,997.
    - [x] NO REGRESSION — `bash scripts/check_doctrines.sh` on the staged tree → exit 0. No code changed.
    - [x] LOCKSTEP — the tree, the index and the Commit Log.
  Verification: the seal's own proof, the gate.
  Commit: `ARCHOGEN-PROGRAM-1307 (leaf PROGRAM.142)`

- ID: `PROGRAM.143`
  Status: `done` — `2026-11-14`, `docs/tasks/M3.md` over its line ceiling again, `README-ROUTES` refusing `M3.4.17.5`.
  Goal: bring `docs/tasks/M3.md` under its ceiling by the project's own sealing, nothing summarised and nothing lost.
  Acceptance:
    - [x] ROOT CAUSE — WHY: two leaves closed in `docs/tasks/M3.md` since `PROGRAM.142`'s seal, `M3.4.17.3` and `M3.4.17.4`, never sealed. WHERE: `bash scripts/check_doctrines.sh` on `M3.4.17.5`, staged → `README-ROUTES: docs/tasks/: 3003 lines in its largest file, over its ceiling of 3000`.
    - [x] FIX — `bash scripts/check_task_history.sh --seal M3`, on the committed tree, that leaf's work set aside: 2 closed subtrees and 2 leaves, each with its row in the index and a stub in the tree, and 2 log rows in `rows-0051.md`.
    - [x] ADDRESSED — `task-history: docs/tasks/M3.md — sealed 2 subtree(s), 2 leaves: M3.4.17.3 M3.4.17.4; 2 log row(s) in rows-0051.md; the reconstruction is byte for byte`; `docs/tasks/M3.md` 2,972 lines, from 2,992.
    - [x] NO REGRESSION — `bash scripts/check_doctrines.sh` on the staged tree → exit 0. No code changed.
    - [x] LOCKSTEP — the tree, the index and the Commit Log.
  Verification: the seal's own proof, the gate.
  Commit: `ARCHOGEN-PROGRAM-1318 (leaf PROGRAM.143)`

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
| `PROGRAM.127` | `ARCHOGEN-PROGRAM-1157 (leaf PROGRAM.127)` | **the host guard**: the machine is shared and holds 24 GB, so a watcher stops this repository's processes before the director's RAM lines, never another project's; `make` and every agent's shell command go through it |
| `PROGRAM.141` | `ARCHOGEN-PROGRAM-1302 (leaf PROGRAM.141)` | **the PROGRAM and M4 trees sealed**: 60 closed leaves of the guard's rounds and others, 3 of `M4`, so `docs/tasks/` is under its ceiling |
| `PROGRAM.127.45` | `ARCHOGEN-PROGRAM-1304 (leaf PROGRAM.127.45)` | **the host guard's review, round 45**: `$$` one token; `$[` a marker and 3.2's `${…}` between double quotes; a run's job read as each bash; 3.2's `$(` gathered between double quotes by its parentheses alone and read as expanded; a `#` glued after `<(…)`; the `-1` sentence and the cost. |
| `PROGRAM.142` | `ARCHOGEN-PROGRAM-1307 (leaf PROGRAM.142)` | **the M3 tree sealed**: `M3.4.16`, `M3.4.17.1` and `M3.4.17.2`, so `docs/tasks/M3.md` is under its line ceiling |
| `PROGRAM.127.46` | `ARCHOGEN-PROGRAM-1309 (leaf PROGRAM.127.46)` | **the host guard's review, round 46**: the first run's job read as each bash in the run's place, what it holds read twice where its markers say; bash 3.2's `${…}` nesting a substitution in a double-quoted string; the self-test's `run` rows on a full host; the `-1` and `$$` sentences. |
| `PROGRAM.127.47` | `ARCHOGEN-PROGRAM-1312 (leaf PROGRAM.127.47)` | **the host guard's review, round 47**: each run inside a job read once read as the first run's is; the self-test marking the host on its own line alone; two gaps on forms written to stated. |
| `PROGRAM.127.48` | `ARCHOGEN-PROGRAM-1316 (leaf PROGRAM.127.48)` | **the host guard's review, round 48**: the self-test reading the host's line in `run`'s "not started" line alone and an exit 75 as the host's only on a host lately unclear; two sentences said as they are; two gaps stated. |
| `PROGRAM.143` | `ARCHOGEN-PROGRAM-1318 (leaf PROGRAM.143)` | **the M3 tree sealed again**: `M3.4.17.3` and `M3.4.17.4`, so `docs/tasks/M3.md` is under its line ceiling |

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
- `2026-11-14`: `PROGRAM.127.41` done: the guard's round 41, two findings answered, two defects failing open in round 40's answer; `PROGRAM.127.42` filed for round 42.
- `2026-11-14`: `PROGRAM.127.42` done: the guard's round 42, nine findings answered, seven defects; `PROGRAM.127.43` filed for round 43.
- `2026-11-14`: `PROGRAM.127.43` done: the guard's round 43, eight findings answered, seven defects; `PROGRAM.127.44` filed for round 44.
- `2026-11-14`: `PROGRAM.127.44` done: the guard's round 44, seven findings answered, five defects; `PROGRAM.127.45` filed for round 45.
- `2026-11-14`: `PROGRAM.127.45` done: the guard's round 45, eight findings answered, six defects, three of its own reruns'; `PROGRAM.127.46` filed for round 46.
- `2026-11-14`: `PROGRAM.142` done: three closed leaves of `M3` sealed, the tree under its line ceiling for `M3.4.17.3`.
- `2026-11-14`: `PROGRAM.127.46` done: the guard's round 46, six findings answered, four defects; `PROGRAM.127.47` filed for round 47.
- `2026-11-14`: `PROGRAM.127.47` done: the guard's round 47, four findings answered, two defects; `PROGRAM.127.48` filed for round 48.
- `2026-11-14`: `PROGRAM.127.48` done: the guard's round 48, five findings answered, three defects, none in ordinary work; `PROGRAM.127.49` filed for round 49.
- `2026-11-14`: `PROGRAM.143` done: two closed leaves of `M3` sealed, the tree under its line ceiling for `M3.4.17.5`.
