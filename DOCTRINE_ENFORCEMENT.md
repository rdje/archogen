# DOCTRINE_ENFORCEMENT.md — how every mechanizable doctrine is enforced

Discipline holds because it is **mechanical**, not remembered. A rule that lives only in a
doc is a suggestion; a rule wired into a git hook + CI is enforced for every agent and
every human, identically.

## Defense in depth (four layers)

- **E1 — discovery.** The doctrine docs: this file, `README.md`, `MEMORY_ARCHITECTURE.md`,
  `TOOLBOX.md`, `COMMIT.md`, `docs/CLAIM_VERIFICATION.md`, and `docs/decisions/`. Where an agent
  learns the rules. ⭐ This file asks *"is this rule enforced?"*; `docs/CLAIM_VERIFICATION.md` asks
  *"is this number earned?"* — and that question comes first, because a gate built on an unverified
  measurement enforces the wrong thing precisely and forever.
- **E2 — self-check.** `scripts/check_doctrines.sh` (the driver) + each registered
  `scripts/check_*.sh`. The single source of truth for "which doctrine is enforced by
  what". Runnable by hand anytime.
- **E3 — git hook.** `.githooks/pre-commit` calls the enforcer; `.githooks/commit-msg`
  checks the subject shape. Activate once per clone: `git config core.hooksPath .githooks`.
- **E4 — CI.** The same enforcer runs in CI (`.github/workflows/doctrines.yml`), so a
  locally `--no-verify`'d hook still fails the build. This is the "no matter what" backstop.

## The enforcer registry

`scripts/check_doctrines.sh` carries the **universal** registry:

| ID | Proves | Check |
| --- | --- | --- |
| `MEMORY-ARCH` | the durable 4-layer memory invariants hold | `scripts/check_memory_architecture.sh` |
| `DOCPATH` | tracked `.md` carry no checkout-specific absolute paths | `scripts/check_docpaths.sh` |
| `TASK-TREE-OWNERSHIP` | every staged code change is owned by a task-tree leaf | `scripts/check_task_tree_ownership.sh` |
| `TASK-ACCEPTANCE` | a staged **code** change is owned by a task-tree leaf whose acceptance checklist has ROOT CAUSE / ADDRESSED / NO REGRESSION **ticked**, each backed by tool output **inside that box's own bullet** — and the leaf verified is the one that **owns** the change. ⭐ Scoping is the soundness property, not a nicety: it closes three measured leakage holes — a co-staged unrelated *file* supplying the evidence, a token matched anywhere in the *leaf* rather than in the box, and a co-staged unrelated *leaf* in the same file supplying the boxes (the last one printed `OK` on two real commits having read a checklist written thirteen days earlier for a different leaf). ⛔ The owner is taken from `TASK_ACCEPTANCE_LEAF` or the `(leaf <ID>)` token in the pending message's subject, **never inferred from the staged paths** — measured, the leaf sections a commit touches name the right leaf once in seven — and when it cannot be identified the check **refuses** rather than reading the first checklist in the file, because a green verdict about a leaf nobody claimed is worse than no verdict. ⚠️ Honest limit: it proves the author cited something re-runnable, never that the output is true — the un-fakeable leg is re-running the cited command in CI. Project seams in `.doctrine/` keep it neutral (`--self-test` runs nine RED arms) | `scripts/check_task_acceptance.sh` |
| `WAIVER-ROUTING` | a task leaf saying a gate **does not apply** names the leaf that owns fixing it — ⭐ *an author writing a waiver IS the gate reporting a missing capability*, the highest-signal defect report a gate can receive. Deliberately does **not** punish honesty: the waiver stays legal, it just has to name an owner | `scripts/check_waiver_routing.sh` |
| `README-STABILITY` | `README.md` stays a stable landing page — a **line cap AND a byte cap**, because a line cap alone is measurably bypassable (a real project running this spine passed its 60-line layer-A cap while carrying 138,403 bytes) | `scripts/check_readme_stability.sh` |
| `LIVE-DOC-CURRENCY` | no tracked document reports its own currency (`Last updated:` and kin) — git already carries it, and a hand-kept date is right the day it is typed and false the day after; the upstream instrument that scores distinct dates per live surface against a declared charter is a backlog item | `scripts/check_live_doc_currency.sh` |
| `LESSON-PROMOTION` | a NEW dated lesson heading staged in `DEV_NOTES.md` must be either **promoted** (a `docs/knowledge/` change, or a `docs/decisions/` record gaining `answers:`) or **explicitly declined** (`promotion: declined (<reason>)` in the owning leaf) — never silently dropped. Founding measurement upstream: 1 592 lesson entries, none reachable by question, because no gate asked. Evidence archetype: it verifies a decision was RECORDED, not that it was right | `scripts/check_lesson_promotion.sh` |
| `ROUTING-EVIDENCE` | a task leaf that routes a finding **out to another tree** carries a `ROUTING EVIDENCE` section: does the finding reproduce OUTSIDE the family it is sent to, what was measured, what would make the routing wrong. Keyed on the semantics of leaving the tree (the first cut upstream, keyed on a tree-ID spelling, missed its own founding incident); intra-tree routing is not flagged | `scripts/check_routing_evidence.sh` |
| `GAP-CLAIM-CENSUS` | a task leaf that **ADDS** a *"nothing checks X"* claim records the CENSUS it rests on in the same heading section (a command that enumerates a population, or `census: not run (<why>)`). Such a sentence is a universally quantified claim over the whole tree, false the moment one reader exists; staged-diff-scoped (81 pre-existing claims upstream would otherwise teach bypass); `--all` reports the backlog, advisory | `scripts/check_gap_claims.sh` |
| `TABLE-ARITY-RATCHET` | a staged `.md` may not RAISE the number of table rows whose cell count disagrees with their header — GFM silently DROPS extra cells and PADS missing ones, so the page looks fine and the reader loses the rightmost column (26 of 197 rows of a shipped contract upstream, every enforcer green). Per-file ratchet against HEAD; code spans and escaped pipes respected; a fresh minimal implementation with an 8-arm `--self-test` | `scripts/check_table_arity.sh` |
| `KNOWLEDGE-MAP` | the derived Knowledge Map is in sync (if the subsystem exists) | `knowledge-map/scripts/check_knowledge_map.sh` |
| `PROJECT-SPECIFIC` | this project's own doctrines | `scripts/check_doctrines.project.sh` |

