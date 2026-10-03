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
