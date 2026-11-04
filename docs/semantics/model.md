# The model layer: what a description means, and what the engine concludes from it

`docs/semantics/reference.md` governs the surface: what a description **is** and what its literals are
**worth**. This document governs the layer above it: what the engine concludes about what a description
**means**. That covers quantities and their units, which facts are present and which are absent, whether a
concrete platform honours an abstract one, what a profile admits of a workload, and what a description
may not contain at all. It is normative over the sources declared below, and over nothing else (leaf
`M1.26.1`).

The two documents split on one distinction. A code the reference states is a **refusal of the language**:
the description does not read, so nothing can be concluded from it. A code this document states is a
**verdict of the engine** on a description that reads. Several of them are named for the §5.5 verdict they
produce, such as `missing-fact`, `infeasible-configuration` and `unsupported-profile`. So the surface codes
belong to `eadl/1` and are frozen with it. These grow with every profile and knowledge base §12 admits,
and they are not frozen (the reason is recorded in `crates/eadl-front/examples/language_freeze.rs`).

## How to read this document

The conventions are the reference's. A table introduced by a `<!-- machine-read: … -->` comment is
**executed**: `crates/eadl-front/tests/reference.rs` reads it with the same reader it uses for the
reference, because two parsers for one table shape are two things that can disagree about what a rule
says. A table without that comment is prose. A rule is either carried by a row or cites the code that
enforces it.

⛔ **No count appears in this document**, for the reason the reference gives: a count in prose is a figure
nothing re-derives.

## What this document is normative over

<!-- machine-read: normative-sources -->
| source | what this document states about it |
| --- | --- |
| `crates/eadl-model/src/quantity.rs` | §1 quantities and units, and every diagnostic in §6 whose code begins `quantity-` |
| `crates/eadl-model/src/presence.rs` | §2 presence and absence: a fact offered, absent, required or described nowhere |
| `crates/eadl-model/src/refinement.rs` | §3 refinement: what a concrete platform owes the abstract one it refines |
| `crates/eadl-model/src/workload.rs` | §4 the workload a profile admits: release models, deadlines and priorities |
| `crates/eadl-model/src/check.rs` | §4 profile admission, §3 the target a refinement names, and §6's `tool-failure` |
| `crates/eadl-model/src/boundary.rs` | §5 the implementation boundary |
| `crates/eadl-resolve/src/vocabulary.rs` | §7 rule 1, the capability vocabulary read into a typed table |
| `crates/eadl-resolve/src/value.rs` | §7 rule 2, the domains' values and their exact comparison |
| `crates/eadl-resolve/src/offer.rs` | §7 rule 3, what a provider offers and declares absent |
| `crates/eadl-resolve/src/requirement.rs` | §7 rule 4, what a side requires, read whole |
| `crates/eadl-resolve/src/relation.rs` | §7 rule 5, each fact's outcome at a provider and each requirement's verdict there |
| `crates/eadl-resolve/src/refusal.rs` | §7 rule 6, the relation's refusals, their codes and their repair directions |
| `crates/eadl-resolve/src/joint.rs` | §8, whether the bindings of one assignment hold together: channels, owned regions, connectivity, routes, privilege, placement and the initialization graph |

The census over this declaration runs in both directions, in `crates/eadl-front/tests/reference.rs`. A code
these sources can emit and §6 does not state is a rule nobody has written down. A code §6 states and no
declared source emits is a rotted row. Both fail the build, as they do for the reference.

⚠️ **`crates/archogen-emit/src/lib.rs` is deliberately not declared.** Its refusals state what the hosted build
cannot yet express (`docs/decisions/decision_hosted-build.md`), not the rules below, so declaring it normative would
freeze a surface that grows with each target. The consequence is stated rather than left:
`analysis-inconclusive`, which the search answers when its budget ends, is stated by no normative document. The
book's rendering of any of them is checked for existence (the code is one a production source emits), not for meaning.

## 1. Quantities carry a unit, and the unit is one of a small set

A quantity is written as a number followed by a unit, such as `10 ms` or `48 MHz`
(`crates/eadl-model/src/quantity.rs`, `Quantity::read`).

1. **A bare number is not a quantity.** Nothing would say what it measures, so it could be compared with
   nothing: `quantity-missing-unit`.
2. **The units are an explicit table, not SI prefixes applied freely** (`UNITS`). Time is `s`, `ms`, `us`
   and `ns`; frequency is `Hz`, `kHz`, `MHz` and `GHz`; information is `bit`, `byte`, `KiB` and `MiB`; a count is
   `tick`. A table that accepted any prefix would accept `Ps` and `mHz` too, and a typo that parses is worse than
   one that does not: `quantity-unknown-unit`.
3. **A frequency is strictly positive** (§6.2). Every conversion from ticks to time divides by it:
   `quantity-non-positive-frequency`.
4. **A duration is never negative.** An interval's direction belongs in the clause that uses it, not in
   the sign of its magnitude: `quantity-negative-duration`.
5. **A quantity is exact.** A decimal too precise for the exact representation is refused rather than
   rounded (`quantity-overflow`), and two quantities of different dimensions are never compared.
