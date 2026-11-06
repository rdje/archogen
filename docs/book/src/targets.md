# Where generated systems run

## The idea, in plain words

A generated system has to run somewhere, and archogen uses three places, one after another. First your own
computer, where the system's decisions are tested quickly, with no real hardware involved. Then an **emulator**:
QEMU pretends to be a RISC-V computer, so the real target code runs — starting up, taking interrupts — on hardware
that is only simulated. Last, a **board**: a real microcontroller on your desk. Each place can tell you some things
and not others. The emulator runs the real code, but its timing is not a real chip's, so only a board's own
measurements can back a timing claim about that board.

**Which board?** The first will be a RISC-V microcontroller board, the kind archogen's runtime is built for, and none
has been chosen yet: that is milestone `M5`, the director's decision. To give an idea of the kind, the
[RP2350](ledger.md#rp2350) chip of the Raspberry Pi Pico 2 carries RISC-V cores beside its Arm ones — "Hazard3
(`RV32IMAC+`) processors" in its datasheet. Such cores are 32-bit, where archogen's emulated target is 64-bit today,
so a first board brings a port of its own. Boards built on another processor family, such as Arm Cortex-M
microcontrollers, would each need another port, which nothing plans yet; and a full computer built to run a
general-purpose operating system is a different class of machine from the devices this profile describes.

> **In one minute, for engineers.** `hosted-playground` tests the shared runtime logic deterministically;
> `riscv-virt-up` runs real target binaries on QEMU's `virt` machine at a pinned release and command line, its device
> tree checked against the platform's eADL description, a missing emulator reported rather than skipped; `board-first`
> is the same profile on a named board with its own measurements. The first code on the emulated target is a
> hand-written timer-interrupt measurement, not a generated system. No board exists yet.

## How it works

Three environments, with different jobs and different limits. Confusing them is how a project
comes to believe it has hardware evidence.

| Environment | Job | What it cannot tell you |
| --- | --- | --- |
| `hosted-playground` | fast deterministic testing of the shared runtime logic and explicit device models | nothing about target execution speed; preemption only at simulated boundaries |
| `riscv-virt-up` | independent execution of real target binaries — startup, interrupt paths, MMIO contracts | it is a virtual platform, not a board and not a cycle-accurate timing reference |
| `board-first` | the same profile on a named physical processor and device revision | only what its documented configuration and measurement procedure support |

**The hosted playground is a simulator, not an emulator** (`crates/sim-playground`, `M4.3.1`). No target code runs on it. It is
a small program that executes the runtime's real scheduling logic, `crates/rt-core`, against modeled time, and writes down
every event as it happens: an interrupt becoming pending, its service, each release, each job starting, being preempted,
resuming and completing. It has one rule for what happens first when several things fall on one instant, so one system has
one trace, and two runs can be compared byte for byte:

```text
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
```

Time is exact: `1/100 s` is 10 ms, with no rounding anywhere. In order: a job with nothing left that takes time finishes first, so
one that ends exactly as its next release falls due has not overrun; then an interrupt that has arrived is taken as soon as the
hardware is unmasked, the routes in the plan's order and then the release timer, none nesting another; then the scheduler
decides; then the running job goes on. While a job holds interrupts masked, an arrival stays pending, and its service runs at
the unmask, before the job's next instruction, even if that is its last. A job's body is data, a list of *run this long*, *mask*, *unmask*. Until an application's own inputs exist, a
generated system's bodies are empty, so its jobs start and complete at one instant, and only the releases are informative;
the simulator's own tests run longer bodies, so preemption, masking and overrun are held today.

When something goes wrong the trace says so in the words the fault paths' record, `docs/profiles/rt-static-up-v1-faults-observation.md`,
fixes, so a hosted run and a target's can be compared: `1/50 s fault overrun late skip-late-job nominal 1/50 s` is one late job abandoned,
that release becoming the task's next job, with no separate `release` line. A task that leaves the schedule says `fault` once, and its
later releases say nothing. A halt says its kind, its case and whom it was attributed to, and nothing follows it.

**A timer and an interrupt controller, as contracts you can run** (`crates/sim-devices`, `M4.5`). The roadmap asks a timer's contract to say
what happens when its counter wraps, when a deadline is too far ahead to tell apart, and when a deadline has already passed by the time it is
programmed. These models make each of those something you can execute: a deadline past the counter's horizon is refused rather than quietly
becoming a nearer one, a deadline already passed is raised at once and says how late, and an interrupt that arrives while masked waits and is
delivered at the unmask. Their tests walk every case at small counter sizes. They do not run in a generated system yet: the playground's own
release timer is an exact clock with no counter to wrap, so they wait for a plan that names a target's devices.

## The precise rules

### The emulator is pinned

[QEMU](ledger.md#qemu)'s `virt` machine is a *configurable* virtual platform: its device set and the device tree
it generates depend on the release and the options. An unpinned invocation silently changes the
platform under an unchanged description — which invalidates every observation taken against it
without invalidating anything anyone can see.

So the configuration is data, and one tool renders it:

```console
$ scripts/target_emulator.sh --print build/uc1.elf
qemu-system-riscv64 -machine virt -cpu rv64 -smp 1 -m 128M -bios none -nographic -kernel build/uc1.elf
```

`-bios none` is deliberate: the generated image boots directly, with no firmware whose behavior
has not been described.

And the platform the emulator *actually* offers is checked, not assumed —
`scripts/target_emulator.sh --dump-dtb` writes QEMU's generated device tree for comparison
against the eADL platform fixture. That is the difference between "we described a platform" and
"we described *this* platform".

### Absence is reported, never skipped

QEMU is installed here. This is the same `--check` run with it left off `PATH`, which is how a machine without it
sees the step:

```console
$ PATH=/usr/bin:/bin:/usr/sbin:/sbin:$HOME/.cargo/bin bash scripts/target_emulator.sh --check
target-emulator: required tool unavailable: qemu-system-riscv64 is not on PATH
target-emulator:   the riscv-virt-up environment cannot be exercised on this machine
target-emulator:   install it, then re-run; do NOT record an emulator result without it
$ echo $?
20
```

A required tool that is unavailable is reported as unavailable. It never becomes a passed
check by way of a skipped one.

The configuration carries `TARGET_VERIFIED`. It was `no` until an installed QEMU had confirmed everything, and it
has been `yes` since leaf `M2.8.3.4`. `--check` keeps its answers apart:
- A release other than the pin, a machine the emulator does not offer, a device tree that differs from the one
  recorded, or a description that disagrees with the platform is a comparison that ran and disagreed: exit `1`.
- `yes` with no description to agree with is refused too, exit `1`: a flag that says verified needs the comparison
  that verifies it to have run.
- A target whose agreement holds but whose flag is still `no` could not be declared verified: exit `20`.

[The verification chapter](verification.md) has the terms.

The recorded device tree is `docs/targets/riscv-virt-up.dtb.summary.md` (leaf `M2.8.2`). It was
dumped from the pinned emulator, rendered as text by `cargo xtask dtb-summary`, and kept beside the
dump it came from, with a short account of which nodes the runtime stands on. Those are the one
hart, the RAM at `0x8000_0000`, the CLINT timer and its route to the hart, and the `ns16550a`
console. Every `--check` dumps the tree again and compares it with the file. So a QEMU that changed
the platform under the pinned options would fail the check rather than silently invalidate what was
measured against it. One property is left out of the comparison, by name: a random seed QEMU writes
at every boot. The file also records a deliberate gap. The hart offers floating point, and archogen
builds for `riscv64imac`, without it.

### The target, described in eADL, and checked against the device tree

§3.2 asks for agreement between the target and its "eADL platform fixture". That fixture is
`targets/riscv-virt-up.eadl` (leaf `M2.8.3.2`, `docs/decisions/decision_target-platform-description.md`). It uses
only words the language already has, and states one fact for each thing the profile requires of a target:

```text
(defblock target.timer
  (offers (tick-rate 10 MHz) (reachable-at-privilege machine)
    (region target.timer (base 0x0200_0000) (size 64 KiB))))
(defblock target.console
  (offers (observable-output true) (reachable-at-privilege machine)
    (region target.console (base 0x1000_0000) (size 256 byte))))
(defplatform target.riscv-virt-up
  (offers (core-count 1 tick) (reachable-at-privilege machine)
    (region target.ram (base 0x8000_0000) (size 128 MiB) (executable true)))
  (requires (uses target.timer target.console)))
```

`cargo xtask target-agreement` compares each fact with the device-tree node it names, and matches the node by
what it is, not by address alone:
- the core count with the harts under `/cpus`;
- the executable region with the `memory` node;
- the timer's region with the CLINT, and its tick rate with the timebase;
- the console's region with the node `/chosen`'s `stdout-path` names;
- each block's and the platform's level with the `.env`'s `REQUIRES_PRIVILEGE=machine`, which no node carries: a hart
  starts in M at reset, an M-mode access that no locked PMP entry matches succeeds
  ([privileged specification](ledger.md#riscv-privileged) §2.1.4, §2.1.7.1.3), and the spike runs at M, programming
  the CLINT and writing to the console (leaf `M3.3.4.4`).

A region, an offer or a clause that no rule reads is a disagreement too, and a region's sub-forms follow the joint
rules' grammar: each once, `executable` `true` or `false`, and `reserved` refused, since no rule compares it. `archogen check` must also admit the file. Every `--check`
runs the agreement on the same fresh dump it compared with the recorded tree:

```console
$ bash scripts/target_emulator.sh --check
target-emulator: found: QEMU emulator version 11.1.1
target-emulator: the platform presented matches docs/targets/riscv-virt-up.dtb.summary.md
target-emulator: the §3.2 agreement holds: targets/riscv-virt-up.eadl agrees with the platform presented
```

The description claims no instruction set, no interrupt route and no other device. Those would be facts
nothing checks, and the interrupt timings the runtime analysis needs are the catalog's, with their evidence.

### A system on a target: which device is which

A description names devices of its own, and a target has others: `console.uart` on a playground, `target.console` on the emulator. Nothing says the two
are one device, and nothing has to, because a description says what it asks for. The engine gives the system the target's platform in place of its own and
searches again. The device then bound for "`console.write` needs observable output" is the device `console.uart` was
(`docs/decisions/decision_device-correspondence.md`; `crates/eadl-resolve/src/target.rs`).

The description's own hardware is then gone from what is read, so nothing the target replaces can answer for it. What the system does not reach neither asks nor answers, and a service's offer answers nothing there, since only a device can serve. So everything a target build says of a device is the target's. A description's rate, width and regions do not reach it, each requirement is judged again at
the target's providers, what a system's `platform` clause asks beside its hardware among them, and the joint rules read the target's memory map. One device may become two, two may become one, and an adapter a better device no
longer needs is gone. What no device can serve is the refusal any system gets.

The engine refuses these before it searches, because no requirement can answer them:

- a name both the description and the target declare;
- a file naming what only the other declares, since each must say what it holds by itself: read together, the target's name would find the description's device and bind it;
- a system, a task, or a service or policy the system reaches, naming a block or a platform itself, where it could say what it asks of one;
- a fact the one declaration a system's platform clause stands on declares absent that the target offers. Another's absence covers what it is made of, and says nothing of a device beside it;
- two `platform` clauses on one system, which read as one would hide that it has two;
- hardware that refines what leads back to it, on which no machine stands.

A target's file holds what its agreement with the machine checks, so what it does not state it does not offer. A system that declares a fact absent lands on a target that is silent about it, even where the machine has the device.

⚠️ Two devices no requirement tells apart are not told apart: the policy's order chooses. And this says what is judged, not that two runs match, which is
`M4.9`'s. No command names a target yet: that is `M4.1.7`, with the plan format that reads this rule.

### What releases a task on a target

On the playground the simulator's clock releases tasks. On a real machine a device has to raise that interrupt, so the kernel asks for an alarm as anything else asks, and the search binds it to a
device that can serve, under the same rules and the same count of channels (`docs/decisions/decision_release-alarm.md`). It asks one that runs while the machine idles, and wired so that the core takes its
interrupt with nothing to claim. A second alarm, an adapter's say, takes a channel of its own, and is wired through the interrupt controller. How late an alarm may be delivered, how long between two reads of its counter and how far ahead it may stand are three sums in that record's table, which the model that holds them reads and runs. Neither this nor what follows is code yet: format `/2` and the
scheduling checker build them.

Each alarm's interrupt is charged as any other interrupt source is, by the analysis as it stands, so what has to be right is what it is told: how close two arrivals can come. Nobody can state that of what a job
may program, so the code writing the alarm enforces it, holding each compare a least distance past the last arrival, as far as it can know that, which costs a deadline about that much lateness. `crates/rt-analysis/tests/alarm_model.rs` runs that design on a timeline against the analysis: with the floor no response passes its bound, and without it a set that holds on paper misses. Its rank among the others goes by the deadline it has to meet, so an alarm due soon stands ahead of a slow source. A call into generated code counts as a call of a primitive, whose cost holds
what it calls in turn. And a job runs in no interrupt, so a job hears of an alarm when a task is released, which something else must have programmed for it, since a set of tasks that only release each other has no first job.

### The first code on the target

`M2.8` asks for an architecture spike: startup, a timer interrupt, the return from it, observable output, and
context preservation, run in the emulator. `targets/riscv-virt-up/spike/` is that image (leaf `M2.8.4`). It is a
small bare-metal program built for the `.env`'s `RUST_TARGET`, with no dependencies. It stays off the generation
path; nothing archogen generates depends on it. Every address it touches is one the recorded device tree names:
the RAM, the CLINT, the UART and the test device that powers the machine off.

It starts in machine mode, installs a trap handler, arms the CLINT one millisecond ahead, and waits. The wait holds
a known value in every register it can, callee-saved and caller-saved alike, because an interrupt can arrive
between any two instructions and the handler must restore all of them. When the interrupt returns, it compares
every one. Then it powers off through the test device, so QEMU's own exit status is the verdict.

A context check that had never seen a corrupted context would pass for a handler that restored nothing. So the
same image, built with its `clobber` feature, corrupts one register on the way out of the trap, and must be caught.
`scripts/target_spike.sh` builds and runs both, with every QEMU option from the pinned `.env`, and is the `spike`
step of the `integration` tier:

```console
$ bash scripts/target_spike.sh
target-spike: spike: boot on riscv-virt-up
target-spike: spike: timer armed
target-spike: spike: machine-timer interrupt taken
target-spike: spike: interrupt returned
target-spike: spike: context preserved in 23 registers
target-spike: spike: ok
target-spike: the negative control was caught: a clobbered register reports `context CLOBBERED`, status 2
```

## Today and ahead: there is no board

**No physical board has been selected or procured.** Physical-target evidence is blocked, and
that is recorded as a programme risk rather than left to surface at the release.

All seven facts the roadmap requires — board and revision, processor and revision, debug
interface, executable memory region, clock configuration, available specifications, access
path — are recorded as `unrecorded`. Naming a plausible board from memory would be worse than
naming none: each of those rows is a fact a later timing claim would rest on, and a board named
without its datasheet in hand is a proposal wearing a fact's clothes.

What this blocks: physical execution, and any claim of target support. What it does **not**
block: everything through M4, which the hosted playground and the emulator carry between them.

The selection criteria are written out in `docs/targets/first-target.md`, so the decision is
ready the moment a board is. The failure this record exists to prevent is the quiet one —
arriving at a release with the board row still empty and the documentation describing support
for a target nothing ever ran on.
