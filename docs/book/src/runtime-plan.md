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

⚠️ **`(complete no)` is true of every plan today.** The application's own inputs have a format ([below](#the-applications-inputs)),
and nothing yet carries them into a plan or a run, so each job is an empty body the release trace alone observes, and a plan
says so rather than implying a finished system.

## The application's inputs

A description never says how a task's code works, so it cannot say how long that code takes, how much stack it needs or
which calls it makes. Whoever wrote the code says, in one file per system, and the plan's `(owed … (by application))` lines are
what that file answers. Its format is `archogen-application/1`.

```text
(format "archogen-application/1")
(system heartbeat)
(target hosted-playground)
(code (crate app) (place "../app") (digest "sha256:…"))
(leaves-interrupt-hardware-alone yes (clause "…the words below, whole…") (basis "it touches no device"))
(initialisation
  (entry-point "app::init")
  (stack-bound (value 256) (unit byte) (scope "its deepest path") (binary unbuilt) (evidence assumed) (basis "chosen, to be measured"))
  (node-calls (scope "all of it") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by reading")
    (call console.write 1)))
(task beat
  (entry-point "app::beat")
  (execution-bound (value 200) (unit us) (scope "one job") (binary unbuilt)
                   (evidence analytically-established) (locator "docs/argument.md") (basis "by the loop's bound"))
  (os-calls (scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by reading")
    (call mask 2) (call unmask 2))
  (node-calls (scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by reading")
    (call console.write 1))
  (masked-runs (scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by reading")
    (run tail (ending unmask) (value 5) (unit us) (scope "the tail's update") (binary unbuilt)
              (evidence analytically-established) (locator "docs/argument.md") (basis "four loads")))
  (stack-bound (value 512) (unit byte) (scope "its deepest path") (binary unbuilt) (evidence assumed) (basis "chosen, to be measured"))
  (suspends no) (locks-scheduler no) (shares-outside-sections no)
  (masks-only-through-api yes)
  (figures-under-preemption yes)
  (deadlines-within-reach))
```

- **Nothing is defaulted.** A clause left out, written twice or unknown is refused, naming it.
- **Each bound and each list of counts says what it rests on**: its scope, the binary it is of, and one of `ROADMAP.md` §7.3's four categories,
  with where to check it, unless it is an assumption, and why it holds. So an empty list still says what backs it. A safety factor is checked, is kept and changes no category, so a
  padded observation is still an observation.
- **It says which code.** `code` names the crate a job's `entry-point` is a path in, where it lies and a digest of its sources, so an entry point is no bare name. The crate is its package's library, compiled alone, and none the toolchain links of its own, `std`, `core`, `alloc`, `proc_macro` or `test`. The digest is of what the compiler read in compiling that crate, by the compiler's own account of it: one line for each file, a sha256 and the name it was read by, wherever a source names it from, with the crate's manifest and its lock, and one line for each variable of the environment it read, which may be none but `CARGO_PKG_*` and `CARGO_CRATE_NAME`, every other moving with the host or with where the build stands. A file no source names moves nothing, this file among them, wherever it lies. The crate may depend on no other crate, its workspace is itself alone, by cargo's own account, and nothing an assembler directive reads without the compiler's knowing is held; a crate with a build script is refused, and so is a package declaring a binary, an example, a test or a bench by name alone or naming one by `default-run`, and a manifest holding a `[profile]` table: the digest's two compiles and the image's compile of the copy run under one profile, the build's, named none of `test`, `bench` and `check`, so that the three read the same files. [Cargo](ledger.md#rust-toolchain)'s own configuration could name another compiler or set a flag with no line of the digest moved, so the build lets cargo read none it does not hold, as the catalog's gate does. The build compiles a copy it writes of what the compiler reads inside the crate's directory, and hashes that copy, so the digest is of the bytes compiled, as the catalog's gate builds from a tree it writes, and links the image from one workspace whose root holds the generated code, the catalog's code and the copy, every compile there remapped to a fixed name and run under the gate's environment, its compile of the copy held to the digest's account, so that where the build stands moves nothing in the image identity, which is of the objects, though a linker may still write where they lay into the image itself; a name that leaves that directory, or a crate reading where it lies, is refused.
- **The application's initialisation is named too, or said to be none**: `(initialisation (entry-point …) (stack-bound …) (node-calls …))`, the path the boot calls before interrupts are first enabled, its own stack bound and each node it calls a function of, or `(initialisation none)`.
- **A `binary` names an image by its image identity**, which a build computes of the plan's code and of what builds it, the toolchain, its flags and features, a flag remapping a directory by the fixed name it gives alone and none naming a directory of the build's own, the target and the linker's input but for where its stacks and tables are laid. A change of any of those makes a bound or a list of counts stale, the application's code as far as its digest holds it, and it is never the image file's own digest, which a stack bound moves. An observed figure names the image it was observed on: beside `unbuilt` it is refused.
- **`node-calls` says which of the plan's nodes a job calls**, and how often, each as the plan writes it, and `os-calls` the runtime's primitives. `deadlines-within-reach` names each alarm the task writes on a time base that is the counter itself, saying that each deadline it writes stands at or after its own release and within what that alarm reaches.
- **A masked run says how it ends**, at an `unmask` or at the job's completion, since the two cost differently, and one that ends both ways is written once for each. It keeps its own evidence. Its job's `os-calls` names the `mask` that begins it, and the `unmask` that ends one ending so, or the file is refused, as is a job that names a `mask` and states no run. A job may call `unmask` and end every run at completion: a mask inside a mask is closed that way.
- **The one declaration about the whole application is quoted whole.** `leaves-interrupt-hardware-alone` is what every composite
  is gated on, and its name no longer says what it holds, so the file carries the words, and a file quoting others does not read.
  Those words count an overrun of code's own stack bound among what the code must not do, so the declaration holds only as far as
  each stack bound does, the initialisation's and each task's, whose category is said.

