# Refinement

A concrete description can claim to satisfy an abstract one:

```text
(defplatform soc.abstract
  (offers (counter-width (at-least 32 bit)))
  (absent dma))

(defplatform soc.concrete
  (refines soc.abstract)
  (offers (counter-width 64 bit) uart spi))
```

`ROADMAP.md` §5.1.1 is precise about what that claim is worth:

> A refinement declaration is an **obligation to check**, not permission to trust a claim
> blindly.

## Three obligations, and a violation names which

| Obligation | Violated when |
| --- | --- |
| **guarantee** | the abstract offers a fact and the concrete does not |
| **constraint** | the concrete's value does not satisfy the abstract's stated bound |
| **exclusion** | the abstract declares a fact absent and the concrete offers it |

"This refinement is invalid" sends an author to re-read two descriptions and guess. This does
not:

```text
error[refinement-violated]: `soc.concrete` does not offer `wrap-behavior`, which `soc.abstract` guarantees
  |
  | this refinement drops a guarantee
  |
  | guaranteed here
  = hint: violated obligation `guarantee`: a refinement keeps every guarantee the abstract
          description offers
```

Every violation is reported, not just the first.

## The unused device

§5.3 adds the rule that keeps the check from being useless:

> Platform refinement must preserve the obligations actually relied upon by the profile,
> including relevant negative constraints; **adding an unused device is not automatically an
> invalid refinement**.

So `uart` and `spi` above are **additions**: reported, never refused. A checker that demanded
equality would reject every real refinement — saying more is the entire point of refining.

Allowed is not the same as invisible. Additions appear in the report, because the author should
be able to see what grew.

## Why an exclusion is not an omission

The two rules above pull against each other, and the exclusion obligation is where they meet. A
checker that only compared what both descriptions mention would miss the case that matters most:

```text
error[refinement-violated]: `soc.concrete` offers `dma`, which `soc.abstract` declares absent
  = hint: violated obligation `exclusion`: a refinement does not offer what the abstract
          description declares absent — an explicit absence is a constraint something relies on,
          not an omission to be filled in
```

An abstract description declares a fact absent *because something depends on its absence*. A
refinement that quietly adds it has changed what the abstract description meant, while still
looking like it says strictly more.

## Why a bound carries a direction

An abstract description writes `(counter-width (at-least 32 bit))`, not `(counter-width 32
bit)`. §5.2: *"More bits or a faster clock is not universally better."*

| Bound | 64 bit | 16 bit |
| --- | --- | --- |
| `(at-least 32 bit)` | satisfies | violates |

| Bound | 80 µs | 20 µs |
| --- | --- | --- |
| `(at-most 50 us)` | violates | satisfies |

The same numeric relationship passes one and fails the other. A bare value would leave the
checker guessing, and guessing right for a counter width while guessing wrong for a delivery
bound.

`exactly` refuses even a "better" value — a tick rate of 20 MHz does not satisfy a description
that needs 10 MHz. It does accept `10000000 Hz`, because the comparison is on the amount and not
the spelling.

And a bound checked against the wrong dimension is a **type error**, not a `false`:

```text
`horizon` cannot be checked against its bound: `MHz` measures frequency and `s` measures time —
they cannot be compared
```
