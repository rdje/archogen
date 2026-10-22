# Every block and platform a suite system has states the level it is reachable from

- version: eadl/1
- date: 2026-10-22
- leaf: M3.3.4.3 (`docs/tasks/M3.md`)
- status: applied
- constructs: suite/docs/semantics/cases/infeasible-required-absence.eadl, suite/docs/semantics/cases/invalid-two-release-models.eadl, suite/docs/semantics/cases/missing-release-model.eadl, suite/docs/semantics/cases/missing-relevant-fact.eadl, suite/docs/semantics/cases/positive-decimal-quantity.eadl, suite/docs/semantics/cases/positive-explicit-absence-unused.eadl, suite/docs/semantics/cases/positive-minimal-system.eadl, suite/docs/semantics/cases/positive-skip-late-job.eadl, suite/docs/semantics/cases/positive-sporadic-release.eadl, suite/docs/semantics/cases/positive-unrelated-unknown.eadl, suite/docs/semantics/cases/unsupported-arbitrary-deadline.eadl, suite/docs/semantics/cases/unsupported-duplicate-priority.eadl, suite/docs/semantics/cases/unsupported-general-ipc.eadl, suite/docs/semantics/cases/unsupported-precedence-over-missing-fact.eadl, suite/examples/bounded-queue/system.eadl, suite/examples/high-interference/system.eadl, suite/examples/periodic-three/system.eadl, suite/examples/s0-heartbeat/system-changed.eadl, suite/examples/s0-heartbeat/system-non-harmonic.eadl, suite/examples/s0-heartbeat/system-unsupported.eadl, suite/examples/s0-heartbeat/system.eadl
- invalidates: none, measured — every tracked description's `archogen check` verdict, frozen in `crates/archogen-cli/tests/verdicts.txt`, is unchanged; the descriptions gain an offer, and no requirement moves

## What changed

Each block and platform in the hardware of each suite system — its platform, and what that names through `uses` and
`needs` — gains `(reachable-at-privilege machine)` among its offers: 45 providers in these 21 descriptions, as of
`2026-10-22`. `examples/alternative-timer` is the one exception, its description changed only with the director's yes
(`SR-H10`, findings §15, `M3.3.4.5`). `crates/eadl-resolve/tests/privilege_census.rs` holds the rule over the suite.

## Why

`SR-H12` refuses a provider a plan binds that states no level, and `rt-static-up-v1` runs every caller at `machine` (its
Privilege row, `M3.3.4.2`). Without a level, each of these descriptions would be refused the day `M3.4` wires the joint
rules into `archogen check`, though nothing in it is wrong; stating the level now keeps every frozen verdict as it is,
which `SR-H20` requires of the wiring.

## Which descriptions it invalidates

None. The level is an offer, which no requirement of these descriptions reads; the joint rule that does is not called
by any command yet. Presence's closure report lists the new fact among those "needed by nothing requested", as it lists
`core-count`: which closure the report carries is `SR-H15`'s, `M3.4`'s to decide.

## Which version it lands in

`eadl/1`: no construct of the language changed; descriptions gained an offer of an existing vocabulary entry.
