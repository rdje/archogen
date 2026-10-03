# The substitutability relation: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active`
- **Owner / source:** leaf `M3.1.1` (`docs/tasks/M3.md`). This is the review history of
  [[decision_substitutability-relation]], kept apart from it as `docs/reviews/INDEX.md` describes.

## The fact / decision

`M3.1.1`'s acceptance is a review, by a context that did not write the record, finding no requirement the relation
satisfies that it should not — a stronger precondition read as a capability, a value read in the wrong direction, an
implication assumed that nothing declares — and no case of `ROADMAP.md` §5.2 misclassified. Each round below is one
such review, with every finding and the answer the record gives it. The section numbers are the record's as answered.

Each round is a new context, read-only, which had not written the record. It is given:
- the record;
- `ROADMAP.md` §4.2, §4.3, §5.2, §5.3, §5.4, §5.5 and §5.6;
- `docs/semantics/model.md` and `docs/semantics/reference.md` §7;
- `crates/eadl-model/src/quantity.rs`, `presence.rs` and `refinement.rs`;
- the corpus: `examples/`, `docs/semantics/boundary/accept/`, `targets/riscv-virt-up.eadl`.

It is barred from other implementation, and may check external specifications on the web.

## Rounds

**Round 1**, `2026-10-03`: the reviewer re-derived the three worked horizons exactly (`33554432/78125 s`,
`512/78125 s`, `67108864/15625 s`), found every §5.2 case classified in the right direction, found the diagnostic
codes §5.5's and model.md §6's, confirmed the record's statements about `presence.rs` and `refinement.rs`, and
re-counted the census to 37 by adding three `needs`-only names. There were 15 findings, 7 of them defects, and the
verdict was "not acceptable as it stands". Every answer edits the words the finding names; one finding (A3) was
answered by refusing a case rather than charging it — mediation is an adapter's, `M3.2`'s — and one (A4) by giving
every requirement one tightening direction, `exactly`, instead of changing a fact's direction and exempting the
refinement corpus from the rule.

| Finding | Defect | Answer |
| --- | --- | --- |
| A1 — derivation chaining: "offers every fact it is derived from" leaves the horizon undefined where only the width is offered, un-satisfying the record's own worked case | yes | §3 rule 1 and §4: a derived fact has a value when every fact it is derived from *has a value* at `P`, offered or itself derived; `derived-from` is acyclic and the registry refuses a cycle (§1.1) |
| A2 — which closure: §3 made the fact's `needs`-membership decisive, so a constraint on a fact nobody `needs` and no provider values was "nothing" and rule 5 passed the clause; §9 needed the declaration's membership and under-counted what wiring would break | yes | one rule in §3 rule 1, restated in §5, §8 and §9: a constraint written by a declaration inside the closure puts its fact inside it, so an unvalued fact is `missing-fact`; §9 now says every example turns `missing-fact`, since policy and service-level facts are the realization's, supplied by a catalog record's contract |
| A3 — mediation: a disjunction over three facts nothing declares; `or-through-mediation` declared boolean where the corpus writes `allowed`; `reachable-through-mediation` in no file | yes | a third role, `statement` (§1.1, §3 rule 6): the requirer's word about itself, constraining no provider, read by the rule or leaf its entry names; `or-through-mediation` is `enumeration allowed forbidden`, read by `M3.2`'s adapter search alone; in `/1` a failed privilege test is not satisfied; `reachable-through-mediation` deleted |
| A4 — `counter-width` at-least reads more bits as better where the width is the interface | yes | §1.1: every requirement may write `exactly`, the one bound every direction admits, which refuses what the fact's direction would accept and never the reverse; §6 "more bits" worked with `(exactly 32 bit)`; an abstract platform's `(at-least 32 bit)` stays the same bound under the same rule, so `positive-valid-refinement.eadl` is unchanged |
| A5 — the width limit 127 is outside `i128`; an overflowed comparison has no stated verdict | yes | §4: 126, the largest power of two `i128` holds; §2, §4 and §8: `QuantityError::Overflow` is `unsupported-profile`, a named limit, never a verdict on the requirement |
| A6 — "one fact, two values" refuses `targets/riscv-virt-up.eadl`'s three regions | yes | §5: the rule covers facts the vocabulary declares; an undeclared fact has no domain to contradict, and `region` is named in §10 |
| A7 — `1 h` is not a unit the language knows | yes | §6: `3600 s` |
| A8 — the safeguard is the direction, not the role | no | §3 rule 4 and Why: what forbids it is where the vocabulary puts the operands — the demand read as the offered value against the requirer's statement — and rule 5's checking every imposed condition; a condition's direction is `at-most` or `exact` in `/1`, an `at-least` entry added with its own worked case |
| A9 — "no join or meet of offers" over-claims in the negative | no | §7: none is claimed or used; some per-fact preorders have them, and the relation takes none |
| A10 — `ordering` and `region` fit no domain | no | §10 names both as `M3.3`'s, undeclared in `/1` |
| A11 — the written spelling of `exact` is `exactly` | no | §1.1 and §2 say so |
| A12 — the census lists re-derive 34 of 37 | no | §1 adds the three `needs`-only names |
| A13 — the absence is `timer.delay`'s, not `soc.delay-only`'s | no | §6 corrected |
| A14 — the horizon rule assumes a modular counter | no | §4 says so, and that a saturating counter derives no horizon |
| A15 — `deffact`'s `doc` holds a string and `domain` nests; fact names against reference §7 rule 6 | no | §1.1: the clauses' `holds`, and fact names live in the vocabulary, loaded by the registry and never checked as a description, so a block named like a fact collides with nothing |