6. **`quantity-invalid` is a totality arm, not a rule.** `Quantity::new` refuses exactly the cases
   rules 3 and 4 state, and each has its own code. The catch-all exists so that a refusal added to
   `Quantity::new` later is reported rather than dropped. No input produces it today. Its row is the
   only one of its kind in §6, so a second such row is a question to ask, not a pattern to follow.

## 2. A fact is offered, absent, or unknown, and the difference is load-bearing

A platform declares what it **offers** and what it declares **absent**; a system **requires** facts
(`crates/eadl-model/src/presence.rs`). Absent is a definite answer, not a gap.

1. **A fact is not both offered and absent by one declaration.** A block, a platform or a service that offers
   a fact and declares it absent contradicts itself, and a contradiction is not resolved by preferring one side,
   because only the author knows which half was meant: `invalid-description`. A declaration's absence covers what it
   is made of: a block or platform declaring a fact absent while a block or platform it composes — through every
   `uses` it writes, at any composition depth (§8 rules 1 and 5) — offers it says both of itself, `invalid-description`
   too. Two other declarations' offer and absence are two statements of two devices, each judged on its own, as two
   values are; every declaration is compared, not the first of each fact (`docs/decisions/decision_absence-per-provider.md`, leaf `M3.4.8`, which made this rule
   per declaration from description-wide; `docs/semantics/migrations/eadl-1-an-absence-is-a-declaration-s-statement.md`).
2. **A required fact that is declared absent makes the configuration infeasible.** A fact is absent to the
   description when some declaration declares it absent and none offers it: either the requirement or the platform
   is wrong, `infeasible-configuration`. A fact a block or platform offers is the search's to bind (`M3.4`); one
   offered only outside the system's hardware or only by a service, which the search binds to nothing, is accepted
   here and refused by the search until `M3.4.7` makes the check adopt its verdict.
3. **A required fact that nothing describes is missing.** It lies inside the dependency closure of what
   the system requests, so no decision that depends on it can be made: `missing-fact`.
   **Rules 2 and 3 and the engine's adapters** (`SR-H9`; leaf `M3.2.4`, `docs/specs/adapters/decision_engine-adapters.md`
   §7). For a required fact one of the engine's adapter kinds **adds** — offers, not passed through from itself, and
   not named in what it requires beneath — both rules ask whether one block or platform of the description offers,
   by name, every fact that kind requires beneath: where one does, a **path**, the fact is the search's to bind or
   refuse and these rules refuse nothing, an adapter's offer satisfying past a declared absence; where one offers part
   of it and leaves the rest undescribed there, declaring none of it absent, a **partial path**, the fact is
   `missing-fact`, its repair the facts to describe; otherwise rule 2 or 3 reads as above, its repair naming what an
   adapter would need. Presence reads names alone, so a path by name that fails by value, or one at a block or platform
   outside the system's hardware, is accepted here and refused by the search until `M3.4.7`.
4. **What lies outside the closure fails nothing, and stays visible** (§5.3). An undescribed fact that nothing
   the description requests depends on is not an error. It is reported as metadata: `archogen check` prints,
   after its verdict, the closure with what pulled each fact in and then every fact outside it
   (`Closure` in `crates/eadl-model/src/check.rs`). Those lines are not diagnostics, so the reference's §4
   rule 1 holds (leaf `M1.30`).

## 3. A refinement keeps every obligation of what it refines

A concrete platform may refine an abstract one (`crates/eadl-model/src/refinement.rs`). Every obligation
the abstract states is checked against the concrete one, and a violated one names the obligation it
violated: `refinement-violated`.

1. **A guarantee is kept.** The concrete platform offers every fact the abstract one offers, and not only
   as `false`. A bare boolean is `true`, and `(f false)` says the fact does not hold, so it keeps no
   guarantee of it (leaf `M1.40`).
2. **A bound is met, in its direction, and a value is kept.** Every bound the abstract platform states on a
   fact is checked, not the last of them: the concrete platform gives the fact a quantity, and every
   quantity it gives the fact satisfies the bound's direction. A value that is merely different is not a
   substitute, and one of another dimension cannot be checked against the bound at all. A value the
   abstract platform offers is kept only by the same value: a quantity equal as a quantity, so `10000 kHz`
   keeps `10 MHz`, and any other value equal as written, a bare offer reading as `true`. Written is the
   conservative reading, because this check reads no vocabulary: `(pow2 32)` does not keep `4294967296`,
   nor does a set written in another order or with another member, and each is refused, never wrongly
   accepted. A fact offered more than once, as a `region` is per named region, is kept one offer at a
   time (leaf `M1.40`).
3. **An absence is kept.** A fact the abstract platform declares absent is not offered by the concrete
   one, nor by any block or platform the concrete one composes — every one a `uses` it writes names, at its top or
   inside a `requires`, and theirs in turn, at any composition depth (§8 rules 1 and 5) — and a concrete platform keeps
   the absences of every platform up the chain its abstract one refines. An explicit absence is a constraint something relies on, not an omission to fill in, and is lost neither
   through what a refinement is made of nor through a chain (leaf `M3.4.8`).
4. **The target is declared.** A refinement names the platform it refines, and that platform must be
   present in the description (`crates/eadl-model/src/check.rs`). A target described nowhere is
   `missing-fact`: the contract the obligations are owed to is unavailable, so none of them can be
   checked, and the repair is to supply it. Until `M1.26.3` (`2026-09-30`) it was refused as
   `infeasible-configuration`, §2 rule 2's code, which then named more than one rule. An undeclared name is not an
   infeasibility. The change moved one exit code, `13` to `11`.

