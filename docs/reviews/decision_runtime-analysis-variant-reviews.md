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

## Why

The record states the variant as it stands, and this file keeps how it got there, as for the catalog record.

## How to apply

- A new round appends its paragraph and table here, and one row to the summary table in the record's `## Review`.
- A finding is answered in the record first, and its row here names the sections that answer it.