**Round 2**, `2026-10-03`, a new context that had not read round 1: it read all 124 tracked descriptions, `rational.rs`
and `kind.rs`'s `check_values` beside the inputs above, recomputed the horizons and the `i128` limit, and found every
§5.2 direction right, `exactly` a subset of every direction (conditions, sets, intervals and group heads included),
the `statement` role unable to pass anything, and the closure rule sound against §5.3 and `presence.rs`. There were
10 findings, 7 of them defects, and the verdict was "not acceptable as it stands". Two defects (B2, B4) were round-1
answers that had not reached every restatement of their rule — the habit the knowledge card on review answers names
fourth — and one (B1) was an acceptance the record would have made: a reload counter forced to omit its modulus,
whose width then derived a horizon it has not.

| Finding | Defect | Answer |
| --- | --- | --- |
| B1 — the agreement rule refuses a 32-bit register reloaded at `1000000` as `invalid-description`, and dropping the modulus derives `2^32` and a `429 s` horizon for a counter that wraps in `0.1 s`; a saturating counter that offers its width derives both | yes | §4: an offered value is the value and wins; a rule is a default for what the offer leaves unsaid; only a modulus above `2^width` is `invalid-description`; `wrap-behavior` (`modular`, `saturating`) enters both rules, and a saturating counter derives nothing from its width; the reload counter worked, `0.1 s`, failing the requirement |
| B2 — §10 and rule 4 still say "the mediation boolean" and "`at-most` or `exact`" after A3 made mediation a statement | yes | §3 rule 4 and §10: `/1`'s one condition is `reachable-at-privilege`, direction `at-most` |
| B3 — `derived-from` declared `(holds values symbol)`, one value wide, refuses the record's own example on load (`schema-arity`); "symbols and nothing else" false of `doc` and `domain` | yes | §1.1: `derived-from` `(holds forms)`; the intro says names, lists of names and one line of text |
| B4 — `exactly` granted in §1.1, refused by §3 rule 2 and §8; an offer's bound `(exactly 10 MHz)`, which `refinement.rs` reads, refused by §1.1 | yes | §1.1 grants it to a requirement and to a bound an offer writes; rule 2 and §8 say "in `f`'s direction or as `exactly`" |
| B5 — two providers' differing offers called a contradiction while differing records are listed; `app.two-timers`'s intent refused | yes | §5: an offer is one provider's statement, judged per provider, each satisfying provider listed, each value named where none satisfies; one provider offering one fact twice is the contradiction; §4: derivation per provider |
| B6 — `(absolute-deadline true)` flat, as `alternative-timer` writes it, outside the group domain | yes | §2: a group's head may be required alone, and its sub-facts constrained flat beside it |
| B7 — a written count is the language's `i64`, not `i128` | yes | §2: a written number is the language's 64-bit integer, refused above it as read; `i128` is the arithmetic's |
| B8 — 26 of 37 census facts have no stated direction; `core-count` an `at-least` trap | no | §1.1: the entries of `/1`, every census fact with domain, role, direction and reason; `core-count` `exact`; `M3.1.2` adds none without a row |
| B9 — presence's declared-name exemption passes what rule 1 calls `missing-fact` | no | §9: the relation judges by a provider's offer; the library's verdict is the stricter, which the report carries decided at `M3.4` |
| B10 — slips: "(rule 4)", "the word is not used", "every example", the enumeration row's `at-most`, the payload's profile, an offer of a statement | no | each fixed in place; an offer of a statement fact is `invalid-description` (§3 rule 6, §8) |

