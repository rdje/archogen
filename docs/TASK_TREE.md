# Task-Tree Workflow

This document defines the repo-local task-tree workflow. A step-by-step setup guide is in
[TASK_TREE_README.md](TASK_TREE_README.md). Individual trees live under
[`tasks/`](tasks/); the leaf template is [`tasks/TEMPLATE.md`](tasks/TEMPLATE.md).

## Purpose

Use a task tree when a top-level task is too broad to finish safely as one signoff-quality
slice, or when it is expected to discover subtasks over time. The tree owns the recursive
breakdown, current frontier, acceptance criteria, blockers, decisions, validation, and
completion evidence for one top-level task — so the project survives a lost session and
continuity holds across sessions, machines, and harness switches.

The tree is not a second roadmap. `ROADMAP.md` states the high-level direction; a task tree
owns the disciplined execution of one lane of it.

## Code-change doctrine (binding, non-negotiable)

**It is strictly forbidden to make any code change unless it is first tracked/owned by a
task-tree leaf.** "Code change" = any edit to Rust sources, `Cargo.toml`/build scripts,
generated artifacts, or anything altering behavior. Before touching code, a leaf must exist
that owns the change (create/extend a tree, or add a leaf). The leaf — its goal, acceptance,
verification, and commit — is the unit of traceability. Enforced by
`scripts/check_task_tree_ownership.sh`.

## Leaf lifecycle (statuses)

`proposed` → `pending` → `active`/`in_progress` → `done`. A leaf is `done` only when its
acceptance criteria are met, verification is recorded, and it is committed via `COMMIT.md`.
Mark `blocked` (with the blocker named) rather than leaving a stalled leaf `active`.

## The pivot rule

**Do not pivot to a different task-tree while the repo is dirty.** The repo is
handoff-ready only when the tree is clean (no modified/untracked work except the task-tree
file itself). Finish the current leaf and get the repo clean before switching — even if
asked to pivot immediately. The guarantor of repo integrity holds this line.

## Commit traceability

Each slice uses a work-unit id in the commit subject (e.g. `MYPROJ-AREA-0007`). When the
slice belongs to a leaf, the subject or first body line also names the leaf ID
(e.g. `MYPROJ-AREA-0007 (leaf FEATURE-X.2): …`), so the slice id and the tree node coexist
on the same commit. One commit per completed leaf.

## Active Task Trees

The roadmap (`ROADMAP.md`, revision 2.0) is represented in full by the trees below. The
roadmap-unit → tree mapping and the F01–F30 fixture-ownership map live in
[`tasks/PROGRAM.md`](tasks/PROGRAM.md).

