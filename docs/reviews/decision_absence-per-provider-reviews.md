# Review history — `decision_absence-per-provider.md`

The independent review of [`docs/decisions/decision_absence-per-provider.md`](../decisions/decision_absence-per-provider.md),
leaf `M3.4.8`'s decision of `SR-H16`. Each round is a reader who had not seen the reasoning that wrote the record,
asked whether it is true to `ROADMAP.md` §5.3, `docs/semantics/model.md` §2 and §3, the substitutability relation's
record and the code, and complete for what `SR-H16` asks — never a hunt for code mutants.

## Round 1 — `2026-10-27`: not acceptable, 13 findings

The record then said a fact is not both offered and absent "at one provider", a block or a platform, and that a
refinement pair and composition changed nothing.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| D1 | defect | "a block or a platform" drops services: `defservice` admits `offers` and `absent`, and `presence.rs` refused a service's self-contradiction, which the record would stop refusing | Rule 1 reads per **declaration** — a block, a platform or a service — as the code compares any declaration's own; a test holds a service's contradiction refused |
| D2 | defect | an abstract platform's absence beside a block its refinement composes, offering the fact, was refused by description-wide rule 1 and would be refused by nothing: `refinement.rs` reads the concrete's own offers only — losing a negative constraint §5.3 asks refinement to preserve | Model §3 rule 3 amended and built: an exclusion is kept through what the concrete platform composes, at any depth (`refinement::exclusions_through`, called by `check.rs`); `tests/refinement_through.rs` |
| D3 | gap | a chain of three — A absent, B refines A, C refines B and offers — is accepted before and after, and owned by nothing | The same rule keeps every absence up the chain its abstract refines; presence's transitive exemption and its refinement map go; a test holds the chain refused |
| D4 | defect | rule 2 reads names, so a fact one provider declares absent and another only derives is refused infeasible, unlike an offered one | Stated: rule 2 is presence's, which reads no vocabulary, so it keeps refusing that case — unchanged by the record, owned by `M3.4.7` when the relation's verdicts reach `check`; the search binds the derivation today (false: corrected in round 2's R2-3). At one provider, a derived fact absent beside a fact its rule reads is the relation's refusal |
| D5 | gap | rule 4 leans on a search `archogen check` does not run: a system's block declaring a fact absent beside an offer outside its hardware moves from refused to accepted by `check` | Named as an interim verdict in the record and owned by `M3.4.7`, where `check` and the search come to agree |
| D6 | gap | "fact" against the names presence reads in `offers` and `absent` | Stated: rules 1 and 5 cover every such name; a declaration's own name is no offer |
| D7 | wording | rule 5's trigger narrower than the relation's "beside a fact its rule reads" | The relation's wording quoted, the refusal its |
| D8 | wording | `(f false)` and `(absent f)` called one statement; "(§5.3)" cited for "about the provider" | "two ways a device says it lacks `f`, a value and an absence"; the reading marked the record's own |
| D9 | wording | the quotation is the relation's §5, not §1; "the platform's own offer of the fact is absent" is wrong | §5 quoted with its text; "a platform declaring a fact absent of itself" |
| D10 | wording | "two accounts of one platform" would make a refinement pair one provider | "two declarations, the concrete owing the abstract model §3's obligations" |
| D11 | gap | How to apply leaves out model §6's row, §2 rule 2, the book's presence chapter, the relation's §10 bullet, `presence.rs`'s comment | Each named and changed — model §2, §3, §6, and §8 rule 9's sentence on rule 1; the book's presence and refinement chapters; the relation's §10, a dated note; the comment gone with the map |
| D12 | wording | "reviewed" and the case's rewrite claimed before they existed | They land in the commit that carries this review |
| D13 | wording | "went unseen" overstates the first-of-each bug: the verdict held, the pair reported was wrong | "went unreported — the verdict held, the wrong pair named"; "went unseen" kept for the exempt first pair |

## Round 2 — `2026-10-27`: not acceptable, 8 findings

The record then read rule 1 per declaration and rule 3 through a `requires`' `uses` and up a chain, and handed rule 2's
derived fact and the search's out-of-hardware verdict to `M3.4.7` in its own text only.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R2-1 | defect | composition read only a top-level `requires`' direct `uses`, narrower than model §8 rule 1's "a platform's or a block's `uses` is composition": `(defplatform board.rev2 (refines board) (uses uart.debug))` and a `uses` inside a nested `requires` dodged the exclusion; "at any depth" was ambiguous | `presence::composition` reads every `uses` a block or platform writes, at its top and inside any `requires`, except inside `offers`, `absent` and `refines`; "at any composition depth" in the record, model §2 rule 1 and §3 rule 3, and the book's refinement chapter; `refinement_through.rs` holds both spellings refused |
| R2-2 | defect | a platform's absence meant two things: `board (absent debug-port) (requires (uses uart.debug))` was accepted while `board.rev2`, repeating that composition, was refused — and a system using `board` would bind the block's offer past the platform's absence, choosing one side | One scope for a declaration's absence, itself and its composition: presence refuses a block or platform composing a part that offers what it declares absent, `invalid-description` (model §2 rule 1, the record's rule 4); a refinement repeating it is refused exactly when the abstract is; a test holds both |
| R2-3 | defect | rule 7 said `archogen resolve` binds the derivation today, but `resolve_with` answers the check's refusal before any search | Rule 7: refused `infeasible-configuration` by `archogen check`, and so by `archogen resolve`; nothing binds it until `M3.4.7`; round 1's D4 answer marked corrected |
| R2-4 | gap | the "owned by `M3.4.7`" claims were not written into `M3.4.7` | Both obligations written into the `M3.4.7` leaf, quoted: presence's rule 2 yields to the relation for a derived fact; `check` adopts the search's verdict for an offer outside the system's hardware or a service's |
| R2-5 | gap | the migration note said "per provider" and "refusing less", while rule 3 refuses a chain of three it accepted, and moves the composition case from 10 to 13 | Note renamed `eadl-1-an-absence-is-a-declaration-s-statement.md` and rewritten: per declaration, services included; "refuses less and more", the chain refused now, the 10→13 move named, measured as moving no frozen verdict |
| R2-6 | gap | a service's offer moves like an out-of-hardware one: accepted by `check` now, refused by the search; model §2 rule 2's "a fact one declaration offers is the search's to bind" is untrue for a service | Named in rule 8 and in `M3.4.7`'s obligation; model §2 rule 2 now reads "a fact a block or platform offers", the service's and the out-of-hardware offer accepted there and refused by the search until `M3.4.7` |
| R2-7 | wording | How to apply left out model §8 rule 9's sentence and the book's refinement chapter; §6's `refinement-violated` row named only the direct abstract | Both listed; the row reads "or of a platform up its chain"; §6's rule-1 row names composition |
| R2-8 | wording | a test comment said "a refinement that composes neither" over a fixture composing `uart.a` | "a refinement composing only a block that keeps the absence keeps it" |

## Round 3 — `2026-10-27`: acceptable, 5 findings, no defect — the review closed

Every round-2 answer settles its finding; R2-3 and R2-6 left one phrase each, R3-1 and R3-2.

| ID | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R3-1 | wording | rule 7 holds only for a presence requirement: a valued constraint, `(requires (unambiguous-horizon (at-least 60 s)))`, never reaches rule 2, so `check` accepts it and the search binds the derivation | "a presence requirement, `(needs f)`", in rule 7 and in `M3.4.7`'s first obligation |
| R3-2 | wording | rule 5's "a fact one declaration offers is the search's to bind" contradicts model §2 rule 2 and rule 8 for a service's offer | "a fact a block or platform offers is the search's to bind; one a service offers, or a block outside the system's hardware, is rule 8's" |
| R3-3 | wording | "neither composing the other" reads two-way where the rule runs one way: a platform offering a fact while a part it composes declares it absent is accepted | "the absent one not composing the offerer" in the record and the migration note; the reverse case named accepted, and held by `an_absence_covers_what_its_declaration_composes_and_no_more` |
| R3-4 | wording | the relation's §10 note, the book's presence summary and refinement chapter, `presence.rs`'s F06 comment and the record's first Why bullet still lack the one-scope rule | Each says the declaration "and what it is made of", or "which it does not compose" |
| R3-5 | gap | the `M3.4.8` leaf lags: placeholders, a Plan the review reversed, a Built list without composition, and no run cited on the final tree | The Plan marked reversed, Built brought up to date, the placeholders filled from runs on the committed tree |