**Round 3**, `2026-10-03`, a new context that had not read rounds 1 or 2: it read all 124 descriptions and `kind.rs`
whole, re-derived the census name by name, recomputed the horizons and the `i128` limits (`i64::MAX × 10^9` fits
`in_base`; `Rational::cmp` is 256-bit), and found every §5.2 case right in fact, domain and direction, `exactly` a
subset of every direction it is written against, the closure rule a widening of `FactMap::closure` as §5.3 requires,
and "offered value wins" working on the reload counter. There were 13 findings, 8 of them defects, and the verdict
was "not acceptable as it stands". Three defects came from rules added by earlier answers (`wrap-behavior`, the
vocabulary rows, `exactly`), which the knowledge card on review answers predicts; two were restatements missed (C3,
§10's old sentence; §8's dropped *declared*); two were real gaps in the relation's order and scope (C1, C2). Where
two special cases had grown, one general rule replaces them: any input a rule reads, offered without a value, is
unknown, and the rule derives nothing.

| Finding | Defect | Answer |
| --- | --- | --- |
| C1 — rule 1 derived before it looked for a declared absence, so `(absent unambiguous-horizon)` beside an offered width and rate was satisfied at `429 s` | yes | §3 rule 1: absence is checked before derivation, infeasible by model §2 rule 2 — no derivation corrects what an offer says; an input declared absent leaves the derived fact underivable and a constraint on it infeasible (§8) |
| C2 — §5 enumerated only clauses that "write constraints", so a provider's condition was never checked against a requirer that only `needs`: a machine-only UART accepted silently | yes | §5: every declaration in the closure with a `requires` clause, `needs` judged as bare presence, rule 5's condition check applied to each provider that satisfies — `missing-fact` on the service's side when it states no privilege |
| C3 — §10 still called two providers' values a contradiction; §8 dropped *declared* | yes | §10's sentence replaced by the scoping remark; §8 says *declared* |
| C4 — `priorities` under `includes` read `unique`, a demand on the task set, as a capability | yes | §1.1: `exact`, like `deadlines`; a policy that wants both writes both |
| C5 — `wrap-behavior` offered bare fell between "modular or not offered" and "saturating"; read by both rules but in neither `derived-from` | yes | §4: any input a rule reads, required or optional, offered without a value is unknown and the rule derives nothing; `derived-from` lists required inputs, a rule may read a named optional one; both table rows say "or offered without a value" |
| C6 — a modulus beside `(wrap-behavior saturating)` unrefused, and satisfying a modulus requirement | yes | §4 and §8: `invalid-description`, a modulus being what a counter wraps at |
| C7 — `exactly` granted for every direction with no satisfaction condition for intervals; §1's offer definition lacked "or `exactly`" | yes | §2 interval row: `exact`, the intervals equal; §1: a bound in the fact's direction or `exactly` |
| C8 — rule 5's clause verdict had no home in §5, so `M3.4` would receive per-constraint lists only | yes | §5: the enumeration emits, per clause and provider, rule 5's verdict; `M3.4` chooses among providers that satisfy the clause |
| C9 — `quantity.rs`'s `Exact` comment names a counter modulus | no | §9: `M3.1.2` aligns the comment with §1.1's `at-least` |
| C10 — a fixed-frequency offer had no spelling for an interval fact | no | §2: a point `(f v)` is the interval `[v, v]` |
| C11 — `deffact`'s cardinalities unstated | no | §1.1: `doc`, `domain`, `role`, `direction` once; `derived-from`, `rule` at most once |
| C12 — a non-positive width, and `2^width` past 126 against an offered modulus | no | §4: a width is a positive whole number of bits; a written modulus is at most `2^63 − 1`, so against 64 bits or more the bound holds uncomputed |
| C13 — a service may `offers` but is no provider | no | §1: its offers are not judged in `/1` |

**Round 4**, `2026-10-03`, a new context that had not read rounds 1 to 3: it was asked to check eight restated rules
statement by statement and to walk `periodic-three`, `alternative-timer`, `bounded-queue` and `app.two-timers`
through the relation by hand. It found every verdict of the walk-through right, six of the eight restatements one
thing everywhere, every §5.2 case right through the derived path, the `deffact` example admitted by `check_values`,
and the numbers exact. There were 10 findings, 3 of them defects, and the verdict was "not acceptable as it stands".
Defects per round: 7, 7, 8, 3.

