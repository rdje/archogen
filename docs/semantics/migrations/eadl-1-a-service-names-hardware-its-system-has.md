# A service or a policy names hardware its system has, or the checker refuses the description

- version: eadl/1
- date: 2026-11-04
- leaf: M1.49 (`docs/tasks/M1.md`)
- status: pending
- constructs: docs/semantics/reference.md#diagnostics
- invalidates: a description a system of which serves a service or a policy naming a block or platform outside that system's hardware; none tracked is refused for it but the runtime plan's own corpus

## What changed

§4's `plan-description` row gained `docs/semantics/model.md` §8 rule 7: a service or a policy the plan's system serves
that names, by `uses` or `needs`, a block or platform outside that system's hardware. The rule used to give such a name
no meaning: "a block that only a service names is not hardware" kept the block out, and nothing said what the name then
was. It is now a conflict of each system that serves the namer, `infeasible-configuration`, whatever the assignment.

## Why

An operand that means something to some rules and nothing to others is refused. The name could not be a path into the
hardware without undoing the sentence that keeps it out, so it is a conflict, of each serving system alone, so that a
system whose platform does compose the block is not refused for it (the runtime plan's round 21, R21-R4; `RTP-H22`).

## Which descriptions it invalidates

A description whose system serves such a name, which `archogen resolve` now refuses, naming the namer, the hardware and
the system. The repair is to compose the hardware into the system's platform, or to have the namer name hardware the
platform composes. `archogen check`, which judges no system's hardware, accepts it as before. No frozen verdict moves:
`crates/archogen-cli/tests/verdicts.txt` is unchanged, and of the tracked descriptions only five of the runtime plan's
own corpus reach it, each already refused there.

## Which version it lands in

`eadl/1`, as an addition to its specification. It is not a new language version.
