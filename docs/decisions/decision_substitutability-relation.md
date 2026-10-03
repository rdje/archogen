# The substitutability relation: when an offer satisfies a requirement, over a decidable fragment

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active` — written; under independent review (leaf `M3.1.1`'s closure rule: the first round that
  finds no defect closes it); round 1 answered `2026-10-03`
- **Owner / source:** leaf `M3.1.1` (`docs/tasks/M3.md`). `ROADMAP.md` §5.2 asks for "explicit matching rules in a
  decidable fragment" with "a documented comparison direction" per parameter, declared cross-field implications,
  and no stronger precondition "silently accepted as stronger capabilities"; §5.3 for a "versioned capability
  vocabulary"; §4.2 names the component `eadl-resolve`. The corpus the rules are written against is the one the
  repository tracks: 37 fact names across 124 descriptions, counted `2026-10-03` (§1).

## The fact / decision

The engine decides whether what one party **offers** satisfies what another **requires** by one relation, fact by
fact, over a small set of value kinds in which every comparison is decidable by exact arithmetic. Every fact a
description writes is an entry of a **versioned vocabulary**, which fixes the fact's **domain**, its **role** —
a guarantee the offer makes, or an operating condition the offer imposes — and its **direction**: which way an
offered value must lie against a required one. A requirement is satisfied when every fact it constrains has a
value at the offer, directly or by a derivation the vocabulary names, and each value lies in its fact's direction;
and when every condition the offer imposes is one the requirer meets, compared the other way round. Nothing is
inferred from a value being "more" unless the vocabulary says more is better for that fact, and a stronger
precondition is never a stronger capability, because the vocabulary says which facts are conditions and the
relation reads those backwards. The relation is a predicate between one requirement and one offer. It induces a
preorder on the values of each fact and nothing more: no join, no meet, no best offer, and the word *lattice* is
not used (§7).

### 1. What is compared

**The surface today.** An offer writes a fact bare, `(offers counter-width)`; with a value, `(counter-width 32 bit)`,
`(observable-output true)`, `(tick-unit ns)`, `(counter-modulus 4294967296)`, `(frequency (range 1 MHz 200 MHz))`;
or with a bound, `(counter-width (at-least 32 bit))`, which only an abstract platform writes (model §3). A
requirement writes a constraint inside `requires`: a bound, `(unambiguous-horizon (at-least 60 s))`; a value,
`(preemptive true)`, `(available-in-state idle)`, `(queue-capacity 8 tick)`; a list, `(priorities static unique)`;
or a group, `(absolute-deadline (supported-horizon (at-least 10 s)) (delivery-bound (at-most 50 us)))`. `absent`
names a fact the provider does not have; `needs` names the facts and services a declaration depends on, which is
what puts a fact inside the closure (model §2); `uses` names declarations. A constraint's head is a constraint on a
fact, not a claim that the fact is available, as `examples/alternative-timer/system.eadl` says of itself.

**Definitions.**

- A **fact** is a name of the vocabulary (§1.1). A **provider** is a block or a platform of the description or, once
  the catalog's records exist (`M2.7.4.5`), a record's contract facet (`decision_catalog-records.md` §2); the
  relation is the same for both.
- An **offer** is a provider's statement about a fact: a value of the fact's domain; a bound, in the fact's
  direction; or bare presence, which for a boolean fact is the value `true` and for any other domain is presence
  without a value (§2).
- A **requirement** is a constraint on a fact: a bound written with its direction, `(f (at-least v))`; a bare value,
  `(f v)`, which is a bound in the fact's own direction; a group, `(f (g …) (h …))`, which requires the boolean head
  `f` and each sub-constraint; or, through `needs`, bare presence.
- A requirement's **side** is the declaration that writes it — a service, a policy, a system — and the facts that
  side states about itself are what an offer's conditions are checked against (§3, rule 4).

**The census the vocabulary is written from** (`2026-10-03`, every tracked `.eadl`): offers are written for
`bus-width`, `clock-rate`, `core-count`, `counter-modulus`, `counter-width`, `debug-port`, `frequency`,
`interrupt-source`, `low-power-timer`, `min-arrival-separation`, `multicore`, `observable-output`,
`observation-coherent`, `region`, `relative-delay`, `tick-rate`, `tick-unit`, `transfer-engine`, `uart`,
`wrap-behavior`; requirements for `absolute-deadline`, `available-in-state`, `bounded-arrival`, `deadlines`,
`delivery-bound`, `observation-coherent`, `or-through-mediation`, `ordering`, `preemptive`, `priorities`,
`queue-capacity`, `reachable-at-privilege`, `release-accuracy`, `supported-horizon`, `unambiguous-horizon`;
absences for `absolute-deadline`, `debug-port`, `low-power-timer`; and three names only `needs` writes,
`application-mutexes`, `descriptor-format`, `general-ipc` (R1 A12). Names that are a test's own (`p`, `x`,
`something`) are not vocabulary.

#### 1.1 The vocabulary

The vocabulary is an eADL module the engine ships beside its kinds, `docs/semantics/kinds/vocabulary.eadl`, read
by the same registry (`crates/eadl-model/src/kind.rs`) and versioned with the language, `eadl/1`; an entry changes
only by §15's migration process. It is written with one new kind, `deffact`, declared in `core.eadl` with
`defkind` like every other kind, so it is no more privileged than a kind a user adds (reference §7 rule 1) and
carries no behaviour (rule 3): its clauses hold symbols and nothing else.

```text
(deffact unambiguous-horizon
  (doc "how long a time observation stays unambiguous: the counter's modulus over its rate")
  (domain quantity time)
  (role guarantee)
  (direction at-least)
  (derived-from counter-modulus tick-rate)
  (rule horizon-from-modulus-and-rate))
