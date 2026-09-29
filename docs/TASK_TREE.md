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
| [`PROGRAM`](tasks/PROGRAM.md) | `active` | `PROGRAM.11` — **medium**, and **the frontier leaf**: the repository-boundary doctrine in **both** directions (`docs/decisions/decision_repository-boundary-read-only.md`). The rule was nowhere in the committed tree, which is how an *inbound* crossing got recorded backwards in a durable record; the outbound half is preventive, the inbound half fired once and was handled correctly. **`PROGRAM.21` is closed** (`2026-09-29`, `7c59b0b`): `TASK-ACCEPTANCE` is **leaf-scoped** — the owner comes from `TASK_ACCEPTANCE_LEAF` or the `(leaf X)` token in the pending message's subject through the `.doctrine/commit_message_file` seam, and the check **refuses** when it cannot tell, because falling back to the first checklist in the file *was* the defect. The obvious signal was priced before it was discarded: which leaf sections a commit's diff touch agrees with the commit subject on **1 of 7** real code commits. Nine `--self-test` arms, whose own first oracle scored **4 passes on `exit 127`** — reproduced deliberately as mutation B2 and promoted into `docs/knowledge/verify-the-mutation-applied.md`. Then `PROGRAM.18` (**nine** of eighteen registered controls carry no repeatable RED arm — the figure was ten until `.21` armed one, so `.18` re-runs its census rather than reusing it), and `PROGRAM.24` — the **mirror** of `BOOK-ANCHORS`, which nothing covers: does a capability the codebase has get described in the book? Filed on one measured instance, the `rt-analysis` crate named nowhere in `docs/book/` | repo-local |
| [`M1`](tasks/M1.md) | `active` | `M1.28` — **high, a live production defect measured `2026-09-29`**: `archogen check` **accepts** `examples/s0-heartbeat/system.eadl` with `(period 10 parsec)` and `archogen build` refuses the same bytes as `quantity-unknown-unit`, and a zero tick rate — §13.1's **F03**, an M1 gate fixture — draws no diagnostic from `check` at all. Three consumers of `Quantity::read` discard what it finds (`workload.rs:178`, `refinement.rs:177` and `:188`) and the only one that propagates is the S0 prototype `S0-RETIREMENT` exists to delete; the schema cannot ask the question either, because `period` is declared `(holds values number symbol)` and `ValueType` has no quantity. ⭐ Every gate stayed green because all of them are **existence** censuses and existence is a property of the emitter — promoted to `docs/knowledge/an-existence-census-cannot-see-a-discarded-result.md` with three more instances of the shape found in the same hour (**F-H** a §4 row describing a mechanism `grep` cannot find, **F-J** a code whose only site is a totality arm, **F-K** the suite's F03 case passing on a clause-name typo). Sequenced **ahead of `M1.26.1`**, which would otherwise be born stating a rule the pipeline does not enforce, and it costs a migration note. Then `M1.26.1` — **medium**: `M1.26` is `active` and **decomposed into two children** (`2026-09-29`) on six censuses run before either was written, and the measurement moved three figures the leaf carried. §4's population is **60** rows, not 55 (`M1.13.3` added five after `M1.13.1` measured); **27** are named by no test, but the composition falsifies the leaf's own note — the two `read-` codes it credited to `M1.13.1` are still named by no test, because what `.1` pinned was reachability *from a §2 input*, a different property; and the ungoverned population is **16**, not 15, the extra being `analysis-inconclusive` from a crate the earlier six-file census did not walk. `.1` is gap (a): a second normative document for the model layer's codes, its own declaration, a both-directions census, and leg 8 widened to read *every* normative document's declaration — first because it moves no frozen construct, so it costs no migration note. Then `.2` (gap (b): §4's `fires on` column, all 60 rows in one commit, plus **F-H**), then `.27`, `.10`, `.21`, `.22`. ⛔ Two findings the census turned up, both owned: **F-H** — §4's `module-too-large` row states "more addressable parts than an instance identifier can hold", its only call site fires on a source of 2^32 bytes, and `grep` finds no such limit anywhere in the elaborator, so a live false normative sentence sits green under both existing legs (and the one condition behind it surfaces as **four** codes or messages); **F-I** — the frozen population is scoped by *file* (`TABLE_DOCUMENTS` names `reference.md` alone) and the reason is written down nowhere, which `.1` must answer because it adds the second document. **`M1.25` is closed**: the reach gate's history escape went from a past-tense **substring scan** to an explicit list of the two lines that carry a historical figure, quoted verbatim, so history stays legal *and visible* and rewording one of them fails the gate — the escape `M1.24` had already measured passing on its own defect in a sibling gate, where the `before` belonged to a quotation seven words ahead of the wrong number. ⛔ Its own leaf's premise was false and the census is what showed it: "all five existing arms are re-expressed" against `grep -c '^fn arm_' crates/eadl-model/tests/kinds.rs` → **0**, so the gate that closed `M1.23` had never been observed firing and the seven arms the sentence described belong to a different gate over different surfaces. Six arms were written, each pinning its violation count, and the discriminating evidence is a **meta-mutation**: putting the marker scan back fails exactly the two arms that test the replacement and leaves four green. Fourth instance of that shape in this tree, so it is promoted to `docs/knowledge/a-leafs-claims-about-the-repository-are-hypotheses.md`. No figure moved and no correct prose was rewritten. **`M1.13` is closed** — `eadl/1` is a version with a name on the surface, a declared conformance suite (`docs/semantics/conformance.md`, no count in it), a **70**-digest baseline enumerated at run time, and `LANGUAGE-FREEZE`, whose second leg compares the tracked baseline with `HEAD`'s so `--emit` is an explicit act needing a pending migration note and not the waiver a one-legged gate leaves open. **541 passed / 0 failed** over 39 suites, from 476 when `M1.13` opened | repo-local |
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
