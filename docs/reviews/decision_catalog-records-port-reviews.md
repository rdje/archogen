# The port's assembly format: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active` — rounds 1 and 2 answered; round 3 next; the review closes on the first round that finds no
  defect
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

**Round 2**, `2026-10-02`, of commit `e9f3a50`, by a new context with the same probing licence. It confirmed round 1's
approach: every admitted mnemonic with every admitted operand kind left no undefined symbol and no relocation but
against `sym` targets and the compiler's own labels; out-of-range immediates fail at assembly; `const` operands are
integers only. But 2 defects, each a place where the text's reading was not the compiler's or the assembler's. 14
findings: 2 defects, 3 gaps, 5 drafting points, 4 nits.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | "`{}` the next positional operand" read as "after the last placeholder" admitted `call {}` that the compiler bound to a `reg` operand: `call a0`, `U a0` | `{}` and `{<digits>}` refused; `sym`, `const` and `(reg)` operands named, placeholders `{<identifier>}` only |
| 2 | defect | a label with a leading zero is octal to the assembler: `010:` with `j 10b` jumped to another function's `10:` | a label number is `0` or a decimal not beginning with `0`, compared by value |
| 3 | drafting | §4's "a written path appears at most once in a facet" refused every declaration, and two facts locating one file | §4 amended: once per list — `sources`, a declaration's packages, one fact's locators — and two facts may locate one file |
| 4 | gap | `asm!` with `noreturn` falls off into `unimp` on the pin, which is behaviour, not a rule; "the function around it, which its package holds" false under inlining | the terminal transfer required of `noreturn` too, the Reference quoted ("behavior is undefined if it does"); the sentence reworded |
| 5 | gap | step 2 dropped "from the first lock on, a change is a new rules version" for §14 | §14.2 and §14.3 are part of what `archogen-catalog/1` names, in §14.2 and §3's bullet |
| 6 | gap | nothing tied a declared package's builds to the dialect's triple, and other records reaching it could name other targets | §14.3 lists its triple; every target of every record reaching a declared package has its `RUST_TARGET` among them |
| 7 | drafting | the port-fact rule stated three ways, §12's and §13's looser | both now state §14.2's: a locator whose own record declares assembly for the package it locates |
| 8 | drafting | the `cfg` rule's reach: which `fn`, and `cfg_attr` | the innermost enclosing `fn` item's outer attribute, written exactly, not inside `cfg_attr`; "another architecture" rather than "the host" |
| 9 | drafting | the version reasoning cited "a corrected implementation", which refuses more, where the amendment relaxes | every amendment named a relaxation; no lock line can hold a record only the amendment admits; no earlier verdict changes |
| 10 | drafting | which refusal code wins for a mixed locator list, a disallowed record, an empty or doubled declaration | stated in §14.2's refusals: structure `catalog-shape`, an inadmissible locator `catalog-locator`, the rest `catalog-field`, packages `catalog-source` |
| 11 | nit | `sym` to a sysroot item is covered by premise 1, not the dependency information | said so |
| 12 | nit | "every token lowercase" against placeholder identifiers; `-` before a `const` placeholder; `0x` with no digit | mnemonics and registers lowercase; no `-` before a placeholder; `0x` and one or more digits |
| 13 | nit | `tail {f}` writes `t1`, which the line does not name | §14.3 says which registers `call`, `tail`, `la` and `li` write, measured on the pin: `ra`, `t1`, and only their operand |
| 14 | nit | `rust-toolchain.toml` pins two targets | "one of the two" |

## Why

The format's claim is a soundness claim, and its first review broke it with the compiler's own output. Rounds go on
until one finds nothing, as the catalog record's own did.

## How to apply

- A new round appends its paragraph and table here, and one row to the summary in §14's record.
- A finding is answered in the record first, and its row here names what answers it.
