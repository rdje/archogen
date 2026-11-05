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
> generated: the package holds `rt-core` and `sim-playground` byte for byte beside tables of exact integer ticks. Where it lands, and
> the ways that can go, are one table a test runs, and no byte of the package depends on the directory its description lies in or on
> where it goes. It stays off the programmatic surface, by `ROADMAP.md` §10.4.
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
; archogen-playground-trace/3
; system heartbeat
; plan sha256:390fb545f810c644ba38a4341dc4a158f2d586d3c728dea3879867e0fd529fa2
; scenario one hyperperiod, 3 ticks of 1/100 s; each source at its stated separation, from time zero
; no job runs application code: each body is empty, so only the releases are informative
; the plan is not complete: what it owes is unmet, and this run establishes no property
0 s initialised 1 console.uart
0 s initialised 2 console.write
0 s enabled
0 s timer-programmed 0 s
0 s pending timer
0 s entry timer
0 s release beat nominal 0 s
0 s release chime nominal 0 s
0 s timer-programmed 1/100 s
0 s exit timer
0 s start beat
0 s complete beat
0 s start chime
0 s complete chime
0 s idle
1/100 s pending timer
1/100 s entry timer
1/100 s release beat nominal 1/100 s
1/100 s timer-programmed 1/50 s
1/100 s exit timer
1/100 s start beat
1/100 s complete beat
1/100 s idle
1/50 s pending timer
1/50 s entry timer
1/50 s release beat nominal 1/50 s
1/50 s exit timer
1/50 s start beat
1/50 s complete beat
1/50 s idle
```

Reading it from the top: the lines beginning `;` say what the trace is, which plan it ran and what it did not exercise. The
parts start in the plan's order, with the serial port before the service that writes to it. Then the timer fires, `beat` and
`chime` are released in rank order, each with the instant it was due, and each job starts and completes at one instant, because its
body is empty. `idle` marks the moment nothing is left to run. Times are exact
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
| `provenance.json` | what the plan was lowered from, what each table entry came from, every other file with its length and its digest, and its own digest |
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
| 3 | `--out`, and what stands beside it | the place: `usage`, before anything is written, and the table below says which |
| 4 | the installed `archogen-plan-check` | it could not judge, or two programs disagree about one input: `tool-failure`; or the description or a module cannot be handed to it as a file, by a name it can read: `usage` |
| 5 | the lowering and the emitter | what this format cannot yet express |
| 6 | writing and landing the package | the output directory: `tool-failure`, or `usage` where a name a module brought leaves no room in a path |

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

**Where the package lands.** The command builds it in `DIR.building`, beside `DIR`, and moves it into place by two renames once every
file is written. So `DIR` holds at every moment what stood there, nothing, or the whole package, and a refused build leaves it as it
found it. Your changes belong in the description, not in the output.

⛔ **A build deletes nothing it did not write**, but for two things: an empty directory at `--out`, which it takes, and its own stage,
which on a refusal goes with anything put in it, unless the table says it stays: for a report, or because something else wrote there. Of an earlier
build it removes each file `provenance.json` lists, one by one and only if still the regular file it wrote, then the directories that
held them where that leaves them empty. What building and looking at a package leave at its top, `target`, `Cargo.lock` and `.DS_Store`,
is neither judged nor deleted: it moves to the new package, so a rebuild keeps cargo's work.

**The stage is private to its owner**, mode `700`, until the package lands, whatever `DIR` lets others do. Where something stands at
`DIR` it is `700` from the call that makes it; where nothing stands it is made first at what a new directory takes there, empty, for the
instant before it is closed, since that is the mode the package will keep. Then the package takes the mode and the group of what it
replaces, read again just before it moves, so closing a directory while a build runs holds. ⚠️ Those two are all it keeps. The standard
library reads no access control list, owner or flag, so where one of those is what makes a directory private, a build there loses it; and
a list a parent passes down to what is made in it applies to the stage, whatever its mode.

**The routes `--out` can take.** Each row is a fixture of `crates/archogen-cli/tests/build.rs`, which sets the route up, runs it, holds
what is left and compares the status and the words with the row. A row with no fixture, or a fixture with no row, fails. So this table is
the rule: what it says is run, and a route it does not name is promised nothing and worth reporting.

<!-- machine-read: out-routes -->
| Route | What stands there, or what happens | Status | It says | What is left |
| --- | --- | --- | --- | --- |
| `no-name` | `--out ..` | `usage` | `names no directory of its own` | nothing made |
| `back-through` | `--out missing/../x`, which reads as absent until `missing` is made | `usage` | `goes back through a name` | nothing made |
| `a-file` | a file at `--out` | `usage` | `is no directory` | the file |
| `a-link` | a link at `--out` | `usage` | `is no directory` | the link, and what it points at |
| `closed` | a directory this user cannot read and search | `usage` | `chmod u+rx` | it, at its mode |
| `not-a-build` | a directory holding what no build wrote | `usage` | `a build deletes nothing it did not write` | all of it |
| `edited` | an earlier build with a file edited | `usage` | `has changed since the build wrote it` | all of it |
| `closed-within` | an earlier build with a directory this user cannot read | `usage` | `open what it could not read` | all of it |
| `listed-gone` | an earlier build with a listed file removed | `usage` | `cannot be replaced` | the rest of it |
| `a-provenance-past-what-it-reads` | a provenance larger than this engine reads one, told by its length without reading it | `usage` | `larger than this engine reads one` | all of it |
| `flagged` | a flag on `--out`, found at the last step, where the platform has flags | `tool-failure` | `any flag on either` | all of it; nothing this run wrote |
| `other-format` | a provenance of a format this engine does not read | `usage` | `this engine reads` | all of it |
| `stage-stands` | anything at `DIR.building` | `usage` | `a build that stopped may have left it` | it |
| `aside-stands` | anything at `DIR.replaced` | `usage` | `kept there what could not go` | it |
| `no-room-name` | a name with no room for `.building` beside it | `usage` | `too long` | nothing made |
| `no-room-path` | a path with no room for what a build writes beneath it, as the checker is handed it | `usage` | `leaves no room in a path` | nothing made |
| `file-on-the-way` | a file among `--out`'s parents | `usage` | `is no directory` | the file |
| `link-on-the-way` | a link to nothing among `--out`'s parents | `usage` | `a link to nothing` | the link |
| `link-to-a-directory-on-the-way` | a link to a directory among `--out`'s parents | `ok` | `generated` | the package, beneath what the link points at |
| `link-loop-on-the-way` | a loop of links among `--out`'s parents | `usage` | `name another place` | the links |
| `closed-parent` | a parent this user cannot write | `usage` | `reach and write` | the parent, empty |
| `reads-once` | a description that is no regular file, such as a pipe or `<(…)`, which the checker would read again | `usage` | `no regular file` | nothing made |
| `reads-differently` | a description that reads differently the second time, such as `/dev/stdin` from a file | `usage` | `reads differently` | nothing made |
| `a-module-that-reads-once` | a module that is no regular file | `usage` | `no regular file` | nothing made |
| `no-checker` | no install beside the command, so no checker can be run | `tool-failure` | `has no install manifest` | nothing made |
| `absent` | nothing | `ok` | `generated` | the package, at what a new directory takes there |
| `empty` | an empty directory | `ok` | `generated` | the package, at that directory's mode |
| `earlier` | an earlier build, with what cargo left | `ok` | `generated` | the package, at that mode, what cargo left moved to it |
| `stays-aside` | an earlier build with what cannot be removed, a closed directory | `ok` | `keeps what could not go` | the package; that at `DIR.replaced`, which the next build refuses over until it is removed |
| `closed-to-writes` | a directory closed to its owner's writes, a `555` or a `500` | `ok` | `open it if cargo stops` | the package, closed as it was |
| `came-at-out` | while it runs, something comes to `--out`, where nothing stood | `tool-failure` | `appeared while the build ran` | what came; no package |
| `came-aside` | while it runs, something comes to `DIR.replaced`, an earlier build standing at `--out` | `tool-failure` | `appeared while the build ran` | what came, and the earlier build |
| `aside-not-needed` | while it runs, something comes to `DIR.replaced`, nothing standing at `--out`, so the name is not needed | `ok` | `generated` | the package and what came, which the next build names |
| `gone` | while it runs, what stood at `--out` goes | `tool-failure` | `moved or removed what the build was moving` | nothing |
| `came-in-earlier` | while it runs, something comes into the earlier build | `ok` | `keeps what could not go` | the package; what came at `DIR.replaced` |
| `came-in-empty` | while it runs, something comes into an empty directory at `--out` | `ok` | `keeps what could not go` | the package; what came at `DIR.replaced` |
| `swapped-meanwhile` | while it runs, what stood at `--out` is swapped for a file | `ok` | `is no longer a directory` | the package; the file at `DIR.replaced` |
| `came-in-stage` | while it runs, a new name comes in the stage | `ok` | `generated` | the package and what came, which the next build names, or carries on under a name a build carries |
| `filled-in-stage` | while it runs, something fills a name the package writes | `tool-failure` | `came in the stage` | the stage, with what came; no package |
| `linked-in-stage` | while it runs, a link stands where the package writes a directory | `tool-failure` | `came in the stage` | the stage; what the link points at, untouched |
| `stage-swapped` | while it runs, the stage is swapped for a link | `tool-failure` | `came in the stage` | the link and what it points at; no package |
| `stage-replaced` | while it runs, another directory comes to stand at the stage's name | `tool-failure` | `wrote no record` | that directory, with what it holds; no package |
| `stage-gone` | while it runs, the stage is removed | `tool-failure` | `wrote no record` | nothing |
| `parent-closed-meanwhile` | while it runs, `--out`'s parent is closed | `tool-failure` | `a build renames` | no package; the stage, which it says could not be removed |
| `carried-in-stage` | while it runs, the stage comes to hold a name a build carries, an earlier build holding it too | `ok` | `keeps what could not go` | the package with what came; the earlier one at `DIR.replaced` |
| `closed-meanwhile` | while it runs, the directory at `--out` is closed further | `ok` | `generated` | the package, at the mode it was closed to |
| `record-unsound` | the checker's record does not hold together with what it did | `tool-failure` | `does not hold together` | the stage, with the assignment and the record for the report |
| `disagreed` | the checker refuses the search's plan | `tool-failure` | `refuses the plan the search wrote` | the stage, with the assignment and the record for the report |

No sync call is made, so none of this claims anything across a power loss. ⚠️ Nor does it hold against another program working in the same
directory at the same instant: the standard library has no rename that refuses to replace and no directory-relative open, so each path is
held again just before it is used, and in that instant what stands there is what is used. The checker's record is the one name a
program's rename fills on purpose, so what comes there while the checker runs is replaced by it.

**The two standard streams.** If a write to standard output fails, at any byte, the command says so on standard error and exits `70`,
as every command does: a pipe nobody reads, a full disk. What it did stands, so a package it placed is in place. ⚠️ A failure is what it can
see: the Rust runtime gives a stream closed before the run `/dev/null`, and takes a stream open only for reading as one that wrote, so
`archogen build … >&-` exits with the command's own code and says nothing. A standard error nobody reads changes nothing: what it would have
said is lost, and the command and its exit code are as they would have been.

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
no account of a plan it did not write: the search can only speak for its own. Its repair is a build into a new directory, since `archogen build` will not write over
one that has changed. What cannot be read is no change: a file or directory of a build closed to this user is `usage`, whose repair is to open it, and with nothing rebuilt it then reads as it did. ⚠️ A digest catches an edit and a stale file, never a
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