The words a file quotes are these, whole, the three parts joined by ` … `, "this section" being the composite-inputs record's §2 and "§7" its §7:

> after the first enabling of interrupts, application code runs only as a declared task's job or as a service's application code; and no application code, a task's, a service's or application initialisation's, writes the timer's counter or compare, the interrupt controller's configuration, or the hart's interrupt or trap state other than through the runtime API's masking primitives (a job's only: the fault contract, Terms) or reads or writes the controller's claim and complete registers, a read of which claims, or a declared source's device registers; no service calls a runtime primitive; and no application code releases a task or calls a runtime function other than the runtime API record's primitives and its completion path. A runtime function is any function of a catalog record's implementation, a driver's included, called after the first enabling of interrupts; before it, application initialisation reaches that hardware only through catalog functions, which `runtime-discipline.<id>` covers.

> Application code calls no entry of the fault path — in initialisation, before the first enabling of interrupts, as in a job or a service's application code after it; a panic reaching the image's handler is no such call

> The runtime's own data is what this section's facts read in memory or in a register a port reserves: what the scheduler holds, its mask depth among it, what the timer service holds of the next release, the plan's tables, each saved context, and what a service, a record's or generated, holds of its source. Application code writes none of it, nor any other data or code of a catalog record's implementation or of the code the build generates, but through the calls `leaves-interrupt-hardware-alone` lets it make, or into what a call between it and that code hands it to fill, and it hands no call a place in any of it to write, but what that code handed it; in initialisation as in a job or a service's application code, and by no other means, a stray pointer or an overrun of its own stack bound among them (its own stack, within its bound, is its own, and a write a guard stops writes nothing, and is the fault contract's). For this clause a runtime function is also a function of the code the build generates, but for a node's code its task line's `uses` lists or its own `bind` lines name (the runtime plan's §2.2, `RTP-H5`, `RTP-H13`), which §7 says what charges; and a job's code calls the completion path only as its last act, the path not returning to it.


A file is held against a plan, and each thing it leaves unmet is said: another system or target, a task the plan does not have or one with
nothing said of it, a call into a node the task's lines do not name, a node called as a primitive, an alarm named that the plan holds no node of, a node the initialisation calls that the plan holds none of, and a fact the plan owes that this format does not hold of that subject.

⚠️ This reads the format and holds it against a plan, and no more. `archogen-runtime-plan/2` carries it (`M4.1.7`), and the scheduling checker judges its figures
(`M2.21`), so today no figure in it reaches a verdict. The rules are `docs/decisions/decision_application-inputs.md`.

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

## On a real machine: format `/2`

