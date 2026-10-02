# The runtime analysis variant: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **External sources:** [QEMU](../book/src/ledger.md#qemu) — version, scope and limits in the ledger
- **Owner / source:** leaf `M2.6.1` (`docs/tasks/M2.md`). This is the review history of
  [[decision_runtime-analysis-variant]], moved out of it verbatim on `2026-09-30` by leaf `PROGRAM.37`, as
  `docs/reviews/INDEX.md` describes, when `docs/decisions/` went over its total ceiling. Section numbers are the
  record's.

## The fact / decision

The section below is the record's `## Review` exactly as it stood, four readers on `2026-09-30`: three
independent reviews and `M2.6.3`'s derivation. A later round appends here.

## Review (`ROADMAP.md` §7.4: "reviewed applicability conditions")

`2026-09-30`: the first draft was reviewed by an independent context. It had not written the draft, it was told to
read no implementation, and it worked read-only against published theory, `ROADMAP.md` §7.3, §7.4, §7.4.1 and
§13.4, the profile and the ledger's categories. It confirmed several things:

- the recurrence's structure: jitter in the ceilings and added once to the response;
- the initial value;
- the single-job stop, and that `D_i ≤ T_i` needs no `T_i − J_i` companion;
- blocking once;
- the verdict mapping.

It made twenty findings. The table gives each answer as the first revision made it. The second review then
revised several of them: `B_i` is gone, `W_idle` became `W_wake`, and the floors changed. Where the tables differ,
the second table and the text above stand.

| # | Severity | Finding | Answer |
| --- | --- | --- | --- |
| 1 | unsound | `J_i` was taken from the description with `0` as default, so timer granularity and delivery were lost | `J_i = J_i^event + J_i^release`, the second engine knowledge, never defaulted (§1, conditions 7 and 9) |
| 2 | unsound | `C_i` was defined as ledger *task execution*, leaving the critical sections, kernel services and instrumentation run for the job charged nowhere | `C_i` is everything the job executes between its transition in and its completion, services excluded (§1); a ledger mapping needs an owner on *critical section* (How to apply) |
| 3 | gap | transitions and the idle window also block, and were sound only through an extra `S` the draft called exact | `B_i` includes `S` and `W_idle` (§2); stated as a deviation (§3) |
| 4 | gap | "the timer is a source like any other" is wrong for an event-driven timer, whose arrivals are every task's releases | the timer term runs over every task (§2); `/1` admits only the event-driven timer (condition 6) |
| 5 | wording | "exactly once" is false: a direct switch is charged twice, and the analysed job's own transition out lies outside its response | "at least once" (§3); the analysed job is charged `S`, not `2S`; the observation boundary is stated (§2) |
| 6 | gap | nothing prevented lost interrupt arrivals | the acknowledge point is an input, and condition 7 bounds `J_s` against `T_s` |
| 7 | gap | interrupt priority, nesting and masking were not inputs | `/1` admits no nesting and masking of every interrupt, and refuses the rest (conditions 5, §5) |
| 8 | gap | "every source is declared" cannot be checked against the declaration itself | the declared set must equal the build's enabled set (condition 5); the limit before `M4` is stated |
| 9 | gap | the cost of being preempted was in neither `C` nor `S` | `γ` per interfering job and service (§2), and condition 8 |
| 10 | gap | a conclusion built on observed maxima read as established | observed inputs are named as assumptions of the conclusion (§5) |
| 11 | gap | the boundary between a service and a transition was not stated | the decision to switch is the boundary (§3) |
| 12 | gap | the rounding direction was not stated | costs round up, separations and deadlines round down (§1) |
| 13 | minor | overflow is decidable, and an arbitrary iteration cap turns decidable cases inconclusive | overflow is the busy-period stop, so `not-established`; the step bound is derived and there is no other cap (§2) |
| 14 | wording | the double-charged section was misdescribed | "for any one section at most one of the two happens" (§3). The reviewer's tentative alternative, measuring from the level-`i` busy period with no `J_s`, is not adopted: it was not checked against the literature |
| 15 | wording | releases and arrivals | nominal releases; arrivals for sources (§1) |
| 16 | wording | "one formulation, cited" overstated a combination | assembled from named sources, with the deviations listed (§3); equation numbers left to `M2.6.3` |
| 17 | wording | precedence is not in §5.5, and a counterexample can still come from elsewhere | cites `Verdict::precedence`; a validated witness from elsewhere is named (§5) |
| 18 | wording | the baseline is exact only where a synchronous release can occur | said so (§5) |
| 19 | gap | the review the draft claimed to record was missing | this section |
| 20 | wording | whether application-level masked sections are admitted | they are, as `CS` (§1, condition 4) |