| Finding | Defect | Answer |
| --- | --- | --- |
| D1 — an offered `unambiguous-horizon` beside derivable inputs was never checked against them, so a 16-bit counter at `10 MHz` writing `(unambiguous-horizon 3600 s)` passed a 60 s requirement the record's own prose said it caught | yes | §4: one rule in place of the modulus special case — an offered derived fact beside the values its rule would compute from may not lie beyond the rule's result in the fact's direction (a modulus above `2^width`, a horizon above `modulus / rate`), `invalid-description`; below it the offer is the value; worked in §4 and §6, and in §8 |
| D2 — §5 enumerated providers "that have a value" and called a fact no provider offers or derives `missing-fact`, where rule 1 makes a provider's declared absence infeasible (`alternative-timer`'s `absolute-deadline`) | yes | §5: providers that have a value or declare the fact absent; `missing-fact` only where no provider offers, derives or declares absent |
| D3 — §1 made bare presence `true` for a boolean fact alone; §2's group row made a bare head `true` | yes | §1: for a boolean fact, or a group's head |
| N1 — `frequency`'s role undecided between a capability and an input clock | no | §1.1: the rates the block can run at, a capability; the clock a block must be fed is a condition and another fact |
| N2 — `needs time.monotonic` read alone as `missing-fact` | no | §5: a `needs` entry naming a vocabulary fact; one naming a declaration is presence's |
| N3 — rule 1's "the configuration is infeasible" from one provider's absence | no | §3 rule 1, §6, §8: not satisfied by `P`, infeasible where no other provider satisfies |
| N4 — the `modular` assumption against a constraint on `wrap-behavior` | no | §4: the assumption serves the rules alone; a constraint on the fact is judged on what is offered |
| N5 — `decision_catalog-records.md` cited as a sibling | no | its path, `docs/specs/catalog/`, written |
| N6 — rule 4's example named `available-in-state` by description | no | the clock a block must be fed |
| N7 — a decimal magnitude is digits and a scale; the count row's unit-less requirement | no | §2 |

**Round 5**, `2026-10-03`, a new context that had not read rounds 1 to 4, asked to write, for each §5.2 case, an
offer that would make the relation accept what the hardware cannot meet: six were refused by the sentence it named,
two were not. There were 10 findings, 3 of them defects, and the verdict was "not acceptable as it stands". The
first defect reopened the fourth round's closed case by one symbol — the impossibility bound was attached to a
derivation's *branch*, so a `wrap-behavior` that was `saturating`, or offered bare, switched it off — and the answer
makes the bounds properties of the facts, whatever `wrap-behavior` says or omits, and refuses a modulus or a horizon
beside anything but a modular counter: one rule, no branches. Defects per round: 7, 7, 8, 3, 3.

| Finding | Defect | Answer |
| --- | --- | --- |
| E1 — the bound "may not lie beyond the rule's result" vanished where the rule had no result: `(wrap-behavior saturating)` or bare `wrap-behavior` beside a 16-bit, 10 MHz counter let `(unambiguous-horizon 3600 s)` satisfy 60 s, and a modulus of `2^32` on a 16-bit register stood | yes | §4: a `counter-modulus` beside a width may not exceed `2^width`, a horizon beside a rate and a modulus or width may not exceed `modulus / rate`, whatever `wrap-behavior` says or omits; both facts are a modular counter's, and either offered beside a `saturating` or bare `wrap-behavior` is `invalid-description`; worked in §4, §6 and §8 |
| E2 — a platform's or block's `requires` constraint was in the closure by rule 1 and judged by nobody under §1 and §5; the corpus's one `ordering` constraint never reached §10's `unsupported-profile` | yes | §1 and §5: every declaration in the closure with a `requires` clause — a platform or a block included — is a side and is enumerated |
| E3 — a bare non-boolean offer had three verdicts: `missing-fact` (§2), derived and satisfied (rule 1, §4), none (§5) | yes | one reading: a bare offer is unknown at that provider and stops rule 1's chain before derivation — `missing-fact` whatever a rule could have derived; §2, rule 1, §4 and §5 say so |
| E4 — `counter-modulus` `at-least` against `quantity.rs`'s own example of `Exact`, with no requirer in the corpus to argue from | no | taken: `exact` — the modulus is the interface fact §3.1 names, wrapping arithmetic depends on it, and the horizon carries "wraps later"; the crate's comment stands and §9 says so |
| E5 — a condition fact offered as a bound or bare, or declared absent, had no stated reading | no | §3 rule 4: a demand is a value, a bound or bare demand is `missing-fact` for every requirer; a declared absence imposes no demand, the one place rule 1's absence arm does not apply |
| E6 — §2's example columns narrower than §1.1's `exactly` | no | §2: `(f (exactly b))` and "a bound in the fact's direction or `exactly`" |
| E7 — "other than `boolean`" literally covered a group's head | no | §2: "or a group's head" |
| E8 — a non-whole width is ill-typed, not a limit; a comparison itself cannot overflow, its unit conversion can | no | §2 and §4 say so |
| E9 — the optional input declared in the engine and the record, not in `vocabulary.eadl` | no | §1.1: a `reads` clause, at most once, holding the optional inputs; the example entry carries `(reads wrap-behavior)` |
| E10 — one provider offering one fact twice with the same value | no | §5: one offer |

