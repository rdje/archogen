# Catalog records: the port's assembly

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active` — §14.1, the measurements, done (`M2.12.1`); the format, §14.2, is `M2.12.2`'s and not yet
  decided
- **External sources:** [the Rust Reference](../../book/src/ledger.md#rust-reference) shipped with the pinned
  toolchain, [the Rust toolchain](../../book/src/ledger.md#rust-toolchain) itself, and [the RISC-V privileged
  specification](../../book/src/ledger.md#riscv-privileged) — versions, hashes and limits in the ledger
- **Owner / source:** leaf `M2.12` (`docs/tasks/M2.md`). This is §14 of [[decision_catalog-records]]: normative,
  numbered as its §14, and reviewed with it. §13 of that record says why it is needed: §3 refuses assembly, so the
  port's trap entry and exit, its transitions and its masking instructions are in no record, and every fact about
  them is `unknown` (`decision_catalog-records-limits.md`).

## The fact / decision

### 14. The port's assembly

#### 14.1 What the port's assembly needs, measured (`M2.12.1`)

Nothing here decides the format. It records what the format has to admit and what it can rest on, each measured or
quoted, so `M2.12.2` designs against evidence.

**What the port must hold.** The code the fault contract and the composition record leave to the port, which Rust
alone cannot write on the target:
- the trap entry and exit: saving the interrupted context beyond what the hart saves, reading the trap's cause,
  calling the runtime, restoring and returning with `mret`;
- the transitions: saving the outgoing context and restoring the incoming one, so `S` and `eager-switching` are about
  code a record holds;
- the masking instructions the runtime API's primitives use, on `mstatus`;
- writing the trap vector, `mtvec`.

The spike under `targets/riscv-virt-up/spike/` holds all of this today, in `global_asm!`, and holds besides what is
the image's and not a record's: the reset entry `_start`, the stack and `.bss` set-up from symbols the linker script
defines (`__stack_top`, `__bss_start`, `__bss_end`), and the placement directives `.section`, `.globl`, `.text` and
`.balign`. It names its Rust functions by `#[no_mangle]` symbols (`spike_main`, `spike_trap`), which §3 refuses in a
record. Which part is the image's is `M4`'s to build; what the port's record must hold is the list above.

