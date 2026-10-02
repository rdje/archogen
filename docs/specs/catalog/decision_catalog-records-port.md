# Catalog records: the port's assembly

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active` — §14.1, the measurements, done (`M2.12.1`); the format, §14.2 and §14.3, decided by `M2.12.2`
  on `2026-10-02`, its seventh review finding no defect; the port's statement, §14.4, drafted by `M2.12.3` and under
  review
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
  compiler resolves, so the reach is in the source and its dependency information — unless the path goes through a
  generic parameter, which §14.2 refuses (the fifth review's probe p2);
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
- **`sym` keeps every reach in the source**, its path being concrete (§14.2). `call {h}` and `la t0, {e}` assembled to references to the mangled
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
only the rest admits; when `M2.12.4` lands, after the first lock or before it, no earlier line's verdict
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
  identifiers: `sym <path>`, the path identifiers joined by `::`, optionally beginning `crate::` or `self::`, or with
  one or more `super::`, with no `Self` segment and no generic arguments; `const <expression>`; and in `asm!` only, `in`, `out` or `inout`, each with `(reg)`,
  then its expression, `_` admitted for an output, `inout` optionally followed by `=> <expression>`;
- in `asm!` only, at most one `options(nostack)`.

An invocation holding a `sym` operand lies where no enclosing function, `impl` or `trait`, at any depth, has type or
const parameters — an argument-position `impl Trait` counting as one — and no enclosing item is a `trait`: a `sym`
through a generic parameter names code the crate that instantiates the function chooses, outside every set of the
port's record (the fifth review's probe p2). A plain generic call in Rust reaches the same way, and so does a `const`
or `in` operand of a generic function whose value the instantiating crate chooses, as a computed address: both are the
review's, as Rust is.

A comma after the last argument, or inside `options(…)`, is admitted, as the compiler admits it.

Refused: an attribute on any argument, a template that is a macro invocation, a `label` operand, an operand with an
explicit register, `lateout` and `inlateout` (below), `clobber_abi`, any option but `nostack` — `pure`, `nomem` and `readonly` among them, which let the
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
- **integer:** `0`, or a decimal number not beginning with `0`, that number optionally preceded, with no space, by `-`
  (`-0` refused), its value
  within the signed 64-bit range, since the assembler reads a larger one modulo `2^64` (the third review's probe pi;
  `addi a0, a0, 18446744073709549568` assembled as `-2048`); or a placeholder for a `const` operand, with no `-`
  before it, whose value is the compiler's: one wider than 64 bits fails to assemble, and the rest is read modulo
  `2^64`, as 64-bit arithmetic is;
- **memory:** an integer, then a register in parentheses, as `8(sp)`; it occurs only in a naked body, since no inline
  mnemonic takes one;
- **code:** a placeholder for a `sym` operand;
- **label:** a label number then `b`, naming the nearest earlier label line of that number, by value, in the same
  invocation, or `f`, the nearest later one; one must exist there.

A placeholder is `{<identifier>}`, with no modifier and no space, naming a named operand; its kind is that operand's.

**An `asm!` declares every register it touches, and no output shares a register.** Every line of an `asm!` is an
instruction whose mnemonic §14.3 marks inline, and each register operand is `zero` or a placeholder: where the
signature writes the register, one for an `out` or `inout` operand; where it reads it, one for an `in` or `inout`
operand.
The compiler reads an `asm!` by its declared operands, not its lines — the Reference: "Any registers not specified as
outputs must have the same value upon exiting the assembly code as they had on entry, otherwise behavior is
undefined" — so a line that wrote a register it did not declare could make compiled Rust jump to the integer it left
there (the third review's probe pk: loading 2147483648 into `a0` before a call through `f` jumped there, to the image's
reset code).
`lateout` and `inlateout` are refused because the compiler may give such an output an input's register — the
Reference: "Note that a lateout may be allocated to the same register as an in" — so a line writing it would change
what a later line reads (the fourth review's probe l3: `li {o}, 2147483648` then `csrw mtvec, {h}` installed the
integer, not the handler). `out` and `inout` never share one with an `in`. Two `in` operands of one value may share one
(measured on the pin, in release builds), which no admitted line observes, since none writes an `in` operand's
register. With every
register declared and no output sharing one, the gate's binding of every placeholder is the compiler's. The values are Rust's, and one detail of them is the
review's: an input narrower than 64 bits leaves the register's upper bits undefined — the Reference: "If a value is
of a smaller size than the register it is allocated in then the upper bits of that register will have an undefined
value for inputs" — so what a system register or an output receives from one is checked by the review.
`{}` and `{<digits>}` are refused: how the compiler binds them is not what a reading of "the next" or "the n-th"
gives (`M2.12.2`'s second review, its probe p2, where `call {}` became `call a0`).

So each of these is refused, naming the line: a directive or any token beginning with `.`; a name in a position
whose kind it is not — `call mepc`, `la t0, t1` or `j mstatus` would each assemble to a reference to a symbol of that
name, which the linker binds (`M2.12.2`'s first review, its probe p1); a placeholder in a position its operand's kind
does not take, as `call {r}` for a `reg` operand; an integer where a system register or code goes, as `csrw 261,
zero`; a relocation operator (`%`); a comment; a `;`; `{{` or `}}`; a tab; and a label reference that resolves
outside its invocation.

**A naked body ends.** In `naked_asm!`, the last line is an instruction that never falls through — `mret`, `ret`,
`jr`, `tail`, or `j` to a label. Otherwise a naked body runs on into whatever the linker places next, which no `sym`
names (the first review's probe p5), and the Reference says of a naked function "Behavior is undefined if execution
falls through past the end of the assembly code". An `asm!` admits no such instruction and no `noreturn`, so it falls through into
the compiled Rust code around it, wherever the compiler places that, inlined or not, or traps. What either form writes
to a system register, and how that changes the code after it — the interrupts taken, the trap vector, the privilege
and byte order of later loads and stores, a trap — is the review's.

**What a naked body leaves is the review's.** The Reference requires of a naked function that "All callee-saved
registers must have the same value upon return as they had on entry". A transition restores another context's by
design, so the gate cannot hold a naked body to it. What a body leaves on each exit and at each `call` in `sp`, `gp`,
`tp` and the callee-saved registers, and on an exit by `mret` or into another context in every register — which holds
the interrupted context's value, or the incoming one's for a transition, since compiled Rust was interrupted anywhere
— what it passes to the function a `call` or `tail` enters, or a trap enters through a vector it installs, in every
argument the compiled function reads — its signature's, in registers or on the stack, and those the compiler adds:
the address a result returned in memory is written to, and a `#[track_caller]` function's location, which a `sym`
names without the shim a function pointer gets, since "rustc implements track_caller in a codegen context by
appending an implicit parameter to the function ABI" (the Reference's code-generation chapter) — and what it stores,
is the review's, with the code in view.

