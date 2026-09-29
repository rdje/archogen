# `quantity` joins the value types, so a clause can say that it holds a measurement

- version: eadl/1
- date: 2026-09-29
- leaf: M1.28.2 (`docs/tasks/M1.md`)
- status: applied
- constructs: docs/semantics/reference.md#diagnostics, suite/docs/semantics/kinds/os-rt.eadl, suite/docs/semantics/cases/invalid-zero-clock-frequency.eadl, suite/docs/semantics/cases/invalid-quantity-without-unit.eadl, suite/docs/semantics/cases/invalid-unknown-unit.eadl
- invalidates: none in this repository, measured; outside it, a description whose task clause carries a unit the table does not hold, or whose offered `<number> <symbol>` fact is not a readable quantity

## What changed

`ValueType` gained one member, **`quantity`**, and it is the only value type that consumes **two**
values — a number and a unit. A kind clause may now declare `(holds values quantity)`, and the schema
refuses a pair that is not a quantity by propagating `crates/eadl-model/src/quantity.rs`'s **own**
diagnostic rather than inventing a second code for the same mistake.

Four clauses of the `task` kind moved onto it, from `(holds values number symbol)`:

| clause | was | now |
| --- | --- | --- |
| `period` | `(holds values number symbol)` | `(holds values quantity)` |
| `min-separation` | `(holds values number symbol)` | `(holds values quantity)` |
| `deadline` | `(holds values number symbol)` | `(holds values quantity)` |
| `jitter` | `(holds values number symbol)` | `(holds values quantity)` |

`docs/semantics/reference.md` §4's `schema-bad-value-type` row and §7 rule 4 both enumerate the value
types, so both name `quantity` now; the row is a frozen construct and is why this note exists. §7 rule 4
also states the width rule, because a reader of the enumeration cannot otherwise tell that one type takes
two values.

`crates/eadl-model/src/refinement.rs` stopped **discarding** what `Quantity::read` refused, so a quantity
written inside an `(offers …)` clause — which `docs/semantics/kinds/core.eadl` is explicit the schema does
not interpret — now reaches the author too. The shape it reads is `<number> <symbol>`; a lone number is a
count and is nobody's quantity.

## Why

`archogen check` accepted a description `archogen build` could not realize. Measured on the parent commit:

```console
$ archogen check <system.eadl with `(period 10 parsec)`>
<…>: accepted against profile `rt-static-up-v1` (4 declaration(s))
$ archogen build <the same bytes>
error[quantity-unknown-unit]: `parsec` is not a known unit
```

`(holds values number symbol)` is the reason: a symbol is a *name*, and `parsec` is a perfectly good one,
so the declaration said nothing about whether the pair measured anything. And a zero clock frequency —
`ROADMAP.md` §13.1's **F03**, an M1 gate fixture — drew no diagnostic from `check` at all, because the
three consumers of `Quantity::read` that could have reported one discarded it. F03 was verified at the
type, one level below where the fixture gate reads. Leaf `M1.28`.

## Which descriptions it invalidates

**None in this repository, and that is measured rather than assumed.** Every tracked description was
driven through `archogen check` at the parent commit and at this one, and the two runs differ on exactly
one file:

- `docs/semantics/cases/invalid-zero-clock-frequency.eadl` — **rewritten here on purpose**. It put
  `(refines …)` on a `defblock`, which is `defplatform`'s clause, so it was refused as
  `schema-unknown-clause` and collected its expected `invalid-description` without ever reaching a zero
  frequency. It is now one `defblock` offering `(tick-rate 0 MHz)` and is refused for that, which is what
  its header always claimed.

Two cases join the suite, because a new rule with no worked case is a rule nothing in the suite exercises:
`invalid-unknown-unit.eadl` (a task period whose unit the table does not hold) and
`invalid-quantity-without-unit.eadl` (a stated bound with a bare number).

⚠️ **One accepted case is the reason the rule is a shape and not "every value is a quantity".**
`docs/semantics/boundary/accept/counter-width-and-rate.eadl`, verdict `accept`, holds
`(counter-modulus 4294967296)` — a count. Reading every offered value as a quantity turned that accepted
case into `invalid-description`, and `crates/eadl-model/tests/f27_boundary.rs` now runs the whole pipeline
over the accept corpus so the guard cannot be lost again.

Outside this repository, a description is invalidated when a `period`, `min-separation`, `deadline` or
`jitter` carries a unit the table does not hold or a magnitude the domain refuses, or when an offered
`<number> <symbol>` fact does. Each such refusal names the unit table or the domain rule and says what to
write instead, and each was previously either silently accepted or reported only by `archogen build`.

## Which version it lands in

`eadl/1`. The language has not been released, so this is a tightening of the first version rather than a
compatibility break against a shipped one — but it *is* a change in what a description means, which is
exactly what §15 of `ROADMAP.md` requires to be explicit, and the frozen baseline is what makes "explicit"
a gate rather than an intention.