## 4. A profile admits a workload, or says what it would take

A profile is a set of promises the engine can keep, and a description asks for things inside or outside
it (`crates/eadl-model/src/check.rs`, `crates/eadl-model/src/workload.rs`).

1. **A task declares exactly one release model**, a `period` or a `min-separation`. §3.1 admits both, and
   the analysis needs one. With none there is no interval over which interference can be bounded:
   `missing-fact`. With both, a periodic task's period *is* its minimum separation, and §5.3 requires
   contradictory declarations to be refused: `invalid-description`.
2. **What a profile does not admit is refused, never weakened.** A requested capability the active
   profile excludes is `unsupported-profile`. The diagnostic says what admitting it would add, and the
   requirement is never silently reduced to a weaker guarantee.
3. **`rt-static-up-v1` admits constrained deadlines**, a deadline no longer than the release separation.
   With a longer one, more than one job of a task is alive at once, and a different analysis is needed:
   `unsupported-profile`.
4. **`rt-static-up-v1` admits static, unique priorities.** Equal priorities need a tie-break policy with
   its own analysis, which belongs to another profile: `unsupported-profile`.
5. **A priority is a rank: an integer from 1, and 1 is the highest**
   (`docs/decisions/decision_priority-comparison-direction.md`). A larger number is a lower priority, and
   the ranks of a system need not be contiguous, because only their order is used. A value below 1 names no
   rank: `priority-below-one`. That is the language's rule, not a profile's, so its verdict is
   `invalid-description`.
6. **`rt-static-up-v1` performs two overrun policies**, `fault` and `skip-late-job` (rule 5 of the profile's fault
   contract, `docs/profiles/rt-static-up-v1-faults.md`), and a task without an `on-overrun` clause has `fault`, the profile's default. Any other policy is
   one this runtime cannot perform, so it is refused rather than mapped onto one of the two:
   `unsupported-profile`. Another profile could perform more.
7. **A failure of the toolchain is never a verdict about the description** (§5.5): `tool-failure`.
8. **A task's `period`, `min-separation`, `deadline` and `jitter` are times, and its `period` or `min-separation` is above
   zero.** A quantity of another dimension describes nothing a run or an analysis can read, and no run can release a task with
   nothing between its releases. Both are the language's rule, and both `invalid-description`, naming the clause.

## 5. A description contains no implementation

eADL describes functionality and contains no implementation
(`docs/decisions/decision_eadl-engine-boundary.md`). A construct that is implementation, such as a
worst-case execution time or a scheduling mechanism, is refused where it appears, and the diagnostic
names the test it fails and the layer it belongs to: `boundary-implementation-in-description`
(`crates/eadl-model/src/boundary.rs`). The boundary's cases are in `docs/semantics/boundary/`, and
`docs/book/src/boundary.md` explains it.

## 6. Diagnostics

Every diagnostic these sources emit is an error, as the reference's §4 rule 1 states for the whole
toolchain. The `fires on` column is an input, written and run as the reference's notation section
describes: `crates/archogen-cli/tests/fires_on.rs` executes every row and requires its code. A code names a rule, not a call site (reference §4 rule 3): each row below states the rule, and
the one code that names more than one rule today says so.

