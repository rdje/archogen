# The substitutability relation: when an offer satisfies a requirement, over a decidable fragment

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active` — written; under independent review (leaf `M3.1.1`'s closure rule: the first round that
  finds no defect closes it); rounds 1 to 7 answered `2026-10-03`
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
and when every condition the offer imposes is one the requirer meets, the demand compared as the offered value
against what the requirer has. Nothing is
inferred from a value being "more" unless the vocabulary says more is better for that fact, and a stronger
precondition is never a stronger capability, because the vocabulary says which facts are conditions and the
relation reads those backwards. The relation is a predicate between one requirement and one offer. It induces a
preorder on the values of each fact and nothing more: no join, no meet, no best offer, and no lattice is claimed
(§7).

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
  the catalog's records exist (`M2.7.4.5`), a record's contract facet (`docs/specs/catalog/decision_catalog-records.md` §2); the
  relation is the same for both.
- An **offer** is a provider's statement about a fact: a value of the fact's domain; a bound, in the fact's
  direction or `exactly` (§1.1); or bare presence, which for a boolean fact, or a group's head, is the value `true`
  and for any other domain is presence without a value (§2; R4 D3). A service may write `offers` too — `core.eadl` admits the clause on
  `defservice`, not on `defpolicy` (R7 G5) — but is not a provider, and its offers are not judged in `/1` (R3 C13).
- A **requirement** is a constraint on a fact: a bound written with its direction, `(f (at-least v))`; a bare value,
  `(f v)`, which is a bound in the fact's own direction; a group, `(f (g …) (h …))`, which requires the boolean head
  `f` and each sub-constraint; or, through `needs`, bare presence.
- A requirement's **side** is the declaration that writes it — a service, a policy, a system, a platform or a block,
  whichever writes a `requires` clause (R5 E2) — and the facts that side states about itself are what an offer's
  conditions are checked against (§3, rule 4).

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
carries no behaviour (rule 3): its clauses hold names, lists of names and one line of text, and nothing that could
be run.

```text
(deffact unambiguous-horizon
  (doc "how long a time observation stays unambiguous: the counter's modulus over its rate")
  (domain quantity time)
  (role guarantee)
  (direction at-least)
  (derived-from counter-modulus tick-rate)
  (reads wrap-behavior)
  (rule horizon-from-modulus-and-rate))
