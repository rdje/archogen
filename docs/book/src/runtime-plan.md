# The runtime plan: what the generated system is told

## The idea, in plain words

By this point the engine knows which part of the platform serves each need of the application, and a second program has
checked that choice. What it does not yet have is a set of instructions for building the system: in what order the parts
start, which task runs at which priority, what releases each task, which interrupts are switched on, and what each part
owes before the system can be trusted.

That set of instructions is the **runtime plan**. It is a short text, written by the engine and read by the code
generator, and it is the only thing the generator is told. Nothing about a system gets into the generated code except
through this text.

Think of a theatre's prompt book. The casting is settled, and it has been checked. The prompt book then says who enters
when, which lights come up on which cue, and which props somebody still has to bring. It holds no acting, and it does not
pretend the props are already there. A plan is the same: it says what the running system must do, and it says openly what
is still owed, by whom, before anyone can claim anything about it.

> **In one minute, for engineers.** `crates/archogen-plan/src/lib.rs` lowers an `archogen-assignment/2` record into
> `archogen-runtime-plan/1`, for a system run on the hosted playground; a target's memory layout is the next format's
> (`M4.1.7`). The production entry, `lower_checked`, takes the elaborated program, the record and the configuration
> checker's `archogen-result/1`, and refuses all but what the checker accepted for those same inputs. The plan is one
> canonical text, whose sha256 is its identity, and a change that would move a plan for the same inputs is a new format
> number. `crates/archogen-plan/src/contents.rs` reads it back into typed contents and refuses anything its writer would
> not write. The design is `docs/decisions/decision_runtime-plan.md`.

## A worked example

The S0 heartbeat is two periodic tasks writing to a console through one service. Its plan begins:

```text
(format "archogen-runtime-plan/1")
(profile rt-static-up-v1)
(system heartbeat)
(program "…")
(assignment "…")
(adapters adapters/1)
(task 0 (task heartbeat beat) (rank 1) (period 1/100 s) (deadline 1/100 s) (deadline-from release) (jitter 0 s) (overrun fault) (released-by timer) (uses console.write))
(task 1 (task heartbeat chime) (rank 2) (period 3/100 s) (deadline 3/100 s) (deadline-from release) (jitter 0 s) (overrun fault) (released-by timer) (uses console.write))
(bind console.write observable-output console.uart)
(timer simulator (releases (task heartbeat beat) (task heartbeat chime)))
(enabled timer)
(interrupt-model (nesting no) (masking all))
```

Reading it from the top:

- The opening lines say what this plan is, and pin what it came from: the digests of the program and of the assignment,
  written as `…` here, and the adapter catalog.
- Each `task` line gives the task's runtime index and rank, then its timing, as exact fractions of a second. A period of
  10 ms is written `1/100 s`, so no rounding can creep in.
- `bind` copies the checked assignment: `console.write`'s need for output is served by `console.uart`.
- `timer` says the simulator's timer releases both tasks, and `enabled` that the timer is the one interrupt switched on.

It goes on to say how the parts start:

```text
(node console.uart block)
(node console.write service)
(context console.uart job initialisation)
(context console.write job initialisation)
(depends console.uart console.write (because (binding observable-output)))
(step 1 console.uart)
(step 2 console.write)
```

The console's service depends on the serial port it was bound to, so the port starts first. A description never writes
this order, and it could not get it wrong: the plan derives it from the bindings (model §8 rule 12).

Most of its length is what it does not have:

```text
(owed (task heartbeat beat) entry-point (by application))
(owed (task heartbeat beat) execution-bound (by application))
(owed (task heartbeat beat) os-calls-category (by application))
(owed console.uart runtime-discipline (by realization))
(complete no)
```

⚠️ **`(complete no)` is true of every plan today.** Nothing yet supplies the application's own inputs (`M4.12`), so each job
is an empty body the release trace alone observes, and a plan says so rather than implying a finished system.

## The rules

