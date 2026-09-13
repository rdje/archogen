# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. The
`bedrock-scaffold` entries below the separator are the provenance of the discipline spine
this repository was created from, not archogen's own history.

## archogen — `rt-static-up-v1` as checked data, not prose

`ARCHOGEN-M0-0005` (leaf `M0.4`).

- New crate `crates/eadl-model` — the §4.2 home for typed declarations, units, contract IDs
  and profile definitions. It holds `rt-static-up-v1`: 13 concern decisions and 18 named
  exclusions, each carrying the obligation admitting it would add.
- §3.1 requires an out-of-profile request to be refused *by name* rather than silently
  weakened. That is only enforceable if the engine holds the list; before this it was prose in
  the roadmap (`git grep -ln 'rt-static-up-v1' -- crates/` → no match).
- `docs/profiles/rt-static-up-v1.md` is the published form, and a test fails if it drifts.
  The gate was proven with a RED arm: misspelling `posix` as `posiks` on the page →
  `docs/profiles/rt-static-up-v1.md has drifted from the profile data`, 1 failed.
- An unknown capability is not admitted by silence — a known exclusion is refused by name,
  an unknown one is a `missing-fact`.
- New book chapter `docs/book/src/profile.md`.
- Validation: fmt clean; clippy -D warnings clean; `cargo test --all` → 34 passed, 0 failed;
  all doctrines green; `mdbook build` OK.

## archogen — the controlling eADL/engine boundary

`ARCHOGEN-M0-0004` (leaf `M0.1`).

- Recorded `docs/decisions/decision_eadl-engine-boundary.md`: eADL describes functionality and
  contains no implementation. This is the precedence rule — it overrides ambiguous wording in
  any source draft and any later convenience argument.
- The record fixes the three classification tests (externality, implementation-independence,
  non-prescription), all eight worked cases of `ROADMAP.md` §4.3 with both sides named, the
  direction of every obligation (a declared capability is not evidence; a refinement is an
  obligation to check), and what the boundary rules out — including `defkind` becoming a
  template language, and implementation syntax as an escape hatch for missing engine support.
- New book chapter `docs/book/src/boundary.md`, because this is the concept a reader must
  have before any other chapter makes sense.
- Validation: 8/8 §4.3 rows present; layer-C index in sync
  (`scripts/check_memory_architecture.sh`, `rc=0`); `mdbook build` OK; all doctrines green.

## archogen — the `osgen` command-line shell

`ARCHOGEN-PROGRAM-0003` (leaf `PROGRAM.2`).

- Replaced the bedrock starter crate with `crates/osgen-cli` — library `osgen_cli` plus the
  `osgen` binary. The workspace now has a real entry point.
- The `ROADMAP.md` §10.2 command surface (`check`, `resolve`, `build`, `analyze`, `verify`,
  `explain`, `replay`) is declared **once, as data**; `--help` renders from that table and the
  parser validates against it, so a documented option is always an accepted option.
- The §5.5 outcome vocabulary is implemented as a stable exit-code contract, with diagnostic
  results (`invalid-description` … `tool-failure`) kept separate from process-level statuses
  (`ok`, `usage`, `unimplemented`). Every refusal carries a concrete repair direction, as §5.5
  requires.
- Every command is unimplemented and says so precisely, naming the task-tree leaf that owns
  building it and exiting 20 — a gap in this toolchain is tracked work, not an unknown.
- Zero external dependencies, recorded as a decision
  (`docs/decisions/decision_zero-dependency-engine-core.md`): §4.4 trust inventories, §10.3
  locked offline builds, §5.5 diagnostic wording.
- New book chapter: `docs/book/src/cli.md`.
- Validation: `cargo fmt --check` clean; `clippy -D warnings` clean; `cargo test --all` →
  28 passed, 0 failed; `scripts/check_doctrines.sh` → all doctrines green; `mdbook build` OK.

## archogen — roadmap seeded into task-trees

`ARCHOGEN-PROGRAM-0002` (leaf `PROGRAM.1`).

- `ROADMAP.md` revision 2.0 (eADL and OS Generation — Consolidated Roadmap) adopted as the
  project's direction and converted, in full, into ten task-trees: `PROGRAM` for the
  cross-cutting engineering substrate and one tree per roadmap milestone (`M0`, `S0`,
  `M1`–`M7`).