**What the pinned Reference guarantees** ([the Rust Reference](../../book/src/ledger.md#rust-reference), the one
shipped with `1.95.0`, ledgered by hash):
- a `sym` operand "must refer to a fn or static", and "A mangled symbol name referring to the item is substituted
  into the asm template string". A template that reaches code only through `sym` names it by a Rust path, which the
  compiler resolves, so the reach is in the source and its dependency information;
- `naked_asm!` and `global_asm!` "can only use sym and const operands";
- of directives, the Reference lists a subset every supported assembler accepts, and "The result of using other
  directives is assembler-specific (and may cause an error, or may be accepted as-is)". So a directive outside the
  list, one that reads a file among them, may be accepted;
- the code-generation attributes chapter offers no attribute that sets a function's alignment.

**What the hart requires.** The [privileged specification](../../book/src/ledger.md#riscv-privileged) on `mtvec`:
"The value in the BASE field must always be aligned on a 4-byte boundary, and the MODE setting may impose additional
alignment constraints on the value in the BASE field."

**What the pinned compiler emits**, measured `2026-10-02` with `rustc 1.95.0 (59807616e 2026-04-14)` for
`riscv64imac-unknown-none-elf`. The probe is a crate outside this repository, `crate-type = ["rlib"]`, `panic =
"abort"` in its release profile, built by `cargo +1.95.0 rustc --release --target riscv64imac-unknown-none-elf --
--emit asm`. Its source, sha256 `d3f078573bc4e82912e076b1a6f2515a946e81aae632b8e63d12e88d4daac540`:

```rust
#![no_std]
use core::arch::{asm, naked_asm};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub extern "C" fn handler() -> usize {
    7
}

#[unsafe(naked)]
pub extern "C" fn trap_entry() {
    naked_asm!(
        "addi sp, sp, -16",
        "sd ra, 0(sp)",
        "call {h}",
        "ld ra, 0(sp)",
        "addi sp, sp, 16",
        "mret",
        h = sym handler,
    )
}

#[unsafe(naked)]
pub extern "C" fn reset() {
    naked_asm!("la t0, {e}", "csrw mtvec, t0", "ret", e = sym trap_entry)
}

#[inline(never)]
pub fn install() {
    unsafe { asm!("csrw mtvec, {}", in(reg) trap_entry as *const () as usize) };
}
```

What it emitted for the two naked functions, verbatim apart from blank and `.cfi` lines:

```text
	.section	.text._ZN10nakedprobe10trap_entry17h9bf1aa9842e58abcE,"ax",@progbits
	.p2align	2
	.globl	_ZN10nakedprobe10trap_entry17h9bf1aa9842e58abcE
	.type	_ZN10nakedprobe10trap_entry17h9bf1aa9842e58abcE,@function
_ZN10nakedprobe10trap_entry17h9bf1aa9842e58abcE:
	addi	sp, sp, -16
	sd	ra, 0(sp)
	call	_ZN10nakedprobe7handler17h814b32a0db93ce13E
	ld	ra, 0(sp)
	addi	sp, sp, 16
	mret
	.section	.text._ZN10nakedprobe5reset17h280811f798d2250bE,"ax",@progbits
	.p2align	2
	.globl	_ZN10nakedprobe5reset17h280811f798d2250bE
	.type	_ZN10nakedprobe5reset17h280811f798d2250bE,@function
_ZN10nakedprobe5reset17h280811f798d2250bE:
.Lpcrel_hi0:
	auipc	t0, %pcrel_hi(_ZN10nakedprobe10trap_entry17h9bf1aa9842e58abcE)
	addi	t0, t0, %pcrel_lo(.Lpcrel_hi0)
	csrw	mtvec, t0
	ret
```

and for `install`, after the same `auipc`/`addi` pair into `a0`, `csrw mtvec, a0` between `#APP` and `#NO_APP`. The
ordinary functions `handler` and `install` were emitted `.p2align 1`. A debug build of the same source gave `.p2align
2` for both naked functions and `.p2align 1` for the others.

**What follows for the format** — constraints `M2.12.2` must meet, not its decisions:
- **A naked function needs no directive.** The compiler places it, aligns it and makes it global. A template that
  only holds instructions, registers, immediates, `{…}` placeholders and numeric local labels can write the whole
  trap entry, exit and transition.
- **`sym` keeps every reach in the source.** `call {h}` and `la t0, {e}` assembled to references to the mangled
  symbols of `handler` and `trap_entry`; no name was written. A template that writes a name — a linker-script symbol,
  a `#[no_mangle]` function — reaches code §3 cannot see, and the format must refuse one.
- **Alignment is measured, not guaranteed.** Both profiles gave the naked trap entry the 4-byte alignment `mtvec`
  needs, and no attribute can ask for it. A record that rests on it states it as a fact about the pin, revalidated
  when the pin moves, or the port checks it, for instance by reading `mtvec` back.
- **Any directive the Reference does not list may be accepted.** So the format cannot allow "safe" directives by
  name and refuse the rest by the assembler: it refuses every directive, and a template needing one is not admitted.

#### 14.2 The format (`M2.12.2`)

Not yet decided.

## Why

A format designed before its evidence is measured is how a hidden reach gets in: §3's refusal of assembly answered the
catalog's review findings C9 and E7, which showed `asm!` reaching code at link time with no dependency edge. The
measurements say what the port needs and what the toolchain does with it, so the format admits that and nothing more.

## How to apply

- A bare section number here is the catalog record's.
- The measurements are the pin's. When the toolchain pin moves, `M2.12.1`'s probe is re-run and §14.1 re-read before
  any record rests on it.
