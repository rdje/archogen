# The port's statement: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `closed` — `2026-10-02`: round 7 found no defect, and §14.4 was decided in the change that answered it
- **External sources:** [the Rust Reference](../book/src/ledger.md#rust-reference) shipped with the pinned toolchain,
  and [the RISC-V privileged specification](../book/src/ledger.md#riscv-privileged) — versions, hashes and limits in
  the ledger
- **Owner / source:** leaf `M2.12.3` (`docs/tasks/M2.md`). The design under review is §14.4 of the catalog record, kept
  in [`decision_catalog-records-port.md`](../specs/catalog/decision_catalog-records-port.md), with the amendments it
  makes to the fault contract (`docs/profiles/rt-static-up-v1-faults.md`), the composition record, §2, §3, §9, §11,
  §12, §13 and §14's summary of the catalog record, its §5 and §14.1, findings §12, `M4.6`, `M4.10`, `M2.12.4`, `M2.7.4`, and
  the book's `annex-runtime.md`, `catalog.md` and `runtime.md`. Section numbers are the catalog record's.

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

**Round 2**, `2026-10-02`, of commits `d13c870` and `b551072`, by a new context that went through the contract
itself, with compile-only probes on the pin. It found the convention record through `depends` mechanical and sound
in its core, and the contract's amendments sound in substance, but 6 defects: §14.4's facts placed in the `switch`
group made §13's port-fact rule read two ways; §14.4 stated falsely that no `riscv64` code can trap deliberately (a
load from an address nothing answers is one, in the dialect and in Rust); a read condition let F26 owe a fixture that
cannot arise; a basis clause the contract requires had been dropped; a fact the contract requires was not obligatory;
and the inventory above claimed 15 ambiguities while listing 13. 30 findings: 6 defects, 11 gaps, 10 drafting points,
3 nits. Defects per round: 9, 6.

The inventory's two missing ambiguities, recovered from its report: whether "each such check" and "each guard check"
agree — they do, the contexts stated being guard checks', since a check finding an unexpected trap has its raiser
fixed by the Terms; and the port-dependent points the contract relies on but *Still open* omits — another stack the
port guards, where the port places the decision, and a further timer interrupt found by its pending bit — answered by
`guarded-stacks`, `decision-placement` and `one-claim-per-trap`'s basis. The inventory's quotations were not kept in
the repository: its entries are recorded above by name, the contract they quote is the source.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | §14.4's facts in the `switch` group made §13's and §12's "every `switch`-group fact … known only as §14.2 states" bind them, against §14.2's twelve | §13 and §12 narrowed to "each of the twelve facts §14.2 names"; §14.4 says its facts need no locator into declared assembly |
| 2 | defect | "no record's code can … make a deliberate trap" false: `ld t0, 8(zero)`, `jr zero`, `read_volatile(0x8)` | the conclusion deleted; `checks-trap` added, scoping the trapping cases |
| 3 | defect | `window-primitive-consistent` readable where no release can be observed inside a primitive, owing an impossible fixture | read only when `window-release-abandons` and `window-trap-preempts-inside` are both `yes` |
| 4 | defect | the basis clause "a call through its own runtime-API trap included" dropped from `detects-non-job-calls` | restored |
| 5 | defect | `generated-guard-check-contexts` not obligatory, though the contract requires it | obligatory, `unknown` admitted until `M4` |
| 6 | defect | the inventory claimed 15 ambiguities, listing 13; "each quoted" untrue of the record | the two recovered and answered above; the quotations' loss said |
| 7 | gap | obligatory facts and the costs could be absent, and absence is not refused | the port record supplies each obligatory fact and both costs, or the catalog is refused at load |
| 8 | gap | the convention record's form unstated against §6, §7 and §12 | catalog `interfaces`, implementation `none` enforced, profiles and targets admitting its dependers', a self-named `convention-stated.<id>` fact |
| 9 | gap | which review judges conformance; "stale" cited to §10 | the behavioral model's review; §3 and §5 cited, the contract, behavioral and timing reviews named |
| 10 | gap | §14.4's names' kind and facet fixed nowhere the loader checks | rows in §12's table: behavior-model code facts, `panic-strategy-abort` with a `file` locator, `fatal-path.*` timing costs |
| 11 | gap | nothing obligatory carried the window whole | `fault-window`, obligatory, its basis the whole window |
| 12 | gap | other records had no form for their guard-check contexts | `guard-check-contexts.<id>`, self-named, outside every group |
| 13 | gap | the compiler-prologue case, the window's contexts and the trapping cases unscoped | each scoped: entries named as compiled functions, the window facts' contexts, `checks-trap` |
| 14 | gap | one undeclared fact disqualified every fixture, and one stays `unknown` until `M4` | each case read by its own fact; production waits for `M4`'s generated checks, said |
| 15 | gap | §14.4's measured premises neither reproducible nor carried forward | each probe re-run here, its source hashed and recorded; How to apply and `M2.12.4`'s acceptance carry them |
| 16 | gap | an image under a selection with no `switch` escaped the check; the entry's name only in prose | `M4.10`: images only where a record supplies `switch`; the entry's path exposed by a field of `M4`'s design |
| 17 | gap | combinations that cannot be stated or are not tied together | the bases state them and the review judges them, said |
| 18 | drafting | what the check refuses and what it names | the catalog refused at load, naming the port's `depends` or the depending record's |
| 19 | drafting | "that record's §3", "that record's §14.4" in the contract | "the catalog record's" |
| 20 | drafting | three `M2.12.3` edits unmarked | marked |
| 21 | drafting | the director note's "narrows no claim further" | the two small consequences stated: the raising later by the handler's call, the window holding generated code |
| 22 | drafting | the depth rule's exception for `mask` can never apply: a `mask` past `M` holds a region, and rule 5 abandons no job holding one | reverted to `unmask` alone, with the window's case |
| 23 | drafting | F26's polarity and keying ambiguous | each F26 phrase mapped to its fact; "not where it states the opposite" |
| 24 | drafting | "the contract requires each" wrong for `vector-direct` and `panic-strategy-abort`; the strategy's locator | both said to be §14.4's narrowings; `abort` the target's default; a `file` locator |
| 25 | drafting | "a trap may preempt" read as including exceptions | "an interrupt may preempt" |
| 26 | drafting | "necessary, not sufficient" named one case | both named: no convention, or the right one with nonconforming code |
| 27 | drafting | the versioning claim unmatched in §3 or §5 | narrowed to the names, which §3's bullet now carries |
| 28 | nit | positional references | named |
| 29 | nit | "in the primitive or in its trap" dropped | restored |
| 30 | nit | `one-claim-per-trap`'s basis duty recorded once | in the composition record's row and §12's table too |

**Round 3**, `2026-10-02`, of commit `66cf1f9`, by a new context asked first for regressions. It found the
convention record through `depends` mechanical and sound, every port obligation covered with the right shape, and the
contract's amendments keeping every settled rule, the depth rule's exception now right; and 4 defects, three of the
regression or false-statement kind: the inside run-on fixture still owed where no interrupt reaches a primitive; §12's
"§14.4's facts" sweeping in the convention record's own fact and refusing every catalog with a port; `M4.6` still
carrying the rule round 2's 14 replaced; and probe sources said to be recorded that were not. 19 findings: 4 defects, 3
gaps, 6 drafting points, 6 nits. Defects per round: 9, 6, 4.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | the inside run-on case owed with `primitives-preemptible` `no`, where no release can be observed in a primitive | owed inside only where `window-trap-preempts-inside` is `yes`; outside likewise keyed on its own fact |
| 2 | defect | §12's "§14.4's facts" took in `convention-stated.<id>`, making it a `switch`-group code fact and refusing every valid convention record | "the facts of §14.4's table"; §12 rows for `convention-stated.<id>` (not a code fact, a `file` locator) and `guard-check-contexts.<id>` (code), each self-named, outside every group |
| 3 | defect | `M4.6` still said one undeclared fact disqualifies every fixture | `M4.6` now reads each case by its own fact |
| 4 | defect | the probe sources "recorded" were not: four hashes with prose only, not re-derivable | each source printed verbatim with its command, and its hash re-derived from the printed text before landing |
| 5 | gap | facts the contract says the port states could be absent | the port record supplies every fact whose read condition holds, and `one-claim-per-trap`, as well as the obligatory ones |
| 6 | gap | the further timer-interrupt subcases keyed only on `one-claim-per-trap` `no` | scoped by its basis: a timer service run for one, or a claim in a timer trap, only where the basis says so |
| 7 | gap | `generated-guard-check-contexts` a code fact about code in no record; generated code's checks outside `checks-trap` | a `file` locator, like `panic-strategy-abort`, a `code` or `ledger` one refused; generated code's checks said to be `M4`'s |
| 8 | drafting | the read-condition refusal on an absent or `unknown` conditioning fact | a condition holds only when every fact it names is stated `yes` |
| 9 | drafting | the convention record's form mixed refusals with descriptions; contract or file authoritative | each refusal named with its code; the contract governs, the file a copy |
| 10 | drafting | the refusal's reason missed rule 2's other clause | the trap of an entry naming neither counts as a primitive, said in the fact and the reason |
| 11 | drafting | "such a check" lost the contract's "that finds a fault" | quoted |
| 12 | drafting | the prologue case depended on what `fault-path-entries` did not state | its basis states, per entry, naked or compiled; the case mapped per entry and route |
| 13 | drafting | no polarity for the completion-interval cases | the waiting-context cases owed where `services-preempt-completion-interval` is `yes` |
| 14 | nit | positional references remained | named |
| 15 | nit | three contract edits unmarked; the header's R1–R15 sentence; "which that record names" | marked; the header names `M2.12.3`'s reviews; "the port's catalog record" |
| 16 | nit | §13's heading overstated; "a known port fact" undefined | "the twelve port facts §14.2 names" in both |
| 17 | nit | the history's header list; the id prefix unversioned | §3 and §13 added; the prefix part of the grammar |
| 18 | nit | Cargo syntax for a `rustc` probe; the `c.j` bytes; "locators into the code that does it" | each probe's command given; "a two-byte compressed `c.j`"; "where code does it" |
| 19 | nit | the obligatory-`no` rule's reach; `panic-strategy-abort`'s other locators | the port's record only, the sole supplier of the group; `code` and `ledger` locators refused |

**Round 4**, `2026-10-02`, of commit `7d2dd36`, by a new context asked first for regressions. It re-derived all four
probe hashes from the printed sources and ran every command, each answering as stated, and found the core sound; but 3
defects, two of them regressions from round 3's answers: defining when a read condition holds turned
`window-release-abandons`'s "or" into "and", refusing valid records; quoting "a check that finds a fault" into
`checks-trap` dropped the invariant checks F26's trap case keys on; and §11 and §2 did not carry round 3's new
`catalog-locator` and `catalog-dependency` refusals. 17 findings: 3 defects, 2 gaps, 7 drafting points, 5 nits.
Defects per round: 9, 6, 4, 3. Its stronger panic-strategy premise was reproduced here before it was written.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | "every fact it names is stated `yes`" made an "or" condition an "and" | a condition's facts combined as it says, each for `and`, one for `or`; absent or `unknown` not `yes` |
| 2 | defect | `checks-trap` narrowed to checks that find a fault, leaving F26's invariant-check trap case unkeyed | both scopes restored: a check that finds a fault, or a failed check of a record's own invariants |
| 3 | defect | §11 and §2 lacked round 3's `catalog-locator` and `catalog-dependency` refusals; the convention fact's locator filed unevenly | §11's locator and dependency rows name §14.4's cases; §2 says a file-located fact takes a `file` locator and no other; the convention fact's locator case `catalog-locator` too |
| 4 | gap | further-pending subcases partly scoped: a later claim, a timer-trap claim for an external interrupt, the several-trap delivery | each subcase scoped by `one-claim-per-trap`'s basis; the several-trap case where a delivery can span several |
| 5 | gap | the catch-all reached only contexts or points; API-trap, check-route and stack-guard cases unscoped | `api-entry-by-trap`'s basis names its entries; the catch-all takes contexts, points, entries, routes and stacks, the convention's routes and `guarded-stacks` among them |
| 6 | drafting | the presence rule's facts with no read condition | "every fact of the table that has no read condition, or whose read condition holds" |
| 7 | drafting | the convention record's one fact possibly no longer required | "exactly one fact, `convention-stated.<id>`, `yes`", a single `file` locator |
| 8 | drafting | the inside run-on clause's grouping; inside abandonment not keyed on consistency | "and either …"; inside abandonment owed only where `window-primitive-consistent` is `yes` |
| 9 | drafting | `panic-strategy-abort` against F26's "whose stated strategy aborts" | F26's phrase quoted; the fact's `yes` said not to be such a strategy |
| 10 | drafting | §14.2's `catalog-field` bullet still said "a known port fact" (wrapped, so missed) | "a known value of one of the twelve" |
| 11 | drafting | §3's bullet lacked the id prefix | added |
| 12 | drafting | the contract's routing line omitted the convention record | "the facts, costs and check-passing convention record §14.4 … names" |
| 13 | nit | the hashes match only without the list's indentation | said |
| 14 | nit | the `RUSTC_BOOTSTRAP` premise understated | reproduced and stated: the binary is still refused against the precompiled `core` |
| 15 | nit | rule 2's "for rule 5" dropped | restored |
| 16 | nit | which records the obligatory and within-record rules bind | every record stating these names; only the `switch` supplier read |
| 17 | nit | a non-convention record's `convention-stated.<id>`, and the port's own `guard-check-contexts.<id>` | both refused |

**Round 5**, `2026-10-02`, of commit `a9be943`, by a new context asked first for regressions. It re-derived all five
probe hashes, reproduced every panic-strategy answer, measured E0152 across two crates as well as in one, and found
every answer of rounds 1 to 4 still in the text; but 2 defects. One is a regression of round 4's answers 3 and 7: "other
than a single `file` locator" filed a convention fact with several locators under `catalog-locator` while §14.2 files
it under `catalog-shape`, two codes for one refusal. The other: §3 still said `/1` was amended once, by §14.2, whose
reason, a relaxing, does not cover §14.4's refusals. 12 findings: 2 defects, 1 gap, 6 drafting points, 3 nits.
Defects per round: 9, 6, 4, 3, 2. Its unmeasured premise, that `core` cannot be rebuilt on the pin, was measured
here: `RUSTC_BOOTSTRAP=1 cargo +1.95.0 build -Zbuild-std=core` over `bin_main.rs` is refused for want of `rust-src`.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | a convention fact's several `file` locators refused under two codes | §14.4 refuses a single locator that is not `file` (`catalog-locator`), several being §14.2's `catalog-shape` |
| 2 | defect | §3: `/1` "amended once, by §14.2"; §14.4 tightens it with no reason given | §3: amended twice, by §14.2, which relaxes, and §14.4, which adds names and refusals; §14.4's versioning says why it may: no record or lock exists |
| 3 | gap | a stack-guard case reached by an access owed on a guard only a check finds | `guarded-stacks`' basis says how each guard's reach is found; the access cases owed only where it says the access faults |
| 4 | drafting | "absent or `unknown` cannot be decided" made a case whose condition fails on a `no` undecidable | undecidable where the fact is `unknown`, or absent while its read condition is open |
| 5 | drafting | the outside run-on's "`no` there" missed a release abandoning only inside | "`no`, or `yes` with its basis naming only inside a primitive" |
| 6 | drafting | `M4.6` kept round 3's "contexts or points" | `M4.6` names contexts, points, entries, routes or stacks, and the convention, and the open-condition rule |
| 7 | drafting | `checks-trap`'s basis cannot name another record's checks | the port's own and its `describes` records'; another record's take the convention's routes |
| 8 | drafting | "one for `or`" read as exactly one | "at least one" |
| 9 | drafting | the trap list omitted a store | "such as a load from, a store to, or a jump to" |
| 10 | nit | How to apply omitted round 4's `RUSTC_BOOTSTRAP` premise | added, with `-Zbuild-std`'s |
| 11 | nit | "built `abort` or not at all" rests on the pin holding no `rust-src` | measured and stated |
| 12 | nit | §2's "marks file-located", a term neither §12 nor §14.4 uses | "says has a `file` locator" |

**Round 6**, `2026-10-02`, of commit `11be781`, by a new context asked first for regressions. It re-derived all five
probe hashes, reproduced every toolchain answer, the new `-Zbuild-std` refusal included, and found every answer of
rounds 1 to 5 in the text; but 1 defect, again of the previous round's answer: round 5 narrowed the undecidable case in
the fixtures paragraph and `M4.6`, but not where the port's record paragraph states it a second time, so one input,
a fact absent because its condition failed on a `no`, had two answers. 8 findings: 1 defect, 1 gap, 3 drafting
points, 3 nits. Defects per round: 9, 6, 4, 3, 2, 1. Its rustup premise was not written: the answer scopes the claim
to a toolchain installed from the file alone instead.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | the port's record paragraph kept "absent or `unknown` … leaves that case undecidable" | the same rule as the fixtures paragraph: `unknown`, or absent while its read condition is open, "can leave" it |
| 2 | gap | §14.4's refusals tighten `/1`, yet no leaf names them and nothing orders them before the first lock | `M2.12.4`'s goal names them; the versioning paragraph says no lock is written before them; `M2.7.4` waits for it |
| 3 | drafting | an `unknown` fact under a failed condition, and a case keyed on two facts one of which settles it, read as undecidable | a fact under a failed condition is refused whatever its value; a case is undecidable when its facts, combined as it says, do not settle it |
| 4 | drafting | §3's scope named §14.4's names while its history named its refusals | §3 and the versioning paragraph both put §14.4's refusals in `/1`: a change after the first lock is a new rules version, as §5's "a rule tightened later" needs |
| 5 | drafting | no presence rule for another record's `guard-check-contexts.<id>` | the loader passes a record stating none, and its behavioral review refuses it |
| 6 | nit | the catch-all called a fetch from a guard a stack-guard case, inside nested dashes | "a case reached by an access (…, or a fetch, the last an unexpected trap)" |
| 7 | nit | the `rust-src` premise holds of what is installed, not of the file | scoped to a toolchain installed from the file alone, as CI installs it |
| 8 | nit | the book's guard sentence unconditional, though check-only guards are admitted | `annex-runtime.md`: found because the access faults or by a check, the record saying which |

**Round 7**, `2026-10-02`, of commit `40cc989`, by a new context asked first for regressions. It read every sentence
round 6 changed against the text before it, grepped every other statement of the same rules, re-derived all five probe
hashes and reproduced every toolchain answer, adding side doors of its own — a `staticlib`, and an `immediate-abort`
library linked into an `abort` binary, each refused the same way. It found every answer of rounds 1 to 6 in place and
**no defect**, so the review closes on it. 12 findings: 1 gap, 4 drafting points, 7 nits, one of them in §14.1,
`M2.12.1`'s, reproduced here before it was answered. Defects per round: 9, 6, 4, 3, 2, 1, 0.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | gap | a stack-guard case found by a check owed on a stack no guard check covers | owed only where `guard-check-contexts`, `generated-guard-check-contexts` or a record's `guard-check-contexts.<id>` names a check of that stack |
| 2 | drafting | §14.2 still let `M2.12.4` land after the first lock | "before the first lock, which §14.4's refusals need" |
| 3 | drafting | §5 named only §3's grammar and the ledger-time checks as `/1`'s | "§3's grammar, with what §3 says it versions (§14.2 to §14.4)" |
| 4 | drafting | the inside run-on and abandonment missed a release abandoning only outside | the inside run-on also where its basis names only outside; the inside abandonment "as its basis says" |
| 5 | drafting | `one-claim-per-trap` `yes` alone does not let a delivery span several traps | "on a target with an external source" |
| 6 | nit | the `switch` supplier cited to the composition's §1 | §2 |
| 7 | nit | "that basis" could read as any fact's | "`guarded-stacks`' basis" |
| 8 | nit | `M2.7.4`'s "`M2.12.3` with it" | both closed, and the wait for `M2.12.4` kept |
| 9 | nit | the book's guard owners omitted a stack the port guards | added |
| 10 | nit | rustup's profile decides whether `rust-src` is installed from the file | left as scoped, "as CI installs it": `M2.12.4`'s compile-only test of the premise runs in CI, so a runner that held `rust-src` would fail it first |
| 11 | nit | the header's amended-file list incomplete | completed |
| 12 | nit | §14.1's assembly "verbatim apart from blank and `.cfi` lines" omitted each body's end label, `.Ltmp`, `.size` and `.text` | reproduced on the pin and said |

## Why

The port's statement is what the fault contract's fifteen rounds left open; it is reviewed as that contract was,
until a round finds nothing.

## How to apply

- A new round appends its paragraph and table here, and one row to the summary in §14's record.
- A finding is answered in the record first, and its row here names what answers it.
