# Building a system

## The idea, in plain words

`archogen build` takes a description and writes a small project you can compile and run. To get there it walks the whole
pipeline this book describes, in order: the language checks the description, the search chooses which part serves each need, a
second program checks that choice, the choice is lowered into a [runtime plan](runtime-plan.md), and the plan is turned into
source files. It stops at the first step that says no. Nobody edits the output, and nothing is fetched from the network.

What you get today is honest but modest. The project holds the engine's real scheduler, on the [hosted
playground](targets.md), driven by tables made from the plan. But nothing yet supplies the application's own code, so each
task's job is empty. Running it is a dress rehearsal with the cues called and no actors on stage: you see which task is released
when, and that is all it can show.

> **In one minute, for engineers.** `crates/archogen-cli/src/build_cmd.rs`: the engine API's `resolve`, the installed
> `archogen-plan-check` over the plan it wrote, `archogen_plan::lower_checked` on its acceptance, then
> `archogen_emit::emit`, which accepts the plan only by lowering it again and comparing bytes. Behavior is copied and never
> generated: the package holds `rt-core` and `sim-playground` byte for byte beside tables of exact integer ticks. `--out` is at
> every moment a whole package or absent, a build deletes nothing it did not write, and no byte of the package depends on the
> directory its description lies in or on where it goes. It stays off the programmatic surface, by `ROADMAP.md` §10.4.
> The design is `docs/decisions/decision_hosted-build.md`. The system's own behavior is held by `crates/archogen-cli/tests/functional_fixtures.rs`,
> against observations frozen before any generator existed.

## A worked example

The description is the two periodic tasks of [A tour of archogen](tour.md):

```console
$ archogen build examples/s0-heartbeat/system.eadl --out build/heartbeat
examples/s0-heartbeat/system.eadl: generated `system-heartbeat` into build/heartbeat
  13 file(s): Cargo.toml, src/main.rs, rt-core/Cargo.toml, sim-playground/Cargo.toml, rt-core/src/lib.rs, rt-core/src/scheduler.rs, rt-core/src/fault.rs, sim-playground/src/lib.rs, plan.txt, description/system.eadl, assignment.eadl, plan-check.json, provenance.json
  checked by `archogen-plan-check`: accepted (build/heartbeat/plan-check.json)
  2 task(s), a run of 3 tick(s) of 1/100 s
  experimental: the hosted playground: the plan's releases run, and no job holds application code; completed by M4.12
    the plan is not complete: what it owes is unmet, and a run establishes no property. See `archogen help build`.
  next: cargo run --manifest-path build/heartbeat/Cargo.toml
```

The checker's own verdict line reaches standard error as it runs. A task a source releases gets a line of its own, saying which
source the plan inferred. Run what it wrote:

```console
$ cargo run --quiet --manifest-path build/heartbeat/Cargo.toml
; archogen-playground-trace/1
; system heartbeat
; plan sha256:390fb545f810c644ba38a4341dc4a158f2d586d3c728dea3879867e0fd529fa2
; scenario one hyperperiod, 3 ticks of 1/100 s; each source at its stated separation, from time zero
; no job runs application code: each body is empty, so only the releases are informative
; the plan is not complete: what it owes is unmet, and this run establishes no property
0 s initialised 1 console.uart
0 s initialised 2 console.write
0 s enabled
0 s timer-programmed 0
0 s pending timer
0 s entry timer
0 s release beat
0 s release chime
0 s timer-programmed 1
0 s exit timer
0 s start beat
0 s complete beat
0 s start chime
0 s complete chime
1/100 s pending timer
1/100 s entry timer
1/100 s release beat
1/100 s timer-programmed 2
1/100 s exit timer
1/100 s start beat
1/100 s complete beat
1/50 s pending timer
1/50 s entry timer
1/50 s release beat
1/50 s exit timer
1/50 s start beat
1/50 s complete beat
```

Reading it from the top: the lines beginning `;` say what the trace is, which plan it ran and what it did not exercise. The
parts start in the plan's order, with the serial port before the service that writes to it. Then the timer fires, `beat` and
`chime` are released in rank order, and each job starts and completes at one instant, because its body is empty. Times are exact
fractions of a second, so `1/50 s` is 20 ms and no rounding can creep in. The tick is derived from what the run repeats: here
periods of 10 ms and 30 ms, so it is 1/100 s.

