# The runtime variant's composite inputs: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `M2.10.1` (`docs/tasks/M2.md`). This is the review history of
  [[decision_runtime-composite-inputs]], kept apart from it as `docs/reviews/INDEX.md` describes.

## The fact / decision

`M2.10.1`'s acceptance is a review, by a context that did not write the record, that finds no composite
under-charging against the runtime variant's §1 definitions. Each round below is one such review, with every
finding and the answer the record gives it. The section numbers are the record's as answered.

Each round is a new context, read-only, which had not written the record. It is given:
- the record;
- `decision_runtime-analysis-variant.md`;
- the catalog record's §2, §7 and §12;
- `ROADMAP.md` §7.3–§7.5 and §10.3;
- `crates/rt-core`'s scheduler.

It is barred from other implementation, and may check external specifications on the web.

**Round 1**, `2026-09-30`: the `C_i` sum, the max-of-sums form of `CS_i` and the `J` fixed point were judged sound
within the model. The reviewer checked the fixed point numerically against a simulation of the record's own model,
on 1 920 admitted random sets. There were no violations when the interrupt is chosen at the moment the trap is
taken, and 156 violations, each at most `δ − 1`, when delivery commits at its start (K2). The reviewer also
fetched the ratified privileged specification and confirmed the record's M-mode interrupt order. There were 19
findings, 3 of them defects, and the verdict was "Not acceptable as it stands". The answers keep the variant's
record and its code unchanged. Each new requirement is a precondition of the composition (§2), with a verdict when
it is declared false. The specification's two sentences were re-read by the answering context before the ledger
entry `riscv-privileged` was written, and that settled the variant's own open question about `mret`.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| K1 | defect | a job that completes while still masked: a run ended by completion was in no term, so `CS_i` and every `J` built on it under-charged (a true 30 against a composed 7 or 27) | a run ends at its `unmask` or at the job's completion, and the application says which. A run ending at completion adds `completion` in place of `unmask` (§1, §3, §4) |
| K2 | gap | where delivery ends and a service begins was not pinned, and masked time after the processor commits to a trap belonged to no part | `δ` covers arrival or compare match to pending, plus pending-and-unmasked to the trap being taken, and every service begins at the trap. At least the variant's `δ`, so its floors stay safe (§2) |
| K3 | defect | a compare written late during initialisation, whose masked run `L` does not hold | the code fact `releases-after-initialisation`: every nominal release is at or after the first unmask of the running system (§2, §4) |
| K4 | gap | nothing said that a task's own `unmask` lets a pending interrupt in, so two runs could fuse | the fact `pending-taken-after-unmask`. The privileged specification requires evaluation "immediately following … an explicit write to" `mstatus` or `mie` (§2, §4) |
| K5 | defect | a source's stops gave `not-established` for no fixed point and overflow, but `unsupported-profile` past `T_s` | every source stop means `J_s` cannot be bounded below its no-loss limit: condition 7's `unsupported-profile`. The timer's stops stay `not-established`; limits are `analysis-inconclusive`. The limit at exit is `T_s − C_s − S` (§3, §6) |
| K6 | gap | interrupts behind a stopped one had no `J_q`, and an iterate past a refusal bound was handed on | a stopped interrupt has no value; everything behind it is not composed and is named under the stop. No iterate is passed on (§3, §6, Why) |
| K7 | gap | three verdicts unstated: the pre-check's rational outside 128 bits, the fact declared `no`, a non-strict order | `analysis-inconclusive`, `unsupported-profile` and `unsupported-profile` (§3, §6) |
| K8 | gap | the timer's place in the order is fixed by hardware, not configured by the plan | the hardware facts `external-before-timer` and `every-source-external`; the plan orders only the sources (§1, §2) |
| K9 | ambiguity | `rt-core`'s `unmask` does deliver releases latched while its mask depth is raised | the code fact `releases-never-latched`: the port's ordering means no service runs while the depth is raised, so the path is never taken on the target (§2, §4) |
| K10 | ambiguity | "from the call to its return" is undefined once a primitive is inlined | the code fact `primitives-out-of-line`: out-of-line primitives with barrier masking instructions, so boundaries are instructions (§1, §2, §4) |
| K11 | nit | the category order is the record's choice, not §7.3's; §7.3's "unless a valid argument" dropped; categoryless parts | all three stated (§6) |
| K12 | nit | the timer count cited the weaker fact | each service maps to the release that was due when its interrupt was raised (§4) |
| K13 | ambiguity | whether a caller-supplied `C_s` stops the `J` composites | it does not; the composites built on it are composed and the figure is named (§6) |
| K14 | nit | the application's entry-state declaration was not among what §12 gains | listed, with each run's ending (§5) |
| K15 | nit | "switch ends at the job's first instruction" did not fit a return | a return ends at the resumed instruction (§2) |
| K16 | nit | the timer's refusal bound is undefined with no timer-released task | `Δ_timer` is then not computed (§3) |
| K17 | nit | "the most by which" overstated a bound | "an upper bound on how late" (§4) |
| K18 | nit | platform-local interrupts and the Advanced Interrupt Architecture can reorder the timer | `every-source-external` puts them outside; the ledger entry quotes AIA §4.1 (§2) |
| K19 | nit | a future primitive that returns masked or unmasks would be missed | the new-primitive rule classifies each before it is costed (How to apply) |

