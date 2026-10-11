# Generated sources: the parent's tests held to a mutation criterion

- **Type:** `decision`
- **Date:** `2026-10-10`
- **Status:** `active` — written `2026-10-10` by leaf `M3.6.9`; its review closed `2026-10-11`, round 3: zero defects and one remark; `.9.1` built strict and attributed outcomes, remaining implementation pending
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
     the census below, a `route-…` entry and a twin named `parent-…`, its file, text and replacement the same, its
     `test` line `-p xtask -- trust:: trust_generated:: trust_gate:: --skip route_tests`. The census maps missing
     judgments of the parent's rules that the history identifies by a mutation or missing fixture; corrections to
     route prose, ledgers and summaries introduce no separate code defect. Repeated findings map to the same family;
     several mutations may witness one family. It retains all 23 existing entries and adds nine, deriving 32 pairs,
     rather than calling the 23 entries an exhaustive census. Each new row specifies the code and narrowing below.
     A test reads this machine-read table and holds the catalogue's route/parent populations and files equal to it,
     and each twin's file/from/to equal to its route entry. Route controls that kill a new entry are added before
     its two populations are measured.
     Review round 1 ran the existing 23 against the parent's tests at `583cf07`: 22 survive, one is already killed
     (an untracked runner, not durable). The build records actual outcomes, then adds the parent's missing tests.
     Both route and parent commands gain `--timeouts-fail`: a hang fails the criterion, even though ordinary
     catalogue mode continues to count a reached hang as killed. Strict timeout handling is opt-in. A finishing kill
     requires a failed libtest summary and at least one failure-list name. In either mode an invocation failure,
     crash or abort without that evidence is unjudged (exit 2), never a kill; round 3's existing false-green is owned
     by `M3.6.9.1`. The sweep applies the same attribution rule and admits no unjudged equivalent.
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
   - `contains`: `X.contains(Y)` made `X.first() == Some(Y)` and `X.last() == Some(Y)`, and each end with
     `.copied()` before the comparison. The direct form compiles for the recorded vector membership; the copied form
     preserves the borrowed element for `BTreeSet<&str>`, whose direct form is unviable. Other types may make either
     form unviable: compilation, not listing or a universal spelling claim, decides each candidate;
   - `clogic`: an operand of a top-level `&&` or `||` inside a closure on one line dropped.

   The runner stores source-line intervals and replacement text so `guard` changes header and branch together.
   `M3.6.9.2.1` builds this and JSON multiline equivalents (`from-json`, `to-json`); raw fields stay literal.
   Each record is validated at separators and EOF; missing/duplicate/unknown/empty fields or duplicate entries refuse.
   `M3.6.9.2.2.1` builds `iter` with lexical/compiled controls; guard scope and emission are `.2.2.2.1`/`.2.2.2.2`,
   holding complete conditions, evaluation order and body work. Chain call spans/variants are `.2.3.1`, with compiled
   ordering and multiline CLI controls; dropped calls retain inert comments for nonempty equivalent fields.
   Membership variants are `.2.3.2`: concrete compiler controls distinguish direct/copy viability and type refusals,
   with complete receiver/argument spans and preserved negation. `.2.3.3.1` repairs function-type arrows in generic
   receivers, held by positive emitted-candidate compilation; counts alone had hidden the lost viable spellings.
   `.2.3.3.2.1` repairs the existing condition deletion's middle-separator defect before closure reuse: remove the
   first operand with its following separator, or a later operand with its preceding separator. Native compiler
   truth tables and real CLI restoration hold homogeneous/mixed conditions; changed old logic candidates are
   attributed explicitly rather than claimed byte-identical.
   `.2.3.3.2.2` builds the one-line closure operator: expression/final/returned boolean spans, typed returns and
   transparent parentheses; nested closures own their own top-level connectives. Balanced calls/generics and
   complete literal text are retained, with native compiler/value-selection and real CLI restoration controls.
   Production reach remains unmeasured.
   A mutant that does not compile is unviable, as every other, and the sweep reports each operator's count of them.
   `.2.3.4` builds terminal per-operator `unviable/attempted` totals, including zero refusals. Baseline, list-only
   and filtered-out candidates do not count; real compiler/libtest CLI controls hold both zero/nonzero totals and
   selected/no-attempt runs. The existing strict timeout and unjudged outcome rules remain unchanged.
   The operators are held by measurement, not by `--list`: after the new route controls and before the parent gains
   a test, the build runs every candidate they make on each census entry's replacement span against both test
   populations. It records each candidate's actual outcomes, including unviable and timed out, and distinguishes an
   exact or effect-equivalent witness of the entry from a different defect on the same lines. It never chooses a
   witness just because the verdict pair is desired. An already-killed parent entry is valid evidence. Each entry's
   row names its viable witnesses or "none", with the missing shape or type reason; a promised loop narrowing with
   no viable spelling is repaired before claiming that operator reaches it. Unviable candidates remain counted and
   prove no coverage. Point 2's exact entries must all pass regardless of the operators' measured reach.
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
   repository the copy was made from. The test seam canonicalizes both paths and admits that extra root only when
   its Git top-level is exactly that root and `real_root()` is exactly its `target/mutation-sweep/tree` child, not
   merely enclosed by it. A malformed, intermediate or unrelated root refuses before any configuration is admitted;
   without the variable, the existing single-root tests run as before. The sweep sets the variable from its own
   canonical Git root, replacing any inherited value. Its configuration is then held as
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

   `M3.6.9.3.1` builds explicit root-list support in the configuration checker and trust inventory/gate; the
   catalog's single-root entry point and production's sole repository root remain unchanged. Filesystem controls
   hold root-order independence and independent tracked-byte/content/symlink/target/ancestor refusals. Canonical
   source binding is built by `.9.3.2`: test wrappers validate canonical paths, the source's own Git top-level and
   the exact physical copy child before inventory. Explicit source hints now let a real copied parent fixture pass;
   zero mutations were attempted in that probe. `.9.3.3` supplies the canonical own source root to every baseline
   and mutant command, replacing inherited hints. Actual copied parent populations pass with an invalid inherited
   hint and zero mutation attempts; production measurements remain separately owned.
