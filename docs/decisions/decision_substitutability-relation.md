# The substitutability relation: when an offer satisfies a requirement, over a decidable fragment

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active` — written; under independent review (leaf `M3.1.1`'s closure rule: the first round that
  finds no defect closes it); rounds 1 to 15 answered `2026-10-03`
- **Owner / source:** leaf `M3.1.1` (`docs/tasks/M3.md`). `ROADMAP.md` §5.2 asks for "explicit matching rules in a
  decidable fragment" with "a documented comparison direction" per parameter, declared cross-field implications,
  and no stronger precondition "silently accepted as stronger capabilities"; §5.3 for a "versioned capability
  vocabulary"; §4.2 names the component `eadl-resolve`. The corpus the rules are written against is the one the
  repository tracks: 37 fact names across 124 descriptions, counted `2026-10-03` (§1).

## The fact / decision

The engine decides whether what one party **offers** satisfies what another **requires** by one relation, fact by
fact, over a small set of value kinds in which every comparison is decidable by exact arithmetic. Every fact the
relation judges is an entry of a **versioned vocabulary** (R12 L9), which fixes the fact's **domain**, its **role** —
a guarantee the offer makes, or a statement the requiring side makes about itself — and its **direction**: which
way an offered value must lie against a required one. A requirement is satisfied at one provider when the fact it
constrains has a value there, offered or derived by a rule the vocabulary names, that lies in the requirement's
written direction — the fact's, or equality under `exactly` (R12 L2) — or, for a presence requirement, when an offer of the fact other than `false`, or its derivation, meets it by §3 rule 3 (R13 M8). Nothing is inferred from a value being "more" unless the vocabulary says more is better for that fact,
and a stronger precondition is never a stronger capability, because what a provider accepts of its caller — the
privilege levels it is reachable from, the power states it works in — is offered as a set and judged by inclusion,
which reads no order. The relation is a predicate between one requirement and one provider (R12 L9). It induces a
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
what puts a fact inside the closure (model §2); `uses` names declarations, and a `uses` naming a vocabulary fact,
bare or as a list, is `invalid-description` — a fact is needed, never used — since presence would otherwise put it
in the closure and judge neither its value nor `false` (R15 1); no tracked description writes one. A constraint's head is a constraint on a
fact, not a claim that the fact is available, as `examples/alternative-timer/system.eadl` says of itself.

**Definitions.**

- A **fact** is a name of the vocabulary (§1.1). A **provider** is a block or a platform of the description. A
  catalog record's contract facet is not one in `/1`: it holds prose today, and a record is not type-checked
  (`docs/specs/catalog/decision_catalog-records.md` §1–§2), so giving it offers the relation can judge is `M3.7`'s
  (R13 M6); an adapter is offered only by `M3.2`'s search (§3 rule 7; R13 M2).
- An **offer** is a provider's statement about a fact: a value of the fact's domain, which `(f (exactly v))` also
  writes; bare presence, which for a boolean fact, or a group's head, is the value `true` and for any other domain is
  presence without a value (§2; R4 D3); or an abstract platform's bound in the fact's direction, which the
  refinement check reads (model §3) and the relation does not read as a value (§2; R10 J4, J8; R13 M12). A service may write `offers` too — `core.eadl` admits the clause on
  `defservice`, not on `defpolicy` (R7 G5) — but is not a provider, and its offers are not judged in `/1` (R3 C13); a service's `absent` is presence's (model §2),
not a provider's (R11 K14).
- A **requirement** is a constraint on a fact: a bound written with its direction, `(f (at-least v))`; a bare value,
  `(f v)`, which is a bound in the fact's own direction; a group, `(f (g …) (h …))`, which requires the boolean head
  `f` and each sub-constraint; or, through `needs`, bare presence. Inside `requires`, a bare fact name, or `(f)`
  with nothing after it, states no constraint and is `invalid-description` — presence is written `(needs f)`; and a
  list inside `needs` naming a vocabulary fact, `(needs (tick-unit us))`, is `invalid-description` too — a value
  belongs in `requires` — since presence would otherwise read only its head and drop the value. No tracked
  description writes either (census `2026-10-03`; R14 6, 7).
- A requirement's **side** is the declaration that writes it — a service, a policy, a system or a task, a platform
  or a block, whichever writes a `requires` clause or a `needs` (R5 E2; R10 J2).

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

The vocabulary is an eADL module the engine ships beside its kinds, in a folder of its own,
`docs/semantics/vocabulary/vocabulary.eadl` — not under `docs/semantics/kinds/`, every file of which
`crates/archogen-cli/tests/kind_modules.rs` holds to declaring a kind — answered by `archogen check` with exit 20 as
`crates/archogen-cli/tests/verdicts.txt` freezes the kind modules (R14 5), by the content rule below; and a root of
`docs/semantics/conformance.md`, so `eadl/1`'s baseline freezes it, with `deffact.eadl`'s addition covered by a
migration note (R15 8). It is versioned with the language, `eadl/1`; an entry changes only by §15's migration process. It is written with one
kind, `deffact`, declared with `defkind` in a kind module of its own, `docs/semantics/kinds/deffact.eadl`, which
`crates/eadl-resolve` registers to validate the vocabulary with `kind.rs`'s `validate` and which no description's
registry holds. `archogen check` recognises the language's own definitions by content, as `archogen-api`'s
`kind_module` recognises a `defkind`: a file whose declarations include a `deffact` is answered as the vocabulary is,
exit 20, the language's own definition and not a description — the vocabulary file, a copy of it, or a description
that writes a `deffact` alike — so no description's registry is ever asked for the kind (R11 K7; R15 7). So it is no more privileged than a kind a user adds (reference §7 rule 1) and carries no behaviour
(rule 3): its clauses hold names, lists of names and one line of text, and nothing that could be run.

```text
(deffact unambiguous-horizon
  (doc "the longest interval whose elapsed ticks two reads of the counter determine without wrap ambiguity: its modulus less one, over its rate")
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
| `role` | a symbol | `guarantee` or `statement` | a guarantee the offer promises, what it accepts of its caller included (§3 rule 4); or a statement the requiring side makes about itself, which constrains no provider and is read only by the rule, or the leaf, named here for it (§3 rule 6; R6 F4; R10 J1) |
| `direction` | a symbol | `at-least`, `at-most`, `exact`, `includes`, `within` | how an offered value satisfies a required one (§2's table) |
| `derived-from` | forms | facts | the facts a derivation computes this one from, when it is not offered directly (§4); the graph these clauses and `reads` make is acyclic (R10 J12), and the typed table `crates/eadl-resolve` reads the vocabulary into refuses an entry that would close a cycle (§9; R9 I12) |
| `reads` | forms | facts | the optional inputs the rule reads beside `derived-from`'s required ones (§4; R5 E9) |
| `rule` | a symbol | a rule name | the engine's rule that computes it, named here and listed in §4 |
| `doc` | a string | a string | what the fact means, one line; required, as a kind's `doc` is |

`deffact`'s clauses are `(holds forms)` for `domain`, since `(ordered …)` nests, and for `derived-from` and `reads`,
which list facts; `(holds values string)` for `doc`; and `(holds values symbol)` for `role`, `direction` and
`rule`, each exactly one value — reference §7 rule 4 makes `values` a fixed number, and `kind.rs` refuses any
other as `schema-arity` (R2 B3). `doc`, `domain`, `role` and `direction` appear exactly once, `derived-from`,
`reads` and `rule` at most once (R3 C11). A fact's name lives in the vocabulary, not in a
description's declaration namespace: the vocabulary is validated on its own, never checked as a description, so a
block named like a fact (`presence.rs`'s own fixtures write one) collides with nothing (R8 H7; R10 J12; R11 K7), and
reference §7 rule 6 — a name declared once — holds among `deffact`s (R1 A15).

A description that writes a fact the vocabulary does not declare may still offer it, declare it absent or need it
— presence judges those, as today — but a **constraint** on it is `unsupported-profile`: no rule decides a
parameter without a documented direction (§8). A requirement, or an abstract platform's bound, written in a
direction other than the fact's is `invalid-description`, naming both — with one direction either may always
write: **`exactly`** (R1 A4; R2 B4). In an offer, `(f (exactly v))` is, **for the relation**, simply the value `v`, judged as any value is
— the width–modulus check and the two-values rule included — so an offer cannot use it to escape either (R10 J8);
**for refinement** it stays the `exact` bound model §3 rule 2 checks on a platform another refines, the only bound
an abstract platform can write on a fact whose direction is `exact` (R12 L1). The relation reads offers with a
reader of its own in `crates/eadl-resolve`; `refinement.rs`'s `Facets::of` is not the relation's to change — what a refinement keeps of an abstract
platform's values is `M1.40`'s (§10) — so it goes on reading an
offered direction as a bound holding a quantity, refusing a non-quantity one as `quantity-not-a-number`, and a count
with no unit as `quantity-missing-unit`, before the relation runs, which only refuses — so §2's `exactly` offers of a
boolean, a set, an interval or a unitless count are the library's reading until `M3.4` wires it (R15 10), and keeping model §6's `fires on` inputs for `quantity-missing-unit` and
`quantity-missing` (R11 K8; R12 L1). Equality is the one bound every direction admits, so it never reads a value
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
| `counter-modulus` | count | guarantee | `exact` | interface: wrapping arithmetic depends on the modulus itself (§3.1's "explicit counter modulus"), as `quantity.rs`'s own example of `Exact` says; what wraps later is the horizon, which has its own entry; offered, never derived, and positive — a modulus of 0 is `invalid-description` (§4; R5 E4; R6 F1; R9 I13) |
| `counter-width` | quantity information | guarantee | `at-least` | capacity: a wider register holds every narrower modulus, and a modulus offered beside it may not exceed `2^width` (§4); `exactly` where the width is the interface (§6) |
| `deadlines` | enumeration `constrained arbitrary` | guarantee | `exact` | a policy's word |
| `debug-port`, `interrupt-source`, `low-power-timer`, `multicore`, `observable-output`, `observation-coherent`, `relative-delay`, `transfer-engine`, `uart` | boolean | guarantee | `exact` | |
| `delivery-bound` | quantity time | guarantee | `at-most` | a shorter delivery satisfies |
| `frequency` | interval frequency | guarantee | `within` | the rates the block can run at, a capability the offered range covers; the clock a block must be fed is another fact, added when a description needs it (§10; R4 N1) |
| `min-arrival-separation` | quantity time | guarantee | `at-least` | arrivals further apart satisfy a bound on separation |
| `or-through-mediation` | enumeration `allowed forbidden` | statement | `exact` | the requirer's word, read by `M3.2` (§3 rule 6) |
| `preemptive` | boolean | guarantee | `exact` | |
| `priorities` | set `static unique dynamic` | guarantee | `exact` | a policy's word, like `deadlines`: `unique` is a demand on the task set, not a capability, so a realization offering `static unique` does not satisfy a policy that wrote `static` alone; a policy that wants both writes both (R3 C4) |
| `queue-capacity` | count | guarantee | `at-least` | capacity |
| `reachable-at-privilege` | set `user supervisor machine` | guarantee | `includes` | the levels the function is reachable from, each listed: a requirer states the level it calls from, and the offered set must include it. A machine-mode register offers `machine` and does not satisfy a supervisor caller; a user mapping a supervisor cannot reach without `sstatus.SUM` offers `user` alone. Inclusion reads no order among levels, so a stronger precondition is never a stronger capability (§3 rule 4; R8 H9; R9 I1, I2; R10 J1, J6) |
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
two, `(pow2 N)` with `N` an integer from 0 to 126, anything else `invalid-description` — a literal's spelling, not
the arithmetic §10 refuses (R8 H2) — the modulus of a 64-bit timebase being `(pow2 64)`
(R7 G1) — and a written magnitude its digits with a scale (reference §1, `Rational::decimal`), refused above that as
it is read, while `i128` is the arithmetic's,
where a derivation's value lives (R2 B7; R4 N7); nothing is rounded, and a derivation, or the unit conversion inside a
comparison, whose computation overflows — `Rational::checked_div` multiplies before it reduces, so a quotient that
would fit can still overflow, which only ever refuses (R10 J9) — is `QuantityError::Overflow` (R5 E8), which the relation reports as `unsupported-profile`, a named limit of the arithmetic and
never a verdict on the requirement — `unsupported-profile` rather than `analysis-inconclusive` because the limit is
the implemented value domain's (model §1), not a search's budget: a description whose numbers pass it asks for a
wider domain, which §15 versions (R1 A5). The
fragment is §5.2's: enumerations, booleans, bounded integers, rational quantities with units, intervals, finite
sets, and the restricted arithmetic of §4's rules. No description writes arithmetic; a value that is not of the
fact's domain is `invalid-description`. In the table, `exact` is written `exactly`.

| Domain | An offer writes | A requirement writes | Direction | Satisfied when |
| --- | --- | --- | --- | --- |
| `boolean` | `(f true)`, `(f false)`, or bare `f` for `true`; `(f (exactly v))` is the value `v`, and any other bound is `invalid-description` (R9 I7; R10 J4) | `(f true)`, `(f false)` or `(f (exactly true))`; `f` inside `needs` is the requirement `(f true)`, the mirror of the bare offer (R7 G2, G12) | `exact` | the values are equal |
| `count` | `(f 8)` or `(f 8 tick)`, a non-negative integer with an optional dimensionless unit; or `(f (pow2 N))`, `N` an integer from 0 to 126, for a power of two however spelt (R7 G1; R8 H2) | `(f 8)`, `(f 8 tick)`, `(f (pow2 64))`, or a bound in the fact's direction or `exactly`, `(f (at-least 8 tick))` | `at-least`, `at-most`, `exact` | the integers compare in the direction |
| `quantity <dim>` | `(f 10 MHz)`, a number and a unit of the dimension | `(f (at-least 60 s))` in the fact's direction, `(f (exactly 60 s))`, or `(f 60 s)` in the fact's direction | `at-least`, `at-most`, `exact` | `ComparisonDirection::satisfied_by` in base units; another dimension is `invalid-description` |
| `interval <dim>` | `(f (range lo hi))`, two quantities of the dimension, `lo ≤ hi`; or a point `(f v)`, the interval `[v, v]` (R3 C10) | a point `(f v)` or an interval `(f (range lo hi))`, `lo ≤ hi` here too (R12 L12), or `(f (exactly (range lo hi)))` (R6 F6) | `within`, `exact` | the required point or interval lies inside the offered one: `o.lo ≤ r.lo` and `r.hi ≤ o.hi`; under `exactly`, the intervals are equal (R3 C7) |
| `enumeration a b c` | `(f b)`, one of the alternatives | `(f b)` or `(f (exactly b))`; with `(ordered …)`, also `(f (at-least b))` or `(f (at-most b))` in the fact's direction | `exact`; with an order, `at-least` or `at-most` | equal; or in the direction over the declared order |
| `set a b c` | `(f a b)`, alternatives without repetition, at least one — `(f)` with none is a bare offer (R12 L12) | `(f b)`, `(f a b)`, or `(f (exactly a b))` | `includes`, `exact` | the offered set contains every required alternative; or the sets are equal |
| `group g h …` | `(f true)` or bare `f`, and the sub-facts offered on their own; the head takes a bound as a boolean does (R10 J4) | `(f true)`, the head alone, which the head named in `needs` also means (R9 I6); or `(f (g …) (h …))`, the head and sub-constraints; a sub-fact is a fact in its own right and may be constrained flat beside the head, as `examples/alternative-timer/system.eadl` writes `(absolute-deadline true)` beside `(supported-horizon (at-least 10 s))` (R2 B6) | the head is `boolean exact`; each sub-fact its own | the head holds and every sub-constraint, nested or flat, is satisfied, each judged as a requirement on its sub-fact |

A bound an offer writes (an abstract platform's `(counter-width (at-least 32 bit))`) is not a value — `exactly`
apart, which is (§1.1) — it satisfies no valued constraint of a requirement — presence it does satisfy (§3 rule 3;
R8 H5) — and is the refinement check's business (model §3); a provider that writes a bound beside a value of the
same fact is `invalid-description` (R10 J8). A fact offered bare in a domain
other than `boolean` or a group's head, with no value of it beside (§5; R13 M7), is **unknown** at that provider (R6 F5): it satisfies presence, a constraint on it
is unknown there whatever a rule could have derived — the bare offer stops §3 rule 1's chain before derivation —
and a rule that reads it derives nothing (§4; R3 C5; R5 E3, E7).

### 3. The relation

For one requirement `R` on fact `f`, written by side `S`, and one provider `P`:

1. **The outcome at `P`.** An offer §8 refuses — a provider offering and declaring absent one fact, two values, a
   derived fact beside a fact its rule reads, and the rest of §8's first row — is `invalid-description`, judged
   before anything else (R10 J5; R12 L11). Otherwise `f` at `P` is, in this order:
   - **valued** — the value `P` offers for `f`;
   - **absent** — `P` declares `f` absent, or, where `P` does not offer `f`, declares absent a required input of its
     derivation (R12 L3): a declared absence
     is something the offer says, and no derivation corrects it (§4; R3 C1; R7 G3; R10 J7);
   - **unknown** — `P` offers `f`, or an input its rule reads, required or optional, without a value (§2, §4; R3 C5;
     R5 E3; R12 L15);
   - **derived** — the value §4's rule computes from the facts that have a value at `P`, offered or themselves
     derived (R1 A1); an optional input declared absent makes the rule derive nothing, and `f` is then
     **undescribed** (R7 G3);
   - **undescribed** — none of these.

   A valued or derived `f` satisfies `R` or is refused by it (rule 3). A derivation or comparison whose arithmetic
   overflows gives no outcome at `P`: it is §2's `unsupported-profile` (R13 M10). These are the relation's outcomes at
   one provider. What they make of a description when several providers are judged together — which providers and which
   requirements a description presents, a declared absence at one provider beside an offer at another, and whether
   the description's verdict is `missing-fact` or `infeasible-configuration` — is `M3.4`'s, and its acceptance carries
   what this record's reviews found of each (§5; R11 K1, K2, K5, K6).
2. **The domain.** The value is of `f`'s domain, or the offer is `invalid-description`; `R`'s constraint is of `f`'s
   domain and written in `f`'s direction or as `exactly` (§1.1), or `R` is `invalid-description` (R2 B4).
3. **A presence requirement** — `f` named in `needs`, wherever the `needs` is written, inside `requires` or at a
   declaration's own level (R10 J2) — is satisfied by any offer of `f` — bare, valued, or an abstract platform's
   bound — other than a boolean or a group's head whose value is `false`, `(f (exactly false))` included (R10 J4),
   or by `f`'s derivation where the vocabulary names one (§4; R7 G10), and by nothing
   else: a boolean `f`, or a group's head, named in `needs` is the requirement `(f true)` (R9 I6), so `(f false)` satisfies no presence
   requirement, as `absent` does not (R7 G2); its outcome at `P` is otherwise rule 1's (R6 F3; R8 H6). **A guarantee** is
   satisfied when the value lies in `R`'s written direction against `R`'s bound — the fact's direction, or equality
   under `exactly` — by §2's table (R12 L2). `exact`
   admits no "better" value: a tick unit of `us` does not satisfy `ns`, and `20 MHz` does not satisfy a tick rate
   that must be `10 MHz`.
4. **No conditions in `/1`.** What a provider accepts of whoever uses it is offered as a guarantee:
   `reachable-at-privilege` is the set of levels the function is reachable from, and a requirer's level must be in it
   (§1.1), as `available-in-state` is the set of states it works in. Inclusion reads no order among levels, so a
   function reachable only from `machine` does not satisfy a `supervisor` requirer, and a stronger precondition is
   never a stronger capability; a provider that states no level — `reachable-at-privilege` undescribed, declared
   absent, or offered bare — is, by rule 1, undescribed, absent or unknown for a requirer that states one, and
   satisfies it in none of the three (R13 M3). A requirer that states no level asks nothing of the relation: which context a plan binds it to, and
   whether the provider's set includes it, is the joint constraint §5.4 lists as privilege, `M3.3`'s (§10); a
   provider that states no level, in any of those three ways, is refused there for any caller a plan binds to it, and
   `M3.3`'s acceptance says so (R11 K4; R12 L10; R13 M3). So every provider a plan binds a caller to must state its
   level; no tracked description does yet, and the migration — the target's level from its agreement and a ledger
   source, as §4 adds the modulus — is `M3.3`'s, in its acceptance (R14 2). A
   reversed condition role, read with the demand on the offered side, drew defects in six of ten review rounds and
   is deleted (R10 J1, J6).
5. **A `requires` clause** is satisfied by `P` when each of its constraints is, a statement apart (rule 6); a
   group's sub-constraints count among its constraints (R12 L13).
6. **A statement** — a fact whose role is `statement`, such as `or-through-mediation` — is the requiring side's
   word about itself. It is checked for its domain and nothing else: it constrains no provider, puts no fact
   inside the closure, and is read by the rule or the leaf its entry names, which for `/1`'s one statement is
   `M3.2`'s adapter search (§6, "mediation"; R1 A3), whose acceptance carries it. It is the requiring side's word
   alone: an offer of a statement fact, and a `needs` of one, is `invalid-description` (R2 B10; R12 L6, L13).
7. **Not decided here.** Which of several satisfying providers to take (`M3.4`: a deterministic preferred order in
   engine configuration); whether two providers may both be taken (`M3.3`: capacity, ownership, topology); and
   whether a requirement no provider satisfies can be met by an adapter (`M3.2`: an adapter is a provider whose
   contract this relation judges like any other, with its costs and obligations its own, offered only by `M3.2`'s
   search, which holds rule 6's mediation gate; `M3.4`'s enumeration lists the description's blocks and platforms,
   never an adapter, and a catalog record only after `M3.7`; R13 M2).

### 4. Derived facts and declared implications

A fact the vocabulary marks `derived-from` is **derived, or stands alone**. At a provider that offers any fact its
rule reads — required or optional, bare or valued — the derived fact is derived and never offered: an offer of it
there is `invalid-description`, since whoever offers the inputs has offered the result's grounds, and the engine
computes it (R4 D1; R5 E1; R6 F1). At a provider that offers none of those facts, the derived fact may be offered,
and the offered value is the value — a claim, as every offer is (§5.3) — such as the horizon an epoch-extending
adapter guarantees once `M3.2`'s search offers it, a provider in its own right (R9 I3; R13 M2). So the derived fact has a value at `P` when `P` offers it alone,
or when every fact it is derived from has a value at `P`, offered or itself derived, and the named rule computes it;
`derived-from` is acyclic (§1.1), so the derivation terminates (R1 A1). Any input a rule reads, required or
optional, that is offered without a value is unknown, and the rule derives nothing (R3 C5); an optional input
declared absent makes the rule derive nothing too, the conservative branch, while a required one declared absent is
rule 1's (R7 G3). A derived fact offered beside facts the rule does not read stands as a claim: a block offering
`(counter-width 16 bit)` and `(unambiguous-horizon 3600 s)` and no rate or modulus is believed, since a width does
not decide a horizon — a 16-bit modular counter at 18 Hz holds an hour, `65535 / 18 s ≈ 3640.8 s` (R7 G6; R14 9). Every derivation is per
provider: a horizon is computed from one block's modulus and rate, never one block's modulus and another's rate (R2
B5). The rules are engine knowledge — exact rational arithmetic in `crates/eadl-resolve`, each named, and each
listed here. No implication between facts holds unless a rule names it; `/1` has one.

A counter's modulus is offered, never derived from its width. §3.1 asks for an "explicit counter modulus"; a
register's width does not decide where it wraps — a 32-bit register reloaded at a million ticks has a modulus of a
million (R2 B1) — and a rule that guessed `2^width` was where three review rounds in turn found an author's value and
the guess disagreeing, each answer adding a bound and the next round a way round it (R4 D1; R5 E1; R6 F1). What
the width still does is bound the modulus: a `counter-modulus` and a `counter-width` both offered with values must
agree, a modulus above `2^width` being `invalid-description`, as a 32-bit register holds no modulus of `2^33` (R3
C12); a width whose value in bits is not a positive whole number is `invalid-description` (R8 H3; R12 L12), and a width of 127 bits or
more holds every writable modulus, `(pow2 126)`
the largest, so the check is made only below that (R7 G8). The rule is a modular counter's, one that wraps at its modulus — which is what `counter-modulus` means: a
modulus offered beside a `wrap-behavior` that is `saturating` is `invalid-description`, and a saturating counter's
range is another fact, added when a description needs it (R1 A14; R3 C6). The `modular` reading of an omitted
`wrap-behavior` serves the rule alone: a constraint on `wrap-behavior` itself is judged on what is offered, so
against an unstated offer it is undescribed (R4 N4). `derived-from` lists a rule's required inputs; a rule may
also read a named optional one, which the entry's `reads` clause names — `wrap-behavior` here (R5 E9).

| Rule | Computes | From | Formula |
| --- | --- | --- | --- |
| `horizon-from-modulus-and-rate` | `unambiguous-horizon` (quantity, time) | `counter-modulus` (count); `tick-rate` (quantity, frequency, positive by model §1 rule 3); `wrap-behavior`, optional | `(modulus − 1) / rate` seconds, exact, when `wrap-behavior` is `modular` or undescribed; nothing when it is offered without a value or declared absent (R7 G3) — `saturating` beside a modulus is `invalid-description` (above), so the rule never reads it (R9 I5); a quotient, or the unit conversion inside a comparison, past `i128` is `QuantityError::Overflow`, `unsupported-profile` (§2; R5 E8) |

**Why one less than the modulus** (R14 9; R15 9). Between two reads a time `Δ` apart the counter advances `D`
ticks, `⌊Δ·rate⌋` or `⌈Δ·rate⌉` as the phase gives, and the two reads, taken modulo the modulus, determine `D` only
while `D ≤ modulus − 1`: an advance of `D` and of `D + modulus` give the same pair of reads. So `D` is recovered at
every phase exactly when `⌈Δ·rate⌉ ≤ modulus − 1`, which is `Δ ≤ (modulus − 1) / rate` — at that endpoint `Δ·rate` is
the whole number `modulus − 1` at every phase; an interval of `modulus / rate`, or just under it, can advance a whole
modulus and leave the reads equal. So the horizon is inclusive, as `ROADMAP.md` §4.3's "unambiguous across
at least 60 seconds" reads, and `at-least` compares it as any capacity: `(counter-modulus 600000000)` at `10 MHz` has
`59.9999999 s` and does not meet `60 s`, where `modulus / rate` would have given exactly `60 s` and met it.

**Worked, from the corpus.** `timer.counter` in `examples/periodic-three/system.eadl` offers `counter-width 32 bit`
and `tick-rate 10 MHz` and no modulus, so in the library its horizon is undescribed at that provider:
the profile asks for the modulus (§3.1). A counter that states its modulus, `(counter-modulus 4294967296)`, as
`docs/semantics/boundary/accept/counter-width-and-rate.eadl` already does, has a horizon; whether each tracked counter
gains one is `M3.4`'s (§9), `uc3`'s `M3.2`'s under the director (§10), and a counter that offers `tick-rate` bare
derives nothing however its modulus is stated (R15 3). For `targets/riscv-virt-up.eadl`'s `target.timer`, the 64-bit `mtime`, `(counter-modulus (pow2 64))` (§2; R7 G1) — that
file carrying nothing its agreement does not check, `cargo xtask target-agreement` and
`decision_target-platform-description.md` change in the same commit (R8 H8); the device tree carries no modulus, so
what the agreement checks it against — [the privileged specification](../book/src/ledger.md#riscv-privileged)'s
`mtime` width — the commit that adds the modulus adds to the `riscv-privileged` ledger entry, which quotes neither
yet, "The `mtime` register has a 64-bit precision on all RV32 and RV64 systems" and that it "will wrap around if the
count overflows" (R10 J13; R11 K15) — is decided there, under that decision's
§2 (R9 I9) — a
description gaining a fact §3.1 names, not a requirement weakened. The target's horizon is then
`(2^64 − 1) / 10 000 000 s = 1 844 674 407 370.9551615 s`, and fits the arithmetic. With it, the horizon is
`4294967295 / 10 000 000 s = 429.4967295 s`, and `time.monotonic`'s `(unambiguous-horizon (at-least 60 s))` is
satisfied; `timer.delay` in `examples/alternative-timer/system.eadl`, at `1 MHz` and with that modulus stated, would
have `4294.967295 s`, slower and better here. A 16-bit counter, `(counter-modulus 65536)` at `10 MHz`, has
`65535 / 10 000 000 s = 6.5535 ms` and
fails the same requirement: the faster wrapping counter §5.2 warns of, caught because the direction belongs to the
horizon and the derivation to the vocabulary, not to anyone's sense of which counter is better. A reload counter,
`(counter-modulus 1000000) (tick-rate 10 MHz)`, has `999 999 / 10 000 000 s = 0.0999999 s` and fails it too (R2 B1). A
counter that writes `(unambiguous-horizon 3600 s)` beside its modulus, its rate or its `wrap-behavior`, bare or
valued, is `invalid-description`: the horizon is derived where its grounds are offered, and a horizon that long over
that hardware is an epoch extender's, a provider in its own right that offers the horizon alone (R4 D1; R5 E1; R6
F1).

### 5. Candidate enumeration

For one requirement and the providers it is judged against, the enumeration lists each provider with its outcome
under §3 rule 1 — valued or derived and satisfied or refused, absent, unknown, undescribed — with the value, the
derivation used and the direction; and, per `requires` clause and provider, rule 5's verdict, the clause satisfied by
that provider or not (R3 C8). An offer is one provider's statement (§1), so two providers offering one fact with two
values are two offers, each judged on its own, as two catalog records will be (R2 B5). One provider offering one
declared fact twice with two values, or a bound beside a value, is `invalid-description`; the same value twice — the same in the fact's domain, `10 MHz` beside `10000 kHz` included (R15 13) — is one
offer, and a bare offer beside a value of the same fact is that value, in a domain where bare presence has no value;
for a boolean or a group's head, where bare is `true`, a bare offer beside `(f true)` is the same value twice and
beside `(f false)` two values, `invalid-description` (R5 E10; R6 F1; R10 J8; R11 K9; R13 M7; R14 8). An undeclared fact
has no domain to contradict, so `region`, offered once per named region, is untouched (R1 A6; §10). When `M3.7` gives a
catalog record's contract offers the relation can judge, a record is one more provider, judged like a block, and a
record that mediates enters only through `M3.2`'s search (R13 M2, M6).

**What the enumeration is handed, and what its list makes of a description, are `M3.4`'s** — the search that wires
the relation into `archogen check` — and its acceptance carries each of these as this record's reviews found them:

- **the closure:** which requirements and which providers a description presents, rooted at the system with its
  tasks and platform, `uses` and `needs` followed transitively, every requirement judged — each constraint of a
  `requires`, and each `needs` of a vocabulary fact wherever written — and which closure the report carries where it
  and presence's differ (R2 B9; R10 J2, J3; R11 K1);
- **absence across providers:** a fact declared absent at one of the description's blocks and platforms and offered
  at another, outside a direct refinement pair, is `invalid-description` (model §2 rule 1), compared over every offer
  and absence, not presence's first of each, and through no chain of refinements (R8 H1; R11 K2); an adapter
  `M3.2`'s search offers is not one of them, and its offer satisfies past a declared absence (R13 M4);
- **one clause, several providers:** which constraints of one clause may be met by different providers — a group and
  its sub-constraints, nested or flat, never (R12 L16) — and the requirements one service makes of one device's facts
  met by the one provider a plan binds it to, which is `M3.3`'s and in its acceptance (R6 F9; R11 K3; R12 L5);
- **the description's code:** a provider takes part in `f`'s code when it states `f` — offered in any form, or
  declared absent — or an input of `f`'s derivation; one that states neither is silent and decides nothing (R13 M5).
  `missing-fact` while some provider that takes part is unknown or undescribed and none satisfies, so an author is
  sent to the fact that could still decide it, and when none takes part (R12 L4); `infeasible-configuration` once at
  least one takes part and every one that does is refused or absent; and model §2 rule 2 — "a required fact that is declared absent makes the configuration infeasible" — amended in
  `docs/semantics/model.md`, where a provider's derivation, or an adapter's offer `M3.2`'s search makes, satisfies
  past another's absence (R6 F8; R10 J10; R11 K5, K6; R13 M4).

What the enumeration is not: it does not choose, does not resolve `uses`, and does not run inside `archogen check` in
`M3.1` (§9).

### 6. The timer cases of §5.2

Each case is a fact of the vocabulary and a row of §2; the worked values are from the corpus or
`docs/semantics/boundary/accept/`.

| Case | Fact | Domain, role, direction | Satisfied, not satisfied |
| --- | --- | --- | --- |
| wrap interval | `unambiguous-horizon` | quantity time; guarantee; `at-least`; derived (§4) | `(counter-modulus 4294967296)` at `10 MHz` → `429.4967295 s ≥ 60 s`; `(counter-modulus 65536)` at `10 MHz` → `6.5535 ms`, not satisfied; `(unambiguous-horizon 3600 s)` written beside a modulus, a rate or a `wrap-behavior`, bare or valued → `invalid-description` (§4; R4 D1; R5 E1; R6 F1) |
| read atomicity | `observation-coherent` | boolean; guarantee; `exact` | offered `true` or bare → satisfied; `false`, or declared absent → not |
| programming range | `supported-horizon`, under the group `absolute-deadline` | quantity time; guarantee; `at-least` | an offer `(absolute-deadline true) (supported-horizon 3600 s)` satisfies `(at-least 10 s)`; `5 s` does not; a provider with `absent absolute-deadline` is absent for the group's head, as `timer.delay` is today (R1 A7, A13; R4 N3) |
| power state | `available-in-state` | set `run idle sleep`; guarantee; `includes` | offered `run idle` ⊇ required `idle`; offered `run` does not include it |
| access privilege | `reachable-at-privilege` | set `user supervisor machine`; guarantee; `includes` | offered `supervisor machine`, requirer `supervisor` → satisfied; offered `machine` alone → not: inclusion reads no order, so a stronger precondition is never a stronger capability; a user mapping a supervisor cannot reach offers `user` and does not satisfy `supervisor`; a provider stating nothing is undescribed, one declaring the fact absent absent; a requirer stating no level, and a provider stating none against the context a plan binds, are `M3.3`'s (§3 rules 1, 4; R10 J1, J6; R11 K4) |
| mediation | `or-through-mediation` | enumeration `allowed forbidden`; **statement**; `exact` | the requirer's own word that a mediation boundary may stand between it and the function (§4.3's "allowed mediation boundary"). No rule of `/1` reads it: a privilege test that fails is not satisfied, and a mediated path is an adapter — a provider in its own right, whose contract states the levels it is reachable from and which this relation judges like any other — that `M3.2`'s search may offer only where the requirer wrote `allowed` (R1 A3) |
| output units | `tick-unit` | enumeration `ns us ms`; guarantee; `exact` | `ns` satisfies `ns`; `us` does not, however a conversion might be arranged: rescaling is an adapter's, a provider in its own right (`M3.2`) |
| more bits | `counter-width` | quantity information; guarantee; `at-least` | `64 bit` satisfies `(counter-width 32 bit)`, a bound in the fact's direction, and an abstract platform's `(counter-width (at-least 32 bit))` is the same bound owed by a refinement; a service for which the width is the interface — one that reads a 32-bit word and relies on what it read — writes `(counter-width (exactly 32 bit))`, which `64 bit` does not satisfy (§1.1; R1 A4). What is longer with a wider counter is the horizon, which the counter's stated modulus and rate decide (§4), so a requirement on the horizon is written as one |

### 7. Not a lattice

The relation is a predicate on a pair, a requirement and a provider. Per fact, its direction induces a preorder on
the fact's values — `at-least` and `at-most` the order of the numbers or of the declared enumeration, `includes`
set inclusion, `within` interval inclusion, `exact` equality — and that is all the structure claimed: two offers of
one fact may be incomparable, two offers of two facts always are, and no join, meet or best offer is claimed or
used — some of these preorders have joins and meets (two sets under `includes` have a union and an intersection)
and the relation takes none of them (R1 A9). The engine never ranks providers by this relation; `M3.4`'s order
is configuration. Establishing that offers form a lattice would need a proof over the whole vocabulary that
nobody has made, so none is claimed, as §5.2 asks.

### 8. Diagnostics (§5.5)

The relation's own codes are about an offer or a requirement as written, whatever else the description holds:

| Code | When |
| --- | --- |
| `invalid-description` | a bare fact name, or `(f)`, inside `requires`; a list inside `needs` naming a vocabulary fact; a `uses` naming a vocabulary fact; a wrapper other than `at-least`, `at-most` and `exactly` — `(f (includes …))`, `(f (within …))` — since a requirement writes its value and the direction is the fact's (R15 16); a value outside its fact's domain; a width that is not a positive whole number of bits; a set written with no member in a requirement, or as `(f (exactly))` in an offer — `(f)` in an offer is bare (§2;
R13 M11) — or an interval with `lo > hi` on either side; a `needs` of a statement fact; a direction written against the vocabulary's, `exactly` apart (§1.1); a provider offering and declaring absent one fact; one provider offering one declared fact with two values, or a bound beside a value; a boolean or group head offered with a bound other than `exactly`; a derived fact offered beside a fact its rule reads (§4); a `counter-modulus` of 0, above a valued `2^width`, or beside `(wrap-behavior saturating)`; an offer of a statement fact |
| `unsupported-profile` | a constraint on a fact the vocabulary does not declare; exact arithmetic that overflows `i128` |

Whether each cause of a row takes a code of its own, as reference §4 rule 3 asks of a new rule, is `M3.1.2`'s to decide
when it writes these rows into `docs/semantics/model.md` §6 (R12 L14).

A description's `missing-fact` and `infeasible-configuration` come from the enumeration's list, and are `M3.4`'s (§5).
Every diagnostic carries the requirement's span and the offer's, the fact, the constraint as written, the value
found, the direction, the derivation used, the supported profile and a repair direction (§5.5).

### 9. Where it lives, and what reads it

`crates/eadl-resolve` (`ROADMAP.md` §4.2's `eadl-resolve`): the vocabulary read into a typed table, the domains,
the relation, the rules of §4 and the enumeration of §5, over `eadl-model`'s `Quantity`, `ComparisonDirection` and
`FactMap`. A library in `M3.1`: `archogen check` does not call it, and every verdict the corpus has today stands.
Wiring it alone would leave every valued constraint the examples write undescribed, unknown or absent at every
provider but one — `periodic-three`'s `(observation-coherent true)`, which `timer.counter` offers (R12 L8);
`bounded-queue`'s horizon is unknown at its counter, which offers `tick-rate` bare (R13 M9) — and the four
`s0-heartbeat` descriptions write no valued constraint: a requirement whose fact no block or platform offers — `time.periodic-release`'s
`(release-accuracy (at-most 1 ms))`, every policy's `priorities`, `preemptive` and `deadlines`, `bounded-arrival`,
`queue-capacity` — is a constraint on the realization, which a catalog record's contract will supply
once `M3.7` gives it offers the relation judges, and no record exists until `M2.7.4.5` (R1 A2; R13 M6). The search
(`M3.4`) wires it, over the description's providers, and the catalog's after `M3.7`. Beyond the examples, wiring it
changes other tracked verdicts too — a horizon unknown where a counter offers `tick-rate` bare, a requirement no
block or platform offers, a description with no platform (R14 3) — and keeping or migrating each, named in the
commit that changes it, is `M3.4`'s, in its acceptance; this record claims no verdict for a tracked description once
wired. The relation judges a fact by a provider's offer, never by a
declaration that bears the fact's name, which presence today counts as satisfying the closure: a block named
`low-power-timer` that offers nothing of that name passes presence and is undescribed here; the library's
verdict is the stricter, and which one the report carries is decided when `M3.4` wires it (R2 B9). `quantity.rs`'s
comment on `ComparisonDirection::Exact` names a counter modulus as its example, and §1.1 agrees (R3 C9; R5 E4). `docs/semantics/model.md` gains a section normative over the crate when
the crate lands (`M3.1.2`), so that `reference.rs` holds its codes to the document as it holds every other
source's.

### 10. What stays open

- Which of several providers a service is bound to — which block's counter it reads — is topology and ownership,
  `M3.3`'s, in its acceptance; the relation judges each provider on its own (§5; R3 C3), and which constraints of one
  clause may be met by different providers is `M3.4`'s, a group and its sub-constraints never (§5; R6 F9; R11 K3; R12
  L5).
- What a refinement keeps of an abstract platform is model §3's, `M1.40`'s: a concrete `(f false)` keeping an
  abstract bare boolean guarantee and two abstract bounds on one fact, the last winning (R12 L17), and every value an
  abstract platform offers, none of which was kept, so a system written against an abstract platform was judged on
  values its refinement may not have — `(tick-unit ns)` refined by `us` was accepted (R13 M1). The relation judges
  the providers the closure reaches; refinement is what makes an abstract platform's values hold of what refines it.
  `M1.40` closed `2026-10-03`: each case is refused, a value kept only by the same value as written or, for a
  quantity, the same amount.
- `region` and `ordering` fit no domain of §2: a region is a name with sub-clauses, offered once per name, and an
  ordering is a pair of events; both are placement and ordering, `M3.3`'s, and both stay undeclared in `/1` —
  presence judges them, and a constraint on either is `unsupported-profile` when the enumeration is wired (R1 A10).
  `required-ordering-guarantee.eadl` writes one: whether `M3.3`, which judges a required ordering and a region's
  placement, gives them entries and domains under §15 first, or the case is migrated, is decided there and in
  `M3.4`'s keep-or-migrate rule (§9), not here (R9 I11; R14 4; R15 2).
- `uc3`'s seal — refused before `M3.2` and built after it with its description unchanged (`examples/README.md`,
  `docs/usecases/uc3-alternative-timer.md`) — meets this record's rules: `timer.delay` states no modulus, so its
  horizon is undescribed (§4); no privilege level (§3 rule 4, `M3.3`); and its policy's constraints wait on a catalog
  record (`M3.7`). A provider is judged on its own (§4, §5), so no record's fact is joined to `timer.delay`'s: the
  horizon and the level come from one provider — `timer.delay` itself, an adapter `M3.2`'s search offers, or a record
  after `M3.7` offering every fact `time.monotonic` is judged on. Which, or whether the seal moves, is `M3.2`'s, and a
  change to the description is brought to the director before it is made (R14 1; R15 4).
- `/1` has no condition role (§3 rule 4). Which context a plan binds a caller to, and whether the provider's
  `reachable-at-privilege` includes it, is the joint constraint §5.4 calls privilege, `M3.3`'s; a clock a block must
  be fed is another fact, added when a description needs it (R2 B2; R10 J1, J6).
- Arithmetic in a description stays refused — `(pow2 N)` is a literal's spelling (§2), not arithmetic (R8 H2); a new
  derivation is a new named rule here and in the engine.
- A catalog record's contract facet as a provider is `M3.7`'s, after `M2.7.4.5` and `M3.1.2`, a mediating record
  entering only through `M3.2`'s search; the choice among providers is `M3.4`'s (R13 M2, M6). Two questions go with
  it, each answered by those leaves' own reviewed designs and not here: how a record's offer or absence meets a block's
  absence or offer — model §2 rule 2's amendment names the answer (R15 5) — and how a record's preconditions become
  offered sets judged by inclusion, so a stronger precondition is never a stronger capability (R15 6).
- A derived fact as an abstract platform's bound or value (R15 12) cannot yet be refined by a concrete platform that
  states its grounds: model §3 rule 2 needs a value in the concrete platform, and §4 forbids the derived fact there; teaching
  refinement a derivation is a later leaf's, filed when a description needs it (R9 I8).
- A saturating counter states no horizon in `/1` — no modulus, and not the horizon beside its rate; its range is
  another fact (§4; R9 I10).

## Why

- §5.2 asks for explicit rules in a decidable fragment and a documented direction per parameter. The corpus
  already writes directions in requirements; nothing checked them, and `refinement.rs` compared bounds only
  between an abstract platform and its refinement. The vocabulary makes the direction a property of the fact,
  checked wherever the fact is written.
- §5.2's "stronger preconditions are not silently accepted as stronger capabilities" is kept by reading no order
  where a precondition lives: what a provider accepts of its caller is a set, and inclusion cannot read `machine` as
  more than `supervisor`. A reversed condition role, which ten rounds tried, drew defects in six; the set needs no
  reversal (§3 rule 4).
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
  (the contract facet as a provider, `M3.7`'s), [[decision_priority-comparison-direction]] (one direction already ruled).

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
| 8 | 11 | 1 (H1, the absence rule, §5 and §8's example judging per provider a fact offered at one and absent at another, which model §2 rule 1 refuses description-wide before any relation runs) | "not acceptable as it stands"; no probe made the relation read a demand as a capability, a value the wrong way, or an undeclared implication; every §5.2 case classified as the fact's meaning requires |
| 9 | 13 | 2 (I1, the round-8 `exactly` escape for a non-nesting privilege the very form rule 4 reads as an unknown demand; I2, a condition fact declared absent read as no demand, the one permissive absence) | "not acceptable as it stands"; every other §5.2 case classified as the fact's meaning requires; no undeclared implication; the arithmetic exact and bounded |
| 10 | 13 | 8 (J1, a provider stating no privilege accepted where one stating `user` was refused; J2, a declaration-level `needs` escaping rule 3 and the condition check; J3, the system outside the closure, its own constraints judged by nobody; J4, `(exactly false)` meeting `needs`; J5, a provider offering and declaring absent one fact read by its value; J6, a block requirer refused with no legal repair; J7, a required input declared absent given two codes; J8, an `exactly` width escaping the width–modulus check) | "not acceptable as it stands"; probes run against the built checker; every other §5.2 case right, the arithmetic exact |
| 11 | 16 | 7 (K1, the closure stated two ways, one admitting a provider on an unused platform; K2, a declared absence passed by another provider's offer, presence comparing only the first of each; K3, one clause's head and range from two providers; K4, a provider stating no privilege untaken by `M3.3`; K5, model §2 rule 2 cited for its opposite; K6, infeasible while an unknown provider decides; K7, no mechanism refusing a stray `deffact`) | "not acceptable as it stands"; for one provider every §5.2 case right, no stronger precondition passing; the derivation seam closed |
| 12 | 17 | 8 (L1, an offered `exactly` read as the value would strip refinement's only bound on an `exact` fact; L2, satisfaction stated in the fact's direction, not the requirement's; L3, a bare derived fact beside an absent input two outcomes; L4, `infeasible-configuration` with no provider; L5, one service's facts bound to one counter taken by no acceptance; L6, the mediation gate taken by no acceptance; L7, `M3.1.2` told to put `deffact` in `core.eadl`; L8, §9's corpus claim) | "not acceptable as it stands"; for one provider every §5.2 case right, no stronger precondition passing |
| 13 | 13 | 9 (M1, refinement keeping no value an abstract platform offers, so a system on an abstract platform is judged on values its refinement lacks; M2, a mediating record or adapter reaching the relation outside `M3.2`'s gate; M3, a bare privilege offer outside rule 4's and `M3.3`'s words; M4, the absence rule over every provider refusing `uc3` after `M3.2`; M5, silent providers turning tracked infeasible cases into `missing-fact`; M6, a record's contract as a provider carried by no leaf; M7, a bare offer beside a value two outcomes; M8, the opening's presence sentence; M9, §9's corpus claim) | "not acceptable as it stands"; for one provider every §5.2 case right, no condition read as a capability |
| 14 | 18 | 9 (N1, `uc3` said to build after `M3.2` unchanged, which the record's own horizon and privilege rules forbid; N2, the privilege refusal stopping every use case and the target, unsaid; N3, wiring changing seven tracked `0` verdicts, carried by no leaf; N4, the `ordering` and `region` handover uncarried; N5, the vocabulary in the kinds folder, which a tracked test holds to kinds; N6, a bare name inside `requires` with no outcome; N7, a value inside `needs` dropped; N8, a bare boolean beside `false` two outcomes; N9, the horizon's endpoint, `modulus / rate` ambiguous) | "not acceptable as it stands"; for one provider the directions right and the arithmetic exact |
| 15 | 16 | 8 (O1, a `uses` naming a fact bypassing rule 3 and the value; O2, the ordering case both `unsupported-profile` and kept at `0`; O3, §4 still deciding how the tracked counters migrate; O4, a record's modulus joined to `timer.delay`'s rate; O5, a record's absence against a block's offer decided by no leaf; O6, a record's preconditions carried by no acceptance; O7, the vocabulary module classified two ways; O8, the vocabulary's versioning carried by nothing) | "not acceptable as it stands"; the relation sound at one provider, every worked number and the endpoint argument right |
