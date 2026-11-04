# A task's durations are times

- version: eadl/1
- date: 2026-11-04
- leaf: M1.42 (`docs/tasks/M1.md`)
- status: pending
- constructs: docs/semantics/model.md#4-a-profile-admits-a-workload-or-says-what-it-would-take
- invalidates: a description whose task writes a `period`, `min-separation`, `deadline` or `jitter` that is not a time, or a `period` or `min-separation` of zero; the tracked ones are named below

## What changed

A task's four durations held any quantity, so `(period 100 Hz)` and `(deadline 10 kHz)` passed `archogen check` and the configuration
checker. So did `(period 0 ms)`. Each is now `invalid-description`, naming the clause.

## Why

No stage after the check could read them. The runtime plan's lowering refused each as `tool-failure` meanwhile (`RTP-H14`), the
analysis refuses a zero separation (its condition 3), and a deadline of a frequency beside a period could be compared with nothing,
so it went unjudged.

## Which descriptions it invalidates

One whose task writes such a clause. The repair is to write a time, and one above zero for what separates releases. The frozen verdicts
that move are in `crates/archogen-cli/tests/verdicts.txt`, each from `0 -` to `10 invalid-description`, and each is the case this rule
refuses, written to reach the refusal the lowering gave it until now.

## Which version it lands in

`eadl/1`, as a correction of its specification. It is not a new language version.