**Round 2**, `2026-09-30`: K1, K3–K12 and K14–K19 were judged closed, and K2 and K13 partial. The reviewer
confirmed §12's names, owners, co-location pairs and fact kinds against the record, and re-read the privileged
specification's two sentences. It also noted the clause that mattered: trap conditions are evaluated "in a bounded
amount of time" after an interrupt becomes pending, and immediately only after `xRET` or an explicit CSR write.

It simulated the record's model, 1 991 admitted sets per setting:
- no violation when masking cannot overtake a pending interrupt, or when there is no latency after the unmask;
- violations in 1 529 sets when both can happen, each at most the second part of `δ`. That is L1.

There were 11 findings, 1 of them a defect, and the verdict was "cannot be accepted as it stands"; nothing else
under-charges once L1 is fixed. The answer to L1 was checked on the reviewer's own example. There, `B_s = L + 2δ`
gives 24 against the true 23, and task B's bound becomes 30 against its deadline of 24, so the miss is no longer
reported as holding.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| L1 | defect | an interrupt pending while code runs unmasked; the code masks before the trap, and delivery is paid again after the unmask, so `B = L + δ` misses up to the second part of `δ` (a deadline miss reported as holding) | `B_timer = ρ + L + 2δ`, `B_s = L + 2δ`. §2 says delivery is time, not a barrier, and §4's step 2 bounds the wait by `2δ + L` before the queue (§2, §3, §4) |
| L2 | gap | three image-wide facts are owned by code facts whose locators reach only catalog code | the application task fact `leaves-interrupt-hardware-alone`, and the caller's declaration of it for a service's application code, composed with the kernel's facts (§2, §5, §6) |
| L3 | ambiguity | a run that ends at `unmask` on one path and at completion on another | each ending is an alternative, with its own figures (§1, §3) |
| L4 | ambiguity | `rt-core`'s `complete` leaves the mask depth raised | `releases-never-latched` covers a completion entered with the depth raised: the port lowers it, inside `completion` (§2) |
| L5 | ambiguity | the variant's "entry" against the trap; a caller's `C_s` bound only to entry | "entry" is the trap being taken, and a caller-supplied `C_s` begins there (§2, §6) |
| L6 | gap | no verdict for an overflow while composing `C_i`, `CS_i`, `L` or `B` | `not-established` for `C_i`; every interrupt stopped as by its own overflow for the others (§3, §6) |
| L7 | gap | a stop leaves the whole set unbounded; interrupts behind a lower-precedence stop went unchecked; a real fixed point past the bound was discarded | stated; each interrupt behind a stop is checked through `B_y`; a fixed point reached is a value, which the variant judges (§3, §6) |
| L8 | nit | `T_s` is an arrival assumption, not configuration | named as one in the conclusion (§6) |
| L9 | nit | where `pending-taken-after-unmask` comes from differed from §12 | aligned; no rule pairs two facts yet, which the catalog record's round 9 takes up (§5) |
| L10 | nit | §12's `J` row omits the description's parts | taken to §12 with the catalog record's round-9 answers |
| L11 | nit | the timer mapping's wording | a release due at the raising that the service performs, so the mapping is one to one (§4) |