5. **The sweep takes each file's complete production range.** `trust.rs`, `trust_generated.rs` and `trust_gate.rs`
   each run from the first function or method to the first `#[cfg(test)]`, as the existing tool does. No function
   filter or extraction is needed. This includes the helpers and their callers: the reading order, read sets and
   generator judgments/builds; marked-file census (§1, §3); per-program provenance (§4); pair/harness publication
   (§5); form changes and standing list (§2); item identity, aspects and comparisons (§5); staleness (§6).
   Older inventory and gate branches inside these ranges are deliberately included: a survivor that changes their
   promised behavior gains a test, never an equivalence reason merely saying it is outside generated sources.
   At `d67abc3`, the existing operators list 656, 98 and 178 candidates respectively (932 total); that is a planning
   size, not viability, coverage or a result from the widened operators. The recogniser, `generated_header.rs`, stays
   held by the parent's own sweep (the parent's §3); imported build, catalog and eADL facilities keep their own
   contracts and tests. This criterion does not claim to sweep their implementations or mutations of constants and
   type declarations outside these production ranges.

### The recorded-defect census

The source is `docs/review-history/decision_trust-generated-refusals-reviews.md`, read through its closing round 23.
The table's suffix names both `route-<suffix>` and `parent-<suffix>`. Its 32 rows are specified mutations, not 32
independent defect families. Findings about the same judgment are deduplicated; earlier route needs are included
where a later review identifies missing parent-rule coverage (R1-D2, R6-D1, R7-D1). Rounds 15–17's missing-axis
prescriptions and rounds 19–22's corrections to the criterion are not additional reproduced code defects. They
constrain the method and its scope above. R2's census adds the four families its reviewer named and, on reading the
history, the transitive dependency and middle-generator provenance cases; neither is silently left to operators.

