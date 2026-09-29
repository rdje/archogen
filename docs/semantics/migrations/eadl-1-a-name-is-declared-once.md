# A name is declared once

- version: eadl/1
- date: 2026-09-29
- leaf: M1.33 (`docs/tasks/M1.md`)
- status: applied
- constructs: docs/semantics/reference.md#diagnostics, suite/docs/semantics/cases/invalid-duplicate-name.eadl, suite/docs/semantics/modules/bad.name-collision.eadl
- invalidates: none in this repository, measured — every tracked description run through `archogen check` before and after, 118 of them, 0 changed; outside it, any description that declares one name twice, which was accepted and is now `invalid-description`

## What changed

`docs/semantics/reference.md` §7 gained **rule 6: a name is declared once.** Two declarations carrying one
name are refused with both sites named, never merged and never resolved by order. §4 gained the row
`schema-duplicate-name`, the frozen construct that moved, and §6's open-limit paragraph now points at the rule
instead of at an owner. The suite gained `docs/semantics/cases/invalid-duplicate-name.eadl` and
`docs/semantics/modules/bad.name-collision.eadl`.

## Why

A single description declaring `timer.counter` twice, offering 32 and 16 bits, was **accepted**, and the
presence pass kept the first offer — the silent choice §5.3 of `ROADMAP.md` forbids. §6 rule 9 added a
second way to reach it, a local `inner.x` beside an import aliased `inner`.

## Which descriptions it invalidates

A description with a repeated declaration name, which was accepted and meant whichever declaration came
first. Measured: none in this repository.

## Which version it lands in

`eadl/1`, as a correction: no description that meant one thing now means another — one that meant two
things is now refused.