**Round 6**, `2026-10-03`, a new context that had not read rounds 1 to 5, given the adversarial brief and the ten
restated rules: it refused every adversarial offer against a *valued* sibling fact by a sentence it named, found the
§5.2 cases right in fact, domain, role and direction, and walked the corpus again. There were 9 findings, 3 of them
defects, and the verdict was "not acceptable as it stands". F1 was the third round in turn to find an author's
horizon beside its own grounds slipping past a bound — this time because a bounding input offered *bare* made the
bound vacuous. Defects per round: 7, 7, 8, 3, 3, 3 — not falling on that seam, which the knowledge card on review
answers reads as a class to refuse rather than charge. The answer shrinks the design: `modulus-from-width` is
deleted — §3.1 asks for an explicit counter modulus, and a register's width does not decide where it wraps — leaving
one rule, and a derived fact is **derived or stands alone**: offered beside any fact its rule reads, bare or valued,
it is `invalid-description`; offered alone, it is a claim like any other offer. No bounds, no branches, nothing for
a sibling's spelling to switch off.

| Finding | Defect | Answer |
| --- | --- | --- |
| F1 — §4's bound bound on the inputs' *values*, so `(counter-width 16 bit) tick-rate (unambiguous-horizon 3600 s)` — the rate bare — made it vacuous and the 60 s requirement was satisfied; a bare width beside a modulus the same; bare beside valued at one provider unstated | yes | §4 rewritten: a derived fact is derived or stands alone — offered beside any fact its rule reads, bare or valued, `invalid-description`; `modulus-from-width` deleted, the modulus offered never derived, with the one consistency check that a valued modulus beside a valued width may not exceed `2^width`; §5: bare beside valued at one provider is the two-values rule's `invalid-description`; §1.1, §6, §8 and the Why follow |
| F2 — an absent condition fact: rule 4 satisfied, §5 and §8 infeasible | yes | §5 and §8 carve out a condition fact declared absent — no demand, by rule 4 |
| F3 — a presence requirement against a bare or bound offer: §2 satisfied, rule 1 `missing-fact` | yes | rule 3: a presence requirement is satisfied by any offer of the fact, bare, valued or a bound, and by nothing else; rule 1's bare arm says "a presence requirement apart" |
| F4 — a statement entry said to name "the leaf" with no clause that can | no | §1.1: the rule, or the leaf, named here for it |
| F5 — "other than `boolean`, or a group's head" parsed as making a bare head unknown | no | §2: "other than `boolean` or a group's head" |
| F6 — `exactly`'s spelling absent from the quantity, interval and set rows | no | §2: each row writes it |
| F7 — a condition fact written in a block's or platform's `requires` read as a self-statement, dropping the demand | no | §3 rule 4 and §8: `invalid-description` — a provider states its demands in `offers` |
| F8 — precedence between `infeasible-configuration` and `missing-fact` across providers unstated; §9 said `missing-fact` alone | no | §8: both codes carried, the verdict the higher in `Verdict::precedence`; §9 names both |
| F9 — a clause whose constraints span two providers is satisfied by no single provider | no | §10 says so, and that `M3.4` is what may take two |

**Round 7**, `2026-10-03`, a new context that had not read rounds 1 to 6, briefed against the shrunk §4: it refused
every adversarial offer against a valued sibling, found each §5.2 case right, no condition read as a capability and no
undeclared implication used, and read the target's spike source to find `mtime`'s width. There were 12 findings, 5 of
them defects — two of them consequences of round 6's shrink, which the reviewer found where the record's own sentence
about the corpus's counters was false. Defects per round: 7, 7, 8, 3, 3, 3, 5.

