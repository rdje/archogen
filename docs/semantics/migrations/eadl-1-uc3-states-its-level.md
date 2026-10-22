# uc3's hardware states the level it is reachable from

- version: eadl/1
- date: 2026-10-22
- leaf: M3.3.4.5 (`docs/tasks/M3.md`)
- status: applied
- constructs: suite/examples/alternative-timer/system.eadl
- invalidates: none, measured — `archogen check` still answers `13 infeasible-configuration` with the same diagnostic,
  frozen in `crates/archogen-cli/tests/verdicts.txt`; its two locations move one line down, the timer's offers having
  gained a line

## What changed

In `examples/alternative-timer/system.eadl`, use case uc3, each block and platform of the system's hardware —
`timer.delay`, `console.uart` and `soc.delay-only` — gains `(reachable-at-privilege machine)` among its offers. It was
the one suite description `eadl-1-providers-state-their-level.md` left out, its description then changed only with the
director's yes (`SR-H10`, findings §15); `crates/eadl-resolve/tests/privilege_census.rs` now holds the rule over the
whole suite, its exclusion list empty.

## Why

`SR-H12` refuses a provider a plan binds that states no level, and `rt-static-up-v1` runs every caller at `machine` (its
Privilege row, `M3.3.4.2`). Left without one, uc3 would be the one suite example refused once `M3.4` wires the joint
rules into `archogen check`, for a fact true of its hardware rather than for the gap the case exists to expose. Decided
under the director's delegation of `2026-10-22` (findings §15, (a)).

uc3's rule is that its description must not change *to build*: weakening `absolute-deadline` into `relative-delay` is
the failure §17's first stop/rework trigger names. This change moves no requirement and no absence — `absolute-deadline`
is still required and still declared absent, which `crates/eadl-model/tests/examples.rs` asserts — and it lands before
`M3.2`, so the description `M3.2` must build is the one this commit leaves, and any later movement of it is this gate's
to refuse without a note.

## Which descriptions it invalidates

None. The level is an offer no requirement of uc3 reads; the joint rule that reads it is not called by any command yet.

## Which version it lands in

`eadl/1`: no construct of the language changed; a description gained an offer of an existing vocabulary entry.
