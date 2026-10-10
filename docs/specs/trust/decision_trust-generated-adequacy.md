# Generated sources: the parent's tests held to a mutation criterion

- **Type:** `decision`
- **Date:** `2026-10-10`
- **Status:** `active` — written `2026-10-10` by leaf `M3.6.9`; its review open, round 1 answered `2026-10-11`
  ([`decision_trust-generated-adequacy-reviews.md`](../../reviews/decision_trust-generated-adequacy-reviews.md))
- **Owner / source:** leaf `M3.6.9` (`docs/tasks/M3.md`), filed by `M3.6.6.4`'s review, round 14. It holds the rules of
  [`decision_trust-generated-sources.md`](decision_trust-generated-sources.md), cited below as "the parent", to a
  criterion: its instrument's tests hold each rule wherever its code judges it.

## The question

The parent decides rules its instrument judges — what a program reads, the chain, the blob rule, a file named in both
clauses, a lone `.rs` file, the role judgment of each generator against each reader, each input's provenance — and its
instrument's tests hold each rule at the instances they build.
[`M3.6.6.4`'s review](../../review-history/decision_trust-generated-refusals-reviews.md) found, round after round,
rules held one way only — a form's first generator judged and not its last, one reader and not another, a judgment
made only for an element at one position or in one role — and found every way of naming where a rule must be held
leaving one out: a list of axes (rounds 15 to 17), a census of the code's loops (round 18), the project's mutation
sweep as it stands (round 19). This record decides a criterion, and states how far it reaches.

## Decision

1. **The parent's tests are named.** They are the tests of `xtask/src/trust.rs`, `xtask/src/trust_generated.rs` and
   `xtask/src/trust_gate.rs`, the route tests of `decision_trust-generated-refusals.md` skipped:
   `cargo test -q -p xtask -- trust:: trust_generated:: trust_gate:: --skip route_tests` — 88 tests, passing in 12.5 s
   on `2026-10-11`, the 16 route tests left out. The rules are the parent's and the claim is about its instrument's
   tests; the route tests hold the refusals' routes, a later record's.
2. **The criterion has two parts, both judged by those tests.**
   - **The evidence, exactly.** Every one-way defect of the parent's rules that `M3.6.6.4`'s review recorded is a
     catalogue entry (`xtask/mutations.txt`) run against the parent's tests and expected killed: for each of
     `M3.6.8`'s 23 `route-…` entries a twin named `parent-…`, its file, text and replacement the same, its `test`
     line `-p xtask -- trust:: trust_generated:: trust_gate:: --skip route_tests`; and for the three the catalogue
     lacks, an entry against the route tests and its twin — `route-an-input-reached-through-a-link` (R23-R2),
     `route-held-judged-for-the-first-alone` (R18-R1), `route-a-harness-never-judged-against-a-program-target`
     (R14-D4). Review round 1 ran the 23 against the parent's tests at `583cf07`: 22 survive (an untracked runner, not
     durable). The build adds the tests that kill them.
   - **Beyond the evidence, the sweep.** Every mutant `scripts/mutation_sweep.sh` makes of the judgment code (point 5)
     is killed by the parent's tests or listed as equivalent with its reason, in one equivalents file per swept file.
     A mutant that times out fails: a hang a test reaches is no equivalent, and is killed by a test that finishes. The
     sweep gains `--timeouts-fail`, under which a timeout counts as a survivor no entry may list.

   What the criterion holds is every recorded one-way defect and every mutant the operators make. A defect of a shape
   neither makes is outside it — among them the judgment kinds no operator mutates: an `if let` or `let … else`
   branch, a `match` arm, `max_by_key`'s choice, a condition written over several lines (review round 1). A review
   that finds one adds its entry and twin, and an operator where its shape is a rewrite of the code's text.
