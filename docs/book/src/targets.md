# Where generated systems run

Three environments, with different jobs and different limits. Confusing them is how a project
comes to believe it has hardware evidence.

| Environment | Job | What it cannot tell you |
| --- | --- | --- |
| `hosted-playground` | fast deterministic testing of the shared runtime logic and explicit device models | nothing about target execution speed; preemption only at simulated boundaries |
| `riscv-virt-up` | independent execution of real target binaries — startup, interrupt paths, MMIO contracts | it is a virtual platform, not a board and not a cycle-accurate timing reference |
| `board-first` | the same profile on a named physical processor and device revision | only what its documented configuration and measurement procedure support |

## The emulator is pinned

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

## Absence is reported, never skipped

```console
$ scripts/target_emulator.sh --check
target-emulator: required tool unavailable: qemu-system-riscv64 is not on PATH
target-emulator:   the riscv-virt-up environment cannot be exercised on this machine
target-emulator:   install it, then re-run; do NOT record an emulator result without it
$ echo $?
20
```

A required tool that is unavailable is reported as unavailable. It never becomes a passed
check by way of a skipped one.

The configuration also carries `TARGET_VERIFIED=no` until an installed QEMU has confirmed it.
A proposal recorded as a proposal is a fact about what is known; the same line marked verified
with nothing behind it would be a fabricated fact.

With QEMU installed, `--check` keeps two answers apart. A release other than the pin, a machine
the emulator does not offer, or a device tree that differs from the one recorded is a comparison
that ran and disagreed: exit `1`. A
configuration that matches, whose §3.2 agreement check has nothing to compare against yet, could
not be run: exit `20`, the same code as the absent tool, and the runner accepts it only because a
quarantine names its owner, `M2.8` — [the verification chapter](verification.md) has the terms.

The recorded device tree is `docs/targets/riscv-virt-up.dtb.summary.md` (leaf `M2.8.2`). It was
dumped from the pinned emulator, rendered as text by `cargo xtask dtb-summary`, and kept beside the
dump it came from, with a short account of which nodes the runtime stands on. Those are the one
hart, the RAM at `0x8000_0000`, the CLINT timer and its route to the hart, and the `ns16550a`
console. Every `--check` dumps the tree again and compares it with the file. So a QEMU that changed
the platform under the pinned options would fail the check rather than silently invalidate what was
measured against it. One property is left out of the comparison, by name: a random seed QEMU writes
at every boot. The file also records a deliberate gap. The hart offers floating point, and archogen
builds for `riscv64imac`, without it.

## There is no board

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
