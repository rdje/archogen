# The hosted simulator's device models: a timer and a controller as exact state machines over a contract

- **Type:** `decision`
- **Date:** `2026-11-05`
- **Status:** `active` — written by leaf `M4.5.1`, with the crate it describes; held by its bounded exhaustive tests, and not yet
  reviewed as a design of its own
- **External sources:** none: this record decides how the engine models what `ROADMAP.md` §6.2 asks a contract to state
- **Owner / source:** leaf `M4.5` (`docs/tasks/M4.md`); `ROADMAP.md` §6.2 and §13.1's F13, F14 and F15.

## The fact / decision

`crates/sim-devices` models a timer and an interrupt controller as state machines over exact integer ticks. Each reads its contract as
data, does no I/O and takes no clock: a caller says what instant it is. So a contract's promise can be run, and every small case of it
enumerated, which is how F13, F14 and F15 are held (`docs/decisions/decision_executable-design-reviews.md`, item 1).

**The timer.** Its contract is the modulus, `counter-modulus` with `wrap-behavior` `modular`, and whether the compare is level or edge.
- A deadline within one below the modulus ahead is armed, and raised at its instant, across a wrap (F13).
- A deadline a whole wrap or more ahead is refused, `AmbiguousHorizon`, and what stood stays armed (F13): its count is one the counter
  reaches sooner, and it is never programmed as that nearer deadline.
- A deadline already passed when the write lands is raised at once, saying how late, and never lost (F14). On a level compare the hardware
  raises it. On an edge compare the hardware would raise nothing until the count came round, so what programs it must read the count again
  after the write and raise it itself: the model says which, since that is what a realization owes.

**The epoch.** `Epoch` extends a modular count to a monotonic one as the `epoch-extender` adapter does, correct while it is read at least
once in every wrap. A gap of a whole wrap cannot be seen from the count alone, which is why the unambiguous horizon is an obligation.

**The controller.** Sources are level or edge, and the lower place is taken first, as the plan's route ranks configure it. One interrupt is
served at a time, none nested and none while masked (F15). A request made while masked or disabled stays pending and is delivered when it
can be. A level line asserted when its service is acknowledged is served again until its device lowers it. An edge request while served is one
more, and two before it is taken are one, which is why `one-request-per-arrival` is a fact a source's record must state.

**Why the playground does not run on these yet.** Its release timer is an exact clock with no counter to wrap, and a `/1` plan names no target
device. Wiring them into a run is `M4.3.6`, once `M4.1.7`'s format `/2` names a target's devices (`M4.4.1`), so that the facts come from the plan
and never from the description (`RTP-H9`). Until then they are held by their contract tests alone, and nothing is claimed of a generated system.

## Why

- **As data, over a contract.** A second timer written for each target would be a second thing to test. The facts vary, and the behaviour does not.
- **Exact integers, and no clock.** So one input has one outcome, and a test can walk every instant at small moduli rather than sample them.
- **A crate of its own.** A generated system does not need these until a plan names a device, and `crates/sim-playground`'s sources are copied
  into every generated system, so adding them there now would put unused code into each.

## How to apply

- A change to what a contract promises changes `crates/sim-devices/tests/contract.rs` in the same commit.
- `M4.3.6` wires them in, and `M4.9` compares a target against what they say. What a build enforces of a stated delivery bound, which the
  profile names `M4.5` for, is `M4.5.2`: a model of a controller enforces no bound.
