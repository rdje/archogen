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
| [`PROGRAM`](tasks/PROGRAM.md) | `active` | `PROGRAM.27` — **high, and the frontier leaf: a registered doctrine gate that cannot fail.** `LANGUAGE-FREEZE`'s explicitness leg — the one `M1.13.5` called "the one that matters", because it is what stops `--emit` being the waiver — prints `OK` on the real tree with an amended baseline and **no migration note at all**. Two holes that only close together: the notes population grep matches `docs/semantics/migrations/README.md`, whose form template carries `- status: pending \| applied`, so the README is permanently a pending note; and `names_construct`'s `*all*` **substring** case reads that template's trailing `— or: all` as covering every construct id there is. ⛔ Nine RED arms and none could see it, because all nine point `LANGUAGE_FREEZE_NOTES` at a scratch directory holding only the fixture note — the arms proved the mechanism and never the deployed population, which is `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md` one level up. Found by `M1.28.2`, the first leaf to run the workflow; two of the README's own workflow steps are false as written and are inside the same fix. Ahead of `PROGRAM.11`, whose rule is preventive and has fired once and been handled, while this one is inert now. Then `PROGRAM.11` — **medium**: the repository-boundary doctrine in **both** directions (`docs/decisions/decision_repository-boundary-read-only.md`). The rule was nowhere in the committed tree, which is how an *inbound* crossing got recorded backwards in a durable record; the outbound half is preventive, the inbound half fired once and was handled correctly. **`PROGRAM.21` is closed** (`2026-09-29`, `7c59b0b`): `TASK-ACCEPTANCE` is **leaf-scoped** — the owner comes from `TASK_ACCEPTANCE_LEAF` or the `(leaf X)` token in the pending message's subject through the `.doctrine/commit_message_file` seam, and the check **refuses** when it cannot tell, because falling back to the first checklist in the file *was* the defect. The obvious signal was priced before it was discarded: which leaf sections a commit's diff touch agrees with the commit subject on **1 of 7** real code commits. Nine `--self-test` arms, whose own first oracle scored **4 passes on `exit 127`** — reproduced deliberately as mutation B2 and promoted into `docs/knowledge/verify-the-mutation-applied.md`. Then `PROGRAM.18` (**nine** of eighteen registered controls carry no repeatable RED arm — the figure was ten until `.21` armed one, so `.18` re-runs its census rather than reusing it), and `PROGRAM.24` — the **mirror** of `BOOK-ANCHORS`, which nothing covers: does a capability the codebase has get described in the book? Filed on one measured instance, the `rt-analysis` crate named nowhere in `docs/book/` | repo-local |
| [`M1`](tasks/M1.md) | `active` | `M1.29.2` — **high**: a disk-backed module loader wired into `archogen check` and `archogen build` through the routing pair `M1.29.1` landed, so §6's `module-*` codes are reachable from a command; two ⛔ design constraints are recorded on the leaf (resolve by a *stated* rule from the imported name so `module-name-mismatch` stays firable; a tracked module fixture is a `conformance.md` manifest decision). Then `M1.29.3` — **high**: the name rule an elaborated program needs, which §6 does not state; `M1.29.4` — module parameters, bound and consumed by nothing. Then `M1.26.1`, `M1.26.2`, `M1.30`, `.10`, `.21`, `.22`, `.27`, `.32`. ⭐ **Closed `2026-09-29`:** `M1.29.1` — a module file is refused as `unimplemented` (exit **20**, was **10**) by both commands, and `archogen help check` no longer claims elaboration; `M1.31` — a caret stays on the line it is drawn under (**14 runs over 10 tracked descriptions → 0**), pinned by a leg over the whole conformance suite. ⛔ The figure first published for that defect (23 of 78) was wrong in both directions and is corrected everywhere. **575 passed / 0 failed** over 41 suites | repo-local |
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
