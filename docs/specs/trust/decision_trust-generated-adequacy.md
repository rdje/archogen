# Generated sources: the parent's tests held to a mutation criterion

- **Type:** `decision`
- **Date:** `2026-10-10`
- **Status:** `active` — written `2026-10-10` by leaf `M3.6.9`; its review open
  ([`decision_trust-generated-adequacy-reviews.md`](../../reviews/decision_trust-generated-adequacy-reviews.md))
- **Owner / source:** leaf `M3.6.9` (`docs/tasks/M3.md`), filed by `M3.6.6.4`'s review, round 14. It holds the rules of
  [`decision_trust-generated-sources.md`](decision_trust-generated-sources.md), cited below as "the parent", to a
  criterion: its instrument's tests hold each rule wherever its code judges it.

## The question

The parent decides rules its instrument judges — the chain, the blob rule, a file named in both clauses, a lone `.rs`
file, the role judgment of each generator against each reader — and its instrument's tests hold each rule at the
instances they build. [`M3.6.6.4`'s review](../../review-history/decision_trust-generated-refusals-reviews.md) found,
round after round, rules held one way only — a form's first generator judged and not its last, one reader and not
another — and found every way of naming where a rule must be held leaving one out: a list of axes (rounds 15 to 17), a
census of the code's loops (round 18), the project's mutation sweep as it stands (round 19). This record decides a
criterion that names no axis, and what it needs to be true.

## Decision

1. **The criterion is mutation adequacy of the judgment code against the parent's tests.** Every mutant the sweep
   (`scripts/mutation_sweep.sh`) makes of the code that refuses or judges a generated source is killed by a test of
   the parent's instrument, or listed as equivalent with its reason in `xtask/trust_generated.equivalents`. The route
   tests of `decision_trust-generated-refusals.md` are excluded from the run: the rules are the parent's, and the
   claim is about its tests. A mutant that times out does not pass the criterion; it is a hang a test reaches, and is
   either killed by a test that finishes or shown to be equivalent.
2. **The sweep gains the operators that make the defects found.** Each one-way defect `M3.6.6.4`'s review found is a
   narrowing of an iteration or of a closure's condition, which the sweep's line operators never make. Three are added:
   - `iter`: a `for X in Y {` loop narrowed to its first element, `Y.into_iter().take(1)`, and to its last,
     `Y.into_iter().rev().take(1)` — a mutant that does not compile is unviable, as every other;
   - `chain`: in an iterator chain, `.iter()` and `.into_iter()` narrowed likewise, and a `.filter(…)` dropped;
   - `clogic`: an operand of a top-level `&&` or `||` inside a closure on one line dropped.
   Their acceptance is the catalogue: for each of `M3.6.8`'s 23 entries, the sweep's `--list` makes a mutant on the same
   line that narrows the same iteration or condition the same way.
3. **The sweep runs beside the repository when the tests demand it.** The parent's tests build fixtures and refuse a
   cargo configuration above the repository they judge; the sweep's copy under `target/mutation-sweep/tree` sits below
   this repository's own `.cargo/config.toml`, so its unmutated copy fails there. The sweep gains `--root <dir>`, the
   directory its copy and its build go under, defaulting to `target/mutation-sweep`; this criterion passes the
   repository's parent directory's `.<name>-mutation-sweep`, on the repository's volume, removed when the run ends.
   The director's data-locality rule forbids an off-volume location, which this is not; the project's narrower rule,
   "under `target/`", gains a dated exception for this one use in
   [`decision_scratch-on-the-repository-volume.md`](../../decisions/decision_scratch-on-the-repository-volume.md), and
   `SCRATCH-LOCALITY` admits it. Two other ways were weighed and refused: the gate admitting an outer configuration
   byte-identical to the tracked one — cargo merges configurations, arrays concatenated, so a second copy is not
   nothing, and it weakens the gate for a test's sake; and mutating the working tree in place — the catalogue's
   harness does, for seconds, but a sweep takes hours, and the working tree is shared.
4. **The judgment code is named, not counted in lines.** `trust.rs`'s steps 3 to 5 of the generated-sources
   inventory move into functions of their own, each named for its step, so the sweep's new `--fn <name>` restricts it
   to them; `trust_generated.rs` is swept whole, from its first `fn` to its tests. The criterion's commands, each
   written whole once built — file, functions, package, test filters, the route tests skipped, the equivalents file,
   the timeout, the root — are its How to apply.

## Why

- A list or a census names the axes and so can leave one out; mutation adequacy names none. Its one proviso — that the
  operators make the mutant an untested axis needs — is what point 2 makes true for the defects found so far, and what
  the catalogue holds them to.
- The parent's tests, not the route tests: the parent's rules are the parent's, decided by its record, and what its
  record claims hold them are its instrument's tests.
- Beside the repository, not inside it: the gate's refusal of an outer configuration is a rule of the trust model, not
  an accident of the sweep, and a test's needs do not loosen it.

## How to apply

Built with this record's review closed: the operators, `--root`, `--fn`, the steps' functions, the equivalents file,
and the commands, written here whole; then the parent's tests gain what the first run's survivors show they lack.

## Review

| Round | Date | Defects | Outcome |
| --- | --- | --- | --- |
