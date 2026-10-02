# The port's statement: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active` — round 1 answered; round 2 next; the review closes on the first round that finds no defect
- **External sources:** [the Rust Reference](../book/src/ledger.md#rust-reference) shipped with the pinned toolchain,
  and [the RISC-V privileged specification](../book/src/ledger.md#riscv-privileged) — versions, hashes and limits in
  the ledger
- **Owner / source:** leaf `M2.12.3` (`docs/tasks/M2.md`). The design under review is §14.4 of the catalog record, kept
  in [`decision_catalog-records-port.md`](../specs/catalog/decision_catalog-records-port.md), with the amendments it
  makes to the fault contract (`docs/profiles/rt-static-up-v1-faults.md`), the composition record, §2, §9, §11, §12
  and §14's summary of the catalog record, `M4.6` and `M4.10`. Section numbers are the catalog record's.

## The fact / decision

**The inventory the design rests on**, `2026-10-02`, by a read-only context asked to list every obligation the fault
contract leaves to the port, each quoted at its source, with its shape and whether another record states a
counterpart. It found 25 entries: the port's half of *Still open*, split into its items (non-job-call detection, the
completion interval, guard-check contexts, a further pending interrupt in one trap, the order one trap serves, the API
entry by trap and its preemption before decode, primitives' preemption, the window before a raising, an abandoned
primitive's completion, the panic strategy, check passing, the fault path's entries, trap discrimination, the fatal
path's bound, the kept record's read-out, the check per selection, the generated-check limit, the idle-dispatch cost);
the trap vector's alignment and landing, carried from §14.2; and the carried points — F26's scoping class, DR1, nits
2 and 4, and R14's checks' points on the window's other contexts and the kept job's place. Nits 2 and 4, and R14's
detail, survive only as one-line summaries in `docs/decisions/decision_runtime-contract-gaps.md`; they cannot be
recovered, and the window's facts cover the class they belonged to. It named 15 ambiguities in the contract's
wording, each answered in §14.4 or by its amendments: which record is the port's (the one supplying `switch`); the
completion interval's two or three choices (two facts); the granularity of preemption (per fact, its basis scoping
it); the aborting strategy (measured: none on the pin); what "states otherwise" compares (a convention record by id);
guard-check contexts of other records (their own, as the contract says); the read-out's timing (the fact's basis);
the fatal bound per route (two costs); the idle cost (`S`'s); the generated-check limit (the contract's, for `M4`);
the carried points' loss (said); the window's boundary (the fact's basis, the contract's rules over it); and
`one-claim-per-trap` against a further pending interrupt (one fact, the existing one).