<!-- machine-read: diagnostics -->
| code | when it fires | what to do | fires on |
| --- | --- | --- | --- |
| `quantity-missing` | a quantity is expected and nothing is written (§1) | write a quantity as a number followed by a unit, e.g. `10 ms` | `check (defplatform soc.abstract (offers (tick-rate (exactly))))` |
| `quantity-not-a-number` | the magnitude of a quantity is not a number (§1) | write a quantity as a number followed by a unit, e.g. `10 ms` | `check (defsystem s (task t (period fast ms) (deadline 10 ms) (priority 1)))` |
| `quantity-missing-unit` | a number has no unit, or what follows it is not a unit (§1 rule 1) | write the unit after the number, e.g. `10 ms`; the diagnostic lists the known units | `check (defplatform soc.abstract (offers (tick-rate (exactly 10))))` |
| `quantity-unknown-unit` | the unit is not in the table (§1 rule 2) | use one of the known units, which the diagnostic lists | `check (defsystem s (task t (period 10 parsec) (deadline 10 ms) (priority 1)))` |
| `quantity-non-positive-frequency` | a frequency is zero or negative (§1 rule 3) | write a positive frequency; every conversion from ticks to time divides by it | `check (defblock timer.counter (offers (tick-rate 0 MHz)))` |
| `quantity-negative-duration` | a duration is negative (§1 rule 4) | write a non-negative duration; an interval's direction belongs in the clause that uses it | `check (defsystem s (task t (period -10 ms) (deadline 10 ms) (priority 1)))` |
| `quantity-overflow` | a decimal is too precise to represent exactly (§1 rule 5) | reduce the precision, or change the unit so fewer digits are needed | `check (defblock timer.counter (offers (tick-rate 0.0000000000000000000000000000000000000001 MHz)))` |
| `quantity-invalid` | no input today: the totality arm for a refusal `Quantity::new` may grow (§1 rule 6) | write a quantity as a number followed by a known unit | `none: the catch-all arm of Quantity::read, for a refusal Quantity::new may grow later; today it refuses only what the frequency and duration rows state, and each has its own code (section 1 rule 6)` |
| `priority-below-one` | a task's priority is below 1 (§4 rule 5) | write a rank of 1 or more: 1 is the highest priority, and a larger number is a lower one | `check (defsystem s (task t (period 10 ms) (deadline 10 ms) (priority 0)))` |
| `invalid-description` | a description contradicts itself: a fact one declaration both offers and declares absent, itself or through what it composes (§2 rule 1), or a task with two release models (§4 rule 1); or the substitutability relation refuses what it writes, each refusal naming its cause (§7 rule 6); or a provider offers a consumed function with no channel, or a region that names no byte range (§8 rules 3 and 5), or a system's memory map makes two claims on one byte or reserves all its placement memory (§8 rule 10) | remove one of the two declarations; only the author knows which was meant | `check (defblock b (offers counter-width) (absent counter-width))` |
| `infeasible-configuration` | a required fact is declared absent (§2 rule 2); or an assignment gives a provider more requesters than channels, or two bound providers overlapping bytes, or a requester bound to, or a service or policy naming, a block or platform outside its system's hardware (§8 rules 3, 5 and 7), or a caller bound to a provider not reachable at its system's level, or declared reachable from none (§8 rule 9), or a placement given outside placement memory (§8 rule 10), once no assignment avoids it | correct the requirement or the platform | `check (defblock timer.counter (offers counter-width) (absent low-power-timer)) (defservice time.lowpower (requires (needs low-power-timer))) (defsystem s (requires (uses time.lowpower)))` |
| `missing-fact` | something the system requires is described nowhere: a required fact (§2 rule 3), a task's release model (§4 rule 1), or the platform a refinement names (§3 rule 4); or what §8 cannot judge an assignment without — a provider's `channels`, a region's size, a bound provider's level, the separation of a source a sporadic task binds itself (§8 rules 3, 5, 9, 10 and 16) | declare the fact offered or absent on the platform; give a task a `period` or a `min-separation`; for a refinement, import the module that declares the target, or correct its name | `check (defservice time.monotonic (requires (needs wrap-behavior))) (defsystem s (requires (uses time.monotonic)))` |
| `refinement-violated` | a concrete platform breaks an obligation of the abstract one it refines, or of a platform up its chain (§3 rules 1–3) | honour the obligation the diagnostic names: offer the guarantee, give the bounded value, or drop what the abstract declares absent | `check (defplatform soc.abstract (offers counter-width) (absent debug-port)) (defplatform soc.concrete (refines soc.abstract) (offers counter-width debug-port))` |
| `unsupported-profile` | the description asks for something the active profile does not admit (§4 rules 2–4 and 6); or the substitutability relation meets a constraint on a fact no rule decides, or a comparison past the exact arithmetic (§7 rule 6), a region's range among them (§8 rule 5); or a caller stating a level its system does not run its callers at (§8 rule 9); or a sporadic task bound itself to a source faster than it, unbounded, or past the exact arithmetic to compare (§8 rule 16) | request a profile that supports it, or change the description to fit this one; nothing is silently weakened | `check (defsystem s (task a (period 10 ms) (deadline 10 ms) (priority 1)) (task b (period 10 ms) (deadline 10 ms) (priority 1)))` |
| `tool-failure` | the toolchain could not proceed (§4 rule 7) | this is a failure of the toolchain, not a verdict about the description; report it | `none: only a shipped kind module over 4 GiB reaches it, and the kind modules are embedded in the toolchain (shipped_registry in crates/eadl-model/src/check.rs)` |
| `boundary-implementation-in-description` | a construct is implementation, which eADL does not contain (§5) | move it to the layer the diagnostic names; the description states what, not how | `check (defsystem app.rt (task sensor (period 10 ms) (deadline 10 ms) (wcet 850 us)))` |

## 7. Whether an offer satisfies a requirement

The substitutability relation decides whether what one provider offers satisfies one requirement. Its design is
`docs/decisions/decision_substitutability-relation.md`, reviewed until a round found no defect, and its executable
model is `crates/archogen-plan-check/src/model/`; this section is normative over the production code in `crates/eadl-resolve`,
which is held to the model over the model checker's whole universe (`crates/eadl-resolve/tests/production.rs`).
`archogen check` does not call the relation yet: `M3.4` wires it, keeping or migrating every verdict it moves.

1. **The vocabulary decides each fact's domain, role, direction and function.** `docs/semantics/vocabulary/vocabulary.eadl`,
   written with the kind `docs/semantics/kinds/deffact.eadl`, is read into a typed table
   (`crates/eadl-resolve/src/vocabulary.rs`). Each entry's frame is held to the kind; then the table refuses what no
   frame can say — a direction its domain does not admit, a derivation without the rule that computes it or with
   inputs that rule does not read, a cycle among derivations, `implies` off a set fact, `use consumed` off a boolean
   fact or a group's head, a guarantee naming no `function` or a statement naming one, `each` on a consumed, grouped or
   derived fact, a group's sub-fact or a derivation's input of another function (`M3.3.6`), a name twice, a fact named like a clause word. A fact the table does not declare may be offered, absent or needed, and presence judges it (§2); a
   constraint on it is `unsupported-profile`.
