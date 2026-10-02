# Catalog records: the port's assembly

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active` — §14.1, the measurements, done (`M2.12.1`); the format, §14.2 and §14.3, drafted by
  `M2.12.2` and under review
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

**In plain words.** A record may name packages of its implementation that hold assembly, and the architecture whose
dialect they are written in. In those packages, and only there, `::core::arch::asm!` and `::core::arch::naked_asm!`
are admitted, written out in full, with templates the gate reads line by line against closed lists of instructions,
registers and system registers. Every reach into other code is a `sym` operand, a Rust path the compiler resolves.
Nothing else assembly can do is admitted: no directive, no symbol written by name, no `global_asm!`.

**`/1` is amended, not replaced.** No catalog record and no lock exist yet (`catalog/` is absent, `2026-10-02`), so
nothing has been ledgered under `/1` and no history needs its rules kept (§5). The change lifts a limit of `/1`, as
§13 foresaw, and `ROADMAP.md` §15's ask that a change be explicit is met here. From the first lock on, a change of
this kind is a new rules version (§9).

**The declaration.** An implementation facet may end with `(assembly <architecture> "<package>" …)`:
- `<architecture>` names a dialect of §14.3; `riscv64` is the only one;
- each package is one of the facet's `sources`, and none is named twice;
- the declaration is among the facet's forms, so its own hash covers it, and adding or removing it moves that hash.

A package that no declaration names holds no assembly: §3's token rules refuse `asm` and `naked_asm` there, as
before.

**The invocation.** In a declared package, `asm` and `naked_asm` are admitted as tokens only in the sequence
`:: core :: arch :: asm ! (` or `:: core :: arch :: naked_asm ! (`, the leading `::` included. Any other occurrence —
a `use` of either, a rename, a bare `asm!` — is refused, and `global_asm` stays refused everywhere. So that `::core` is
the toolchain's: the declared package's sources hold no `extern crate`, and its manifest names no dependency `core`
and gives none a `package` key.

**The arguments**, read as tokens:
- first, one or more template strings, each a string literal holding no `\` and not raw. The compiler joins them
  with line breaks, so each is one line of the template;
- then operands, each optionally named `<identifier> =`: `sym <path>`; `const <expression>`; and in `asm!` only,
  `in`, `out`, `lateout`, `inout` or `inlateout`, each with `(reg)` or a register of the dialect written as a string,
  then its expression, `inout` and `inlateout` optionally followed by `=> <expression>`;
- then, optionally, `clobber_abi("C")`, and `options(…)` holding only `nomem`, `readonly`, `pure`,
  `preserves_flags`, `noreturn` and `nostack`. `raw`, which leaves the template unread for placeholders, is refused,
  and so is any other option.

The expressions are Rust, held to §3's token rules like any other.

**The template.** Each line, read by §14.3's dialect for the declared architecture, is:
- optionally a numeric local label, digits then `:`;
- then optionally one instruction: a mnemonic from the dialect's list, then its operands separated by commas;
- an operand is a register, a system register's name, an integer (decimal, or `0x` and hex digits, either
  optionally preceded by `-`), a placeholder (`{}`, `{<digits>}` or `{<identifier>}`, with no modifier), a memory
  operand `<integer or placeholder>(<register or placeholder>)`, or, as a branch's or jump's target, a numeric
  label reference, digits then `b` or `f`; `fence` alone takes its own operands (§14.3);
- spaces may separate tokens, and nothing else is admitted.

So a directive (any token beginning with `.`), a name that is neither a register nor a system register — a symbol
written by name — a relocation operator (`%`), a comment (`#`, `//` or `/*`), a `;` putting two statements on one
line, and `{{` or `}}` are each refused, naming the line.

**What the refused forms would reach, and what remains the review's.** A directive can read a file, place code where
the image's layout does not expect it, or define a symbol another package could supply in place of a Rust one; a name
written in a template reaches whatever the linker resolves it to. Those are what §3 refused assembly for (the
catalog's review findings C9 and E7), and they stay refused. What the admitted forms reach is a Rust item by `sym`,
inside the dependency information §3 scans, or an address the code computes. A jump through a computed address, or
code written to memory and executed, reaches what Rust's own casts can reach, and the review checks it as for Rust
(§13: the token rules are necessary, not sufficient).

**Alignment.** `mtvec` needs a 4-byte-aligned base (§14.1). The pinned compiler gives a naked function that
alignment, and no attribute asks for it. How the port knows its trap entry is aligned is therefore a fact its record
states (`M2.12.3`). The pin's measured behaviour can be its basis only because the toolchain file is in the record's
bound hash (§3), so a moved pin makes the review stale.

**The port's facts, by what a locator reaches.** §13 refused, by name, a known value of every fact about the port's
code. That is restated:
- a code fact may carry one or more `code` locators, each into code §2's locator rule admits for its facet, and its
  bound hash covers each;
- a fact §12 lists as about the port's code — the `switch` group's, and the port's half of `preemptive-everywhere`,
  `sections-mask-every-interrupt`, `releases-never-latched` and `primitives-out-of-line` — may be known only with a
  locator into a package an `assembly` declaration names; without one it is refused (`catalog-field`), as §13 refused
  it;
- a fact whose basis rests on code beyond its record's own implementation — the port's, for
  `acknowledge-at-entry.<source>`, `one-request-per-arrival.<source>` or `raised-only-when-due` — names that code's
  record in `describes` and carries a locator into it. That the locators reach all the code the basis rests on is the
  review's to check (§13: a code locator is necessary, not sufficient).

**Refusals.** A package outside these rules is `catalog-source`; a declaration or a fact outside them is
`catalog-field` (§11).

#### 14.3 The `riscv64` dialect

For code built for `riscv64imac-unknown-none-elf`, the target `rust-toolchain.toml` pins, running in machine mode on
one hart (`docs/targets/first-target.md`).

| Kind | Admitted |
| --- | --- |
| registers | `x0`–`x31`; `zero`, `ra`, `sp`, `gp`, `tp`, `fp`, `t0`–`t6`, `s0`–`s11`, `a0`–`a7` |
| system registers | `mstatus`, `misa`, `medeleg`, `mideleg`, `mie`, `mtvec`, `mcounteren`, `mscratch`, `mepc`, `mcause`, `mtval`, `mip`, `mhartid`, `mcycle`, `minstret`, `cycle`, `time`, `instret`, `pmpcfg0`–`pmpcfg15`, `pmpaddr0`–`pmpaddr63` |
| base instructions | `lui`, `auipc`, `jal`, `jalr`, `beq`, `bne`, `blt`, `bge`, `bltu`, `bgeu`, `lb`, `lh`, `lw`, `ld`, `lbu`, `lhu`, `lwu`, `sb`, `sh`, `sw`, `sd`, `addi`, `slti`, `sltiu`, `xori`, `ori`, `andi`, `slli`, `srli`, `srai`, `add`, `sub`, `sll`, `slt`, `sltu`, `xor`, `srl`, `sra`, `or`, `and`, `addiw`, `slliw`, `srliw`, `sraiw`, `addw`, `subw`, `sllw`, `srlw`, `sraw`, `fence`, `fence.i`, `ecall`, `ebreak` |
| multiplication | `mul`, `mulh`, `mulhsu`, `mulhu`, `div`, `divu`, `rem`, `remu`, `mulw`, `divw`, `divuw`, `remw`, `remuw` |
| system registers' instructions | `csrrw`, `csrrs`, `csrrc`, `csrrwi`, `csrrsi`, `csrrci` |
| machine mode | `mret`, `wfi` |
| assembler shorthands | `nop`, `li`, `la`, `mv`, `not`, `neg`, `negw`, `sext.w`, `seqz`, `snez`, `sltz`, `sgtz`, `beqz`, `bnez`, `blez`, `bgez`, `bltz`, `bgtz`, `bgt`, `ble`, `bgtu`, `bleu`, `j`, `jr`, `ret`, `call`, `tail`, `csrr`, `csrw`, `csrs`, `csrc`, `csrwi`, `csrsi`, `csrci`, `rdcycle`, `rdtime`, `rdinstret` |

`fence` takes no operands, or two, each a non-empty string of `i`, `o`, `r` and `w` in that order. Compressed forms
are the assembler's to choose: a `c.` mnemonic is refused. Atomic instructions and floating point are absent: one
hart needs no atomics, and the build target has no floating-point extension, though the emulated hart offers one. A later need adds to the list by a change to this record.

## Why

A format designed before its evidence is measured is how a hidden reach gets in: §3's refusal of assembly answered the
catalog's review findings C9 and E7, which showed `asm!` reaching code at link time with no dependency edge. The
measurements say what the port needs and what the toolchain does with it, so the format admits that and nothing more.

## How to apply

- A bare section number here is the catalog record's.
- The measurements are the pin's. When the toolchain pin moves, `M2.12.1`'s probe is re-run and §14.1 re-read before
  any record rests on it.