**Between rounds 2 and 3**, `2026-09-30`: the catalog record's round 9 found four defect-level items in the names
§12 had taken from this record (I1–I4). Its answers changed this record too:
- `every-source-external`, which spoke of a description's sources, is now `external.<source>`, a hardware fact per
  source in that source's `service.<source>` group;
- every fact here is behavioral, and every pairing is one of §12's groups, checked per selection;
- "the record that supplies the runtime API" is the one that supplies `completion`.

Round 3 re-checks this record with those changes.

**Round 3**, `2026-09-30`: L1, L3–L8, L10 and L11 were closed, and L2 and L9 partial. The cross-file check against
§12 agreed on every name, owner, kind, facet and group, apart from M5. It found 13 findings, one a defect, M1, and
the verdict was "cannot be accepted as it stands". The reviewer read the privileged specification's timer section
from the manual's source, since the pinned rendering stops before it. Its simulation, biased toward late writes,
found 183 and 240 violating sets of 1 500, and none with fix (a). The answering context re-derived M1's
counterexample by hand: a true 60 against a composed 42, and 74 with the fix. It re-read the specification's
sentence at the manual's `main`, commit `51c1291`, line 2566, and added it to the ledger entry `riscv-privileged`.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| M1 | defect | a late compare write passing through a value above the counter drops the interrupt until after the service, so a full masked stretch fits before it is pending again | `B_timer = max(ρ, C_rel + S) + L + 2δ`; steps 1, 2 and 5 restated; the specification's "eventually, not necessarily immediately" in the ledger (§3, §4) |
| M2 | gap | an arrival during initialisation waits out its masked run | an arrival assumption, named like `T_s`: no source arrives before the first unmask (§2) |
| M3 | gap | the timer's counter and the controller's claim registers were covered by no fact | both facts widened to the counter and the claim and complete registers (§2) |
| M4 | gap | catalog code in records other than the facts' owners, drivers and services, was covered by nothing | `interrupt-hardware-discipline.<id>`, stated by every record about its own code, required from every closure implementation, and from `M4` the image's (§2) |
| M5 | gap | §12 held the application's fact more narrowly | §12 takes the full wording and the caller's declaration (catalog round 10, J4) |
| M6 | ambiguity | `external-before-timer`'s `no` against the `no` rule | exempted (§2) |
| M7 | ambiguity | a caller's `C_s` with no record anchoring `service.<source>` | every declared source needs such a record, with its cost `unknown` if it has none; otherwise undeclared (§6) |
| M8 | gap | an enabled source never delivered | the plan states every source deliverable; one that is not is `unsupported-profile` (§1, §6) |
| M9 | nit | a numerator overflowing while the quotient is small | numerators formed as the variant's §2 forms them (§3) |
| M10 | nit | the order of the fixed-point and refusal tests | fixed point first (§3) |
| M11 | nit | `pending-taken-after-unmask`'s kind worded unlike §12 | aligned (§2) |
| M12 | nit | the ledger still named `every-source-external` | renamed (the ledger) |
| M13 | nit | which composites an unreadable fact gates | listed (§2) |