The reviewer's overall verdict was that the recurrence was correctly assembled, and that the record was not yet
fit to implement until findings 1–5 were fixed.

**Second review, same day, of the revision.** A new independent context, under the same constraints, found no term
of the recurrence that under-counts, given true inputs. It confirmed:
- the busy-period stop is consistent with the `k = i` ceiling;
- the monotone iteration reaches the least fixed point;
- the precedence cited is the code's.

The weakness was the admission layer, which is supposed to guard the inputs. Its sixteen findings, and where each
is answered:

| # | Severity | Finding | Answer |
| --- | --- | --- | --- |
| 1 | unsound | the floor `M` took the longest single activity, not the longest contiguous masked run, and it left out hardware delivery. A delay is a sum (masked run, queueing, new arrivals), so no-loss could not be established from `M` | `L` is the longest contiguous run with `+ S`, and `δ`, `ρ` are inputs; the floors are stated as necessary only; the full delay is engine evidence, named as an assumption; the queueing form is an open question (condition 7, §6) |
| 2 | gap | "fires" is not the nominal release: resolution, late reprogramming and edge compare were missed, and condition 6 could not be checked | `J^release` is defined from the nominal release, everything included; level compare, round-up, one counter and every-interrupt-releases are declared facts (§1, condition 6) |
| 3 | gap | ready at service start can precede the nominal release | ready at the later of the two (§2) |
| 4 | wording | after readiness nothing of lower priority runs, so `B_i` only repeated what `J_i` charged; two over-charges were undeclared | `B_i` removed, blocking carried in `J^release` (§2); both over-charges declared (§3) |
| 5 | wording | condition 2 did not name transitions as non-preemptible | named (condition 2) |
| 6 | gap | `T_i ≥ T_s` pointed the wrong way | a task released on every arrival has `T_i ≤ T_s`; a rate limit is an assumption (condition 5) |
| 7 | gap | exit acknowledgement ignored a switch, and `≤` admitted a coincident arrival | `J_s + C_s + S < T_s` (condition 7) |
| 8 | gap | the overflow claim was false for intermediate numerators | numerators formed so a small quotient never overflows; any remaining overflow is the busy-period stop (§2) |
| 9 | gap | the step bound was Θ(`T_i`), a hang in practice | an exact utilisation pre-check; the bound stated as pseudo-polynomial (§2) |
| 10 | gap | several conditions had no input to be checked against | each is a declared task or platform fact; a missing one is `analysis-inconclusive`, an unsupported value `unsupported-profile` (§1, §4, §5) |
| 11 | gap | `CS_i`, `J_s` and the acknowledge point defaulted | no input defaults (§1, condition 9) |
| 12 | gap | three evidence kinds, where §7.3 has four | §7.3's four; every input not analytically established is named (§1, §5) |
| 13 | wording | the idle quantity is pending-to-unmasked, not the masked window | `W_wake` (§1) |
| 14 | wording | instrumentation inside services and transitions | charged in `C_rel`, `C_s` or `S` (§3) |
| 15 | wording | fixed offsets can admit a simultaneous release | said so (§5) |
| 16 | information | §13.4 under the variant: `R_H = 10` holds; `R_L = 50` is not established; the fixture's release service is charged | recorded for `M2.6.3` (§6) |