```

| Clause | Holds | Values | Meaning |
| --- | --- | --- | --- |
| `domain` | forms | `boolean`; `count`; `quantity <dimension>`; `interval <dimension>`; `enumeration <symbol>…`, optionally `(ordered <symbol>…)` for a total order, lowest first; `set <symbol>…`; `group <fact>…` | the fact's value kind (§2) |
| `role` | a symbol | `guarantee`, `condition` or `statement` | a guarantee the offer promises; a condition the offer demands of the requirer (§3 rule 4); or a statement the requiring side makes about itself, which constrains no provider and is read only by the rule or the leaf its entry names (§3 rule 6) |
| `direction` | a symbol | `at-least`, `at-most`, `exact`, `includes`, `within` | how an offered value satisfies a required one (§2's table); for a condition, how the offer's demand must lie against what the requirer has |
| `derived-from` | symbols | facts | the facts a derivation computes this one from, when it is not offered directly (§4); the graph these clauses make is acyclic, and the registry refuses an entry that would close a cycle |
| `rule` | a symbol | a rule name | the engine's rule that computes it, named here and listed in §4 |
| `doc` | a string | a string | what the fact means, one line; required, as a kind's `doc` is |

`deffact`'s clauses are `(holds forms)` for `domain`, since `(ordered …)` nests, `(holds values string)` for `doc`,
and `(holds values symbol)` otherwise (reference §7 rule 4). A fact's name lives in the vocabulary, not in a
description's declaration namespace: the module is loaded by the registry as the kinds are, never checked as a
description, so a block named like a fact (`presence.rs`'s own fixtures write one) collides with nothing, and
reference §7 rule 6 — a name declared once — holds among `deffact`s (R1 A15).

A description that writes a fact the vocabulary does not declare may still offer it, declare it absent or need it
— presence judges those, as today — but a **constraint** on it is `unsupported-profile`: no rule decides a
parameter without a documented direction (§8). A requirement or an offer that writes a direction other than the
fact's is `invalid-description`, naming both — with one direction every requirement may write: **`exactly`**.
Equality is the one bound every direction admits, so it never reads a value the wrong way; it refuses what the
fact's direction would accept, never the reverse, and it is how a requirer says that for it the value is the
interface, not a capacity (§6, "more bits"; R1 A4). `exactly` is the written form of the direction §2 calls
`exact`, as `refinement.rs` reads it (R1 A11). The written direction stays in the description, as today's corpus
writes it, because a reader should see which way a bound runs without the vocabulary open; the vocabulary is what
makes it checkable.

### 2. The value domains

Exact arithmetic throughout: a quantity is `eadl-model`'s `Quantity`, compared in its base unit as a `Rational`
over `i128` (model §1); a count is an exact integer in the same type; nothing is rounded, and a comparison or a
derivation whose exact value does not fit is `QuantityError::Overflow`, which the relation reports as
`unsupported-profile`, a named limit of the arithmetic and never a verdict on the requirement (R1 A5). The
fragment is §5.2's: enumerations, booleans, bounded integers, rational quantities with units, intervals, finite
sets, and the restricted arithmetic of §4's rules. No description writes arithmetic; a value that is not of the
fact's domain is `invalid-description`. In the table, `exact` is written `exactly`.

| Domain | An offer writes | A requirement writes | Direction | Satisfied when |
| --- | --- | --- | --- | --- |
| `boolean` | `(f true)`, `(f false)`, or bare `f` for `true` | `(f true)` or `(f false)`; `f` inside `needs` is presence only | `exact` | the values are equal |
| `count` | `(f 8)` or `(f 8 tick)`, a non-negative integer with an optional dimensionless unit | `(f 8 tick)` or `(f (at-least 8 tick))` | `at-least`, `at-most`, `exact` | the integers compare in the direction |
| `quantity <dim>` | `(f 10 MHz)`, a number and a unit of the dimension | `(f (at-least 60 s))`, or `(f 60 s)` in the fact's direction | `at-least`, `at-most`, `exact` | `ComparisonDirection::satisfied_by` in base units; another dimension is `invalid-description` |
| `interval <dim>` | `(f (range lo hi))`, two quantities of the dimension, `lo ≤ hi` | a point `(f v)` or an interval `(f (range lo hi))` | `within` | the required point or interval lies inside the offered one: `o.lo ≤ r.lo` and `r.hi ≤ o.hi` |
| `enumeration a b c` | `(f b)`, one of the alternatives | `(f b)`; with `(ordered …)`, also `(f (at-least b))` | `exact`; with an order, `at-least` or `at-most` | equal; or in the direction over the declared order |
| `set a b c` | `(f a b)`, alternatives without repetition | `(f b)` or `(f a b)` | `includes`, `exact` | the offered set contains every required alternative; or the sets are equal |
| `group g h …` | `(f true)` or bare `f`, and the sub-facts offered on their own | `(f (g …) (h …))` | the head is `boolean exact`; each sub-fact its own | the head holds and every sub-constraint is satisfied, each judged as a requirement on its sub-fact |

A bound an offer writes (an abstract platform's `(counter-width (at-least 32 bit))`) is not a value: it satisfies
no constraint of a requirement, and is the refinement check's business (model §3). A fact offered bare in a domain
other than `boolean` has no value: it satisfies presence, and a constraint on it is `missing-fact` — the value is
described nowhere (§8).

### 3. The relation

For one requirement `R` on fact `f`, written by side `S`, and one provider `P`:

1. **The value.** `f`'s value at `P` is the value `P` offers for `f`; else the value §4's rule for `f` derives from
   the facts that have a value at `P`, offered or themselves derived (R1 A1); else, if `P` declares `f` absent,
   `R` is not satisfied and the configuration is infeasible (model §2 rule 2); else `f` is undescribed at `P`, and
   that is `missing-fact`: **a constraint written by a declaration inside the closure puts its fact inside the
   closure** — a provider's precondition included, as §5.3 lists them — so the fact is one model §2 rule 3 blocks
   on (R1 A2). A declaration outside the closure constrains nothing the enumeration judges (rule 4).
2. **The domain.** The value is of `f`'s domain, or the offer is `invalid-description`; `R`'s constraint is of `f`'s
   domain and written in `f`'s direction, or `R` is `invalid-description`.
3. **A guarantee** is satisfied when the value lies in `f`'s direction against `R`'s bound, by §2's table. `exact`
   admits no "better" value: a tick unit of `us` does not satisfy `ns`, and `20 MHz` does not satisfy a tick rate
   that must be `10 MHz`.
4. **A condition** is read the other way round. A condition fact's value at `P` is what `P` demands of whoever uses
   it — the privilege a call must have, the state the block must be in — and `R` on `S`'s side states what `S`
   has or accepts. It is satisfied when `P`'s demand lies in the fact's direction against `S`'s statement, where
   the direction is written for the demand: `(direction at-most)` on `reachable-at-privilege` says the offer may
   demand at most the privilege the requirer has. A demand the requirer does not state is `missing-fact` on `S`'s
   side. So an offer reachable only at `machine` does not satisfy a requirer at `supervisor`, although `machine`
   is the higher level: more privilege demanded is a stronger precondition, and §5.2 forbids reading it as a
   stronger capability. What forbids it mechanically is where the vocabulary puts the operands: a condition's
   direction is written for the demand, and the relation reads the demand as the offered value and the requirer's
   statement as the bound, so no comparison can put a demand on the capability side; and rule 5 checks every
   condition the offer imposes whether or not the requirer wrote a constraint on it. A condition's direction is
   `at-most` or `exact` in `/1`; an entry that needs `at-least` — a block that must be fed at least some rate —
   is added with its own worked case, so that the reversed reading is checked where it is first used (R1 A8).
5. **A `requires` clause** is satisfied by `P` when each of its constraints is, and every condition `P` imposes is
   met (rule 4); a group's sub-constraints count among its constraints.
6. **A statement** — a fact whose role is `statement`, such as `or-through-mediation` — is the requiring side's
   word about itself. It is checked for its domain and nothing else: it constrains no provider, puts no fact
   inside the closure, and is read by the rule or the leaf its entry names, which for `/1`'s one statement is
   `M3.2`'s adapter search (§6, "mediation"; R1 A3).
7. **Not decided here.** Which of several satisfying providers to take (`M3.4`: a deterministic preferred order in
   engine configuration); whether two providers may both be taken (`M3.3`: capacity, ownership, topology); and
   whether a requirement no provider satisfies can be met by an adapter (`M3.2`: an adapter is a provider whose
   contract this relation judges like any other, with its costs and obligations its own).

### 4. Derived facts and declared implications

A fact the vocabulary marks `derived-from` has a value at `P` when `P` offers it directly, or when every fact it
is derived from has a value at `P` — offered, or itself derived — and the named rule computes it; `derived-from`
is acyclic (§1.1), so the derivation terminates (R1 A1). Where both exist they must agree, or the offer is
`invalid-description`: a counter whose stated modulus is not `2^width` is describing two counters. The rules are
engine knowledge — exact rational arithmetic in `crates/eadl-resolve`, each named, and each listed here. No
implication between facts holds unless a rule names it; `/1` has two, and both assume a modular counter, one
that wraps at its modulus — which is what `counter-modulus` means; a counter that saturates offers no modulus and
derives no horizon (R1 A14):

| Rule | Computes | From | Formula |
| --- | --- | --- | --- |
| `modulus-from-width` | `counter-modulus` (count) | `counter-width` (quantity, information, in bits) | `2^width`; refused when the width is not a whole number of bits or exceeds 126, the largest power of two the arithmetic's `i128` holds (`unsupported-profile`, a named limit; R1 A5) |
| `horizon-from-modulus-and-rate` | `unambiguous-horizon` (quantity, time) | `counter-modulus`, `tick-rate` (quantity, frequency, positive by model §1 rule 3) | `modulus / rate` seconds, exact; a quotient or a comparison past `i128` is `QuantityError::Overflow`, `unsupported-profile` (§2) |

**Worked, from the corpus.** `timer.counter` in `examples/periodic-three/system.eadl` offers `counter-width 32
bit` and `tick-rate 10 MHz`: the modulus is `4294967296`, the horizon `4294967296 / 10 000 000 s = 429.4967296 s`,
and `time.monotonic`'s `(unambiguous-horizon (at-least 60 s))` is satisfied. `timer.delay` in
`examples/alternative-timer/system.eadl`, at `1 MHz`, has a horizon of `4294.967296 s`: slower, and better here. A
16-bit counter at `10 MHz` has a horizon of `65536 / 10 000 000 s = 6.5536 ms` and fails the same requirement: the
faster wrapping counter §5.2 warns of, caught because the direction belongs to the horizon and the derivation to
the vocabulary, not to anyone's sense of which counter is better.

### 5. Candidate enumeration

Over one description, the facts a requirement is checked against are the description's: presence already reads
them into one map (`FactMap`), and the closure says which are relevant. The relation adds two rules to that map.
A fact the vocabulary declares, offered twice with two different values, is a contradiction and is
`invalid-description`, as §5.3 says of a fact both offered and absent — today's map keeps the first offer and says
nothing, which the implementation leaf corrects; an undeclared fact has no domain to contradict, so `region`,
offered once per named region, is untouched (R1 A6; §10). And the closure the enumeration judges by is presence's
closure widened by §3 rule 1: every fact a declaration inside it constrains is inside it too (R1 A2); the
closure `archogen check` reports stays presence's until `M3.4` wires the enumeration and decides whether the
report widens with it. Which provider offers a fact is kept, for the diagnostic and for `M3.3`.

For each declaration in the closure whose `requires` clause writes constraints — a service, a policy, the system —
the enumeration lists, per constraint, the provider whose fact it was judged against and the verdict: satisfied,
with the value and the derivation used; or not, with the fact, the constraint, the value, the direction and the
code of §8. A constraint whose fact no provider in the closure offers or derives is `missing-fact`, by §3 rule 1.
When the catalog's records join (`M3.4`), a record's contract is one more provider, and several records may
satisfy one requirement: the enumeration lists each, and choosing is `M3.4`'s.

What the enumeration is not: it does not look outside the closure (§5.3), it does not resolve `uses` (presence
does), it does not choose, and it does not run inside `archogen check` in `M3.1` (§9).

### 6. The timer cases of §5.2

Each case is a fact of the vocabulary and a row of §2; the worked values are from the corpus or
`docs/semantics/boundary/accept/`.

| Case | Fact | Domain, role, direction | Satisfied, not satisfied |
| --- | --- | --- | --- |
| wrap interval | `unambiguous-horizon` | quantity time; guarantee; `at-least`; derived (§4) | `32 bit` at `10 MHz` → `429.4967296 s ≥ 60 s`; `16 bit` at `10 MHz` → `6.5536 ms`, not satisfied |
| read atomicity | `observation-coherent` | boolean; guarantee; `exact` | offered `true` or bare → satisfied; `false`, or declared absent → not |
| programming range | `supported-horizon`, under the group `absolute-deadline` | quantity time; guarantee; `at-least` | an offer `(absolute-deadline true) (supported-horizon 3600 s)` satisfies `(at-least 10 s)`; `5 s` does not; a provider with `absent absolute-deadline` makes the group infeasible, as `timer.delay` does today (R1 A7, A13) |
| power state | `available-in-state` | set `run idle sleep`; guarantee; `includes` | offered `run idle` ⊇ required `idle`; offered `run` does not include it |
| access privilege | `reachable-at-privilege` | enumeration `(ordered user supervisor machine)`; **condition**; `at-most` | offer demands `supervisor` or `user`, requirer has `supervisor` → satisfied; offer demands `machine` → not: a stronger precondition, never a stronger capability |
| mediation | `or-through-mediation` | enumeration `allowed forbidden`; **statement**; `exact` | the requirer's own word that a mediation boundary may stand between it and the function (§4.3's "allowed mediation boundary"). No rule of `/1` reads it: a privilege test that fails is not satisfied, and a mediated path is an adapter — a provider in its own right, whose contract states the privilege it demands and which this relation judges like any other — that `M3.2`'s search may offer only where the requirer wrote `allowed` (R1 A3) |
| output units | `tick-unit` | enumeration `ns us ms`; guarantee; `exact` | `ns` satisfies `ns`; `us` does not, however a conversion might be arranged: rescaling is an adapter's, a provider in its own right (`M3.2`) |
| more bits | `counter-width` | quantity information; guarantee; `at-least` | `64 bit` satisfies `(counter-width 32 bit)`, a bound in the fact's direction, and an abstract platform's `(counter-width (at-least 32 bit))` is the same bound owed by a refinement; a service for which the width is the interface — one that reads a 32-bit word and relies on what it read — writes `(counter-width (exactly 32 bit))`, which `64 bit` does not satisfy (§1.1; R1 A4). What is longer with a wider counter is the horizon, and §4 derives it, so a requirement on the horizon is written as one |

### 7. Not a lattice

The relation is a predicate on a pair, a requirement and an offer. Per fact, its direction induces a preorder on
the fact's values — `at-least` and `at-most` the order of the numbers or of the declared enumeration, `includes`
set inclusion, `within` interval inclusion, `exact` equality — and that is all the structure claimed: two offers of
one fact may be incomparable, two offers of two facts always are, and no join, meet or best offer is claimed or
used — some of these preorders have joins and meets (two sets under `includes` have a union and an intersection)
and the relation takes none of them (R1 A9). The engine never ranks providers by this relation; `M3.4`'s order
is configuration. Establishing that offers form a lattice would need a proof over the whole vocabulary that
nobody has made, so the word is not used, as §5.2 asks.

### 8. Diagnostics (§5.5)

| Code | When |
| --- | --- |
| `invalid-description` | a value outside its fact's domain; a direction written against the vocabulary's; one fact offered with two values; a derived fact offered with a value its rule disagrees with |
| `missing-fact` | a constrained fact with no value and no derivation at any provider — inside the closure by §3 rule 1, since its constraint is written there; a condition the requirer's side does not state |
| `infeasible-configuration` | a constraint not satisfied; a constrained fact, or a group's head, declared absent |
| `unsupported-profile` | a constraint on a fact the vocabulary does not declare; a derivation past a rule's limit; exact arithmetic that overflows `i128` |

Every diagnostic carries the requirement's span and the offer's, the fact, the constraint as written, the value
found, the direction, the derivation used, and a repair direction; a missing or absent fact names the provider the
closure bound it to.

### 9. Where it lives, and what reads it

`crates/eadl-resolve` (`ROADMAP.md` §4.2's `eadl-resolve`): the vocabulary read into a typed table, the domains,
the relation, the rules of §4 and the enumeration of §5, over `eadl-model`'s `Quantity`, `ComparisonDirection` and
`FactMap`. A library in `M3.1`: `archogen check` does not call it, and every verdict the corpus has today stands.
Wiring it alone would turn every example into `missing-fact`: a requirement whose fact no block or platform
offers — `time.periodic-release`'s `(release-accuracy (at-most 1 ms))`, every policy's `priorities`, `preemptive`
and `deadlines`, `bounded-arrival`, `queue-capacity` — is a constraint on the realization, which a catalog record's
contract supplies (`decision_catalog-records.md` §2), and no record exists until `M2.7.4.5` (R1 A2). The search
(`M3.4`) wires it, over the description's providers and the catalog's. `docs/semantics/model.md` gains a section normative over the crate when
the crate lands (`M3.1.2`), so that `reference.rs` holds its codes to the document as it holds every other
source's.

### 10. What stays open

- A fact offered by two providers with two values is a contradiction in `/1`; scoping facts to a provider — which
  block's counter a service reads — is topology and ownership, `M3.3`'s.
- `region` and `ordering` fit no domain of §2: a region is a name with sub-clauses, offered once per name, and an
  ordering is a pair of events; both are placement and ordering, `M3.3`'s, and both stay undeclared in `/1` —
  presence judges them, and a constraint on either is `unsupported-profile` when the enumeration is wired (R1 A10).
- Conditions are decided for one ordered enumeration and the mediation boolean; a condition over a quantity (a
  clock the block must be fed) takes the same rule 4 and a dimension, and is added to the vocabulary when a
  description needs it.
- Arithmetic in a description stays refused; a new derivation is a new named rule here and in the engine.
- The catalog's contract facet as a provider, and the choice among providers, are `M3.4`'s after `M2.7.4.5`.

## Why

- §5.2 asks for explicit rules in a decidable fragment and a documented direction per parameter. The corpus
  already writes directions in requirements; nothing checked them, and `refinement.rs` compared bounds only
  between an abstract platform and its refinement. The vocabulary makes the direction a property of the fact,
  checked wherever the fact is written.
- The reverse reading of conditions (§3 rule 4) is the one mechanical way to keep §5.2's "stronger preconditions
  are not silently accepted as stronger capabilities": an engine that compared `machine ≥ supervisor` with the
  demand on the capability side would accept it, and no reviewer reads every comparison; the vocabulary fixes
  which side the demand is read on.
- Derived facts (§4) are where §5.2's "faster clock is not universally better" bites: the horizon's direction is
  `at-least` and its derivation divides by the rate, so a faster counter of the same width is worse. A relation
  without derivations would compare widths and rates on their own and get the horizon wrong in either direction.
- The relation is a library first (§9) because wiring it would change the corpus's verdicts before the providers
  that satisfy service-level requirements exist; the use cases' expected outcomes (`examples/README.md`) are a
  sealed fixture of what progress looks like.

## How to apply

- A new fact is a new `deffact` entry with its domain, role and direction; a constraint on an undeclared fact is
  refused, so the entry comes first.
- A new derivation is a new rule in §4's table and in `crates/eadl-resolve`, named by the facts' `rule` clauses.
- A new value kind extends §2's table, and the implementation's domain enumeration, together.
- Related: [[decision_eadl-engine-boundary]] (§4.3's cases are the timer cases of §6), [[decision_catalog-records]]
  (the contract facet as a provider), [[decision_priority-comparison-direction]] (one direction already ruled).

## Review

`M3.1.1`'s acceptance is a review, by a context that did not write this record, finding no requirement the relation
satisfies that it should not — a stronger precondition read as a capability, a value read in the wrong direction,
an implication assumed that nothing declares — and no case of §5.2 misclassified; every finding answered here. The
history is [`decision_substitutability-relation-reviews.md`](../reviews/decision_substitutability-relation-reviews.md).

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 15 | 7 (A1, derivation chaining as written un-satisfied the worked case; A2, a constraint on a fact nobody `needs` judged "nothing"; A3, mediation an undeclared implication over a domain the corpus contradicts; A4, `counter-width` read as more-is-better where the width is the interface; A5, the width limit off by one and the comparison's overflow unstated; A6, the two-values rule refusing the target's three regions; A7, a unit the language refuses) | "not acceptable as it stands"; the §5.2 cases none misclassified in direction, the worked numbers right |