<!-- machine-read: parent-mutation-census -->
| Suffix | Source findings | Catalogue state and file |
| --- | --- | --- |
| `the-first-generator-alone` | R8-D1; R9-D1 | existing, `trust.rs` |
| `the-last-generator-alone` | R9-D1 | existing, `trust.rs` |
| `the-first-reader-alone` | R8-D1; R9-D1 | existing, `trust.rs` |
| `the-last-reader-alone` | R9-D1 | existing, `trust.rs` |
| `a-program-target-judged-for-its-first-reader-alone` | R9-D1 | existing, `trust.rs` |
| `a-program-target-judged-for-its-last-reader-alone` | R9-D1 | existing, `trust.rs` |
| `chain-and-blob-rules-on-each-clause-s-first-entry-alone` | R7-D1; R10-D1; R10-R2 | existing, `trust_generated.rs` |
| `chain-and-blob-rules-on-each-clause-s-last-entry-alone` | R7-D1; R10-D1; R10-R2 | existing, `trust_generated.rs` |
| `both-clauses-the-first-input-alone` | R10-D1 | existing, `trust_generated.rs` |
| `both-clauses-the-last-input-alone` | R10-D1 | existing, `trust_generated.rs` |
| `both-clauses-the-last-generator-alone` | R10-D1 | existing, `trust_generated.rs` |
| `a-program-target-judged-when-first-alone` | R11-D1 | existing, `trust.rs` |
| `a-program-target-judged-when-last-alone` | R11-D1 | existing, `trust.rs` |
| `a-program-target-built-when-first-alone` | R11-D2; R12-D2 | existing, `trust.rs` |
| `a-program-target-built-when-last-alone` | R11-D2; R12-D2 | existing, `trust.rs` |
| `a-lone-rs-refused-when-first-alone` | R11-D2 | existing, `trust.rs` |
| `a-lone-rs-refused-when-last-alone` | R11-D2 | existing, `trust.rs` |
| `a-middle-script-never-judged` | R12-D1 | existing, `trust.rs` |
| `a-middle-reader-never-judged` | R12-D3; R13-D1 | existing, `trust.rs` |
| `a-middle-input-never-reached` | R12-D4 | existing, `trust_generated.rs` |
| `an-implementation-reader-never-judged` | R13-D1 | existing, `trust.rs` |
| `a-marked-build-read-dropped-beside-other-generators` | R13-D3 | existing, `trust.rs` |
| `only-a-form-s-first-input-reached` | R1-D2; R9-D2 | existing, `trust_generated.rs` |
| `an-input-reached-through-a-link` | R23-R2 | add, `trust_generated.rs` |
| `held-judged-for-the-first-alone` | R18-R1 | add, `trust.rs` |
| `a-harness-never-judged-against-a-program-target` | R13-D2; R14-D4 | add, `trust.rs` |
| `the-last-live-form-alone` | R14-D3 | add, `trust_generated.rs` |
| `a-harness-judged-by-its-first-role-alone` | R18-R2 | add, `trust.rs` |
| `a-data-reader-never-judged` | R16-R3; R18-R3 | add, `trust.rs` |
| `a-build-read-naming-its-first-generator-alone` | R18-R4 | add, `trust.rs` |
| `a-middle-generator-never-reached` | R13-D4 | add, `trust_generated.rs` |
| `a-transitive-generator-build-read-dropped` | R6-D1; R10-D2 | add, `trust.rs` |

The nine additions have these exact intended narrowings; the catalogue's unique replacement blocks include enough
unchanged context to select the named occurrence, and are checked against the committed production source:

- `an-input-reached-through-a-link`: in `blobs_and_chains`, add `role != "input"` to the symbolic-link condition.
  This is a diagnostic branch control, as R23-R2 records, not an acceptance-changing defect. Native original and
  exact-mutant fixtures in `M3.6.9.4.1.1.1` both refuse the linked input: the original names a symbolic link,
  while the mutant names no blob. The regular-file tree excludes symlinks, so the following blob check still
  refuses it. Operator witnesses for this row must preserve that actual effect rather than infer admission.
- `held-judged-for-the-first-alone`: in `inventory_with`'s `foreign` closure, replace `for pkg in held` with iteration
  over `held.iter().take(1)`.
- `a-harness-never-judged-against-a-program-target`: filter `foreign(&held)`'s result in the program-target branch to
  discard readers whose `Program` has `pair.is_some()`; the script branch stays unchanged.
- `the-last-live-form-alone`: in `live`, insert `.rev().take(1)` after the live-form filter, before collection.
- `a-harness-judged-by-its-first-role-alone`: in `own_roles`, remove the second name's operand from the pair filter,
  leaving `r.name == *x`.
- `a-data-reader-never-judged`: add `!p.data.contains(&g.path)` to the step-4 readers filter, retaining its existing
  read-set membership condition.
