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