3. **The sweep gains operators for the shapes the recorded defects took.** They narrow which elements a judgment
   reaches: a `for` loop or an iterator chain narrowed to its first or last element, or its middle dropped; a branch of
   a loop's body taken only at one position; a membership test narrowed to an end; an operand of a closure's condition
   dropped; and one, `route-an-implementation-reader-never-judged`, a branch taken only in one role, which no rewrite
   of the code's text makes without knowing the roles. The three operators this record first proposed made, by review
   round 1's count, mutants with the effect of 7 of the 23 entries; 16 took a shape no operator, old or new, made.
   Added, each spelled to compile on an owned or a borrowed iterable:
   - `iter`: a `for P in E {` line rewritten as `let mut __it = (E).into_iter().peekable(); let mut __k = 0usize;
     while let Some(P) = __it.next() { let (__first, __last) = (__k == 0, __it.peek().is_none()); __k += 1;`, with
     `if !__first { continue; }` after it — and in two further mutants `!__last` and `!(__first || __last)`: the loop
     narrowed to its first element, its last, and its two ends;
   - `guard`: the same rewrite of a loop's header without the `continue`, and one `if C {` or `} else if C {` line of
     its body, outside any nested loop or closure, made `if (C) && __first {`, `&& __last` or
     `&& (__first || __last)`: that branch alone narrowed;
   - `chain`: in an iterator chain, `.iter()` or `.into_iter()` followed by `.take(1)`, and by `.rev().take(1)`; a
     `.filter(…)` followed by the same two; a `.filter(…)` dropped; a `.chain(…)` dropped;
   - `contains`: `X.contains(Y)` made `X.first() == Some(Y)`, and `X.last() == Some(Y)`;
   - `clogic`: an operand of a top-level `&&` or `||` inside a closure on one line dropped.

   A mutant that does not compile is unviable, as every other, and the sweep reports each operator's count of them.
   The operators are held by measurement, not by `--list`: before the parent gains a test, the build runs every mutant
   they make on the lines of the 26 entries against the parent's tests and the route tests, and writes here, entry by
   entry, the operator mutant with the entry's two verdicts — surviving the parent's tests, killed by the route tests
   — or "none" and the shape no operator makes. A loop of those lines whose operator mutants do not compile is a
   spelling fixed before the run counts. Point 2's first part holds every entry whatever the table says; the table
   states how far its second part reaches.
4. **The sweep's copy stays under `target/`; the parent's tests admit the configuration of the repository it was
   copied from.** What the gate admits today (`configurations_on_path`, `xtask/src/catalog_build.rs`): a cargo
   configuration on a build's path inside the written tree, held to the content rules; one outside it only as the
   tracked copy, byte for byte, of the repository `inventory_with` is given as its `config_root` — the judged
   repository for the gate, and for a test's fixture nested under this repository this repository, `real_root()`, so
   two copies already sit on every fixture's path; `target/.cargo` and anything above that repository refused. The
   content rules admit `alias` keys alone (`crates/archogen-catalog/src/package.rs`), and the repository's one alias,
   `xtask`, is invoked by no command the inventory runs. A copy under `target/mutation-sweep/tree` has this
   repository's `.cargo/config.toml` above its `real_root()`, refused there: every test that builds a fixture fails in
   the copy, and so the unmutated copy (reproduced by review round 1). Decided: `config_root` becomes a list, a
   configuration judged against the innermost root that holds it. The gate passes its repository alone and judges
   exactly as before; the tests pass `real_root()` and, when the sweep names it in `ARCHOGEN_SWEEP_SOURCE`, the
   repository the copy was made from, admitted only if it encloses `real_root()`. Its configuration is then held as
   the other outer one is: tracked, byte for byte, aliases alone. The sweep writes under `target/` alone; no exception
   to the project's scratch rule is taken. Refused:
   - a copy beside the repository, as this record first decided: a second exception to "scratch under `target/`",
     which only the director has ruled
     ([`decision_ci-rehearsal-beside-the-repository.md`](../../decisions/decision_ci-rehearsal-beside-the-repository.md)),
     for what a test seam answers;
   - the gate itself admitting an outer configuration identical to the tracked one: a change to the gate's rule where
     the tests' seam suffices;
   - the sweep in place, in the working tree: the catalogue's harness does so for seconds, a sweep takes hours, and
     the working tree is shared;
   - the CI rehearsal's checkout beside the repository: ruled for that script alone.