**Third review, same day, of the second revision**, by a third new context told to break it. It proved the
no-blocking bound directly. Inside the level-`i` window only these run:
- services, each mapped to a release or arrival the ceilings count;
- higher-priority jobs and the analysed job;
- at most two transitions per interfering job, and one for the analysed job.

Folding a blocking time into every jitter reproduces the published `B_i` result. It also confirmed:
- the analysed job's single `S`;
- that the utilisation pre-check is exact in both directions.

It broke one claim, and made twelve other findings:

| # | Severity | Finding | Answer |
| --- | --- | --- | --- |
| 1 | unsound | early release: condition 6 required a service to release every due task, but not *only* due ones. With H (C 5, T 20), L (C 6, T 100, D 20) and K (C 1, T 100), a kernel that releases tasks due within 8 finishes L at 23 while the analysis gives `R_L = 20` | no early release is a declared, checked platform fact (condition 6), refused `unsupported-profile` when false and `analysis-inconclusive` when missing |
| 2 | gap | conditions 2, 4, 5 and 6 had no §1 input, and a scheduler lock would add blocking that `J^release` does not carry | §1's task facts and platform facts; conditions 2 and 4 name them (§1, §4, §5) |
| 3 | wording | if `C` ended at "completion", an interfering job's completion path was charged nowhere | `C` runs up to the decided switch, completion path included (§1) |
| 4 | gap | what a pre-check overflow does was not said | `analysis-inconclusive`, a named resource limit; and a stated iteration budget (§2, §5) |
| 5 | gap | §6's fixture values depended on inputs the record did not pin | the values are withdrawn, and `M2.6.3` pins every input (§6) |
| 6 | wording | the floor demanded the whole resolution | `ρ` is the largest delay the rounding adds (§1) |
| 7 | wording | an absent `jitter` clause looked like a default | it is the language's meaning, not the analysis's default (§1) |
| 8 | wording | the timer is enabled but not a declared source | the declared sources plus the timer; non-maskable interrupts and firmware traps included (§1, condition 5) |
| 9 | wording | a source-released task's nominal release was undefined, and `T_i ≤ T_s` mixes nominal and arrival separation | the arrival; the comparison stated as safe and possibly over-refusing (§1, condition 5) |
| 10 | wording | `C_rel`'s decision, idle/wakeup's split, and a delay below its floor mapped to the wrong verdict | the decision up to the switch point; `W_wake` then `S`; a delay below its floor is `analysis-inconclusive` (§1, §5) |
| 11 | gap (low) | a spurious re-fire, and due-checks on other exits | a timer interrupt is raised only when a release is due; only the timer service releases timer tasks (condition 6) |
| 12 | wording (low) | `L` assumes transitions end unmasked; unmaskable interrupts and traps | a platform fact (condition 5); named in the enabled set (§1) |
| 13 | gap | a lazily deferred context save falls outside `S` | eager switching, or `S` includes the deferred work: a platform fact (condition 5) |

Each round has found something that could under-estimate a response: the first two in the inputs, the third in an
admission condition. So the record is implemented as the contract, and its next check is `M2.6.3`. That context
reads the record without the implementation, derives the fixtures' expected values, and reports anything it cannot
derive from the record alone as a gap in the record. Three points of theory no reviewer settled are §6's open
questions, and `/1` relies on none of them.

