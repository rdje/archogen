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
  (offers (tick-rate 10 MHz) (region target.timer (base 0x0200_0000) (size 64 KiB))))
(defblock target.console
  (offers (observable-output true) (region target.console (base 0x1000_0000) (size 256 byte))))
(defplatform target.riscv-virt-up
  (offers (core-count 1 tick) (region target.ram (base 0x8000_0000) (size 128 MiB) (executable true)))
  (requires (uses target.timer target.console)))
```

`cargo xtask target-agreement` compares each fact with the device-tree node it names, and matches the node by
what it is, not by address alone:
- the core count with the harts under `/cpus`;
- the executable region with the `memory` node;
- the timer's region with the CLINT, and its tick rate with the timebase;
- the console's region with the node `/chosen`'s `stdout-path` names.

A region that no rule compares is a disagreement too. `archogen check` must also admit the file. Every `--check`
runs the agreement on the same fresh dump it compared with the recorded tree:

```console
$ bash scripts/target_emulator.sh --check
target-emulator: found: QEMU emulator version 11.1.1
target-emulator: the platform presented matches docs/targets/riscv-virt-up.dtb.summary.md
target-emulator: the §3.2 agreement holds: targets/riscv-virt-up.eadl agrees with the platform presented
```

The description claims no instruction set, no interrupt route and no other device. Those would be facts
nothing checks, and the interrupt timings the runtime analysis needs are the catalog's, with their evidence.

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
