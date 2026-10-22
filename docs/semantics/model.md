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
| `crates/eadl-resolve/src/joint.rs` | §8, whether the bindings of one assignment hold together: channels, owned regions, connectivity, routes and privilege |

The census over this declaration runs in both directions, in `crates/eadl-front/tests/reference.rs`. A code
these sources can emit and §6 does not state is a rule nobody has written down. A code §6 states and no
declared source emits is a rotted row. Both fail the build, as they do for the reference.

⚠️ **`crates/archogen-s0/src/interpret.rs` is deliberately not declared.** It is the S0 prototype, and it
carries an expiry that `S0-RETIREMENT` enforces (`docs/decisions/decision_s0-retirement.md`). Declaring it
normative would freeze a surface built to be deleted. The consequence is stated rather than left:
`analysis-inconclusive`, which only the prototype emits today, is stated by no normative document. S0's
own uses of `invalid-description` and `unsupported-profile` state S0's limits, such as "the S0 realization
needs whole-millisecond periods", not the rules below. The book's rendering of any of them is checked
for existence (the code is one a production source emits), not for meaning.

## 1. Quantities carry a unit, and the unit is one of a small set

A quantity is written as a number followed by a unit, such as `10 ms` or `48 MHz`
(`crates/eadl-model/src/quantity.rs`, `Quantity::read`).

1. **A bare number is not a quantity.** Nothing would say what it measures, so it could be compared with
   nothing: `quantity-missing-unit`.
2. **The units are an explicit table, not SI prefixes applied freely** (`UNITS`). Time is `s`, `ms`, `us`
   and `ns`; frequency is `Hz`, `kHz`, `MHz` and `GHz`; information is `bit` and `byte`. A table that
   accepted any prefix would accept `Ps` and `mHz` too, and a typo that parses is worse than one that
   does not: `quantity-unknown-unit`.
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

1. **A fact is not both offered and absent.** A contradiction is not resolved by preferring one side,
   because only the author knows which half was meant: `invalid-description`.
2. **A required fact that is declared absent makes the configuration infeasible.** Either the requirement
   or the platform is wrong: `infeasible-configuration`.
3. **A required fact that nothing describes is missing.** It lies inside the dependency closure of what
   the system requests, so no decision that depends on it can be made: `missing-fact`.
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
   one. An explicit absence is a constraint something relies on, not an omission to fill in.
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
| `invalid-description` | a description contradicts itself: a fact both offered and absent (§2 rule 1), or a task with two release models (§4 rule 1); or the substitutability relation refuses what it writes, each refusal naming its cause (§7 rule 6); or a provider offers a consumed function with no channel, or a region that names no byte range (§8 rules 3 and 5) | remove one of the two declarations; only the author knows which was meant | `check (defblock b (offers counter-width) (absent counter-width))` |
| `infeasible-configuration` | a required fact is declared absent (§2 rule 2); or an assignment gives a provider more requesters than channels, or two bound providers overlapping bytes, or a requester bound to a block or platform outside its system's hardware (§8 rules 3, 5 and 7), or a caller bound to a provider not reachable at its system's level, or declared reachable from none (§8 rule 9), once no assignment avoids it | correct the requirement or the platform | `check (defblock timer.counter (offers counter-width) (absent low-power-timer)) (defservice time.lowpower (requires (needs low-power-timer))) (defsystem s (requires (uses time.lowpower)))` |
| `missing-fact` | something the system requires is described nowhere: a required fact (§2 rule 3), a task's release model (§4 rule 1), or the platform a refinement names (§3 rule 4); or what §8 cannot judge an assignment without — a provider's `channels`, a region's size, a bound provider's level (§8 rules 3, 5 and 9) | declare the fact offered or absent on the platform; give a task a `period` or a `min-separation`; for a refinement, import the module that declares the target, or correct its name | `check (defservice time.monotonic (requires (needs wrap-behavior))) (defsystem s (requires (uses time.monotonic)))` |
| `refinement-violated` | a concrete platform breaks an obligation of the abstract one it refines (§3 rules 1–3) | honour the obligation the diagnostic names: offer the guarantee, give the bounded value, or drop what the abstract declares absent | `check (defplatform soc.abstract (offers counter-width) (absent debug-port)) (defplatform soc.concrete (refines soc.abstract) (offers counter-width debug-port))` |
| `unsupported-profile` | the description asks for something the active profile does not admit (§4 rules 2–4 and 6); or the substitutability relation meets a constraint on a fact no rule decides, or a comparison past the exact arithmetic (§7 rule 6), a region's range among them (§8 rule 5); or a caller stating a level its system does not run its callers at (§8 rule 9) | request a profile that supports it, or change the description to fit this one; nothing is silently weakened | `check (defsystem s (task a (period 10 ms) (deadline 10 ms) (priority 1)) (task b (period 10 ms) (deadline 10 ms) (priority 1)))` |
| `tool-failure` | the toolchain could not proceed (§4 rule 7) | this is a failure of the toolchain, not a verdict about the description; report it | `none: only a shipped kind module over 4 GiB reaches it, and the kind modules are embedded in the toolchain (shipped_registry in crates/eadl-model/src/check.rs)` |
| `boundary-implementation-in-description` | a construct is implementation, which eADL does not contain (§5) | move it to the layer the diagnostic names; the description states what, not how | `check (defsystem app.rt (task sensor (period 10 ms) (deadline 10 ms) (wcet 850 us)))` |

## 7. Whether an offer satisfies a requirement

