# uc3's timer states the count it wraps at

- version: eadl/1
- date: 2026-10-27
- leaf: M3.4.7.1 (`docs/tasks/M3.md`)
- status: applied
- constructs: suite/examples/alternative-timer/system.eadl
- invalidates: none, measured — `archogen check` moves from `11 missing-fact`, the repair it named, to `0`, accepted, frozen in
  `crates/archogen-cli/tests/verdicts.txt`; `archogen resolve` then refuses `missing-fact` for its policy's three
  constraints alone, the catalog's (`M3.7`)

## What changed

In `examples/alternative-timer/system.eadl`, use case uc3, the block `timer.delay` gains
`(counter-modulus 4294967296)` beside its `(counter-width 32 bit)`: the count its 32-bit counter wraps at.

## Why

The engine's adapter `deadline-over-delay` realizes `time.deadline`'s absolute deadline over a relative delay, and needs
its timer's modulus beneath (`docs/specs/adapters/decision_engine-adapters.md` §2, §8): its supported horizon is the
timer's unambiguous horizon, `(modulus − 1) / rate`. The vocabulary makes the modulus a fact a description states,
"offered, never derived", since a counter may wrap below its register's width; a modulus beside a width "may not exceed
two to the width" (`docs/semantics/vocabulary/vocabulary.eadl`). `2^32` is the modulus of a 32-bit counter running
through its whole width, the reading this synthetic timer's description gives, and the largest the language admits
beside it. `wrap-behavior` stays undescribed, which the rule reads as `modular` (the relation record §3.1,
`horizon-from-modulus-and-rate`). At `1 MHz` the horizon is `4294967295 / 1 000 000 s = 4294.967295 s`, past
`time.deadline`'s `10 s` and `time.monotonic`'s `60 s`. Before this change, `archogen check` named this fact as the one
whose description would open the adapter's path (`M3.2.4`).

uc3's rule is that its description must not change *to build*: weakening `absolute-deadline` into `relative-delay` is
the failure §17's first stop/rework trigger names. This change, like `eadl-1-uc3-states-its-level.md` before it, states
a fact true of the hardware and moves no requirement and no absence: `absolute-deadline` is still required and still
declared absent at `timer.delay`, which `crates/eadl-model/tests/examples.rs` asserts.

`SR-H10` brought this change to the director before it was made. On `2026-10-27` the director delegated it: "is yours to
take, but it better be sota, signoff and production-ready". It was decided on the measurements above:
- a hardware fact, not a requirement;
- the canonical modulus of the width the description states;
- the smallest change, the one fact the refusal named;
- the gap the case exists to expose kept.

## What it affects

- `crates/archogen-cli/tests/verdicts.txt`: `examples/alternative-timer/system.eadl` from `11 missing-fact` to `0 -`, accepted.
- `docs/semantics/BASELINE.txt`: the description's canonical form, re-emitted.
- `examples/README.md`, `docs/usecases/uc3-alternative-timer.md`, the book's checking and matching chapters, the
  relation record's `SR-H10` row and the adapters record's §8: the verdict, and the change as made.

## What a description author does

Nothing: no construct's meaning moved. A description whose counter wraps below its register's width states that
modulus; one running through its width states two to the width.
