# Matching an offer to a requirement

## The idea, in plain words

A description says what each part of a system **offers** and what each part **requires**. A timer block offers a
counter that is 32 bits wide and ticks ten million times a second; a timekeeping service requires a counter whose
readings stay unambiguous for at least a minute. *Matching* answers the question in between: does this offer satisfy
that requirement? It answers one fact at a time, and only for facts the engine has a rule for.

Two things make the question harder than it looks. First, **more is not always better**. A wider counter is better
for a service that needs *at least* 32 bits, but not for a driver that reads exactly one 32-bit word and relies on
what it read. A faster counter is not better either: it wraps around sooner, so the span over which two readings can
be told apart gets shorter. Second, **what a part accepts of its caller is not something more of which is better**.
A function you can only reach from the processor's most privileged mode does not serve a caller in the mode below,
however "strong" that sounds. Matching treats such things as a list of what is accepted, and asks whether the
caller's case is on the list — never whether one level outranks another.

An everyday comparison: a fuse, a plug and a key. For a fuse, a higher rating is fine; for a plug, the pins must be
exactly the right shape; for a key, it opens your door or it does not, whatever else it opens. Each fact the engine
judges is marked, in one shared list called the **capability vocabulary**, with which kind of fact it is.

> **In one minute, for engineers.** The vocabulary `/1` (`docs/semantics/vocabulary/vocabulary.eadl`, written with
> the kind `deffact`) gives every judged fact a domain, a role and a direction: `at-least`, `at-most`, `exact`,
> `includes` (sets) or `within` (intervals); `exactly` is the one direction either side may always write. Values are
> exact rationals over `i128`, and an overflow is `unsupported-profile`, never a verdict. A fact has one outcome at a
> provider — valued, absent, unknown, derived or undescribed — and `unambiguous-horizon` is derived as
> `(modulus − 1) / rate`, inclusive. Preconditions are offered sets judged by inclusion, so no stronger precondition
> passes as a stronger capability. The relation is a predicate per (requirement, provider) and a preorder per fact;
> no lattice is claimed. It lives in `crates/eadl-resolve`, is held to the design's executable model over the model
> checker's universe, and `archogen check` does not call it yet.

## How it works

### The vocabulary

Every fact a requirement may constrain is one entry of the vocabulary. The entry says what kind of value the fact
has, whose word it is, and which way an offered value must lie against a required one:

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

The direction belongs to the fact, so it is checked wherever the fact is written:

| Kind of value | Example fact | Direction | Satisfied when |
| --- | --- | --- | --- |
| boolean | `observation-coherent` | `exact` | the values are equal |
| count | `queue-capacity` | `at-least` | the offered count is at least the required one |
| quantity | `delivery-bound` | `at-most` | the offered time is at most the required one |
| interval | `frequency` | `within` | the required range lies inside the offered one |
| enumeration | `tick-unit` | `exact` | the same alternative |
| set | `reachable-at-privilege` | `includes` | every required member is offered |
| group | `absolute-deadline` | `exact` head | the head holds and every sub-fact's requirement does |

A fact the vocabulary does not declare may still be offered, declared absent or needed — presence judges that — but a
constraint on it is `unsupported-profile`: no rule decides a value without a documented direction.

### What a provider offers

A **provider** is a block or a platform. It offers a fact bare, with a value, or, on an abstract platform, with a
bound a refinement must keep:

```text
(defblock timer.counter
  (offers (counter-modulus 4294967296) (tick-rate 10 MHz) uart))
(defplatform soc.abstract
  (offers (counter-width (at-least 32 bit))))
```

`(tick-rate (exactly 10 MHz))` in an offer is simply the value `10 MHz`. An abstract bound is not a value: it meets a
`needs`, and it leaves a valued requirement unknown. A provider that says two things of one fact — two values, a
value and a bound, an offer and an absence — is refused, since only its author knows which was meant.

### What a side requires

A requirement constrains one fact: a bound in the fact's direction, a bare value read in that direction, or a value
under `exactly`. Presence is written `(needs f)`. A group requires its head and each sub-fact:

```text
(defservice time.monotonic
  (requires (unambiguous-horizon (at-least 60 s)) (tick-unit ns)))
(defservice deadline.absolute
  (requires (absolute-deadline (supported-horizon (at-least 10 s)) (delivery-bound (at-most 50 us)))))
```

Everything one declaration requires is read together, wherever it is written, so two clauses that cannot both hold
are refused exactly as two constraints in one clause would be.

### One outcome per fact, then a verdict

At one provider, each fact has exactly one outcome, in this order: the **value** it offers; **absent**; **unknown**
(offered without a value); **derived** by the fact's rule; or **undescribed**. The requirement is then judged against
it: satisfied, refused, or one of absent, unknown and undescribed when there is no value to compare.

The horizon shows why derivation matters. A counter wraps at its modulus; between two readings a time `Δ` apart it
advances about `Δ × rate` ticks, and the two readings tell that advance apart only while it stays below the modulus.
So the unambiguous span is `(modulus − 1) / rate`:

| Counter | Horizon | Against `(at-least 60 s)` |
| --- | --- | --- |
| `(counter-modulus 4294967296)` at `10 MHz` | `429.4967295 s` | satisfied |
| `(counter-modulus 65536)` at `10 MHz` | `6.5535 ms` | refused |
| `(counter-modulus 600000000)` at `10 MHz` | `59.9999999 s` | refused — `modulus / rate` would have said exactly 60 s |

The same counter at `1 MHz` has ten times the horizon: slower is better here, because the direction belongs to the
horizon, not to anyone's sense of which counter is faster.

### Sets: what a part accepts

