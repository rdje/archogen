# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. The
`bedrock-scaffold` entries below the separator are the provenance of the discipline spine
this repository was created from, not archogen's own history.

## archogen — the `os/rt` workload module, and a measured gap closed

`ARCHOGEN-M1-0018` (leaf `M1.7`).

- `docs/semantics/kinds/os-rt.eadl` declares the `task` kind — release model, deadline with its
  reference event, bounded jitter, static unique priority, functional needs, overrun response —
  using the same `defkind` primitive as everything else.
- ⭐ **The 10-of-11 schema gap `M1.2` measured is now 13 of 13.** `Holds::Kind` makes a clause
  validate recursively, so `(clause task … (holds kind task))` lets the schema see inside a task.
  `execution-bound` is now caught by the schema *and* the boundary classifier, and the refusal
  carries the boundary's wording — `wcet` is not a typo, it is content in the wrong layer. The
  pinned assertion was replaced, not loosened: it now asserts the out-of-reach set is **empty**.
- A registry that has not loaded `os-rt.eadl` **says so**. Silently accepting whatever is inside
  an unvalidatable clause is the failure mode that let the gap exist in the first place.
- `entry-point` and `stack-allocation` join the forbidden registry, each with a worked
  ambiguous case — both *feel* like task properties and both belong elsewhere. The corpus grew
  from 21 to 23 cases and every census assertion with it.
- `examples/` holds the three M0 use cases as real descriptions, checked by a 7-arm suite: they
  validate, every task has an arrival model and a deadline with its reference event, priorities
  are unique, and **none carries an execution bound, a code reference, or an allocation** —
  asserted by the classifier rather than by anyone remembering. `uc3` is additionally checked for
  still containing its own contradiction, because the tempting "fix" would make the case go quiet.
- ⭐ **Three defects were found by running the new tests.** (a) `defblock` had no `absent`
  clause: §5.3's three presence states were implemented in the checker and missing from the
  language. (b) A composition reference to a declared block was reported as `missing-fact`;
  a `uses` target that is declared is satisfied by its declaration, and the absence check was
  reordered so that being declared does not repeal an explicit absence. (c) `diagnose | head -3`
  panicked with `Broken pipe` — a closed pipe is not a failure.
- New book chapter `docs/book/src/workload.md`.
- Validation: 234 tests across 17 suites, 0 failed; fmt and clippy clean; all doctrines green;
  `mdbook build` OK.

## archogen — refinement as an obligation to check: F07

`ARCHOGEN-M1-0017` (leaf `M1.6`).

- `crates/eadl-model/src/refinement.rs`: three obligations — **guarantee**, **constraint**,
  **exclusion** — and a violation names which one broke. §5.1.1: a refinement declaration is "an
  obligation to check, not permission to trust a claim blindly".
- ⭐ **Two roadmap sentences pull against each other here, and both directions are tested.**
  §5.1.1 demands the obligations be checked; §5.3 says "adding an unused device is not
  automatically an invalid refinement". A checker that demanded equality passes the exclusion arm
  and rejects every real refinement; one that allowed any addition passes the addition arm and
  silently drops the exclusion obligation — which is the one that matters most, because an
  abstract description declares a fact absent *because* something depends on its absence.
- Additions are reported even though they are allowed: allowed is not the same as invisible, and
  the author should be able to see what grew.
- An abstract description states **bounds with a direction**, not bare values. The same numeric
  relationship satisfies `(at-least 32 bit)` and violates `(at-most 50 us)`, so a bare value
  would leave the checker guessing. `exactly` refuses a merely "better" value but accepts the
  same amount written differently, because the comparison is on the amount, not the spelling.
- A bound checked against the wrong dimension is a type error, not a `false` — reusing F03's
  refusal.
- New book chapter `docs/book/src/refinement.md`.
- Validation: 15 new arms, 223 tests across the workspace, 0 failed; fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — presence, relevance, and the closure: F04, F05, F06

`ARCHOGEN-M1-0016` (leaf `M1.5`).

- `crates/eadl-model/src/presence.rs`: offered / explicitly absent / undescribed, and the
  transitive dependency closure that decides which unknowns matter.