## What is in the package

Thirteen files, and only one of them is generated in any interesting sense:

| File | What it is |
| --- | --- |
| `Cargo.toml` | the system's package and its workspace; every dependency is a relative path, so it builds wherever it is put |
| `src/main.rs` | **the only specialized code**: the plan as tables, and the call that runs them |
| `rt-core/Cargo.toml` | a generated manifest |
| `rt-core/src/lib.rs` | the engine's scheduler, byte for byte |
| `rt-core/src/scheduler.rs` | the engine's scheduler, byte for byte |
| `rt-core/src/fault.rs` | the engine's fault path, byte for byte |
| `sim-playground/Cargo.toml` | a generated manifest |
| `sim-playground/src/lib.rs` | the engine's simulator, byte for byte |
| `plan.txt` | the runtime plan's canonical text |
| `provenance.json` | what the plan was lowered from, what each table entry came from, every other file with its digest, and its own |
| `description/system.eadl` | the description it was made from, with every module it imports beside it |
| `assignment.eadl` | the assignment the search wrote, which the checker judged |
| `plan-check.json` | the checker's result record for it |

⭐ **Generation writes a table, never behavior.** A generated scheduler would be a second scheduler nobody tested. What runs here
is code the engine compiles and tests, and `crates/archogen-emit/tests/emit.rs` holds each copy byte for byte to its source.

## The rules

**The order is the contract**, and the first step that refuses is the answer:

| Step | Who | A refusal means |
| --- | --- | --- |
| 1 | the profile, then `--locked` | the invocation cannot be honored |
| 2 | the engine API's `resolve`: the check, then the search | the description, or what it asks of its hardware: the same verdict `archogen check` or `archogen resolve` gives |
| 3 | `--out`, and what stands beside it | the place: `usage`, before anything is written |
| 4 | the installed `archogen-plan-check` | two programs disagree about one input, so `tool-failure` |
| 5 | the lowering and the emitter | what this format cannot yet express |
| 6 | writing the package | the output directory |

There is one frontend, so `build` can never accept what `check` refuses.

**What the emitter refuses.** A task the plan cannot say what releases, naming the task and the cause, is `missing-fact`,
repaired by the task's own `(needs interrupt-source)`, or `unsupported-profile` where the analysis cannot admit the source. That
comes first, since what a run would cost means nothing until it can run. A
period or a separation the run repeats, or a hyperperiod of them, past 64 bits of ticks is `unsupported-profile`, rather than
approximated, and so is a run of more than
`2^20` releases and arrivals, which nobody could read: periods of `1 ms` and `2305843009213693951 ms` both fit, and that run would
never end. A plan that is not what its inputs lower to is `tool-failure`: `crates/archogen-emit` accepts a plan only by lowering it
again from the same inputs and comparing bytes.

```console
$ archogen build examples/s0-heartbeat/system-unsupported.eadl --out build/x
archogen: missing-fact: the plan cannot say what releases task `chime`, `unreached`: write the task's own `(needs interrupt-source)`, so the search binds it a source at least as far apart — nothing was generated from examples/s0-heartbeat/system-unsupported.eadl
$ echo $?
11
```

**`--locked` is refused, not ignored.** Its contract is to fail on any missing or mismatched locked input, and there is no lock
data to compare against until `M4.7`. Accepting the flag would hand someone who asked for a reproducible build an ordinary one
that claimed to be reproducible. Refusing costs a user one re-run; accepting costs them a belief. It exits `20`, `unimplemented`.

**Where the package lands.** The command builds it in `DIR.building`, beside `DIR`, and moves it into place by two renames once
every file is written. So what a build places at `DIR` is at every moment a whole package or absent, and a refused build leaves it
as it found it: a refusal of the description creates nothing, and an earlier build stays.

⛔ **A build deletes nothing it did not write**, but for an empty directory named by `--out`, which it takes, keeping the mode. It
removes the stage this very run made, with anything put in it while it runs, and of an earlier build each file `provenance.json`
lists, one by one and only if still the regular file it wrote, then the directories that held them where that leaves them empty.
An earlier build must be exactly a build's before any of that: a directory holding anything more, an empty directory within it
included, a generated file you edited, a link named by `--out`, and anything standing at `DIR.building` or `DIR.replaced` are all
refused, and left alone.
Your changes belong in the description, not in the output. What building and looking at a package leave at its top, `target`,
`Cargo.lock` and `.DS_Store`, is neither judged nor deleted: it moves to the new package, so a rebuild keeps cargo's work.