**What the refused forms would reach, and what remains the review's.** A directive can read a file, place code where
the image's layout does not expect it, or define a symbol another package could supply in place of a Rust one; a name
in a code position, a body that falls off its end and a label resolved elsewhere each reach what the linker places.
Those are what §3 refused assembly for (the catalog's review findings C9 and E7), and they stay refused. What the
admitted forms reach is a Rust item by `sym`, in the dependency information §3 scans or the pinned toolchain's own
library (premise 1), or an address the code computes, reached through `jr`, a return, or a trap. A trap goes where the
vector the code wrote sends it: to its `BASE` field, the low two bits being the mode, and in vectored mode an
interrupt to `BASE` "plus four times the interrupt cause number", as the [privileged
specification](../../book/src/ledger.md#riscv-privileged) puts it. A jump through a computed address, code written to memory and executed, and a `sym` naming a `static` in a code
position, which executes its bytes, and a fixed address reached through `zero` — `jr zero`, or `zero` written to `mtvec`
or to `mepc` before `mret` — reach what Rust's own casts can reach, and the review checks them as for Rust
(§3: the token rules are necessary, not sufficient).

**Alignment.** `mtvec` needs a 4-byte-aligned base (§14.1). The pinned compiler gives a naked function that
alignment, and no attribute asks for it. How the port knows the address it installs as the vector is aligned, and
where each trap then lands, is therefore a fact its record states (`M2.12.3`). The pin's measured behaviour can be its basis only because the toolchain file is in the record's
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
  assembly for the package `<path>` lies in; the record stating the fact names its targets, `any` refused, and each
  has its `RUST_TARGET` among the triples §14.3 lists for that declaration's architecture. The locators are well formed either way, so the refusal is of the value, not
  `catalog-locator`. Whether the locators reach the port's half of a fact that rests on other code too is the
  review's;
- a fact whose basis rests on the port's code beyond its own record's, as `acknowledge-at-entry.<source>`,
  `one-request-per-arrival.<source>` and `raised-only-when-due`'s may, should name the port's record in `describes`
  and carry a locator into it. The loader checks nothing more here; the review checks that the locators reach the
  code the basis rests on (§13: a code locator is necessary, not sufficient).

**Refusals** (§11), one code per case, taken in this order — structure, then locators, then sources, then fields:
- `catalog-shape`: a second `assembly` subform, or one naming no package; two identical locators of one fact; several
  locators on a fact that is not a code fact; a code fact's locators not one after another;
- `catalog-locator`: a locator §2's rule does not admit, a `file` or `ledger` locator on a code fact among them;
- `catalog-source`: a declaration naming a package twice, or one not among the facet's `sources` (§4's path rules, as
  §11 files them); a package outside §14.2 and §14.3;
- `catalog-field`: an unknown architecture; a record reaching a declared package, or stating a known port fact, with
  `any` or a target off the dialect's triples; two declarations of one package naming different architectures; a known
  port fact whose locators are all admitted, none of them one the port-fact rule requires.

A refusal names the record whose field breaks the rule: for `any` or a target off the triples, the reaching or
stating record at its contract's targets; for two declarations that disagree, the later record in id order at its `assembly` subform.

#### 14.3 The `riscv64` dialect

Its one target triple is `riscv64imac-unknown-none-elf`, one of the two `rust-toolchain.toml` pins, for code running in
machine mode on one hart (`docs/targets/first-target.md`). The list is what a port needs and no more — its trap entry and exit, its
transitions, its masking and its trap vector, as §14.1 found them in the spike. A later need adds to it by a change
to this record; from the first lock on, that is a new rules version, which moves every hash (§5).

| Kind | Admitted |
| --- | --- |
| registers | `x0`–`x31`; `zero`, `ra`, `sp`, `gp`, `tp`, `fp`, `t0`–`t6`, `s0`–`s11`, `a0`–`a7` |
| system registers | `mstatus`, `mie`, `mip`, `mtvec`, `mscratch`, `mepc`, `mcause`, `mtval`, `mhartid` — the last read-only in the [privileged specification](../../book/src/ledger.md#riscv-privileged)'s CSR listing, which says "Attempts to write a read-only register raise illegal-instruction exceptions", so admitted only as `csrr`'s system register |

Each mnemonic, with the kind of each operand in order (R register, C system register, I integer, M memory, S code,
L label). An R that leads a signature is written, every other R and a memory operand's register are read, and `sd`,
`sw`, the branches and `jr` write none; their leading R is read. The rows marked *inline* are the only ones an `asm!` admits:

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

#### 14.4 The port's statement (`M2.12.3`)

**In plain words.** The fault contract (`docs/profiles/rt-static-up-v1-faults.md`, its Terms, its rules 1, 2, 5
and 7, and its *Still open*) leaves a list of things to the port: how it catches a primitive called from outside a
job, what may run after a job's last instruction, whether its traps serve one interrupt or several, how a failed check
reaches the fatal path, how long that path takes, and more. The port's record states each as a fact, yes or no, whose
basis says how, with locators into the code that does it, so a review judges it beside the code. One of them is
matched across records mechanically, through a record of its own: the convention by which a failed check passes what
it found.

**The port's record** is the record that supplies `switch` under the selection
(`decision_runtime-composite-inputs.md` §1). Its behavioral model states the facts below and its timing model the
costs below, all in the `switch` group (§12), each fact a code fact whose locators reach its own implementation or a
`describes` record's, as §2 admits — the runtime API record's for what rests on `rt-core`'s primitives, scheduler or
detection. A fact absent and a fact `unknown` are both undeclared (§12), and a port record leaving one of them so
cannot back `M4`'s fixtures (below).

| Fact | `yes` states | The basis says | Obligatory |
| --- | --- | --- | --- |
| `detects-non-job-calls` | the port raises every call of a primitive or the completion path from a context that is not a job, as the contract's Terms require | how it tells a job's call apart | yes |
| `services-preempt-completion-interval` | a service may preempt the interval after the instruction at which a job completes | how it guarantees the choice and the next fact's | |
| `completion-decision-remade` | a decision a service preempted is made again; `no`, no service preempts the decision. Read only when the fact above is `yes` | — | |
| `guard-check-contexts` | for each guard check the port's own code makes, the context that runs it is stated | the table of checks and contexts, from the contract's rule 2's set | yes |
| `generated-guard-check-contexts` | the same for each guard check generated code makes; `unknown` until `M4` defines generated code's checks | the table | |
| `guarded-stacks` | every stack the port guards is named | each stack, by name | yes |
| `decision-placement` | where the port makes a scheduling decision is stated | the place, the completion path's included | yes |
| `api-entry-by-trap` | the port enters the runtime API by a trap | the trap and how it tells its entry apart | |
| `api-trap-preemptible-before-decode` | an interrupt may preempt that trap before it tells its entry apart. Read only when `api-entry-by-trap` is `yes` | in which contexts it can | |
| `primitives-preemptible` | an interrupt may preempt a primitive | which primitives, and at which points: at the entry, at the return, between | |
| `unmask-preemptible-before-check` | an interrupt may preempt an `unmask` at depth zero before its check. Read only when `primitives-preemptible` is `yes` | — | |
| `window-trap-preempts-outside` | a trap may preempt the context between a failed check, or a panic's start, and the raising, outside a primitive | the window whole: what runs before the raising, in which contexts, and where the kept job stands in the schedule | |
| `window-trap-preempts-inside` | the same inside a primitive. Read only when `primitives-preemptible` is `yes` | as above | |
| `window-release-abandons` | a release observed in that window may abandon the job. Read only when one of the two facts above is `yes` | — | |
| `window-primitive-consistent` | inside a primitive, an abandonment in the window leaves the runtime's state consistent; `no`, the job is not abandoned there. Read only when `window-release-abandons` is `yes` | — | |
| `abandoned-primitive-completion` | how and in which context the runtime completes a primitive on an abandoned job's behalf is stated | the context, one in whose role the composition lets that code act, and how a fault met there is raised | yes |
| `fault-path-entries` | the fault path's two entries are named | each entry's function and signature: the one the image's panic handler calls, and the one a check's call and the trap path reach; what the handler passes it | yes |
| `traps-discriminated` | the trap path tells a check's deliberate trap from any other trap | how | yes |
| `kept-record-readout` | the kept record can be read out of a halted runtime | where it lives and how it is read | yes |
| `panic-strategy-abort` | the image is built with `panic = "abort"`, under which every panic calls the handler | the pin's measurement (below) | yes |
| `vector-direct` | the port installs the trap vector in direct mode, every trap landing at its base | how it knows that base is aligned as `mtvec` needs: by the pin's measured alignment (§14.1), or by a check in its code | yes |

A known `no` of a fact marked obligatory is refused (`catalog-field`): the contract requires each. Vectored mode is
not admitted: its four-byte slots would rest on the assembler choosing four-byte encodings, which the dialect cannot
force, since `j` assembles to the two-byte `c.j` (`M2.12.3`'s first review). Whether a trap serves one interrupt or
several is `one-claim-per-trap`'s (§12), whose basis states, when it is `no`, how a trap finds a further pending
interrupt — a timer interrupt by its pending bit — and the order among the interrupts one trap serves.

**Within the record**, the loader refuses (`catalog-field`): a fact stated `yes` or `no` where it is read only when
another is `yes` and that one is not; and `api-trap-preemptible-before-decode` `yes` with `primitives-preemptible`
`no`, since the contract's rule 2 makes a primitive's trap, from its entry to its return, the primitive.

**The fatal path's bound** is two timing costs of the same record, per target: `fatal-path.entry`, from a fault's
raising through an entry of the fault path to the terminal state, and `fatal-path.trap`, from a fault raised at a
trap, along the trap path and its other entry, to the terminal state (the contract's rule 7). Each covers a fault
taken in the handler or on the trap path on the way, which ends the runtime at once. A later idle-to-task dispatch
needs no cost of its own: the switch out of idle is `S`'s, and the wake before it `W_wake`'s
(`decision_runtime-analysis-variant.md` §1).

**The panic handler.** §3 refuses `panic_handler` in every package a record reaches, since a global hook another
package could supply in its place is code no record would hold. So the image's `#[panic_handler]` is generated by
`M4`, and holds nothing but a call of the entry `fault-path-entries` names, with what the compiler emits around it;
the port's record supplies that entry. An application's own handler beside it does not build: two are a duplicate lang
item (E0152, measured on the pin).

**The panic strategy, measured on the pin** (`rustc 1.95.0 (59807616e 2026-04-14)`, `riscv64imac-unknown-none-elf`,
`2026-10-02`). A `no_std` crate built `panic = "abort"`, holding `fn checked(v: &[u8], i: usize) -> u8 { v[i] }`
and `fn explicit(x: u32) -> u32 { if x == 7 { panic!("seven") } x + 1 }`, calls
`core::panicking::panic_bounds_check` and `core::panicking::panic_fmt`, which end in the handler. `-C panic=unwind`
builds a library but not a binary ("unwinding panics are not supported without std", the first review's probe), and
`-C panic=immediate-abort`, the strategy that aborts without calling the handler, is answered "`-Cpanic=immediate-abort`
requires `-Zunstable-options` and a nightly compiler"; and an image's build runs in the gate's allowlisted environment,
where a `RUSTC_BOOTSTRAP` that would unlock it is seen (§7).
So on the pin a binary is built `abort` or not at all, every panic calls the handler, and the contract's cases under a
strategy that aborts without calling it cannot arise. These are premises §14.4 rests on, re-run with §14.1's when the
pin moves.

**The check per selection.** A check-passing convention is a record of its own, whose id begins
`convention.check-passing.`, whose implementation is `none`, and whose contract states the convention: how a failed
check, a deliberate trap or call on a failed check of a record's own invariants, and the trap path pass the kind, the
raiser and, for a guard, whose guard, to the fault path — registers or memory, encodings, which entry, and the route
of a check that panics through the image's handler. The port's record lists exactly one such record in `depends`,
and so does every record whose code makes such a check or deliberate trap or call. The convention's meaning is then
in each of their bound hashes, and a change to it makes their reviews stale (§10). Under a selection where a record
supplies `switch`, the selection is refused (`catalog-conflict`) when the port depends on no convention record, or
on more than one, or when any record in the selection depends on one the port does not. Under a selection with no
`switch`, the check is not made. The check is necessary, not sufficient: a record whose code makes a check but
depends on no convention passes it, and its review refuses it, as the contract says. Generated code, in no record, is
held to the port's convention by the review of the plan's generator (`M4`).

**For `M4`'s fixtures** (`M4.6`, the contract's F26 list), each case is owed where the port's record makes it arise:
- a case the contract owes "on each port that" does something, on a port whose record states the fact for it `yes` —
  `primitives-preemptible`, `api-entry-by-trap`, `api-trap-preemptible-before-decode`,
  `unmask-preemptible-before-check`, `services-preempt-completion-interval`, `completion-decision-remade`, and
  `one-claim-per-trap` `no` for a further pending interrupt served in the same trap — and not where it is `no`;
- the window's cases one by one: a trap taken before a raising, outside a primitive where
  `window-trap-preempts-outside` is `yes`, inside one where `window-trap-preempts-inside` is; a release observed then
  abandoning the job, where `window-release-abandons` is `yes`; what becomes of the primitive, as
  `window-primitive-consistent` states; the kept context run on to the raising under both policies, where
  `window-release-abandons` is `no`, or `window-primitive-consistent` is `no`; and a fatal fault raised before the
  raising, kept if it is the first, on every port;
- a case a fact's basis scopes to some contexts or points — `api-trap-preemptible-before-decode`'s contexts,
  `primitives-preemptible`'s points — is owed there, matched by `M4.6`'s review against the basis;
- the cases under a strategy that aborts without calling the handler are owed on no port while the pin offers none.

The `riscv64` dialect (§14.3) has no `ecall`, `ebreak` or `unimp`, and `core::arch::breakpoint` and
`core::intrinsics::abort` are unstable on the pin (the first review's probe): so today no record's code can enter the
runtime API by a trap or make a deliberate trap, and the cases owed on a port that enters the runtime API by a trap
are owed on no `riscv64` port.

**Still the contract's.** Whether the contract will limit what a port may state of a release that abandons a job
outside a primitive while a check of the runtime's state that generated code makes has yet to raise what it found
stays the contract's open question for `M4`, whose generated code alone can reach it.

**Versioning.** §14.4's facts, costs and loader rules are part of what `archogen-catalog/1` names, like §14.2's and
§14.3's.

## Review

The format is reviewed by contexts that did not write it, until a round finds no defect. The findings and the answer
to each are in [`decision_catalog-records-port-reviews.md`](../../reviews/decision_catalog-records-port-reviews.md).

| Round | Findings | What it turned on | Outcome |
| --- | --- | --- | --- |
| 1, `2026-10-02` | 18 | operands typed by spelling: `call mepc` and the like assembled to symbols the linker binds by name; a naked body falling off its end | 8 defects, 4 gaps; typed operand signatures, a terminal transfer, labels within the invocation; all answered |
| 2, same day | 14 | `call {}` bound by the compiler to a `reg` operand; a leading-zero label read as octal | 2 defects, 3 gaps; named placeholders only, labels by value without a leading zero, `noreturn` ending, the triple; all answered |
| 3, same day | 20 | an `asm!` writing an undeclared register made compiled Rust jump to an integer; labels aliasing modulo `2^32` | 2 defects, 4 gaps; inline `asm!` narrowed to declared registers and CSR work, labels of four digits, a hosted build's `cfg` excluded; all answered |
| 4, same day | 13 | `lateout` sharing an input's register installed an integer as the trap vector | 1 defect, 3 gaps; `lateout` and `inlateout` refused, `mret` returns and system-register effects the review's, `mhartid` read only; all answered |
| 5, same day | 13 | three wording defects — inputs sharing a register, a naked exit's duty narrowed, a lost quote — and a generic `sym` | 3 defects, 3 gaps; words restored, a generic `sym` refused, the trap's address arithmetic stated; all answered |
| 6, same day | 12 | a quotation lost in round 5's compaction; what a naked body hands a function at a `call` | 1 defect, 1 gap; the `mhartid` sentence quoted, the `call` hand-off the review's; all answered |
| 7, same day | 9 | **no defect**: every quotation verbatim, nothing lost across seven states, no admitted construction past the claim | the review closed; its two gaps and points answered |

§14.4, the port's statement, is reviewed apart, until a round finds no defect; its findings and answers are in
[`decision_catalog-records-port-statement-reviews.md`](../../reviews/decision_catalog-records-port-statement-reviews.md).

| Round | Findings | What it turned on | Outcome |
| --- | --- | --- | --- |
| 1, `2026-10-02` | 30 | the check per selection colliding with §12's one-supplier rule and binding no meaning; the contract's amendments incomplete; F26's run-on scoping inverted | 9 defects, 13 gaps; a convention record through `depends`, the contract's Terms and rule 7 amended, the window's facts split; all answered |

## Why

A format designed before its evidence is measured is how a hidden reach gets in: §3's refusal of assembly answered the
catalog's review findings C9 and E7, which showed `asm!` reaching code at link time with no dependency edge. The
measurements say what the port needs and what the toolchain does with it, so the format admits that and nothing more.

## How to apply

- A bare section number here is the catalog record's.
- The measurements are the pin's. When the toolchain pin moves, `M2.12.1`'s probe is re-run, with every premise §14.2
  and §14.3 state as measured on the pin — the labels' and integers' wrapping, a `const` wider than 64 bits, two `in`
  operands sharing a register, the registers `call`, `tail`, `la` and `li` write — before any record rests on it. A
  changed result is a change to §14.2 or §14.3, so a new rules version from the first lock on. `M2.12.4` holds these
  premises as compile-only tests built with the pinned toolchain, so the bump's own run fails first.
