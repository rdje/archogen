# Quantities and units

## The idea, in plain words

"Wait 10" means nothing until you say 10 what: seconds, milliseconds, ticks of a clock. In eADL a number that
measures something carries its unit — `10 ms`, `10 MHz`, `64 KiB` — and the unit is part of what it means. archogen
checks units before it does any arithmetic, so a period measured in parsecs, a clock running at zero hertz, or a
comparison of a time with an amount of memory is refused with a message saying what is wrong, instead of producing
a number that looks right.

It never rounds quietly either. Every amount is held as an exact fraction, because a timing analysis adds its own
results up again and again, and small rounding errors grow into a wrong answer that still looks like a number. And
every comparison says which way is better: a longer time horizon is better, a longer delay is worse, and some
values must simply match.

> **In one minute, for engineers.** A quantity is the shape `<number> <symbol>`, its unit from a closed table; a
> lone number is a count. Magnitudes are exact rationals, `i128` over `i128`, normalized, with checked operations —
> overflow yields no result, never a wrapped one — exact comparison (`crates/eadl-model/src/rational.rs`) and upward
> rounding where the analysis needs it; there is no floating point anywhere. F03 refuses a non-positive frequency
> and incompatible units as type errors before arithmetic, the dimension checked before either magnitude is read.
> Every comparison declares `at-least`, `at-most` or `exact`, with no default.

## How it works

A number in an eADL description carries a unit whenever it *measures* something, and the unit is part of
its meaning.

⛔ **The rule is a shape, and the shape is `<number> <symbol>`.** `(tick-rate 10 MHz)` is a quantity, so
`MHz` must be a unit the table holds. `(counter-modulus 4294967296)` is a **count** — a lone number, which
is nobody's quantity and is accepted, and which
`docs/semantics/boundary/accept/counter-width-and-rate.eadl` ships as a case with verdict `accept`. This
chapter used to say "a number always carries a unit" and showed the refusal that would follow, and the
sentence was false for that accepted case: the reason nothing noticed is that every pass which read a
quantity **discarded** what it found, so no refusal ever reached an author. Leaf `M1.28` measured both
halves of that.

```text
(tick-rate 10 MHz)
(unambiguous-horizon (at-least 60 s))
(delivery-bound (at-most 50 us))
(size 64 KiB)
```

## The precise rules

### Exact, always

`ROADMAP.md` §7.4 requires "exact integer time units or checked rational arithmetic, upward
rounding where needed, overflow detection". So every magnitude is an exact rational — `i128`
over `i128`, normalized — and there is no floating point anywhere in the toolchain.

This is not fastidiousness. A response-time recurrence accumulates its own result: rounding
error compounds, and the answer still looks like a number. `1/3 + 1/3 + 1/3` is exactly `1`
here, and `1 ns` converted to milliseconds is exactly `1/1000000`, not zero.

Values that have no short decimal form are printed as fractions rather than rounded into
something that reads like a measurement:

```text
1/3        not 0.333333
22/7       not 3.142857
0.125      exact, so written as a decimal
```

A value whose exact decimal would need a power of ten beyond the representation, such as `1/2^100`,
is written as its fraction too. Until leaf `M1.36` the printer multiplied that power of ten
unchecked. It panicked in a debug build and printed a wrong decimal in a release build, and the
fuzz step found it on its first run.

Every operation is checked. Overflow returns "no result", never a wrapped one — a wrapped
numerator turns an unschedulable system into a schedulable-looking one, and nothing in the
output would say so.

**Comparison is exact too, and until leaf `M1.34` it was not.** Two fractions are ordered by
multiplying across, and that product can pass `i128` long before either value does. It used to be
saturated to the largest `i128`, and that went wrong when *both* products saturated: two different
amounts then compared as equal. A task whose deadline exceeded its period in the 17th decimal place,
`(deadline 1.00000000000000003 ns)` against `(period 1.00000000000000001 ns)`, was admitted under a
profile that requires deadline ≤ period. The products are now computed exactly, in 256 bits
(`crates/eadl-model/src/rational.rs`), and that description is refused like any other.

Rounding, where it is needed, goes **up**: analysis needs `⌈R/T⌉`, and rounding the other way
understates interference, which turns a missed deadline into a reported pass.

### The unit table is small on purpose

| Dimension | Units |
| --- | --- |
| time | `s` `ms` `us` `ns` |
| frequency | `Hz` `kHz` `MHz` `GHz` |
| information | `bit` `byte` `KiB` `MiB` |
| dimensionless | `tick` |

A table that accepts arbitrary SI prefixes accepts `Ps` and `mHz` too, and a typo that parses
is worse than one that does not. Binary prefixes only for storage: a `KiB` is 1024 bytes, and a
"KB" is an argument nobody needs to have again.