**Round 1**, `2026-10-02`, of commit `867100c`, by a new context that went through the contract itself, with
compile-only probes on the pin. It confirmed the panic-strategy measurements, adding that `-C panic=unwind` builds no
`no_std` binary and that two handlers are a duplicate lang item. Its verdict: the check per selection fails in its
central mechanism, the contract amendments are incomplete, two obligations are uncovered, and F26's run-on scoping is
inverted. 30 findings: 9 defects, 13 gaps, 6 drafting points, 2 nits.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | `check-passing.<convention>` stated by two records collides with §12's one-supplier rule and with the `switch` group: a matching pair is refused | the convention is a record of its own, `convention.check-passing.<…>`, which the port and every checking record list in `depends`; the selection compares ids |
| 2 | defect | a convention name means what the port's basis says, outside every other record's hash; a change leaves nothing stale | through `depends`, the convention record's contract is in every stater's bound hash, and a change makes their reviews stale |
| 3 | defect | the panic strategy left to the port, with no fact stating it | `panic-strategy-abort`, obligatory, its basis the pin's measurement |
| 4 | defect | the Terms (a panic raised at the handler's first act), rule 7 and "the first act of each" still made the handler an entry | each amended: a panic raised at the first act of the entry the image's handler calls; rule 7's entries named; "of each entry" |
| 5 | defect | the header's approved narrowing still said the handler is the port's, unrecorded against the approval | the header item states the mechanism, dated; findings §12 notes it for the director's review, effect unchanged |
| 6 | defect | generated code's guard-check contexts, which the contract leaves to the port, uncovered | `generated-guard-check-contexts`, `unknown` until `M4` defines generated code's checks; `M4.10` collects it |
| 7 | defect | F26's run-on case inverted: owed where the window's facts are `no`, scoped as if `yes` | the window's cases scoped one by one; the run-on owed where `window-release-abandons` or `window-primitive-consistent` is `no`; a fatal fault in the window owed on every port |
| 8 | defect | one window fact merged inside and outside a primitive, leaving the carried scoping instance open | two facts, `window-trap-preempts-outside` and `-inside`, the latter read only when primitives are preemptible |
| 9 | defect | §12, §2 and §11 unamended: the facts neither code facts nor the `switch` group's to the loader | §12's `switch` group lists §14.4's facts and costs; §2's code-fact rule names §14.4; §11 and §9's conflict row name its refusals |
| 10 | gap | `serves-further-pending-in-trap` and `one-claim-per-trap` are complements, only yes/yes refused | the first deleted: `one-claim-per-trap` decides it, its basis stating the order a trap serves when `no` |
| 11 | gap | the check's scope under a selection with no `switch`, "none or two", and `no`/`unknown` statements | not made without `switch`; "no convention record, or more than one"; a convention is depended on, not stated yes or no |
| 12 | gap | a checking record depending on no convention passes | the check necessary, not sufficient; its review refuses it |
| 13 | gap | a `no` on an obligatory fact meant nothing | an obligatory column; a known `no` refused (`catalog-field`) |
| 14 | gap | scoping by a basis's contexts or points is prose | said: matched by `M4.6`'s review against the basis; `primitives-preemptible`'s basis names its points |
| 15 | gap | dependency rules among the facts missing | each "read only when" stated, and `api-trap-preemptible-before-decode` `yes` refused with `primitives-preemptible` `no` |
| 16 | gap | the generated handler's code before its call, and the new `M4` obligations, owned by no one | the handler holds nothing but the call; `M4.10` collects the handler, generated checks' convention and contexts |
| 17 | gap | the entries' signatures and the panic route unstated | `fault-path-entries`' basis gives each signature and what the handler passes; the convention states the panic route |
| 18 | gap | the fatal path's costs omitted a fault taken in the handler or trap path | each covers it |
| 19 | gap | the panic measurements and the inventory not durable | the probe's source and answers recorded in §14.4, a premise re-run with §14.1's; the inventory recorded above |
| 20 | gap | locators limited to the port's own code, though facts rest on the runtime API record's | "its own implementation or a `describes` record's, as §2 admits" |
| 21 | gap | the depth rule's window exception for `unmask` only | "save, for either", `mask` past `M` included |
| 22 | gap | vectored mode's four-byte slots rest on the assembler's encoding choice (`c.j` is two bytes) | vectored mode refused; `vector-direct` obligatory |
| 23 | drafting | "`S` bounds … idle wake" misread the variant; `rt-reference`'s comment stale | "the switch out of idle is `S`'s, the wake before it `W_wake`'s"; the comment updated |
| 24 | drafting | *Still open* still listed the check per selection; "facts" without costs; "of the runtime's state" dropped | removed; "facts and costs"; restored |
| 25 | drafting | the amendments unrecorded in the headers | the contract's amendments paragraph names `M2.12.3`; the composition's line marked |
| 26 | drafting | `M4.6` untouched; an `unknown` fact's meaning for F26 | `M4.6` cites §14.4; a port leaving one undeclared cannot back the fixtures |
| 27 | drafting | §14.4's place in the rules version; §14's summary | part of what `archogen-catalog/1` names; the summary names it |
| 28 | drafting | "or after it in the window" ambiguous | "or after its check, in the window before its failure is raised" |
| 29 | nit | a tautology; "the handler's symbol" LTO-dependent | reworded; "ends in the handler" |
| 30 | nit | absent and `unknown`; no `ecall`, `ebreak` or `unimp` in the dialect | both undeclared, said; no record's code can trap deliberately or enter the API by a trap on `riscv64` today, said |

## Why

The port's statement is what the fault contract's fifteen rounds left open; it is reviewed as that contract was,
until a round finds nothing.

## How to apply

- A new round appends its paragraph and table here, and one row to the summary in §14's record.
- A finding is answered in the record first, and its row here names what answers it.