```

| Clause | Holds | Values | Meaning |
| --- | --- | --- | --- |
| `domain` | forms | `boolean`; `count`; `quantity <dimension>`; `interval <dimension>`; `enumeration <symbol>…`, optionally `(ordered <symbol>…)` for a total order, lowest first; `set <symbol>…`; `group <fact>…` | the fact's value kind (§2) |
| `role` | a symbol | `guarantee`, `condition` or `statement` | a guarantee the offer promises; a condition the offer demands of the requirer (§3 rule 4); or a statement the requiring side makes about itself, which constrains no provider and is read only by the rule, or the leaf, named here for it (§3 rule 6; R6 F4) |
| `direction` | a symbol | `at-least`, `at-most`, `exact`, `includes`, `within` | how an offered value satisfies a required one (§2's table); for a condition, how the offer's demand must lie against what the requirer has |
| `derived-from` | forms | facts | the facts a derivation computes this one from, when it is not offered directly (§4); the graph these clauses make is acyclic, and the registry refuses an entry that would close a cycle |
| `reads` | forms | facts | the optional inputs the rule reads beside `derived-from`'s required ones (§4; R5 E9) |
| `rule` | a symbol | a rule name | the engine's rule that computes it, named here and listed in §4 |
| `doc` | a string | a string | what the fact means, one line; required, as a kind's `doc` is |

`deffact`'s clauses are `(holds forms)` for `domain`, since `(ordered …)` nests, and for `derived-from` and `reads`,
which list facts; `(holds values string)` for `doc`; and `(holds values symbol)` for `role`, `direction` and
`rule`, each exactly one value — reference §7 rule 4 makes `values` a fixed number, and `kind.rs` refuses any
other as `schema-arity` (R2 B3). `doc`, `domain`, `role` and `direction` appear exactly once, `derived-from`,
`reads` and `rule` at most once (R3 C11). A fact's name lives in the vocabulary, not in a
description's declaration namespace: the module is loaded by the registry as the kinds are, never checked as a
description, so a block named like a fact (`presence.rs`'s own fixtures write one) collides with nothing, and
reference §7 rule 6 — a name declared once — holds among `deffact`s (R1 A15).

A description that writes a fact the vocabulary does not declare may still offer it, declare it absent or need it
— presence judges those, as today — but a **constraint** on it is `unsupported-profile`: no rule decides a
parameter without a documented direction (§8). A requirement, or an offer that writes a bound, written in a
direction other than the fact's is `invalid-description`, naming both — with one direction either may always
write: **`exactly`** (R1 A4; R2 B4). Equality is the one bound every direction admits, so it never reads a value
the wrong way; it refuses what the fact's direction would accept, never the reverse, and it is how a requirer says
that for it the value is the interface, not a capacity (§6, "more bits"). `exactly` is the written form of the
direction §2 calls `exact`, as `refinement.rs` reads it (R1 A11). The written direction stays in the description,
as today's corpus writes it, because a reader should see which way a bound runs without the vocabulary open; the
vocabulary is what makes it checkable.

**The entries of `/1`**, one per fact the census writes, each direction with its reason; `M3.1.2` ships them as
`vocabulary.eadl` and adds none without a row here (R2 B8). A fact marked *interface* takes `exact` because a
different value is a different interface, not a better one (§5.2); a *capacity* takes the direction in which more
of it satisfies more.

| Fact | Domain | Role | Direction | Why |
| --- | --- | --- | --- | --- |
| `absolute-deadline` | group `supported-horizon delivery-bound` | guarantee | `exact` | a boolean head |
| `application-mutexes`, `general-ipc`, `descriptor-format` | boolean | guarantee | `exact` | presence facts; the first two are §3.1 exclusions |
| `available-in-state` | set `run idle sleep` | guarantee | `includes` | the offer covers every state required |
| `bounded-arrival` | boolean | guarantee | `exact` | |
| `bus-width` | quantity information | guarantee | `exact` | interface: a wider bus is another bus |
| `clock-rate` | quantity frequency | guarantee | `exact` | interface: a block's timing is its rate's |
| `core-count` | count | guarantee | `exact` | two cores do not satisfy a uniprocessor profile (§3.1: one active core) |
| `counter-modulus` | count | guarantee | `exact` | interface: wrapping arithmetic depends on the modulus itself (§3.1's "explicit counter modulus"), as `quantity.rs`'s own example of `Exact` says; what wraps later is the horizon, which has its own entry; offered, never derived (§4; R5 E4; R6 F1) |
| `counter-width` | quantity information | guarantee | `at-least` | capacity: a wider register holds every narrower modulus, and a modulus offered beside it may not exceed `2^width` (§4); `exactly` where the width is the interface (§6) |
| `deadlines` | enumeration `constrained arbitrary` | guarantee | `exact` | a policy's word |
| `debug-port`, `interrupt-source`, `low-power-timer`, `multicore`, `observable-output`, `observation-coherent`, `relative-delay`, `transfer-engine`, `uart` | boolean | guarantee | `exact` | |
| `delivery-bound` | quantity time | guarantee | `at-most` | a shorter delivery satisfies |
| `frequency` | interval frequency | guarantee | `within` | the rates the block can run at, a capability the offered range covers; the clock a block must be fed is a condition and another fact (§10; R4 N1) |
| `min-arrival-separation` | quantity time | guarantee | `at-least` | arrivals further apart satisfy a bound on separation |
| `or-through-mediation` | enumeration `allowed forbidden` | statement | `exact` | the requirer's word, read by `M3.2` (§3 rule 6) |
| `preemptive` | boolean | guarantee | `exact` | |
| `priorities` | set `static unique dynamic` | guarantee | `exact` | a policy's word, like `deadlines`: `unique` is a demand on the task set, not a capability, so a realization offering `static unique` does not satisfy a policy that wrote `static` alone; a policy that wants both writes both (R3 C4) |
| `queue-capacity` | count | guarantee | `at-least` | capacity |
| `reachable-at-privilege` | enumeration `(ordered user supervisor machine)` | condition | `at-most` | the offer demands no more privilege than the requirer has (§3 rule 4) |
| `release-accuracy` | quantity time | guarantee | `at-most` | a tighter accuracy satisfies |
| `supported-horizon` | quantity time | guarantee | `at-least` | capacity |
| `tick-rate` | quantity frequency | guarantee | `exact` | interface: a faster counter is another counter, and what is longer or shorter with it is derived (§4) |
| `tick-unit` | enumeration `ns us ms` | guarantee | `exact` | interface (§4.3, representation units) |
| `unambiguous-horizon` | quantity time | guarantee | `at-least` | capacity; derived from `counter-modulus` and `tick-rate`, and offered only where neither is, nor `wrap-behavior` (§4) |
| `wrap-behavior` | enumeration `modular saturating` | guarantee | `exact` | read by §4's rule |

`region` and `ordering` have no entry in `/1` (§10).

### 2. The value domains

Exact arithmetic throughout: a quantity is `eadl-model`'s `Quantity`, compared in its base unit as a `Rational`
over `i128` (model §1); a written count is the language's exact 64-bit integer — or, where a count is a power of
two that literal cannot hold, `(pow2 N)` with `N` at most 126, the modulus of a 64-bit timebase being `(pow2 64)`
(R7 G1) — and a written magnitude its digits with a scale (reference §1, `Rational::decimal`), refused above that as
it is read, while `i128` is the arithmetic's,
where a derivation's value lives (R2 B7; R4 N7); nothing is rounded, and a derivation, or the unit conversion inside a
comparison, whose exact value does not fit is `QuantityError::Overflow` (R5 E8), which the relation reports as `unsupported-profile`, a named limit of the arithmetic and
never a verdict on the requirement — `unsupported-profile` rather than `analysis-inconclusive` because the limit is
the implemented value domain's (model §1), not a search's budget: a description whose numbers pass it asks for a
wider domain, which §15 versions (R1 A5). The
fragment is §5.2's: enumerations, booleans, bounded integers, rational quantities with units, intervals, finite
sets, and the restricted arithmetic of §4's rules. No description writes arithmetic; a value that is not of the
fact's domain is `invalid-description`. In the table, `exact` is written `exactly`.

| Domain | An offer writes | A requirement writes | Direction | Satisfied when |
| --- | --- | --- | --- | --- |
| `boolean` | `(f true)`, `(f false)`, or bare `f` for `true` | `(f true)`, `(f false)` or `(f (exactly true))`; `f` inside `needs` is the requirement `(f true)`, the mirror of the bare offer (R7 G2, G12) | `exact` | the values are equal |
| `count` | `(f 8)` or `(f 8 tick)`, a non-negative integer with an optional dimensionless unit; or `(f (pow2 64))`, a power of two past the 64-bit literal, `N ≤ 126` (R7 G1) | `(f 8)`, `(f 8 tick)`, `(f (pow2 64))`, or a bound in the fact's direction or `exactly`, `(f (at-least 8 tick))` | `at-least`, `at-most`, `exact` | the integers compare in the direction |
| `quantity <dim>` | `(f 10 MHz)`, a number and a unit of the dimension | `(f (at-least 60 s))` in the fact's direction, `(f (exactly 60 s))`, or `(f 60 s)` in the fact's direction | `at-least`, `at-most`, `exact` | `ComparisonDirection::satisfied_by` in base units; another dimension is `invalid-description` |
| `interval <dim>` | `(f (range lo hi))`, two quantities of the dimension, `lo ≤ hi`; or a point `(f v)`, the interval `[v, v]` (R3 C10) | a point `(f v)` or an interval `(f (range lo hi))`, or `(f (exactly (range lo hi)))` (R6 F6) | `within`, `exact` | the required point or interval lies inside the offered one: `o.lo ≤ r.lo` and `r.hi ≤ o.hi`; under `exactly`, the intervals are equal (R3 C7) |
| `enumeration a b c` | `(f b)`, one of the alternatives | `(f b)` or `(f (exactly b))`; with `(ordered …)`, also `(f (at-least b))` or `(f (at-most b))` in the fact's direction | `exact`; with an order, `at-least` or `at-most` | equal; or in the direction over the declared order |
| `set a b c` | `(f a b)`, alternatives without repetition | `(f b)`, `(f a b)`, or `(f (exactly a b))` | `includes`, `exact` | the offered set contains every required alternative; or the sets are equal |
| `group g h …` | `(f true)` or bare `f`, and the sub-facts offered on their own | `(f true)`, the head alone; or `(f (g …) (h …))`, the head and sub-constraints; a sub-fact is a fact in its own right and may be constrained flat beside the head, as `examples/alternative-timer/system.eadl` writes `(absolute-deadline true)` beside `(supported-horizon (at-least 10 s))` (R2 B6) | the head is `boolean exact`; each sub-fact its own | the head holds and every sub-constraint, nested or flat, is satisfied, each judged as a requirement on its sub-fact |

A bound an offer writes (an abstract platform's `(counter-width (at-least 32 bit))`) is not a value: it satisfies
no constraint of a requirement, and is the refinement check's business (model §3). A fact offered bare in a domain
other than `boolean` or a group's head is **unknown** at that provider (R6 F5): it satisfies presence, a constraint on it
is `missing-fact` whatever a rule could have derived — the bare offer stops §3 rule 1's chain before derivation —
and a rule that reads it derives nothing (§4; R3 C5; R5 E3, E7).

### 3. The relation

For one requirement `R` on fact `f`, written by side `S`, and one provider `P`:

1. **The value.** `f`'s value at `P` is the value `P` offers for `f`; else, if `P` declares `f` absent, `R` is not
   satisfied by `P`, and the configuration is infeasible where no other provider satisfies it (model §2 rule 2) — a
   declared absence is something the offer says, and no derivation corrects what an offer says (§4; R3 C1; R4 N3);
   else, if `P` offers `f` without a value, `f` is unknown at `P` and `R` is `missing-fact` there — a presence
   requirement apart (rule 3; §2; R5 E3); else
   the value §4's rule for `f` derives from the facts that have a value at `P`, offered or themselves derived
   (R1 A1), where an input named in `derived-from` that `P` declares absent leaves `f` underivable and `R`
   unsatisfied by `P` the same way, since what `f` would be derived from is declared not to exist, and an optional
   input declared absent makes the rule derive nothing (§4; R7 G3); else `f` is undescribed at `P`, and that is
   `missing-fact`: **a constraint written by a declaration inside the closure puts its fact inside the
   closure** — a provider's demand included, which rule 4 reads from its offers and §5.3 lists among the closure's
   "provider preconditions" (R7 G9) — so the fact is one model §2 rule 3 blocks
   on (R1 A2). A declaration outside the closure constrains nothing the enumeration judges (model §2 rule 4).
2. **The domain.** The value is of `f`'s domain, or the offer is `invalid-description`; `R`'s constraint is of `f`'s
   domain and written in `f`'s direction or as `exactly` (§1.1), or `R` is `invalid-description` (R2 B4).
3. **A presence requirement** — `f` named in `needs` — is satisfied by any offer of `f`, bare, valued or a bound, or
   by `f`'s derivation where the vocabulary names one (§4; R7 G10), and by nothing else; a boolean `f` named in
   `needs` is the requirement `(f true)`, so `(f false)` satisfies no presence requirement, as `absent` does not (R7
   G2); of rule 1, only its absence and undescribed arms apply otherwise (R6 F3). **A guarantee** is
   satisfied when the value lies in `f`'s direction against `R`'s bound, by §2's table. `exact`
   admits no "better" value: a tick unit of `us` does not satisfy `ns`, and `20 MHz` does not satisfy a tick rate
   that must be `10 MHz`.
4. **A condition** puts the demand on the offered side (R7 G7). A condition fact's value at `P` is what `P` demands of whoever uses
   it — the privilege a call must have, the clock a block must be fed — and `R` on `S`'s side states what `S`
   has or accepts. It is satisfied when `P`'s demand lies in the fact's direction against `S`'s statement, where
   the direction is written for the demand: `(direction at-most)` on `reachable-at-privilege` says the offer may
   demand at most the privilege the requirer has. A demand the requirer does not state is `missing-fact` on `S`'s
   side. So an offer reachable only at `machine` does not satisfy a requirer at `supervisor`, although `machine`
   is the higher level: more privilege demanded is a stronger precondition, and §5.2 forbids reading it as a
   stronger capability. What forbids it mechanically is where the vocabulary puts the operands: a condition's
   direction is written for the demand, and the relation reads the demand as the offered value and the requirer's
   statement as the bound, so no comparison can put a demand on the capability side; and rule 5 checks every
   condition the offer imposes whether or not the requirer wrote a constraint on it. `/1` has one condition,
   `reachable-at-privilege`, and its direction is `at-most`; an entry that needs another — a block that must be fed
   at least some rate — is added with its own worked case, so that the reversed reading is checked where it is
   first used (R1 A8; R2 B2). A demand is a value: a condition fact offered as a bound or bare leaves the demand
   unknown, and rule 5 finds `missing-fact` at that provider for every requirer; a provider that declares a condition
   fact absent imposes no such demand, and rule 5 has nothing to check there — the one place rule 1's absence arm
   does not apply (R5 E5). A provider states its demands in `offers`: a condition fact written in a block's or a
   platform's `requires` is `invalid-description`, since read as that side's statement about itself it would drop
   the demand the author meant (R6 F7).
5. **A `requires` clause** is satisfied by `P` when each of its constraints is, and every condition `P` imposes is
   met (rule 4); a group's sub-constraints count among its constraints.
6. **A statement** — a fact whose role is `statement`, such as `or-through-mediation` — is the requiring side's
   word about itself. It is checked for its domain and nothing else: it constrains no provider, puts no fact
   inside the closure, and is read by the rule or the leaf its entry names, which for `/1`'s one statement is
   `M3.2`'s adapter search (§6, "mediation"; R1 A3). It is the requiring side's word alone: an offer of a statement
   fact is `invalid-description` (R2 B10).
7. **Not decided here.** Which of several satisfying providers to take (`M3.4`: a deterministic preferred order in
   engine configuration); whether two providers may both be taken (`M3.3`: capacity, ownership, topology); and
   whether a requirement no provider satisfies can be met by an adapter (`M3.2`: an adapter is a provider whose
   contract this relation judges like any other, with its costs and obligations its own).

### 4. Derived facts and declared implications

A fact the vocabulary marks `derived-from` is **derived, or stands alone**. At a provider that offers any fact its
rule reads — required or optional, bare or valued — the derived fact is derived and never offered: an offer of it
there is `invalid-description`, since whoever offers the inputs has offered the result's grounds, and the engine
computes it (R4 D1; R5 E1; R6 F1). At a provider that offers none of those facts, the derived fact may be offered,
and the offered value is the value — a claim, as every offer is (§5.3) — such as the horizon a time service
guarantees as a black box over an epoch extender. So the derived fact has a value at `P` when `P` offers it alone,
or when every fact it is derived from has a value at `P`, offered or itself derived, and the named rule computes it;
`derived-from` is acyclic (§1.1), so the derivation terminates (R1 A1). Any input a rule reads, required or
optional, that is offered without a value is unknown, and the rule derives nothing (R3 C5); an optional input
declared absent makes the rule derive nothing too, the conservative branch, while a required one declared absent is
rule 1's (R7 G3). A derived fact offered beside facts the rule does not read stands as a claim: a block offering
`(counter-width 16 bit)` and `(unambiguous-horizon 3600 s)` and no rate or modulus is believed, since a width does
not decide a horizon — a 16-bit modular counter at 18 Hz holds an hour (R7 G6). Every derivation is per
provider: a horizon is computed from one block's modulus and rate, never one block's modulus and another's rate (R2
B5). The rules are engine knowledge — exact rational arithmetic in `crates/eadl-resolve`, each named, and each
listed here. No implication between facts holds unless a rule names it; `/1` has one.

A counter's modulus is offered, never derived from its width. §3.1 asks for an "explicit counter modulus"; a
register's width does not decide where it wraps — a 32-bit register reloaded at a million ticks has a modulus of a
million (R2 B1) — and a rule that guessed `2^width` was where three review rounds in turn found an author's value and
the guess disagreeing, each answer adding a bound and the next round a way round it (R4 D1; R5 E1; R6 F1). What
the width still does is bound the modulus: a `counter-modulus` and a `counter-width` both offered with values must
agree, a modulus above `2^width` being `invalid-description`, as a 32-bit register holds no modulus of `2^33` (R3
C12); a width is a whole number of bits, and a width of 127 bits or more holds every writable modulus, `(pow2 126)`
the largest, so the check is made only below that (R7 G8). The rule is a modular counter's, one that wraps at its modulus — which is what `counter-modulus` means: a
modulus offered beside a `wrap-behavior` that is `saturating` is `invalid-description`, and a saturating counter's
range is another fact, added when a description needs it (R1 A14; R3 C6). The `modular` reading of an omitted
`wrap-behavior` serves the rule alone: a constraint on `wrap-behavior` itself is judged on what is offered, so
against an unstated offer it is `missing-fact` (R4 N4). `derived-from` lists a rule's required inputs; a rule may
also read a named optional one, which the entry's `reads` clause names — `wrap-behavior` here (R5 E9).

| Rule | Computes | From | Formula |
| --- | --- | --- | --- |
| `horizon-from-modulus-and-rate` | `unambiguous-horizon` (quantity, time) | `counter-modulus` (count); `tick-rate` (quantity, frequency, positive by model §1 rule 3); `wrap-behavior`, optional | `modulus / rate` seconds, exact, when `wrap-behavior` is `modular` or undescribed; nothing when it is `saturating`, offered without a value or declared absent (R7 G3); a quotient, or the unit conversion inside a comparison, past `i128` is `QuantityError::Overflow`, `unsupported-profile` (§2; R5 E8) |

**Worked, from the corpus.** `timer.counter` in `examples/periodic-three/system.eadl` offers `counter-width 32 bit`
and `tick-rate 10 MHz` and no modulus, so in the library its horizon is undescribed, `missing-fact` at that provider:
the profile asks for the modulus (§3.1), and when `M3.4` wires the relation the corpus's counters state theirs,
`(counter-modulus 4294967296)`, as `docs/semantics/boundary/accept/counter-width-and-rate.eadl` already does, and
`targets/riscv-virt-up.eadl`'s `target.timer`, the 64-bit `mtime`, `(counter-modulus (pow2 64))` (§2; R7 G1) — a
description gaining a fact §3.1 names, not a requirement weakened (§9). The target's horizon is then
`2^64 / 10 000 000 s = 1 844 674 407 370.9551616 s`, and fits the arithmetic. With it, the horizon is
`4294967296 / 10 000 000 s = 429.4967296 s`, and `time.monotonic`'s `(unambiguous-horizon (at-least 60 s))` is
satisfied; `timer.delay` in `examples/alternative-timer/system.eadl`, at `1 MHz`, has `4294.967296 s`, slower and
better here. A 16-bit counter, `(counter-modulus 65536)` at `10 MHz`, has `65536 / 10 000 000 s = 6.5536 ms` and
fails the same requirement: the faster wrapping counter §5.2 warns of, caught because the direction belongs to the
horizon and the derivation to the vocabulary, not to anyone's sense of which counter is better. A reload counter,
`(counter-modulus 1000000) (tick-rate 10 MHz)`, has `1 000 000 / 10 000 000 s = 0.1 s` and fails it too (R2 B1). A
counter that writes `(unambiguous-horizon 3600 s)` beside its modulus, its rate or its `wrap-behavior`, bare or
valued, is `invalid-description`: the horizon is derived where its grounds are offered, and a horizon that long over
that hardware is an epoch extender's, a provider in its own right that offers the horizon alone (R4 D1; R5 E1; R6
F1).

### 5. Candidate enumeration

Over one description, the facts a requirement is checked against are its providers': presence reads them into one
map (`FactMap`), the closure says which are relevant, and the relation keeps what that map drops today — **which
provider offers which value**. An offer is one provider's statement (§1), so two providers offering one fact with
two values are two offers, each judged on its own, as two catalog records will be: the enumeration judges a
requirement against every provider in the closure that offers a value for its fact, derives one from its own facts
(§4), offers it bare — unknown, §3 rule 1's `missing-fact` — or declares it absent (§3 rule 1; R4 D2; R5 E3; a
condition fact declared absent imposes no demand, §3 rule 4, R6 F2), and lists each that satisfies it; where none
does, the diagnostic names each provider's value, bare offer or absence. One provider offering one declared fact
twice with two values, or bare beside a value, is `invalid-description` — the same value twice is one offer (R5 E10;
R6 F1) — as §5.3
says of a fact both offered and absent (R2 B5); an undeclared fact has no domain to contradict, so `region`,
offered once per named region, is untouched (R1 A6; §10). Choosing among the providers that satisfy is `M3.4`'s;
whether two may both be taken is `M3.3`'s. And the closure the enumeration judges by is presence's closure widened
by §3 rule 1: every fact a declaration inside it constrains is inside it too (R1 A2); the closure `archogen check`
reports stays presence's until `M3.4` wires the enumeration and decides whether the report widens with it.

For each declaration in the closure with a `requires` clause — a service, a policy, the system, a platform or a
block (R5 E2) — the enumeration judges every requirement the clause writes, a `needs` entry naming a vocabulary fact
as bare presence (§1; one written outside `requires`, at declaration level, is presence's, with the same verdict,
R7 G11) — one naming a declaration, a service, is presence's as today (R4 N2) — against every provider
the paragraph above names, and applies rule 5's condition check to each provider that satisfies them:
a block that demands `(reachable-at-privilege machine)` is checked against a service that only `needs` what it
offers, and is `missing-fact` on that service's side when the service states no privilege (R3 C2). It lists, per
constraint, each provider it was judged against and the verdict: satisfied, with the value and the derivation
used; or not, with the fact, the constraint, the value, the direction and the code of §8. It also emits, per
`requires` clause and provider, rule 5's verdict — the clause satisfied by that provider or not — and `M3.4`
chooses among the providers that satisfy the clause, never a constraint of it (R3 C8). A constraint whose fact no
provider in the closure offers with a value, derives or declares absent is `missing-fact`, by §3 rule 1 (R4 D2; R5
E3). When the catalog's records join
(`M3.4`), a record's contract is one more provider, judged like a block.

What the enumeration is not: it does not look outside the closure (§5.3), it does not resolve `uses` (presence
does), it does not choose, and it does not run inside `archogen check` in `M3.1` (§9).

### 6. The timer cases of §5.2

Each case is a fact of the vocabulary and a row of §2; the worked values are from the corpus or
`docs/semantics/boundary/accept/`.

| Case | Fact | Domain, role, direction | Satisfied, not satisfied |
| --- | --- | --- | --- |
| wrap interval | `unambiguous-horizon` | quantity time; guarantee; `at-least`; derived (§4) | `(counter-modulus 4294967296)` at `10 MHz` → `429.4967296 s ≥ 60 s`; `(counter-modulus 65536)` at `10 MHz` → `6.5536 ms`, not satisfied; `(unambiguous-horizon 3600 s)` written beside a modulus, a rate or a `wrap-behavior`, bare or valued → `invalid-description` (§4; R4 D1; R5 E1; R6 F1) |
| read atomicity | `observation-coherent` | boolean; guarantee; `exact` | offered `true` or bare → satisfied; `false`, or declared absent → not |
| programming range | `supported-horizon`, under the group `absolute-deadline` | quantity time; guarantee; `at-least` | an offer `(absolute-deadline true) (supported-horizon 3600 s)` satisfies `(at-least 10 s)`; `5 s` does not; a provider with `absent absolute-deadline` does not satisfy the group's head, which is infeasible where it is the only provider, as `timer.delay` is today (R1 A7, A13; R4 N3) |
| power state | `available-in-state` | set `run idle sleep`; guarantee; `includes` | offered `run idle` ⊇ required `idle`; offered `run` does not include it |
| access privilege | `reachable-at-privilege` | enumeration `(ordered user supervisor machine)`; **condition**; `at-most` | offer demands `supervisor` or `user`, requirer has `supervisor` → satisfied; offer demands `machine` → not: a stronger precondition, never a stronger capability |
| mediation | `or-through-mediation` | enumeration `allowed forbidden`; **statement**; `exact` | the requirer's own word that a mediation boundary may stand between it and the function (§4.3's "allowed mediation boundary"). No rule of `/1` reads it: a privilege test that fails is not satisfied, and a mediated path is an adapter — a provider in its own right, whose contract states the privilege it demands and which this relation judges like any other — that `M3.2`'s search may offer only where the requirer wrote `allowed` (R1 A3) |
| output units | `tick-unit` | enumeration `ns us ms`; guarantee; `exact` | `ns` satisfies `ns`; `us` does not, however a conversion might be arranged: rescaling is an adapter's, a provider in its own right (`M3.2`) |
| more bits | `counter-width` | quantity information; guarantee; `at-least` | `64 bit` satisfies `(counter-width 32 bit)`, a bound in the fact's direction, and an abstract platform's `(counter-width (at-least 32 bit))` is the same bound owed by a refinement; a service for which the width is the interface — one that reads a 32-bit word and relies on what it read — writes `(counter-width (exactly 32 bit))`, which `64 bit` does not satisfy (§1.1; R1 A4). What is longer with a wider counter is the horizon, which the counter's stated modulus and rate decide (§4), so a requirement on the horizon is written as one |

### 7. Not a lattice

The relation is a predicate on a pair, a requirement and an offer. Per fact, its direction induces a preorder on
the fact's values — `at-least` and `at-most` the order of the numbers or of the declared enumeration, `includes`
set inclusion, `within` interval inclusion, `exact` equality — and that is all the structure claimed: two offers of
one fact may be incomparable, two offers of two facts always are, and no join, meet or best offer is claimed or
used — some of these preorders have joins and meets (two sets under `includes` have a union and an intersection)
and the relation takes none of them (R1 A9). The engine never ranks providers by this relation; `M3.4`'s order
is configuration. Establishing that offers form a lattice would need a proof over the whole vocabulary that
nobody has made, so none is claimed, as §5.2 asks.

### 8. Diagnostics (§5.5)

| Code | When |
| --- | --- |
| `invalid-description` | a value outside its fact's domain; a direction written against the vocabulary's, `exactly` apart (§1.1); one provider offering one declared fact with two values, or bare beside a value; a derived fact offered beside a fact its rule reads (§4); a `counter-modulus` above a valued `2^width`, or beside `(wrap-behavior saturating)`; a condition fact written in a block's or platform's `requires`; an offer of a statement fact |
| `missing-fact` | a constrained fact with no value and no derivation at any provider — inside the closure by §3 rule 1, since its constraint is written there; a condition the requirer's side does not state |
| `infeasible-configuration` | a constraint that has a value at some provider and that no provider's value satisfies (R7 G4); a constrained guarantee fact, or a group's head, or an input it would be derived from, declared absent where no other provider satisfies the constraint (§3 rule 1) — a condition fact declared absent imposes no demand (§3 rule 4; R6 F2) |
| `unsupported-profile` | a constraint on a fact the vocabulary does not declare; exact arithmetic that overflows `i128` |

Every diagnostic carries the requirement's span and the offer's, the fact, the constraint as written, the value
found, the direction, the derivation used, the supported profile and a repair direction (§5.5); a missing or
absent fact names each provider the closure holds. One constraint judged against two providers can get two codes —
one declares the fact absent, another offers it bare; the diagnostic carries both, and the verdict is the higher
in `Verdict::precedence`, as the checker's is (`crates/eadl-front/src/diagnostic.rs`; R6 F8).

### 9. Where it lives, and what reads it

`crates/eadl-resolve` (`ROADMAP.md` §4.2's `eadl-resolve`): the vocabulary read into a typed table, the domains,
the relation, the rules of §4 and the enumeration of §5, over `eadl-model`'s `Quantity`, `ComparisonDirection` and
`FactMap`. A library in `M3.1`: `archogen check` does not call it, and every verdict the corpus has today stands.
Wiring it alone would turn every example that writes a constraint into `missing-fact`, or `infeasible-configuration`
where a provider declares the fact absent (R6 F8) — the four `s0-heartbeat` descriptions write none: a requirement whose fact no block or platform offers — `time.periodic-release`'s
`(release-accuracy (at-most 1 ms))`, every policy's `priorities`, `preemptive` and `deadlines`, `bounded-arrival`,
`queue-capacity` — is a constraint on the realization, which a catalog record's contract supplies
(`docs/specs/catalog/decision_catalog-records.md` §2), and no record exists until `M2.7.4.5` (R1 A2). The search (`M3.4`) wires it,
over the description's providers and the catalog's. The relation judges a fact by a provider's offer, never by a
declaration that bears the fact's name, which presence today counts as satisfying the closure: a block named
`low-power-timer` that offers nothing of that name passes presence and is `missing-fact` here; the library's
verdict is the stricter, and which one the report carries is decided when `M3.4` wires it (R2 B9). `quantity.rs`'s
comment on `ComparisonDirection::Exact` names a counter modulus as its example, and §1.1 agrees (R3 C9; R5 E4). `docs/semantics/model.md` gains a section normative over the crate when
the crate lands (`M3.1.2`), so that `reference.rs` holds its codes to the document as it holds every other
source's.

### 10. What stays open

- Which of several providers a service is bound to — which block's counter it reads — is topology and ownership,
  `M3.3`'s; `/1` judges each provider on its own (§5; R3 C3), so a `requires` clause whose constraints span two
  providers — an output and a counter — is satisfied by no single provider in `/1`, and the search (`M3.4`) is what
  may take two (R6 F9).
- `region` and `ordering` fit no domain of §2: a region is a name with sub-clauses, offered once per name, and an
  ordering is a pair of events; both are placement and ordering, `M3.3`'s, and both stay undeclared in `/1` —
  presence judges them, and a constraint on either is `unsupported-profile` when the enumeration is wired (R1 A10).
- Conditions are decided for one ordered enumeration, `reachable-at-privilege`; a condition over a quantity (a
  clock the block must be fed) takes the same rule 4 and a dimension, and is added to the vocabulary when a
  description needs it (R2 B2).
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
  `at-least` and its derivation divides by the rate, so a faster counter of the same modulus is worse. A relation
  without derivations would compare moduli and rates on their own and get the horizon wrong in either direction;
  one that let an author write the horizon beside its grounds let three review rounds find a way round each bound
  it set, which is why a derived fact is derived or stands alone.
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
| 2 | 10 | 7 (B1, the agreement rule forcing a reload counter to omit its modulus, whose width then derived a horizon it has not; B2, mediation still a boolean condition in §10 and rule 4; B3, `derived-from` declared one symbol wide; B4, `exactly` granted in §1.1 and refused by rule 2 and §8; B5, two providers' differing offers called a contradiction; B6, a group's head alone outside the domain as the corpus writes it; B7, a written count called `i128`) | "not acceptable as it stands"; every direction right, `exactly` a subset of every direction, the statement role and the closure rule sound, the numbers exact |
| 3 | 13 | 8 (C1, a derivation reached before a declared absence; C2, conditions unchecked against a requirer that only `needs`; C3, §10 still calling two providers' values a contradiction and §8 dropping *declared*; C4, `priorities` under `includes` reading `unique`, a demand, as a capability; C5, `wrap-behavior` offered bare between the rule's branches; C6, a modulus beside `saturating` unrefused; C7, `exactly` with no satisfaction condition for intervals; C8, rule 5's clause verdict with no home in §5) | "not acceptable as it stands"; every §5.2 case right in fact, domain and direction, the arithmetic exact and bounded |
| 4 | 10 | 3 (D1, an offered horizon beside derivable inputs escaping the impossibility rule that caught a modulus, so a 16-bit counter writing `3600 s` passed; D2, §5 leaving out a provider that declares the fact absent, `missing-fact` where rule 1 says infeasible; D3, a bare group head `true` in §2 and valueless in §1) | "not acceptable as it stands"; the four corpus descriptions walked by hand, every verdict right; six of eight restatements one thing everywhere |
| 5 | 10 | 3 (E1, the impossibility bound attached to a derivation's branch, so `(wrap-behavior saturating)` or a bare `wrap-behavior` switched it off and the 16-bit counter's `3600 s` passed again; E2, a platform's or block's `requires` constraint judged by nobody; E3, a bare non-boolean offer with three verdicts) | "not acceptable as it stands"; every §5.2 case right in fact, domain, role and direction; six adversarial offers refused by the sentence named, two not |
| 6 | 9 | 3 (F1, a bare bounding input switching the bound off, the third round in turn to find an author's horizon beside its grounds; F2, an absent condition fact satisfied by rule 4 and infeasible by §5 and §8; F3, a presence requirement against a bare offer satisfied by §2 and `missing-fact` by rule 1) | "not acceptable as it stands"; every adversarial offer against a valued sibling refused by a sentence named; the restatements (d), (e), (g), (i), (j) one thing everywhere |
| 7 | 12 | 5 (G1, the target's 64-bit `mtime` unable to write `2^64` as a count, so the one physical counter could state neither modulus nor horizon; G2, a boolean offered `false` meeting a `needs` for it; G3, an optional input declared absent derivable by §4's row and underivable by rule 1; G4, §8's infeasible row swallowing `missing-fact`; G5, `defpolicy` admitting no `offers`) | "not acceptable as it stands"; every §5.2 case right; no condition read as a capability; no undeclared implication used |
