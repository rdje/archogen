# The S0 corpus: `heartbeat`

The three descriptions and the frozen observations of `ROADMAP.md` §12 S0 / fixture **F28** —
the early executable generation path. Owned by task-tree [`S0`](../../docs/tasks/S0.md).

| File | Case | `archogen check` | `archogen build` |
| --- | --- | --- | --- |
| [`system.eadl`](system.eadl) | base | `0` accepted | `0` + the observation in [`expected/system.txt`](expected/system.txt) |
| [`system-changed.eadl`](system-changed.eadl) | one parameter changed | `0` accepted | `0` + the observation in [`expected/system-changed.txt`](expected/system-changed.txt) |
| [`system-unsupported.eadl`](system-unsupported.eadl) | no engine realization | `0` accepted | `12` refusal, per [`expected/system-unsupported.txt`](expected/system-unsupported.txt) |

Every number in the right-hand column is **frozen**, and was written before any emitter existed.
That ordering is the whole point of the corpus, so it is a fact of the commit history rather
than a claim: this directory landed in leaf `S0.1` and the emitter in `S0.3`, two commits later.
When the emitter was finished, both runnable fixtures produced output **byte-identical** to what
had been frozen — compared with `diff`, not by eye.

## The observation contract

What a generated S0 system does is the **engine's** decision, not the description's. These are
the rules the frozen observations were derived from, stated once so a reader can check every
expected file by hand:

1. **Horizon.** The generated hosted executable simulates modeled time from `0` to the system's
   hyperperiod `H`, the least common multiple of the declared task periods. The hyperperiod is
   the repeating unit of a periodic release schedule, so it is the shortest horizon that shows
   the system's whole behavior — and it is a function of the description, not a run-length knob.
2. **Releases.** A task of period `T` is released at every `t = k·T` with `0 ≤ t < H`.
3. **Order.** Events are emitted by ascending time, and coincident releases by ascending
   priority number — see
   [`decision_priority-comparison-direction`](../../docs/decisions/decision_priority-comparison-direction.md).
4. **Form.** One `system <name>` line, one `release <t> ms <task>` line per release, and a
   closing `summary hyperperiod <H> ms releases <N>` line.
5. **Refusal.** A task that declares `min-separation` instead of `period` has no derivable
   release schedule, and the build is refused with `unsupported-profile` (exit `12`) naming the
   missing engine capability.

⛔ **These are release events, not an execution trace.** A release is a workload fact that
follows from the description; when a task actually *runs* is a scheduling result, and S0
establishes none. §12 S0 is explicit that the path "does not establish catalog reuse, sound
scheduling, or suitability for real hardware".

## How to check an expected file by hand

Take the base case. `beat` has period 10 ms, `chime` 30 ms, so `H = lcm(10, 30) = 30 ms`.
`beat` is released at 0, 10 and 20; `chime` at 0. That is 4 releases, and the two at `t = 0`
come out `beat` first because priority 1 outranks priority 2:

```text
system heartbeat
release 0 ms beat
release 0 ms chime
release 10 ms beat
release 20 ms beat
summary hyperperiod 30 ms releases 4
```

The changed case moves `chime` to a 20 ms period. `H` becomes `lcm(10, 20) = 20 ms`, the 20 ms
`beat` release falls outside the horizon, and the count drops to 3.

## What the oracle asserts, and why it is independent

The oracle is [`crates/archogen-cli/tests/s0_oracle.rs`](../../crates/archogen-cli/tests/s0_oracle.rs).
It re-derives each expected observation from its description, using its own implementation of
the five rules above, and compares the result to the frozen bytes. So an edit to a fixture that
forgets its expected file fails, and an edit to an expected file that does not follow from any
description fails.

Independence is **structural**, not promised:

- The frozen observations predate the emitter in `git log`, so they cannot have been read off it.
- The oracle lives in `tests/`, which Rust cannot link into a library — so no future emitter can
  call it, however convenient that would be. The emitter must be written from the contract
  above.

§4.4 requires shared logic to be disclosed rather than hidden, so: the oracle **does** share the
eADL reader (`eadl-front`) with the toolchain, and shares nothing else. A reader bug that
mis-parses `(period 10 ms)` would mislead both sides identically. That is an accepted, named
dependency — building a second parser to avoid it would contradict §4.1 and would be the more
dangerous kind of green.

## What the corpus cannot see

⛔ **The three cases above share a blind spot, and it was measured rather than guessed.** Their
periods are 10 and 30, then 10 and 20. In both, one period divides the other — the task sets are
**harmonic** — so the hyperperiod equals the longest period. A realization that returned `max`
instead of `lcm` therefore passes every one of them, end to end. Replacing `lcm` with `max` in
`crates/archogen-s0` left all twelve oracle tests green.

Two things close it:

- a unit test in `crates/archogen-s0/src/interpret.rs` on a non-harmonic set, where
  `lcm(10, 15) = 30` and the longest period is `15`; and
- [`system-non-harmonic.eadl`](system-non-harmonic.eadl), which runs the whole path on
  `beat` at 10 ms against `chime` at 15 ms.

That fourth description has **no frozen expectation**, deliberately. The three F28 cases were
frozen before any emitter existed, which is what their independence claim rests on; a file frozen
afterwards would sit beside them carrying a weaker pedigree that a later reader could not tell
apart. Its expectation is instead *derived* by the oracle — an implementation of the contract that
predates the emitter and that the emitter cannot call — and compared to what the generated system
prints. Two independent implementations agreeing is weaker evidence than a literal frozen in
advance, which is the right strength for a case added afterwards.

The general lesson is not about hyperperiods: a fixture set can be complete against its own
specification and blind to a class of error, and a green gate says nothing about which. See
[`a-gate-is-only-as-sharp-as-its-fixtures`](../../docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md).

## S0 assumptions recorded here for retirement

Leaf `S0.6` owns the retirement note. These are the temporary decisions this corpus rests on:

| Assumption | Retired by |
| --- | --- |
| The observation horizon is exactly one hyperperiod | `M4.3` — the harness declares its own observation policy |
| Task periods must be whole milliseconds | `M4.1` — the plan carries exact rational time |
| The event format above is fixed and engine-internal | `M4.3` — the §6.3 observation event set |
| Only periodic releases are realized | `M4.3` — the modeled event source |
| One fixed engine-owned realization, no provider search | `M3` — joint resolution |