**`M2.6.3`'s derivation, a fourth reader, same day.** Besides the results in §6, it reported what it could not
derive from the record alone:

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | gap | condition 1 had no §1 input, and a set with both a missing input and two processors turns on it | "there is one processor" is a platform fact (§1); the implementation already declared it |
| 2 | gap | when several conditions fail at once, the verdict was not stated | every condition is checked, and the highest-precedence verdict wins (§4) |
| 3 | gap | whether a task a source releases counts as its deferred work | no: released tasks are analysed through `released by`, and `deferred` names any other task that runs deferred work (§1) |
| 4 | gap | what `J^event` means for a timer-released task, and which ceilings take it | how far the description lets a release vary; it enters every ceiling, the timer term included (§1) |
| 5 | wording | "below one the right-hand side is bounded" | "at most `c + U·w`, so a least fixed point exists" (§2) |
| 6 | gap | condition 8 could make `γ` charge lost state a second time | `C`, `C_s`, `C_rel` and `S` exclude what `γ` charges (condition 8) |
| 7 | confirmation | the third review's `R_L = 20`, and the §13.4 bounds | reproduced (§6) |
| 8 | open questions | Q1, Q2, Q3 | Q1 settled in the negative with its residue; Q2 and Q3 not settled (§6) |
| 9 | not exercised | the iteration budget, 128-bit overflow, a missing task fact, a rate-limited release, the naming of assumptions | a missing task fact and a rate-limited release now have legs in `runtime_variant.rs`, and the naming of evidence and the catalog's enabled set already had one. The budget and the overflow need inputs past `u64` sums and a million iterations; they are stated as unexercised, not claimed |

It also said it had derived "19 fixtures", where the file holds 18, a miscount in its summary. The fixture file
is its Part 1, copied verbatim.