| Finding | Defect | Answer |
| --- | --- | --- |
| G1 — the target's `mtime` is 64 bits wide; its modulus `2^64` is `read-number-overflow` as a literal, and with `tick-rate` offered a horizon may not be written beside it, so the one physical counter could state neither in `/1` | yes | §2: a count may be written `(pow2 N)`, `N ≤ 126`, where the 64-bit literal cannot hold a power of two; §4: `target.timer` states `(counter-modulus (pow2 64))` when `M3.4` wires the relation, its horizon `1 844 674 407 370.9551616 s` fitting the arithmetic |
| G2 — `(observation-coherent false)` satisfied `needs observation-coherent` while `absent` failed it | yes | §2 boolean row and rule 3: a boolean named in `needs` is the requirement `(f true)`, the mirror of the bare offer; `false` satisfies no presence requirement |
| G3 — an optional input declared absent: derivable by the rule row's "not offered", underivable by rule 1 | yes | rule 1 names `derived-from`'s inputs; §4 and the row: an optional input declared absent derives nothing, the conservative branch; the row says "undescribed" where it said "not offered" |
| G4 — §8's infeasible row, "a constraint no provider satisfies", swallowed `missing-fact` under the precedence tie-break | yes | §8: a constraint that has a value at some provider and that no provider's value satisfies |
| G5 — `core.eadl` admits `offers` on a service, not a policy | yes | §1 says so |
| G6 — a horizon beside a width alone accepted | no | §4 states the probe's outcome and why: a width does not decide a horizon |
| G7 — "read the other way round" invites a search for an operand swap that is not there | no | §1 and rule 4: the demand on the offered side |
| G8 — `2^width` needs whole bits and a short-circuit | no | §4: whole bits; 127 bits or more holds every writable modulus |
| G9 — "precondition" in two senses across rules 1 and 4 | no | rule 1: a provider's demand, read from its offers, §5.3's "provider preconditions" |
| G10 — a `needs` on a derived fact met only by a stand-alone offer | no | rule 3: or by its derivation |
| G11 — a `needs` outside `requires` not among "every requirement the clause writes" | no | §5: presence's, with the same verdict |
| G12 — `(f (exactly true))` absent from the boolean row | no | §2 row |

**Round 8**, `2026-10-03`, a new context that had not read rounds 1 to 7: it probed `(pow2 N)` at 63, 64, 126, 127,
negative and non-integer, every `wrap-behavior` state beside a modulus, the width check at 127 bits, a boolean
`false` against both forms, and a fact absent at one provider and offered at another; no probe made the relation read
a demand as a capability, a value the wrong way or an undeclared implication, and every §5.2 case was classified as
the fact's meaning requires. There were 11 findings, 1 of them a defect, and the verdict was "not acceptable as it
stands". Defects per round: 7, 7, 8, 3, 3, 3, 5, 1.

| Finding | Defect | Answer |
| --- | --- | --- |
| H1 — the absence rule, §5 and §8's worked example judged per provider a fact offered by one block and declared absent by another, which model §2 rule 1 refuses description-wide (`invalid-contradictory-presence.eadl`) before the relation runs, while §9 pledges the corpus's verdicts stand | yes | §3 rule 1: model §2 rule 1 stays description-wide, so within a description another provider satisfies past an absence only by deriving the fact from its own facts, and after `M3.4` as a catalog record; §8's example is an absence beside a provider that could derive but offers an input bare |
| H2 — `(pow2 N)`'s lower bound, integrality and the case where a literal would do unstated, beside two sentences refusing arithmetic | no | §2 and §10: `N` an integer from 0 to 126, anything else `invalid-description`, a literal's spelling and not arithmetic |
| H3 — a width not a whole number of bits had no verdict | no | §4: `invalid-description` |
| H4 — §8's `missing-fact` row omitted a provider's unknown demand | no | §8 names it |
| H5 — "a bound satisfies no constraint" against rule 3's presence | no | §2: no valued constraint; presence it satisfies |
| H6 — rule 3's clauses over-reached and self-corrected | no | rule 3: any offer other than a boolean `false`, or the derivation; otherwise rule 1's three arms |
| H7 — a `deffact` written in a description neither admitted nor refused | no | §1.1: `invalid-description` outside the vocabulary module |
| H8 — the target file carries nothing its agreement does not check | no | §4: the agreement tool and its decision change in the same commit |
| H9 — the privilege order declares nesting that one hardware case breaks | no | §1.1: the order's meaning stated, `exactly` for a target where it fails |
| H10 — a failed value beside an unknown sibling takes the checker's precedence | no | a judgement, as the reviewer says; unchanged |
| H11 — a saturating provider may not write a horizon beside its wrap behaviour | no | deliberate; unchanged |

**Round 9**, `2026-10-03`, a new context that had not read rounds 1 to 8: every `wrap-behavior` state, `(pow2 N)` at
0, 62, 63, 126 and 127, a boolean `false` and a group head `false` against `needs`, absence against offer and
against derivation, a `deffact` in a description; every §5.2 case but access privilege classified as the fact's
meaning requires, no undeclared implication, the arithmetic exact. There were 13 findings, 2 of them defects, both
in the one condition fact — and the first a regression of round 8's own answer, the knowledge card's first habit.
Both answers delete a special case. Defects per round: 7, 7, 8, 3, 3, 3, 5, 1, 2.