- ⭐ **F04 and F05 are run against the same description**, deliberately. They are two halves of
  one decision: §2's correction table revised "unknown capability anywhere blocks generation"
  into "required facts in the selected dependency closure must be known", so that "irrelevant
  unknown facts do not invalidate unrelated systems". A checker that always blocks passes F04
  and fails F05; one that never blocks passes F05 and fails F04. Two separate descriptions could
  have been passed by two different bugs.
- **Absent is not undescribed.** A required absent fact is `infeasible-configuration` — "a
  definite answer, not a gap to be filled in" — while a required undescribed one is
  `missing-fact`. Collapsing them would send an author to describe something the platform has
  already said it does not have.
- A contradiction is `invalid-description` whether or not it is reachable, and names **both**
  sites: §5.3 qualifies the unknown-fact rule by relevance and states the contradiction rule
  without a qualifier. It is never resolved by preferring one side, because only the author knows
  which half was meant.
- A `needs` edge nested inside a `requires` clause is still followed. A closure that only read
  top-level clauses would miss most real descriptions and would fail **open**.
- `offered` remains a claim, not evidence: nothing in this analysis says a platform really has a
  capability.
- New book chapter `docs/book/src/presence.md`.
- Validation: 14 new arms, 208 tests across the workspace, 0 failed; fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — modules and instances, F01 and F02

`ARCHOGEN-M1-0015` (leaf `M1.4`).

- `crates/eadl-front/src/module.rs`: namespaced imports, explicit exports, typed parameters with
  defaults, version constraints, and elaboration into a program.
- ⭐ **Elaboration produces instances, not modules.** §5.1.1 requires instantiating a module more
  than once "without sharing mutable elaboration state", so importing `hw.timer` twice yields two
  instances with independent bindings and independent qualified names. A cache keyed on module
  name would have been smaller and would have silently made the second import a no-op — a system
  with two timers would have had one, and nothing would have said so.
- Names carry their whole alias path (`platform.timer.timer.counter`), and instances come out in
  dependency order, children before parents.
- **F01 and F02 green, 18 arms.** A cycle reports its whole chain — `circular import: a → b → c →
  a` — because "there is a cycle" is a puzzle and the chain is a diagnostic. A duplicated export
  and a duplicated alias each name **both** sites, since the author looking at one cannot see the
  other. Also refused: a dangling export, a name mismatch, a missing required parameter, an
  unknown parameter (listing the real ones), and an unversioned module.
- A **major** version difference is never satisfied, however much newer the module is. "Newer" is
  not "compatible", and §15 keeps a locked description's meaning, which a major bump is defined
  not to preserve.
- A cycle is the only failure that stops elaboration; everything else is collected, so three
  problems cost one edit cycle.
- ⭐ A parser defect was caught by the fixture, not by inspection: the import version clause was
  read flat while the syntax is nested, so every versioned import in the valid composition was
  refused — five of eighteen arms red, including the F01 happy path. The nested shape was kept
  rather than flattened to match the bug, because the relation being its own form leaves room for
  `at-most` and `exactly`.
- New book chapter `docs/book/src/modules.md`.
- Validation: 18 new arms, 194 tests across the workspace, 0 failed; fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — exact quantities, and F03

`ARCHOGEN-M1-0014` (leaf `M1.3`).

- `src/rational.rs`: exact `i128/i128` rationals, normalized on construction so equality and
  ordering are structural, with **every** operation checked. §7.4 requires overflow detection,
  and a wrapped numerator turns an unschedulable system into a schedulable-looking one with
  nothing in the output to say so. `ceil` rounds up, because that is the direction response-time
  analysis needs — rounding the other way understates interference and turns a missed deadline
  into a reported pass.
- No floating point anywhere. `1/3 + 1/3 + 1/3` is exactly `1`; `1 ns` in milliseconds is
  exactly `1/1000000`, not zero; values with no short decimal form print as fractions rather
  than being rounded into something that reads like a measurement.
- `src/quantity.rs`: 13 units over four dimensions, exact conversion, and declared comparison
  directions with **no default** — §5.2 warns that "more bits or a faster clock is not
  universally better", and an undeclared direction is a bug waiting for a substitution to expose
  it. The unit table is small on purpose: one accepting arbitrary SI prefixes accepts `Ps` too,
  and a typo that parses is worse than one that does not.
