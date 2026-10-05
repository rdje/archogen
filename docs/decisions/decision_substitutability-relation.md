# The substitutability relation: when an offer satisfies a requirement, over a decidable fragment

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active` — written; under independent review (leaf `M3.1.1`'s closure rule: the first round that
  finds no defect closes it); rounds 1 to 15 answered `2026-10-03`, rounds 16 to 20 `2026-10-05`, its hand-offs
  in a ledger since (§11), reviewed beside its executable model since round 17 (`M3.1.1.1`)
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
written direction — the fact's, or equality under `exactly`, a set fact's implied members joined to what it writes
either way (R12 L2; R18 3) — or, for a presence requirement, when an offer of the fact other than `false`, or its derivation, meets it by §3 rule 3 (R13 M8). Nothing is inferred from a value being "more" unless the vocabulary says more is better for that fact,
and a stronger precondition is never a stronger capability, because what a provider accepts of its caller — the
privilege levels it is reachable from, the power states it works in — is offered as a set and judged by inclusion,
which reads no order. The relation is a predicate between one requirement and one provider (R12 L9). It induces a
preorder on the values of each fact and nothing more: no join, no meet, no best offer, and no lattice is claimed
(§7).

### 1. What is compared

**The surface today.** An offer writes a fact bare, `(offers counter-width)`; with a value, `(counter-width 32 bit)`,
`(observable-output true)`, `(tick-unit ns)`, `(counter-modulus 4294967296)`, `(frequency (range 1 MHz 200 MHz))`;
or with a bound, `(counter-width (at-least 32 bit))`, which an abstract platform writes (model §3) and which the
relation reads the same way from a block, as `archogen check` admits one today: the fact offered, its value unknown to
a valued constraint (§3 rules 1, 3; R18 4). A
requirement writes a constraint inside `requires`: a bound, `(unambiguous-horizon (at-least 60 s))`; a value,
`(preemptive true)`, `(available-in-state idle)`, `(queue-capacity 8 tick)`; a list, `(priorities static unique)`;
or a group, `(absolute-deadline (supported-horizon (at-least 10 s)) (delivery-bound (at-most 50 us)))`. `absent`
names a fact the provider does not have, by its name alone, a value inside it being `invalid-description` (§8; R18 2); `needs` names the facts and services a declaration depends on, which is
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
`kind_module` recognises a `defkind`: a module file is classified first, as today, so a `deffact` inside a module's body is `schema-unknown-kind` and one
beside a `defmodule` `module-multiple-forms`, exit 10, both refusals (R16 3); any other file whose declarations include
a `deffact` is answered as the vocabulary is,
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
| `implies` | forms | members of the fact's set | for a set fact, the members every valued constraint on it holds beside what it writes, under every written direction — `available-in-state` implies `run`, the state every use runs in; a presence requirement writes no value and asks only for an offer (§2, §3 rule 3; R16 2; R17 1, 2; R18 3) |
| `doc` | a string | a string | what the fact means, one line; required, as a kind's `doc` is |

`deffact`'s clauses are `(holds forms)` for `domain`, since `(ordered …)` nests, and for `derived-from`, `reads` and
`implies`, which list facts or members (R18 8); `(holds values string)` for `doc`; and `(holds values symbol)` for `role`, `direction` and
`rule`, each exactly one value — reference §7 rule 4 makes `values` a fixed number, and `kind.rs` refuses any
other as `schema-arity` (R2 B3). `doc`, `domain`, `role` and `direction` appear exactly once, `derived-from`,
`reads`, `rule` and `implies` at most once, `implies` only on a set fact (R3 C11; R17 2). A fact's name lives in the vocabulary, not in a
description's declaration namespace: the vocabulary is validated on its own, never checked as a description. A
declaration whose local name is a vocabulary fact is `invalid-description`: module resolution (reference §6 rule 10)
binds a `uses`, `needs` or `refines` operand to a declaration of its own instance before anything else, so such a
declaration would capture a requirement on the fact, and an import would rename it out of the relation's sight
(measured, R16 1). No tracked description declares one; `presence.rs`'s unit fixtures that do judge presence alone
and are untouched (R8 H7; R10 J12; R11 K7; R16 1). And
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
boolean, an enumeration, a set, an interval or a unitless count are the library's reading, and `archogen check`
goes on refusing them (R15 10; R16 6), and keeping model §6's `fires on` inputs for `quantity-missing-unit` and
`quantity-missing` (R11 K8; R12 L1). Equality is the one bound every direction admits, so it never reads a value
the wrong way; it refuses what the fact's direction would accept, never the reverse, and it is how a requirer says
that for it the value is the interface, not a capacity (§6, "more bits"). `exactly` is the written form of the
direction §2 calls `exact`, as `refinement.rs` reads it (R1 A11). The written direction stays in the description,
as today's corpus writes it, because a reader of a bound should see which way it runs without the vocabulary open;
a bare value, `(counter-width 32 bit)`, takes its direction from the vocabulary, which is what makes both checkable
(R16 9).

**The entries of `/1`**, one per fact the census writes, each direction with its reason; `M3.1.2` ships them as
`vocabulary.eadl` and adds none without a row here (R2 B8). A fact marked *interface* takes `exact` because a
different value is a different interface, not a better one (§5.2); a *capacity* takes the direction in which more
of it satisfies more.

| Fact | Domain | Role | Direction | Why |
| --- | --- | --- | --- | --- |
| `absolute-deadline` | group `supported-horizon delivery-bound` | guarantee | `exact` | a boolean head |
| `application-mutexes`, `general-ipc`, `descriptor-format` | boolean | guarantee | `exact` | presence facts; the first two are §3.1 exclusions |
| `available-in-state` | set `run idle sleep`, `(implies run)` | guarantee | `includes` | the offer covers every state required and `run`, the state every use runs in, which the entry's `implies` names: `(available-in-state idle)` asks for `run` and `idle`, and `(available-in-state (exactly idle))` for exactly those two (R16 2; R17 1, 2) |
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
never a verdict on the requirement — an interval whose endpoints' order is past it included, and an interval judged
`within` or `exactly` with both endpoints compared before either decides, so one endpoint's refusal never hides the
other's overflow (R18 7) — `unsupported-profile` rather than `analysis-inconclusive` because the limit is
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
| `set a b c` | `(f a b)`, alternatives without repetition, at least one — `(f)` with none is a bare offer (R12 L12) | `(f b)`, `(f a b)`, or `(f (exactly a b))`, without repetition (R19 7) | `includes`, `exact` | the required set, with every member the entry's `implies` names joined to it, is contained in the offered one; under `exactly`, equal to it (R17 1, 2) |
| `group g h …` | `(f true)` or bare `f`, and the sub-facts offered on their own; the head takes a bound as a boolean does (R10 J4) | `(f true)`, the head alone, which the head named in `needs` also means (R9 I6), or `(f false)`, the head required false as a boolean's row reads (R18 5); or `(f (g …) (h …))`, the head and sub-constraints, a head value beside them being `invalid-description` (R19 7); a sub-fact is a fact in its own right, beside a head declared absent or `false` too (R19 5), and may be constrained flat beside the head, as `examples/alternative-timer/system.eadl` writes `(absolute-deadline true)` beside `(supported-horizon (at-least 10 s))` (R2 B6) | the head is `boolean exact`; each sub-fact its own | the head holds and every sub-constraint, nested or flat, is satisfied, each judged as a requirement on its sub-fact |

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
     is something the offer says, and no derivation corrects it (§4; R3 C1; R7 G3; R10 J7) — save that a derived fact
     declared absent where `P` offers a fact its rule reads is `invalid-description`, as an offer of it there is (§4),
     since `ROADMAP.md` §5.3 rejects contradictory declarations rather than choosing one (R16 8);
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
   declaration's own level (R10 J2) — is satisfied by any offer of `f` — bare, valued, or a
   bound, an abstract platform's or a block's (§1; R18 4) — other than a boolean or a group's head whose value is `false`, `(f (exactly false))` included (R10 J4),
   or by `f`'s derivation where the vocabulary names one (§4; R7 G10), and by nothing
   else: a boolean `f`, or a group's head, named in `needs` is the requirement `(f true)` (R9 I6), so `(f false)` satisfies no presence
   requirement, as `absent` does not (R7 G2); its outcome at `P` is otherwise rule 1's (R6 F3; R8 H6). **A guarantee** is
   satisfied when the value lies in `R`'s written direction against `R`'s bound — the fact's direction, or equality
   under `exactly` — by §2's table (R12 L2); at a provider declaring `f` absent it is absent, `(f false)` included,
   since absence offers no value and none is inferred (R19 4). `exact`
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
   group's sub-constraints count among its constraints (R12 L13). A group requirement's outcome at `P` is its head's
   when the head is not satisfied; otherwise each sub-constraint is judged on its own sub-fact, and the group is
   satisfied when every one is, and else takes the first of refused, absent, unsupported, unknown and undescribed among
   them — whatever order they are written in (R17 4). A clause whose constraints on one fact no value satisfies
   together — a statement with two values, two equalities that differ, an equality another constraint refuses — is
   `invalid-description`: contradictory requirements are refused, never one chosen (`ROADMAP.md` §5.3; R19 2, 8);
   past the arithmetic it is `unsupported-profile`. Its `uses` and services are §1's, never constraints (R20 1, 2).
6. **A statement** — a fact whose role is `statement`, such as `or-through-mediation` — is the requiring side's
   word about itself. It is checked for its domain and nothing else: it constrains no provider, puts no fact
   inside the closure, and is read by the rule or the leaf its entry names, which for `/1`'s one statement is
   `M3.2`'s adapter search (§6, "mediation"; R1 A3), whose acceptance carries it. It is the requiring side's word
   alone: an offer of a statement fact, wherever written, a service's `offers` included, a declared absence of one,
   and a `needs` of one, are `invalid-description` (R2 B10; R12 L6, L13; R16 7; R17 R4). It is written bare or under
   `exactly`, the one direction either side may always write, and with one value in a clause (rule 5; §1.1; R17 3;
   R19 2).
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
`(2^64 − 1) / 10 000 000 s = 1 844 674 407 370.9551615 s`, and fits the arithmetic. With `(counter-modulus 4294967296)`
at `10 MHz`, the horizon is
`4294967295 / 10 000 000 s = 429.4967295 s`, and `time.monotonic`'s `(unambiguous-horizon (at-least 60 s))` is
satisfied; `timer.delay` in `examples/alternative-timer/system.eadl`, at `1 MHz` and with that modulus stated, would
have `4294.967295 s`, slower and better here. A 16-bit counter, `(counter-modulus 65536)` at `10 MHz`, has
`65535 / 10 000 000 s = 6.5535 ms` and
fails the same requirement: the faster wrapping counter §5.2 warns of, caught because the direction belongs to the
horizon and the derivation to the vocabulary, not to anyone's sense of which counter is better. A reload counter,
`(counter-modulus 1000000) (tick-rate 10 MHz)`, has `999 999 / 10 000 000 s = 0.0999999 s` and fails it too (R2 B1). A
counter that writes `(unambiguous-horizon 3600 s)` beside its modulus, its rate or its `wrap-behavior`, bare or
valued, is `invalid-description`: the horizon is derived where its grounds are offered, and a horizon that long over
that hardware is an epoch extender's, a provider in its own right offering the horizon alone, or, where a service
needs the counter's other facts of one provider (`SR-H14`), its extended modulus and rate beside them, from which
the horizon is derived (R4 D1; R5 E1; R6 F1; R19 10).

### 5. Candidate enumeration

For one requirement and the providers it is judged against, the enumeration lists each provider with its outcome
under §3 rule 1 — valued or derived and satisfied or refused, absent, unknown, undescribed — with the value, the
derivation used and the direction; and, per `requires` clause and provider, rule 5's verdict, the clause satisfied by
that provider or not (R3 C8). An offer is one provider's statement (§1), so two providers offering one fact with two
values are two offers, each judged on its own, as two catalog records will be (R2 B5). One provider offering one
declared fact twice with two values, or a bound beside a value, is `invalid-description`; the same value twice — the same in the fact's domain, `10 MHz` beside `10000 kHz` included (R15 13) — is one
offer, and a bare offer beside a value of the same fact is that value, in a domain where bare presence has no value;
for a boolean or a group's head, where bare is `true`, a bare offer beside `(f true)` is the same value twice and
beside `(f false)` two values, `invalid-description` (R5 E10; R6 F1; R10 J8; R11 K9; R13 M7; R14 8). Where comparing two
offers of one fact overflows the exact arithmetic, the fact is `unsupported-profile` at that provider (§2), for presence
too (R20 4), never two values — unless two of its offers compare unequal without overflowing, which are two values whatever the others and
whatever order they are written in (R17 5; R18 1). Two amounts in one unit are compared as written, by their
numbers, in every comparison: one value written twice is one offer, two that differ two values (R19 1, 6; R20 2). An undeclared fact
has no domain to contradict, so `region`, offered once per named region, is untouched (R1 A6; §10). When `M3.7` gives a
catalog record's contract offers the relation can judge, a record is one more provider, judged like a block, and a
record that mediates enters only through `M3.2`'s search (R13 M2, M6).

**What the enumeration is handed, and what its list makes of a description, are `M3.4`'s** — the search that wires
the relation into `archogen check` — in the hand-off ledger's `SR-H15` to `SR-H20` (§11): the closure, absence across
providers, one clause across several providers, the description's code, the providers enumerated and the tracked
verdicts wiring moves (R2 B9; R6 F8, F9; R8 H1; R10 J2, J3, J10; R11 K1–K3, K5, K6; R12 L4, L16; R13 M4, M5; R14 3;
R15 1).

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
| power state | `available-in-state` | set `run idle sleep`; guarantee; `includes` | offered `run idle` ⊇ required `idle` with `run`; offered `run` does not include `idle`, and offered `idle` alone does not include `run` (R16 2) |
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
| `invalid-description` | a bare fact name, or `(f)`, inside `requires`; a clause's constraints on one fact that no value satisfies together (§3 rule 5; R19 2); a list inside `needs` naming a vocabulary fact; a list inside `absent` naming a vocabulary fact, since `absent` names a fact by name alone and a value there would read as the whole fact absent (R18 2); a `uses` naming a vocabulary fact; a declaration whose local name is a vocabulary fact (R16 1); a derived fact declared absent beside a fact its rule reads (R16 8); a direction wrapper other than `at-least`, `at-most` and `exactly` — a list headed by a direction's name, `(f (includes …))`, `(f (within …))`, `(f (exact …))` — since a requirement writes its value and the direction is the fact's (R15 16; R16 4); a value outside its fact's domain; a value of information that is not a positive whole number of bits, and a `counter-modulus` of 0, wherever written — an offer, a bound or a requirement (R17 6); a statement fact declared absent (R17 R4); a provider's own name a vocabulary fact (R17 R5); a set written with no member in a requirement, or as `(f (exactly))` in an offer — `(f)` in an offer is bare (§2;
R13 M11) — or an interval with `lo > hi` on either side; a `needs` of a statement fact; a direction written against the vocabulary's, `exactly` apart (§1.1); a provider offering and declaring absent one fact; one provider offering one declared fact with two values, or a bound beside a value; a boolean or group head offered with a bound other than `exactly`; a derived fact offered beside a fact its rule reads (§4); a `counter-modulus` above a valued `2^width`, or beside `(wrap-behavior saturating)`; an offer of a statement fact |
| `unsupported-profile` | a constraint on a fact the vocabulary does not declare, `(x)` or `(x v)` (R19 7); exact arithmetic that overflows `i128` |

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
`low-power-timer` passes presence today and is `invalid-description` here (§1.1; R16 1); the library's
verdict is the stricter, and which one the report carries is decided when `M3.4` wires it (R2 B9). `quantity.rs`'s
comment on `ComparisonDirection::Exact` names a counter modulus as its example, and §1.1 agrees (R3 C9; R5 E4). `docs/semantics/model.md` gains a section normative over the crate when
the crate lands (`M3.1.2`), so that `reference.rs` holds its codes to the document as it holds every other
source's.

### 10. What stays open

Each item is another leaf's; what a leaf must build is §11's hand-off, quoted there. Here, only why it is not
decided in this record.

- Which provider a service is bound to, and which constraints of one clause different providers may meet, are
  `M3.3`'s and `M3.4`'s; the relation judges each provider on its own, a group never split from its sub-constraints
  (§5; R3 C3; R6 F9; R11 K3; R12 L5).
- What a refinement keeps of an abstract platform is model §3's: `M1.40`, closed `2026-10-03`, keeps a value only by
  the same value as written or, for a quantity, the same amount (R12 L17; R13 M1).
- `region` and `ordering` fit no domain of §2 and stay undeclared in `/1`, placement and ordering being `M3.3`'s;
  whether they gain entries or `required-ordering-guarantee.eadl` migrates is decided there and by `M3.4`'s
  keep-or-migrate rule (R1 A10; R9 I11; R14 4; R15 2).
- `uc3`'s seal: `timer.delay` states no modulus and no privilege level, and its policy waits on a record, so its
  horizon and level come from one provider — itself, an adapter or a record after `M3.7` — `M3.2`'s to choose, the
  director's before the description changes (`SR-H10`; R14 1; R15 4).
- `/1` has no condition role: the context a plan binds a caller to is §5.4's privilege constraint, `M3.3`'s; a clock
  a block must be fed is another fact, added when a description needs one (R2 B2; R10 J1, J6).
- Arithmetic in a description stays refused: `(pow2 N)` is a literal's spelling, and a new derivation is a new named
  rule (R8 H2).
- A catalog record as a provider is `M3.7`'s, its offers and absences meeting a block's by model §2 rule 2's
  amendment, its preconditions read as offered sets (`SR-H21`; R13 M2, M6; R15 5, 6).
- A derived fact as an abstract platform's bound cannot yet be refined by a concrete platform stating its grounds;
  teaching refinement a derivation is a later leaf's (R9 I8; R15 12).
- A saturating counter states no horizon in `/1`; its range is another fact (§4; R9 I10).

### 11. The hand-off ledger

Every obligation this record hands to another leaf is one sentence here, quoted word for word beside its identifier
by the leaf that takes it, which `HANDOFF-LEDGER` checks (`PROGRAM.52.2`); the sections above give the reasons, and
the review history the rounds that found each.

<!-- machine-read: handoffs -->
| Id | Leaf | Obligation |
| --- | --- | --- |
| `SR-H1` | `M3.1.2` | `crates/eadl-resolve` holds the vocabulary as a typed table, the domains, the relation with an offer reader of its own, the derivation rule and the enumeration as the record decides, each rule of its §1.1, §2, §3, §4, §5 and §8 a test that fails when the rule is removed, a test per domain, and every row of its §6 a test. |
| `SR-H2` | `M3.1.2` | `deffact` lives in its own kind module, `docs/semantics/kinds/deffact.eadl`, which no description's registry holds, and the vocabulary in `docs/semantics/vocabulary/`, a root of `docs/semantics/conformance.md`, with the kinds root's purpose amended and a migration note covering both in `eadl/1`'s baseline. |
| `SR-H3` | `M3.1.2` | `archogen check` classifies a module file first, refusing its `deffact` exit 10, and answers any other file with a `deffact`, recognised by content as a `defkind` is, exit 20 as the language's own definition, frozen in `verdicts.txt`, while `kind_modules.rs` holds every file under `docs/semantics/kinds/` to declaring a kind. |
| `SR-H4` | `M3.1.2` | A declaration whose local name is a vocabulary fact is refused, at the root and inside an imported module. |
| `SR-H5` | `M3.1.2` | `docs/semantics/model.md` becomes normative over the crate and decides whether each cause of the record's §8 rows takes a code of its own. |
| `SR-H6` | `M3.1.2` | `refinement.rs`'s reader is not the relation's to change, and an abstract `exactly` still refuses a different concrete value. |
| `SR-H7` | `M3.1.2` | The production relation is tested against `M3.1.1.1`'s executable model over the same bounded universe. |
| `SR-H8` | `M3.2` | A mediating adapter — a path across a privilege boundary through firmware or another mediator — is offered only where the requirer wrote `or-through-mediation` as `allowed`, bare or under `exactly`. |
| `SR-H9` | `M3.2` | An adapter's offer satisfies past a declared absence, model §2 rule 2 amended to say so, and the absence across providers is judged among the description's blocks and platforms, the adapter not one of them. |
| `SR-H10` | `M3.2` | `examples/alternative-timer` builds as `examples/README.md` seals, each fact the relation needs of it — a horizon, a privilege level, its policy's constraints — offered by one provider judged on its own, `timer.delay`, the adapter — an epoch extender offering its extended modulus and rate beside the counter's other facts where a service needs them of one provider (`SR-H14`) — or a record after `M3.7`, never a record's fact joined to `timer.delay`'s, and a change to its description is brought to the director before it is made. |
| `SR-H11` | `M3.3` | A caller bound to an execution context that its provider's `reachable-at-privilege` does not include is refused, a requirer that states no level included. |
| `SR-H12` | `M3.3` | A provider that states no level — `reachable-at-privilege` undescribed, declared absent or offered bare — is refused for any caller a plan binds to it, so every provider a plan binds a caller to states its level, the tracked descriptions and the target migrated in this leaf, the target's level from its agreement and a ledger source, or the refusal narrowed here under a review. |
| `SR-H13` | `M3.3` | A required ordering, `ordering`, and a region's placement, both undeclared in `/1`, are judged here, with entries and domains added under §15 first, or `required-ordering-guarantee.eadl` migrated by `M3.4`'s rule. |
| `SR-H14` | `M3.3` | The requirements one service makes of one device's facts are met by the one provider the plan binds it to, or by an adapter `M3.2` selects, so a service whose horizon one counter meets and whose coherent reads another does is refused. |
| `SR-H15` | `M3.4` | The closure is rooted at the system with its tasks and platform, `uses` and `needs` followed transitively, every constraint of a `requires` and every `needs` of a vocabulary fact judged wherever written, a `uses` naming a vocabulary fact refused, and the closure the report carries decided where it and presence's differ, a provider on a platform the system does not use never satisfying. |
| `SR-H16` | `M3.4` | A fact declared absent at one of the description's blocks and platforms and offered at another, outside a direct refinement pair, is refused as `invalid-description` over every offer and absence and through no chain of refinements, `presence.rs`'s first-of-each comparison corrected, an adapter `M3.2`'s search offers not being one of them, with the reason it records for reading two providers' absence and offer as one contradiction. |
| `SR-H17` | `M3.4` | No group and its sub-constraints, nested or flat, are met by different providers, and this leaf decides which other constraints of one clause may be. |
| `SR-H18` | `M3.4` | The description's code is decided over the providers that take part — those stating `f`, offered in any form or declared absent, or an input of its derivation, a silent one deciding nothing — `unsupported-profile` while one that takes part is past the arithmetic and none satisfies, before the others; `missing-fact` while one that takes part is unknown or undescribed and none satisfies, and when none takes part; `infeasible-configuration` once at least one takes part and every one that does is refused or absent, with model §2 rule 2 amended where a provider's derivation or an adapter's offer satisfies past another's absence. |
| `SR-H19` | `M3.4` | The enumeration lists the description's blocks and platforms, a catalog record only after `M3.7`, and a mediating record never directly. |
| `SR-H20` | `M3.4` | Every verdict `crates/archogen-cli/tests/verdicts.txt` and `module_cases.rs` freeze is kept, or changed only in the commit that names the case, the rule that moves it and its migration — a description gaining a fact, the target's agreement and its ledger source included — ordered after `M3.3` and `M3.7` where a migration needs them, the seven `0` verdicts `M3.1.1`'s round 14 measured among them. |
| `SR-H21` | `M3.7` | The catalog record decision's §2 is amended so a contract facet states offers and absences of vocabulary facts, read by the relation's reader and judged like a block's, and a record whose path crosses a mediation boundary says so and enters only through `M3.2`'s search, where the requirer wrote `or-through-mediation` as `allowed`, bare or under `exactly`. |
| `SR-H22` | `M3.7` | A design reviewed by a context that did not write it answers how a record's offer or absence meets a block's absence or offer, named in model §2 rule 2's amendment and decided with `M3.4`, and how a record's preconditions become offered sets judged by inclusion, a record with a precondition no entry states being no provider. |

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

Defects per round, oldest first: 7, 7, 8, 3, 3, 3, 5, 1, 2, 8, 7, 8, 9, 9, 8, 3, 6, 2, 2, 2. Each round's findings, its reader's measurements and every answer are in
the review history linked above, which holds them whole; this record keeps only the count (`PROGRAM.52.2`, the
folder's ceiling).