2. **Values are exact** (`crates/eadl-resolve/src/value.rs`). A quantity is §1's, two amounts in one unit compared by
   their written numbers and in two units in the base unit; a count is an integer or `(pow2 N)`; nothing is rounded.
   A comparison whose arithmetic overflows decides nothing: `unsupported-profile`, the value domain's limit, never a
   verdict on the requirement.
3. **A provider states one value per fact** (`crates/eadl-resolve/src/offer.rs`). A block's or platform's offers and
   absences are read into one state per fact: valued, in every spelling written; offered without a value; absent; or
   past the arithmetic. Two values of one fact, an offer beside its absence, a bound beside a value, a derived fact
   beside the facts it is derived from, and a counter's modulus above its width or beside a saturating wrap are each
   `invalid-description`, as is an offer of a statement fact. A service's offers are read and refused alike, and judged
   against nothing.
4. **A side is read whole** (`crates/eadl-resolve/src/requirement.rs`). Every `requires`, `needs` and `uses` a
   declaration writes, at the positions §2's presence reads, is read as one side, so a contradiction split across two
   clauses is one: constraints on one fact that no value satisfies together are `invalid-description`, and what the
   exact arithmetic cannot decide is `unsupported-profile`. The positions hold a grammar, and a form written where it
   does not belong is `invalid-description`. A declaration whose local name is a vocabulary fact is refused, at the
   root and inside every imported module.
5. **Each fact has one outcome at a provider, and a requirement is judged against it**
   (`crates/eadl-resolve/src/relation.rs`). The outcome is, in order, the value offered; absent; unknown; derived by the
   fact's rule — a counter's horizon is its modulus less one over its rate; or undescribed. A value satisfies a
   requirement when it lies in the requirement's written direction: the fact's own, or equality under `exactly`. A set
   is judged by inclusion, which reads no order, so a stronger precondition never passes as a stronger capability. The
   relation chooses no provider and claims no lattice: it lists every provider's answer, with the value found, the
   derivation used and the direction, for the resolver to choose from.