- **F03 green, 15 arms.** ⭐ The "before arithmetic" property is *demonstrated*, not asserted:
  the fixture compares two quantities whose magnitudes would overflow if touched — asserting
  first that they would — and the comparison still returns the dimension error, which is only
  possible if the dimension check precedes any read. A zero clock frequency is refused at
  construction, so no such quantity exists for anything to divide by later.
- New book chapter `docs/book/src/quantities.md`.
- Validation: 15 new arms, 176 tests across the workspace, 0 failed; fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — kinds and schemas, with exactly one trusted primitive

`ARCHOGEN-M1-0013` (leaf `M1.2`).

- `crates/eadl-model/src/kind.rs` implements the kind registry, the `defkind` facility, and
  schema validation of the declaration frame: known kind, name when required, known clauses,
  cardinality, value shapes.
- ⭐ **Exactly one declaration is a trusted primitive.** §2 requires that "a small trusted
  semantic foundation remains explicit" and that "the registry cannot silently introduce new
  trusted axioms". So `defkind` is Rust, and all five surface kinds are declared *in eADL* in
  `docs/semantics/kinds/core.eadl` — no more privileged than a kind added tomorrow. A test
  asserts both halves, including that `defkind` is **not** a registry entry.
- §5.6's prohibition has no back door: the boundary classifier runs over kind definitions too,
  so `(defkind deftimer … (implementation …))` is refused by the same machine that refuses it in
  an ordinary declaration. A kind must also carry a `doc` — a kind nobody can explain is a kind
  nobody should be adding.
- A forbidden construct gets the **boundary's** wording, not "unknown clause", which is true and
  useless. A typo gets an edit-distance-bounded suggestion; the bound exists so that
  `implementation` is never "corrected" to `defsystem`, which would send an author to rename
  rather than to reconsider.
- ⭐ **A claimed equivalence was refused by its own test and replaced with a measurement.** The
  first version asserted that the schema refuses every rejected boundary case too;
  `execution-bound` broke it, because `wcet` hides inside `(task …)`, which is opaque at this
  layer. The honest result — schema 10 of 11, classifier 11 of 11 — is now pinned in both
  directions, with `M1.7` named as the leaf that closes the gap.
- New book chapter `docs/book/src/kinds.md`.
- Validation: 13 new arms, 150 tests across the workspace, 0 failed; fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — F27: the boundary gets a machine

`ARCHOGEN-M0-0011` (leaf `M0.3`). **M0 is complete.**

- `crates/eadl-model/src/boundary.rs` registers nine constructs that are implementation by
  definition — `implementation`, `model`, `provider`, `emit`, `permits`, `wcet`, `init-order`,
  `save-order`, `read-sequence` — each with the boundary test it fails, what it actually is, and
  where it belongs instead. A refusal that does not say where the content belongs is an obstacle
  rather than guidance.
- **A construct registry, not a keyword scan**, and the corpus is why: the *accepted*
  `required-ordering-guarantee` declaration contains `write` twice, naming observable effects an
  ordering requirement is stated over. A word-based classifier refuses a contract the roadmap
  explicitly permits. What separates a step from a reference is the construct it sits inside.
- F27 runs six arms over all 21 cases, `6 passed 0 failed`. The load-bearing one is **mutation**:
  seeding each of the nine constructs into each of the ten accepted cases — 90 mutations — must
  flip every one to rejected with the matching test, and stripping the offending construct from
  each rejected case must flip it back. Agreement alone is passed by a classifier that accepts
  everything.
- ⭐ **The coverage arm found a real gap in the corpus, and it was fixed at the corpus**:
  `save-order` and `read-sequence` were registered but never reached, because both cases wrapped
  their procedure in an `implementation` block that was blamed first. Both now carry the
  procedure as an ordinary clause — the realistic shape, since an author who believes a retry
  loop belongs in the description does not label it "implementation" beforehand.
- The honest limit is in the module, the book and the corpus README: acceptance means "contains
  no construct the registry knows to be implementation", not "is sound". §4.3 requires human
  review for intent, because a field can hide an algorithm behind an innocent name.
- Book: `docs/book/src/boundary.md` gains "How much of this a machine can check".
- Validation: 137 tests across 11 suites, 0 failed; fmt and clippy clean; all doctrines green.

