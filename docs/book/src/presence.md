# Presence, absence, and relevance

A description says one of three things about any fact:

| State | Meaning |
| --- | --- |
| **offered** | the description claims it — a *claim*, not evidence |
| **absent** | the description states it is not available — a negative fact is a fact |
| **undescribed** | nothing says |

The third is not the second. That difference decides which diagnostic an author gets, and
therefore what they go and do.

## Relevance decides whether an unknown matters

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

`dma-channels` is outside it, produces **no** diagnostic, and appears in the report's metadata
because §5.3 requires it to stay visible.

The fixtures F04 and F05 are deliberately run against the *same* description. A checker that
blocks on every unknown passes F04 and fails F05; one that blocks on nothing passes F05 and
fails F04. Only relevance passes both — and using two different descriptions would have let two
different bugs pass.

## Absent is a definite answer

```text
error[infeasible-configuration]: `low-power-timer` is required by this system but declared absent
  = hint: either the requirement or the platform is wrong; an explicitly absent fact is a
          definite answer, not a gap to be filled in
```

Reporting this as `missing-fact` would send the author off to describe something the platform
has already said it does not have.

## Contradictions are rejected, never resolved

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

## Offered is a claim, not evidence

> "Offered" records a claim whose evidential status is separate; declaring it does not make it
> proven.

Nothing in this analysis says a platform *really* has a capability. Presence is what the
description asserts; whether that assertion is backed lives in the evidence vocabulary, with its
own category and its own scope.

## What this is not

This is presence and relevance analysis, not provider resolution. It answers "is every fact this
system depends on actually known?" It does not choose implementations, allocate resources, or
check capacity — that is the joint resolver, and conflating the two here would produce a
resolver nobody reviewed.