- `a-build-read-naming-its-first-generator-alone`: in step 5's loop over `generator_reads`, replace `for g in
  generators` with `for g in generators.iter().take(1)`.
- `a-middle-generator-never-reached`: in `provenance`, enumerate the innermost `files` loop and skip an entry when its
  role is `generator` and its index is neither zero nor `files.len() - 1`, leaving input iteration unchanged.
- `a-transitive-generator-build-read-dropped`: in the successful generator compilation branch, before recording a
  unit's files in `generator_reads`, skip a unit whose package is neither `pkg` itself nor the destination of any
  graph edge directly out of `pkg`. This drops a dependency reached through another package from the chain's second
  clause while retaining own/direct reads; the root-reuse branch stays unchanged.

The catalogue table test checks the recorded source references and all existing/new suffixes, and refuses a missing,
extra or unmatched twin. Exact entries still must compile, land once and restore byte for byte under the existing
harness. New route controls are necessary for the previously unheld cases; their kills are measured, not presumed.
`M3.6.9.4.1.1` builds the three source/provenance additions; `.4.1.2` builds the four reader/role additions.
Their exact route mutations have named finishing kills. Native controls cover every held role package, both
harness pair roles and foreign script/target generators, and readers handed generated runtime data. The two
build-read additions, census/parity test, parent twins and operator measurements remain pending; parent fixtures
have not been strengthened.

## Why

- A list or a census names the axes and so can leave one out; mutation adequacy names none, but reaches only as far
  as its operators. The first part makes the criterion exact on the evidence whatever the operators make; the second
  carries it past the evidence as far as the operators go, which point 3's table states rather than assumes.
  Round 2 corrected the first answer's incomplete population: the explicit source-linked census above defines the
  exact entries, and a later finding extends it rather than being assumed covered by an operator or similar line.
- The parent's tests, not the route tests: the parent's rules are the parent's, decided by its record, and what its
  record claims hold them are its instrument's tests.
- A test seam, not an exception: the gate's refusal of an outer configuration is a rule of the trust model, and the
  tests already judge their fixtures through a seam that admits one outer configuration as a tracked copy; a second,
  admitted in the tests alone and held to the same rule, changes nothing the gate judges.

## How to apply

Its design review closed at round 3; implementation is owned in order by `M3.6.9.1` through `.6`. Built in this order: strict timeout modes in the catalogue and sweep, and the
operators; the bound test seam; the census/table test, nine new route entries and controls, and all 32 parent twins;
the operators' table measured against both populations before adding parent tests; then the commands below, the
parent's tests gaining what each survivor shows they lack, until every exact entry passes with finishing tests and
each full sweep exits 0. No selective function refactor is required. Configuration-seam controls must cover normal
fixtures, a correctly bound sweep copy, unset/malformed/other-ancestor source values, an intermediate `target/.cargo`
configuration, and an outer repository configuration not tracked or byte-identical. Production gate behavior must
remain the same. Run the three sweeps sequentially: they share one scratch tree and target directory.
Strict catalogue-mode controls distinguish a finishing kill, a survivor, a hang and an unviable entry, while checking
byte-exact restoration and the unchanged ordinary-mode hang result. A strict timeout fails even for an entry
expected to survive; it cannot satisfy either expected verdict.

```console
$ cargo xtask mutate --timeouts-fail --only $(sed -n 's/^mutation \(route-.*\)$/\1/p' xtask/mutations.txt)
$ cargo xtask mutate --timeouts-fail --only $(sed -n 's/^mutation \(parent-.*\)$/\1/p' xtask/mutations.txt)
$ bash scripts/mutation_sweep.sh xtask/src/trust_generated.rs --package xtask --filter=trust:: \
    --filter=trust_generated:: --filter=trust_gate:: --filter=--skip --filter=route_tests \
    --equivalents xtask/trust_generated.equivalents --timeout 180 --timeouts-fail
$ bash scripts/mutation_sweep.sh xtask/src/trust.rs --package xtask \
    --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: --filter=--skip \
    --filter=route_tests --equivalents xtask/trust.equivalents --timeout 180 --timeouts-fail
$ bash scripts/mutation_sweep.sh xtask/src/trust_gate.rs --package xtask \
    --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: --filter=--skip \
    --filter=route_tests --equivalents xtask/trust_gate.equivalents --timeout 180 --timeouts-fail
```

## Review

| Round | Date | Defects | Outcome |
| --- | --- | --- | --- |
| 1 | `2026-10-11` | 9, and 8 remarks | the 23 entries run against the parent's tests, 22 surviving, and the proposed operators' mutants, by an untracked runner; the placement reproduced; answered: each recorded one-way defect a `parent-…` entry the parent's tests must kill; `guard` and `contains` added, `chain` widened, their reach measured entry by entry; the parent's tests and the commands named; the copy under `target/` through the tests' seam, no exception taken |
| 2 | `2026-10-11` | 2, and 4 remarks | answered: a 32-row census maps the existing 23 and nine additions to source findings; whole production ranges include publication and comparison callers; the seam bound to the source Git root and exact sweep path; actual operator verdicts and semantic witnesses recorded, borrowed membership spelled separately; both catalogue and sweep reject timeouts; round 3 next |
| 3 | `2026-10-11` | 0, and 1 remark | independent 32-row/source-anchor/static-semantic/scope/seam review; 932 planning candidates reproduced, no implementation verdict claimed; accepted, review closed; existing unattributed invocation-error kills owned by M3.6.9.1 before measurements |
