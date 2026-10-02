# The port's assembly format: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active` — rounds 1 to 3 answered; round 4 next; the review closes on the first round that finds no
  defect
- **External sources:** [the Rust Reference](../book/src/ledger.md#rust-reference) shipped with the pinned toolchain
  — version, hashes and limits in the ledger
- **Owner / source:** leaf `M2.12.2` (`docs/tasks/M2.md`). The design under review is §14.2 and §14.3 of the catalog
  record, kept in [`decision_catalog-records-port.md`](../specs/catalog/decision_catalog-records-port.md), with the
  amendments to §1, §2, §3, §4, §11, §12's note and §13 that point to it. Section numbers are the catalog record's.

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

**Round 3**, `2026-10-02`, of commit `71b6791`, by a new context with the same probing licence. It confirmed every
relocation against a `sym` target or a compiler label, named placeholders binding as the gate reads them, the ending
rule, the `::core` argument and the version reasoning, and found 2 defects. The first broke the reach claim another
way: an admitted `asm!` writing a register it did not declare, after which the compiled Rust jumped to the integer it
left there — the image's reset code at 2147483648. 20 findings: 2 defects, 4 gaps, 6 drafting points, 8 nits.
Defects per round: 8, 2, 2.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | a line writing an undeclared register breaks the compiler's reading of an `asm!` — "Any registers not specified as outputs must have the same value upon exiting …" — so `li a0, …` before a call through `f` jumped to an integer; a naked body writing `s1` likewise | in `asm!`: the inline mnemonics only (§14.3's column), every register operand `zero` or a placeholder of the right direction, no label, no `noreturn`, no `clobber_abi`; for `naked_asm!`, what a body leaves in `sp`, `gp`, `tp` and the callee-saved registers, and what it stores, is the review's; both Reference rules quoted and ledgered |
| 2 | defect | label numbers alias modulo `2^32`: `4294967296:` caught `j 0b` | a label number has one to four digits; the measured truncation recorded |
| 3 | gap | an invocation inside no `fn`, in a closure in a `static` or `const`, escaped the `cfg` rule; methods unclear | each invocation lies inside a function, an associated `fn` included; one inside none is refused |
| 4 | gap | on a riscv64 host, `cfg(target_arch = "riscv64")` holds, and the templates were assembled position-independent | `#[cfg(all(target_arch = "riscv64", target_os = "none"))]`, which no hosted build meets |
| 5 | gap | a record citing declared assembly through `describes` or its closure was not held to the triple | the port-fact rule also requires every target of the stating record to be among the declaration's triples |
| 6 | gap | `pure`, `nomem` and `readonly` let the compiler remove a masking line or move memory across it | refused; `options(nostack)` alone remains |
| 7 | drafting | refusal codes conflicting across §1, §4, §11 and §14.2 | one code per case, listed in §14.2; §4's per-fact locator rule removed, two locators of one fact naming different records admitted |
| 8 | drafting | "its own record's declared assembly" read two ways | "one of its locators names a record that declares assembly for the package its path lies in" |
| 9 | drafting | which record a cross-record refusal names; "own or reached set" of which facet | the reaching record at its targets, or the later declaring record in id order at its subform; "in any facet" |
| 10 | drafting | "written exactly so" — bytes or tokens | "compared as tokens" |
| 11 | drafting | the reach sentence omitted a trap to a written vector, and the plain words and the book computed addresses | all three added |
| 12 | drafting | answer columns no longer true; the header's list of amendments | superseded answers named below; §4 and §12's note added to the header |
| 13 | nit | operand names that are keywords or raw | neither admitted; names compared as §3 compares identifiers |
| 14 | nit | "and no label line follows it" was vacuous | deleted |
| 15 | nit | integers wrap modulo `2^64`, hex and, measured here, decimal too: `18446744073709549568` assembled as `-2048` | decimal only, within the signed 64-bit range |
| 16 | nit | a `sym` naming a `static` in a code position executes data | said to be the review's |
| 17 | nit | the version paragraph's list omitted §4's per-list reading, which the loader already applies | added |
| 18 | nit | a later dialect change after the first lock moves every hash | said so in §14.3 |
| 19 | nit | the `noreturn` quote, and the two register rules, absent from the ledger's scope | all three added to `rust-reference` |
| 20 | nit | `any` refused for the declaring record only | refused for every record reaching a declared package |

**Superseded by this round's answers**, the earlier rows standing as written: round 1's answers 2 (placeholders, since
`{}` and `{n}` are refused), 9 (the triple binds every reaching record, and the `cfg` condition names `target_os`),
10 (integers decimal only; lowercase covers mnemonics and register names), 11 (`clobber_abi` and all options but
`nostack` refused) and 16 (its reasoning restated by round 2's 9); round 2's 2 (labels bounded to four digits), 4
(`noreturn` refused in `asm!`), 8 (the `cfg` condition) and 10 (the codes listed per case).

## Why

The format's claim is a soundness claim, and its first review broke it with the compiler's own output. Rounds go on
until one finds nothing, as the catalog record's own did.

## How to apply

- A new round appends its paragraph and table here, and one row to the summary in §14's record.
- A finding is answered in the record first, and its row here names what answers it.