**Project-specific doctrines go in `scripts/check_doctrines.project.sh`** (the pluggable
slot) — never in the universal driver. That is where a project adds the equivalent of its
own build gates, format checks, invariant proofs, etc.

### archogen's own doctrines

The human-readable mirror of the `PROJECT_DOCTRINES` array in
`scripts/check_doctrines.project.sh`:

| ID | Proves | Check |
| --- | --- | --- |
| `FROZEN-EVALUATION` | the sealed evaluation set (`ROADMAP.md` §12 M0, §16) stays **sealed** and stays **unseen**: every sealed file still hashes to its seal-time digest, nothing is added or removed unlisted, and ⭐ no tracked file outside the sealed directory names a sealed case — the leg that actually protects the measurement, since a case discussed in a task tree or design note is no longer unused. ⚠️ Honest limit: it cannot prove nobody *read* them; it raises the cost of accidental contamination, which is the common failure | `scripts/check_frozen_evaluation.sh` |
| `S0-RETIREMENT` | a prototype with a stated expiry cannot quietly become permanent (`ROADMAP.md` §12 S0: "no hidden special-case generator is grandfathered into the release"). Three legs: every `S0-ASSUMPTION: <id>` marker in the source is **listed** in `docs/decisions/decision_s0-retirement.md`; every listed assumption **names a leaf some tree declares** — "removed later" is not an owner; and ⭐ the prototype crate has acquired **no consumers** beyond its declared ones, which is what grandfathering actually looks like in practice. ⚠️ Honest limit: it cannot force an author to *write* a marker — it makes an unmarked assumption a thing someone chose not to record rather than one nobody noticed. Three RED arms in `--self-test` | `scripts/check_s0_retirement.sh` |
| `BOOK-ANCHORS` | prose that describes the code cites code that exists — over **two populations**: the mdBook's chapters, and the normative documents under `docs/semantics/`. ⭐ The book is the project's public surface and, for its director, the **only** window into it — the code is not read, the book is — which makes a confident chapter describing something the engine no longer does the most expensive drift here. The normative documents are the same failure one layer down and in the other direction: they are the authority the code is held to, so a rotted citation in one sends an implementer to nothing. Two legs per population: every behavior document **cites** a repository path (one chapter cited nothing when this was written), and every cited path **exists**; an empty normative population is a breach rather than a "not applicable", because the frontend `include_str!`s the reference and cannot compile without it. Matched on a repository path rather than any filename, so writing about a *generated* `src/main.rs` costs nothing. ⚠️ Honest limit: it proves a document points somewhere real, never that what it says there is true — and that residue is owned, not left: `reference.rs`'s leg 8 compares the diagnostics the book *renders* against the codes the engine emits. Six RED arms in `--self-test`, three per population | `scripts/check_book_anchors.sh` |
| `FEEDBACK-SELF-CONTAINED` | a bug report archogen sends upstream is read inside **someone else's** project, so each issue directory under `docs/feedback/<vendor>/issues/` must stand alone. Four legs: **complete** (`README.md`, `SETUP.md`, an executable `repro.sh`, a non-empty `evidence/`), **closed** (no file references a path outside the directory — the leg that actually matters, since a `../../SETUP.md` reads fine here and breaks the moment the directory is handed over), **portable** (no checkout-specific absolute path in any frozen observation), and **registered** (the vendor's `INDEX.md` names it, so a bug cannot exist unlisted). ⭐ It caught five real escapes in the reproducers on its first run. ⚠️ Honest limit: it proves a directory is closed and complete, never that its reproducer reproduces anything — the exit-code contract inside each issue carries that. Six RED arms in `--self-test` | `scripts/check_feedback_self_contained.sh` |
| `LANGUAGE-FREEZE` | no construct of `eadl/1` moves without a migration note (`ROADMAP.md` §15: "any changed behavior must be explicit"; §12 M1's frozen compatibility baseline). `docs/semantics/BASELINE.txt` digests every frozen construct — the grammar's EBNF fence, every machine-read table in the reference, and the **canonical form** of every description the manifest declares — and this check recomputes it. **Three legs**: *integrity*, the tracked baseline agrees with a fresh run over the working tree, which catches a construct edited and the baseline left alone; *explicitness*, the tracked baseline agrees with `HEAD`'s **or** a pending note in `docs/semantics/migrations/` names every construct the amendment moves, which catches the other direction — a baseline regenerated because something moved, with nothing written down; and *spent notes*, a note `HEAD` already carries as pending covers nothing and is refused until it says `applied`, so a note is a permission for exactly one commit. ⛔ **The explicitness leg could not fail on the real tree until `PROGRAM.27`** (`2026-09-29`): the directory's own README, whose form template reads `- status: pending \| applied` and `constructs: … — or: all`, was read as a pending note covering every construct, because the status test was unanchored and the constructs test a substring. A pending note is now a line that is *exactly* `- status: pending`, and `constructs:` a list compared exactly. ⛔ Any movement needs a note, *including a correction*, because a gate that tried to tell a bug fix from a language change would need judgement it cannot have. ⚠️ Honest limit: a digest proves a construct moved, never that the note covering it is correct or complete; and canonical form carries no comment, so a corpus file's header is invisible here. RED arms in `--self-test`, each required to name the construct or note its refusal is about — two passed for the wrong reason before that — and two of them against the real tree: a description edited and restored byte-identically, and an amendment the **deployed** notes directory must leave uncovered | `scripts/check_language_freeze.sh` |

## Adding a doctrine

1. Write `scripts/check_<name>.sh` — cheap, deterministic, self-describing; exit nonzero
   with a one-line stderr message on breach. Keep it fast (heavy proofs belong in CI).
2. Register it — universal → the `DOCTRINES` array in the driver; project → append it to
   `scripts/check_doctrines.project.sh`.
3. Mirror it in the table above (this file is the human-readable mirror of the registry).

## The task-acceptance checklist (every code-change leaf must pass)

A code change cannot commit until its owning task-tree leaf records all six:

- [ ] **REPRODUCE / ISSUE** — the problem, shown (not asserted).
- [ ] **ROOT CAUSE (WHY + WHERE)** — tool-backed and pinpointed (`TOOLBOX.md`).
- [ ] **FIX** — the change, made at the lowest-risk level that actually works.
- [ ] **ADDRESSED (verified)** — measured before→after (the global metric where one exists).
- [ ] **NO REGRESSION** — the guard set stays green; state how you proved it.
- [ ] **LOCKSTEP** — live docs (`MEMORY.md`, `CHANGELOG.md`, `DEV_NOTES.md`,
  `LIVE_STATUS.md`), the book, and any trackers updated in the SAME commit.