**Round 4**, `2026-09-30`: M2, M3 and M5–M12 were closed, and M1, M4 and M13 partial. The reviewer simulated the
model with the compare's rounding at its full width, and a late write dropping the interrupt: 28 of 1 500 biased sets
exceeded the record's `B_timer`, none the summed form. It read the PLIC specification 1.0.0 for N2. `C_i` and `CS_i`
were judged sound. There were 9 findings, 2 of them defects, and the verdict was "cannot be accepted as it stands";
nothing else under-charges once N1 and N2 are fixed and N3 is stated. The answering context checked the new base term
against both counterexamples: 115 against a true 102, and 74 against 60. It re-read the PLIC sentences at their
source, `riscv-plic.adoc` at `f8ec1b7`, and gave the specification a ledger entry, `riscv-plic`. The self-statement
was renamed `runtime-discipline.<id>`, since it now covers releases; the rows above keep its earlier name.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| M1 | partial | `max(ρ, C_rel + S)` is wrong when `ρ > 0` | through N1 |
| M4, M13 | partial | generated code uncovered; the gating list incomplete | through N4 and N6 |
| N1 | defect | a service that starts before `⌈t⌉` does not release `t`, then writes the compare late: a true 102 against 83 | `B_timer = ρ + C_rel + S + L + 2δ`, with step 1 split on `⌈t⌉` (§3, §4) |
| N2 | defect | a notification that lags a claim makes a trap that claims nothing, which no arrival pays for, and can starve the timer | the port fact `no-empty-claim`, in the `switch` group, with the PLIC specification in the ledger; the variant's same omission filed as `M2.11` (§2) |
| N3 | gap | "external interrupt" named no privilege level, and a supervisor-routed source is taken below the timer | machine external interrupts, through the controller's machine-mode context, in both facts and in deliverability (§2) |
| N4 | gap | generated code is in no record | from `M4` the plan's generator states the self-statement of it; until then the conclusion names it, and compiled records outside the closure, as assumptions (§2) |
| N5 | ambiguity | the self-statement had no initialisation role | initialisation's roles before the first unmask (§2) |
| N6 | nit | the gating list omitted two facts | both placed (§2) |
| N7 | nit | §12's "a hardware fact is in no group" | the exception named (§12) |
| N8 | nit | §12's "whose implementation the claim reads" | the record's wording (§12) |
| N9 | nit | deliverability missing from §12's plan row, and no verdict when omitted | in the row; an omitted statement is `analysis-inconclusive` (§12, §6) |

**Round 5**, `2026-09-30`: N1 and N3–N9 were closed, and N2 partial. The reviewer simulated the model with rounding
up to 31 units, late writes, writes passing above the counter, overtaking, idle wake and sources on either side of
the timer: no violation in 5 900 admitted sets. It read the PLIC specification's source and QEMU's controller for
O1. `C_i`, `CS_i` and `B_timer` were judged sound. There were 8 findings, 1 of them a defect, latent: a board's
conforming gateway re-forwards a level-triggered request on completion, which QEMU's does not. The verdict was
"cannot be accepted as it stands"; nothing else under-charges once O1 is fixed and O2 and O3 are stated. The
answering context re-read the gateway sentence at its source, `riscv-plic.adoc` at `f8ec1b7`, and added it to the
ledger entry `riscv-plic`. It set the leaf a closure rule like the catalog record's (`M2.10.1`). The catalog's
round 12 changed this record in the same change: the compare's first write belongs to the timer-service record,
and "runtime function" is defined.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| N2 | partial | a trap that claims a request no arrival made | through O1 |
| O1 | defect, latent | a level-triggered source completed before its device is cleared gets a second request, so it is served twice per arrival: a true 27 against 10 | the code fact `one-request-per-arrival.<source>` in each `service.<source>` group, cited in step 4 and the gating list; `no-empty-claim` about requests; the variant's same omission joins `M2.11` (§2, §4, §5) |
| O2 | ambiguity, latent | MTIP can stay high after a compare write moves forward, so a timer service releases nothing: s1 served at 18 against 12, and s2 starved | "raised" covers an interrupt still pending after the compare moved on, and the fact's basis shows the hardware's half; the conservatism bullet qualified (§4; the catalog's §12) |
| O3 | ambiguity, live | the roles are stated of code, but `rt-core`'s `release`, `unmask` and `fault` act on a caller's behalf | a function's writes and releases take its caller's role; a path another fact makes unreachable is named in the basis with that fact (§2) |
| O4 | gap, latent | the hart's interrupt state was in no role | `mstatus.MIE`, `mie`, `mideleg`, `msip` and hart-local priorities are in the roles and the task fact; a hart whose local priorities can reorder the timer writes `external-before-timer` `unknown` (§2) |
| O5 | nit | initialisation could not drain requests left by firmware; "unmask" named two things | initialisation may claim and complete before the first enabling of interrupts, and a request left pending counts as an arrival otherwise (§2) |
| O6 | nit | `⌈t⌉ = t + ρ`, and step 1's cases | `≤`, and the first case stated so the two cover every timeline (§4) |
| O7 | nit | `no-empty-claim` stated a mechanism software cannot observe | the property stated, the mechanism the basis's, and `yes` with no external source (§2; the catalog's §12) |
| O8 | nit | §5's "the packages it compiles" | §2's wording (§5) |