`reachable-at-privilege` lists the levels a function can be called from. A supervisor caller is served by an offer
of `supervisor machine` and not by `machine` alone, nor by `user`. `available-in-state` works the same way, with one
member always implied: a requirement for `idle` asks for `idle` and `run`, the state every use runs in.

### The list a resolver chooses from

Matching does not choose. For one requirement it lists every provider in the order given, with its verdict, the
value it found, the derivation it used and the direction it judged in; for a `requires` clause, whether each provider
satisfies all of it. Choosing among them, combining them and checking capacity and ownership is the resolver's work
in later leaves.

## The precise rules

The design is `docs/decisions/decision_substitutability-relation.md`; `docs/semantics/model.md` §7 is normative over
the code. Section numbers below are the record's.

### The vocabulary is a typed table

`crates/eadl-resolve/src/vocabulary.rs` reads the vocabulary against `docs/semantics/kinds/deffact.eadl`, holds each
entry's frame to the kind with the validator every declaration meets, and then refuses what a frame cannot say: a
direction its domain does not admit (§2's table), a derivation without its rule or with inputs that rule does not
read, a cycle among `derived-from` and `reads`, `implies` off a set fact, a name twice, and a fact named like a
clause word of the description kinds, since an offer on it would read as the clause (§1.1). The vocabulary is part
of `eadl/1`: it is frozen with the language, and `archogen check` answers a file that writes a `deffact` as the
language's own definition, exit `20` ([Checking a description](checking.md)).

### Values and arithmetic

`crates/eadl-resolve/src/value.rs` (§2). A count is a non-negative integer, with an optional dimensionless unit, or
`(pow2 N)` for `N` from 0 to 126. Two amounts in one unit compare by their written numbers; in two units, in the base
unit. Information is a positive whole number of bits; a counter's modulus is positive. A comparison whose exact
arithmetic overflows decides nothing and is `unsupported-profile` — the value domain's limit, which a wider domain
would lift under §15.

### Offers

`crates/eadl-resolve/src/offer.rs` (§1, §2, §4, §5, §8). Refused, `invalid-description`: an item of `offers` naming
nothing or holding a clause; a list inside `absent`; an offer or an absence of a statement fact; a value outside its
domain; a direction's name, `includes`, `within` or `exact`, used as a wrapper; a boolean offered with a bound other
than `exactly`; a bound against the fact's direction; one fact offered and absent, with two values, or with a bound
beside a value; a derived fact offered or declared absent beside a fact its rule reads; a modulus above `2^width`,
or beside `(wrap-behavior saturating)`. One value written in several spellings is one offer, kept in every spelling,
and a comparison is decided by any spelling whose arithmetic does not overflow (§5).

### Requirements and sides

`crates/eadl-resolve/src/requirement.rs` (§1, §3 rules 5 and 6). A side holds every `requires`, `needs` and `uses`
written at any depth of a clause but `offers`, `absent` and `refines` — the positions presence reads. A `platform`
clause holds `needs`, `uses` and `requires` alone; `requires` holds constraints, `needs`, `uses` and nested
`requires`, each a clause of its own; `needs` and `uses` hold names. A `uses` naming a fact, a `needs` of a
statement, a bare name or `(f)` inside `requires` are refused. On one fact, two equalities that differ, or an
equality another constraint refuses, are a contradiction; what the arithmetic cannot decide is
`unsupported-profile`. A declaration whose local name is a fact is refused, at the root and inside every imported
module, where its qualified name would hide it (§1.1).

### Outcomes and verdicts

`crates/eadl-resolve/src/relation.rs` (§3, §4). Rule 1's order is value, absent — declared, or a required input of
the derivation declared absent — unknown — offered without a value, or an input the rule reads so offered — derived,
undescribed; an optional input declared absent derives nothing. A boolean named in `needs` is `(f true)`, so `false`
meets no presence; any other offer meets presence. A group's verdict is the first of refused, absent, unsupported,
unknown and undescribed among its head and its parts, whatever order they are written in. A statement, such as
`or-through-mediation`, is the requirer's own word: checked for its domain, judged against no provider.

### Codes

Every refusal is reported under the record's two codes, rows of `model.md` §6: `invalid-description` for what a
description writes that cannot mean anything, and `unsupported-profile` for a fact no rule decides or a comparison
past the arithmetic. The diagnostic names the fact and what is wrong and says what to do; the library keeps each
rule a typed cause (`crates/eadl-resolve/src/refusal.rs`). A code of its own per cause waits until a command can reach
the relation (`model.md` §7 rule 6).

### What is not claimed

The relation is a predicate on one requirement and one provider. Per fact its direction induces a preorder and
nothing more: no join, no meet, no best offer, and no lattice (§7).

## Today and ahead

The relation is a library, complete for `M3.1`, and held to the design's executable model
(`crates/eadl-resolve/src/model/`) over the model checker's whole universe by `crates/eadl-resolve/tests/production.rs`.
Measured `2026-10-06`: 154 381 provider-and-requirement pairs judged alike, 72 234 provider declarations read alike,
and the horizon agreeing with a simulation of the counter's reads on every case of the checker's grid. Each rule
has a catalogued mutation that its test must kill (`xtask/mutations.txt`).

`archogen check` does not call it yet, so no verdict a description gets today comes from it. Leaf `M3.4` wires it in,
over the description's providers, deciding a description's `missing-fact` or `infeasible-configuration` from the
enumeration and keeping or migrating every verdict it moves. `M3.2` adds adapters — a provider in its own right, such
as an epoch extender — offered only where a requirer allows mediation; `M3.3` judges what one provider cannot, such as
the privilege context a plan binds a caller to; `M3.7` makes a catalog record a provider.
