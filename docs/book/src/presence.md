# Presence, absence, and relevance

## The idea, in plain words

Ask whether a car has air conditioning and you can get a yes, a no, or "I don't know" — and "I don't know" is not
a no. A description says the same kinds of thing about every fact: it is **offered**, it is **absent**, or nothing
says. archogen keeps them apart because each sends the author off to do something different: a no is a definite
answer, an "I don't know" is a gap to fill.

A gap matters only where something depends on it. A board's description mentions hardware the system never uses,
so archogen works out what the system actually depends on — the **closure** of what it requests — blocks only on
gaps inside it, and lists the rest for the author to see. A description that says both yes and no about the same
fact is refused: archogen will not guess which half was meant.

> **In one minute, for engineers.** Offered is a claim, not evidence; absent is a definite negative; undescribed is
> neither. By `ROADMAP.md` §5.3, an unknown inside the transitive closure of the requested services is
> `missing-fact`, and one outside it is metadata that `archogen check` lists and nothing fails. A required fact
> declared absent is `infeasible-configuration`; one declaration offering and declaring absent one fact is
> `invalid-description`, reachable or not, as is one declaring absent what a part it composes offers; two other declarations' statements are two. F04, F05 and F06 run against one description, so neither blocking on every unknown nor on none passes.
> The pass is `crates/eadl-model/src/presence.rs`, and it is not provider resolution.

## How it works

A description says one of three things about any fact:

| State | Meaning |
| --- | --- |
| **offered** | the description claims it — a *claim*, not evidence |
| **absent** | the description states it is not available — a negative fact is a fact |
| **undescribed** | nothing says |

The third is not the second. That difference decides which diagnostic an author gets, and
therefore what they go and do.

## The precise rules

### Relevance decides whether an unknown matters

The original plan said an unknown capability anywhere blocks generation. That was revised, and
the roadmap records why: a platform description carries facts about hardware the system never
touches, and failing on those makes every real platform undescribable.

> Compute the transitive dependency closure of the requested services… **Unknown facts inside
> that closure block the relevant decision. Unknown facts outside it remain visible in metadata
> and do not fail generation.**

So on one description, with two equally undescribed facts:

```text
(defsystem app.rt      (uses time.monotonic))
(defservice time.monotonic (needs counter-width wrap-behavior))
(defblock timer.counter    (offers counter-width))
(defblock unrelated.dma    (needs dma-channels))
```

`wrap-behavior` is inside the closure and blocks:

```text
error[missing-fact]: `wrap-behavior` is required by this system and nothing describes it
  = hint: declare `wrap-behavior` as offered or absent on the platform — it is inside the
          dependency closure of what this system requests, so no decision that depends on it
          can be made
```

`dma-channels` is outside it, produces **no** diagnostic, and stays visible, because §5.3 requires
it to: `archogen check` lists the closure after its verdict, with what pulled each fact in, and then
everything outside it. On F05's own fixture, where `descriptor-format` is needed only by an engine
nothing requests:

```console
$ archogen check docs/semantics/cases/positive-unrelated-unknown.eadl
docs/semantics/cases/positive-unrelated-unknown.eadl: accepted against profile `rt-static-up-v1` (5 declaration(s))
  closure: counter-width (needed by time.monotonic), soc.p (requested), time.monotonic (requested), timer.counter (requested)
  outside the closure, needed by nothing requested: core-count, descriptor-format, other.engine, s, transfer-engine
  this checks the description, not a system: no resolution, generation or analysis has run
```

The closure lines are metadata, not diagnostics: they fail nothing, and the reference's §4 rule 1 (no
warning, no note) still holds (`docs/semantics/model.md` §2 rule 4). ⚠️ Until leaf `M1.30` this
paragraph said the fact "appears in the report's metadata" and it did not: the presence pass computed
the boundary and nothing showed it. A test now reads the report of every accepted description
(`crates/archogen-cli/tests/closure_report.rs`), so the boundary cannot be dropped again unseen.

The fixtures F04 and F05 are deliberately run against the *same* description. A checker that
blocks on every unknown passes F04 and fails F05; one that blocks on nothing passes F05 and
fails F04. Only relevance passes both — and using two different descriptions would have let two
different bugs pass.

### Absent is a definite answer

```text
error[infeasible-configuration]: `low-power-timer` is required by this system but declared absent
  = hint: either the requirement or the platform is wrong; an explicitly absent fact is a
          definite answer, not a gap to be filled in
```

Reporting this as `missing-fact` would send the author off to describe something the platform
has already said it does not have.