### F03: refusal before arithmetic

`ROADMAP.md` §13.1 requires two things to fail as a **type or constraint error before arithmetic**:

> Zero clock frequency or incompatible units → type/constraint error before arithmetic

"Before arithmetic" is a structural property here, not a matter of ordering statements carefully. The
dimension check happens before either magnitude is read, and the test demonstrates it rather than
asserting it: two quantities whose magnitudes would *overflow* if touched still produce the dimension
error. Every transcript below is a real run of `archogen check` over a tracked file in the conformance
suite, so a reader can re-run it and get these bytes.

**A zero (or negative) clock frequency.**

```console
$ archogen check docs/semantics/cases/invalid-zero-clock-frequency.eadl
error[quantity-non-positive-frequency]: a frequency must be strictly positive
  --> docs/semantics/cases/invalid-zero-clock-frequency.eadl:9:44
  |
9 | (defblock timer.counter (offers (tick-rate 0 MHz)))
  |                                            ^^^^^ this frequency is not positive
  = hint: §6.2 requires a positive clock frequency: every conversion from ticks to time divides by it, so zero makes the platform's time contract undefined rather than merely wrong
archogen: invalid-description: 1 diagnostic(s) in docs/semantics/cases/invalid-zero-clock-frequency.eadl
```

It is refused **at construction**, so there is no zero-frequency quantity in existence for anything to
divide by later — and it is refused at the *surface* too, which is the half leaf `M1.28` added: before
that, the passes which read quantities discarded what they found, so this description was accepted by
`archogen check` and the zero was only ever seen by whatever tried to use it.

**A unit the table does not hold.**

```console
$ archogen check docs/semantics/cases/invalid-unknown-unit.eadl
error[quantity-unknown-unit]: `parsec` is not a known unit
  --> docs/semantics/cases/invalid-unknown-unit.eadl:9:33
  |
9 | (defsystem s (task t (period 10 parsec) (deadline 10 ms) (priority 1)))
  |                                 ^^^^^^ unknown unit
  = hint: the known units are `s`, `ms`, `us`, `ns`, `Hz`, `kHz`, `MHz`, `GHz`, `bit`, `byte`, `KiB`, `MiB`, `tick`
archogen: invalid-description: 1 diagnostic(s) in docs/semantics/cases/invalid-unknown-unit.eadl
```

The refusal names the whole table rather than saying "wrong", because the cheapest diagnostic to write is
the most expensive to receive. This one is the **schema**'s: `period` is declared
`(holds values quantity)` in `docs/semantics/kinds/os-rt.eadl`, and a quantity is the one value type that
consumes two forms, so a clause can declare that it holds a measurement rather than merely a number and a
symbol. `(holds values number symbol)` said nothing about whether the pair measured anything — `parsec` is
a perfectly good symbol — which is how `archogen check` came to accept this description while
`archogen build` refused it.

**A bare number where a bound was meant.**

```console
$ archogen check docs/semantics/cases/invalid-quantity-without-unit.eadl
error[quantity-missing-unit]: this number has no unit
  --> docs/semantics/cases/invalid-quantity-without-unit.eadl:9:55
  |
9 | (defplatform soc.abstract (offers (tick-rate (exactly 10))))
  |                                                       ^^ a bare number is not a quantity
  = hint: write the unit after the number, e.g. `10 ms` — a bare number cannot be compared with anything, because nothing says what it measures
archogen: invalid-description: 1 diagnostic(s) in docs/semantics/cases/invalid-quantity-without-unit.eadl
```

**Comparing things that measure different things.** A cross-dimension comparison is refused without
reading either magnitude, and where a refinement is the thing comparing them the author sees it as a
violated `constraint` obligation ([Refinement](refinement.md) shows the whole rendering):

```text
`ms` measures time and `bit` measures information — they cannot be compared
```

### More is not better

§5.2 warns that "more bits or a faster clock is not universally better", so every comparison
carries a **declared direction**, and there is no default:

| Direction | Satisfied when | Example |
| --- | --- | --- |
| `at-least` | the offer is greater or equal | an unambiguous-time horizon |
| `at-most` | the offer is less or equal | a delivery bound |
| `exact` | the offer is equal | a tick unit, a counter modulus |

A 60 s horizon satisfies a 30 s requirement. A 60 µs delivery bound does **not** satisfy a 30 µs
one. And an `exact` parameter is not satisfied by a "better" value at all — a 20 MHz tick rate
does not satisfy a description that needs 10 MHz, though `10000000 Hz` does, because the
comparison is on the amount and not the spelling.

A comparison with no declared direction is a bug waiting for a substitution to expose it, which
is why the type has no default to fall back on.
