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
  Status: `done` — filed `2026-10-10` by `PROGRAM.69`'s review round 7, its P1
  Goal: the history's index is append-only in order as well as in presence, as its record says ("Rows are only ever
  appended").
  Reproduce / issue: `scripts/check_task_history.sh`'s leg 3 compares each committed index's rows with the current
  index's as sets, so a row inserted above committed rows passes; measured by the round's own runner, `exp/e2`, untracked
  and so not durable → *"OK"*, and a
  mutation of `add_rows` that puts new rows first passes every arm. Present at `fea69ad`, before `PROGRAM.69`.
  Direction: leg 3 also requires each committed index's rows to stand, in their order, as a prefix of the current
  table's — or the record and the gate's line say "present and unchanged" rather than "append-only"; an arm either way.
  Built: the first direction, so the record's "Rows are only ever appended" and the gate's "append-only" stay true:
    leg 3 keeps each table's rows as a list, and refuses a committed table whose rows, all present and unchanged, no
    longer stand first and in their order — a row inserted above them or moved — beside its refusal of a row gone or
    changed; an arm moving a committed row above another.
  Acceptance:
    - [x] ROOT CAUSE — WHY: leg 3 compared rows as sets, so their order was not judged; WHERE, `git show b3266f25:scripts/check_task_history.sh`, `current = {t: set(r) for t, r in index.items()}`.
    - [x] FIX — leg 3's `in_order`, lists for sets; the arm.
    - [x] ADDRESSED — the arm before the fix: `bash scripts/check_task_history.sh --self-test`: `170 pass / 1 fail (171 arms)`, the moved row `expected exit 1, got 0`; after it: `171 pass / 0 fail (171 arms)`; the leaf's mutant, `add_rows` putting new rows first, `161 pass / 11 fail (172 arms)`, where it passed every arm before.
    - [x] NO REGRESSION — `bash scripts/check_task_history.sh`: OK over the real history, every committed table a prefix in order; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 256 s.
    - [x] LOCKSTEP — the gate and its arm; the record and the gate's line, already saying "append-only", now held to it.
  Commit: `ARCHOGEN-PROGRAM-0971 (leaf PROGRAM.71)`

- ID: `PROGRAM.72`
  Status: `done` — filed `2026-10-10` by `PROGRAM.69`'s review round 8, its P1; done `2026-10-28`
  Goal: a tree file in a sub-folder of `docs/tasks/` is judged as any tree file is, or is no tree file.
  Reproduce / issue: `scripts/check_task_history.sh`'s leg 6, its foreign-name and named-twice checks read
  `docs/tasks/*.md` alone, while `TASK-ACCEPTANCE` takes an owning leaf from any staged `docs/tasks/**/*.md`
  (`scripts/check_task_acceptance.sh`); the round's probe, its own untracked runner and so not durable: a live
  `M1.12.1.9` in `docs/tasks/m1/M1.md` passes the gate, at `docs/tasks/M1x.md` refused. Present at `fea69ad`.
  Direction: the gate walks `docs/tasks/` whole, `TEMPLATE.md` excepted — or `TASK-ACCEPTANCE` takes owners from
  `docs/tasks/*.md` alone; an arm either way.
  Built: the second way, the gate's: a tree file is `docs/tasks/<TREE>.md`, so `TASK-HISTORY`'s leg 6 refuses a
    Markdown file in any sub-folder of `docs/tasks/`, which none of its legs would judge. `TASK-ACCEPTANCE` is held for
    the scaffold (`.doctrine/scaffold_held.txt`) and stays as it is; the only sub-folder today, `artifacts/`, holds
    scripts.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `HEAD`'s gate with a live `M1.12.1.9` planted in `docs/tasks/m1/M1.md`: `old rc=0`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — leg 6 read the folder's own entries alone:
    `git show HEAD:scripts/check_task_history.sh | grep -c 'for name in sorted(os.listdir(TASKS))'` → `1`.
  - [x] **FIX** — a walk of `docs/tasks/`'s sub-folders in leg 6, its header and two arms; the book's Annex B.
  - [x] **ADDRESSED (verified)** — the patched gate on the same plant: `TASK-HISTORY: docs/tasks/m1/M1.md is a tree
    file in a sub-folder of docs/tasks/`, `1 breach(es)`; `bash scripts/check_task_history.sh` → `exit 0` on the real
    tree; `bash scripts/check_task_history.sh --self-test` → `174 pass / 0 fail (174 arms)`.
  - [x] **NO REGRESSION** — no Rust source changed; `bash scripts/run_self_tests.sh` →
    `exit 0`, `self-tests: OK — 58 self-test(s) passed`; `bash scripts/check_doctrines.sh` → `exit 0`.
  - [x] **LOCKSTEP** — the gate's header, the book's `annex-repository.md`, this leaf and the Commit Log.
  Commit: `ARCHOGEN-PROGRAM-1040 (leaf PROGRAM.72)`