**Round 6**, `2026-09-30`, the first judged under the closure rule: O2–O4 and O6–O8 were closed, and O1 and O5
partial. The reviewer read the PLIC specification and QEMU 11.1.1's `sifive_plic.c` and `goldfish_rtc.c`, and
simulated the model on 900 admitted sets per setting: no violation once the §2 facts hold. `C_i` and `CS_i` were
sound, and `J^release` and `J_s` sound within the model. There were 10 findings, 6 live and 4 latent. Two defects
were live on the emulator, both through facts: P1 and P2. The verdict was that the record "does not meet its closure
rule". The answering context fetched `hw/intc/sifive_plic.c` at `v11.1.1` and confirmed both at their lines (a read
of the claim register claims, at 163–174; any raise of a line pends, claimed or not, at 353–360), and the PLIC
specification's claim sentences at their source, and put QEMU's behaviour in the ledger.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| O1 | partial | through P2 | as P2 |
| O5 | partial | through P4 | as P4 |
| P1 | defect, live | a claim is a register read, and both facts forbade only writes; a task that claims starves its source | reads and writes of the claim and complete registers governed, a read named as a claim, in both facts and §12; `no-empty-claim` and `one-request-per-arrival` listed among the image-wide facts that need the task fact (§2; the catalog's §12) |
| P2 | defect, live | QEMU pends a request on any raise of a claimed source's line, so clearing before completing is not enough | the property stated alone, and the basis shows it for the target's controller, a PLIC gateway or QEMU's `sifive_plic`; QEMU's behaviour in the ledger; the variant's same omission, `M2.11`, now live too (§2; the catalog's §12) |
| P3 | ambiguity, latent (board) | a PLIC gateway holds a new arrival until the completion, so "acknowledged at entry" is unclear | for a PLIC source, `yes` means the completion, and a level device's clear, happen at entry (the catalog's §12) |
| P4 | ambiguity, latent (board) | the start-up assumption against a drained source forwarded again | stated about requests: after initialisation's last completion, none but an arrival's at or after the first enabling (§2) |
| P5 | gap, latent | under `external-before-timer` `no`, a claim loop serves several sources ahead of the timer | `no` is `unsupported-profile` in `/1` (§1, §2) |
| P6 | ambiguity, live, a false refusal | masking by the port's transitions, idle wake and fault path had no role | those roles added (§2) |
| P7 | gap, live in principle | `mtvec` and `mscratch` were in no role | the hart's trap state, in the roles and both facts (§2) |
| P8 | nit | step 1's cases rest on `timer-event-driven`, uncited, and omit the on-time service | cited, and named (§4) |
| P9 | nit, latent | a stale MTIP at the first enabling | `raised-only-when-due`'s basis covers initialisation's first enabling too (the catalog's §12) |
| P10 | nit | "no external source" against "no external interrupt"; `one-external-controller`'s vacuous case | aligned, and stated in the record (§2; the catalog's §12) |