Everything above describes `archogen-runtime-plan/1`, a system on the hosted playground. A real machine needs more, and
the design of `/2`, `docs/decisions/decision_runtime-plan-format-2.md`, not yet built (`M4.1.7`), says what: which machine it is, by the digest of
its description; which device releases tasks and which each alarm takes, with each count written as the device states it, so that the
checker alone converts it; where each stack lies; and what the interrupt controller is told. It also carries the
application's inputs. A size only a catalog record can state stays owed until one does, and every allocation after it is owed too, since no
address below it is known. The controller is told a line and a priority for each source and each alarm wired through it, in the order the plan ranks them;
one whose interrupt bypasses it is outside what the checker composes, so a system holding one is refused, and whether each reaches it is owed
until the realization, the generated code's own statement, says, for a source as for an alarm. The generated code around each job's start and each interrupt's
dispatch is timed inside the catalog's figures for the switch and the completion, and inside each interrupt service's own cost, the catalog's for the
timer service and the generated code's own for a generated one, each measured on the image that holds it. On a machine, each cost that statement gives,
and an alarm's lag, says too that it holds under any preemption, as a catalog record's does, and the checker reads it so, never assuming it;
the last act's lead needs no such word, as it runs before interrupts are first enabled.

Stacks are laid from the top of a region down and the image from the bottom up, so a change in the image's size moves no
allocation. A stack's size is what its owners state, by one table of sums: what generated code holds beneath what it calls, a task's own bound, the most the runtime's code adds in a
call, what a fault adds on top of the deepest of those, since a failed check or a panic can arrive there, and the frame the port saves. What the generated code uses of a stack comes of that code, so a first plan generates it with those figures
owed, and a second, from a file stating them, generates the same code: what an image holds of its plan leaves out the figures of that code and of the application's, and where each came from, by one table of the plan's lines, and keeps the version of what generated it, on the hosted playground as on a machine. A figure the generated code itself reads is another matter, the port's lead, which it adds to its read, and the line and priority it writes to the controller: a plan where either is owed generates no code until a record states it.

Time on a machine is counted from a read: the last step of initialisation reads the counter and counts every release from that read plus the port's
bound on everything from it to the first enabling of interrupts, so no release stands before interrupts are on, and the checker holds that one
write of the timer reaches that far, the port's lead and the timer's rounding added. What initialisation arms for an alarm it arms before that read, so that nothing of the
generated code's own lies between the read and the enabling but the two calls into the port's code, which the port's bound counts, a limit the
record states. A wake armed there must not come due before interrupts are on, so its period
outlasts what the generated code states of that step and the port's bound together, which the checker holds; on the hosted playground the
simulator turns interrupts on at time zero, where its wakes are armed, so nothing is held there. Every other alarm's compare is given a far-off
placeholder in that step, at the farthest point on its compare's step that one write reaches, and the checker holds that it too lies past the
enabling, counting each read, and each delay's write, from a tick of its own counter before it and the placeholder a whole step short of what one
write reaches. And an alarm's deadlines are read as its
own counter's time, which wraps, where it has no reader or one not bound at an extender over that same device, and as the epoch's where each
reader of its time is.

The record's formulas are not left to reading. A model beside it, `crates/archogen-plan/tests/format2_model.rs`, runs each one: where the image may lie and
where each stack is laid, against a reference that tries every address in turn, over every small region; each stack's sum, read from the record's own table,
against what can stand on that stack at once; and the read that time is counted from, at every phase of the counter's tick. Where a rule is already
code, what the joint rules refuse of a memory map, the model is held to that code and not to a reading of it.

## Format `/2` as it becomes code

`/2` becomes code a part at a time (`M4.1.7.6`), and each part is held to the model the same way. Where the image may
lie, where each allocation lies, what an owed one waits on, each stack's sum and what its figures rest on are
`crates/archogen-plan/src/layout.rs` (leaf `M4.1.7.6.1`). Every small region, unit and set of figures the model runs, and
each of its rounds' probes, runs through that code too, and the two must agree. It counts addresses past 64 bits, so a
region ending at the top of a 64-bit address space is laid whole. The code identity and the catalog's digest are
`identity.rs`; the routes' ranks, which reach the controller, the priority each takes and the `(enabled …)` line are
`controller.rs`; an alarm's time base, what it owes, how its compare is read, a rate and the epoch's line are `alarm.rs`
(leaf `M4.1.7.6.2`), each held to the model over every small input. No command writes a `/2` plan yet.

## From a plan to a generated system

`crates/archogen-emit` turns a plan into the files of a package that builds and runs by itself, and `archogen build` is the
command that does it ([Building a system](building.md)). A plan is accepted only by lowering it again from the same inputs and
comparing bytes, so a plan that reached the emitter by any route is what the checked inputs make, or nothing is generated.

## What this does not establish

The plan is a statement of intent, and checked input for the generator. It does not establish that the system meets a
deadline, which is the scheduling checker's to say, under its own assumptions ([analysis](analysis.md)), nor that any
generated code does what the plan says, which `M4.4`'s checks of the built image answer.
