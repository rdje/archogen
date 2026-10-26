# The configuration checker's joint check states its code in §4

- version: eadl/1
- date: 2026-10-26
- leaf: M3.5.5.1 (`docs/tasks/M3.md`)
- status: pending
- constructs: docs/semantics/reference.md#diagnostics
- invalidates: none — no description changes meaning; the reference states the code of the checker's check (5)

## What changed

§4's diagnostics table gained `plan-joint`, the code of the configuration checker's check (5): the bindings of one
system's assignment judged together by the joint rules of `docs/semantics/model.md` §8. Its first rule is rule 7, a
requester the system serves bound to a block or platform outside the system's hardware, and its `fires on` input binds
a console to a serial port the system's platform does not compose. The row names the rules as those the check runs,
so it stays true as `M3.5.5`'s children add theirs.

## Why

`M3.5`'s design puts a code per check in §4's table, each with an input that fires it, and the census holds the
table to the constants `crates/archogen-plan-check/src/check.rs` declares: the check's code lands with the check.

## Which descriptions it invalidates

None. No rule of the language changed, and every description reads and checks as it did.

## Which version it lands in

`eadl/1`, as an addition to its specification. It is not a new language version.
