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
> declared absent is `infeasible-configuration`; offered and absent together is `invalid-description`, reachable
> or not. F04, F05 and F06 run against one description, so neither blocking on every unknown nor on none passes.
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

### Contradictions are rejected, never resolved

```text
error[invalid-description]: `low-power-timer` is declared both offered and absent
  --> t.eadl:5:9
  |
5 |         (defblock timer.b (absent low-power-timer))
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ declared absent by `timer.b`
  --> t.eadl:4:9
  |
4 |         (defblock timer.a (offers low-power-timer))
  |         ------------------------------------------ declared offered by `timer.a`
  = hint: remove one declaration — a contradiction is not resolved by preferring one side,
          because only the author knows which half was meant
```

Both sites are named, because the author looking at one of them cannot see the other.

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
stays admissible) and **F06** (offered and absent together → `invalid-description`) are the two
halves of §5.3's single decision, in `crates/eadl-model/tests/f04_f06_presence.rs`, with worked
cases in `docs/semantics/cases/`.

### What comes after presence: substitutability, in design

Presence asks whether a fact is known. The next question is whether what a provider offers is *good enough* for
what a requirement asks — whether a 64-bit counter satisfies a service that needs 32 bits, or an idle-only timer
one that must keep running. That relation is still a design under review
(`docs/decisions/decision_substitutability-relation.md`), and its rules are held to an executable model rather than
to prose alone: `crates/eadl-resolve/src/model/` transcribes the design rule by rule, an exhaustive checker
(`crates/eadl-resolve/tests/checker.rs`) asserts on thousands of inputs that no value is read the wrong way and that
no stronger precondition passes as a capability, and every probe its reviewers ran is a permanent fixture
(`crates/eadl-resolve/tests/corpus.rs`). `archogen check` does not call it yet; the chapter on the relation itself
comes with the relation (leaf `M3.1.3`).