| Finding | Defect | Answer |
| --- | --- | --- |
| I1 — round 8's escape for a target whose privilege order does not nest — write `exactly` — is a condition fact offered as a bound, which rule 4 reads as an unknown demand; so a user mapping without `sstatus.SUM` offering `user` satisfied a supervisor requirer, with no working form to say otherwise | yes | §1.1: the fact's value is the lowest level from which the function is reachable, every higher level reaching it too; a function whose reachability is not upward-closed has no value, offers none, and its requirers get `missing-fact`; the `exactly` escape deleted; §6 says so |
| I2 — a condition fact declared absent read as no demand, the one permissive absence, against model §2's meaning of `absent` and the exclusion obligation an abstract platform's absence would then put on a refinement | yes | rule 4: rule 1's absence arm applies to a condition fact as to any — reachable on no terms, no requirer satisfied; a provider with no gate writes `user`; the carve-outs in §5 and §8 deleted |
| I3 — a service named as a provider offering a derived fact | no | §4: an epoch-extending adapter, as a catalog record |
| I4 — "anywhere" omitted the refinement pair | no | rule 1: outside a refinement pair, which model §3 rule 3 judges |
| I5 — the rule row's `saturating` branch unreachable | no | the row says so |
| I6 — a group head `false` against `needs` | no | rule 3 and the group row: a group's head named in `needs` is `(f true)` |
| I7 — a boolean offered as an `exactly` bound | no | §2 boolean row: never a bound |
| I8 — a derived fact as an abstract bound cannot be refined by a counter stating its grounds | no | §10: a later leaf teaches refinement a derivation |
| I9 — what the target's `(pow2 64)` is checked against | no | §4: the privileged specification's `mtime` width, ledgered, decided under the target decision's §2 |
| I10 — a saturating counter states no horizon | no | §10 says so |
| I11 — `ordering` constrained in a tracked accept case | no | §10: the classifier runs there, not the enumeration |
| I12 — acyclicity attributed to the kind registry | no | §1.1: the typed table in `eadl-resolve` |
| I13 — a modulus of 0 | no | §1.1 and §8: `invalid-description` |

**Round 10**, `2026-10-03`, a new context that had not read rounds 1 to 9, which ran its probes against the built
`archogen` binary as well as by hand and fetched the privileged specification's `mtime` paragraph. There were 13
findings, 8 of them defects, and the verdict was "not acceptable as it stands". Defects per round: 7, 7, 8, 3, 3, 3,
5, 1, 2, 8. They cluster where the record had grown into other leaves' domains — the condition role, the
enumeration's scope over a description, and bounds written in offers — which the knowledge card on review answers
reads, in its fifth habit, as the signal to shrink and delegate. The condition role, which had drawn defects in six
of ten rounds, is deleted: what a provider accepts of its caller is offered as a set and judged by inclusion, and
the caller's bound context is `M3.3`'s privilege constraint. An offered `exactly` is the value, and no other offered
bound is read. The closure is rooted at the system, every `needs` judged wherever written, and which closure the
report carries is `M3.4`'s.

| Finding | Defect | Answer |
| --- | --- | --- |
| J1 — a provider stating no privilege imposed nothing, so silence was more permissive than `user`; `target.timer`, machine-mode `mtime`, accepted for any silent requirer | yes | the condition role deleted (§3 rule 4): `reachable-at-privilege` is a guarantee, the set of levels the function is reachable from, judged by inclusion; a provider stating none is `missing-fact` for a requirer that states one; a requirer stating none is `M3.3`'s, added to its acceptance |
| J2 — a declaration-level `needs` escaped rule 3 and the condition check; `(low-power-timer false)` met it, measured | yes | §3 rule 3 and §5: a `needs` of a vocabulary fact is judged wherever written; with no condition role, nothing else attaches to its placement |
| J3 — the system never inside presence's closure, its own constraints and its tasks' `needs` judged by nobody, measured | yes | §3 rule 1 and §5: the closure's root is the system, with its tasks; which closure the report carries is `M3.4`'s, added to its goal |
| J4 — `(f (exactly false))` offered met `needs f`; a group head's bound unaddressed | yes | §1 and §1.1: an offered `exactly` is the value; §2: any other bound on a boolean or group head is `invalid-description`; rule 3 names `(f (exactly false))` |
| J5 — a provider offering and declaring absent one fact, in a refinement pair, read by its value; the carve-out cited the wrong rule | yes | rule 1: such a provider is `invalid-description` before anything else; the carve-out names model §3 rule 1 and rule 3 for their two directions |
| J6 — a block requirer satisfied by a privileged provider refused with no legal repair | yes | gone with the condition role: a block that states no level asks nothing of the relation |
| J7 — a required input declared absent: infeasible by rule 1 and §8, `missing-fact` by §5 | yes | §5: a provider declaring absent a required input of its derivation is enumerated; §8's infeasible row says "required input" |
| J8 — an `exactly` width escaped the width–modulus check; a bound beside a value read by the value | yes | §1.1: an offered `exactly` is the value, under every check; a bound beside a value at one provider is `invalid-description` |
| J9 — `checked_div` multiplies before reducing, so a fitting quotient can overflow | no | §2: the computation's overflow, which only refuses |
| J10 — infeasible reported while a provider is still unknown | no | §8: the diagnostic names the unknown provider |
| J11 — §5 and §10 on whether `M3.4` takes two providers | no | `M3.4`'s to reconcile, unchanged |
| J12 — acyclicity over `reads`; who refuses a `deffact` outside the vocabulary | no | §1.1: the graph of `derived-from` and `reads`; `eadl-resolve`'s loader reads the vocabulary module alone |
| J13 — the `mtime` sentences not yet in the ledger | no | §4: the commit adding the modulus quotes both in the `riscv-privileged` entry |