One exception, and it is not a weakening: a fact the **engine** can supply. Some facts an
adapter the engine knows can realize over another device — an absolute deadline programmed
through a relative delay timer (`docs/specs/adapters/decision_engine-adapters.md`, `SR-H9`).
For such a fact, presence asks whether one block or platform of the description offers, by
name, everything that adapter needs. If one does, the fact is left to the search, which binds
the adapter or says why not; the platform's absence still stands for the platform, and the
adapter is no declaration of the description. If one offers part of it and leaves the rest
undescribed, the answer is `missing-fact`, naming the facts to describe — `uc3`'s refusal,
whose timer leaves its modulus undescribed ([Checking a description](checking.md)). Presence
reads names only, so a path that holds by name and fails by value is the search's to refuse
(leaf `M3.2.4`).

A second kind of fact no declaration states at all: one the vocabulary **derives**. How far a
counter can be read without ambiguity, `unambiguous-horizon`, is computed from its modulus and its
tick rate, and the search computes it at the device that offers both. So presence asks the same
question of such a fact, by name: does one block or platform offer everything the rule needs? If
one does, the fact is the search's to derive and bind. If one offers part, the answer is
`missing-fact`, naming what is left to describe there. Before leaf `M3.4.17.1` a need written
directly on such a fact was refused by `check` and bound by `resolve`, on one description; a test
at the command now holds the two together (`crates/archogen-cli/tests/presence_derived.rs`).

### Contradictions are rejected, never resolved

A contradiction is one declaration saying both of itself — F06's case, `docs/semantics/cases/invalid-contradictory-presence.eadl`:

```text
$ archogen check docs/semantics/cases/invalid-contradictory-presence.eadl
error[invalid-description]: `low-power-timer` is declared both offered and absent by `a`
  --> docs/semantics/cases/invalid-contradictory-presence.eadl:11:46
   |
11 | (defblock a (offers low-power-timer) (absent low-power-timer))
   |                                              ^^^^^^^^^^^^^^^ declared absent by `a`
  --> docs/semantics/cases/invalid-contradictory-presence.eadl:11:21
   |
11 | (defblock a (offers low-power-timer) (absent low-power-timer))
   |                     --------------- declared offered by `a`
  = hint: remove one declaration — a contradiction is not resolved by preferring one side, because only the author knows which half was meant
archogen: invalid-description: 1 diagnostic(s) in docs/semantics/cases/invalid-contradictory-presence.eadl
```

Both sites are named, because the author looking at one of them cannot see the other.

Two declarations are two statements. A serial port declaring `(absent debug-port)` beside another offering
`debug-port` describes two devices, not a contradiction, and nothing refuses it: the fact is offered, and a requirement
on it is the search's to bind to the port that has it. A declaration's absence does cover what it is made of: a
board declaring `(absent debug-port)` while composing — through any `uses` it writes, at any depth — a port offering
it says both of itself, and is refused as F06 is. Every declaration is compared, a service's too, not the first
of each fact (leaf `M3.4.8`, `docs/decisions/decision_absence-per-provider.md`, which made the rule per declaration
from description-wide; F06's case wrote its blocks so before it). An abstract platform's absence is another matter, model
§3's: a platform refining it must not offer the fact, nor compose a block that does, nor may any refinement further
down its chain.

Unlike an unknown fact, a contradiction is invalid whether or not it is reachable. §5.3 states
the relevance rule for unknowns specifically, and states the rejection rule for contradictions
without a qualifier.

### Offered is a claim, not evidence

> "Offered" records a claim whose evidential status is separate; declaring it does not make it
> proven.

Nothing in this analysis says a platform *really* has a capability. Presence is what the
description asserts; whether that assertion is backed lives in the evidence vocabulary, with its
own category and its own scope.

### What this is not

This is presence and relevance analysis, not provider resolution. It answers "is every fact this
system depends on actually known?" It does not choose implementations, allocate resources, or
check capacity — that is the joint resolver, and conflating the two here would produce a
resolver nobody reviewed.

### Where it lives

`crates/eadl-model/src/presence.rs`, run as the fifth pass of `archogen check`. Fixtures **F04**
(a relevant capability undescribed → `missing-fact`), **F05** (an irrelevant one → the system
stays admissible) and **F06** (one declaration offering and declaring absent one fact → `invalid-description`) are the two
halves of §5.3's single decision, in `crates/eadl-model/tests/f04_f06_presence.rs`, with worked
cases in `docs/semantics/cases/`.

### What comes after presence: matching

Presence asks whether a fact is known. The next question is whether what a provider offers is *good enough* for what a
requirement asks — whether a 64-bit counter satisfies a service that needs 32 bits, or an idle-only timer one that must
keep running. That is matching, and [Matching an offer to a requirement](matching.md) treats it. Its design, reviewed
until a round found no defect, is `docs/decisions/decision_substitutability-relation.md`, and its code is
`crates/eadl-resolve`, held to the design's executable model (`crates/archogen-plan-check/src/model/`, with its checker in
`crates/eadl-resolve/tests/checker.rs` and its corpus in `crates/eadl-resolve/tests/corpus.rs`). `archogen check` does
not call it yet.
