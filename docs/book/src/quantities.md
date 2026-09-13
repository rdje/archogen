# Quantities and units

A number in an eADL description always carries a unit, and the unit is part of its meaning.

```text
(tick-rate 10 MHz)
(unambiguous-horizon (at-least 60 s))
(delivery-bound (at-most 50 us))
(size 64 KiB)
```

## Exact, always

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

Every operation is checked. Overflow returns "no result", never a wrapped one — a wrapped
numerator turns an unschedulable system into a schedulable-looking one, and nothing in the
output would say so.

Rounding, where it is needed, goes **up**: analysis needs `⌈R/T⌉`, and rounding the other way
understates interference, which turns a missed deadline into a reported pass.

## The unit table is small on purpose

| Dimension | Units |
| --- | --- |
| time | `s` `ms` `us` `ns` |
| frequency | `Hz` `kHz` `MHz` `GHz` |
| information | `bit` `byte` `KiB` `MiB` |
| dimensionless | `tick` |

A table that accepts arbitrary SI prefixes accepts `Ps` and `mHz` too, and a typo that parses
is worse than one that does not. Binary prefixes only for storage: a `KiB` is 1024 bytes, and a
"KB" is an argument nobody needs to have again.

## F03: refusal before arithmetic

Two things must fail, and §13.1 requires them to fail as a **type or constraint error before
arithmetic**:

**A zero (or negative) clock frequency.**

```text
error[quantity-non-positive-frequency]: a frequency must be strictly positive
  --> platform.eadl:3:16
  |
3 |   (tick-rate 0 MHz)
  |              ^^^^^^ this frequency is not positive
  = hint: §6.2 requires a positive clock frequency: every conversion from ticks to time divides
          by it, so zero makes the platform's time contract undefined rather than merely wrong
```

It is refused **at construction**, so there is no zero-frequency quantity in existence for
anything to divide by later.

**Comparing things that measure different things.**

```text
`ms` measures time and `KiB` measures information — they cannot be compared
```

"Before arithmetic" is a structural property here, not a matter of ordering statements
carefully. The dimension check happens before either magnitude is read, and the test
demonstrates it rather than asserting it: two quantities whose magnitudes would *overflow* if
touched still produce the dimension error.

A bare number is not a quantity either:

```text
error[quantity-missing-unit]: this number has no unit
  = hint: write the unit after the number, e.g. `10 ms` — a bare number cannot be compared with
          anything, because nothing says what it measures
```

## More is not better

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
