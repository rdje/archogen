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
| `invalid-description` | a description contradicts itself: a fact both offered and absent (§2 rule 1), or a task with two release models (§4 rule 1) | remove one of the two declarations; only the author knows which was meant | `check (defblock b (offers counter-width) (absent counter-width))` |
| `infeasible-configuration` | a required fact is declared absent (§2 rule 2) | correct the requirement or the platform | `check (defblock timer.counter (offers counter-width) (absent low-power-timer)) (defservice time.lowpower (requires (needs low-power-timer))) (defsystem s (requires (uses time.lowpower)))` |
| `missing-fact` | something the system requires is described nowhere: a required fact (§2 rule 3), a task's release model (§4 rule 1), or the platform a refinement names (§3 rule 4) | declare the fact offered or absent on the platform; give a task a `period` or a `min-separation`; for a refinement, import the module that declares the target, or correct its name | `check (defservice time.monotonic (requires (needs wrap-behavior))) (defsystem s (requires (uses time.monotonic)))` |
| `refinement-violated` | a concrete platform breaks an obligation of the abstract one it refines (§3 rules 1–3) | honour the obligation the diagnostic names: offer the guarantee, give the bounded value, or drop what the abstract declares absent | `check (defplatform soc.abstract (offers counter-width) (absent debug-port)) (defplatform soc.concrete (refines soc.abstract) (offers counter-width debug-port))` |
| `unsupported-profile` | the description asks for something the active profile does not admit (§4 rules 2–4 and 6) | request a profile that supports it, or change the description to fit this one; nothing is silently weakened | `check (defsystem s (task a (period 10 ms) (deadline 10 ms) (priority 1)) (task b (period 10 ms) (deadline 10 ms) (priority 1)))` |
| `tool-failure` | the toolchain could not proceed (§4 rule 7) | this is a failure of the toolchain, not a verdict about the description; report it | `none: only a shipped kind module over 4 GiB reaches it, and the kind modules are embedded in the toolchain (shipped_registry in crates/eadl-model/src/check.rs)` |
| `boundary-implementation-in-description` | a construct is implementation, which eADL does not contain (§5) | move it to the layer the diagnostic names; the description states what, not how | `check (defsystem app.rt (task sensor (period 10 ms) (deadline 10 ms) (wcet 850 us)))` |
