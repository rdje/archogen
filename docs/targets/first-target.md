# Target selection

`ROADMAP.md` §3.2 defines three environments and requires M0 to record the target decision —
including, explicitly, recording an *unavailability* rather than delaying all physical testing
or pretending an emulator is hardware.

| Environment | Purpose | Limitation | Status here |
| --- | --- | --- | --- |
| `hosted-playground` | fast deterministic testing of shared runtime logic and explicit device models | host execution speed gives no target WCET guarantee; simulated preemption covers defined boundaries only | available — the default development target |
| `riscv-virt-up` | early execution of target binaries, startup code, interrupt paths and MMIO contracts in an independent emulator | a virtual platform, not a physical board and not a cycle-accurate timing reference | **verified: QEMU 11.1.1 installed and pinned, its device tree recorded, and in agreement with `targets/riscv-virt-up.eadl`; `TARGET_VERIFIED=yes` (`M2.8.3.4`)** |
| `board-first` | the same OS profile on a named physical processor and device revision | selection and timing evidence must be documented before any hardware claim | **no board procured — decision recorded below** |

## `riscv-virt-up` — installed and pinned, **unverified**

The configuration is pinned as data in [`targets/riscv-virt-up.env`](../../targets/riscv-virt-up.env)
and rendered by one tool, so no two callers can type it slightly differently:

```console
$ scripts/target_emulator.sh --print build/uc1.elf
qemu-system-riscv64 -machine virt -cpu rv64 -smp 1 -m 128M -bios none -nographic -kernel build/uc1.elf
```

`-bios none` is deliberate: the generated image boots directly, with no firmware whose behavior
we have not described. §3.2 wants the platform we describe, not that platform plus an opaque
payload.

**Two facts, recorded rather than smoothed over:**

1. **QEMU is installed, and its release is pinned.** `qemu-system-riscv64 --version` reports
   `QEMU emulator version 11.1.1`; `-machine help` offers `virt` ("RISC-V VirtIO board"); `-cpu help`
   offers `rv64`. `targets/riscv-virt-up.env` carries `QEMU_VERSION_PINNED=11.1.1` (leaf `M2.8.1`),
   and `--check` compares that string against the installed version and refuses a mismatch — measured,
   not assumed, by pinning `99.99.99` and reading `PINNED RELEASE MISMATCH: pinned '99.99.99',
   installed 'QEMU emulator version 11.1.1'`. A pin nothing compares is prose.

   ```console
   $ scripts/target_emulator.sh --check
   target-emulator: found: QEMU emulator version 11.1.1
   target-emulator: the platform presented matches docs/targets/riscv-virt-up.dtb.summary.md
   target-emulator: the §3.2 agreement holds: targets/riscv-virt-up.eadl agrees with the platform presented
   $ echo $?
   0
   ```

   Exit `0` since `M2.8.3.4` (`2026-09-30`). The check finds the pinned release and machine, re-dumps the device
   tree and compares it with [`riscv-virt-up.dtb.summary.md`](riscv-virt-up.dtb.summary.md) (`M2.8.2`), and
   compares the target's eADL description with the same dump (`M2.8.3.2`). A mismatch in any of them exits `1`.
   Before the description existed, the check exited `20`, §14.3's quarantine, owned by `M2.8`.

   §14.3 still governs the absent case, and the tool still honours it: on a machine without
   `qemu-system-riscv64` this exits `20` and says so, because "a required tool skipped or unavailable
   is reported as such, not a passed check".

