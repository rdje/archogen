# A tour: from a description to a board

Before the details, the destination. This chapter follows one small system from the words that
describe it to the program that runs it, says plainly which parts are real today and which are
still being built, and ends where the project is heading: the same description, running on a
microcontroller board on your desk.

> ⚠️ **Read the status before the promise.** What runs today is an *experimental* path that
> rehearses the whole pipeline on your computer. It makes no timing claim and touches no real
> hardware. Nothing in this book claims a verified operating system; every claim says what it
> rests on.

## 1. Describe what you want

You do not write an operating system. You describe what you need: the platform's functionality,
the services your tasks use, and the tasks themselves — when they run, how urgent they are, and
what happens if one runs late. This is `examples/s0-heartbeat/system.eadl`, with its comments
removed:

<!-- verbatim: examples/s0-heartbeat/system.eadl -->
```text
(eadl-version eadl/1)
(defblock console.uart
  (offers (observable-output true)))
(defplatform host.playground
  (offers (core-count 1 tick))
  (requires (uses console.uart)))
(defservice console.write
  (requires (needs observable-output)))
(defsystem heartbeat
  (task beat
    (period 10 ms) (deadline 10 ms) (deadline-from release)
    (priority 1) (uses console.write) (on-overrun fault))
  (task chime
    (period 30 ms) (deadline 30 ms) (deadline-from release)
    (priority 2) (uses console.write) (on-overrun fault))
  (requires (uses console.write))
  (platform (uses host.playground)))
```

Read it as a list of facts and requirements:

- a **UART** that can show output, on a single-core **platform**;
- a **service**, `console.write`, which needs that output;
- a **system**, `heartbeat`, with two tasks. `beat` is released every 10 ms and must finish within
  10 ms; `chime` every 30 ms, within 30 ms. `beat` is the more urgent (priority 1 outranks 2), and
  if either is still running when its next release arrives, the overrun policy `fault` takes that
  task out of the schedule while the other continues.

There is no code in it, no algorithm and no register address. That is the language's rule, not a
style: eADL describes *what* is required, and the engine decides *how*
([The boundary](boundary.md)).

## 2. Check it, build it, run it

`archogen check` reads the description against the supported profile, `rt-static-up-v1`, and
either accepts it or says exactly what is wrong and how to fix it ([Checking a description](checking.md)).

`archogen build` then generates a system. Today that is the **S0 path** ([The S0 early generation
path](s0.md)): a small Rust program — a `main`, a runtime, the `console.write` service — plus a
provenance file naming what it was generated from. Run it, and it prints the schedule over one
hyperperiod, the 30 ms after which the pattern repeats:

<!-- verbatim: examples/s0-heartbeat/expected/system.txt -->
```text
system heartbeat
release 0 ms beat
release 0 ms chime
release 10 ms beat
release 20 ms beat
summary hyperperiod 30 ms releases 4
```

At 0 ms both tasks are released, and `beat` comes first because it outranks `chime`. `beat` runs
again at 10 and 20 ms; `chime`'s next release is at 30 ms, the start of the next hyperperiod.

⭐ **This output was written down before the generator existed.** It is the frozen expectation
`examples/s0-heartbeat/expected/system.txt`, committed with its rationale while no line of the
emitter had been written. A test — F28's, in `crates/archogen-cli/tests/s0_oracle.rs` — generates
the program from a clean directory, compiles it, runs it and compares what it prints with that
file, byte for byte; and `crates/archogen-cli/tests/book_tour.rs` compares this chapter's copies
of both files with the files themselves.

## 3. What is real underneath, today

The S0 program is a rehearsal. The parts of the real system are being built and checked one at a
time:

- **The runtime's core**, `rt-core` ([The runtime](runtime.md)): the fixed-priority scheduler and
  the fault path, as a state machine that decides what happens next while the board port
  performs it. It builds for bare-metal RISC-V with no operating system beneath it, and it agrees
  with a second runtime model, written independently from the written contract alone, on every
  randomised sequence of releases, critical sections and faults the comparison generates.
- **The fault contract** (`docs/profiles/rt-static-up-v1-faults.md`): what happens when a task runs
  late, when a trap fires, when a stack overflows. It is being reviewed independently, round after
  round, and it is not finished until two careful readers can no longer build different systems
  from it — which, at the time of writing, they still can.
- **The scheduling check** ([What the scheduling checker establishes](analysis.md)): response-time
  analysis, the calculation that says whether every task meets its deadline in the worst case, and
  under which assumptions.
- **The catalog** ([The catalog](catalog.md)): the reviewed building blocks a generated system is
  assembled from, each identified by a hash of its content, so a claim can say exactly which code
  and which review it rests on.
