# Generated sources' refused shapes and believed tool: the independent review, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-10`
- **Status:** `active`
- **Owner / source:** leaf `M3.6.6.4` (`docs/tasks/M3.md`). This is the review history of
  [`decision_trust-generated-refusals.md`](../specs/trust/decision_trust-generated-refusals.md), kept apart from it as
  `docs/reviews/INDEX.md` describes.

## The fact / decision

**Round 1**, `2026-10-10`, of `f3f1ad0`, by a read-only context that had not written the record. It reproduced the
measurement at `614d5c7`, ran the crate's tests (163 passed) and the whole mutation catalogue (*"OK — 333 mutation(s)"*),
checked the code moved only messages, and probed each route with fixtures of its own — its module untracked, so its
outcomes are not durable. Verdict: 8 defects and 8 remarks.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R1-D1 | defect | the chain route's program-target step "becomes a later step of a script that runs both": its build, made from the commit, cannot compile an uncommitted intermediate, and a committed one is a chain or escapes as a plain file | the later step reads the intermediate at run time, handed over by the script, its build compiling no generated file; one that must compile it has no route (§5) |
| R1-D2 | defect | the route named "the first step's committed inputs", leaving a later step's committed inputs out of every provenance | "every committed file any step reads" |
| R1-D3 | defect | a committed copy or pin whose header marks it generated is a chain the parent refuses, so "each need has a route" was false | the routes hold for unmarked files; a marked one has no route (§5), said in §1, §3, §4 and the chapter |
| R1-D4 | defect | the gitlink route served an input alone | a vendored generator's route: a committed script runs it, the vendored one that script's believed tool |
| R1-D5 | defect | "their messages name this record": only the chain's do | "the chain's refusals name this record"; the changelog's entry corrected |
| R1-D6 | defect | R1-6 was answered by transitive provenance, not left open | the history as it was: transitive provenance, then R3-5 and R3-7, then the narrowing |
| R1-D7 | defect | "the form's `command` names the tool": the command names the script | the tool is named in the script the form names as its generator |
| R1-D8 | defect | the leaf's `grep -n "R3-7"` output | `97`, `112` |
| R1-R1 | remark | "a chain hides nothing" holds for one the parent's rules see | qualified: a chain through an unmarked, undeclared file is the parent's §8 limit |
| R1-R2 | remark | a change of tool is a provenance change only through the pin | said |
| R1-R3 | remark | "no file under it has a sha256 the commit records" | the files are another repository's blobs, which the gate never reads |
| R1-R4 | remark | §5 judges generator files; inputs are matched | said |
| R1-R5 | remark | the doc comment at `trust_generated.rs` garbled | rewritten |
| R1-R6 | remark | two catalogue `why` lines still said "until `M3.6.6.4`" | they name the record |
| R1-R7 | remark | the leaf's "held with their new messages": the tests match the messages' heads | the leaf says the refusals are held |
| R1-R8 | remark | the decisions index's row lacked "under review"; the reviews index row split two rows; the parent's §6 note removed text from a closed record | each repaired; the note adds text alone |