2. **`TARGET_VERIFIED=yes`, on the comparison §3.2 names.** A pinned release is a fact about the *tool*.
   Verification is a claim about the *platform*, and §3.2 says what justifies it: "inspect the produced hardware
   description, and verify agreement with the eADL platform fixture". Both sides exist now:
   - [`riscv-virt-up.dtb.summary.md`](riscv-virt-up.dtb.summary.md), the device tree the pinned QEMU presents,
     from a measured dump (`M2.8.2`);
   - `targets/riscv-virt-up.eadl`, the target in eADL
     (`decision_target-platform-description.md`, `M2.8.3.2`).

   `cargo xtask target-agreement` compares them fact by fact, and every `--check` runs it on a fresh dump. The flag
   was flipped on that evidence (`M2.8.3.4`), and `--check` refuses `yes` whenever the agreement did not run. Until
   `2026-09-30` the flag was `no`, and neither side existed. A configuration recorded as verified with nothing
   behind it would have been a fabricated fact, which §9 forbids.

### The agreement check this enables

§3.2 requires inspecting the hardware description QEMU actually produces and verifying it
against the eADL platform fixture — not assuming it. `scripts/target_emulator.sh --dump-dtb`
writes the generated device tree for exactly that comparison. It is the difference between
"we described a platform" and "we described *this* platform".

## `board-first` — explicitly not procured

**Decision, 2026-09-13: no physical board has been selected or procured. Physical-target
evidence is blocked. This is a recorded programme risk, not a passed gate.**

§3.2 requires M0 to name the exact board, processor revision, debug interface, memory
execution region, clock configuration, and available specifications. None of those can be
recorded truthfully today, so all of them are recorded as unrecorded:

| Required fact (§3.2) | Value |
| --- | --- |
| Board and revision | `unrecorded` |
| Processor and revision | `unrecorded` |
| Debug interface | `unrecorded` |
| Executable memory region | `unrecorded` |
| Clock configuration | `unrecorded` |
| Available specifications | `unrecorded` |
| Access path (who has it, how it is reached) | `unrecorded` |

Naming a plausible board from memory would be worse than naming none. Every row above is a
fact a later timing claim would rest on, and §9 is explicit that extraction output is "a
proposal with a source location, not automatically an accepted fact". A board named without
its datasheet in hand is a proposal wearing a fact's clothes.

### The selection criteria, so the decision is ready when the board is

§3.2: "Prefer accessible debugging, simple clocks, executable RAM, and documented
timing-relevant behavior. A board that is useful for functional bring-up may still be
unsuitable for strong timing guarantees." Applied to `rt-static-up-v1`:

| Criterion | Why it matters here |
| --- | --- |
| One active core, or one usable in isolation | the profile excludes `multicore`; a board that cannot be run single-core changes the profile |
| A timer with a documented modulus, rate, and read atomicity | §6.2's minimum contract; without the documentation the timer facts are assumptions, not facts |
| Executable RAM and a documented memory map | §7.6 checks the linked image against the available map; an undocumented map makes the check vacuous |
| A simple, documented clock configuration | §3.1 fixes the clock; `dynamic-clock-scaling` is excluded, and an undocumented PLL makes every bound conditional on a guess |
| Accessible debugging (a documented interface, not a vendor blob) | §12 M5 requires observing context preservation and fault paths on the target |
| Documented timing-relevant behavior (caches, buffers, interference) | the difference between a board useful for bring-up and one that can carry a timing claim |
| Obtainable, with a known access path | a board nobody can reach produces no evidence, however suitable |

### What is blocked, and what is not

**Blocked:** tree `M5` in full; leaf `M2.8`'s board half; any `target-evidence` claim; any
statement that the toolchain supports a physical target.

**Not blocked:** everything through M4. `hosted-playground` carries S0 through M4. `riscv-virt-up`
carries the independent-execution half once `M2.8.3` exists — QEMU is installed and pinned, but there
is no bare-metal generated image to run yet (`no-std-build` proves `rt-core` *compiles* for the
target; S0's prototype emits a *host* crate) and no agreement check to justify `TARGET_VERIFIED`.
§12 M5 is
explicit that "M6 development may proceed while a physical access issue is resolved, but M7
cannot claim board support without M5 evidence."

The failure mode this record exists to prevent is the quiet one: reaching M7 with the board row
still empty and the release describing itself as supporting a target it never ran on.