Two things it leaves behind on purpose. Where the checker refuses the search's plan, or its record does not hold together,
`DIR.building` keeps the assignment and the record, for the report that disagreement deserves, and the next build refuses until
someone has looked at it. And what could not go from an earlier build, or appeared while the build ran, stays at `DIR.replaced`, with
the new package already in place and the command naming what stayed. A directory of yours keeps its mode. No sync call is made, so
none of this claims anything across a power loss. ⚠️ Nor does any of it hold against another program working in the same directory at
the same instant: the standard library has no rename that refuses to replace, so each path is held again just before it is used.

**Nothing in it depends on where it was built.** A provenance entry names a source relative to the description, so the same
description in two directories, built into two others, gives the same bytes. A test compares them, for one file and for a module
tree. The description's own file name is recorded, so a renamed copy is a different package.

**Where each entry came from.** `provenance.json` follows each table entry to its line in `src/main.rs` and its line in the plan,
and a task's to its form in the description. The failure to fear is not a missing entry but one that points somewhere wrong, which
sends a reader somewhere confidently. So `crates/archogen-cli/tests/build.rs` follows every entry from both ends, and counts them.

**The command line alone.** `ROADMAP.md` §10.4 rules generation outside the programmatic interface: it writes a crate tree, and
exposing it would give a remote consumer a filesystem authority the rest of the surface does not need. So the engine API, the
wasm module and the [MCP](ledger.md#mcp-specification) server offer no build, and only the `archogen` executable holds a launcher
that can run the checker. What `build` asks of the engine API is what the search already holds: the plan and what it was written
from.

**A build can be asked about later.** It records the description it was made from, so `archogen explain` reads a build as §10.2
writes it, explaining what that description requires and comparing the search's plan with the one the build holds:

```console
$ archogen explain build/heartbeat --requirement observable-output
`observable-output` required by `console.write`
  written at build/heartbeat/description/system.eadl:32:20
  `console.uart` meets it
  `host.playground` leaves it undescribed
  bound to `console.uart` by the search's plan
build/heartbeat: the search's plan for system `heartbeat` is the one this build holds, assignment sha256:e77556b645eea31dda156903d6ddf570d5f5fea1f7ad6e167a4eca7467bd6280, which `archogen-plan-check` accepted
  experimental: one requirement of one system, over a description's own blocks and platforms; completed by M3.7
```

It holds the build to what `archogen build` holds an earlier one to first: every listed file and the provenance itself unchanged, and
nothing else there but what building and looking at a package leave. A build that fails that, whose description no longer gives its
plan, or whose record does not accept that plan from the checker it names is `stale-input`, and the command prints
no account of a plan it did not write: the search can only speak for its own. ⚠️ A digest catches an edit and a stale file, never a
forger: whoever can write the directory can rewrite the list too. A provenance that no longer reads, or names another format, is
`usage`, being no build this engine reads, and so is a request that misses, a requirement nobody writes, as of any description.

**The two programs together.** `crates/archogen-cli/tests/common/mod.rs` runs the checker's own judgement in the test's process,
so a test sees a real plan past the real rules. `scripts/resolve_installed.sh`, in the `integration` tier, runs the real
executables from a real install, and builds and runs the package they write ([Verifying the toolchain](verification.md)).

## S0's fixtures, through this path

The observations [the S0 path](s0.md) froze before any generator existed still hold, byte for byte: `crates/archogen-cli/tests/functional_fixtures.rs`
generates each system from a clean directory, compiles it, runs it, reads the releases out of its trace and compares them with
the frozen file. One moved, with its reason in its own header. The task S0 could only call unsupported, a minimum separation with
nothing to release it, is refused now for what it is: a missing fact of the description.

## What this does not establish

A generated system runs its releases and nothing else. No job holds application code until `M4.12`'s inputs exist, so a trace says
nothing about what a task does or how long it takes. The plan is `(complete no)`, and it lists what is owed. Nothing here is a
timing result, which is [the scheduling checker's](analysis.md) to give under its own assumptions, and nothing says the hosted
playground behaves like hardware: it models no device yet (`M4.3`).