**What it takes.** The lowering has four inputs, and it takes none of them on trust:

| Input | Where it comes from | What is checked |
| --- | --- | --- |
| the elaborated program | the language's check, which accepted it | the plan's digest is computed from it, never given |
| the assignment record | the search, `archogen-assignment/2` | it reads, and names this program, a declared system and this engine's catalog |
| the checker's result record | `archogen-plan-check`, `archogen-result/1` | it reads, it is that program's, it judged this assignment and this program under this profile, and it accepted |
| the vocabulary and the adapter catalog | compiled into the engine | nothing: a caller cannot hand it others |

**What it refuses, and in what order.** The refusals come in one order, and the first that applies is the one given:

| Refusal | Code |
| --- | --- |
| the assignment record does not read, or names another format | `tool-failure` |
| it names another program, a system the program does not declare, or another catalog | `stale-input` |
| the result record does not read, another program wrote it, or it judged a build | `tool-failure` |
| it judged another assignment, program or profile | `stale-input` |
| it refused what the generator accepted, the two verdicts both named | `tool-failure` |
| the profile is not `rt-static-up-v1` | `unsupported-profile` |

After those come the refusals of what this format cannot lower, each naming what is missing: a system with no task,
placement memory, a deadline measured from anything but the release, and a task that would receive an alarm's or an
expiry's interrupt. A `tool-failure` is never a verdict about your description: it means two parts of the engine disagree,
and you should report it.

**Where it came from.** The lowering cannot see whether the checker that wrote a result was the installed, verified one,
so beside the plan it returns its **provenance**: the generator's version and digest, the result record's digest, the
checker's digest, and whether the command line ran the checker or the result was handed over as text. The provenance is
the build's metadata, and never part of the plan's text, so it cannot change a plan's identity. A plan whose acceptance was
handed over as text can only ever be built marked experimental.

**One text.** Each line is one form; tokens are parted by a single space; there is no comment and no blank line. A value
has one spelling, so `2/200 s` is no way to write `1/100 s`, and the reader refuses it rather than normalising it. Every plan
the lowering writes passes through that reader before it is returned, so the writer and the reader cannot part.

**Goldens.** Three plans, the heartbeat, a system with an epoch extender and a sporadic system, are frozen with the
assignment and the result record they were lowered from, in `crates/archogen-plan/tests/goldens/`. Nothing rewrites them:
each is held to its digest, and a change that would move one is a new format number (`docs/book/src/versions.md`).

## From a plan to a generated system

`crates/archogen-emit` turns a plan into the files of a package that builds and runs by itself, with nothing fetched:

```text
Cargo.toml                 the system's package, and its workspace
src/main.rs                the tables specialized from the plan, and a call of the simulator
rt-core/                   the engine's scheduler, byte for byte
sim-playground/            the engine's simulator, byte for byte
plan.txt                   the plan's text
provenance.json            what it was lowered from, and what each table entry came from
```

Behavior is copied, never generated: the only specialized code is data, so what runs is code the engine compiles and tests,
and a test holds each copy to its source. A plan is accepted only by lowering it again from the same inputs and comparing
bytes, so a plan that reached the emitter by any route is what the checked inputs make, or nothing is generated. It also
refuses a task the plan cannot say what releases, naming the task and the cause. Running the package prints a trace that
opens by saying what it is and what it did not do (`crates/sim-playground`; see [Where generated systems run](targets.md)):

```text
; archogen-playground-trace/1
; system heartbeat
; plan sha256:390fb545…
; scenario one hyperperiod, 3 ticks of 1/100 s; …
; no job runs application code: …
; the plan is not complete: …
0 s initialised 1 console.uart
```

## What this does not establish

The plan is a statement of intent, and checked input for the generator. It does not establish that the system meets a
deadline, which is the scheduling checker's to say, under its own assumptions ([analysis](analysis.md)), nor that any
generated code does what the plan says, which `M4.4`'s checks of the built image answer.