**Round 7**, `2026-09-30`: its first launch stopped at once on the account's weekly usage limit, and a relaunch
ran. P1, P5–P7, P9 and P10 were closed, and P2–P4 and P8 partial. It read QEMU 11.1.1's `cpu.c`, `cpu_helper.c`,
`sifive_plic.c`, `serial.c` and `goldfish_rtc.c`, and simulated the model: no violation with external interrupts
first, and 276 of 609 admitted sets over-run `J_s` under QEMU's order composed as if it were the specification's.
`C_i` and `CS_i` were sound, and `J^release` and `J_s` sound where external interrupts come first once Q2 is fixed.
There were 11 findings, 1 a defect. The answering context fetched `target/riscv/cpu.c` and
`target/riscv/tcg/cpu_helper.c` at `v11.1.1` and confirmed the order at lines 880–883 and 368–373, checked that the
target's device tree offers no `smaia`, and put the order in the ledger.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| P2, P3, P4, P8 | partial | through Q2, Q4, Q9 and Q3 | as those rows |
| Q1 | gap, live | QEMU 11.1.1 without Smaia takes the timer before external interrupts, so `external-before-timer` is `no` on the emulator and `/1` composed nothing there | `no` admitted with the port fact `one-claim-per-trap`; the premise that the hart takes external interrupts first corrected; QEMU's order in the ledger (§1, §2, Why; the catalog's §12) |
| Q2 | defect, live | a declared source's device registers were governed by no fact, and QEMU's UART re-raises on an enable write | only the source's service and initialisation access them; initialisation leaves no request pending at the first enabling that no arrival made; `sifive_plic`'s basis restated (§2; the catalog's §12) |
| Q3 | ambiguity | step 1 read "the earliest release not yet performed" into `timer-event-driven` | stated of `reprograms-only-in-service`, and cited from it (§2, §4) |
| Q4 | gap, a lost release | an arrival between the trap and the claim merges into the pending request | a PLIC source states `acknowledge-at-entry` `no`, so its no-loss limit is the at-exit one (the catalog's §12) |
| Q5 | nit | claims had no caller's role; the port's dispatch | writes, claims and releases take the caller's role; the dispatch acts in its service's (§2) |
| Q6 | nit | a task calling the completion path broke the task fact | the completion path allowed (§2; the catalog's §12) |
| Q7 | nit | `mscratch` in transitions | allowed there (§2) |
| Q8 | nit | "in the variant as here" | "outside this composition" (How to apply) |
| Q9 | ambiguity | the start-up assumption read as holding for the whole run | scoped to firmware's and the boot loader's leftovers; what code does is the facts' (§2) |
| Q10 | nit | the fault path's masked run in no term | assumed: no fault trap is taken (§2) |
| Q11 | nit, latent | Sstc's registers | `stimecmp` and `menvcfg.STCE` in the hart's interrupt state; the timer is the one the timer-service record programs (§2) |

**Round 8**, `2026-09-30`: Q3–Q11 were closed, and Q1 and Q2 partial. The reviewer read QEMU 11.1.1's
`riscv_aclint.c` and `serial.c` besides the files before it, and simulated the model under QEMU's timer-first
order: no violation in 6 300 admitted sets, with late writes, coarse rounding, commitment at the trap or at
delivery, dense sources and QEMU's latch. Every counterexample came through a fact's wording. There were 8
findings, 3 of them defects live on the emulator, and the verdict was that the record "does not meet its closure
rule"; once they are restated, `C_i` and `CS_i` are sound and `J^release` and `J_s` sound under both orders. The
answering context restated each in the record and in the catalog's §12, which changes now through this record's
rounds, `M2.7.1` having closed; catalog round 16's V7 landed with them.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| Q1, Q2 | partial | through R1, R2, R3 and R4 | as those rows |
| R1 | defect, live | `one-claim-per-trap` bound external traps only, and a timer trap whose exit drains the controller put services ahead of the timer: 22 against 14 | every trap serves exactly one interrupt, a timer trap claiming nothing; a service ends at its trap's return (§2; the catalog's §12) |
| R2 | ambiguity, live, a false refusal | §2's opening, §6 and step 4 still put `no` outside `/1` | the exception stated in both general rules, and step 4's phrase dropped (§2, §4, §6) |
| R3 | defect, live | application initialisation was bound by nothing, and could reorder the controller: 21 against 13 | the task fact binds all application code, initialisation included, which reaches the hardware only through catalog functions (§2; the catalog's §12) |
| R4 | defect, live in principle | the fact bounded requests per arrival, not requests no arrival made: 32 against 24 | every request a service claims was made by an arrival, and each arrival makes at most one; QEMU's window starts at the first enabling too; step 4 cites both halves (§2, §4) |
| R5 | nit, a false refusal | no compare value when no release remains; step 1's "always" | or its largest value; "after each write" (§2, §4) |
| R6 | gap | the no-fault assumption was not named in the conclusion | named beside `T_s` and the start-up assumption (§6) |
| R7 | gap, a false refusal | initialisation's role could not read a device | it reads and writes each declared source's device to configure it (§2) |
| R8 | nit | §5 lacked `one-claim-per-trap`, and §12 filed its kind apart | listed in §5, and placed with the port's trap facts in §12 (§5; the catalog's §12) |

## Why

The record states the composition as it stands, and this file keeps how it got there, as for the catalog record.

## How to apply

- A new round appends its paragraph and table here, and one row to the summary table in the record's `## Review`.
- A finding is answered in the record first, and its row here names the sections that answer it.