6. **Codes.** The relation reports every refusal under the record's two codes, rows of §6:
   `invalid-description` for what a description writes that cannot mean anything, and `unsupported-profile` for a fact
   no rule decides or a comparison past the arithmetic. The diagnostic names the fact and what is wrong, says what to
   do, and points where it is written (`crates/eadl-resolve/src/refusal.rs`'s `Refusal::diagnostic`); the library
   keeps each rule a typed `Cause` for any consumer that must tell one from another. A code of its own per cause, as
   the reference's §4 rule 3 asks of a new rule, is declined for now: while `archogen check` does not call the
   relation, each would be a row whose `fires on` no command reaches, naming a rule its typed cause and its message
   already name. `M3.4`, which makes the causes reachable from a command, takes the question up again.

## 8. Whether the bindings of one assignment hold together

The relation judges one binding at a time (§7). An **assignment** binds each requester's requirement to one provider
the relation satisfies it at, and these rules judge the bindings together (`crates/eadl-resolve/src/joint.rs`). Their
design is `M3.3`'s in `docs/tasks/M3.md`, and connectivity's and routes' `M3.3.3`'s, each reviewed once by a context that
did not write it, as are privilege's, placement's, the initialization graph's and the functions', `M3.3.4`'s to
`M3.3.7`'s. `archogen check` does not call them yet: `M3.4` searches for an assignment and wires both in. The
configuration checker judges a plan's bindings again by those that refuse an assignment — rules 1 to 3, 5, 7, 9, 10,
13, 14 and 16, with rule 4's node counted once by rule 3, an adapter's instance among them (rule 15) — as its check (5),
written from this section alone (`crates/archogen-plan-check/src/joint.rs`, `M3.5.5`).

1. **Requesters.** A requester is a task, for what the task itself writes; a service; or any other declaration, for
   what it writes at its own level outside its tasks. Two tasks of one system are two requesters, though §7 reads them
   as one side. A platform's or a block's `uses` is composition, never a binding.
2. **Consumed facts.** A vocabulary entry marked `use consumed` — `relative-delay` and `absolute-deadline` in `/1` —
   is a function a requester programs. A binding takes a channel when its requirement is on a consumed fact and is met
   by the value `true`: `(needs f)`, `(f true)` or the group whose head it is. `(f false)` takes none, and a shared
   fact never does, however many read it.
3. **Channels** (F08). The requesters taking channels at one provider number at most its `channels`; more is a conflict.
   A requester counts once at a provider, however many of its bindings take a channel there. One requester needs no
   count. Two or more where `channels` is undescribed, or offered without a value, is `missing-fact`. A provider that
   offers a consumed function `true` and declares `channels` absent, or `0`, is `invalid-description`, refused when it
   is read, whether or not anything binds it.
4. **A node several users reach is one requester** (`M4.1.2.4`; `docs/decisions/decision_runtime-plan.md` §2.9). A
   node of one system's initialization graph (rule 12) bound for a requirement that takes a channel (rule 2) — a
   service, a policy, or a block or a platform bound for requirements of its own; an adapter's instance aside, whose
   kind's obligations say how it shares — counts once at the provider whose channel it takes, however many reach it
   (rule 3). Its **users** are the jobs and the interrupt routines of its system that can call it:
   - each task of the system that **reaches** it. A requester reaches what it names by `uses` or `needs` and the
     provider each of its bindings names, at any depth — but for a block's or a platform's own `uses` or `needs`, which
     name only hardware (rules 1 and 7) and are never a call;
   - each route of the system (rule 8) whose interrupt **runs** it. The interrupt of a source or a timer route at
     provider `X` runs the code of `X`, where `X` is a node; of each requester the route names that is a node bound to
     `X` itself — for `interrupt-source` met by `true` where the route is a source, by a binding that takes a channel
     where it is a timer route; where it is a timer route, of each instance the system serves bound to `X` by a binding
     that takes a channel, and of each requester the route names that is a node bound to such an instance, where the
     instance's kind's cost runs its requester's code — `expiry`, a delivery to its requester, does, and `wake`, an
     epoch advance no requester observes but by reading, does not (`docs/specs/adapters/decision_engine-adapters.md`
     §9); and of every node these reach;
   - but not its own timer route — a timer route at the provider whose channel it takes, or at the provider the instance
     whose channel it takes programs — unless another node that route runs reaches it: that route's own code delivering
     to it is no caller, another node's code there calling it is.

   The system's own level is no user: its `uses` says what the system includes. Another system's tasks and routes are
   that system's (rule 7), and a node naming itself is not its own user. A node with two or more users is the
   **multiplexer**: its realization serves them through the channel it takes, and the assignment reports that it owes
   them the shared semantics and a bounded overhead (`ROADMAP.md` §5.4), for the realization and the analysis to
   discharge — an obligation, never a finding. A multiplexing adapter is likewise a provider with more channels and
   itself one requester of the channel beneath it.
5. **Ownership** (F09). A provider owns each `(region NAME (base B) (size S))` it offers: the bytes `[B, B+S)`, `B` a
   non-negative address and `S` a positive whole number of bytes. Its other sub-forms are `(executable true)` or `false`
   and `(reserved true)` or `false`; each sub-form at most once, and any other, a second one or another value is
   `invalid-description` (`M3.3.5.1`). Placement memory — executable, unreserved — is rule 10's, owned by none. Two
   distinct providers both bound in an assignment — named by a binding; the placement a plan gives binds nothing — whose
   bytes overlap are a conflict, unless one composes the other: names it through `uses`, at any depth through blocks and
   platforms, whose `uses` alone is composition (rule 1); a service's `uses` is a binding and composes nothing. A region
   without `size` owns at least its base byte; where another bound provider owns bytes above its base, whether they
   overlap is `missing-fact`. A region without a base — the bare symbol `region` among them, which names no byte range
   (`M4.1.19`) — with two, or with a size that is not a positive whole number of bytes is `invalid-description`, bound
   or not, composed or not; one past the exact arithmetic, a decimal too precise to hold exactly (§1 rule 5) among them,
   which no one can say is whole (`M4.1.20`), `unsupported-profile`. A provider nothing is bound to owns
   nothing: an unused device costs nothing (`ROADMAP.md` §5.3) but the bytes it occupies in memory, which rule 10
   charges.
6. **One assignment, then the search.** A conflict refuses the assignment it is found in, and an unknown blocks it. When
   no assignment holds, the description takes the highest code, by precedence, of one assignment's findings: the first
   blocked by unknowns alone, `missing-fact`, or, where every assignment has a conflict, the first's conflicts,
   `infeasible-configuration` — each finding its own code where it names one, as rule 9's stated level, rule 13's
   comparisons past the exact arithmetic and rule 16 name `unsupported-profile` — `M3.4`'s search to conclude, never a
   single assignment's. Each finding names the provider, the requesters with where they require the fact, and the count
   or the two ranges, with a repair: another provider, a sharing realization, or the fact described.
7. **One system's assignment, and its hardware** (`M3.3.3`). An assignment is one system's: a description of several
   systems is resolved system by system, so channels count and ownership compares within one. A system's **hardware** is
   the blocks and platforms its `platform` clause, its own `uses` and `needs` or its tasks' name, and every block or
   platform those blocks and platforms name in turn through `uses` or `needs` — a clause of their own, or inside
   `requires` — at any depth. A declaration of another kind ends the path: a block that only a service names is not
   hardware. `refines` is no edge: a refinement keeps the abstract platform's obligations (§3), not its blocks. A
   requester the system serves — its own level, one of its tasks, or a declaration its closure reaches through `uses`
   and `needs` — bound to a block or platform outside that hardware is a conflict naming the requester, the provider and
   the system: `SR-H15`'s "a provider on a platform the system does not use never satisfying", the joint check's. What
   the description's code then is, is `SR-H18`'s, `M3.4`'s to decide. A service or a policy the system serves that names,
   by `uses` or `needs`, a block or platform outside that hardware is no path into it. That is the description's own
   conflict, judged once for the system whatever the assignment and whatever a plan binds: `infeasible-configuration`,
   naming the namer, the hardware and the system, its repair to compose the hardware into the system's platform or to
   name hardware the platform composes (`M1.49`). It is each serving system's, so a system the name leaves inside its
   hardware is not refused for it. Another system named is `M1.46`'s. An adapter or a catalog record is composed by no
   platform, and takes the system of the requesters bound to it (`M3.2`, `M3.7`).
8. **Routes** (`M3.3.3`). Under one active core, an assignment reports a **route obligation** for each provider its
   system's requesters rely on for an interrupt, one for each provider and kind: a *source*, bound through
   `interrupt-source` at `true`, with what it states of its own `min-arrival-separation`; and a *timer*, bound through a
   consumed fact at `true`. The description's half of a route is rule 7; the controller between source and core, its
   line and context, and the latency are the catalog's, and the plan enables the release timer and every route's
   interrupt, timer routes included, and on a target the platform's non-maskable interrupts and traps, and no other
   (`docs/decisions/decision_runtime-plan.md` §2.5; `M4.1.2.2`), as the analysis requires
   (`docs/specs/analysis/decision_runtime-analysis-variant.md`, *enabled sources*). A route is an obligation, never a
   finding: a second timer, which the analysis does not model, is the analysis's to refuse. Given the system's level
   (rule 9), a route carries it: the level the interrupt is delivered at.
9. **Privilege** (`M3.3.4.1`; `SR-H11`, `SR-H12`). Given the level a plan binds one system's callers to — a level of
   `reachable-at-privilege`'s domain, the profile's — `rt-static-up-v1`'s Privilege row states `machine`
   (`crates/eadl-model/src/profile.rs`), which `M3.4` gives every system it resolves; a level outside the domain is the engine's
   input error, never a description's code — these rules judge **callers**: the system's own level, its tasks, and the
   services and policies its closure reaches, never a block or platform needing another's fact, which has no level. A
   caller stating it calls from levels that do not hold the system's is `unsupported-profile`, judged first, since no
   context at those levels exists under this profile. Then each provider bound for a caller — any provider, an adapter or
   a record as a block — is reachable at the level: a set without it is a conflict; a level undescribed or offered bare
   is an unknown, and one declared absent, reachable from no level, a conflict — the codes the relation gives a requirer
   that states a level (`SR-H18`), so `SR-H12`'s "refused" is the assignment's, and the description's only by rule 6.
   With no level given, these rules say nothing. One block's absent level beside another's offered one is two
   declarations' statements, which §2 rule 1 no longer refuses (leaf `M3.4.8`), so these rules judge them, each
   provider on its own.
10. **Placement** (`M3.3.5.1`; `SR-H13`, amended `2026-10-22`). Judged once per system, over its hardware, never per
   assignment. **Placement memory** is a region `(executable true)` and not `(reserved true)`: where a plan places the
   image — M3 chooses the region, `M4.1` the offsets. A reserved region overlapping it is a **hole** the plan avoids,
   and each placement region is reported with its holes as a **placement obligation**, for `M4.1` to lay out around and
   `M4.4` (F20) to verify. A region neither executable nor reserved — a device's registers — overlapping placement
   memory, or two placement regions overlapping, are two claims on the same bytes, and placement memory reserved in
   every byte leaves nothing to place in: each `invalid-description`, bound or not, composed or not, a byte surely
   reserved being a sized hole's or the base of one with no size. Placement memory with no size is `missing-fact`,
   naming it: nothing can be laid out in it, and it has no end to judge its holes against. Where a device's registers
   or a reserved range, whose extent is the hole the plan avoids, has no size and a base below placement memory, whether
   they overlap is `missing-fact`, as rule 5 reads it. A reserved range with no size whose base lies inside placement
   memory with a size is surely a hole, taken to reach that region's end: the conservative extent, the largest the region
   allows, which the search, the checker and the layout read alike. That extent is the region's alone: below a second
   placement region the range is still the unknown against it, and rule 5, between bound providers, still reads it as
   owning its base byte. Where a placement region's holes, those with a size at their size and those with none so taken,
   leave it no free byte and the bytes surely reserved do not fill it, the system is `missing-fact`, naming each hole of
   no size in it and asking for its size (`M3.5.5.7.3`, `M4.1.2.5`); free bytes in another placement region do not
   answer it, as they do not answer memory reserved in every byte. A plan records a placement exactly where the system
   has placement memory: one recording none there places the image nowhere, and one naming what is not placement memory
   of the system's hardware, each a conflict of that assignment. A system with no placement memory is said nothing of —
   the hosted playground provides memory — and a system's regions against a target's own map are `M4.4`'s.
11. **Ordering** (`M3.3.5.2`; `SR-H13`). A required ordering — `(ordering (before …) (after …))` — is `unsupported-profile`
   under `rt-static-up-v1`: `ordering` has no entry in `/1`, and a constraint on an undeclared fact is that (§7, record
   §1.1). Its entry and domain wait for the profile that admits `dma`, whose exclusion names ordering among its
   obligations (`docs/profiles/rt-static-up-v1.md`). `docs/semantics/boundary/accept/required-ordering-guarantee.eadl`
   requests nothing, so no closure judges it, and its frozen verdict stays under `M3.4`'s keep-or-migrate rule.
12. **The initialization graph** (`M3.3.7`). From one system's assignment the engine derives which part is initialised
   before which, for `M4.1`'s plan; a description never writes the order
   (`docs/semantics/boundary/reject/initialization-order.eadl`). Its nodes are the providers bound for the requesters
   the system serves — closed over bindings, so an adapter's or a block's own bindings count — the services and policies
   it serves, each block or platform it serves that is bound for requirements of its own (`M4.1.15`), and each block or
   platform a served task, service or policy names by `uses` or `needs`, a call (`M4.1.18`); the system's own level,
   which initialises nothing, and its tasks, which start after all initialisation, are none. An edge runs from a
   provider to each requester bound to it, whatever composes what, but to itself — composition alone giving none
   (`M4.1.16`) — and from each node a service or a policy names by `uses` or `needs`, a block or a platform included
   (`M4.1.17`), to that namer but to itself — a block's or a platform's own names giving none; each carries its reason.
   Nothing is refused: initialisation runs masked, before every release, so a cycle establishes no infeasibility, and
   each strongly connected component is reported as one unit the engine initialises together. This engine initialises
   each node in one phase, a component's members in their texts' order, all before any successor, so that no member's
   initialisation calls another member's code (`docs/decisions/decision_runtime-plan.md` §2.7; `M4.1.2.1`).
13. **One function, one provider** (`M3.3.6`; `SR-H14`). The vocabulary names the function each fact describes, the
   operations of one device that are only valid together (§7 rule 1). One requester's bindings for the facts of one
   function are to one provider: a service reading time reads one counter. Where an assignment binds them to several,
   each of those requirements is judged at each of those providers: if one provider meets them all, the split is that
   assignment's conflict, another binding them whole; if none meets them all and one refutes none — leaving facts
   undescribed or unknown — it is an unknown naming each such provider and the facts it leaves, since describing them
   makes it whole (`SR-H18`'s `missing-fact`; past the arithmetic, `unsupported-profile`); otherwise a conflict naming
   each provider and what it is bound for. So one requester has at most one provider per function, and a description
   wanting two counters writes two requesters. The rule is per requester, so it needs no system, and never spans
   requesters: a task's group head beside its system's own sub-fact, two requesters, is `SR-H17`'s, `M3.4`'s. A
   **qualifier**, `(function each)`, describes every function its requirer relies on: each requirement on one is judged
   at every provider bound for the requester's functions and for those of each requester it reaches through `uses` and
   `needs` — refused or absent a conflict, unknown or undescribed an unknown — and makes no binding of its own; a
   requester relying on no function bound binds it as any requirement. `reachable-at-privilege` is the qualifier rule 9
   judges against the level the system is given, and this rule never repeats it. An adapter offering a function's facts
   is one provider for this rule (`M3.2`).
14. **One time base** (`M3.2.3.1`; `docs/specs/adapters/decision_engine-adapters.md` §4). Where a requester is bound for
   `absolute-deadline`, every other requester it reaches through `needs` and `uses` that is bound for a fact of the
   function `counter` that is not consumed is bound at the same device — a provider being its own device, an adapter's
   instance the provider it programs: a deadline programmed in one counter's time and read in another's is a conflict
   naming both requesters and both devices. Its own facts of the function are rule 13's. It is wider than any one
   service's time on purpose: `/1`'s vocabulary cannot tell a time base from another use of a counter, so a description
   wanting a second counter for another purpose writes it as a requester the deadline's does not reach. Measured over
   every tracked description on `2026-10-27`, it refuses none.
15. **An adapter's instance** (`M3.2`; the record §1, §4), a provider the engine supplies over one device, is judged by
   these rules as one: a provider for its offers and a requester for what its kind requires beneath (rule 1); served by
   a system when a requester it serves is, taking that system (rule 7) — its device must be of the system's hardware,
   the instance itself being none; owning no region and composing nothing, so ownership never compares it and the
   initialization graph keeps its edges, its device before it and it before its requesters (rules 5 and 12); a binding
   to it makes no route, its own binding beneath making its device's timer route, which carries the requesters bound
   to it (rule 8); a caller at its kind's level, stating that level for the callers bound to it (rule 9); one provider
   of its function (rule 13). An exclusive kind's instance serves one requester and takes its own channel beneath
   (rules 2 and 3). Rules 1–14 are amended to that effect, an instance's level judged whether or not a system level is
   given; the search binds one only where no assignment of the hardware alone
   holds (`M3.2.3.2`).
16. **A task released by its own source** (`M4.1.2.3`; `RTP-H24`, `docs/decisions/decision_runtime-plan.md`). A sporadic
   task bound itself to a source for `interrupt-source` at `true` is released on every arrival of that source (the
   runtime plan's §2.2), so the analysis admits it only where the source's stated `min-arrival-separation` is at least
   the task's own `min-separation`: its condition 5, "a task released on every arrival of a source has T ≤ T_s"
   (`crates/rt-analysis/src/runtime.rs`). Judged beside rule 9's judgement of callers, of each served task bound so: a
   separation shorter than the task's, one declared absent — an unbounded source, which the analysis refuses under its
   condition 3 — or a comparison the exact arithmetic cannot decide (§7 rule 2) is a conflict, `unsupported-profile`; a
   source stating no value for it — none, bare, or a bound, which the relation reads as no value — an unknown,
   `missing-fact`. The route rule 8 reports stays an obligation, never a finding. Since the search tries every
   assignment (rule 6), a task whose description offers a source at least as far apart is bound to it, and one whose
   description offers none is refused, naming the task and the source. The rule holds where every arrival releases the
   task, as it does for every task until `M1.44` lets a description say otherwise.
