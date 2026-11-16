# An absence is one declaration's statement

- **Type:** `decision`
- **Date:** `2026-10-27`
- **Status:** `active` — written by leaf `M3.4.8` (`docs/tasks/M3.md`), as `SR-H16` asks of it; its review, closed in round 3, below
- **External sources:** none beyond `ROADMAP.md` §5.3 ("Maintain offered, explicitly absent, and undescribed states
  for facts"; "Reject contradictory declarations rather than choosing one"; "Platform refinement must preserve the
  obligations actually relied upon by the profile, including relevant negative constraints")
- **Owner / source:** the hand-off `SR-H16` of `docs/decisions/decision_substitutability-relation.md` §11, which gives
  `M3.4` the question in a reviewed record with its reason

## The fact / decision

**A fact is not both offered and absent by one declaration.** A block, a platform or a service that offers a fact and
declares it absent contradicts itself, and the description is `invalid-description`, as model §2 rule 1 has said since
`M1` — the relation refusing the same at one provider (its §8, "a provider offering and declaring absent one fact").

**A declaration's absence covers what it is made of.** A block or platform declaring a fact absent while a block or
platform it composes offers it says both of itself — a board with no debug port that includes a serial port with one —
and is `invalid-description` too: one scope for an absence, the declaration and its composition, in model §2 rule 1 as
in §3 rule 3.

**Two other declarations' offer and absence are two statements, each judged on its own.** A block declaring a fact
absent beside another it does not compose offering it says two things of two devices — a serial port with no
debug port beside one with a debug port — and the description is not refused for it. So model §2 rule 1 reads per
declaration, never description-wide. The scope runs one way: a block declaring a fact absent inside a platform
offering it is accepted, since the platform's offer needs only one of its parts to hold it:

1. **Every declaration counts.** Each declaration's offers and absences are kept, not the first of each fact the
   description writes, and rule 1 compares one declaration's own, with what it composes: no refinement exemption is
   needed, since one
   declaration is never a refinement pair with itself. `presence.rs`'s refinement map and its transitive exemption go.
2. **A refinement pair is model §3's.** An abstract and a concrete platform are two declarations, the concrete owing
   the abstract model §3's obligations; a fact the abstract declares absent and the concrete offers is a violated
   exclusion obligation (§3 rule 3), reported by `refinement.rs`, never rule 1's contradiction.
3. **An exclusion is kept through composition and through a chain** (model §3 rule 3, amended here). A concrete
   platform keeps its abstract's absences with what it composes — every block and platform a `uses` it writes names,
   at its top, inside `requires` or inside a nested `requires`, and theirs in turn, at any composition depth: a block's
   or a platform's `uses` alone is composition (model §8 rules 1 and 5) — so an absence is not bypassed by a composed
   part offering the fact; and it keeps the absences of every platform up the chain its abstract refines, so none is
   lost on the way. Before this record both were refused only by rule 1's description-wide reading, under the wrong
   code, or not at all through a chain of three.
4. **Composition is one reading.** `presence::composition` reads what a block or platform composes for rule 1 and for
   rule 3 alike, so a platform's absence binds its own composition and a refinement's the same way: a refinement
   repeating its abstract's composition is refused exactly when the abstract is.
5. **Rule 2 reads the description.** A required fact is absent to the description when some declaration declares it
   absent and none offers it, as `presence.rs` already read it. A fact a block or platform offers is the search's to
   bind at the provider that offers it (`M3.4`); one a service offers, or a block outside the system's hardware, is
   rule 8's.
6. **Names and vocabulary facts.** Presence reads every name in `offers` and `absent` — vocabulary facts and other
   names alike, `region` among them — so rules 1 and 5 cover every such name; the relation's judgement of an offer
   covers vocabulary facts alone, and an undeclared name has no domain to contradict there (its §5). A declaration's
   own name is no offer: a required name some declaration declares absent and none offers stays infeasible.
7. **An offered value and a derived one.** At one provider, a derived fact declared absent beside a fact its rule
   reads is the relation's refusal (its §8, "a derived fact declared absent beside a fact its rule reads"), as an
   offer beside an absence is rule 1's. At two, a provider declaring a derived fact absent beside another that
   derives it are two statements, as two offers are. Rule 2, which presence decides, reads names and no vocabulary,
   so a presence requirement, `(needs f)`, on a fact one provider declares absent and another only derives is refused
   `infeasible-configuration` by `archogen check`, and so by `archogen resolve`, which answers the check's refusal
   before any search: nothing binds it until `M3.4.7` makes presence's rule 2 yield to the relation for a fact the
   vocabulary derives — an obligation written into that leaf. That reading is unchanged by this record.
8. **What `archogen check` decides, and what the search decides.** Rule 5 reads the description, not one system's
   hardware or its providers: a fact a system's own block declares absent and a block outside its hardware offers, or
   a service offers — a service being no provider the search binds (the relation's §1) — is accepted by `archogen
   check` now, refused by rule 1 before, while `archogen resolve`'s search refuses it (`SR-H15`, `SR-H18`). The two
   agree once `M3.4.7` adopts the search's verdict in `check` for such an offer — an obligation written into that
   leaf.
9. **An adapter** (`M3.2`) is no declaration of the description: its offer satisfying past a declared absence is
   `SR-H9`'s, and this record decides nothing of it.

## Why

- **It is what an offer already is.** The relation judges each provider on its own: "An offer is one provider's
  statement (§1), so two providers offering one fact with two values are two offers, each judged on its own" (its §5).
  Two blocks writing `(f false)` and `(f true)` were accepted; the same two writing `(absent f)` and `(f true)` were
  refused — two ways a device says it lacks `f`, a value and an absence, taking two verdicts. This record decides that
  an absence, like a value, is a statement about the declaration that writes it and what it is made of; that reading
  is its own, §5.3 saying
  only that absence is a state to maintain and contradiction a thing to reject.
- **Description-wide reading refuses honest hardware.** A system of two serial ports, two timers or two cores of
  different capability is the ordinary case the joint rules (model §8) and the search exist for; refusing it as a
  contradiction would make the author misdescribe a device to pass.
- **The first-of-each comparison was also unsound.** `presence.rs` kept the first offerer and the first absence of
  each fact and compared those two alone: a contradiction at one declaration written after another pair went
  unreported — the verdict held, the wrong pair named — and where the first pair was exempt as a refinement, every
  other pair went unseen.
- **The negative constraints a refinement relies on stay kept.** §5.3 asks platform refinement to preserve "relevant
  negative constraints"; rule 3 keeps them through what a refinement is made of and through every link of a chain,
  where description-wide rule 1 caught some of them only as a contradiction, and none through a chain of three.
- **A contradiction stays refused.** A declaration saying both of itself is what §5.3's "reject contradictory
  declarations rather than choosing one" is about: only its author knows which half was meant.

## How to apply

- `crates/eadl-model/src/presence.rs` keeps every declaration of each fact and refuses an offer and an absence of one
  fact where one declaration wrote both or where the offerer is part of what the absent declaration composes; its
  refinement map is gone, and `composition` is the one reading of composition. `crates/eadl-model/src/refinement.rs`'s
  `exclusions_through`, called by `check.rs`'s refinement pass, keeps an exclusion through composition and through a
  chain.
- `docs/semantics/model.md`: §2 rules 1 and 2, §3 rule 3, §6's rows for rule 1 and for `refinement-violated`, and §8
  rule 9's sentence on rule 1. The book's presence chapter — its example and its summary — and refinement chapter.
  The relation record's §10 bullet on `SR-H16`, a dated note. `M3.4.7`'s leaf, the two obligations rules 7 and 8
  give it.
- F06's case (`docs/semantics/cases/invalid-contradictory-presence.eadl`) and its tests
  (`crates/eadl-model/tests/f04_f06_presence.rs`) write the contradiction this rule still refuses — one declaration
  offering and declaring absent one fact — and two declarations' statements are held accepted beside it;
  `crates/eadl-model/tests/refinement_through.rs` holds rule 3 through composition and through a chain. The verdict the
  case's old spelling moves is named in `docs/semantics/migrations/eadl-1-an-absence-is-a-declaration-s-statement.md`.
- Related: [[decision_substitutability-relation]] (§5, §8, §10, `SR-H16`), [[decision_eadl-engine-boundary]].

## Review

| Round | Date | Findings | Verdict |
| --- | --- | --- | --- |
| 1 | `2026-10-27` | 13: D1, D2, D4 defects; D3, D5, D6, D11 gaps; D7–D10, D12, D13 wording | not acceptable — answered in the revision: per declaration, services included; rule 3 through composition and chains, built; rule 2 and the search's verdicts stated, owned by `M3.4.7`; every place named |
| 2 | `2026-10-27` | 8: R2-1, R2-2, R2-3 defects; R2-4, R2-5, R2-6 gaps; R2-7, R2-8 wording | not acceptable — answered: composition is every `uses`, read once for both rules; a declaration's absence covers its composition; `resolve` binds no derivation the check refuses; the two obligations written into `M3.4.7`; the migration note rewritten and measured |
| 3 | `2026-10-27` | 5: R3-1–R3-4 wording; R3-5 a gap in the leaf, not the record | acceptable — answered: rule 7 a presence requirement's; rule 5 a block's or platform's offer; the scope one way, the reverse case accepted and tested; the one-scope rule in every lockstep text — the review closed |

The rounds, each finding and its answer, are in
[`docs/reviews/decision_absence-per-provider-reviews.md`](../reviews/decision_absence-per-provider-reviews.md).