## archogen — the eADL reader

`ARCHOGEN-M1-0010` (leaf `M1.1`).

- New crate `crates/eadl-front`: S-expressions, byte spans with **character** columns, and
  caret diagnostics that carry a repair direction, as §5.5 requires of every diagnostic.
- Measured on the real corpus rather than on toys: all 21 boundary-corpus files read with zero
  diagnostics, round-trip semantically through canonical form, and yield their exact metadata
  key set. Those files were written for a different purpose before the reader existed, which is
  what makes the suite evidence rather than confirmation.
- **No float anywhere.** §7.4 requires exact integer or checked rational arithmetic, so a
  decimal literal is kept as an integer and a scale — `0.1` survives a round trip exactly — and
  an out-of-range literal is refused rather than wrapped.
- `3ms` is refused, not read as a symbol: it is a typo for `3 ms`, and accepting it would lose
  the magnitude and surface much later as a mysteriously missing field. Reading does not stop at
  the first error, so three malformed numbers cost one edit cycle.
- New diagnostic tool `cargo run -q -p eadl-front --example diagnose -- <file>`, registered in
  `TOOLBOX.md`.
- ⭐ **That tool immediately found a real defect.** The header parser silently truncated a
  corpus rationale that wrapped onto a line beginning `implementation-independence:` — exactly a
  bare key plus a colon. Two green suites were blind to it, because both asserted only that the
  keys they *wanted* were present, and a presence check cannot see an extra key. The rule now
  requires two independent discriminators, each of which had already been tried alone and failed
  on a real file; the corpus test asserts the exact key set for every case. Promoted to
  `docs/knowledge/presence-checks-cannot-see-an-extra-key.md`.
- New book chapter `docs/book/src/reading.md`.
- Validation: 52 tests in the new crate, 119 across the workspace, 0 failed; fmt and clippy
  clean; all doctrines green; `mdbook build` OK.

## archogen — the emulator pinned, and the board recorded as absent

`ARCHOGEN-M0-0009` (leaf `M0.5`).

- `targets/riscv-virt-up.env` pins the emulator configuration as data, and
  `scripts/target_emulator.sh` is the only thing that renders it — so no two callers can type
  it slightly differently. §3.2: "Pin the emulator configuration … do not rely on changing
  defaults." `--dump-dtb` writes QEMU's generated device tree for the agreement check against
  the eADL platform fixture, which is the difference between describing a platform and
  describing *this* platform.
- Absence is reported, never skipped: with QEMU not installed, `--check` exits `20` saying
  `do NOT record an emulator result without it`. §14.3 requires exactly that. The configuration
  also carries `TARGET_VERIFIED=no` until an installed QEMU confirms it.
- 🔎 **No physical board has been selected or procured.** All seven facts §3.2 requires are
  recorded as `unrecorded`, with the selection criteria written out. Naming a plausible board
  from memory would be worse than naming none — each row is a fact a later timing claim would
  rest on. Tree `M5` is blocked at the root; S0–M4 are not. This is a director decision.
- New book chapter `docs/book/src/targets.md`; `TOOLBOX.md` gains the emulator tool row.
- Validation: `bash -n` clean; docpath clean; all doctrines green; 67 tests pass;
  `mdbook build` OK.

## archogen — a report that cannot say "verified"

`ARCHOGEN-M0-0008` (leaf `M0.7`).

- New crate `crates/osgen-evidence` — the evidence, claim and trust vocabulary of §7.1, §7.3
  and §4.4. A separate crate on purpose: it is shared between the generator and the independent
  checker, and §4.4 requires such sharing to be visible rather than buried.
- §7.1's "one global verified flag is prohibited" is encoded **three ways**, not documented
  once: there is no aggregate verdict type; a report refuses to render while any property is
  unanswered (`a property with no claim is not a pass`); and every positive conclusion carries
  its qualifier by construction — a conditional analysis with an empty assumption list is
  refused as `an unconditional claim`.
- Bounds remember where they came from. An observed maximum stays an observation whatever the
  safety factor — tested at 1/1, 3/2, 10/1 and 1000/1 — because §7.3 says so and because
  multiplying a measurement by 1.5 and calling it a bound is the most common way a timing claim
  becomes untrue while looking like diligence. A bound with no binary identity is refused.
