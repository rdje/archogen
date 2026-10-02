# The port's assembly format: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active` — round 1 answered; round 2 next; the review closes on the first round that finds no defect
- **External sources:** [the Rust Reference](../book/src/ledger.md#rust-reference) shipped with the pinned toolchain
  — version, hashes and limits in the ledger
- **Owner / source:** leaf `M2.12.2` (`docs/tasks/M2.md`). The design under review is §14.2 and §14.3 of the catalog
  record, kept in [`decision_catalog-records-port.md`](../specs/catalog/decision_catalog-records-port.md), with the
  amendments to §1, §2, §3, §11 and §13 that point to it. Section numbers are the catalog record's.

## The fact / decision

**Round 1**, `2026-10-02`, of commit `6512bb2`, by a context that had not written the design. It built compile-only
probe crates with the pinned toolchain, outside the repository, and read their objects with `nm -u` and `objdump -r`.
Its verdict: the approach sound — assembly only in declared packages, invocations written out in full,
`global_asm!`, directives, `%` and raw strings refused — but the claim failing as drafted: "the pinned toolchain turns
several forms the grammar admits into references to symbols that the linker resolves by name". 8 defects, 4 gaps, 5
drafting points, 1 nit. Its central finding was reproduced here before answering: `call mepc` in a naked body left
`mepc` an undefined symbol of the object.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | operands were typed by spelling, so a register or system-register name in a code position — `call mepc`, `la t0, mtvec`, `j mstatus`, `beqz a0, mcause`, the load and store forms at a symbol — assembled to a reference to a symbol of that name | an operand signature per mnemonic (§14.3); each operand checked against the kind its position takes; code positions take only a `sym` placeholder; loads and stores a memory operand only |
| 2 | defect | a `reg` placeholder in a code position, `call {r}`, made the register allocator's choice a symbol name | placeholders typed by the operand the compiler binds them to, `{}`, `{n}` and `{name}` as the compiler reads them |
| 3 | defect | an integer or a `const` placeholder in a system-register position, `csrw 0x105, zero`, wrote a register off the list | a system-register position takes a listed name and nothing else |
| 4 | defect | a naked body could fall off its end into whatever the linker places next | the last line of a `naked_asm!` is `mret`, `ret`, `jr`, `tail` or `j` to a label, and no label line follows it |
| 5 | gap | numeric label references resolved across invocations, and a `cfg` on an argument could remove a label | a label reference resolves within its own invocation; an attribute on any argument is refused |
| 6 | gap | an integer as a branch's or jump's target, decided by layout and able to land mid-instruction; `auipc` with an integer | targets are labels or `sym` placeholders only; `lui`, `auipc`, `jal` and `jalr` absent |
| 7 | defect | "`::core` is the toolchain's" was false: a path dependency whose lib is named `core` passed every rule; nothing harmful followed | the three rules removed, the sentence resting on the reviewer's argument: binding `asm` or `naked_asm` otherwise needs the name outside the admitted sequence, or a macro definition, which §3 refuses wherever a record reaches |
| 8 | gap | whether a package declared by one record holds assembly when another record reaches it; two declarations of one package; which locator counts for a port fact | a declaration admits its package's assembly wherever the package is reached; two declarations of one package name one architecture; a port fact's locator counts only when its own record declares that package |
| 9 | gap | the architecture not tied to what a package is built for: a template under another `target_arch` is assembled by another assembler | the declaring record's targets all of the architecture, `any` refused; each invocation in a function carrying `#[cfg(target_arch = "<architecture>")]`, so the host build assembles none |
| 10 | drafting | a literal holding a line feed is two lines; tabs; octal from a leading zero; hex case; a detached `-` | no line feed in a literal; single spaces only; lowercase; decimal not beginning with `0`; lowercase hex; `-` attached |
| 11 | drafting | the Reference's grammar admits attributes on arguments, macro templates, `label` operands, several option groups and interleaving; `_` unmentioned | each refused by name, the order fixed; `_` admitted for an output |
| 12 | defect | §11 unamended: `catalog-field` still said "in `/1`, … unknown", the new refusals were in no row, and the port-fact refusal's kind clashed with `catalog-locator` | §11's rows amended; the port-fact refusal stays `catalog-field`, the locator being well formed, as §14.2 says |
| 13 | defect | "several `code` locators" contradicted §1's "duplicated … is a refusal" and `catalog-shape`; the form unclear | the form written out: repeated `(locator (code …))` subforms, no two the same; §1, §2 and `catalog-shape` amended |
| 14 | defect | "a fact §12 lists as about the port's code" and "the port's half of" could not be tested | the twelve facts named, as the crate's own `PORT` list holds them; a known value needs a locator into its record's declared assembly; the half is the review's |
| 15 | drafting | the third port-facts bullet read as a loader rule | "the loader checks nothing more here; the review checks" |
| 16 | drafting | amending `/1` in place contradicted §3's "changing any part of this section changes it" | §3's sentence holds from the first lock on; the stronger reason stated: the loader refuses everything the amendment admits until `M2.12.4`, so no lock line can hold it, and `M2.12.4` landing after the first lock is the implementation catching up |
| 17 | drafting | the dialect beyond "admits that and nothing more": system registers that change how all later code runs, `fence.i`, `ecall`, `ebreak`; odd `pmpcfg` registers RV32-only | trimmed to what a port needs: nine system registers, a short instruction list with signatures |
| 18 | nit | "among the facet's forms"; the header's "every fact … is `unknown`"; the book's line | "the subform is part of the facet's form"; the header in the past tense; the book's line moved with finding 1 |

The answers were probed before landing, with the pinned toolchain, outside the repository: a naked body reading
`mcause`, branching to a label in its own invocation and ending `tail {h}`, and an `asm!` storing through `{off}({b})`
with `const` and `reg` placeholders, left no undefined symbol; under `#[cfg(target_arch = "riscv64")]` the host build
assembled none; and rustc refuses to name an explicit-register operand, so a placeholder can only bind a `reg`, `sym`
or `const` operand.

## Why

The format's claim is a soundness claim, and its first review broke it with the compiler's own output. Rounds go on
until one finds nothing, as the catalog record's own did.

## How to apply

- A new round appends its paragraph and table here, and one row to the summary in §14's record.
- A finding is answered in the record first, and its row here names what answers it.