- **A first program on an emulated RISC-V board** ([Where generated systems run](targets.md)):
  under [QEMU](ledger.md#qemu), it boots, takes a timer interrupt, saves and restores the
  processor's registers, and returns. It is a hand-written measurement of the target, not a
  generated system: it shows the board-level steps work on that machine, and nothing archogen
  generates runs there yet.

## 4. What it becomes

The same description will produce:

- a **bare-metal image** for QEMU's RISC-V machine first, then for a physical board: startup code,
  the scheduler, a timer driver that releases each task on time, the UART console, and your task
  code — and nothing the description did not ask for;
- a **simulator** of that system, whose traces are compared with the emulator's;
- a **report** that says what was established and under which assumptions — for example, that
  `beat` finishes within its 10 ms in the worst case *given* the execution bounds its code was
  measured or analysed to have.

## 5. What you could build with it

Anything where *when* something happens matters as much as *what* happens: a control loop for a
motor or a drone that must run every few milliseconds, sensors sampled at fixed rates, a safety
monitor that must react within a bound, small industrial or medical devices. These are the jobs
statically configured kernels do in cars — the [OSEK](ledger.md#osek-os) and
[AUTOSAR](ledger.md#autosar-os) family — and their counterparts in aircraft. The difference archogen
aims for is that the timing promise comes with its evidence and its assumptions written down, rather
than hoped for or tested into confidence.

What a generated system deliberately is *not*: a general-purpose operating system. There are no
processes, files, sockets, `fork`, heap or tasks created at run time in this profile, and each is
excluded by name with the guarantee it would break ([The supported profile](profile.md)). A POSIX
interface is on the roadmap — last, as a named subset layered on top, once the analysable
foundation underneath it exists.

## 6. How you interact with it

A generated system is used the way a device is used, not the way a computer is: you drive its declared
inputs, watch its declared outputs, and look inside with a debugger.

- **Inputs are events that wake a task.** A button, a sensor's "data ready" line or a byte arriving on the
  serial port is an interrupt source the description declares, with the shortest time between two of its
  events; the task it wakes has a deadline to respond within. This description from the language's test
  corpus, `docs/semantics/cases/positive-sporadic-release.eadl`, is accepted by the checker today:

  <!-- excerpt: docs/semantics/cases/positive-sporadic-release.eadl -->
  ```text
  (defblock event.line (offers interrupt-source min-arrival-separation))
  (defsystem s
    (task handler (min-separation 20 ms) (deadline 15 ms) (deadline-from release) (jitter 1 ms)
                  (priority 1) (uses event.release))
    …)
  ```

  It reads: events arrive at least 20 ms apart, and each one wakes `handler`, which must respond within
  15 ms. The one rule is that every input says how often it can arrive — an undeclared input would break
  every timing promise silently, so the profile refuses it by name (`unmodeled-interrupt-load`).
- **Outputs are what the description declares** — the serial console first; lights, motors and displays as
  further device services.
- **Looking inside** is a developer's tool: a debugger, which QEMU can host and a physical board offers through
  a debug probe — accessible debugging is one of the roadmap's criteria for choosing a board — and, after a fatal
  fault, the record the runtime keeps of what happened and to which task.

⚠️ Today the generator runs only periodic tasks, on your computer. Event-driven input on the emulator and on a
board is part of the work ahead, and the repository does not yet attach a debugger to the emulator.

**No login, no shell, no files — by design, for now.** A system in this profile has no users to log in and no
shell: it boots straight into its tasks. The way to play with it is the description itself — change a period, add
a task, give one an impossible deadline, make one overrun on purpose, then rebuild and read what changed in the
output and in the report. A small command console, a task that reads the serial port and answers bounded commands,
would fit the profile's rules, but nothing like it is planned yet. A filesystem is excluded from this profile by
name: it brings power-loss recovery, persistence and memory use this profile does not analyse. The roadmap lists
filesystems, with networking, among the later feature families; which kind is not decided, and the obligations it
names point toward the small, power-loss-safe designs microcontrollers already use rather than a desktop's.

## 7. The road to a board on your desk

A generated system needs very little from a board: a processor core, memory, a timer, an interrupt
controller and a serial port. That is **microcontroller** territory — development boards that
typically cost tens of dollars, not computers that run an operating system of their own. The generated image *is*
the operating system.

The roadmap sets what a first board must offer: accessible debugging, simple clocks, memory that
code can run from, and documented timing behaviour — and it warns that a board good enough to
bring a system up may not be good enough for timing claims. The path, in order:

1. **the hosted playground** — today, on your computer, as above;
2. **`riscv-virt-up`** — the emulated RISC-V machine, where target binaries already boot;
3. **`board-first`** — the same profile on a named physical board, with the board's own
   measurements behind every timing claim.

The second step is under way; the third waits on one decision, which board to buy, and on the work
this book tracks between here and there.

## Where to go next

- How a description is written: [Reading a description](reading.md) and [Describing a
  workload](workload.md).
- What the engine is allowed to claim: [What a report may claim](evidence.md).
- How the toolchain checks itself: [Verifying the toolchain](verification.md).
