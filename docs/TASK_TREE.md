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
| [`PROGRAM`](tasks/PROGRAM.md) | `active` | `PROGRAM.11` — the repository-boundary doctrine: the rule lived only in a session prompt, and upstream published a crossing. Then `PROGRAM.21` (**high** — `TASK-ACCEPTANCE` reads only the first checklist in a tree file, so later leaves are unverified and the gate says otherwise), `PROGRAM.18`, and `PROGRAM.24` — the **mirror** of `BOOK-ANCHORS`, which nothing covers: does a capability the codebase has get described in the book? Filed on one measured instance, the `rt-analysis` crate named nowhere in `docs/book/` | repo-local |
| [`M1`](tasks/M1.md) | `active` | `M1.13.2` — the second of the freeze's five children: state the value domain as a property of `eadl/1` rather than an implementation limit. The decision is already made and recorded in the tree's Decisions — **it stays 64-bit** — on two measurements: the pinned RISC-V privileged spec puts the widest standardized address at 56-bit physical / 57-bit virtual against a 2^63−1 domain, and the largest of the corpus's 27 distinct literals is 2^28. So this child writes the `docs/decisions/` record, replaces §1's ⚠️ implementation-shaped marker with a stated rule, and names the one compatibility-corpus row a later widening would add — a widening being backward compatible, which is why deferring it costs a row and not a migration. **`M1.13.1` is closed**: F-G was three defects, not one, and all three are closed by executed rows rather than sentences — `\u{…}` joins the escape set so it is closed *and* sufficient, the printer escapes every Unicode `Cc` character (where the rule has to live, because `Form::Str` is constructible outside the frontend), the reader refuses a raw control byte inside a string as it already refused one between forms, and a `<0xNN>` source notation made a raw **NUL** an executable table row for the first time — which is also the notation `M1.26`'s gap (b) was missing. Then `.13.3` (the version identifier, which cannot ride the comment-header convention because canonical form drops comments), `.13.4` (the suite and its baseline: **62** files, **13** feedback-evidence files excluded), `.13.5` (the freeze gate), `.25`, `.26`, `.10`, `.21`, `.22` | repo-local |
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
