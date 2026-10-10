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

**Round 9**, `2026-10-10`, of `ffd2708`, by a read-only context that had not written the record, fenced from the
sealed evaluation set. It ran the doctrine gate, the crate's tests (179 passed) and the route tests (14), reproduced
the leaf's test counts at rounds 5 to 8, and mutated the gate and the provenance — untracked, so not durable: the
judgments held in one direction only. Verdict: 2 defects and 3 remarks.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R9-D1 | defect | the §5 judgment held one way: a form's last generator alone, the last reader alone, a program-target generator for one reader alone, each passed every test | each held both ways: another role's step named first and last; a script from either role's package; a program-target generator in either role's second package, a test of its own |
| R9-D2 | defect | "every committed file any step reads" held for one input only | a later step's own committed input named and reached in the chain's route |
| R9-R1 | remark | `M3.6.8` said the two tests fail "under each" mutation | each test under its own; the leaf widened to round 9's mutations |
| R9-R2 | remark | no changelog entry for rounds 7 and 8 | one entry for rounds 7 to 9 |
| R9-R3 | remark | the marked pin held by equivalence | as written |

**Round 10**, `2026-10-10`, of `d05f333`, by a read-only context that had not written the record, fenced from the
sealed evaluation set. It ran the doctrine gate and the crate's tests (180 passed), rebuilt round 9's seven mutations,
each failing a test, and mutated the gate further — untracked, so not durable. Verdict: 2 defects and 2 remarks.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R10-D1 | defect | the chain, the blob rule and the both-clauses rule held only where the refused entry stood alone or at one position | one sweep, `route_tests_refusals_hold_at_every_position_of_a_form_s_entries`: eight refused entries, each alone, first, in the middle and last; the class closed rather than case by case |
| R10-D2 | defect | "directly or through another" held only directly | a package depending on the table's through another, refused, in the dependency test |
| R10-R1 | remark | stale test comments | rounds 5 to 10; the judgment test's lead comment |
| R10-R2 | remark | an input "under" a gitlink held only for a generator | in the sweep: an input under a gitlink at every position |

**Round 11**, `2026-10-10`, of `98f3773`, by a read-only context that had not written the record, fenced from the
sealed evaluation set. It ran the doctrine gate and the crate's tests (181 passed), rebuilt round 10's mutations, each
failing a test, and mutated further — untracked, so not durable. Verdict: 3 defects and 1 remark. Round 10's "the
class closed" was too wide: the sweep held the step-3 refusals, not every entry refusal §6 names.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R11-D1 | defect | a program-target step's §5 judgment held only where it stood alone | in the sweep: another role's program target at every position |
| R11-D2 | defect | the sweep held step 1 and step 3's refusals only: a program target whose build reads a declared file, a lone `.rs` and the gitlink itself as a generator each held at one position | each added to the sweep, every position; six mutations each fail it |
| R11-D3 | defect | `M3.6.8`'s text lagged round 10's widening | rounds 8 to 11, each mutation's test named |
| R11-R1 | remark | no changelog entry for round 10 | one entry for rounds 10 and 11 |

**Round 12**, `2026-10-10`, of `c216de9`, by a read-only context that had not written the record, fenced from the
sealed evaluation set. It ran the doctrine gate and the crate's tests (181 passed), rebuilt round 11's mutations, each
failing the sweep, and mutated further — untracked, so not durable. Verdict: 5 defects and 3 remarks. It proposed a
method — generate the cases from every refusal-producing branch, three readers and three inputs — which this round
adopts: §6 now says, of each test, exactly the positions and counts it holds.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R12-D1 | defect | another role's script in the middle of a form's generators, held by no test | in the sweep, every position |
| R12-D2 | defect | a root's own crate root, whose build reads the declared file, held only alone | in the sweep, `gen`'s own table its declared file, every position |
| R12-D3 | defect | a middle reader judged by no test | both reader tests have three readers, the refused one first, in the middle and last |
| R12-D4 | defect | a middle input's provenance held by no test | the chain's route names three inputs, each reached |
| R12-D5 | defect | round 11's texts: "only where the refused file stood alone"; the gitlink as a generator "at one position" | "alone or at one position"; the gitlink as a generator at none |
| R12-R1 | remark | round 11's account named "the step-3 refusals" and "step 1 and step 3's" | step 1 and step 3's |
| R12-R2 | remark | the tests' doc comment said rounds 5 to 10 | 5 to 12 |
| R12-R3 | remark | the self-generating route's "any package it depends on" without "directly or not" | said, in the record and the chapter |

