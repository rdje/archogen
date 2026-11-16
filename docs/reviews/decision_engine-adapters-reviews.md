# Review history — `decision_engine-adapters.md`

The independent review of [`docs/specs/adapters/decision_engine-adapters.md`](../specs/adapters/decision_engine-adapters.md),
leaf `M3.2.1`'s design of the engine's adapters. Each round is a reader who had not seen the reasoning that wrote the
record, asked whether it is true to `ROADMAP.md`, the model, the substitutability relation's record, the earlier
decisions and the code, and complete for what `M3.2` and its hand-offs ask — never a hunt for code mutants.

## Round 1 — `2026-10-27`: not acceptable, 25 findings

The record then had presence yield for every fact a kind offers, a kind meet only a fact its offers name, one instance
per kind and provider, instances composing their provider, and `uc3` resolving with its description unchanged.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| A1-1 | defect | presence yielding for every name a kind offers would stop F04's case reporting `wrap-behavior`, which the extender offers and needs beneath alike | Presence yields only for what a kind **adds** — offered, not passed through from itself, not named beneath, no qualifier — and only where one declaration offers, by name, everything the kind's `beneath` names (§7); F04's timer offers no `relative-delay`, so it keeps `11` |
| A1-2 | defect | no kind's offers name `unambiguous-horizon`, so the epoch extension could never be bound | An instance meets a pair where the pair's fact has a value at it, offered or derived from its offers (§3.1) |
| A1-3 | defect | the extender offered no width, so a service needing `counter-width` beside the horizon split over two providers, rule 13's unknown | It offers `(counter-width 64 bit)` and passes `tick-rate`, `tick-unit` and `available-in-state` through (§2) |
| A1-4 | defect | `18446744073709551616` is not writable: the reader's integer is `i64` | `(counter-modulus (pow2 64))`, the relation record §2's spelling |
| A1-5 | defect | "composes" would drop the provider-to-instance edge rule 12 keeps, as `Joint::initialization` filters composed pairs | An instance owns no region and composes nothing; rule 12 keeps its edges (§4) |
| A1-6 | defect | one instance per kind and provider refuses two alarms over a timer of two channels, a declared impossibility | Exclusive kinds — those offering `channels` — have an instance per requester, `(adapter KIND PROVIDER REQUESTER)`, each with its own channel beneath; shared kinds one per provider (§1.4, §3.1, §5); a fixture for each case (§10) |
| A1-7 | defect | `uc3` resolving needs `timer.delay` to state a modulus, a change to its description | Said: it must gain `(counter-modulus 4294967296)`, `M3.4.7`'s migration, the director's before it is made; `M3.2` makes no change to it (§8) |
| A1-8 | gap | only the message of `SR-H18` was decided: a kind whose `beneath` lacks a described fact left `uc3` at `13` | The instances tried take part in the pair's code, a `beneath` answering with its side's answer, so describing a fact makes `missing-fact` (§3.4) |
| A1-9 | gap | `M3.3.6`'s time-base agreement across requesters unanswered | Model §8 rule 14, one time base, judging direct bindings too, measured by `.3` (§4) |
| A1-10 | gap | a binding to an instance would make a route with no interrupt, beside its provider's | A binding to an instance makes no route; its provider's carries its requesters (§4, rule 8) |
| A1-11 | gap | costs: the handler, the re-arms and the wake are interrupt load §7.3 asks recorded | A `costs` clause; each kind's interrupts named, reported per instance, carried by `M4.1` for the analysis (§1.3, §2, §9) |
| A1-12 | gap | "every face says so" named no face, version or discharging leaf | The faces, the API's minor, and `M4.1` and `M4.8` named, the obligations written into those leaves (§9) |
| A1-13 | gap | "obliged" was a sixth outcome model §7 rule 5 does not have; `(from g)` had no domain rule | An obliged bound is the requirement's value at an exclusive instance, an ordinary value; `(from g)` across domains is refused by the reader (§1.3, §1.5, §1.6) |
| A1-14 | gap | the catalog file's standing — its verdict, kind, conformance row, refusals, presence's reading — undecided | Decided as the vocabulary's: kind `defadapter.eadl`, exit `20` frozen, a conformance row, the baseline, the reader's refusals listed, presence's reading held to the reader's (§1.1, §1.2, §1.6) |
| A1-15 | gap | the interim divergence not written into `M3.4.7` | Written into `M3.4.7`'s leaf with the record (§7.5) |
| A1-16 | gap | a rescaler, which §5.2 and the relation record §6 name, unmentioned | Not in `/1`, with its reason: the vocabulary has no fact for a rescaled counter's resolution (§2) |
| A1-17 | gap | no positive fixture named for `M3.2`'s acceptance before `M3.7` | Two positive and two negative fixtures named, the positives' plans frozen and checked (§10) |
| A1-18 | gap | a mediating kind's ownership against "composes"; no kind states `available-in-state` | Mediating ownership left to the catalog that first ships one, the test catalog's owning no region (§6); both kinds pass `available-in-state` through (§2) |
| A1-19 | gap | `/2` named no catalog identity | `(adapters adapters/1)`, refused by the checker when its table is another's (§5) |
| A1-20 | wording | §7.4 cited rules 7 and 8 of the absence record; only rule 8 names the divergence | Rule 8 alone |
| A1-21 | wording | "(§7)" pointed at presence where F10's message is §3 | Cross-references corrected |
| A1-22 | wording | the ledger's scope rows claimed quotes the record does not make | `linux-timekeeping`'s row narrowed to the sentence quoted; `linux-highres`, no longer cited, removed |
| A1-23 | wording | "No multiplexer" rested on one reading of "selected" | Rests on F08's fixture and scope; rule 4's multiplexing adapter waits for a catalog with overhead evidence (§2) |
| A1-24 | wording | an instance "as a block" in rule 9; obligations "each binding"; rule 8 missing from the obligations | Rule 9 amended to make an instance a caller; obligations once per instance; rules 4, 8 and 10 named (§4) |
| A1-25 | wording | normative amendments assigned to no child; the book to `.6` alone | Each child amends the model, records and book its slice changes, in its commit (§10) |