**Round 2**, `2026-10-10`, of `c6a3990`, by a read-only context that had not written the record. It reproduced the
measurement at `614d5c7`, ran the crate's tests (165 passed), the chain mutations and the doctrine gate, and probed the
routes with fixtures of its own — untracked, so their outcomes are not durable: the chain route accepted, a pin shared,
a marked pin refused. Verdict: 5 defects and 6 remarks.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R2-D1 | defect | "every need has a route when its files are unmarked": a later step that must compile the intermediate has none, unmarked | "a route where one exists", the cases with none named in §1 |
| R2-D2 | defect | round 1's D3 answer had not reached the parent's notes, the decisions index, the chapter's *Today and ahead*, the leaf, the changelog or the record's title | each says "where one exists" |
| R2-D3 | defect | the vendored generator's route made it a believed tool, for §4's reason, false for it, and outside the Why's guarantees | a vendored generator that is a file: a committed, unmarked copy named as the generator, judged and hashed; a vendored program that cannot be copied: no route, believed for §3's reason, its sharing unseen; the Why narrowed |
| R2-D4 | defect | "a file the review that accepts the form reads (`M3.6.5`)" was nowhere in `M3.6.5` | the script is a provenance file every reader of the form reaches (the parent's §4) |
| R2-D5 | defect | the leaf still said the refusals' messages name the record | "the chain's refusals name the record" |
| R2-R1 | remark | an intermediate a program also reads is committed and declared, and reading it is a chain | said: the later step remakes it |
| R2-R2 | remark | a file one step runs and another reads, named in both clauses, is refused | said: named once, as a generator |
| R2-R3 | remark | §3's "what it hashes, shares and compares is a file" overreached; the parent's round 2 took the pinned commit as a digest | "every file it hashes is a blob"; why the pinned commit is not taken again |
| R2-R4 | remark | a vendored tool's pin is its gitlink, no blob | its route: a committed version file the script checks |
| R2-R5 | remark | "escaped … through one" | "through a chain" |
| R2-R6 | remark | `MEMORY.md` still named round 1 | updated with `PROGRAM.69`'s round 12 answers |

**Round 3**, `2026-10-10`, of `4ed677b`, by a read-only context that had not written the record. It reproduced the
measurement and the leaf's figures, ran the crate's tests (165 passed) and the doctrine gate, and probed the routes —
untracked, so the outcomes are not durable: a copied vendored script accepted and shared, a lone copied `.rs` refused, the
chain route's remade intermediate accepted, a version file shared and a marked pin refused. Verdict: 3 defects and 5
remarks.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R3-D1 | defect | How to apply still routed a vendored generator through a committed script, round 1's answer | as §3 reads: a file copied as the generator; an executable that cannot be copied, no route, believed |
| R3-D2 | defect | §3 and §4 decided a vendored executable a script runs twice, under two reasons, §4's false for it | decided in §3 alone, for its reason; §4's version-file route its only comparison; §4 and the chapter defer to it |
| R3-D3 | defect | "every file it hashes is a blob of the commit" and "the inventory hashes only what the commit holds": a root's artifact is hashed too | what a program reads and what a form names is a blob of the commit, the build's artifacts aside |
| R3-R1 | remark | "program" is a root or the harness in the parent | "a program-target step", "a vendored executable" |
| R3-R2 | remark | §1's no-route case named the intermediate alone | "another generator's output" |
| R3-R3 | remark | a lone copied `.rs` is refused as a generator | a Rust one copied as a program target; a copy the parent's rules refuse counts as one that cannot be copied |
| R3-R4 | remark | a script can reach its tool through the `command` | "in the script or the `command`" |
| R3-R5 | remark | the chapter's chain bullet left its no-route case to the gitlink bullet | moved into the chain bullet |

**Round 4**, `2026-10-10`, of `dbb897e`, by a read-only context that had not written the record. It ran the doctrine
gate and the crate's tests (165 passed), re-measured at `614d5c7` and wrote five probes — untracked, so the outcomes
are not durable: a gitlink input in a form no program reads not refused; a marked vendored copy refused, and the same
need met by one form remaking it from copies of its own generator and input; a program compiling the file its own form
declares refused, and the same need met by a second executable of its package named as the generator. Verdict: 3
defects and 4 remarks. Round 3's answers hold; round 3's row and history above describe round 3's answer as made.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R4-D1 | defect | "every generator and input a form names is a blob of the commit": the blob rule runs over live forms alone | "a live form names"; §3's opening says the rule refuses in a live form, a form no program reads held to the parent's §6; the chapter's bullet a live form's |
| R4-D2 | defect | a marked vendored copy "with no route", where §2's chain route serves it | §2: a marked file a form would name is an intermediate the generation remakes from copies of its own generator and inputs; §3, §1, How to apply and the chapter so; a marked pin remade so too, or replaced by an unmarked version file the script checks |
| R4-D3 | defect | a program that compiles the file its own form declares had neither a route nor a no-route statement | §2: it gives way to another program target compiling no generated file, a second executable of its package, named as the generator; one that must compile a generated file has no route; §1, How to apply and the chapter's plain words so |
| R4-R4 | remark | §4 called its route a host tool's while §3 used it | "this section's reason is a host tool's, and its route serves both" |
| R4-R5 | remark | "or whose copy those rules refuse" missing outside §3 | in §1, How to apply and the chapter |
| R4-R6 | remark | the chapter implied a vendored tool's identity cannot be compared | "its identity compared the same way" |
| R4-R7 | remark | a vendored input or file that cannot be copied had no decision | §3: a vendored file that cannot be copied has no route — an input stays refused, a generator believed when a script runs it; §1 and How to apply |

**Round 5**, `2026-10-10`, of `847ce6d`, by a read-only context that had not written the record. It ran the doctrine
gate and the crate's tests (165 passed) and three probes — untracked, so not durable: a second executable of a package
whose library holds the declared file refused; a copied executable named as a generator file written; a marked crate
root remade by a script written, its role judgment lost. Verdict: 3 defects and 3 remarks. Five rounds had each found
a route the record named fail in an untracked probe; the answer changes method: every route and no-route case is a
committed test (§6), and the record claims no more than they hold.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R5-D1 | defect | "a second executable of its package" fails when the package's library holds the file | a target whose build does not compile it: a second executable when no library holds the file, else one of a package not depending on it; two tests |
| R5-D2 | defect | "an executable" listed among files that cannot be copied, though a copy is a generator file | an executable is a file a copy serves; "cannot be copied" is a licence, a size or a build product; a test |
| R5-D3 | defect | the remake route escaped the role judgment for a marked crate root | the remake serves a marked input or script; a marked crate root has no route; a script remaking it is the parent's §8 limit, stated; a test |
| R5-R1 | remark | the chapter's "the commit does not hold as a file" unqualified | "of such a form", the form a program reads |
| R5-R2 | remark | "copies of its own generator and inputs" for every marked file | "its own generator and inputs, or copies of them for a vendored one" |
| R5-R3 | remark | a generator whose copy the role rules refuse, believed, escapes that judgment | said: its role judgment goes unseen, as the parent's §8 says |

**Round 6**, `2026-10-10`, of `682dab9`, by a read-only context that had not written the record, fenced from the
sealed evaluation set. It ran the doctrine gate and the crate's tests (174 passed), six mutations of the gate, each
caught by the route test it should be, and two probes — untracked, so not durable. Verdict: 3 defects and 5 remarks.
Round 5's account, "five rounds had each found a route fail", is too strong: round 2's probes agreed with the record.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R6-D1 | defect | a second executable of its package fails when a package it depends on holds the file | narrowed to the packages it depends on, directly or not; a test |
| R6-D2 | defect | "each route and each case with none" held by §6, where three were held by none and three by tests §6 did not name | two tests added; the parent's three named in §6 with their module |
| R6-D3 | defect | a vendored build product believed "not for §4's reason", though its bytes are the host's build | believed for §3's reason and §4's too |
| R6-R1 | remark | the believed test's "no `vendor/` path" could not fail | a control: the tool named on the form, refused |
| R6-R2 | remark | the routes of tests 4 and 5 hold "written", not the generator built | kept; test 6 holds the build |
| R6-R3 | remark | round 5's "each" overstated | said above |
| R6-R4 | remark | the chapter's "those rules" had no antecedent | "the role rules" |
| R6-R5 | remark | no changelog entry for round 5's tests | one entry for rounds 5 and 6 |

**Round 7**, `2026-10-10`, of `22247c4`, by a read-only context that had not written the record, fenced from the
sealed evaluation set. It ran the doctrine gate, the crate's tests (177 passed) and the route tests (12), and nine
mutations of the gate — untracked, so not durable: eight caught by the tests §6 names, one, a declared generator not
taken for a chain, caught by none. Verdict: 2 defects and 3 remarks. Round 6's correction of round 5's "each" holds
for round 4 too, whose probes found a no-route claim false and a route missing, not a named route failing.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R7-D1 | defect | a declared script named as a generator, refused by the gate, held by no test | a control in `route_a_chain_made_in_one_generation`: refused, "a chain" |
| R7-D2 | defect | "a generator whose copy the parent's rules refuse: no route", where an admission lifts some of those rules | "the role rules", as §6 and the chapter have it |
| R7-R1 | remark | the leaf and the changelog kept "five rounds" and "review after review" | "review rounds found routes … fail" |
| R7-R2 | remark | the chapter's "does not depend on it" ambiguous | "on the package holding the file" |
| R7-R3 | remark | the self-generating route is judged by §5 too | said |

**Round 8**, `2026-10-10`, of `fd5425e`, by a read-only context that had not written the record, fenced from the
sealed evaluation set. It ran the doctrine gate, the book's gates, the crate's tests (177 passed), the focused tier and
the inventory at `fd5425e`, and seven mutations of the gate — untracked, so not durable: five caught by the tests §6
names, two, a form's first generator judged alone and its first reader judged alone, caught by none. Verdict: 1
defect and 4 remarks.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R8-D1 | defect | the parent's §5 judgment of every step's generator against every reader, which §2, §3 and Why claim, held by no test | two tests, each failing under its mutation; §6 rows |
| R8-R1 | remark | the marked pin's route held only in shape | §6's row names §4 and the pin, an input as a pin is |
| R8-R2 | remark | the tests' doc comment named rounds 5 and 6, and refusals alone | rounds 5 to 8; what is believed, written unseen |
| R8-R3 | remark | a Rust copy's route needs its build, offline from the commit | said, citing the inventory record's §3 |
| R8-R4 | remark | the mutation catalogue holds neither judgment | filed: `M3.6.8` |

## Why

The record closes on a round that finds no defect, as every design here does; each round, its findings and their
answers are appended above this section.
