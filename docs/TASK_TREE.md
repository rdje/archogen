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
| [`M1`](tasks/M1.md) | `active` | `M1.29` — **high, filed `2026-09-29` while answering the director's question about how a `.eadl` is known to be complete**: nothing in production elaborates a module tree. `git grep -n "elaborate(" -- crates` outside `src/module.rs` finds two hits and both are in `crates/eadl-front/tests/f01_f02_modules.rs`, `impl ModuleSource` has exactly one implementation (`MemoryModules`, an in-memory map that exists for tests), and `archogen check` on the `(defmodule …)` shape `docs/book/src/modules.md` opens with answers `error[schema-unknown-kind]: defmodule is not a known kind`. Meanwhile `spec.rs:151` — the string `archogen help check` prints — says "elaborate and type-check", and `cli.md:17`/`:30` copy it. So `ROADMAP.md` §10.1 step 1 does not run, none of §6's 24 `module-*` codes is reachable from any command, F01/F02 are green at library level only, and every existence census stays green because each is an existence census. Beside it `M1.30` (**medium**): §5.3's "unknown facts outside the closure remain **visible in metadata**" is computed by `presence.rs` and dropped by `check.rs:230`, with `archogen-evidence` wired to `rt-analysis` only, and no severity for it to live in because §4 rule 1 forbids warnings. Then `M1.26.1` — **medium**: gap (a) of `M1.26`, which is decomposed into two children. **16** of the workspace's **76** distinct diagnostic codes are stated by no normative document — the model layer's 15 plus `analysis-inconclusive` from `crates/archogen-s0/src/interpret.rs` — and the book renders **8** of them, so leg 8 checks those for *existence* only. Lands a second normative document with its own declaration and a both-directions census, widens leg 8 to read every normative document's declaration, answers **F-I** (the frozen population is scoped by *file*, and nothing says why), and now also carries the rule `M1.28.2` left enforced but unstated: `<number> <symbol>` is a quantity, a lone number is a count. Then `M1.26.2` (gap (b): §4's `fires on` column, all 60 rows in one commit, plus **F-H** — a §4 row naming a mechanism `grep` cannot find in the elaborator), then `.10`, `.21`, `.22`, `.27`. ⭐ **`M1.28` is closed** (`2026-09-29`, both children): `archogen check` and `archogen build` agree about a malformed quantity, because the schema can now say a clause holds a **quantity** (`ValueType::Quantity`, the one value type that consumes two forms, with four `task` clauses moved onto it) and the refinement pass **propagates** what it finds instead of discarding it. `.1` put a rule that had three consumers and two answers into one accessor — `Verdict::of_code`, so a code that is not a §5.5 verdict slug is a malformed description and not a tool failure, exit **70 → 10** — and `.2` made F03 enforced at the level §13.1's gate reads. Measured, not asserted: a census over all **76** tracked descriptions before and after finds **1** changed (F-K's case, from a clause-name typo to the zero frequency its header claimed) and **0** acceptances lost; 14 new legs; seven mutations seen firing; a migration note naming five constructs and the baseline re-emitted **70 → 72**; **559 passed / 0 failed** over 39 suites. ⛔ Two findings out of it, both owned: **F-N** — `quantities.md` claimed "a number always carries a unit" while `docs/semantics/boundary/accept/counter-width-and-rate.eadl` shipped a bare number as an accepted case, and the discard is why nobody noticed; **F-O** — `LANGUAGE-FREEZE`'s explicitness leg prints `OK` on the real tree with an amended baseline and **no note at all**, because its notes grep matches the directory's own README and its `*all*` case matches that README's template, filed as `PROGRAM.27` | repo-local |
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