## Round 2 — `2026-10-27`: not acceptable, 12 findings

Every round-1 answer settled its finding but A1-8's, with remainders of A1-3, A1-7, A1-13 and A1-14 below.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| A2-1 | defect | instances answering "undescribed" with a `beneath` their provider states none of would turn F10 into `missing-fact` and let a silent UART decide a code, against `SR-H18`'s "a silent one deciding nothing" | An instance whose `beneath` fails takes part only where its provider states some fact that side names (§3.4); F10's comparator states none, and F10 keeps `13` |
| A2-2 | defect | "describing a fact an adapter needs makes `missing-fact`" contradicted presence refusing `uc3` `13` before any search | A **partial path** — one block or platform offering part of a kind's `beneath`, the rest undescribed there — makes presence answer `missing-fact`, its hint naming what to describe (§7.2–§7.3); `uc3` moves `13` → `11`, check and search agreeing, measured and migrated by `.4` |
| A2-3 | gap | an obliged value undefined where a requester writes several bounds on one fact | The tightest bound the requester writes there, over all its requirements (§1.5) |
| A2-4 | gap | `SR-H10`'s last step and its director condition written into no leaf | Written into `M3.4.7`'s leaf, with the condition (§8) |
| A2-5 | gap | the qualifier exclusion needs the vocabulary, which `eadl_model` does not embed | The reader admits a qualifier only passed through from itself, so "adds" needs no vocabulary (§1.6, §7.1, §7.4) |
| A2-6 | gap | a path at a service or a stray block fails by place, which the interim divergence did not name | A path is a block's or a platform's; one outside the system's hardware is named in §7.5 and `M3.4.7`'s obligation |
| A2-7 | gap | the reader's refusals omitted the offer reader's own and an exclusive kind's channel shape | Both added (§1.6) |
| A2-8 | wording | F04's reason was "no path"; the operative one is that no kind adds `wrap-behavior` | Corrected (§7) |
| A2-9 | wording | "passes the counter's other facts through" overclaimed | The passed facts named; any other undescribed at the instance (§2) |
| A2-10 | wording | the composition quote attributed to model rule 13 is `M3.3.6`'s note in `M3.2`'s leaf | Attributed (§4) |
| A2-11 | wording | "two negative" fixtures, one of which resolves; `M4.1` still names `/1` | The fixtures described as they are; `.5` moves `M4.1`'s leaf to `/2` (§10) |
| A2-12 | wording | rule 14 refuses more than the note asks | Said deliberate, with its reason and the spelling a description uses instead (§4) |

## Round 3 — `2026-10-27`: acceptable, 4 findings, no defect — the review closed

Every round-2 answer settles its finding; A2-3 and A2-9 left the remainders below. Worked through: F10 keeps `13`,
`uc3` as written is `11` at check and search alike, F04 keeps `11`, F06 `10`, and no other frozen verdict moves — `.4`
moves `crates/eadl-model/tests/semantic_corpus.rs`'s expectation for `uc3` with the rest.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| A3-1 | gap | an `exactly` bound on an obliged fact was neither least nor greatest, so offered bare or at another bound that fails it | An `exactly` bound fixes the value; else the least `at-most`, or the greatest `at-least` (§1.5) |
| A3-2 | wording | rules 2 and 3's partial path, read literally, covered an offered fact — F09, frozen at `0`, would read `11` | Scoped to a fact rule 2 or 3 would refuse; a fact some declaration offers is untouched; §7.1's two conditions said plainly |
| A3-3 | wording | `available-in-state` called a fact of the function `counter`; it is a qualifier | Named as the qualifier it is (§2) |
| A3-4 | wording | the partial path's hint promised a path where a stated value may already refuse | The hint promises a path by name, which the search may refuse by value (§7.3) |