**Round 13**, `2026-10-10`, of `ff0a180`, by a read-only context that had not written the record, fenced from the
sealed evaluation set. It ran the doctrine gate, the crate's tests (181 passed) and the inventory at HEAD, and
mutated and probed the gate — untracked, so not durable: the gate refused what the record says in every case it
probed. Verdict: 6 defects and 3 remarks. Round 11's paragraph above, "the sweep held the step-3 refusals", stands as
written; R12-R1's answer, "step 1 and step 3's", is its correction.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R13-D1 | defect | the third reader never asserted, three of six orders run | both reader tests run all six orders and assert every reader of another role |
| R13-D2 | defect | the harness as a reader held only by a test §6 did not name | `generator_tests::the_harness_may_use_its_pair_s_roles_and_no_other` named, with what it holds |
| R13-D3 | defect | the chain's second clause through a marked file held only alone | in the sweep: a program target compiling a marked, undeclared module, every position — fifteen kinds |
| R13-D4 | defect | no row stated a middle generator's provenance | the vendored test's row: three generators, the executable in the middle, each reached; the chain's row: two steps |
| R13-D5 | defect | `M3.6.8` lagged round 12 | rounds 8 to 13, each mutation's test named |
| R13-D6 | defect | rounds 10 and 11's completeness claims stood | the changelog entry narrowed to the kinds it held; review rows 10 and 11 stand as written, this row their correction |
| R13-R1 | remark | R12-R1's answer was not applied to round 11's paragraph | said above |
| R13-R2 | remark | two sentences held only by equivalence, what a script runs being unread | kept: the parent's §8, as R9-R3 |
| R13-R3 | remark | "a form no program reads is held to the parent's §6" has no row | the parent's behaviour, cited, not a route |

**Round 14**, `2026-10-10`, of `0797cf5`, by a read-only context that had not written the record, fenced from the
sealed evaluation set. It ran the doctrine gate and the crate's tests (181 passed), and mutated the gate — untracked,
so not durable: the rules held in the last live form only, and the harness unjudged against a program target. Verdict:
5 defects and 3 remarks. Rounds 9 to 14 each found one more axis — entry position, reader order, the third reader,
form order, the harness — on which a rule the record restated from the parent was held one way; the answer changes
method: the record says those rules are the parent's, held by its instrument's tests, and their breadth across every
axis is a leaf of its own, `M3.6.9`, where one generated sweep can hold every axis at once.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R14-D1 | defect | R13-D6's answer: the round-11 changelog entry still claimed every kind then named | the entry names the kinds its sweep held, the three others at fewer positions |
| R14-D2 | defect | the leaf's round-12 text said every order | "the refused one first, in the middle and last"; the log row so |
| R14-D3 | defect | every entry refusal and role judgment held in the last live form only | the rules attributed to the parent, held by its tests; their breadth across forms filed as `M3.6.9` |
| R14-D4 | defect | a program-target generator judged against the harness held by no test | the parent's rule; `M3.6.9` holds the harness among every reader |
| R14-D5 | defect | §6's opening said three parent tests, all no-route cases | four, three no-route cases and the harness as a reader |
| R14-R1 | remark | stale test comments | rounds 5 to 14; the sweep's comment names the marked build-read |
| R14-R2 | remark | the sweep's row read as thirteen kinds | "a generator or input at or under a gitlink" |
| R14-R3 | remark | sharing held by equivalence with the parent's tests | the parent's §4, now said so |

**Round 15**, `2026-10-10`, of `0369126`, by a read-only context that had not written the record, fenced from the
sealed evaluation set — one search in its clone was unfenced in form, scanning the manifest, all the sealed directory
holds, for this record's name; it matched and printed nothing, and no case text was seen. It ran the doctrine gate and
the crate's tests (181 passed), found every attribution round 14 made true against the parent's record and tests, and
every route and no-route case built by a §6 test, and mutated the gate — untracked, so not durable. Verdict: 1 defect
and 4 remarks.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R15-D1 | defect | R13-D6's answer: the Commit Log's rows of rounds 10 and 11 still claimed every refusal at every position, and round 8's every reader | every row a later round narrowed — rounds 5, 6, 8, 10, 11 and 13 — annotated with its narrowing, as round 12's was |
| R15-R1 | remark | `M3.6.9` named no clause axis: the chain and the blob rule held for inputs alone outside `route_tests` | "each clause — generators and inputs" in its axes and its sweep |
| R15-R2 | remark | a file named in both clauses and a lone `.rs`, the parent's rules, named neither in §6's opening nor in `M3.6.9` | named in both |
| R15-R3 | remark | "rounds 9 to 14 each found one more axis" overstated | "round after round, another axis or kind", in the changelog, round 14's paragraph and `M3.6.9` |
| R15-R4 | remark | §6's opening readable as the parent's tests holding every form, position and reader; an appositive without its antecedent | "held, at the instances they build"; "(its rule, held by its instrument's tests)" |