- Every roadmap unit (§11–§20) and every mandatory fixture F01–F30 now names an owning tree
  and leaf; the two coverage maps live in `docs/tasks/PROGRAM.md` so "where does roadmap
  item X live?" has a single mechanical answer.
- `README.md` rewritten from the template landing page to archogen's, within the
  `README-STABILITY` caps (77/300 lines, 3 637/16 384 bytes).
- Live docs brought into lockstep: `MEMORY.md` resume pointer, `LIVE_STATUS.md` (twelve
  rows, one per tree plus the spine), `docs/TASK_TREE.md` index, the derived Knowledge Map,
  and the book introduction.
- Validation: `scripts/check_doctrines.sh` → `=== all doctrines green ===`, 13/13.

---

## Provenance — the `bedrock` discipline spine


## bedrock-scaffold 0.6.1 — creating a project is foolproof through its first commit

`BEDROCK-MAINTENANCE.2.7`.

- ⛔ **Measured on a fresh clone of 0.6.0:** `bootstrap.sh` left the crate rename — a CODE change — with no owning
  leaf, so the new project's FIRST commit was refused by `TASK-TREE-OWNERSHIP` and `TASK-ACCEPTANCE`. A new user's
  first contact with the discipline was a refusal about a rename the tool made.
- **`bootstrap.sh` now seeds `docs/tasks/BOOTSTRAP.md`** on a fresh de-template: a done leaf that owns the bootstrap,
  its ticked checklist carrying the evidence of that very run (crate-name count before/after, hooks path, the
  enforcer's summary and verdict with `rc=0`), registered in `docs/TASK_TREE.md`, pointed to by `MEMORY.md`; and it
  prints the exact first-commit command as step 0. Idempotent.
- Proven: clone → `bootstrap.sh <name>` → the printed commit → hooks green → `make gate` green → `make check` green,
  with no hand edits. Two defects in the fix were caught by the trial itself (an enforcer run before the map
  existed; a `grep -c` fallback that split a checklist bullet).

## bedrock-scaffold 0.6.0 — four evidence and ratchet doctrines: lessons reach the retrievable layer, routings carry evidence, gap claims carry their census, tables keep their columns

`BEDROCK-MAINTENANCE.2.6`.

- **Added `LESSON-PROMOTION`**: a new dated lesson heading staged in `DEV_NOTES.md` must be promoted (a
  `docs/knowledge/` change or a `docs/decisions/` record gaining `answers:`) or explicitly declined
  (`promotion: declined (<reason>)` in the owning leaf). Pure verdict with 9 controls at import.
- **Added `ROUTING-EVIDENCE`**: a leaf that routes a finding out to another tree carries a `ROUTING EVIDENCE`
  section. Keyed on the semantics of leaving the tree; 5-arm `--self-test`.
- **Added `GAP-CLAIM-CENSUS`**: a leaf that ADDS a "nothing checks X" claim records the census it rests on in
  the same section (or `census: not run (<why>)`). Staged-diff-scoped; `--all` reports the backlog; 10-arm
  `--self-test` pinning the founding active and passive sentences.
- **Added `TABLE-ARITY-RATCHET`** (a fresh minimal implementation): a staged `.md` may not raise the number of
  table rows whose cell count disagrees with their header; code spans and escaped pipes respected; 8-arm
  `--self-test`.
- ⛔ Two defects in the ports were caught by their own RED arms before the gate ran: a heredoc that consumed
  the table detector's stdin (every arm read 0), and a `pipefail` control in lesson promotion.
- All four scripts join the `NEUTRAL` allow-list of `scripts/update_scaffold.sh`. Backlog notes record the
  input-bound principles (`BASELINE-IDENTITY`, `IDENTITY-CARRIER-CURRENCY`, `SCRATCH-SLOT-HEADER`, the full
  `LIVE-DOC-CURRENCY` instrument) for a future seam.

## bedrock-scaffold 0.5.0 — the day-one batch: no agent trailers, a handoff census, no self-reported dates

`BEDROCK-MAINTENANCE.2.5`.

- ⛔ **`COMMIT.md` had the trailer rule backwards.** It told every generated project to *end commit
  messages with the project's co-authorship trailer*; the upstream maintainer ruled the opposite on
  2026-08-22 (a commit message ends with its own last line — no agent/tool attribution trailers,
  harness-agnostic). The rule is rewritten and `.githooks/commit-msg` now refuses the known
  agent-attribution shapes mechanically; a human co-author's `Co-Authored-By:` still passes.
- **Added `scripts/check_no_background_jobs.sh`**, the handoff census: pattern-free (`lsof` over the
  caller's uid — an open handle under the repo, or a command line naming the checkout), run before
  a session ends; deliberately not a commit gate. Named in `CLAUDE.md`'s non-negotiables.
- **Added the `LIVE-DOC-CURRENCY` doctrine** (principle): no tracked `.md` reports its own currency
  (`Last updated:` and kin) — git carries it, a hand-kept date is false the day after. The field is
  deleted from `docs/tasks/TEMPLATE.md` and the maintenance tree; `scripts/check_live_doc_currency.sh`
  is structural over `git ls-files '*.md'` with a 3-arm `--self-test`.
- Both scripts join the `NEUTRAL` allow-list of `scripts/update_scaffold.sh`.
- Part 2 of the same transfer (`LESSON-PROMOTION`, `ROUTING-EVIDENCE`, `GAP-CLAIM-CENSUS`, a fresh
  `TABLE-ARITY-RATCHET`) is classified in the `.2.5` leaf and queued as `.2.6`, paused by the maintainer.

## bedrock-scaffold 0.4.0 — TASK-ACCEPTANCE: a change lands with evidence, not with a claim

`BEDROCK-MAINTENANCE.2.4`.

- **Added the `TASK-ACCEPTANCE` doctrine**: a staged CODE change must be owned by a task-tree leaf
  whose checklist has ROOT CAUSE / ADDRESSED / NO REGRESSION **ticked**, each backed by output from
  a tool that was actually run — **inside that box's own bullet**.
- ⭐⭐ **Box-scoping is the soundness property**, not a nicety. It closes two measured leakage
  holes: a co-staged, unrelated leaf supplying the evidence, and a token matched anywhere in the
  file rather than in the box it backs. `CTRL-1` demonstrates it directly — a whole-file grep
  PASSES the fixture that the shipped check REJECTS.
- **Neutral by seam, not by rename.** Default signatures are universal to any Rust project
  (`error[E1234]`, `could not compile`, `clippy::…`, `test result: ok`, panics, profilers) plus any
  project's build-flow forensics (`git log -S`, `shellcheck`, `bash -n`, `make -n`, `ENOSPC`…).
  Project-specific tooling is declared in `.doctrine/evidence_tokens.txt`, and what counts as a
  code change in `.doctrine/code_paths.txt` — both optional, both defaulted, both documented in
  `.doctrine/README.md`. ⭐ `CTRL-4`/`CTRL-4b` prove the seam is load-bearing: the same leaf passes
  WITH the declaration and fails WITHOUT it.
- ⛔ **Fixed a portability defect the probes caught**: the box extractor used `IGNORECASE`, a gawk
  extension that BSD awk silently ignores — every leaf would have been reported as having no
  checklist. Rewritten with POSIX `tolower()`.
- ⚠️ Honest limit, stated in the check itself: it proves the author cited something re-runnable,
  never that the output is true. The un-fakeable leg is re-running the cited command in CI.
- Probes 9/0; `make gate` 8/8.

## unreleased — the admission test asks about VALUE first, not vocabulary

`BEDROCK-MAINTENANCE.2.3`. Process only; no check changed, so `DOCTRINE_VERSION` is unmoved
(`MAINTAINING.md` and the maintenance tree are maintainer-only, not re-syncable spine files).

- **The admission test is now two ordered questions.** Q1 (primary, about VALUE): *does this
  objectively benefit any present and any future project?* — answered by stating what the check
  prevents using no project's nouns, then asking whether a brand-new project is better off with it
  on day one. Q2 (secondary, a filter): *can it be expressed without domain nouns?*
- ⛔ **Q2 cannot substitute for Q1.** A check can score 0 domain nouns and still encode a workflow
  only one project needs — neutral vocabulary, project-shaped substance. Q2 measures whether a
  thing CAN be neutralized; Q1 asks whether it SHOULD be. Running Q2 first waves impostors through.
- ⭐ **Measured worked example, which changed a verdict.** A "destructive automation must require
  confirmation" check scored well on Q2 and was ranked an easy win; its logic hardcodes a Makefile
  path and a `clean:` recipe, so it really offers *"benefits any project that builds with make"* —
  a conditional. **Rejected as-is.** Meanwhile `ROUTING-EVIDENCE` measures 0 build-system
  references and presumes only the task-tree system this template ships ⇒ promoted to top.
- **The portability seam to look for:** does the check presume anything beyond what bedrock ships?
  If yes, give it a project-declared seam or leave it upstream — never hardcode one project's
  answer and call it neutral.
- ✅ Retroactive audit: all four already-ported items PASS Q1. Nothing retracted.

## bedrock-scaffold 0.3.0 — WAIVER-ROUTING, and the neutrality bar for every future port

`BEDROCK-MAINTENANCE.2.2`.

- **Added the `WAIVER-ROUTING` doctrine** (`scripts/check_waiver_routing.sh`): a task leaf saying a
  gate DOES NOT APPLY must name the leaf that owns fixing the gate. ⭐ An author writing a waiver
  IS the gate reporting a missing capability — the highest-signal defect report a gate can get.
  Deliberately does **not** punish honesty: the waiver stays legal, it just has to name an owner.
- **Chosen by measurement.** All 15 upstream doctrines were classified by domain-dependence of
  their LOGIC (comments stripped). `WAIVER-ROUTING` scored **0** — portable essentially unchanged.
  The ranked remainder is now a frontier in `docs/tasks/BEDROCK-MAINTENANCE.md`, not a wish list.
- ⭐⭐ **The port FIXED a defect rather than inheriting one**: the origin's `printf … | grep -q …
  || continue` returns failure ON SUCCESS past the pipe buffer under `pipefail`, silently SKIPPING
  the file — a **fail-open**. Both sites here read a file instead. Threshold measured, not assumed:
  65,606 B → no SIGPIPE; 131,139 B → SIGPIPE.
- **Wrote down the neutrality bar** (`MAINTAINING.md`): every doctrine here must be objectively
  applicable to ANY project, with a measurable admission test and its honest bound — plus the rule
  that **transfer runs both ways**, after this repo's layer-C check turned out to be stronger than
  the reference deployment's.
- Probes 5/0; `make gate` 7/7; added to the `update_scaffold.sh` NEUTRAL allow-list.

## bedrock-scaffold 0.2.0 — README Stability Policy + a layer-A byte cap

`BEDROCK-MAINTENANCE.2.1`. Transferred from the reference deployment by maintainer order.

- **Added `README_POLICY.md`** (project-neutral, verbatim) — keeps `README.md` a stable landing
  page instead of a changelog/roadmap/catalogue, and states the caps rule.
- **Added the `README-STABILITY` doctrine** (`scripts/check_readme_stability.sh`): a line cap
  AND a byte cap, a dated-line (release-history) tripwire, and a required link back to the
  policy. Non-mutating; REFUSES (exit 2) rather than passing when the README or policy is
  absent. Template defaults 300 lines / 16384 bytes — generous on purpose, because they ship to
  a project whose README is not this one; tighten after your own trim.
- ⛔ **Closed a bypass the spine was itself shipping.** `scripts/check_memory_architecture.sh`
  capped layer-A `MEMORY.md` by LINES only (cap 120, no byte bound), exactly as
  `MEMORY_ARCHITECTURE.md` §9's reference check prescribed — so **every adopting project
  inherited a bound that does not bind.** Measured on a real project running this spine:
  60 lines (passing, exactly at its cap) carrying **138,403 bytes** — 2,306 B/line, one line of
  18,816 B. Now both caps, in the check **and** in the standard (§6 / §9 / §9.1).
  Layer-A caps: **50 lines** (tightened from 120, to match the "≤ ~50 lines" §6 already stated)
  and **7168 bytes**. Both env-overridable.
- Both new files added to the `update_scaffold.sh` NEUTRAL allow-list, so existing projects
  pull them with `scripts/update_scaffold.sh <bedrock-url>`.
- Verified: `make gate` 6/6 green; a 13-line / 19,304-byte fixture is REJECTED by the byte cap
  while being well under the line cap; the **retired** layer-A guard PASSES that same file
  (exit 0) — the change is proven necessary by execution, not by argument.

Changelog-style summary of completed work + its validation (internal continuity surface;
the immutable audit trail proper is `git log` — memory layer D). Newest first.

## _(YYYY-MM-DD)_ — bootstrap

Instantiated from the `bedrock` discipline-spine template. Next: replace `ROADMAP.md` and
seed the first task-tree.