The substitutability relation decides whether what one provider offers satisfies one requirement. Its design is
`docs/decisions/decision_substitutability-relation.md`, reviewed until a round found no defect, and its executable
model is `crates/eadl-resolve/src/model/`; this section is normative over the production code in `crates/eadl-resolve`,
which is held to the model over the model checker's whole universe (`crates/eadl-resolve/tests/production.rs`).
`archogen check` does not call the relation yet: `M3.4` wires it, keeping or migrating every verdict it moves.

1. **The vocabulary decides each fact's domain, role and direction.** `docs/semantics/vocabulary/vocabulary.eadl`,
   written with the kind `docs/semantics/kinds/deffact.eadl`, is read into a typed table
   (`crates/eadl-resolve/src/vocabulary.rs`). Each entry's frame is held to the kind; then the table refuses what no
   frame can say — a direction its domain does not admit, a derivation without the rule that computes it or with
   inputs that rule does not read, a cycle among derivations, `implies` off a set fact, `use consumed` off a boolean
   fact or a group's head, a name twice, a fact named like a clause word. A fact the table does not declare may be offered, absent or needed, and presence judges it (§2); a
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
did not write it. `archogen check` does not call them yet: `M3.4` searches for an assignment and wires both in.

1. **Requesters.** A requester is a task, for what the task itself writes; a service; or any other declaration, for
   what it writes at its own level outside its tasks. Two tasks of one system are two requesters, though §7 reads them
   as one side. A platform's or a block's `uses` is composition, never a binding.
2. **Consumed facts.** A vocabulary entry marked `use consumed` — `relative-delay` and `absolute-deadline` in `/1` —
   is a function a requester programs. A binding takes a channel when its requirement is on a consumed fact and is met
   by the value `true`: `(needs f)`, `(f true)` or the group whose head it is. `(f false)` takes none, and a shared
   fact never does, however many read it.
3. **Channels** (F08). The requesters taking channels at one provider number at most its `channels`; more is a
   conflict. One requester needs no count. Two or more where `channels` is undescribed, or offered without a value, is
   `missing-fact`. A provider that offers a consumed function `true` and declares `channels` absent, or `0`, is
   `invalid-description`, refused when it is read, whether or not anything binds it.
4. **A service several users reach is one requester.** Its users are the tasks and services that use it; a system's
   own `uses` says what the system includes, and is no second user. Such a service's realization serves them through
   the channel it takes, so it is the multiplexer: the assignment reports that it owes them the shared semantics and a
   bounded overhead (`ROADMAP.md` §5.4), for the realization and the analysis to discharge. A multiplexing adapter is
   likewise a provider with more channels and itself one requester of the channel beneath it.
5. **Ownership** (F09). A provider owns each `(region NAME (base B) (size S))` it offers: the bytes `[B, B+S)`, `B` a
   non-negative address and `S` a positive whole number of bytes; any other sub-clause is another rule's. Two distinct
   providers both bound in an assignment whose bytes overlap are a conflict, unless one composes the other through
   `uses`, at any depth. A region without `size` owns at least its base byte; where another bound provider owns bytes
   above its base, whether they overlap is `missing-fact`. A region without a base, with two, or with a size that is
   not a positive whole number of bytes is `invalid-description`; one past the exact arithmetic `unsupported-profile`.
   A provider nothing is bound to owns nothing: an unused device costs nothing (`ROADMAP.md` §5.3).
6. **One assignment, then the search.** A conflict refuses the assignment it is found in, and an unknown blocks it.
   The description is `infeasible-configuration` when every assignment has a conflict, and `missing-fact` when some
   assignment is blocked by unknowns alone — `M3.4`'s search to conclude, never a single assignment's. Each finding
   names the provider, the requesters with where they require the fact, and the count or the two ranges, with a
   repair: another provider, a sharing realization, or the fact described.
7. **One system's assignment, and its hardware** (`M3.3.3`). An assignment is one system's: a description of several
   systems is resolved system by system, so channels count and ownership compares within one. A system's **hardware**
   is the blocks and platforms its `platform` clause, its own `uses` and `needs` or its tasks' name, and every block or
   platform those name in turn through `uses` or `needs` — a clause of their own, or inside `requires` — at any depth.
   `refines` is no edge: a refinement keeps the abstract platform's obligations (§3), not its blocks. A requester the
   system serves — its own level, one of its tasks, or a declaration its closure reaches through `uses` and `needs` —
   bound to a block or platform outside that hardware is a conflict naming the requester, the provider and the system:
   `SR-H15`'s "a provider on a platform the system does not use never satisfying", in the checker's form. What the
   description's code then is, is `SR-H18`'s, `M3.4`'s to decide. An adapter or a catalog record is composed by no
   platform, and takes the system of the requesters bound to it (`M3.2`, `M3.7`).
8. **Routes** (`M3.3.3`). Under one active core, an assignment reports a **route obligation** for each provider its
   system's requesters rely on for an interrupt: a *source*, bound through `interrupt-source` at `true`, with what it
   states of its own `min-arrival-separation`; and a *timer*, bound through a consumed fact at `true`. The description's
   half of a route is rule 7; the controller between source and core, its line and context, and the latency are the
   catalog's, and the plan enables each such interrupt and no other, as the analysis requires
   (`docs/decisions/decision_runtime-analysis-variant.md`, *enabled sources*). A route is an obligation, never a finding:
   a second timer, which the analysis does not model, is the analysis's to refuse. Given the system's level (rule 9), a
   route carries it: the level the interrupt is delivered at.
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
   With no level given, these rules say nothing. At `archogen check`, §2 rule 1 refuses one block's absent level beside
   another's offered one first, description-wide.
