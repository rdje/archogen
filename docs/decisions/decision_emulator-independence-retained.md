# The independent emulator is retained, and `M2.8` is reframed as making the platform facts *checked*

- **Type:** `decision`
- **Date:** `2026-09-28`
- **Status:** `active`
- **Owner / source:** director ruling, `2026-09-28` — asked "why do we need QEMU in the first place,
  can't we do without it?", heard the argument and the limits, and ruled: keep it, continue with the
  roadmap, apply the reframing below
- **External sources:** [QEMU](../book/src/ledger.md#qemu) — version, scope and limits in the ledger

## The decision

`riscv-virt-up` stays. `ROADMAP.md` §3.2's three environments are **not** re-scoped, and §2's success
condition 2 ("a deterministic hosted environment **and an independently implemented emulator**")
stands. `M2.8` is reframed: its deliverable is not "install and pin a tool" but **make the platform
facts checked rather than asserted**.

## Why

archogen is a generator, so its correctness has two halves: the description is well-formed and the
lowering is right; and the emitted artifact works **on a machine**. The first half is what the suite
tests. The second half is where the *facts* live — a timer's base address, an interrupt controller's
claim/complete registers, a UART's transmit-hold condition, the reset vector, `mtvec` alignment, the
callee-saved set — and every one of them is data in a catalog that nothing in this repository can
verify.

The failure is quiet, which is what makes it the worst class for a project whose deliverable includes
an assurance report. A wrong base address need not crash, and it **cannot** be caught hosted, because
the hosted device model reads the same catalog: both sides agree and the test passes. Every WCET
derived from a tick that does not arrive at the assumed rate is then wrong by an unknown factor,
inside a document that states it as a number. An independent executor is the only thing that can
contradict the catalog, because it derives its device layout from its own implementation of the
platform specifications rather than from ours.

The same argument already worked once at the model level: `M2.2`'s independently derived reference
agreed with `rt-core` over 16 000 events and disagreed in five places, every one a question the
roadmap had not answered. `decision_findings-for-director-review.md` §5 generalises it — "a
specification gap is invisible while one person implements it". The emulator is that second reader,
one level down, for the machine rather than the scheduler.

Measured, so this is not an abstraction — `scripts/target_emulator.sh --dump-dtb` on the installed
QEMU 11.1.1 with the pinned options gives:

| Device | Address | `compatible` |
| --- | --- | --- |
| RAM | `0x8000_0000`, 128 MiB | — |
| console UART | `0x1000_0000` | `ns16550a` |
| timer / software interrupt | `0x0200_0000`, 64 KiB | `sifive,clint0`, `riscv,clint0` |
| interrupt controller | `0x0C00_0000`, 6 MiB | `sifive,plic-1.0.0`, `riscv,plic0` |

and `riscv,isa = "rv64imafdch_…"`. Note the last one: the platform **offers** F and D while archogen
builds `riscv64imac`. That is a deliberate subset and is correct, but it is exactly the kind of fact
that must be written down — otherwise a future reader "fixes" the mismatch by enabling floating point
in generated code. The corpus already contains the shape of the error this catches:
`docs/semantics/boundary/accept/addressable-region.eadl` declares
`(region device.timer (base 0x1000_0000) (size 64 KiB))`, which on this platform is the UART, not the
timer. That fixture is fictional — it tests the boundary classifier — but it is well-formed,
type-checked, boundary-classified `ACCEPT`, and wrong, and nothing in the frontend can tell.

## The limits, recorded so the decision is not over-read

- **QEMU is an independent *implementation*, not independent *truth.** It reads the same ACLINT, PLIC
  and 16550 specifications archogen does, so a *shared* misreading agrees. It catches wrong addresses,
  wrong register offsets, broken lowering and broken startup; it does not catch "we and the emulator
  both misread the spec". Hence §7: "QEMU and hardware can also be incomplete or faulty, so
  disagreements are investigated rather than settled by majority vote."
- **Only ISA, ABI, startup and interrupt *mechanics* transfer to a board.** Device addresses do not.
- **No timing claim rests on it.** §3.2: "not a physical board or a cycle-accurate timing reference";
  §18: "do not treat as a particular physical board or WCET oracle". WCET comes from `M2.6`'s
  runtime-applicable analysis and ultimately from the board. Anyone tempted to cite an emulator run
  for a timing number is citing the wrong artifact.
- **It is prospective, not present.** There is no bare-metal generated image yet: `no-std-build`
  proves `rt-core` *compiles* for the target, and S0's prototype emits a *host* Rust crate. The value
  starts at `M2.8`'s hand-assembled spike and becomes load-bearing at `M4.9`.

## How to apply

- `M2.8` delivers: the release pinned in `targets/riscv-virt-up.env`; the Rust target triple and ISA
  feature set moved there out of `xtask/src/main.rs`; **`docs/targets/riscv-virt-up.dtb.summary.md`
  written from the measured device tree** — it is named by `DEVICE_TREE_FIXTURE` at line 45 of the
  `.env` and does not exist, so the §3.2 agreement check currently has one side and no other; and the
  comparison made a **test**, not a one-off reading, so a QEMU release that moves the platform under
  an unchanged description fails rather than silently invalidating every conformance observation.
- Correct the five stale "QEMU is not installed" surfaces listed in `M2.8`'s leaf as part of it — they
  are the verification-state prose that leaf rewrites anyway.
- Do not treat the emulator as a substitute for the board. `docs/tasks/M5.md` already carries "No
  substitution of QEMU for hardware", and §3.2 says the same: record an explicit target decision
  "rather than delaying all physical testing or pretending QEMU is hardware".
- Where an external specification is needed to read the device tree or a device contract, use
  [[reference_external-document-source-chipdoc]] — `REQ-007` (the QEMU `virt` machine documentation
  and its device-tree bindings) is already `requested` there. The dumped DTB is better evidence than
  the bindings document would be; the bindings help interpret it.