| Tree | Status | Frontier (next leaf) | Owner |
| --- | --- | --- | --- |
| [`S0`](tasks/S0.md) | `active` | `S0.8` — the book's S0 chapter says three descriptions where the directory holds four, and says four later in the same chapter. F28's evidence is unaffected; the seven original leaves stay closed | repo-local |
| [`PROGRAM`](tasks/PROGRAM.md) | `active` | `PROGRAM.21` — **high**: `TASK-ACCEPTANCE` reads only the *first* checklist in a tree file, so for every later leaf it verifies nothing and then says it did. Measured **active**, not latent: `cd355ef` staged five Rust files and the box it read was leaf `M1.1`'s from `2026-09-13`; `3a6bbb9` was gated the same way. ⭐ **The design is already pinned by measurement on the leaf — do not re-derive it**: the obvious signal (which leaf sections a commit's diff touches) agrees with the commit subject on **1 of 7** real code commits, so the owner comes from `TASK_ACCEPTANCE_LEAF` or the `(leaf X)` token in the pending message's subject, and the check **refuses** when it cannot tell. Nine `--self-test` arms and both resume traps are listed there. Then `PROGRAM.11` (the repository-boundary doctrine, in both directions), `PROGRAM.18`, and `PROGRAM.24` — the **mirror** of `BOOK-ANCHORS`, which nothing covers: does a capability the codebase has get described in the book? Filed on one measured instance, the `rt-analysis` crate named nowhere in `docs/book/` | repo-local |
| [`M1`](tasks/M1.md) | `active` | `M1.13.3` — the third of the freeze's five children: put the **version identifier** on the surface, the criterion `M1.13` is named for. Measured constraint: it cannot ride the comment-header convention, because §3 drops comments from canonical form and §12 M4 hashes canonical text — a version in trivia is a version the hashed artifact does not capture — so it must be a **form**, a grammar change that is free only before `.4` writes the baseline. Two questions are free to decide only here: what **absence** of the identifier means, and `M1.13.1`'s routed one (whether an invisible character outside Unicode `Cc` is refused). ⭐ `M1.13.2` already made `eadl/1` a name §1 cites normatively, so this child puts an identifier on a surface already using it. **`M1.13.2` is closed**: F-F settled — the value domain is a stated property of `eadl/1` (§1 rule 9) with its cost stated out loud (§1 rule 10) and both carried by **four new executed rows**, plus `docs/decisions/decision_eadl1-value-domain.md` and a `literals` census instrument that asks the frontend rather than a second regular expression. ⛔ Both measurements it was routed on were **false** when re-derived: a census figure with an unstated scope and a radix-scoped maximum, restated in **7** places over 3 live surfaces, and an address argument true of *physical* addresses only — every canonical high-half **virtual** one is refused today in the unsigned spelling a datasheet prints, while its signed equivalent reads fine. The decision stands on a reason that survives measurement, with three named triggers recorded. **`M1.13.1` is closed**: the escape set is closed *and* sufficient and §3 rule 3 is total, closed by executed rows — `\u{…}` in the language, `- control` in the grammar, and a `<0xNN>` notation that made a raw **NUL** a table row for the first time. Then `.13.4` (the suite and its baseline: **62** files, **13** feedback-evidence files excluded), `.13.5` (the freeze gate), `.25`, `.26`, `.10`, `.21`, `.22` | repo-local |
| [`API`](tasks/API.md) | `active` | `API.1` — **measure** whether the engine compiles for `wasm32-unknown-unknown`, as a tier step in the shape of the existing `no-std-build`. Seeded `2026-09-28` from the director's ruling and `ROADMAP.md` §10.4: one declared engine API with a wasm binding and an **MCP server** any agent can drive, controlling an instance **post-build only** with both builds excluded. Then `API.2` (state and gate the no-subprocess invariant, which §10.4's safety argument rests on and which is currently written down nowhere). `API.3`–`API.7` are sequenced behind `M1.13`'s freeze and do **not** displace it | repo-local |
| [`M0`](tasks/M0.md) | `done` | — all seven leaves closed; F27 green | repo-local |
| [`M2`](tasks/M2.md) | `active` | `M2.8.2` — the release is **pinned** (`M2.8.1`, QEMU 11.1.1), so write the device-tree fixture the §3.2 agreement check compares against: `DEVICE_TREE_FIXTURE` names a file that does not exist, and no eADL description of the target exists either. Then `M2.6`; `M2.8.3` is sequenced after `M1.13` | repo-local |
| [`M3`](tasks/M3.md) | `pending` | `M3.1` — candidate enumeration and substitutability | repo-local |
| [`M4`](tasks/M4.md) | `pending` | `M4.1` — the typed runtime/build plan | repo-local |
| [`M5`](tasks/M5.md) | `blocked` | — **no board procured** (`M0.5`, 2026-09-13) | repo-local |
| [`M6`](tasks/M6.md) | `pending` | `M6.1` — three materially different systems | repo-local |
| [`M7`](tasks/M7.md) | `pending` | `M7.4` — the F01–F30 mandatory-case audit | repo-local |
| [`BOOTSTRAP`](tasks/BOOTSTRAP.md) | `done` | — | repo-local |

## Keeping this index true

This table is a **pointer**, and the trees are the source. It is updated in the same commit as
any leaf that changes a tree's status or frontier. A stale index is worse than no index: it is
the file a resuming session reads first, and it is the one place a wrong "next action" costs a
whole session's direction.