5. **The judgment code is named by function.** The sweep gains `--fn <name>`, restricting it to the named functions'
   lines. `trust_generated.rs` is swept whole, from its first `fn` to its first `#[cfg(test)]`. In `trust.rs` the code
   `inventory_with` runs for generated sources moves into functions of its own: `program_reads` (what each program
   reads, the parent's §1, the harness's through `beside`), `generated_first_clause` (step 3: the blob rule and the
   chain's first clause), `generated_roles` (step 4), `generated_second_clause` (step 5), `harness_item` (the
   comparison harness's item with its provenance aspect, §5) and `generated_unread` (§6's list); they are swept with
   `read_roots` (§2's reading order) and `beside`. In `trust_gate.rs`, `stale_generated` (§6's refusal) and
   `standing_generated` (§2's standing list) move out of `judge`, swept with `identity_key` (§5's item identity). The
   recogniser, `generated_header.rs`, stays held by the parent's own sweep (the parent's §3).

## Why

- A list or a census names the axes and so can leave one out; mutation adequacy names none, but reaches only as far
  as its operators. The first part makes the criterion exact on the evidence whatever the operators make; the second
  carries it past the evidence as far as the operators go, which point 3's table states rather than assumes.
- The parent's tests, not the route tests: the parent's rules are the parent's, decided by its record, and what its
  record claims hold them are its instrument's tests.
- A test seam, not an exception: the gate's refusal of an outer configuration is a rule of the trust model, and the
  tests already judge their fixtures through a seam that admits one outer configuration as a tracked copy; a second,
  admitted in the tests alone and held to the same rule, changes nothing the gate judges.

## How to apply

Built with this record's review closed, in this order: `--fn`, `--timeouts-fail` and the operators in the sweep; the
seam; the functions; the three new route entries and the 26 `parent-…` twins; the operators' table, measured; then the
commands below, the parent's tests gaining what each survivor shows they lack, until every one passes —
`cargo xtask mutate` ending `mutate: OK`, each sweep exiting 0:

```console
$ cargo xtask mutate --only $(sed -n 's/^mutation \(parent-.*\)$/\1/p' xtask/mutations.txt)
$ bash scripts/mutation_sweep.sh xtask/src/trust_generated.rs --package xtask --filter=trust:: \
    --filter=trust_generated:: --filter=trust_gate:: --filter=--skip --filter=route_tests \
    --equivalents xtask/trust_generated.equivalents --timeout 180 --timeouts-fail
$ bash scripts/mutation_sweep.sh xtask/src/trust.rs --package xtask --fn read_roots --fn beside --fn program_reads \
    --fn generated_first_clause --fn generated_roles --fn generated_second_clause --fn harness_item \
    --fn generated_unread --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: --filter=--skip \
    --filter=route_tests --equivalents xtask/trust.equivalents --timeout 180 --timeouts-fail
$ bash scripts/mutation_sweep.sh xtask/src/trust_gate.rs --package xtask --fn identity_key --fn stale_generated \
    --fn standing_generated --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: --filter=--skip \
    --filter=route_tests --equivalents xtask/trust_gate.equivalents --timeout 180 --timeouts-fail
```

## Review

| Round | Date | Defects | Outcome |
| --- | --- | --- | --- |
| 1 | `2026-10-11` | 9, and 8 remarks | the 23 entries run against the parent's tests, 22 surviving, and the proposed operators' mutants, by an untracked runner; the placement reproduced; answered: each recorded one-way defect a `parent-…` entry the parent's tests must kill; `guard` and `contains` added, `chain` widened, their reach measured entry by entry; the parent's tests and the commands named; the copy under `target/` through the tests' seam, no exception taken |