**Round 11**, `2026-10-03`, a new context that had not read rounds 1 to 10, which ran seventeen probes against the
built `archogen` binary. For one provider and a requirer that states its level or state, it found every §5.2 case
read correctly and no stronger precondition passing; the derivation seam closed; the arithmetic exact. There were 16
findings, 7 of them defects, and the verdict was "not acceptable as it stands". Defects per round: 7, 7, 8, 3, 3, 3,
5, 1, 2, 8, 7. Six of the seven concern what a description makes of several providers — the closure, an absence at
one provider beside an offer at another, a clause split across providers, the description's code — which is the
search that wires the relation into a description, `M3.4`, and presence's rules beneath it. The relation is narrowed
to one provider: five outcomes, and the aggregation, with each finding, written into `M3.4`'s acceptance.

| Finding | Defect | Answer |
| --- | --- | --- |
| K1 — the closure stated as presence's widened in §5 and as rooted at the system in §3; under the first, a provider on a platform the system does not use satisfies (p02) | yes | §3 rule 1 and §5: the closure is no longer the relation's; `M3.4`'s acceptance carries one closure, rooted at the system with its tasks and platform, `uses` and `needs` followed transitively, a provider on an unused platform never satisfying |
| K2 — the description-wide absence rule rests on `presence.rs`, which compares only the first offer and first absence, and exempts refinement chains; a decoy chain lets an unrelated offer pass a declared absence (p05) | yes | §5 and `M3.4`'s acceptance: refused over every offer and absence, outside a direct refinement pair, through no chain; `presence.rs`'s comparison corrected there |
| K3 — §5 chose per clause, §10 let `M3.4` take two providers, so a group's head and its range could come from different providers | yes | §5, §10 and `M3.4`'s acceptance: which constraints of a clause may come from different providers is `M3.4`'s, a group and its sub-constraints never |
| K4 — `M3.3`'s acceptance took a provider whose privilege set is stated, not one that states none, so a silent provider and a silent requirer passed | yes | §3 rule 4 and `M3.3`'s acceptance: a provider stating no `reachable-at-privilege` is refused for any caller a plan binds to it, and one stating no power state for a plan entering one |
| K5 — rule 1 cited model §2 rule 2 for a per-provider reading it does not have; presence and the relation gave one `needs` two verdicts (p07) | yes | §5 and `M3.4`'s acceptance: model §2 rule 2 amended in `model.md` where a provider's derivation satisfies past another's absence; the relation itself only reports `absent` at the provider |
| K6 — infeasible reported while an unknown provider could still satisfy (p15) | yes | §5 and `M3.4`'s acceptance: `missing-fact` while some provider is unknown or undescribed and none satisfies |
| K7 — the vocabulary "read by the registry", which refuses any non-`defkind` form; `deffact` declared in `core.eadl` admitted in any description, the promised refusal with no mechanism | yes | §1.1: `deffact` declared in a kind module of its own that `eadl-resolve` registers to validate the vocabulary and no description's registry holds, so a `deffact` in a description is `schema-unknown-kind` |
| K8 — `refinement.rs` reads an offered `exactly` as a quantity bound | no | §1.1: `M3.1.2` changes its reader, a test per domain |
| K9 — a bare boolean beside `(f true)` matched two clauses of §5 | no | §5: the same value twice, one offer |
| K10 — a bound written by a block | no | unchanged: not read beyond presence, whoever writes it |
| K11 — the opening paragraph's wording against rule 3 | no | the opening paragraph names the presence requirement |
| K12 — `priorities` and `deadlines` `exact` over-refuse | no | conservative, unchanged |
| K13 — `(f false)` against an absence | no | conservative, unchanged |
| K14 — a service's `absent` | no | §1: presence's, not a provider's |
| K15 — the ledger entry does not yet quote the `mtime` sentences | no | §4: the commit adding the modulus adds them |
| K16 — a `needs` naming both a block and a fact | no | §5's delegation states `needs` of a vocabulary fact; a declaration's name is presence's |
