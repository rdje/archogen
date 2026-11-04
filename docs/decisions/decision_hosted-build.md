# The hosted build: from a runtime plan to a generated system that runs on the playground

- **Type:** `decision`
- **Date:** `2026-11-04`
- **Status:** `active` — written by leaf `M4.2.1`; built ahead of its review as its executable form
  (`decision_executable-design-reviews.md`)
- **External sources:** none: this record decides how the engine's own parts fit together
- **Owner / source:** leaves `M4.2` and `M4.3` (`docs/tasks/M4.md`), carrying `RTP-H1`, `RTP-H2`, `RTP-H3` and `RTP-H9` of
  [[decision_runtime-plan]]; `ROADMAP.md` §6.3 and §10.1 steps 6 to 8.

## The fact / decision

`archogen build` goes from a description to a package that compiles and runs on the hosted playground, through the
checked runtime plan, with no hand edit and nothing fetched. It replaces the S0 path (`crates/archogen-s0`), which stays
until its retained fixtures run through this one.

### 1. The generated package

The build writes one [Cargo](../book/src/ledger.md#rust-toolchain) workspace, whose root is the system's package:

```text
Cargo.toml                 the system's package, its dependencies by relative path, and `[workspace]`
src/main.rs                the tables specialized from the plan, and a call of the simulator
rt-core/                   crates/rt-core's sources, byte for byte, under a generated manifest
sim-playground/            crates/sim-playground's sources, byte for byte, under a generated manifest
plan.txt                   the runtime plan's canonical text
provenance.json            what the plan was lowered from, and what each table entry came from
```

Behavior is copied and never generated: what `src/main.rs` adds is data. So the code that runs is code the engine compiles
and tests, as `ROADMAP.md` §1.1 admits ("copied or linked components") and as S0 did, and a test holds each copy byte for
byte to its source. A path dependency on the checkout would tie the output to the place it was built, and a registry
dependency would need the network, which §10.3 keeps out of a build.

### 2. Time is exact

A plan's durations are exact rationals, so the tables hold integer **ticks** of `1/L` s, `L` the least common multiple of
every duration's denominator in the plan. Nothing rounds. A plan whose ticks would not fit `u64` is refused
`unsupported-profile`, naming the duration, rather than approximated.

### 3. The simulator

`crates/sim-playground` is the deterministic event harness of `ROADMAP.md` §6.3, over the shared `rt-core`. It does no I/O:
it hands each event to an observer, and the generated `main` prints them.

**The order of one instant** is its whole allowed-ordering policy, so that one input has one trace:
1. an interrupt becomes pending at its arrival, whether or not anything can take it;
2. while the hardware is unmasked, every pending interrupt is taken, one at a time and none nested: the routes in the plan's
   rank order, then the release timer, as `RTP-H9` writes it. Taking one runs entry, its releases and exit, and each
   release is decided by `rt-core`;
3. the scheduling decision follows, and its switch;
4. the running job's body advances, until the next arrival or its own next step.

A mask step masks the hardware before raising the depth, an unmask lowers the depth before unmasking it, so no service ever
runs with the depth raised (`releases-never-latched`). Initialisation runs first, in the plan's steps, interrupts masked,
and interrupts are enabled at time zero.

**A body is data**: a list of steps, *run for N ticks*, *mask*, *unmask*. `/1` writes every job's as empty, so a job starts
and completes at one instant and the release trace alone observes it, until `M4.12`'s inputs give it code. The harness runs
longer bodies all the same, so preemption, masking and overrun are exercised now, by its own tests.

**A run names its scenario.** The horizon is one hyperperiod of every period and every separation the plan states. Each
source whose separation is stated arrives at exactly that separation from time zero, the densest the plan allows. One whose
separation is `unknown` or `absent` does not arrive, and the trace's header says so: a run never claims what it did not
exercise. **The trace** is `archogen-playground-trace/1`, one line an event, its time exact.

### 4. The emitter

`crates/archogen-emit` does no I/O: it turns its inputs into a list of files. It takes the program, the assignment, the
result record and the plan's text, and **accepts the plan only by lowering it again from those inputs and comparing bytes**
(`RTP-H3`). What it then refuses, each naming the task:
- `(released-by (unknown CAUSE))`: `missing-fact` or `unsupported-profile`, as `RTP-H1` gives each cause;
- a duration past `u64` ticks (§2): `unsupported-profile`.

A plan whose acceptance was taken as text is built marked experimental, in `provenance.json` and in the trace's header.

### 5. The command

`archogen build DESCRIPTION --out DIR [--system NAME] [--profile P]` checks the description, searches, runs the installed
configuration checker on the plan it wrote, lowers on its acceptance, emits, and writes the package. `--locked` stays refused
until `M4.7`. It says what it did not do: that no job holds application code, and that the plan is `(complete no)`. The engine
API and the [MCP](../book/src/ledger.md#mcp-specification) tool, which run no checker, take a result record as text and so build only marked experimental (§4).

### 6. What S0 keeps, and when it goes

S0's frozen expected observations are functional fixtures of the system's behavior, and §12 M4 asks that they run through the
supported path. So a test holds each one's releases equal to the new trace's. Then `crates/archogen-s0` is deleted, and each
assumption the retirement record lists is discharged or moved by its leaf.

## Why

- **Copies over dependencies.** The output is a self-contained tree that builds anywhere, and `rt-core` is the same bytes the
  engine tests. A generated scheduler would be a second scheduler nobody tested.
- **Integer ticks over rationals in the run.** The generated system and the harness then need no rational arithmetic, and a
  target's timer, which counts, reads the same tables. Exactness is kept where it matters, in the plan, and the tick is
  derived from it.
- **One order for one instant.** §6.3 asks which orderings a model allows. This harness allows one, so a trace can be compared
  byte for byte, and a later model that allows more states its own policy.
- **A body as data, before any application exists.** A simulator exercised only on empty jobs would not be exercised at all.

## How to apply

- `M4.3.1` builds the simulator; `M4.2.2` the emitter; `M4.2.3` the command, its faces and the book; `M4.2.4` S0's fixtures
  through the supported path and S0's retirement.
- A change that moves a generated file for the same inputs moves `archogen-playground-trace/1` or its emitter's identity, as
  `docs/book/src/versions.md` registers them.
