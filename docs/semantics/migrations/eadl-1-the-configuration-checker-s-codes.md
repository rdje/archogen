# The configuration checker's codes are stated in §4, its check source declared

- version: eadl/1
- date: 2026-10-26
- leaf: M3.5.4.5 (`docs/tasks/M3.md`)
- status: pending
- constructs: docs/semantics/reference.md#diagnostics, docs/semantics/reference.md#normative-sources
- invalidates: none — no description changes meaning; the reference states the codes of a new door into the toolchain, a plan checked against its description

## What changed

§4's diagnostics table gained the configuration checker's codes: `plan-record`, `plan-stale`,
`plan-description`, `plan-binding` and `plan-unsatisfied`, each with when it fires, what to do and an input
that fires it. The input is written with a new verb, `plan T <plan> R`: a description, the module `app`,
and an assignment record checked against it by the checker's checks in order, `<digest>` standing for the
elaborated program's digest. `crates/archogen-cli/tests/fires_on.rs` runs each one.

The normative-sources table gained `crates/archogen-plan-check/src/check.rs`, which governs every code
beginning `plan-`. The checker names each code once, as a constant its findings carry, so the census in
`crates/eadl-front/tests/reference.rs` now counts a declared source's constant whose value begins with a
prefix the declaration governs.

## Why

`M3.5`'s design puts a code per check in §4's table, each with an input that fires it, so a check whose
code no plan can reach any more fails the build as a surface rule does. The plan's record,
`archogen-assignment/1`, is an eADL document read against a description, and its verdicts are what an
author meets when a plan does not read or does not match.

## Which descriptions it invalidates

None. No rule of the language changed, and every description reads and checks as it did. The rows state
what a separate program, the configuration checker, says of a plan.

## Which version it lands in

`eadl/1`, as an addition to its specification. It is not a new language version.
