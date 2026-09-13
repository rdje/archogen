# Target selection

`ROADMAP.md` §3.2 defines three environments and requires M0 to record the target decision —
including, explicitly, recording an *unavailability* rather than delaying all physical testing
or pretending an emulator is hardware.

| Environment | Purpose | Limitation | Status here |
| --- | --- | --- | --- |
| `hosted-playground` | fast deterministic testing of shared runtime logic and explicit device models | host execution speed gives no target WCET guarantee; simulated preemption covers defined boundaries only | available — the default development target |
| `riscv-virt-up` | early execution of target binaries, startup code, interrupt paths and MMIO contracts in an independent emulator | a virtual platform, not a physical board and not a cycle-accurate timing reference | **configuration pinned, toolchain not installed** |
| `board-first` | the same OS profile on a named physical processor and device revision | selection and timing evidence must be documented before any hardware claim | **no board procured — decision recorded below** |

## `riscv-virt-up` — pinned, unverified, uninstalled

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

1. **QEMU is not installed on the current development machine.**

   ```console
   $ scripts/target_emulator.sh --check
   target-emulator: required tool unavailable: qemu-system-riscv64 is not on PATH
   target-emulator:   the riscv-virt-up environment cannot be exercised on this machine
   target-emulator:   install it, then re-run; do NOT record an emulator result without it
   $ echo $?
   20
   ```

   §14.3: "A required tool skipped or unavailable is reported as such, not a passed check."

2. **No QEMU release is pinned yet, and `TARGET_VERIFIED=no`.** Every value in the
   configuration is a *proposal* until checked against an installed QEMU. Recording it as a
   proposal is a fact about our knowledge; recording it as verified with nothing behind it
   would be a fabricated fact, which §9 forbids. Leaf `M2.8` owns flipping it, with evidence.

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

**Not blocked:** everything through M4. `hosted-playground` carries S0 through M4, and
`riscv-virt-up` carries the independent-execution half as soon as QEMU is installed. §12 M5 is
explicit that "M6 development may proceed while a physical access issue is resolved, but M7
cannot claim board support without M5 evidence."

The failure mode this record exists to prevent is the quiet one: reaching M7 with the board row
still empty and the release describing itself as supporting a target it never ran on.