**Round 16**, `2026-10-10`, of `13520b7`, by a read-only context that had not written the record, fenced from the
sealed evaluation set — it read nothing under the sealed directory and ran no recursive search. It ran the doctrine
gate and the crate's tests (183 passed) and the inventory at HEAD, found every rule §6 calls the parent's in the
parent's record with a test outside `route_tests`, and every route and no-route case built by a §6 row, and mutated
the gate — untracked, so not durable. Verdict: 5 defects and 3 remarks. Round 15's answer to R15-D1 annotated the rows
it named and stopped short, as the answers to rounds 13 and 14 had; and that answer rewrote a line of round 14's
paragraph above, which this round's answer restores — its "each found one more axis" stands as written, R15-R3 its correction. The answer changes
method: the leaf's ledger entries are each as its own commit wrote it, a note above its Commit Log says so, and this
history and §6 carry what holds now.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R16-D1 | defect | R15-D1's answer: annotations stopping short, rows narrowed but not annotated, and "every row … annotated" false | every row back to its own commit's text, checked against each commit; one note says what the rows are; the leaf's round-15 sentence corrected |
| R16-D2 | defect | the changelog's entries of rounds 6 and 9 kept claims later rounds narrowed | every entry of this leaf back to its own commit's text, under the same note |
| R16-D3 | defect | round 14's paragraph here, and the changelog's round-14 entry, rewritten in place | both restored; R15-R3's correction stands in round 15's row and here |
| R16-D4 | defect | `M3.6.9` and §6's opening put axes on rules that lack them: clauses on a lone `.rs`, live forms and clauses on a file named in both | each rule's own axes: a lone `.rs` among generators, a file in both clauses in every form, live or not; the sweep only over those cells |
| R16-D5 | defect | `MEMORY.md`'s next action still "push to restore CI" after the push | the next action rewritten by `4b26641` and `313f6d4` |
| R16-R1 | remark | the frontier's "`.8` and `.9`, the instrument's tests' breadth" | `.8` the mutation catalogue's, `.9` the parent's tests' |
| R16-R2 | remark | `M3.6.9`'s list said rounds 9 to 15 and named no round-15 axis | the clause (round 15) and how a reader reads (round 16) named |
| R16-R3 | remark | no test refuses another role's generator for a reader handed the file as data | how a reader reads, in `M3.6.9`'s axes |

**Round 17**, `2026-10-10`, of `c3c55dc`, by a read-only context that had not written the record, fenced from the
sealed evaluation set. Its clone refused by the harness, it read paths outside the sealed directory from the object
store and ran a history-free copy, so the doctrine gate's six history legs went unchecked. It found every row and
round block round 16 restored identical to its commit's text, every rule §6 calls the parent's decided there and
tested outside `route_tests`, and the crate's tests passing (183), and mutated the gate — untracked, so not durable.
Verdict: 3 defects and 5 remarks. Round 16's "every changelog entry" missed the first, sealed in a segment with rounds
1 and 2's edits; and `M3.6.9`'s list of axes, rewritten by rounds 15 and 16, left out two more. The answer changes
method for `M3.6.9`: its goal derives the axes from a census of the code's loops instead of listing them.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R17-D1 | defect | "every changelog entry … back to its own commit's text": the first, sealed in segment `0020`, holds rounds 1 and 2's edits | the note names it, standing as sealed; the leaf's round-16 sentence says so |
| R17-D2 | defect | `M3.6.9` dropped form order for a lone `.rs` and the role judgment, and entry position for a file in both clauses | the axes derived by a census of every loop a refusal or judgment sits in, no list kept |
| R17-D3 | defect | `M3.6.9` said the clause axis passed every test; `route_tests` held it | its list says some were held by `route_tests` alone |
| R17-R1 | remark | the note named rounds 13, 15 and 16; round 14 also found one | "rounds 13 to 16" |
| R17-R2 | remark | rows 13 to 15 state ledger narrowings round 16 undid | row 17 says so; this history carries the narrowing |
| R17-R3 | remark | Why's "as its instrument's tests hold them" unqualified | "at the instances they build", as §6 says |
| R17-R4 | remark | how a reader reads also decides which forms are live, an axis of the chain, the blob rule and a lone `.rs` too | in the census `M3.6.9` derives |
| R17-R5 | remark | round 16's Verification Log row cited its runners without saying they are untracked | round 17's row says so; the note covers the leaf's Verification Log rows as written |