**`M2.11`'s first review, `2026-10-02`,** of commit `1c61e70`, which added "every interrupt taken is paid for by an
arrival" to condition 5. A new read-only context that had not written it judged the change against §1, §2 and §4,
and built a witness: a set every condition admitted, whose bound for its first task, 27, a trap taken on a lagging
notification, before the arrival it then served, exceeded on the timeline at 33. Its verdict: "not yet fit to close
M2.11"; each fix local.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | a service "for one of its arrivals" could start before that arrival, so the ceiling, which counts arrivals at most `J_s` before a start, missed it: 33 against an admitted 27 | the condition stated at the trap: an interrupt other than the timer's is taken only while a request an arrival made is pending; §2 shows the source term then charges everything a source's services run in the window, a trap's entry cut by the window paid for by the request pending when it was taken; a service's start defined (§1); `no-empty-claim` stated at the trap, and step 4 of the composition record amended likewise |
| 2 | defect | the condition counted interrupts taken, not services, so a claim loop's later service was covered by none | every service, a trap's first or a later one, serves one request; a trap's several services each within their `C_s` (condition 5) |
| 3 | defect | a timer interrupt still pending after its compare moved on was "raised when a release was due", and its service, releasing nothing, was charged nowhere | condition 6 says *taken*; the platform fact's text and field comment follow |
| 4 | defect | the catalog composition lacked `external.<source>`, `leaves-interrupt-hardware-alone` and `runtime-discipline.<id>`, stated `no-empty-claim` at the claim, and gave no rule for `no` or `unknown` | every conjunct listed, `one-external-controller` and `raised-only-when-due` with them; `no` if any is `no`, undeclared if any is `unknown` (the catalog's §12) |
| 5 | defect | "so did `M2.11`'s condition" come before any conclusion: §6, the book and the tests had stated `/1` conclusions | the model is `/2` and every `/1` conclusion void, by the record's own rule (How to apply); the code's `MODEL` and the book follow |
| 6 | drafting | the expected results did not declare the new fact; `facts=all` was read to cover it | said so: stipulated, not declared (How to apply) |
| 7 | drafting | `CONDITIONS`, the text every conclusion carries, omitted the new condition and four older ones | all five added |
| 8 | drafting | "paid for by an arrival" while the timer's are paid for by due releases | "by a due release or by an arrival" |
| 9 | nit | the refusal text read as if only its second half failed | "every interrupt taken is paid for by a due release or an arrival does not hold" |
| 10 | nit | the test's comment called its `AtEntry` uart level-triggered and cleared late | put as a hypothetical |
| 11 | nit | "rounds 4 and 5" | "rounds 4 to 6, N2, O1 and P2" |
| 12 | nit | the ledger's `riscv-plic` and `qemu` scopes named only the composition record | both name the variant's condition 5; `riscv-privileged` names condition 6 |
| 13 | nit | the book omitted QEMU's mechanism, and the changelog said "real hardware" | both corrected |
| 14 | nit | a hyphenated name for a fact that does not exist, and "one composed from others" ambiguous | the field's name; "one with no fact of its own" |

**`M2.11`'s second review, `2026-10-02`,** of commit `08a707b`, the first review's answers, by a new read-only
context given the first witness. Its verdict: the answers close the five defects; the witness breaks condition 5's
trap-time clause, and with that trap removed `i` responds in 23 against the bound of 27; no schedule meeting §4
under the intended readings exceeds the bound. One defect, a false justification with no under-charge, and drafting
points mostly about traps that run several services.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | the composition record's new step-4 sentence appealed to the request pending at the trap, which can be `x`'s own, counted by no ceiling of `Δ_x`; the bound survives by the reviewer's own derivation | that derivation adopted in step 4: the cut entry is shorter than `C_w`, and a charge of `w` goes unused that exceeds it, whatever was pending |
| 2 | drafting | "request", "pending" and "claim" undefined, and "pending" read at the hart readmits the witness; a request pending but not claimable | defined in condition 5, with `no-empty-claim`'s stale-notification sentence; a source trap's one claim takes a request |
| 3 | drafting | a claim loop's final empty claim ran "for" no request | moot: each trap now runs one service |
| 4 | drafting | a timer service inside a trap that runs several services had no cost, start or case in §2 | moot: a timer trap runs the timer's service alone, claiming nothing; §2's timer case stated |
| 5 | drafting | the releasing service of a source-released job, and which traps the window holds | a sentence for the releasing service; "every other interrupt trap that runs in the window starts in it" |
| 6 | drafting | the acknowledgement point, `exit` and the floors under the new start and claim loops | the start is the trap's entry again, one service per trap, so `exit` is the trap's and the floors stand as necessary; acknowledgement at entry unchanged, no under-charge, as the reviewer found |
| 7 | drafting | the cost clause for several services per trap stated in condition 5 alone, and owned by no record | removed with claim loops: condition 5 requires one service per trap, and the catalog reads `one-claim-per-trap` for it |
| 8 | drafting | "raised" in the catalog against "taken" in the variant; "due" undefined | the catalog says its "raised" means the variant's *taken*; *due* defined in condition 6 |
| 9 | drafting | the catalog paragraph's "all already preconditions", and a conjunct `no` read as the fact `no` | "each a fact the variant or its composition already reads"; anything but every conjunct `yes` reaches the variant undeclared, naming the conjunct |
| 10 | drafting | `/1` naming two texts, the changed definitions unlisted, and the rule credited to §15 | `/1` is the text before `1c61e70`; the definitions listed; the rule is the record's, §15 asking only that changes be explicit |
| 11 | nit | stale `/1` references, and the composition record's Review silent on its amendments | each fixed where it meant the model; the composition record's Review names this history |
| 12 | nit | "would be served", the refusal's "and releases", and "a service … is taken" | "could be"; "and its service releases"; the interrupt is taken |
| 13 | nit | the book's F17 paragraph and its "serves an event that came after the trap"; the `riscv-privileged` scope's "two sentences" | corrected |
| 14 | nit | no test pinned condition 6's refusal text; the field names no longer say what they hold | a test asserts the text; the fields keep their names, which the catalog's fact names match, and their doc comments carry the meaning |
| 15 | nit | "a conforming platform allows" while P2 is the emulator's | "a conforming controller, or the emulator's" |

**`M2.11`'s third review, `2026-10-02`,** of commit `cc2c141`, by a new read-only context. It confirmed the first
witness excluded by the trap-time clause and the step-4 inequalities, and found no schedule past the bound under the
intended readings; but step 3 had dropped "of a declared source", which let a never-enabled source's request, stuck
pending at the controller, satisfy the clause: the first witness with such a source gives 33 against 27. Its
verdict: "not fit to close M2.11 yet". Defects per round: 5, 1, 3.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | the trap-time clause lost "of a declared source", and a request stuck pending at the controller satisfied it; the stale-notification sentence contradicted the definition | "a request a declared source's arrival made is pending at the controller", in condition 5, §1, §2, the doc comment, `no-empty-claim` and the catalog; the stale-notification sentence now a consequence |
| 2 | defect | §2 charged a cut trap's entry to an arbitrary pending request, whose `C_s` need not cover code that reads the pending bits; no under-charge | the request a claim at the window's end would take: up to that instant the run is the one in which nothing arrives after it, where the trap is that request's service, so the cut part is a prefix within its `C_s` |
| 3 | defect | step 4's `s_0 ≥ t` case: the stretch step 2 counts need not have ended by `s_0`, which can precede `x`'s pending; no under-charge | split at `p_x`, the instant `x`'s interrupt is pending at the hart: before it, `s_0`'s service is paid by elapsed time or is the stretch; after it, it is the stretch or the stretch ended by `s_0` |
| 4 | drafting | "every later change is `/2`'s" against the rule, `/2` conclusions having been stated | `/2` open while `M2.11` is, its conclusions the tests' and §6's re-run with every change; a change after that is `/3` |
| 5 | drafting | three verdicts readable for `one-claim-per-trap` `no` with `external-before-timer` `yes` | the composition record's row restored; the catalog reads the fact for condition 5 and maps its `no` to the fact's `no`; the preconditions row says so |
| 6 | drafting | "a caller may still declare the variant's fact", and a conjunct `no` never mapping to `no` | the sentence deleted; `no` from the three conjuncts that contradict a clause, undeclared otherwise |
| 7 | drafting | *due* not limited to timer-released tasks, and "a service … releases every due one" | "a release of a timer-released task is due …"; "the timer service releases every due one" |
| 8 | drafting | a source with no claim could never meet condition 5, and a claim's request not said to be an arrival's | such a source refused, said so; "such a request", one a declared source's arrival made |
| 9 | drafting | two senses of pending, and of claim | "pending at the controller" in condition 5, the hart's elsewhere, said so; a read that finds none takes nothing |
| 10 | drafting | step 4's loose terms and `w`'s place ahead of `x` | `[t − J_w, t + Δ)` and "that trap's part before the claim"; `w` ahead of `x`, its claim read while `x` waited; the timer case deleted |
| 11 | nit | "each trap runs one service", exception traps running none | "each trap taken for an interrupt" in §1's row and the catalog |
| 12 | nit | the catalog's conjunct dropped the caller's declaration for a service's application code | added |
| 13 | nit | the timer fact's doc comment stricter than condition 6; the test comment's "is served"; §5 and F17 omitting a trap that runs several services | the clause removed; "could be"; both name it |
| 14 | nit | round 2's row 14 said the catalog's fact names match both fields; round 2's row 2 did not reach "pending but not claimable" | corrected here, the earlier rows left as they stand: `services_paid_by_arrivals` has no catalog fact; finding 1 above answers the second |
| 15 | nit | the ledger's `riscv-privileged` Hash field said two sentences | three |

## Why

The record states the variant as it stands, and this file keeps how it got there, as for the catalog record.

## How to apply

- A new round appends its paragraph and table here, and one row to the summary table in the record's `## Review`.
- A finding is answered in the record first, and its row here names the sections that answer it.