- ID: `PROGRAM.73`
  Status: `done` — filed `2026-10-10` by `PROGRAM.69`'s review round 9, its P1; done `2026-10-28`
  Goal: the history gate names every failure it meets as a breach, never a traceback.
  Reproduce / issue: `scripts/check_task_history.sh`'s top level catches `Unreadable` and `OSError` alone, so any other
  exception in the gate's own run ends in a traceback, exit 1 — git printing bytes that are not UTF-8 for
  `rev-parse --is-shallow-repository`, a `UnicodeDecodeError` at `shallow.decode()`; the round's probe, a fake `git` on
  the path, its own untracked runner and so not durable; the self-test's arm for a seal stopped that way (R9-2) expects
  the traceback today. Present at `fea69ad`.
  Direction: the top level names any exception as a breach, and that arm expects the named breach; an arm for the gate.
  Built: the gate's top level and the census each catch any other exception and name it, `the gate stopped on TYPE:
    message`, exit 1; a seal stopped so has already rolled back and said so. The seal's arm for git output that is not
    UTF-8 now requires the named breach and no traceback, and a new arm requires the same of a plain run.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `HEAD`'s gate with a fake `git` printing `\377` for `--is-shallow-repository`, a plain run:
    `old rc=1`, `grep -c '^Traceback'` → `1`, ending `UnicodeDecodeError: 'utf-8' codec can't decode byte 0xff`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — the top level caught two exception types alone:
    `git show HEAD:scripts/check_task_history.sh | grep -c 'except (Unreadable, OSError) as e:'` → `2`, and nothing after them.
  - [x] **FIX** — `except Exception` beside each, naming the type and message; the seal arm's condition and comment;
    the plain-run arm.
  - [x] **ADDRESSED (verified)** — the patched gate under the same fake `git`: `new rc=1`, `grep -c '^Traceback'` → `0`,
    `TASK-HISTORY: the gate stopped on UnicodeDecodeError: …`; `bash scripts/check_task_history.sh` → `exit 0` on the
    real tree; `bash scripts/check_task_history.sh --self-test` → `172 pass / 0 fail (172 arms)`.
  - [x] **NO REGRESSION** — no Rust source changed; `bash scripts/run_self_tests.sh` →
    `exit 0`, `self-tests: OK — 58 self-test(s) passed`; `bash scripts/check_doctrines.sh` → `exit 0`.
  - [x] **LOCKSTEP** — the gate's comments, this leaf and the Commit Log; no book page describes the traceback.
  Commit: `ARCHOGEN-PROGRAM-1039 (leaf PROGRAM.73)`

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
  Status: `done` — fifteenth daily artifact cleanup; the record's last run `2026-10-22T15:54:45+00:00`, over 24 h old
    at the director's prompt (`2026-10-24`)
  Goal: remove only idle, regenerable compiler caches — the main and the wasm debug incremental directories, the
    owned native-sweep cache being absent — after a physical and native inventory, a successful handoff census and an
    identity recheck; retain every dependency store, final product, cited evidence directory and Git workspace; prove
    each removed directory gone, every other entry under `target/` identity-equal or accounted, the tracked inputs
    restored, the focused tier and `scripts/wasm_build.sh` regenerating; overwrite `docs/ARTIFACT_CLEANUP.md` and add
    the run's section to the repository annex. The commit also seals what closed at `HEAD`: `PROGRAM.100`–`.102` and
    `.104`–`.108`.
    [Full evidence and adapters](../book/src/annex-repository.md#daily-artifact-cleanup-on-2026-10-24).
  Built otherwise in one part: the wasm debug cache, empty on earlier runs, held 462 files after `M3.5.2`'s builds, so
    it was admitted beside the main one, the same kind of regenerable compiler cache, its regeneration proven by the
    documented `scripts/wasm_build.sh`; and the audit, which assumed every surveyed location present, now requires the
    one absent, the native-sweep cache, to be empty.
  Acceptance:
  - [x] ROOT CAUSE: `python3 -I -B target/artifact-cleanup/2026-10-24/census.py`, exit code 0: five locations, physical
    and native find/stat inventories agree; no link, nested Git boundary or cross-volume entry; the handoff census
    `handoff: OK` before the inventory and again immediately before the deletion. No `.log` file in any location; the
    `.bin` files are the caches' own.
  - [x] ADDRESSED: `python3 -I -B target/artifact-cleanup/2026-10-24/audit.py`, exit code 0: only 29847 idle cache
    files / 4245385971 logical bytes removed, 2232 bin, no logs; both directories gone by Python and native stat;
    850169 of 850172 other `target/` entries identical, the three directory rows the strict comparison refused
    accounted (two parents, the census's scratch); 120995 evidence digests agree; 4 of 4 falsifying arms refused.
  - [x] NO REGRESSION: `bash scripts/trust_adequacy.sh --check-inputs`: `restored 1277 regular inputs`; production
    source unchanged; `bash scripts/check_task_history.sh --seal PROGRAM`: `sealed 8 subtree(s), 8 leaves`, the
    reconstruction byte for byte; `bash scripts/wasm_build.sh`: exit code 0, the wasm cache regenerated; `make focused` on this staged tree: exit code 0, `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 260 s — cargo's fingerprints held every crate fresh, so nothing recompiled and the main cache, its directory recreated empty, refills on the next change; the wasm build recompiled and refilled its own.
  Verification: `target/artifact-cleanup/2026-10-24/`; the external policies of §14, §17 and §18 re-read at the prompt,
    each body still hashing as adopted (`README_POLICY.md`'s lines 29–187 `77a1e934…`, the adoption guide
    `8f77fa39…`, `pgen`'s claim verification, 285 lines and 18166 bytes, verbatim in the adopted file), so nothing to
    apply.
  Commit: `ARCHOGEN-PROGRAM-0928 (leaf PROGRAM.109)`

- ID: `PROGRAM.110`
  Status: `done` — the director's verification doctrine 22 (`2026-10-26`) recorded: small checks per slice, full
    verification after the feature set
  Built: `docs/decisions/decision_small-checks-per-slice.md`, the doctrine word for word and what it means here — the
    changed crate's tests and clippy and, for a slice changing what a root compiles, the trust inventory; the hooks and
    `make focused` kept as commit integrity; the full xtask suite, the proofs and their pins, workspace clippy and CI
    deferred — indexed; `M3.5.9`, `M3.5`'s deferred verification, filed pending after its feature leaves.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the doctrine existed only in the session; WHERE, `git grep -n "doctrine 22" -- docs`: no line before this commit.
    - [x] FIX — the record, its index row, `M3.5.9`.
    - [x] ADDRESSED — `bash scripts/check_doctrines.sh`: green, the record indexed and inside the folder's ceiling.
    - [x] NO REGRESSION — a records-only change; no code changed.
    - [x] LOCKSTEP — the decisions index and the tree.
  Verification: the doctrines.
  Commit: `ARCHOGEN-PROGRAM-0952 (leaf PROGRAM.110)`

- ID: `PROGRAM.111`
  Status: `done` — sixteenth daily artifact cleanup; the record's last run `2026-10-23T22:20:24+00:00`, over 24 h old
  Goal: remove only idle, regenerable incremental compiler caches, after the handoff census; keep every dependency
    store, product, evidence directory and Git workspace; a light run under doctrine 22
    (`docs/decisions/decision_small-checks-per-slice.md`) — no identity audit of what stayed.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the cleanup record was over 24 hours old; WHERE, `docs/ARTIFACT_CLEANUP.md` at `3c848944`, its entry `2026-10-23T22:20:24+00:00`.
    - [x] FIX — `python3 -I -B target/artifact-cleanup/2026-10-26/cleanup.py --delete`: `target/debug/incremental`, `target/wasm32-unknown-unknown/debug/incremental`, `target/m352101/measure-target/debug/incremental`, `target/m352111/blocks-target/debug/incremental` removed, no link in any.
    - [x] ADDRESSED — `total files 8854 bytes 1586169011`, 495 bin, 0 log, each directory `gone: true` (`target/artifact-cleanup/2026-10-26/removed.json`); `bash scripts/handoff_census.sh`: `handoff: OK` before.
    - [x] NO REGRESSION — caches alone, regenerated by the next build: the main by `make focused`, the wasm by `scripts/wasm_build.sh`; no tracked file but the record.
    - [x] LOCKSTEP — the record and the tree.
  Verification: the census, the removal report.
  Commit: `ARCHOGEN-PROGRAM-0953 (leaf PROGRAM.111)`

- ID: `PROGRAM.112`
  Status: `done` — doctrine 22's record corrected: what `make focused` runs is not deferred
  Built: `docs/decisions/decision_small-checks-per-slice.md`'s deferred bullet split in two — what `make focused` runs,
    its tier `cargo fmt --all -- --check`, workspace clippy and `cargo test --all`, stays at every commit, a pin a slice
    moves re-pinned by that slice; what it does not run — the integration tier and CI, the sweeps themselves, the
    oracle, audits — deferred to `M3.5.9`; its index row likewise.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the record listed the whole xtask suite, the sweep's pinned premises and workspace clippy as deferred, and `make focused`, mandatory at every commit, runs all three; WHERE, `git show d8153b33:docs/decisions/decision_small-checks-per-slice.md`, its *Deferred* bullet, against `xtask/src/main.rs`'s `focused` tier, `steps: &[FMT, CLIPPY, TESTS]` — found when `M3.5.4.5`'s focused run refused the lockfile pin the record had deferred.
    - [x] FIX — the record's two bullets and its index row.
    - [x] ADDRESSED — `bash scripts/check_doctrines.sh` through the hooks: the record indexed and within the folder's ceiling.
    - [x] NO REGRESSION — a records-only change; no code changed.
    - [x] LOCKSTEP — the record, its index and the tree.
  Verification: the doctrines.
  Commit: `ARCHOGEN-PROGRAM-0959 (leaf PROGRAM.112)`

- ID: `PROGRAM.113`
  Status: `done` — filed `2026-10-26` from the CI run on `d5123578`
  Goal: `scripts/check_task_history.sh`'s self-test arm "a SIGALRM just after the tree's rename" holds on every run of
    the Linux runner, or the reason it cannot is stated and the arm says so.
  Reproduce / issue: the `doctrines` workflow's run `40323084588`, attempt 1, on `ubuntu-latest`: the arm `rc 142, not
    142, or the seal not finished and proven`, the seal's output empty — the process ended by the signal before the
    seal printed, as though `SIGALRM` were not held; the same run's `SIGQUIT`, `SIGUSR1` and `SIGUSR2` arms passed, and
    attempt 2 of the same commit passed whole; the arm passed on this host (`aarch64-apple-darwin`) and in the base's
    last green run. Intermittent, so one run diagnoses nothing.
  Direction: tools first — a Linux reproduction (a container or the runner) running the arm in a loop under load,
    recording the process's threads and signal masks (`/proc/self/status`'s `SigBlk`) at the rename; then the mask, the
    arm or the record fixed by what it shows. Not patched blind.
  Since (`2026-10-26`): the runs on `e089751f` (`40326236217`) and `a809a826` (`40327569374`) failed four and five of
    the signal arms after the tree's rename — `HUP`, `QUIT`, `USR1`, `USR2`, `ALRM`, never `INT` or `TERM` — each with
    its expected exit code, so the failures are steady on the runner, not one run's. The self-test runner keeps only
    `^SELF-TEST` lines, so the arms' evidence never reached the log: the arm's failure line now carries it — whether the
    seal said it sealed, whether `Q.1.md` exists, the seal's last words and the gate's verdict in the proof — as the
    tool the next run reads (`ARCHOGEN-PROGRAM-0976`).
  Closed (`2026-10-26`): `.1` put the arms' evidence on the line the runner keeps, `.2` traced the seal, which showed
    it sound and the arms' own check at fault, `.3` read every output through a here-string; the runs on `34b6abeb` —
    `doctrines` `40337545873`, `rust` `40337545686`, `trust-gate` `40337546070` — each `success`.
  Verification: the three runs on `34b6abeb`, green.
  Commit: `ARCHOGEN-PROGRAM-0978 (leaf PROGRAM.113.3)`, its last child's

- ID: `PROGRAM.113.1`
  Status: `done` — the signal arms' failure line carries the seal's evidence, the one line the runner keeps
  Acceptance:
    - [x] ROOT CAUSE — WHY: the self-test runner keeps a failing gate's `^SELF-TEST` lines alone, and the arm printed its evidence on the lines after, so the runner's log said only that the arms failed; WHERE, `git show a809a826:scripts/run_self_tests.sh`, its line 110, `grep -E '^SELF-TEST|self-test' | head -8`.
    - [x] FIX — the arm's failure line names whether the seal said it sealed, whether `Q.1.md` exists, the seal's last words and the gate's verdict in the proof.
    - [x] ADDRESSED — the `SIGALRM` arm forced to fail, its wanted code set to `1` and restored after: its line read `sealed line yes, Q.1.md yes; output: …; gate: task-history: OK …`; `bash scripts/check_task_history.sh --self-test`: `171 pass / 0 fail (171 arms)`.
    - [x] NO REGRESSION — `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 337 s; the gate's behaviour unchanged, the arm's message alone longer.
    - [x] LOCKSTEP — the arm and `PROGRAM.113`'s record.
  Commit: `ARCHOGEN-PROGRAM-0976 (leaf PROGRAM.113.1)`

- ID: `PROGRAM.113.2`
  Status: `done` — the signal arms trace the seal: the signal sent, each mask change, each outcome printed
  Acceptance:
    - [x] ROOT CAUSE — WHY: `.1`'s evidence from the run on `d7fa9c69` — `sealed line no, Q.1.md yes`, the gate's proof OK — fits both a line printed and lost and a process ended before it printed, and one run cannot tell them apart; WHERE, `git show d7fa9c69:scripts/check_task_history.sh`, the arm's failure line, no order of events.
    - [x] FIX — the test hook writes, unbuffered and in order, to a trace file the arm names: `fire N` as the signal is sent, `mask HOW COUNT` at each mask change, `print …` before each `task-history:` outcome; the arm's failure line quotes the trace and the output's first lines.
    - [x] ADDRESSED — the `SIGALRM` arm forced to fail, its wanted code `1` and restored after: `trace: mask 1 23|fire 14|print task-history: docs/tasks/Q.md — sealed 1 subtree|mask 3 0|`, the designed order on this host; `bash scripts/check_task_history.sh --self-test`: `171 pass / 0 fail (171 arms)`.
    - [x] NO REGRESSION — test-only code, the gate's own unchanged; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 308 s.
    - [x] LOCKSTEP — the hook, the arm and `PROGRAM.113`'s record.
  Commit: `ARCHOGEN-PROGRAM-0977 (leaf PROGRAM.113.2)`

- ID: `PROGRAM.113.3`
  Status: `done` — the arms read their output without a pipe, so `pipefail` cannot turn a match into a failure
  Acceptance:
    - [x] ROOT CAUSE — WHY: `.2`'s trace on the runner, run `40333831327` on `80719f25`, read `mask 0 54|fire 1|print task-history: docs/tasks/Q.md — sealed 1 subtree|mask 2 0|`, the output's first line the seal's own — the seal held the signal, said its outcome and was proven; the arm failed in its own check, `printf '%s' "$out" | grep -qE '^task-history: … sealed '` under `set -o pipefail`: GNU grep's `-q` exits at its first match while `printf` is still writing an output the size of the gate's Python, which bash's notice of a signalled job repeats (`git show d7fa9c69:scripts/check_task_history.sh` holds 55174 bytes of it, the base `8cdac117` 35958), so `printf` meets a closed pipe and the pipeline fails though `grep` matched — on Linux's pipe, never on this host's larger one or BSD grep, which reads to the end.
    - [x] FIX — every `printf '%s' "$out" | grep -q…` in the script, fifty on forty-five lines, made `grep -q… <<< "$out"`: no pipeline, so no `pipefail`.
    - [x] ADDRESSED — the mechanism alone, GNU grep under `pipefail` on a matching first line: through the pipe `20/20 pass` at 70 kB and `0/20 pass` at 300 kB; through the here-string `20/20 pass` at 1 MB; the arms with each signalled run's output padded by 300 kB, under GNU grep, before the fix `bash scripts/check_task_history.sh --self-test`: `159 pass / 12 fail (171 arms)`, after it `171 pass / 0 fail (171 arms)`; unpadded, with this host's grep and with GNU grep, `171 pass / 0 fail (171 arms)`.
    - [x] NO REGRESSION — `bash scripts/check_task_history.sh`: OK; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 358 s.
    - [x] LOCKSTEP — the script; `PROGRAM.114` filed for the idiom elsewhere.
  Commit: `ARCHOGEN-PROGRAM-0978 (leaf PROGRAM.113.3)`

- ID: `PROGRAM.114`
  Status: `done` — filed `2026-10-26` by `PROGRAM.113.3`
  Goal: no gate script reads a captured output through `printf … | grep -q` under `pipefail`, where an output larger
    than a pipe turns a match into a failure on Linux.
  Reproduce / issue: the census of `scripts/*.sh` running under `pipefail` finds the idiom 78 times outside
    `check_task_history.sh`, most in self-test arms — `check_frozen_evaluation.sh` 7, `check_task_acceptance.sh` 6,
    `check_feedback_register.sh` 5, and more; each latent, failing only once its output outgrows a pipe, as
    `PROGRAM.113.3` measured.
  Direction: each made a here-string, and a doctrine leg refusing the idiom in a `pipefail` script, its RED arm the
    pipe the padded experiment broke.
  Built: every such read in this repository's `pipefail` scripts made a here-string — seventy-six, in forty files,
    one by hand where its patterns come from a process substitution — and `PIPE-READS`, `scripts/check_pipe_reads.sh`,
    refusing the form in any tracked shell script that sets `pipefail`, a comment not code, registered in
    `scripts/check_doctrines.project.sh`, described in the book's Annex B. The scaffold's eleven sites —
    `check_task_acceptance.sh` six, `check_lesson_promotion.sh` two, `check_routing_evidence.sh` two,
    `check_no_background_jobs.sh` one — are reported, not refused, as `SCRATCH-LOCALITY` reports its own; the change
    they need, for whoever owns the scaffold, never sent there by an archogen agent: each `printf '%s' "$x" | grep -q…`
    or `echo "$x" | grep -q…` made `grep -q… <<< "$x"`. `DOCTRINE_ENFORCEMENT.md` is at its ceiling — twenty-six bytes
    free — so its row waits on that table's compaction, `PROGRAM.115`.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the idiom `PROGRAM.113.3` measured failing a matched check on Linux stood in other gates' self-tests, latent until an output outgrows a pipe; WHERE, `git show 34b6abeb:scripts/check_frozen_evaluation.sh` and the census's forty-four files.
    - [x] FIX — the reads converted; the gate, its registration and the book.
    - [x] ADDRESSED — `bash scripts/check_pipe_reads.sh`: `pipe-reads: OK (74 pipefail script(s) scanned; 11 site(s) in scaffold-owned files reported, …)`; `bash scripts/check_pipe_reads.sh --self-test`: `pipe-reads self-test: 8 pass / 0 fail (8 arms)`; one owned site put back refused, `scripts/check_decision_index.sh:69`; every touched gate's self-test under GNU grep passing, `scripts/run_self_tests.sh`'s two real-tree arms again after the index and the ceiling were answered.
    - [x] NO REGRESSION — `bash scripts/check_doctrines.sh`: every doctrine but `TASK-ACCEPTANCE`, which this leaf answers; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 268 s.
    - [x] LOCKSTEP — the scripts, the gate's registration, the book; the enforcement table's row `PROGRAM.115`'s.
  Commit: `ARCHOGEN-PROGRAM-0980 (leaf PROGRAM.114)`

- ID: `PROGRAM.115`
  Status: `done` — filed `2026-10-26` by `PROGRAM.114`
  Goal: `DOCTRINE_ENFORCEMENT.md` compacted under its ceiling with room for `PIPE-READS`'s row, or its ceiling raised
    by a decision record.
  Reproduce / issue: `bash scripts/check_readme_routes.sh` with the row added: `DOCTRINE_ENFORCEMENT.md: 37469 bytes,
    over its ceiling of 36864`; without it the table holds 36838, twenty-six bytes free. The table is scaffold-owned,
    so a compaction is the scaffold's change too, written down for its owner.
  Corrected by `PROGRAM.116` (`2026-10-26`): the table is archogen's — on the updater's NEUTRAL list, but changed here,
    and the director's ruling of `2026-09-30` (findings §10) holds only what archogen never changed — so a compaction is
    this repository's alone, nothing owed upstream; `PROGRAM.116`'s row edit left it at 36836 bytes.
  Done `2026-10-28`. `M2.7.6.5` (`ARCHOGEN-M2-1033`) took five clauses of history out of other rows to fit
    `CODE-OWNERS`'s; this leaf shortens `BOOK-ANCHORS`' row to its rule, the rationale it repeated kept in
    `scripts/check_book_anchors.sh`'s header, and adds `PIPE-READS`'s row after `NO-SUBPROCESS`'s, the registry's place.
  Verification: `bash scripts/check_readme_routes.sh` → `exit 0`, nothing on `DOCTRINE_ENFORCEMENT.md`, at 36851 of
    its 36864 bytes; `bash scripts/check_table_arity.sh` → `exit 0`. Thirteen bytes stay free, so the next row comes
    with its own compaction or a ceiling the director rules.
  Commit: `ARCHOGEN-PROGRAM-1036 (leaf PROGRAM.115)`

- ID: `PROGRAM.116`
  Status: `done` — P2, filed and fixed `2026-10-26`: what is held for the scaffold read from the director's ruling, not
    from the updater's NEUTRAL array
  Reproduce / issue: the ruling of `2026-09-30` (findings §10, sealed in
    `docs/decision-history/decision_findings-for-director-review/10.md`): "only [template] inherited script you haven't
    modified. Those you modified are yours now" — seventeen files measured, `check_task_acceptance.sh` named archogen's.
    `SCRATCH-LOCALITY` (`PROGRAM.29`, `2026-09-29`, a day before it) takes every file on `scripts/update_scaffold.sh`'s
    NEUTRAL array as the scaffold's and was never brought to it; `PIPE-READS` (`PROGRAM.114`, this session) copied that
    rule. So `check_task_acceptance.sh`'s two off-volume `mktemp` calls and six pipe reads under `pipefail` — the form
    `PROGRAM.113` found failing on CI — were reported for upstream instead of fixed. Found while reading the ruling for
    `M3.5.1.4`'s `DOCTRINE_ENFORCEMENT.md` edit; `PROGRAM.115`'s premise, that table scaffold-owned, falls with it.
  Built: `.doctrine/scaffold_held.txt`, the ruling's held files with its citation — sixteen, `.githooks/pre-commit`
    having left the hold when `M2.7.4.4` (`e2b034dc`, `2026-10-03`) changed it — read by both gates in place of the
    NEUTRAL array, each gaining an arm: a NEUTRAL file archogen changed is refused. `check_task_acceptance.sh`'s eight
    sites fixed: its scratch under `target/doctrine_scratch/`, its self-test's reads through here-strings. The scratch
    record, `DOCTRINE_ENFORCEMENT.md`'s `SCRATCH-LOCALITY` row (two bytes shorter), `.doctrine/README.md`, `TOOLBOX.md`
    and the book's repository annex and ledger say so; `PROGRAM.115` corrected. Upstream is owed five pipe reads and
    two scratch sites, all in held files; `PROGRAM.114`'s list of eleven stands for those five.
  Acceptance:
    - [x] ROOT CAUSE — WHY: both gates read the updater's NEUTRAL array as the scaffold's files, which the ruling of `2026-09-30` narrowed to those archogen never changed; WHERE, `git show 99d32dd1:scripts/check_scratch_locality.sh`, `scaffold_owned()` reading `NEUTRAL=(`.
    - [x] FIX — the held list; both gates read it; `check_task_acceptance.sh`'s eight sites; the records and the book.
    - [x] ADDRESSED — `bash scripts/check_scratch_locality.sh --self-test`: `scratch-locality self-test: 17 pass / 0 fail (17 arms)`; `bash scripts/check_pipe_reads.sh --self-test`: `pipe-reads self-test: 9 pass / 0 fail (9 arms)`; `bash scripts/check_task_acceptance.sh --self-test`: `task-acceptance self-test: 10 pass / 0 fail`; `bash scripts/check_pipe_reads.sh`: `pipe-reads: OK (74 pipefail script(s) scanned; 5 site(s) in scaffold-owned files reported, …)`. Falsified: before the fix `SCRATCH-LOCALITY` refused `check_task_acceptance.sh:374` under the held list, and a pipe read put back at `check_task_acceptance.sh:306` is refused by `PIPE-READS`.
    - [x] NO REGRESSION — `bash -n` of the three scripts `exit 0`; `bash scripts/check_readme_routes.sh`: `readme-routes: OK (25 destination(s) governed)`; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 258 s.
    - [x] LOCKSTEP — the gates, the list, the records, `DOCTRINE_ENFORCEMENT.md`, the book.
  Commit: `ARCHOGEN-PROGRAM-0989 (leaf PROGRAM.116)`

- ID: `PROGRAM.117`
  Status: `done` — filed `2026-10-27` on the director's ruling of that day; closed `2026-10-28` with `.3`, its three
    children done
  Goal: the record folders' ceilings bound what is read, not the corpus, so their growth never stops work.
  Ruling (`2026-10-27`, the director, reopening the one-time raise of `decision_decisions-folder-ceiling.md`): "I do
    not understand why there are even ceilings on those directories ... What's the best solution, to you, to handle
    nicely, elegantly those ceiling. I agree their size shall not blow up, shall somehow be contained but this shall
    not hinder progress. Propose a sota, production-grade solution."
  Reproduce / issue: `docs/specs/`' total was raised by `PROGRAM.56`, `.79`, `.102` and `.108`, four times in
    eighteen days; at `220eb078`, `git ls-tree -r -l HEAD -- docs/decisions docs/specs`: 392564 of 393216 and 649155
    of 655360. No reader's context loads a whole folder, so a total bounds no read; the per-file caps do.
  Children: `.1` the ceilings, the check and the record; `.2` the move tool the lifecycle needs; `.3` the index that
    still binds as one file.

- ID: `PROGRAM.117.1`
  Status: `done` — `2026-10-27`
  Goal: `README-ROUTES` reads a directory row's Files cell `fan-out N` — no directory it governs lists more than N
    entries, its tracked files (archive stubs aside) and sub-folders — and its Total bytes cell `guard N`, a runaway
    guard, with a `--report` of each folder's totals and growth; `docs/decisions/`, `docs/specs/` and `docs/reviews/`
    take both, under a new decision record that supersedes the three ceiling records; the policy, the inventory and
    the book follow.
  Built: the check's two cells (`fanned`, `guarded`) and its `--report` (`scripts/check_readme_routes.sh`); a guard
    refuses a total over it, three quarters of it unless the cell names an open leaf asking for a ruling or a tracked
    decision record that kept it, and a commit growing the folder in the index over `HEAD` past a sixty-fourth of it
    unless `README_ROUTES_GROWTH_LEAF` names a leaf open at `HEAD` or new in the index; every ceiling a whole number,
    compared fail-closed; `### Ceilings a decision fixes` compares a cell with its maximum by kind, a debt or a
    doctrine refused, a directory's maximum binding the rows beneath it through the nearest fixed ancestor; `docs/decisions/decision_ceilings-bound-reads.md`, quoting the
    ruling and the `nygard-adr` ledger entry; the three ceiling records marked superseded; the three rows at
    `fan-out 64` and `guard 16777216`.
  Review (`2026-10-27`, rounds by independent contexts): round 1 on the first draft, five defects and ten remarks;
    round 2 on the answers, two defects and thirteen remarks; round 3, three defects and four remarks; round 4, two
    defects and six remarks; round 5, no defect and three remarks, closing the review; all answered here.
    - D1, a number past 64 bits or with a leading zero failing a comparison open: `whole()`, every comparison
      `[ a -le b ] || note`; arms for each.
    - D2, a decision-fixed cell turned into a debt or a doctrine escaping its maximum: refused; arms.
    - D3, the notice at three quarters printed nowhere, `scripts/check_doctrines.sh` being held and silent on a pass:
      a refusal unless an open leaf asks for a ruling.
    - D4, the record's index split refused by `DECISION-INDEX`: the record states the single index binds; `.3` filed.
    - D5, "only a mistake reaches the guard", false by the record's own numbers, and a 6.3 MB dump passing: reworded,
      and the per-commit step added, sized against the largest honest growth since `6d61f65`, 38818 bytes
      (`efedf6d6`).
    - Remarks taken: the numbers (26 times, fifteen days, "no reader's context"); `docs/reviews/`' stubs and flat
      layout stated; `docs/specs/` has no index of its own; `knowledge-map/subsystems.md` for a new sub-folder;
      superseded records stay in place, `.2`'s goal narrowed; the inventory's widest-listing label; the two stale
      script comments; the self-test gaps.
    - Round 2, defect 1, the step measuring the disk while the commit holds the index — growth staged and deleted
      from the disk passed: `index_bytes()`, the index's blob sizes against `HEAD`'s; an arm.
    - Round 2, defect 2, a partition's row lifting a fan-out a decision fixes for its parent: a maximum fixed for a
      directory bounds every registered row beneath it unless the table fixes that row's own; three arms.
    - Round 2's remarks taken: `README_ROUTES_GROWTH_LEAF` open at `HEAD` or new; an empty ruling clause refused;
      `guard N, ruled by <record>` for a ruling that keeps a guard; the report's date the three-quarter refusal; the
      step's honest limits in the record and the book; the documented batch of 186617 bytes (`PROGRAM.43`); 131072
      for a review history; the index figures from `220eb078`; one reason for a malformed number under a maximum;
      the inventory and the failing-first figures re-measured on the final tree.
    - Round 3, defect 1, a `ruled by` clause satisfied by any pathspec — `.`, `*`, `README.md`: a decision record
      under `docs/decisions/` or `docs/specs/`, by its literal path; arms for each.
    - Round 3, defect 2, a leaf "new in the commit" read from the disk: `leaf_status_in`, at `HEAD` or in the index;
      an arm with a leaf on the disk alone.
    - Round 3, defect 3, a directory's fixed maximum reaching file rows and doctrine cells beneath it that could never
      comply: only directory rows, through the nearest fixed ancestor; arms for a file row and a nested partition
      (narrowed by round 4: a file row is bound in the per-file columns, and a doctrine cell beneath is refused).
    - Round 3's remarks taken: the book's and the report arm's dates the three-quarter refusal, pinned; `index_bytes`
      refusing an object it cannot size; the header rewrapped, `control`'s unused locals gone; this text. Its
      remark on path quoting in the on-disk measures predates this change, filed as `PROGRAM.118`.
    - Round 4, defect 1, file rows freed in every column, so a file row lifted a decided per-file maximum (a row for
      a file of `docs/task-history/` above `decision_task-history-line-ceiling.md`'s 3072): a file row is skipped in
      Files and Total bytes alone; three arms.
    - Round 4, defect 2, the header and `within_fixed`'s message stating round 2's rule: both rewritten.
    - Round 4's remarks taken: a row beneath a fixed column takes its kind, stated and pinned by an arm; an index or
      the template is no ruling; arms reaching the literal-path test (`docs/decisions/../../README.md`, a record on
      the disk alone) and the `unreadable` refusal; the table's rows matched with or without backticks.
    - Round 5's remarks taken: `within_fixed`'s messages read in order; an arm for a file row's doctrine cell under a
      fixed per-file column; a ruling clause names a record whose type is `decision`, the template and a reference
      note refused, and the header states what the check cannot tell.
    - Fence: the round-1 reviewer ran `git log -G'655360|589824|524288|458752|393216' -- README_POLICY.md`, which searched that
      file's diffs in commits older than `6d61f65` and printed commit subjects alone, and disclosed it. No path under
      `docs/evaluation/` was searched and no case text was printed, so no case was read and nothing joins the
      evaluation's *Exposures*; a brief's fence now names `log -G` and `log -S` with `-p`.
  Acceptance:
    - [x] ROOT CAUSE — WHY: `README_POLICY.md` bounded the record folders' totals, which no read loads, so the product's growth met a wall raised four times in eighteen days; WHERE, `git show 220eb078:README_POLICY.md`, the rows of `docs/decisions/`, `docs/specs/` and `docs/reviews/` and their maxima; at `220eb078`, `git ls-tree -r -l HEAD -- docs/specs`: 649155 of 655360.
    - [x] FIX — the check's cells, step and report; the policy's rows, note and maxima; the record; the three records superseded; the inventory, the enforcement table, `TOOLBOX.md`, the book's repository annex and glossary, the ledger, `docs/figures.md`; the review's answers above.
    - [x] ADDRESSED — `bash scripts/check_readme_routes.sh --self-test`: `readme-routes self-test: 113 pass / 0 fail (113 arms)`; `bash scripts/check_readme_routes.sh`: `readme-routes: OK (25 destination(s) governed)`. Failing first, this staged tree under `220eb078`'s policy: `docs/decisions/: 407037 bytes in total, over its ceiling of 393216`; under the new rows before the record existed: six breaches, `fixed by docs/decisions/decision_ceilings-bound-reads.md, which does not exist`.
    - [x] NO REGRESSION — `bash -n scripts/check_readme_routes.sh` exit 0; `bash scripts/check_doctrines.sh` passes; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 300 s
    - [x] LOCKSTEP — the check, the policy, the record, the inventory, the enforcement table, the toolbox, the book, the snapshots, two script headers; `eadl-1-a-plan-names-engine-adapters.md` flipped to `applied`, `M3.2.5`'s note.
  Commit: `ARCHOGEN-PROGRAM-1020 (leaf PROGRAM.117.1)`

- ID: `PROGRAM.117.2`
  Status: `done` — `2026-10-27`
  Goal: one scripted, verified step moves records — an accepted design from `docs/decisions/` to
    `docs/specs/<subject>/`, or a subject's records into a sub-folder of their own — their relative links rewritten,
    every path-form citation in a live file repointed, sealed history and scaffold-held files left and reported, the
    index rows updated, a new sub-folder linked from `knowledge-map/subsystems.md`; a batch past a folder's step,
    262144 bytes, names its leaf in `README_ROUTES_GROWTH_LEAF`; a superseded record stays in place
    (`decision_ceilings-bound-reads.md`).
  Built: `scripts/move_record.sh [--dry-run] <record> <destination>`. It refuses a record that is not a tracked `.md`
    under the two folders, an index or the template, a destination outside them or existing, and — for a move —
    unstaged changes. It `git mv`s the record, rewrites its own relative links, and repoints every written-out path
    and every relative markdown link that resolved to it in a live file. It leaves and lists a `<revision>:<path>`,
    the four archives, `CHANGELOG.md` and `DEV_NOTES.md`, `annex-trust-proofs.md`'s verbatim proofs and the held
    files; it never lists or opens `docs/evaluation/`. It stages the move only when no live file names the old path,
    runs `DECISION-INDEX`, and names a new folder for the knowledge map and a rewritten Rust source or trust root for
    `cargo xtask trust-inventory`. `TOOLBOX.md`'s README-routes row, the record and the book's repository annex say so.
  Acceptance:
    - [x] ROOT CAUSE — WHY: a record's move was a census by reading, its citations found and judged by hand (`PROGRAM.43`, `.56`, `.102`); WHERE, `PROGRAM.102`'s move, `git show --stat --format= efedf6d6`: `20 files changed, 94 insertions(+), 39 deletions(-)`, each edit chosen by reading, its FIX "every path-form citation repointed … bare names left … sealed history untouched".
    - [x] FIX — `scripts/move_record.sh`; `TOOLBOX.md`; the record's lifecycle and how-to; the book's repository annex.
    - [x] ADDRESSED — `bash scripts/move_record.sh --self-test`: `move-record self-test: 24 pass / 0 fail (24 arms)`, the sealed fixture unreadable throughout. On this tree, `bash scripts/move_record.sh --dry-run docs/decisions/decision_engine-api.md docs/specs/api/`: `move-record: planned, nothing changed (--dry-run)`, 12 citations and 2 links rewritten, 9 left as verbatim proofs — `git grep -c docs/decisions/decision_engine-api.md` counts 21 = 12 + 9.
    - [x] NO REGRESSION — `bash -n scripts/move_record.sh` exit 0; `bash scripts/check_doctrines.sh` passes; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 318 s
    - [x] LOCKSTEP — the tool, `TOOLBOX.md`, the record, the book, the snapshots.
  Commit: `ARCHOGEN-PROGRAM-1022 (leaf PROGRAM.117.2)`

- ID: `PROGRAM.117.3`
  Status: `done` — filed `2026-10-27` by `PROGRAM.117.1`'s review (D4); done `2026-10-28`
  Goal: an index never binds the folder it lists: `DECISION-INDEX` accepts a record linked from its sub-folder's own
    `INDEX.md` when the parent index links that index, and the specifications are indexed — two records of
    `docs/specs/trust/` are in no index, a third only in the reviews' index.
  Reproduce / issue: `scripts/check_decision_index.sh` requires every record under `docs/decisions/` to be linked from
    `docs/decisions/INDEX.md` itself; at `220eb078`, 18185 of its 98304 bytes, about 140 a day since `d00c3dee`
    (`2026-10-12`, 16118), so about a year and a half before it binds.
  Built: `DECISION-INDEX` walks the root index and every `INDEX.md` under `docs/decisions/` or `docs/specs/` that an
    index it already reached links, each link resolved against its own index's folder; a record is found by that path,
    and an index nothing reaches is refused, since what it lists would be listed nowhere. An index outside the two
    folders, the reviews', is never read as one. Its population is both folders, so the specifications are held too;
    the three it then found are indexed in `docs/decisions/INDEX.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the widened gate on the tree before the rows: `bash scripts/check_decision_index.sh` →
    `exit 1`, `3 breach(es)`, naming `docs/specs/trust/compiler-provenance-contract.md`,
    `docs/specs/trust/decision_trust-role-programs.md` and `docs/specs/trust/flag-normalization-domain.md`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — the gate read one folder and one index, every record's path taken relative to it
    and compared with the root index's links alone:
    `git show HEAD:scripts/check_decision_index.sh | grep -c 'DIR = "docs/decisions"'` → `1`.
  - [x] **FIX** — the walk over sub-indexes and the two folders in `scripts/check_decision_index.sh`, its header,
    seven new arms; the three rows; the registry's text, `DOCTRINE_ENFORCEMENT.md`'s row, the ceilings record's owed
    clause marked discharged, and the book's Annex B.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_decision_index.sh` → `exit 0`, `decision-index: OK (40
    record(s), 0 in partitions none, and 16 specification(s), each linked by its path from 1 index(es); every link
    resolves)`; `bash scripts/check_decision_index.sh --self-test` → `12 pass / 0 fail (12 arms)`.
  - [x] **NO REGRESSION** — no Rust source changed; `bash scripts/run_self_tests.sh` →
    `exit 0`, `self-tests: OK — 58 self-test(s) passed`; `bash scripts/check_doctrines.sh` → `exit 0`.
  - [x] **LOCKSTEP** — the registry, the enforcement table, `docs/decisions/decision_ceilings-bound-reads.md`, the
    book's `annex-repository.md`, this leaf and the Commit Log.
  Commit: `ARCHOGEN-PROGRAM-1037 (leaf PROGRAM.117.3)`

- ID: `PROGRAM.118`
  Status: `done` — P3, filed `2026-10-27` by `PROGRAM.117.1`'s third review round, fixed the same day
  Goal: `README-ROUTES`' on-disk measures read every tracked path whole: a NUL-separated `git ls-files -z`, never its
    quoted output or a word-split list.
  Reproduce / issue: `git ls-files` quotes a path holding a tab, a quote or a non-ASCII byte, so `wc` fails on it and
    the file drops out of its row's measures; `longest $own` splits a path holding a space, so awk cannot open it and
    the row's longest line reads 0. The guard's total inherits it; the step, measured from the index, does not.
  Built: the directory branch and `--report` read `git ls-files -z` into an array; `longest` takes the row's files as
    a quoted array; a tracked path holding a newline is refused, named by `%q`; the blob-size sums' book filter takes
    the leading quote `git` writes before an unusual name.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the loops read `git ls-files`' quoted, newline-separated output and passed the row's files to awk word-split; WHERE, `git grep -n "longest \$own" HEAD -- scripts/check_readme_routes.sh`: the unquoted call, measured by the round's reviewer as awk's "can't open file docs/tasks/a".
    - [x] FIX — the NUL-separated reads, the quoted array, the newline refusal, the quote-tolerant filters.
    - [x] ADDRESSED — `bash scripts/check_readme_routes.sh --self-test`: `readme-routes self-test: 116 pass / 0 fail (116 arms)`, three new: a name with a space measured (`250 bytes on its longest line, over its ceiling of 200`), one past ASCII (`2100 bytes in its largest file`), one with a newline refused.
    - [x] NO REGRESSION — `bash -n scripts/check_readme_routes.sh` exit 0; `bash scripts/check_readme_routes.sh`: `readme-routes: OK (25 destination(s) governed)`; `bash scripts/check_doctrines.sh` passes; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 376 s
    - [x] LOCKSTEP — the check alone; its header's honest limits unchanged, the defect being no limit but a fault.
  Commit: `ARCHOGEN-PROGRAM-1028 (leaf PROGRAM.118)`

- ID: `PROGRAM.119`
  Status: `done` — seventeenth daily artifact cleanup; the record's last run `2026-10-26T08:56:05+00:00`, over 24 h
    old at `2026-10-27T10:29Z`
  Goal: remove only idle, regenerable incremental compiler caches, after the handoff census; keep every dependency
    store, product, evidence directory and Git workspace; a light run under doctrine 22
    (`docs/decisions/decision_small-checks-per-slice.md`) — no identity audit of what stayed.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the cleanup record was over 24 hours old; WHERE, `docs/ARTIFACT_CLEANUP.md` at `bce7fc94`, its entry `2026-10-26T08:56:05+00:00`.
    - [x] FIX — `python3 -I -B target/artifact-cleanup/2026-10-27/cleanup.py --delete`: `target/debug/incremental` and `target/wasm32-unknown-unknown/debug/incremental` removed, no link in either.
    - [x] ADDRESSED — `total files 52855 bytes 5496697254`, 3589 bin, 0 log, each directory `gone: true` (`target/artifact-cleanup/2026-10-27/removed.json`); `bash scripts/handoff_census.sh`: `handoff: OK` before.
    - [x] NO REGRESSION — caches alone, regenerated by the next build: the main by `make focused`, the wasm by `scripts/wasm_build.sh`; no tracked file but the record.
    - [x] LOCKSTEP — the record and the tree.
  Verification: the census, the removal report.
  Commit: `ARCHOGEN-PROGRAM-1021 (leaf PROGRAM.119)`

- ID: `PROGRAM.120`
  Status: `done` — eighteenth daily artifact cleanup; the record's last run `2026-10-27T10:30:55+00:00`, over 24 h
    old at `2026-10-28T10:36Z`
  Goal: remove only idle, regenerable incremental compiler caches, after the handoff census; keep every dependency
    store, product, evidence directory and Git workspace; a light run under doctrine 22
    (`docs/decisions/decision_small-checks-per-slice.md`) — no identity audit of what stayed.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the cleanup record was over 24 hours old; WHERE, `docs/ARTIFACT_CLEANUP.md` at `393796ad`, its entry `2026-10-27T10:30:55+00:00`.
    - [x] FIX — `python3 -I -B target/artifact-cleanup/2026-10-28/cleanup.py --delete`: `target/debug/incremental` and `target/wasm32-unknown-unknown/debug/incremental` removed, no link in either.
    - [x] ADDRESSED — `total files 15647 bytes 1926076171`, 1068 bin, 0 log, each directory `gone: true` (`target/artifact-cleanup/2026-10-28/removed.json`); `bash scripts/handoff_census.sh`: `handoff: OK` before.
    - [x] NO REGRESSION — caches alone, regenerated by the next build: the main by `make focused`, the wasm by `scripts/wasm_build.sh`; no tracked file but the record.
    - [x] LOCKSTEP — the record and the tree.
  Verification: the census, the removal report.
  Commit: `ARCHOGEN-PROGRAM-1068 (leaf PROGRAM.120)`

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

The PROGRAM.34 row waits on the director's yes. The one other open leaf, `PROGRAM.78`, is blocked until the
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
| `2026-09-28` | `PROGRAM.19` (second run) | the trigger read off `docs/ARTIFACT_CLEANUP.md` rather than assumed; a full inventory taken before any deletion; a residue census over every deleted path; `git grep` for each retention candidate's consumers; the focused tier re-run **cold**, with the scratch it consumes already deleted | ≈18 MB released: `target` 873 MB → 855 MB, `.bin` files 715 → 693, all 7 deleted paths `gone`; **2.2 GB retained on evidence** — `pgen-generated-before-remeasure` is read by `LS-004`'s `remeasure.sh:225-227`, and `target/debug/incremental` holds at most four generations per crate across 121 directories, which is cargo's retention and not residue; `make focused` → `passed — 3 / 0 / 0` cold; 13 doctrines green; `git status --porcelain` empty after the deletions |
| `2026-09-29` | `PROGRAM.19` (third run) | the trigger read off the record's own commit timestamp rather than its date-only line; a full inventory before any deletion; a residue census over the deleted paths; the unexpected item **identified against a read-only sibling repository** before deletion; the focused tier, the whole suite and the doctrine gate all re-run **cold** with the scratch they consume already deleted | ≈11 MB released: `target` 957 MB → 949 MB, then 965 MB once verification recreated the scratch; all five sampled paths `gone`. `target/sync-backup-2026-09-21` matched **no** committed state here (`DIFFER` on 4 of 4 against both `a4cbab5^:` and `a4cbab5:`) and was proven **byte-identical to `bedrock`'s `HEAD`** copies (`MATCH` on 4 of 4), so it was the incoming scaffold and not a backup — deleted on that evidence, and identifying it exposed `PROGRAM.26`. **2.2 GB retained on evidence**, with two new retentions: `build/riscv-virt.dtb` needs the pinned emulator to regenerate and `M2.8.2` compares against it, and `target/s0-demo/base` is cited by a closed leaf and needs a full `archogen build`. `make focused` → `passed — 3 / 0 / 0`; `cargo test --all` → **492 passed, 0 failed**; 13 doctrines green, with `target/doctrine_scratch` recreated by the run |
| `2026-09-29` | `PROGRAM.27` (filed) | **docs only in this tree — a registered doctrine gate reproduced failing to fail.** The reproduction is on the leaf and was taken on the real tree, not on a fixture: the pending note moved out of `docs/semantics/migrations/` entirely, the baseline left amended at 72 constructs, and `scripts/check_language_freeze.sh` re-run. Then the two causes located in the check's own source: the notes population grep and `names_construct`'s `*all*` case, each read against `docs/semantics/migrations/README.md`'s form template. Restored and proven: `diff -q` silent on the note and `language-freeze: OK (72 …)` with it back in place | `language-freeze: OK` with **no note at all** and an amended baseline — the exact waiver leg B was written to close, `--emit` and green. `grep -rl '^- status:[[:space:]]*pending' docs/semantics/migrations` → `README.md`, whose form template carries `- status: pending \| applied`; `grep -n '^- constructs:'` on it → `36:- constructs: <construct id>, <construct id>   — or: all`, which the `*all*) return 0 ;;` substring case reads as covering every construct. ⛔ Nine RED arms and none of them could see it: all nine point `LANGUAGE_FREEZE_NOTES` at `$work/notes`, a scratch directory holding only the fixture note, so the arms proved the mechanism and never the deployed population — `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md` one level up. Filed at priority **high** and sequenced ahead of `PROGRAM.11`: a gate that cannot fail is worse than no gate, because the registry is what a reader consults to find out what is enforced. Two of the migration README's workflow steps are false as written and are part of the same fix, measured in all three note states (`pending` before `--emit` → leg A red, which the README calls green; `pending` after → green; `applied` after → green only because leg B is inert) |
| `2026-09-30` | `PROGRAM.9.3` / `PROGRAM.9` | `cargo xtask mutate` twice (the second naming each kill's tests); harness arms H-1–H-4 through temporary entries; `cargo test -p xtask`; the `extended` tier end to end | 8 of 8 as expected, each kill by the test written for its defect, the `lcm`→`max` blind spot surviving the harmonic corpus; every arm refused as designed, `crates/` clean after each; 14 / 0; **`extended` passed for the first time** |
| `2026-09-30` | `PROGRAM.6.3` / `PROGRAM.6` | both goldens blessed and checked; the lexer's own test; F-1–F-4 with restores; the two new catalog entries and the whole catalog; `cargo test --all` | 20 provenance key paths, re-derived from the renderer; the cost contract frozen; every falsification refused, blessing never overwrites; 11 of 11 mutations as expected; 620 / 0 |
| `2026-09-30` | `PROGRAM.17.3` / `PROGRAM.17` | growth since `2026-09-27` from `git show 4d6d002:<file>`; closed-leaf share of `M1.md` and `PROGRAM.md` by an `awk` over leaf blocks | 77% of `M1.md` and 75% of `PROGRAM.md` are closed leaves; §8 written, `PROGRAM.31`/`.32` filed blocked; `PROGRAM.17` closed |
| `2026-09-30` | `PROGRAM.20.3` / `PROGRAM.20` | a census of figure-shaped text in the live surfaces; the gate's 14 arms; eight mutations; the `S0.8` shape staged on the real tree | 96 phrases over 28 files as backlog; the added figure refused; the register's own first row refused as false; `PROGRAM.20` closed |
| `2026-09-30` | `PROGRAM.19` (fourth run) | the trigger off the record's commit timestamp; a full inventory; each scratch name censused against the tracked tree and its leaf's status read; a residue census; the provisioner, the focused tier, the whole suite and the gate, cold | 6.3 GB → 4.1 GB; ten of ten paths `gone`; both CI tools `already in place`; `passed — 3 passed`; 742 passed / 0 failed over 62 suites; all green — rows added by `ARCHOGEN-PROGRAM-0197`, since `0196` omitted them |
| `2026-10-10` | `PROGRAM.69` (rounds 7, 8) | the self-test, each new arm's rule mutated; the gate over 170 files; the doctrines | 69, then 71 arms, every new rule's mutation killed; OK; all green |
| `2026-10-10` | `PROGRAM.69` (round 9) | the self-test, the new arms failing first on `7ad8e6b`'s tool; nine mutations of round 9's rules; the gate over 172 files; the doctrines | 74 pass / 2 fail — the working tree's first new arms beside `7ad8e6b`'s core, not durable — then 77 / 0; each killed by an untracked runner, not durable; OK; all green |
| `2026-10-10` | `PROGRAM.69` (round 10) | the self-test, the new arms failing first beside `0a7ec7f`'s core; eleven mutations of round 10's rules; the gate over 172 files; every self-test; the doctrines | 79 pass / 5 fail, the working tree's and not durable, then 85 / 0; each killed by an untracked runner, not durable; OK; 47 passed; all green |
| `2026-10-10` | `PROGRAM.69` (round 11) | the new arms beside `b87adbc`'s core; the self-test; six mutations of round 11's rules; the gate over 172 files; every self-test; the doctrines | 82 pass / 9 fail, the working tree's and not durable; 90 / 0; each killed by an untracked runner, not durable; OK; 47 passed; all green |
| `2026-10-10` | `PROGRAM.69` (round 12) | the new arms beside `76d8775`'s core; the self-test; seven mutations of round 12's rules; the gate over 172 files; every self-test | 88 pass / 11 fail, the working tree's and not durable; 98 / 0; each killed by an untracked runner, not durable; OK; 48 passed |
| `2026-10-10` | `PROGRAM.69` (round 13) | the arms as committed beside `1ad87e4`'s core; the self-test; five mutations of round 13's rules; the gate over 172 files; every self-test; focused on the staged tree | 88 pass / 10 fail, the working tree's and not durable; 98 / 0; each killed by an untracked runner, not durable; OK; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 14) | the arms as committed beside `6c79b36`'s core; the self-test; seven mutations of round 14's rules; the gate over 172 files; every self-test; focused on the staged tree | 97 pass / 9 fail, the working tree's and not durable; 106 / 0; each killed by an untracked runner, not durable; OK; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.76` (round 1) | the check on the real set; its self-test; the arms beside `2f6f331`'s core; eleven mutations; the case lines a diff of `2f6f331` shows; every self-test; focused on the staged tree | exit 0; 40 / 0; 27 pass / 13 fail, not durable; 11 killed, not durable; 0; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 15) | the arms as committed beside `421ce47`'s core; the self-test; six mutations of round 15's rules; the gate over 172 files; every self-test; focused on the staged tree | 108 pass / 1 fail, the working tree's and not durable; 109 / 0; each killed by an untracked runner, not durable; OK; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.76` (round 2) | the check on the real set; its self-test; the arms beside `6d61f65`'s core; ten mutations; the stale clones' indexes; every self-test; focused on the staged tree | exit 0, 0.9 s; 53 / 0; 44 pass / 9 fail, not durable; 10 killed, not durable; none holds a case; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 16) | the arms as committed beside `755c151`'s core; the self-test; five mutations of round 16's rules; the gate over 172 files; every self-test; focused on the staged tree | 109 pass / 2 fail, the working tree's and not durable; 111 / 0; each killed by an untracked runner, not durable; OK; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.76` (round 3) | the check on the real set; its self-test; the arms beside `00e08cc`'s core; seven mutations; the clones older than `6d61f65`; every self-test; focused on the staged tree | exit 0; 56 / 0; 52 pass / 4 fail, not durable; 7 killed, not durable; none left; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 17) | the arms as committed beside `4105124`'s core; the self-test; four mutations of round 17's rules; the gate over 172 files; every self-test; focused on the staged tree | 112 pass / 4 fail, the working tree's and not durable; 116 / 0; each killed by an untracked runner, not durable; OK; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.76` (round 4) | the check on the real set; its self-test; the arms beside `a9ec254`'s core; four mutations; every self-test; focused on the staged tree | exit 0; 58 / 0; 56 pass / 2 fail, not durable; 4 killed, not durable; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 18) | texts and comments alone; the self-test; the gate over 172 files; every self-test; focused on the staged tree | 116 / 0; OK; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.76` (round 5) | the check on the real set; its self-test; the arms beside `04e306c`'s core; four mutations; every self-test; focused on the staged tree | exit 0; 63 / 0; 58 pass / 5 fail, not durable; 4 killed, not durable; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 19) | texts and comments alone; the self-test; the gate over 172 files; every self-test; focused on the staged tree | 116 / 0; OK; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.76` (round 6) | the check on the real set; its self-test; the arms beside `fa0ec3f`'s core; four mutations; every self-test; focused on the staged tree | exit 0; 65 / 0; 63 pass / 2 fail, not durable; 4 killed, not durable; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 20) | the self-test; two mutations of `put`; the gate over 172 files; every self-test; focused on the staged tree | 117 / 0; each failing the race arm, not durable; OK; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 21) | the self-test; the gate over 172 files; every self-test; focused on the staged tree | 118 / 0; OK; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.76` (round 7) | the push; the check on the real set; its self-test; four mutations; every self-test; focused on the staged tree | `origin/main` at `a543d10`, its `frozen/` the manifest alone; exit 0; 69 / 0; 4 killed, not durable; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 22) | the self-test; four mutations of the rules round 22 arms; the gate over 172 files; every self-test; focused on the staged tree | 120 / 0; each killed by an untracked runner, not durable; OK; 48 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.76` (round 8) | the self-test; the check on the tree; every self-test; two mutants, `target/p76/mutate_r8.py`, untracked; the case blobs' commits; focused on the staged tree | 71 pass / 0 fail; exit 0; 48 passed; 2 of 2 killed, not durable; `b05fadb` and `2f6f331` each; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.77` (closed) | the restoring push's runs on the runner, `gh run view 38069871798`, `38069871797`, `38069871851` | `focused` success in 1 min 55 s, `integration` in 3 min 45 s; `doctrines`, `trust-gate` success |
| `2026-10-10` | `PROGRAM.69` (round 23) | the self-test; `REVIEW-COUNTS`, its self-test, on the tree before and after row 22's fix; two mutants of round 23's rules; the gate over the history; every self-test; focused on the staged tree | 121 pass / 0 fail; 9 pass / 0 fail, exit 1 naming row 22 then exit 0; 2 of 2 killed, untracked; OK; 49 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.76` (round 9) | the self-test; the check on the tree; six mutants, `target/p76/mutate_r9.py`, untracked; GitHub's push record; every self-test; focused on the staged tree | 81 pass / 0 fail; exit 0; 6 of 6 killed, not durable; `32e6b14` then `a3c0cbd`; 49 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 24) | `REVIEW-COUNTS` and its self-test; eight mutants, `target/p69r17/mutate24.py`, untracked; the sealing self-test; every self-test; focused on the staged tree | OK, 43 rounds of 2 records, 4 archived named (not so, review round 25 found: eight unnamed); 18 pass / 0 fail; 8 of 8 killed, not durable; 121 pass / 0 fail; 49 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.76` (round 10) | the self-test; the check on the tree; eight mutants, `target/p76/mutate_r10.py`, untracked; a monitor hook probed in a scratch repository; every self-test; focused on the staged tree | 92 pass / 0 fail; exit 0; 8 of 8 killed, not durable; `ls-files -m` empty under the hook, the file listed with it forced off; 49 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 25) | `REVIEW-COUNTS` and its self-test; six mutants, `target/p69r17/mutate25.py`, untracked; the sealing self-test; every self-test; focused on the staged tree | OK, every row judged or named (not so, review round 26 found); 20 pass / 0 fail; 6 of 6 killed, not durable; 121 pass / 0 fail; 49 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.76` (round 11) | the self-test; the check on the tree; nine mutants, `target/p76/mutate_r11.py`, untracked; every self-test; focused on the staged tree | 105 pass / 0 fail; exit 0; 9 of 9 killed, not durable; 49 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.69` (round 26) | `REVIEW-COUNTS` and its self-test; eight mutants, `target/p69r17/mutate26.py`, untracked; the sealing self-test; every self-test; focused on the staged tree | OK, 2 reviews listed; 25 pass / 0 fail; 8 of 8 killed, not durable; 121 pass / 0 fail; 49 passed; `passed — 3 passed, 0 failed` |
| `2026-10-10` | `PROGRAM.76` (round 12) | the self-test; the check on the tree; five mutants, `target/p76/mutate_r12.py`, untracked; every self-test; focused on the staged tree | 113 pass / 0 fail; exit 0; 5 of 5 killed, not durable; 49 passed; `passed — 3 passed, 0 failed` |
| `2026-10-11` | `PROGRAM.69` (round 27) | `REVIEW-COUNTS` and its self-test; twelve mutants, `target/p69r17/mutate27.py`, untracked; the sealing self-test; every self-test; focused on the staged tree | OK, 1 review listed, 27 rounds judged, every one; 35 pass / 0 fail; 12 of 12 killed, not durable; 121 pass / 0 fail; 49 passed; `passed — 3 passed, 0 failed` |
| `2026-10-11` | `PROGRAM.69` (the counted list) | `REVIEW-COUNTS` and its self-test; four mutants of the list's rules, `target/p69r17/mutate_list.py`, and round 27's twelve, untracked; `REVIEW-HISTORY`; every self-test; focused on the staged tree | OK, 1 review listed, 27 rounds judged, every one; 38 pass / 0 fail; 4 of 4 and 12 of 12 killed, not durable; OK; 49 passed; `passed — 3 passed, 0 failed` |
| `2026-10-11` | `PROGRAM.69` (round 28) | `REVIEW-COUNTS` and its self-test; thirteen mutants, `target/p69r17/mutate28.py`, and round 27's eleven re-anchored, `mutate27b.py`, untracked; the sealing self-test; every self-test; focused on the staged tree | reported OK, 2 reviews listed, 28 rounds judged (an earlier run, not the final-tree count: review round 29 re-derived 29); 51 pass / 0 fail; 13 of 13 and 11 of 11 killed, not durable; 121 pass / 0 fail; 49 passed; `passed — 3 passed, 0 failed` |
| `2026-10-11` | `PROGRAM.76` (round 13, hand-off) | the draft reader on the tree; the committed check restored; the no-background check; focused on the staged tree | three tracked files refused, exit 1, the draft untracked; exit 0; `handoff: OK`; `passed — 3 passed, 0 failed` |
| `2026-10-11` | `PROGRAM.76` (round 13) | the new arms on the prior reader, then the revised reader; the real-tree check; syntax; book anchors/glossary; review counts; doctrines | 127 pass / 10 fail, then 137 pass / 0 fail; exit 0; all named checks green; case-sensitive filesystem arm skipped explicitly |
| `2026-10-11` | `PROGRAM.69` (round 29) | new regressions before repair, then both self-tests; shell syntax; real history gate and census; final staged review-count gate; book gates; every self-test and doctrines | 53 pass / 13 fail (66 count arms), 122 pass / 2 fail (124 history arms), then 69 / 0 and 124 / 0; history OK, 175 sealed files; census 12 873 bytes, no closed subtree below an open top-level subtree; final staged review-count gate OK, 2 reviews / 30 rounds; book gates OK; doctrines all green; broader self-test runner finished: 47/49 passed restricted, both process-dependent suites passed with OS access (2/2 and 39/39 arms); all 49 covered across two runs before round 30 |
| `2026-10-11` | `PROGRAM.69` (round 30) | eight escaped-pipe regressions before repair, then the count self-test; syntax; final staged gate; history/book/doctrine checks | 70 pass / 8 fail, then 78 pass / 0 fail; final staged count gate OK, 2 reviews / 31 rounds; history OK, 175 files; book anchors/glossary/index OK; doctrines all green |
| `2026-10-11` | `PROGRAM.69` (round 31) | first-link regression arms before repair, then the count self-test; syntax, staged real count/history gates, book gates, doctrines | 79 pass / 2 fail, then 81 pass / 0 fail; staged count gate OK (2 reviews / 32 rounds); history gate OK (175 files); book anchors/glossary/index OK; doctrines all green |
| `2026-10-11` | `PROGRAM.76` (round 14) | six source-line-ending controls before repair, then the complete guard suite; real sealed-tree guard, syntax, review counts, book gates and doctrines | 141 pass / 2 fail, then 143 pass / 0 fail; real guard exit 0, syntax clean; review counts OK (2 reviews / 32 rounds); book gates OK, doctrines all green; filesystem case-distinction arm explicitly skipped |
| `2026-10-11` | `PROGRAM.69` (round 32) | ten staged-read controls on the prior reader, then repaired suite; exact-path comparison mutation; syntax, final staged real count/history gates, book gates and doctrines | 83 pass / 8 fail, then 91 pass / 0 fail; glob-comparison mutation 89 pass / 2 fail; final staged count OK (2 reviews / 33 rounds), history OK (175 files); book gates, stated-order and task-acceptance OK; final doctrines in the commit hook |
| `2026-10-11` | `PROGRAM.76` (round 15, closure) | independent pinned-reader review; real-tree guard; shell syntax; book anchors, glossary and index; final staged doctrine hook | zero defects or actionable arm gaps; 157 / 0 synthetic probes (151 repaired-reader checks plus six prior-reader controls), untracked evidence; real guard exit 0; syntax and book gates OK; checker unchanged |
| `2026-10-11` | `PROGRAM.69` (round 33) | 35 primary-link controls before repair, then repaired suite; original shell reproducer; two valid selection/refusal mutations; syntax, final staged count/history, book and doctrine gates | 96 pass / 30 fail, then 126 pass / 0 fail; original five shell cases refuse the primary table; selection mutation 93/33, refusal mutation 120/6; initial syntax-error mutant excluded; staged counts 2 reviews / 35 rounds, history 175 files / 335 stubs; book anchors 30/4, glossary 67/25, index 316; syntax and stated-order OK; task-acceptance evidence completed after its first refusal; final doctrines in commit hook |
| `2026-10-11` | `PROGRAM.69` (round 34) | native old/new mutation replays, full count self-test, syntax, book and final staged gates | both mutants 126/0 before; validation removal 135/3 and angle restriction 130/8 of 138 after; unchanged production reader 138/0; book anchors 30/4, glossary 67/25, index 316; staged counts 1 review / 34 rounds, history 175 files / 335 stubs; acceptance/order/figure gates OK, final doctrines in hook |
| `2026-10-11` | `PROGRAM.69` (round 35) | new census and summary-boundary controls before/after repair; both full self-tests, syntax, book and final staged gates | history 129/3 of 132 and counts 140/10 of 150 before; repaired 132/0 and 150/0; finite selected-read failures never print a final aggregate, empty counterparts pass; staged counts 1 review / 35 rounds, history 175 files / 335 stubs; book 30/4, 67/25, 316, syntax and final doctrine gates |
| `2026-10-11` | `PROGRAM.69` (round 36) | EOF/path/diagnostic controls before repair, then both complete self-tests; syntax, book and staged gates | history 133/13 of 146 and counts 153/3 of 156 before; repaired 146/0 and 156/0; literal byte and unusual-path counterparts, named no-traceback row refusals; independent round 37 next |
| `2026-10-11` | `PROGRAM.69` (round 37 closure) | independent pinned review; root recount/authentication; staged count/history and book gates | zero defects/actionable arm gaps/remarks; 347 expected checker outcomes, separate 4 096 unit comparisons; retained results recount and 17 source identities pass, exit code 0; scope/limits retained; all 37 summary rows judged before removal from counted list |
| `2026-10-22` | `PROGRAM.101` (fourteenth run) | two handoff censuses; physical and native inventories; identity of every other `target/` entry before and after; input snapshot and restoration; audit and four falsifying arms; the focused tier on the staged tree | 5382 files / 1181873277 logical bytes removed; 590423 of 590426 identical, 3 accounted; 1205 inputs restored; 4 of 4 arms refused |

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
| `PROGRAM.109` | `ARCHOGEN-PROGRAM-0928 (leaf PROGRAM.109)` | **the fifteenth daily artifact cleanup**: only the idle main and wasm debug incremental caches removed, 29847 files / 4245385971 logical bytes; every other entry under `target/` identical or accounted; `PROGRAM.100`–`.102` and `.104`–`.108` sealed |
| `PROGRAM.110` | `ARCHOGEN-PROGRAM-0952 (leaf PROGRAM.110)` | **the verification doctrine 22 recorded**: small checks per slice, full verification after the feature set, deferred to `M3.5.9` for `M3.5` |
| `PROGRAM.111` | `ARCHOGEN-PROGRAM-0953 (leaf PROGRAM.111)` | **the sixteenth daily artifact cleanup**: only idle incremental caches removed, 8854 files / 1586169011 bytes, a light run under doctrine 22 |
| `PROGRAM.112` | `ARCHOGEN-PROGRAM-0959 (leaf PROGRAM.112)` | **doctrine 22's record corrected**: what `make focused` runs — workspace clippy, `cargo test --all`, the sweep's premises — stays at every commit, a pin a slice moves re-pinned by it; the rest deferred to `M3.5.9` |
| `PROGRAM.71` | `ARCHOGEN-PROGRAM-0971 (leaf PROGRAM.71)` | **the task-history index append-only in order**: leg 3 refuses committed rows that no longer stand first and in their order; an arm; `PROGRAM.72` next |
| `PROGRAM.113` | `ARCHOGEN-PROGRAM-0974 (leaf PROGRAM.113)` | **filed**: the task-history self-test's `SIGALRM` arm failed once on the Linux runner and passed on re-run; to be reproduced on Linux before any change |
| `PROGRAM.113.1` | `ARCHOGEN-PROGRAM-0976 (leaf PROGRAM.113.1)` | **the diagnostic**: the signal arms' failure line carries the seal's evidence, the runner's log keeping only that line; the failures now steady on CI |
| `PROGRAM.113.2` | `ARCHOGEN-PROGRAM-0977 (leaf PROGRAM.113.2)` | **the trace**: the signal arms record the signal sent, each mask change and each outcome printed, in order, for the runner to show where Linux departs |
| `PROGRAM.113.3` | `ARCHOGEN-PROGRAM-0978 (leaf PROGRAM.113.3)` | **the cause and its fix**: GNU grep's `-q` closing the pipe under `pipefail` while `printf` still writes a large output; every such read in the script a here-string; `PROGRAM.114` filed for the 78 elsewhere |
| `PROGRAM.113` | `ARCHOGEN-PROGRAM-0979 (leaf PROGRAM.113)` | **closed, CI green**: the arms' own check at fault, not the seal; the three runs on `34b6abeb` each `success` |
| `PROGRAM.114` | `ARCHOGEN-PROGRAM-0980 (leaf PROGRAM.114)` | **`PIPE-READS`**: every pipe read under `pipefail` a here-string, the gate refusing the form, the scaffold's sites reported and their change written down; `PROGRAM.115` filed |
| `PROGRAM.116` | `ARCHOGEN-PROGRAM-0989 (leaf PROGRAM.116)` | **what is held for the scaffold, from the ruling**: `.doctrine/scaffold_held.txt`, sixteen files, read by `SCRATCH-LOCALITY` and `PIPE-READS`; `check_task_acceptance.sh`'s eight sites fixed; `PROGRAM.115`'s premise corrected |
| `PROGRAM.117.1` | `ARCHOGEN-PROGRAM-1020 (leaf PROGRAM.117.1)` | **record folders bound by reads, not totals** (the director's ruling of `2026-10-27`): `fan-out 64` and a 16 MiB `guard` with its three-quarter ruling and per-commit step for `docs/decisions/`, `docs/specs/`, `docs/reviews/`; `--report`; three ceiling records superseded; `.3` filed |
| `PROGRAM.119` | `ARCHOGEN-PROGRAM-1021 (leaf PROGRAM.119)` | **seventeenth daily artifact cleanup**: the main and wasm debug incremental caches, 52855 files / 5496697254 bytes, removed after `handoff: OK` |
| `PROGRAM.120` | `ARCHOGEN-PROGRAM-1068 (leaf PROGRAM.120)` | **eighteenth daily artifact cleanup**: the main and wasm debug incremental caches, 15647 files / 1926076171 bytes, removed after `handoff: OK` |
| `PROGRAM.117.2` | `ARCHOGEN-PROGRAM-1022 (leaf PROGRAM.117.2)` | **the record mover**: `scripts/move_record.sh`, a dry run and a verified, staged move; live citations repointed, the past left and listed |
| `PROGRAM.118` | `ARCHOGEN-PROGRAM-1028 (leaf PROGRAM.118)` | **`README-ROUTES` reads every path whole**: NUL-separated listings, the row's files quoted, a newline in a name refused |
| `PROGRAM.115` | `ARCHOGEN-PROGRAM-1036 (leaf PROGRAM.115)` | **the enforcement table holds `PIPE-READS`**: `BOOK-ANCHORS`' row shortened to its rule, its rationale kept in its script's header; thirteen bytes free |
| `PROGRAM.117.3` | `ARCHOGEN-PROGRAM-1037 (leaf PROGRAM.117.3)` | **an index never binds the folders it lists**: `DECISION-INDEX` reads a sub-folder's own index and holds `docs/specs/` too; the three unindexed specifications indexed |
| `PROGRAM.73` | `ARCHOGEN-PROGRAM-1039 (leaf PROGRAM.73)` | **the history gate names every failure**: any other exception a breach, `the gate stopped on TYPE`, never a traceback; the seal's arm and a plain-run arm require it |
| `PROGRAM.72` | `ARCHOGEN-PROGRAM-1040 (leaf PROGRAM.72)` | **no tree file in a sub-folder**: `TASK-HISTORY`'s leg 6 refuses a Markdown file under a sub-folder of `docs/tasks/`, which no leg would judge |

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