**Round 18**, `2026-10-10`, of `21e1c0b`, by a read-only context that had not written the record, fenced from the
sealed evaluation set. Its clones refused by the harness, it exported the commit without the sealed directory and
read history path by path outside it, so the doctrine gate's history legs went unchecked. It found every ledger entry
the note covers identical to its commit's text and the sealed one as sealed, every rule §6 calls the parent's decided
there and tested outside `route_tests`, every route built by its row, and the crate's tests passing (183); and it
mutated the gate — untracked, so not durable. Verdict: 2 defects and 5 remarks. Round 17's census described in words
still left axes out, and §6 still listed them. The answer changes method a last time for `M3.6.9`: no axis is named;
its criterion is the project's mutation sweep over the judgment code, every survivor killed or listed as equivalent.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R18-D1 | defect | `M3.6.9`'s census, "every loop a refusal or judgment sits in", misses what is no loop — `live()`, `reads`, `own_roles` — and its parenthesis omits the packages a generator's build holds | the criterion a mutation sweep of the judgment code, every survivor killed or listed as equivalent; no axis named |
| R18-D2 | defect | §6 still listed axes — form, clause, position, reader, how a reader reads — and omitted round 17 | §6 names none: "wherever its code judges it", the criterion since round 18 |
| R18-R1 | remark | the packages a program-target generator's build holds, first only, survives | in `M3.6.9`'s list of what rounds found; the sweep's to kill |
| R18-R2 | remark | the harness pair's second role, first only, survives | likewise |
| R18-R3 | remark | readers handed the file as data excluded survives | likewise, as R16-R3 |
| R18-R4 | remark | step 5 naming only the first generator per file survives | likewise |
| R18-R5 | remark | `MEMORY.md` named round 9's holes no committed layer recorded | `PROGRAM.76`'s round 9, answered by `c4523f5`, records them |

**Round 19**, `2026-10-10`, of `56cef5f`, by a read-only context that had not written the record, fenced from the
sealed evaluation set. Every copy refused by the harness, it read the record, the parent and the code path by path and
ran the crate's tests in round 18's export, its code identical to `56cef5f`'s (183 passed); it listed the sweep's
mutants and probed its baseline. Verdict: 2 defects and 3 remarks. Round 18's criterion is incomplete — the sweep's
operators never narrow an iteration nor drop an operand inside a closure, so round 18's survivors sit on lines it does
not mutate — uncheckable as written, and unreachable today, its copy unable to run the trust tests. The answer changes
method: the record and its review stop prescribing `M3.6.9`'s criterion; that leaf states the problem and what its
design must meet, and designs and reviews its criterion itself.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R19-D1 | defect | the sweep's operators make none of the mutants rounds 9 to 18 found; "an axis a test misses is a mutant that survives" false; R18-R1 to R4's "the sweep's to kill" false | `M3.6.9` names the sweep's gap as a fact its design must meet; R18-R1 to R4 are `M3.6.9`'s to hold, by the criterion it designs |
| R19-D2 | defect | the criterion uncheckable: one file whole, "the steps" undefined, no filter or equivalents named, route-test kills counted, exit 0 glossed wrong | no criterion stated outside `M3.6.9`; its design names the command whole, under its own review |
| R19-R1 | remark | the sweep cannot reach exit 0: its copy cannot run the trust tests | in `M3.6.9`'s goal, measured |
| R19-R2 | remark | "a list of axes … rounds 15 to 18" misdescribed round 18 | a list (15, 16), a census (17), the sweep (18, 19) |
| R19-R3 | remark | `MEMORY.md`'s frontier line on `PROGRAM.76` stale | rewritten |

## Why

The record closes on a round that finds no defect, as every design here does; each round, its findings and their
answers are appended above this section.