- ⭐ The compiler caught a design defect: `f64` is not `Eq`, so a float safety factor would
  have cost the whole vocabulary comparability — and a bound that cannot be compared to its
  baseline cannot be checked for drift. Replaced with an exact rational, which §7.4's "exact
  integer or checked rational arithmetic" wanted anyway. The type error was a semantic error.
- §4.4 trust vocabulary: roots, roles, and drift. Shared infrastructure is recorded but costs
  no independence; a shared semantic helper does. An unrelated change produces no warning
  (§14.4 case 5) — a gate that cries wolf is a gate that gets disabled. The honest limit is
  carried in the module: this enforces disclosure, not semantic independence.
- New book chapter `docs/book/src/evidence.md`.
- Validation: 33 new contract tests, 67 in the workspace, 0 failed; fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — four use cases, and a seal that is a check rather than a promise

`ARCHOGEN-M0-0007` (leaf `M0.6`).

- `docs/usecases/` — the three cases §12 M0 requires plus a fourth: `uc1-periodic-three`
  (succeeds by building), `uc2-high-interference` (may succeed by refusing to conclude),
  `uc3-alternative-timer` (refused before M3, supported after), `uc4-bounded-queue` (succeeds
  by refusing). A profile that has never refused anything has not been tested as a profile.
- `uc3` is the only case whose expected answer *changes* when a capability lands. That is how
  the project distinguishes adding an engine capability from weakening a requirement until it
  passes — both turn a red fixture green, only one is progress.
- Five evaluation cases sealed under `docs/evaluation/frozen/` with SHA-256 digests, and a new
  project doctrine `FROZEN-EVALUATION` that enforces three legs: **integrity** (no post-seal
  edit), **completeness** (nothing added or removed unlisted), and **non-contamination** (no
  tracked file outside the sealed directory names a sealed case). All three proven in both
  directions. §16's reuse claim is about *unseen* systems; measured on cases that were in view
  while the catalog was designed, it measures how well the catalog was fitted to them.
- ⭐ A latent defect in the first cut of the checker was caught before it landed: the
  contamination scan word-split tracked paths containing spaces and put the whole file list on
  one command line. Replaced with `git grep --fixed-strings` and re-proven against a path with
  a space in it.
- `scripts/check_doctrines.project.sh` turned from a shipped no-op into a real registry;
  `DOCTRINE_ENFORCEMENT.md` gains the project-doctrine mirror; `TOOLBOX.md`'s placeholder
  table replaced with seven real instruments and the rule that a new tool ships a RED arm.
- New book chapter `docs/book/src/usecases.md`.
- Validation: `bash -n` clean on both scripts (shellcheck not installed on this machine);
  `cargo test --all` → 34 passed, 0 failed; all doctrines green; `mdbook build` OK.

## archogen — the 21-case boundary corpus

`ARCHOGEN-M0-0006` (leaf `M0.2`).

- `docs/semantics/boundary/` — 21 worked cases against minimums of 12 and 3: 10 accepted, 11
  rejected, 5 of them ambiguous and argued rather than asserted. Every rejected case names the
  test that failed.
- Cases are written in **pairs** wherever possible — the accepted contract and the rejected
  procedure that satisfies it — so the corpus shows where the boundary runs rather than only
  recording verdicts: `atomic-observation` vs `atomic-read-retry-loop`, `addressable-region`
  vs `register-programming-sequence`, `required-ordering-guarantee` vs `initialization-order`.
- The ambiguous five are the ones a reasonable author gets wrong in both directions:
  `counter-width-and-rate` and `addressable-region` look like implementation and are accepted;
  `execution-bound`, `initialization-order` and `retry-permitted` look like requirements and
  are rejected. A WCET is evidence about a binary (§7.3), not a description field.
- `docs/semantics/boundary/README.md` fixes the case format F27 will consume, and states the
  honest limit: the mechanical check is a floor, because a field can hide an algorithm behind
  an innocent name.
- Sequencing decision recorded: F27 (`M0.3`) lands after the reader (`M1.1`) rather than
  before it — mechanizing the corpus needs real parsing, and two throwaway tokenizers would
  buy no earlier signal.

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
