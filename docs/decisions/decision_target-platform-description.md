# A real target is described in `eadl/1` as it stands, and checked against the device tree it came from

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **External sources:** [QEMU](../book/src/ledger.md#qemu) — version, scope and limits in the ledger
- **Owner / source:** leaf `M2.8.3.1` (`docs/tasks/M2.md`), deciding what `M2.8.3` warned "should not be improvised":
  how `ROADMAP.md` §3.2's "eADL platform fixture" for `riscv-virt-up` is written, and what its agreement with
  the measured platform means.

## The fact / decision

The target is described in **`targets/riscv-virt-up.eadl`**, beside `targets/riscv-virt-up.env`, which names it as
`PLATFORM_DESCRIPTION`. It uses only kinds and vocabulary the corpus already uses: `defblock`, `defplatform`,
`offers`, `requires (uses …)`, and the `region` form of `docs/semantics/boundary/accept/addressable-region.eadl`.
**No language change.** `eadl/1` is frozen (`M1.13`), and describing a real target must not quietly redefine
`defplatform`.

### 1. What it says, one fact per profile requirement

`targets/riscv-virt-up.env` names what `rt-static-up-v1` requires of a target (§3.1): `REQUIRES_CORES`,
`REQUIRES_TIMER`, `REQUIRES_OBSERVABLE_OUTPUT`, `REQUIRES_EXECUTABLE_RAM` and `REQUIRES_PRIVILEGE`, and what format `/2` reads of
a target, `REQUIRES_RELEASE_TIMER`, `REQUIRES_CATALOG_TARGET`, `REQUIRES_FIRMWARE_TRAPS` and `REQUIRES_NON_MASKABLE_INTERRUPTS`
(`decision_runtime-plan-format-2.md` §2, `M4.1.7.3.3`, `M4.1.7.3.4`). The description states each, and nothing it cannot check.

| Requirement | In the description | Compared with (device tree) |
| --- | --- | --- |
| `REQUIRES_CORES=1` | the platform offers `(core-count 1 tick)` | the number of `/cpus/cpu@…` nodes whose `device_type` is `cpu` |
| `REQUIRES_EXECUTABLE_RAM=yes` | the platform offers `(region target.ram (base …) (size …) (executable true))` | the node whose `device_type` is `memory`: its `reg` base and size |
| `REQUIRES_TIMER=yes` | a block, used by the platform, offering `(tick-rate …)` and `(region target.timer (base …) (size …))` | `/cpus`'s `timebase-frequency`; the node compatible with `sifive,clint0`: its `reg` base and size |
| `REQUIRES_OBSERVABLE_OUTPUT=yes` | a block, used by the platform, offering `(observable-output true)` and `(region target.console (base …) (size …))` | the node `/chosen`'s `stdout-path` names, compatible with `ns16550a`: its `reg` base and size |
| `REQUIRES_PRIVILEGE=machine` | each block and the platform offer `(reachable-at-privilege machine)` | no node: the [privileged specification](../book/src/ledger.md#riscv-privileged)'s reset and PMP sentences, and the spike run at M (`M3.3.4.4`) |
| `REQUIRES_RELEASE_TIMER=yes` | the tick-rate's block offers what the release requester asks of it (`decision_release-alarm.md` §1, `RA-H8`): `(relative-delay true)`, `(channels 1 tick)`, `(alarm-wiring core)`, `(alarm-kind level-compare)`, `(counter-modulus (pow2 64))`, `(compare-step 1 tick)`, `(compare-magnitude true)`, `(observation-coherent true)`, `(available-in-state run idle)` and `(supported-horizon …)` | the CLINT's `interrupts-extended`: its machine-timer lines, cause 7, their count and whether each reaches a hart's `riscv,cpu-intc`; the [privileged specification](../book/src/ledger.md#riscv-privileged)'s machine timer, which the node compatible with `riscv,clint0` is, for the modulus, the compare's kind, step and magnitude and the states, and on a hart whose `riscv,isa-base` is `rv64i` for the coherent read; and the horizon at most what one compare reaches, `2^64 − 1` ticks of `/cpus`'s `timebase-frequency` |
| `REQUIRES_CATALOG_TARGET=yes` | the platform offers `(catalog-target NAME)` | no node: the `.env`'s `TARGET_ID`, its stem |
| `REQUIRES_FIRMWARE_TRAPS=yes` | the platform offers `(firmware-traps 0)` | no node: the `.env`'s `QEMU_BIOS=none`, which starts the image with no firmware beneath it; any other count nothing here checks |
| `REQUIRES_NON_MASKABLE_INTERRUPTS=yes` | the platform offers `(non-maskable-interrupts 0)` | each hart's `riscv,isa-extensions` and `riscv,isa`, naming no `smrnmi`; and the `.env`'s `QEMU_VERSION_PINNED`, the release whose source was read: its hart takes no non-maskable interrupt but Smrnmi's, and no device connects one ([`qemu`](../book/src/ledger.md#qemu)); any other count nothing here checks |

A region names **its device by what it is**, not by address alone. The RAM region must match a `memory` node, the
timer's the CLINT, the console's the node `stdout-path` names. So a region that happened to share an address with
another device still disagrees.

### 2. What it does not claim

- **The ISA.** The hart offers `rv64imafdch` and more, and archogen builds `riscv64imac` (`M2.8.2`'s summary). The
  description claims no ISA facts. The build's subset is the `.env`'s, which leaf `M2.8.3.3` moves there.
- **The interrupt route's delivery bounds.** The CLINT delivers machine-timer interrupts to the hart
  (`interrupts-extended`), which the description states as `(alarm-wiring core)` and its line, `RA-H8` giving way here
  for the wiring; the runtime analysis needs delivery bounds (`decision_runtime-analysis-variant.md`), and those are the
  catalog's (`M2.7`), with their evidence, not a description's.
- **A count of non-maskable interrupts on another release.** A `riscv,isa` without `smrnmi` says only that none
  resumes (`decision_runtime-plan-format-2.md` §2, RP13-1); the count of none rests too on the pinned release's own
  source (`M4.1.7.3.4`), so a moved pin refuses it until the new release is read.
- **Every other device.** The PLIC, the RTC, `virtio` and the rest are present, and nothing relies on them. Claiming
  them would be facts nothing checks.

### 3. The agreement

`cargo xtask target-agreement` reads the description with `eadl-front`, and the device tree with `xtask`'s own
parser (`xtask/src/dtb.rs`), from `targets/riscv-virt-up.env`'s `PLATFORM_DESCRIPTION` and a device-tree file. It
fails, naming the fact, when:
- a requirement is missing its fact;
- a fact is missing its node;
- a number differs;
- the description offers a region, an offer or a clause the rules above do not read (`M3.3.4.4`).

Every value is compared exactly, in the device tree's own integers after the description's quantities are
converted (a `MiB` is 1 048 576 bytes, a `MHz` is 1 000 000 Hz), but a capacity, which a target states at most what its
source says, `RA-H8` giving way here: `supported-horizon`, held within what one compare reaches, since `2^64` ticks at
10 MHz is no number the language reads. A unit test runs it over the kept dump,
`docs/targets/riscv-virt-up.dtb`. The emulator step of the `integration` tier runs it over a fresh dump, after
`dtb-check`, so a QEMU release that moves the platform fails there. `archogen check` must also admit the
description against `rt-static-up-v1`.

### 4. What flips `TARGET_VERIFIED`

`TARGET_VERIFIED=yes` only when the agreement holds on a fresh dump from the pinned release. The leaf that flips
it (`M2.8.3.4`) records that run as its evidence, and lifts the emulator step's quarantine in the same change,
because a quarantine whose step passes is itself a failure.

## Why

- **The corpus vocabulary, not a new one.** Every form the description uses already has meaning in `eadl/1`.
  The one form a real target needs that the corpus lacked is `(executable true)`, a sub-form of `region`. It is the
  same shape as `addressable-region.eadl`'s `(reserved true)`, and `offers` holds forms, so it adds a word, not a
  construct.
- **Name the device, not the address.** An address match alone would call two devices at one address an
  agreement.
- **Claim only what is checked.** A fact the description states and nothing compares is an assertion, which is
  what the `2026-09-28` ruling reframed `M2.8` to end.

## How to apply

- A second target is a second `.eadl` beside its `.env`, checked the same way. A requirement the profile adds is a
  row of §1, with its comparison rule, before its fact is written.
- A fact the runtime analysis needs (delivery bounds, costs) is the catalog's, with its §7.3 evidence. It never
  goes into a description because the device tree happens to show part of it. A compare's facts, its step, its window
  or magnitude, its kind and its wiring, are the target's, each with what checks it, as `RA-H8` hands them.
