# Catalog records: the port's assembly

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active` — §14.1, the measurements, done (`M2.12.1`); the format, §14.2 and §14.3, drafted by
  `M2.12.2` and under review
- **External sources:** [the Rust Reference](../../book/src/ledger.md#rust-reference) shipped with the pinned
  toolchain, [the Rust toolchain](../../book/src/ledger.md#rust-toolchain) itself, and [the RISC-V privileged
  specification](../../book/src/ledger.md#riscv-privileged) — versions, hashes and limits in the ledger
- **Owner / source:** leaf `M2.12` (`docs/tasks/M2.md`). This is §14 of [[decision_catalog-records]]: normative,
  numbered as its §14, and reviewed with it. Why it is needed: §3 refused assembly, so the port's trap entry and
  exit, its transitions and its masking instructions were in no record, and every fact about them was `unknown`
  (`decision_catalog-records-limits.md`).

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
are admitted, written out in full. The gate reads each template line by line: one instruction from a short list,
each operand checked against the kind its position takes — a register, a system register by name, an integer, a
memory operand, a local label, or a `sym` operand where an instruction reaches code. So every reach into other code
is a Rust path the compiler resolves, or an address the code computes. Nothing else assembly can do is admitted: no directive, no symbol written by
name, no `global_asm!`.

**`/1` is amended, not replaced.** No catalog record and no lock exist yet (`catalog/` is absent, `2026-10-02`), so
nothing has been ledgered under `/1`, and no history needs its rules kept (§5). Every amendment here relaxes a rule
— the assembly tokens, the `assembly` subform, several locators, a known port fact, and §4's path rule read per list,
which the loader already applies — and none refuses what `/1` admitted. Until `M2.12.4` the loader refuses the rest, so no lock line written before then can hold a record that
only the amendment admits; when `M2.12.4` lands, after the first lock or before it, no earlier line's verdict
changes. §14.2 and §14.3 are part of what `archogen-catalog/1` names: from the first lock on, changing them, like
changing §3, is a new rules version (§9). `ROADMAP.md` §15's ask that a change be explicit is met here.

**The declaration.** An implementation facet may end with the subform `(assembly <architecture> "<package>" …)`:
- `<architecture>` names a dialect of §14.3; `riscv64` is the only one;
- each package is one of the facet's `sources`, and none is named twice;
- every record whose own or reached set, in any facet, holds a declared package — the declaring record among them —
  names its targets, `any` refused, and each target's `RUST_TARGET` is among the triples §14.3 lists for the
  architecture;
- the subform is part of the facet's form, so the facet's own hash covers it, and adding or removing it moves that
  hash.

A package that a declaration names holds assembly wherever it is reached, in any record's sets. Two declarations of
one package name the same architecture. A package that no declaration names holds none: §3's token rules refuse
`asm` and `naked_asm` there, as before.

**The invocation.** In a declared package, `asm` and `naked_asm` are admitted as tokens only in the sequence `:: core
:: arch :: asm ! (` or `:: core :: arch :: naked_asm ! (`. Any other occurrence — a `use` of either, a rename, a bare
`asm!` — is refused, and `global_asm` stays refused everywhere. Whatever `::core` names, the macro is the toolchain's:
binding `asm` or `naked_asm` to anything else would need the name as a token outside that sequence, or a macro
definition, and §3 refuses both in every package a record reaches. Each invocation lies inside a function, a `fn`
item or an associated `fn` of an `impl` or a `trait`, and the innermost such function carries the outer attribute
`#[cfg(all(target_arch = "<architecture>", target_os = "none"))]`, compared as tokens and not inside `cfg_attr`. An
invocation inside no function, as in a closure in a `static`, is refused. So no hosted build, the gate's build for
its host among them whatever the host's architecture, assembles a template the gate read by this dialect.

**The arguments**, read as tokens, in this order and no other:
- one or more template strings, each a string literal that is not raw and holds no `\` and no line feed. The compiler
  joins them with line feeds, so each is exactly one line of the template;
- operands, each named `<identifier> =`, the identifier neither a keyword nor raw and compared as §3 compares
  identifiers: `sym <path>`; `const <expression>`; and in `asm!` only, `in`, `out`, `lateout`, `inout` or
  `inlateout`, each with `(reg)`, then its expression, `_` admitted for an output, `inout` and `inlateout` optionally
  followed by `=> <expression>`;
- in `asm!` only, at most one `options(nostack)`.

Refused: an attribute on any argument, a template that is a macro invocation, a `label` operand, an operand with an
explicit register, `clobber_abi`, any option but `nostack` — `pure`, `nomem` and `readonly` among them, which let the
compiler remove or move a masking line (the third review's probe pm) — and anything out of that order. The expressions are Rust, held to §3's
token rules like any other.

**The template.** Each line is, with single spaces (`0x20`) only where the dialect's tokens need separating, and a
space after each comma:
- a local label alone: a label number, then `:`; or
- one instruction: a mnemonic from §14.3's list, then exactly the operands its signature there gives, each of the
  kind its position takes.

A label number is `0`, or a decimal number of one to four digits not beginning with `0`: the assembler reads a leading
`0` as octal, and, measured on the pin, reads a number of `2^32` or more modulo `2^32`.
The kinds, every mnemonic and register name lowercase:
- **register:** a name §14.3 lists, or a placeholder for a `reg` operand;
- **system register:** a name §14.3 lists, and nothing else;
- **integer:** `0`, or a decimal number not beginning with `0`, optionally preceded, with no space, by `-`, its value
  within the signed 64-bit range, since the assembler reads a larger one modulo `2^64` (the third review's probe pi;
  `addi a0, a0, 18446744073709549568` assembled as `-2048`); or a placeholder for a `const` operand, with no `-`
  before it;
- **memory:** an integer, then a register in parentheses, as `8(sp)` or `{off}({base})`;
- **code:** a placeholder for a `sym` operand;
- **label:** a label number then `b`, naming the nearest earlier label line of that number, by value, in the same
  invocation, or `f`, the nearest later one; one must exist there.

A placeholder is `{<identifier>}`, with no modifier and no space, naming a named operand; its kind is that operand's.

**An `asm!` declares every register it touches.** In `asm!`, a line's mnemonic is one §14.3 marks inline, it takes
no label, and each register operand is `zero` or a placeholder: where the signature writes the register, one for an
`out`, `lateout`, `inout` or `inlateout` operand; where it reads it, one for an `in`, `inout` or `inlateout` operand.
The compiler reads an `asm!` by its declared operands, not its lines — the Reference: "Any registers not specified as
outputs must have the same value upon exiting the assembly code as they had on entry, otherwise behavior is
undefined" — so a line that wrote a register it did not declare could make compiled Rust jump to the integer it left
there (the third review's probe pk: loading 2147483648 into `a0` before a call through `f` jumped there, to the image's
reset code).
With every register declared, the gate's reading of the block is the compiler's.
`{}` and `{<digits>}` are refused: how the compiler binds them is not what a reading of "the next" or "the n-th"
gives (`M2.12.2`'s second review, its probe p2, where `call {}` became `call a0`).

So each of these is refused, naming the line: a directive or any token beginning with `.`; a name in a position
whose kind it is not — `call mepc`, `la t0, t1` or `j mstatus` would each assemble to a reference to a symbol of that
name, which the linker binds (`M2.12.2`'s first review, its probe p1); a placeholder in a position its operand's kind
does not take, as `call {r}` for a `reg` operand; an integer where a system register or code goes, as `csrw 0x105,
zero`; a relocation operator (`%`); a comment; a `;`; `{{` or `}}`; a tab; and a label reference that resolves
outside its invocation.

**A naked body ends.** In `naked_asm!`, the last line is an instruction that never falls through — `mret`, `ret`,
`jr`, `tail`, or `j` to a label. Otherwise a naked body runs on into whatever the linker places next, which no `sym`
names (the first review's probe p5). An `asm!` admits no such instruction and no `noreturn`, so it falls through into
the compiled Rust code around it, wherever the compiler places that, inlined or not.

**What a naked body leaves is the review's.** The Reference requires of a naked function that "All callee-saved
registers must have the same value upon return as they had on entry". A transition restores another context's by
design, so the gate cannot hold a naked body to it: what it leaves in `sp`, `gp`, `tp` and the callee-saved registers
on each return, and what it stores, is the review's, with the code in view.

**What the refused forms would reach, and what remains the review's.** A directive can read a file, place code where
the image's layout does not expect it, or define a symbol another package could supply in place of a Rust one; a name
in a code position, a body that falls off its end and a label resolved elsewhere each reach what the linker places.
Those are what §3 refused assembly for (the catalog's review findings C9 and E7), and they stay refused. What the
admitted forms reach is a Rust item by `sym`, in the dependency information §3 scans or the pinned toolchain's own
library (premise 1), or an address the code computes, reached through `jr`, a return, or a trap to a vector the code
wrote. A jump through a computed address, code written to memory and executed, and a `sym` naming a `static` in a code
position, which executes its bytes, reach what Rust's own casts can reach, and the review checks them as for Rust
(§13: the token rules are necessary, not sufficient).

**Alignment.** `mtvec` needs a 4-byte-aligned base (§14.1). The pinned compiler gives a naked function that
alignment, and no attribute asks for it. How the port knows its trap entry is aligned is therefore a fact its record
states (`M2.12.3`). The pin's measured behaviour can be its basis only because the toolchain file is in the record's
bound hash (§3), so a moved pin makes the review stale.

**The port's facts, by what a locator reaches.** §13 refused, by name, a known value of every fact about the port's
code. That is restated:
- a code fact may carry one or more `code` locators: its form is `(fact <name> yes|no (locator (code …)) …
  (basis "…"))`, the locators one after another, no two the same, each admitted by §2's locator rule for its facet;
  its bound hash covers each;
- these twelve facts are about the port's code: `eager-switching`, `interrupts-do-not-nest`,
  `services-preempt-every-task`, `pending-taken-and-transitions-unmasked`, `pending-taken-after-unmask`,
  `no-empty-claim`, `one-claim-per-trap`, `starts-by-transition`, `preemptive-everywhere`,
  `sections-mask-every-interrupt`, `releases-never-latched` and `primitives-out-of-line`. A known value of one is
  refused (`catalog-field`) unless one of its locators, `(code <id> "<path>")`, names a record `<id>` that declares
  assembly for the package `<path>` lies in, and every target of the record stating the fact has its `RUST_TARGET`
  among that declaration's triples. The locators are well formed either way, so the refusal is of the value, not
  `catalog-locator`. Whether the locators reach the port's half of a fact that rests on other code too is the
  review's;
- a fact whose basis rests on the port's code beyond its own record's, as `acknowledge-at-entry.<source>`,
  `one-request-per-arrival.<source>` and `raised-only-when-due`'s may, should name the port's record in `describes`
  and carry a locator into it. The loader checks nothing more here; the review checks that the locators reach the
  code the basis rests on (§13: a code locator is necessary, not sufficient).

**Refusals** (§11), one code per case:
- `catalog-shape`: a second `assembly` subform, or one naming no package; two identical locators of one fact; several
  locators on a fact that is not a code fact;
- `catalog-locator`: a locator §2's rule does not admit, a `file` or `ledger` locator on a code fact among them;
- `catalog-source`: a declaration naming a package twice, or one not among the facet's `sources` (§4's path rules, as
  §11 files them); a package outside §14.2 and §14.3;
- `catalog-field`: an unknown architecture; a record reaching a declared package with `any`, or with a target off the
  dialect's triples; two declarations of one package naming different architectures; a known port fact with no
  locator the port-fact rule admits.

A refusal names the record whose field breaks the rule: for a target off the triples, the reaching record at its
contract's targets; for two declarations that disagree, the later record in id order at its `assembly` subform.

#### 14.3 The `riscv64` dialect

Its one target triple is `riscv64imac-unknown-none-elf`, one of the two `rust-toolchain.toml` pins, for code running in
machine mode on one hart (`docs/targets/first-target.md`). The list is what a port needs and no more — its trap entry and exit, its
transitions, its masking and its trap vector, as §14.1 found them in the spike. A later need adds to it by a change
to this record; from the first lock on, that is a new rules version, which moves every hash (§5).

| Kind | Admitted |
| --- | --- |
| registers | `x0`–`x31`; `zero`, `ra`, `sp`, `gp`, `tp`, `fp`, `t0`–`t6`, `s0`–`s11`, `a0`–`a7` |
| system registers | `mstatus`, `mie`, `mip`, `mtvec`, `mscratch`, `mepc`, `mcause`, `mtval`, `mhartid` |

Each mnemonic, with the kind of each operand in order (R register, C system register, I integer, M memory, S code,
L label). An R that leads a signature is written, every other R and a memory operand's register are read, and `sd`,
`sw`, the branches and `jr` write none. The rows marked *inline* are the only ones an `asm!` admits:

| Signature | Mnemonics | Inline |
| --- | --- | --- |
| R, R, I | `addi`, `andi`, `ori`, `xori` | no |
| R, R, R | `add`, `sub`, `and`, `or`, `xor` | no |
| R, I | `li` | inline |
| R, R | `mv` | inline |
| R, M | `ld`, `lw`, `sd`, `sw` | no |
| R, C | `csrr` | inline |
| C, R | `csrw`, `csrs`, `csrc` | inline |
| C, I | `csrwi`, `csrsi`, `csrci` | inline |
| R, C, R | `csrrw`, `csrrs`, `csrrc` | inline |
| R, C, I | `csrrwi`, `csrrsi`, `csrrci` | inline |
| R, R, L | `beq`, `bne` | no |
| R, L | `beqz`, `bnez` | no |
| R, S | `la` | inline |
| S | `call`, `tail` | no |
| L | `j` | no |
| R | `jr` | no |
| none | `ret`, `mret` | no |
| none | `wfi`, `nop` | inline |

Beyond its operands, `call` writes `ra` and `tail` writes `t1`; `la` and `li` write only their register operand, a
large `li` expanding into several instructions on it alone (measured on the pin, `M2.12.2`'s second round). `ld`,
`lw`, `sd` and `sw` take a memory operand only, so the assembler's forms that load or store at a symbol are not
admitted. No instruction takes an integer as a branch's or a jump's target, and `lui`, `auipc`, `jal` and `jalr` are
absent: `la`, `call`, `tail`, `j` and `jr` do what a port needs with a code or label operand. Compressed forms are the
assembler's to choose, and a `c.` mnemonic is refused. Atomic instructions and floating point are absent: one hart
needs no atomics, and the build target has no floating-point extension, though the emulated hart offers one.

## Review

The format is reviewed by contexts that did not write it, until a round finds no defect. The findings and the answer
to each are in [`decision_catalog-records-port-reviews.md`](../../reviews/decision_catalog-records-port-reviews.md).

| Round | Findings | What it turned on | Outcome |
| --- | --- | --- | --- |
| 1, `2026-10-02` | 18 | operands typed by spelling: `call mepc` and the like assembled to symbols the linker binds by name; a naked body falling off its end | 8 defects, 4 gaps; typed operand signatures, a terminal transfer, labels within the invocation; all answered |
| 2, same day | 14 | `call {}` bound by the compiler to a `reg` operand; a leading-zero label read as octal | 2 defects, 3 gaps; named placeholders only, labels by value without a leading zero, `noreturn` ending, the triple; all answered |
| 3, same day | 20 | an `asm!` writing an undeclared register made compiled Rust jump to an integer; labels aliasing modulo `2^32` | 2 defects, 4 gaps; inline `asm!` narrowed to declared registers and CSR work, labels of four digits, a hosted build's `cfg` excluded; all answered |

## Why

A format designed before its evidence is measured is how a hidden reach gets in: §3's refusal of assembly answered the
catalog's review findings C9 and E7, which showed `asm!` reaching code at link time with no dependency edge. The
measurements say what the port needs and what the toolchain does with it, so the format admits that and nothing more.

## How to apply

- A bare section number here is the catalog record's.
- The measurements are the pin's. When the toolchain pin moves, `M2.12.1`'s probe is re-run and §14.1 re-read before
  any record rests on it.
