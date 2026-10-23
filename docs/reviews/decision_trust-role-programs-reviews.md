# The checkers' programs in the trust inventory: the independent review, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-23`
- **Status:** `active`
- **Owner / source:** leaf `M3.5.2` (`docs/tasks/M3.md`). This is the review history of
  [`decision_trust-role-programs.md`](../specs/trust/decision_trust-role-programs.md), kept apart from it as
  `docs/reviews/INDEX.md` describes.

## The fact / decision

Each round is a new context, read-only, which did not write the record, fenced from the sealed evaluation set. It is
given the record; its parent, `decision_trust-inventory.md`, and the generated-sources record it follows in form;
`ROADMAP.md` §4.4, §10.2, §10.4 and §14.4; `M3.5`'s and `M3.5.1`'s reviewed designs in `docs/tasks/M3.md`;
`trust/roots.eadl`, `trust/baseline.eadl`; `xtask/src/trust.rs`, where the parent's harness and roots are read and
judged; and the crates the record names. It may run read-only commands and the inventory under `target/`.

## Rounds

**Round 1**, `2026-10-23`, by a context that did not write the record, read-only, fenced from the sealed evaluation set;
it re-measured §1 at `0a5a4b12` on `aarch64-apple-darwin` (`cargo xtask trust-inventory`: 5 programs, 8 pairs, 10
shared items) and read the instrument (`xtask/src/trust.rs`, `trust_verify.rs`, `install.rs`). Verdict: not acceptable
as written, 23 defects and 10 remarks. The gap was misstated — the gate does not refuse the comparison, it never sees
it — and the instrument already reads a harness for any pair; the answer narrows `defharness`, defines the derivation
against the code, confines invocation to the front, and owns every decision by a hand-off. The author re-measured
R1-D1, D2 and D3 in the code before answering.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R1-D1 | defect | §1's gap is false: the gate builds no test but the harness's and follows no development edge, so the comparison is invisible, not refused | §1 states invisibility, measured against `PROGRAM_KINDS` and parent §3; only a named test is built and judged |
| R1-D2 | defect | `check_pairs` reads a `defharness` for any two roots, so the comparison could be written as a harness, paired with every root and accepted as a producer | `defharness`'s pair narrowed to a reference model and its implementation (§3, §6), its fixture `RP-H2`'s |
| R1-D3 | defect | the harness item's aspects are content, configuration and provenance, not edges and readers | corrected (§3) |
| R1-D4 | defect | "built as the harness" is undefined against the code: pairing, producer acceptance, one test target and one artifact | one test target a form; built and recorded as the harness's; paired with its pair alone; never a producer, `TI-H8` unchanged; the shape refusals listed (§3) |
| R1-D5 | defect | the joint reading's comparison, written apart, shares one item and one class with the relation's | the relation's alone; the joint comparison a form of its own, its class its review's (`RP-H4`) |
| R1-D6 | defect | `RP-H2` at `M3.5.6` breaks the record's own rule: the subject first exists at `M3.5.3`; no hand-off for the joint test | `RP-H2` to `M3.5.3`; `RP-H4` to `M3.5.5` |
| R1-D7 | defect | `eadl-resolve` is no role package of the generator, so the exemption and §1's premise are vacuous, and no leaf makes it one | stated vacuous until then, the item reported all the same (§3); `RP-H7` to `M3.4` |
| R1-D8 | defect | `archogen-outcome` given two classes; a form holds one | interpretation/normalization, its reader the reason (§5) |
| R1-D9 | defect | §5 omits the invocation items §4 creates | a row (§5) |
| R1-D10 | defect | the `(invokes …)` refusals are incomplete and wider than the ruling: a checker could invoke; a non-executable invoker, a repeat, an empty clause pass | on the front's form alone; every listed case refused, from the forms' text, before any build, wherever the gate runs (§4) |
| R1-D11 | defect | the install places every executable root, whatever is invoked | the install places exactly the invoked roots (§4, `RP-H5`) |
| R1-D12 | defect | no stated coverage limit for what the front actually runs or the installed name | *What stays outside* (§4), the parent's §3 list amended (§6), the name held by a test (`RP-H5`) |
| R1-D13 | defect | `(installed …)` against the verifier's handed-dependency rule and the blob rule; its grammar unstated | a bare name, `archogen_install::FILE`; exempt from the blob rule; the package records it beside the artifacts, never handed (§4, `RP-H8`) |
| R1-D14 | defect | no owner adds `scheduling-checker` to the invocations | `RP-H6` to `M2.21.3` |
| R1-D15 | defect | the amended `TI-H18` gives `M3.4` an invocation `M3.5.1.4` builds later | `TI-H18` restored; the invocation is `RP-H5`'s |
| R1-D16 | defect | `TI-H19` and §2 drop every future data declaration | scoped to the kind modules; any other run-time data declared by path (§2, `TI-H19`) |
| R1-D17 | defect | three sentences of the tree still say the kind modules are the root's `(data …)` | amended in this change |
| R1-D18 | defect | `HANDOFF-LEDGER` fails: no hand-off quoted | each quoted by its leaf: 71 hand-offs in 4 ledgers, held |
| R1-D19 | defect | §6 misses parent passages: the opening and standing list, §3's build, scope, admissions and outside, §5's aspects, three codes, `TI-H9`, `TI-H24`, §2's sentence on `analyze` | §6 enumerates each |
| R1-D20 | defect | no owner writes §5's classes into forms; `M3.5.3`'s new items are `trust-form-missing` until the runner proposes them | `RP-H1` (the runner's forms, carrying §5) and `RP-H9` |
| R1-D21 | defect | no owner for the book's trust chapter | in `RP-H2` and `RP-H5` |
| R1-D22 | defect | the generator compiles `archogen-json` directly since `e9245a45`, not only through the outcome package | corrected (§1) |
| R1-D23 | defect | the executable replaces the library form, not `defrole`; the parent's rows are above its clarification; "by this record's review" | corrected (§2, §6, the parent's clarification) |
| R1-R1 | remark | some controls are not true today: the exit code's channel, the amendment order, the canonical re-write for messages | each marked *pending* with its owner, a procedure, or narrowed (§3, §5) |
| R1-R2 | remark | §5 omits the new pairs' build configuration, `archogen-transport`, the scheduling checker's later reading | rows and a sentence (§5); `RP-H10` |
| R1-R3 | remark | the derivation shares transcribed code, not only a reading | stated (§1, §3) |
| R1-R4 | remark | the comparison reads the kind modules at run time | stated as outside (§3) |
| R1-R5 | remark | the invocation item's identity and aspects unspecified | specified (§4) |
| R1-R6 | remark | the `include_str!` sites arrive at `M3.5.4`, not `.3` | §2; `RP-H3` to `M3.5.4` |
| R1-R7 | remark | stale `crates/archogen-check` and `archogen-evidence`'s sha256 in the tree | amended |
| R1-R8 | remark | cite a committed revision and host | `0a5a4b12`, `aarch64-apple-darwin` (§1) |
| R1-R9 | remark | reading §4.4's tools as programs a root runs is an extension | said so (§1) |
| R1-R10 | remark | "refused as unknown" is imprecise: `read_roots` fails, exit 2 | corrected (*How to apply*, §3) |

**Round 2**, `2026-10-23`, a new context at `6fccaabc`, read-only and fenced; it re-measured §1 (5 programs, 8 pairs,
10 shared items) and the hand-off ledger (71, held), and read the gate's workflow and the verifier. Verdict: not
acceptable, 14 defects and 6 remarks, several made by round 1's answer. The heaviest: the gate is built from the base,
so a form cannot be judged in the push that brings its reader — every reader now lands first, in `M3.5.2.1`, and every
form a push later. The author verified it in `.github/workflows/trust-gate.yml` and `read_roots` before answering.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R2-D1 | defect | the gate is built from the base, whose `read_roots` fails on an unknown form or clause, so a form written in the push that brings its reader is unjudged (exit 2) | readers in `M3.5.2.1`, writing no form; each form in a later push (*How to apply*, `RP-H5`, `RP-H11`, `RP-H4`); verified in `trust-gate.yml` and `read_roots` |
| R2-D2 | defect | §3 makes a missing package a read failure, §6 `trust-baseline-stale`; `read_roots` reads text only | text faults read failures; a missing package or target `trust-baseline-stale`, from metadata, wherever the gate runs (§3) |
| R2-D3 | defect | building the comparison compiles `eadl-resolve`, whose two `include_str!` sites no form admits | stated (§3); admitted by `M3.5.3.1` or `M3.4`, whichever is first (`RP-H11`, `RP-H7`) |
| R2-D4 | defect | the derivation item has no identity; a second form for one pair would collide | `<a>+<b> shared-derivation <name>`, one item a form (§3) |
| R2-D5 | defect | "no result may name it" is not enforced: the verifier accepts a matching role text; how a derivation is recorded is unstated | recorded as the harness, role `derivation`; the verifier refuses a derivation's or no role's result (§3, `RP-H2`) |
| R2-D6 | defect | §6 misses the opening, case 5 and `M3.6.7`'s list, §3's build and edge sentences, two §6 rows, case 4 and §7, *How to apply*, and the generated-sources record | §6 lists each, the generated-sources record's three passages among them; a derivation is a program there |
| R2-D7 | defect | §5 has no property; no owner writes the forms of items arriving at later leaves; `M3.5.1.8` contradicts `RP-H9` | a property column; `RP-H11`, `RP-H12`, `RP-H13`; `M3.5.1.8` aligned |
| R2-D8 | defect | the build-configuration row's risk and controls say nothing | written (§5) |
| R2-D9 | defect | the comparison cannot catch a fault both carry; the review rounds are a procedure | the comparison named the coupling; independent controls listed, *pending* where owned; procedures named (§3) |
| R2-D10 | defect | `RP-H6` has the wrong trigger and no order against `M3.5.1.4` | "after both, the commit in which `archogen analyze` runs the sibling"; the order in `M2.21.3` |
| R2-D11 | defect | `RP-H8` amends `TI-H8`, owned by a leaf sealed by then | `TI-H14` alone, `M4.7`'s own (`RP-H8`) |
| R2-D12 | defect | five passages of the tree contradict round 1's answers | each amended |
| R2-D13 | defect | `(installed …)` has no refusals | listed with `(invokes …)`'s (§4) |
| R2-D14 | defect | other two-role tests stay invisible, and no one owns the report's list of what stays outside | an unnamed two-role test target reported from metadata; the list's lines `RP-H2`'s (§3, §4) |
| R2-R15 | remark | narrow `defharness` by roles, either order; three fixtures assert a generator–checker pair | worded so (§3); the fixtures rewritten (`RP-H2`) |
| R2-R16 | remark | the invocation's content misses the launcher, and its identity takes the inventory's order | the launcher's sha256 an aspect; the inventory's order (§4) |
| R2-R17 | remark | uncoded shape errors beside a coded `trust-invocation`; the build command lacks the parent's flags; edges from its own package | every text fault a read failure, as the harness's, `trust-invocation` retired; the parent's command; its own package's edges (§3, §4) |
| R2-R18 | remark | the parent's clarification overstates: "a root's" invocations, an exemption vacuous until `M3.4`, a narrow refusal | reworded |
| R2-R19 | remark | name the case-5 and channel fixtures | in `RP-H2` |
| R2-R20 | remark | `RP-H8` holds only when the package carries the install's own trust build | said so |

**Round 3**, `2026-10-23`, a new context at `551da268`, read-only and fenced; it re-measured §1 and the ledger (74,
held), ran a metadata census of every test's closure, and checked `origin/main`'s encoding. Verdict: not acceptable,
10 defects and 10 remarks; round 2's answers to D5, D6, D7, D10, D12, D14 and R19 not carried out whole. The rule
round 2 added for unnamed tests fired on today's tree and is withdrawn; the runner's forms must come from a run whose
base holds the judging gate's encoding. The author verified D2 (`lifecycle`) and D8 (`62099a39`) before answering.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R3-D1 | defect | "refuse a producer of no role of `trust::ROLES`" refuses the harness, whose role is `harness` | refused: a result whose role is no role of `ROLES`, or whose producer is a derivation's artifact; a reference-model result by its pair's harness accepted, each a fixture (§3, `RP-H2`) |
| R3-D2 | defect | the unnamed two-role test rule fires today on `rt-core`'s `lifecycle`, no form can name it, and it has no code or report part | withdrawn: a test produces no claim's result and no root compiles it, so it shares nothing between roots; named as outside, a missing form not found mechanically (§3); verified with `cargo metadata` |
| R3-D3 | defect | the parent's clarification states the reader in force before it exists | in force from `M3.5.2.1`'s commit, in the record's status, its §6 and both records' clarifications |
| R3-D4 | defect | §6 omits the preflight, the Cargo home reset, case 4 and `trust-inventory-stale`, an admission bullet, four generated-sources passages; the generated-sources record has no clarification; one passage misattributed | each listed; a dated clarification in the generated-sources record; the attribution corrected |
| R3-D5 | defect | `RP-H6`'s owner closes before its trigger; no leaf wires `archogen analyze` to the launcher; that invocation's form has no owner | `M2.21.5` filed, owning both (`RP-H6`) |
| R3-D6 | defect | three tree passages contradict the answers: `.6`'s own runner forms, `trust-invocation`, "ten hand-offs" | amended |
| R3-D7 | defect | *How to apply* covers added items only; a moved item's form is `trust-form-missing` too | added or moved, citing the parent's clarifications; `M3.4`'s in `RP-H7`; `RP-H12` |
| R3-D8 | defect | the next push's base gate lacks the structured-key repair, so its proposal is in an older encoding | the first run whose base gate holds the judging gate's encoding, its base identity recorded (*How to apply*); verified: `62099a39` not an ancestor of `origin/main` |
| R3-D9 | defect | the property column holds no §7.1 property | configuration validity or timing, per pair (§5) |
| R3-D10 | defect | §4 gives the install "exactly the invoked roots" today | today every executable root but the generator's; exactly the invoked from `M3.5.1.4` (§4) |
| R3-R11 | remark | the joint comparison does not fit "derivation" | `defderivation` names any generator–checker comparison, each form's class its own (§3) |
| R3-R12 | remark | after `M3.4` the item holds only the test's own sources | stated as outside (§3) |
| R3-R13 | remark | several controls are change control | named so (§3, §5) |
| R3-R14 | remark | the JSON control covers records, the risk names servers | strictness for records and messages alike; re-writing for records (§5) |
| R3-R15 | remark | `M3.5.3.1` bundles its form with runner forms, and the derivation's runner leaf is unfiled | split: `M3.5.3.2` filed (`RP-H14`) |
| R3-R16 | remark | `RP-H2` names no fixture | named |
| R3-R17 | remark | `(invokes …)` without `(installed …)`, two generator roots, a launcher that is no blob | each a read failure, a missing launcher `trust-undeclared-input` (§4) |
| R3-R18 | remark | classification follows an example's development edges too | §1 says so |
| R3-R19 | remark | `M3.5.1.3`'s plan contradicts R1-D15 | amended |
| R3-R20 | remark | `RP-H10` names no place for the controls | this record's §5 row, through its review |

**Round 4**, `2026-10-23`, a new context at `b788ec4e`, read-only and fenced; it re-measured §1, the ledger (75, held)
and `origin/main`'s encoding, and read the gate's judgement of forms and its report. Verdict: not acceptable, 10
defects and 10 remarks — the second round whose count did not fall. **The method changed** rather than another
answer of prose with prose: §6's enumeration, which every round found incomplete (R1-D19, R2-D6, R3-D4, R4-D10), is
replaced by a reading rule complete by construction; a missing comparison, found by hand this round, is found by
machine — a census of sources, run by the author, that `M3.5.2.1` makes a gate rule; and from round 5 the record is
read against `M3.5.2.1`'s code and fixtures, so a claim about the gate's behaviour is a test.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R4-D1 | defect | the exemption is not vacuous until `M3.4`: from `M3.5.3.1` the comparison compiles the checker's role package | the checker's from `M3.5.3.1`, the generator's from `M3.4` (§3, the parent's clarification reworded) |
| R4-D2 | defect | `eadl-resolve/tests/vocabulary.rs` is a second, unnamed comparison; the outside line's reason is false for it | a census by machine — comments and literals stripped, `use` groups whole — finds `production` and `vocabulary`; both named (`RP-H11`); the rule that finds a missing one mechanically (§3, `RP-H2`) |
| R4-D3 | defect | a transcription's edit moves no shared item and is reported nowhere | said so plainly: the comparison failing is what holds them together (§3) |
| R4-D4 | defect | the runner rule omits its evidence and that a proposal holds only while nothing after moves the item | ancestry against the base the report names; refused until a push holds the form and nothing after moves it (*How to apply*) |
| R4-D5 | defect | `M3.5.1.8` still takes the next push's proposal | the first run whose base holds `62099a39` (the leaf and `RP-H9`) |
| R4-D6 | defect | most new pairs have no property; `eadl-front` would carry the wrong one into the scheduling checker's pair | the property is the pair's, for every pair kind, "none rests on it" for the rest (§5) |
| R4-D7 | defect | the install sentences omit the front and are false against `install.rs` | the front and, as siblings, every other executable root today, exactly the invoked from `M3.5.1.4` (§4, `RP-H5`) |
| R4-D8 | defect | the generated-sources clarification pairs the derivation with its own pair | paired with no root, its pair taking its item |
| R4-D9 | defect | `RP-H13` covers items a root adds, not those it moves | "adds, or whose measured aspects it moves" (`RP-H13`, `RP-H6`) |
| R4-D10 | defect | §6 misses `GS-H10`, `TI-H8`, `TI-H23`, `GS-H3`, `GS-H11`; `GS-H10` has no owner for a derivation | the enumeration replaced by a reading rule complete by construction (§3, §6); `RP-H15` to `M4.8` |
| R4-R11 | remark | the launcher-blob rule is placed in no step; the path is the gate's alone | step 2; the path the one `NO-SUBPROCESS` admits, a test holding them equal (§4, `RP-H5`) |
| R4-R12 | remark | `RP-H2`'s fixtures omit several refusals and items | added |
| R4-R13 | remark | `M3.5.1.4` and `M3.5.6` are not ordered | the executable root first (`RP-H5`) |
| R4-R14 | remark | runner leaves unfiled; `M3.5`'s children omit `.2.1`, `.3.1`, `.3.2` | each hand-off files its own; the children listed |
| R4-R15 | remark | the repair is `M3.6.9.6.3.2.2.9.2`'s, not `M3.6.3.2.1.1`'s | corrected (§1) |
| R4-R16 | remark | the statement both records carry is already made | "from `b788ec4e`" (§6); the obligation dropped from `RP-H2` |
| R4-R17 | remark | the parent's clarification is narrower than §3 | reworded to any generator–checker comparison |
| R4-R18 | remark | only cross-form read failures are found once the whole file is read | split in the generated-sources clarification |
| R4-R19 | remark | `eadl-front` omits the third reader; own-property controls and the pinned toolchain unlabelled | the third reader added; each labelled (§5) |
| R4-R20 | remark | "the two siblings" in `M3.5.1`; whether a harness's target counts as another form's | each sibling's hand-off named; a harness's or a derivation's (§3) |

**Round 5**, `2026-10-23`, a new context at `a203e3ce`, read-only and fenced, the first read against the code: it ran
the readers' tests, re-measured the inventory (`5 program(s), 8 pair(s), 10 shared item(s)`) and the ledger (76, held),
and probed the census in a scratch copy of `xtask`. Verdict: not acceptable, 7 defects and 10 remarks. The readers
exist and their fixtures pass; the census of names missed ten comparisons the probe built, and three coded behaviours
departed from the records. The census is replaced by the build: what a test-like unit compiles, not what its text
names, decides whether it joins two roles, so no spelling hides one and every unit that does is named or classified by
review.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R5-D1 | defect | the census of names misses ten comparisons a probe built — a renamed dependency, `::g`, a nested module, `include!`, `cfg_attr`, a macro, a unit test, a doc test, a bench, a helper's re-export — and the report says nothing | read from the build: each test-like unit of a package whose build, its own development edges followed, compiles role packages of two roots a property rests on is a program target, named by a pair form or classified; the census of names removed (§3, `M3.5.2.2.2`) |
| R5-D2 | defect | a generator–scheduling-checker test is found and no form can name it; a unit linking both that compares nothing has no form | a derivation's pair is the generator and either checker; a unit that compares nothing is classified by `defprogram`, its reason the reviewed claim (`M3.5.2.2.2`) |
| R5-D3 | defect | a clause item that is not a name is dropped silently | every clause holds names alone, anything else a read failure (`M3.5.2.2.1`) |
| R5-D4 | defect | the front's faults within its own form are decided once the file is read, against the generated-sources clarification | decided at the form; the invoked names and two generator roots alone once the file is read (`M3.5.2.2.1`) |
| R5-D5 | defect | the census makes a test a program target and nothing amends the parent's definition, §6's rows, case 5 or `M3.6.7`'s list | the parent's clarification makes each such unit a §2 program target, so every passage written in that term reads it (`M3.5.2.2.2`) |
| R5-D6 | defect | §5's derivation row gives the joint comparison §3's class, which §3 leaves to its review | the row is the relation's two derivations (§5) |
| R5-D7 | defect | `M3.5.1.8`'s plan and verification still take the next push's proposal, whose base lacks `62099a39` | the first run whose base holds it, at the earliest the push after the next (`M3.5.1.8`) |
| R5-R8 | remark | stated behaviours with no test: the unnamed unit's remedy, the invocation through the gate, `(invokes <a derivation>)`, an empty `installed`, a derivation's provenance, reads and other roles, its package gone | a fixture each (`M3.5.2.2.3`) |
| R5-R9 | remark | case 5's outside edit touches a file no program reads | a file of the derivation's package outside its units (`M3.5.2.2.3`) |
| R5-R10 | remark | "`M3.5.2.1`'s commit" is six commits | the ancestry rule names `3f0a27e0`, where both new kinds' encodings stand; "in force" names the readers' leaves |
| R5-R11 | remark | the book's codes table and a stale "read, not yet built"; its outside list held to `UNSEEN` by nothing | corrected, a test holding the list equal (`M3.5.2.2.3`) |
| R5-R12 | remark | an error text holds 18 spaces | corrected, its test reading it whole (`M3.5.2.2.1`) |
| R5-R13 | remark | `ItemId`'s comment omits two kinds | added (`M3.5.2.2.3`) |
| R5-R14 | remark | `M3.5`'s status and design text are stale | corrected |
| R5-R15 | remark | the transcription line is unconditional, true only from `M3.4` | conditioned (`M3.5.2.2.3`) |
| R5-R16 | remark | `NO-SUBPROCESS` "admits" a file that does not exist | "is to admit" (§4) |
| R5-R17 | remark | the arming evidence lives in ignored `target/` | each leaf records its arms; a durable, re-runnable arming is mutation testing, deferred by the ruling of `2026-10-21` — `M7.6` holds it |

**Round 6**, `2026-10-23`, a new context at `0a2f9dd3`, read-only and fenced, read against `M3.5.2.2`'s code: it ran
sixteen filters of the readers' tests, all passing, re-measured the inventory and the ledger, and probed the census in a
scratch workspace against cargo's own builds. Verdict: not acceptable, 5 defects and 8 remarks. The readers do what the
records say; the census from the build counted the units the default build makes, while a build-dependency, a feature
or a platform reaches others, and said nothing of it. Measured before answering, in a scratch package: `cargo test
--lib`, `--test` and `--doc` build a target whose `test` or `doctest` flag is off, so a flag changes the default build
alone. The census now counts what some build can compile, an over-approximation it declares.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R6-D1 | defect | the census misses a role package reached through a build-dependency or an optional dependency, counts another platform's dependencies and targets the default build skips, and states none of it | it counts what some build can compile: every test-like target whatever its flags, over every feature, every platform and build edges, with the package's own development edges — declared; a build script's actions and packages outside the workspace's members stated outside (§3, `UNSEEN`, `M3.5.2.3.1`) |
| R6-D2 | defect | an edit inside a classified unit is reported nowhere, and nothing says so | stated (§3, `UNSEEN`, the book, `M3.5.2.3.1`); a digest of a classified target's own sources, for every classification, filed as `M3.6.10` |
| R6-D3 | defect | the remedy names `defderivation` for the reference model's pair | the inventory records the pairs a unit joins; the remedy names each pair's form (`M3.5.2.3.2`) |
| R6-D4 | defect | `RP-H7` has no order against the reader's push, `M3.5.3` or `M3.5.3.1` | after `M3.5.3.1` and a push carrying `M3.5.2.2`'s reader (`RP-H7`, `M3.4`) |
| R6-D5 | defect | the book gives every derivation the reference-derivation class | the relation's derivations (the book) |
| R6-R6 | remark | no fixture joins an executable's unit tests, nor a harness naming a joined test | both added (`M3.5.2.3.1`) |
| R6-R7 | remark | "keeps a test's crate root" is false for an executable's unit tests | joining gives no crate root a generator file's standing it lacks otherwise (`M3.5.2.3.1`) |
| R6-R8 | remark | the parent's edge rule is not amended for a joined unit | amended in its clarification (`M3.5.2.3.1`) |
| R6-R9 | remark | a classification and a pair form on one unit pass, the classification called "no program target" | a read failure (`M3.5.2.3.2`) |
| R6-R10 | remark | a comparison in a unit other than an integration test has no lawful form | stated: a comparison is an integration test, a pair form's; a classification says its unit compares nothing (§3) |
| R6-R11 | remark | the front's outside line is unconditional, all three stands `M3.5.1.4`'s | conditioned on it (`M3.5.2.3.2`) |
| R6-R12 | remark | `RP-H2`'s text describes the retired census of names | a note in *How to apply*: §3's census from the build supersedes it |
| R6-R13 | remark | no hand-off makes a later `M3.5` leaf name or classify the units it joins | `RP-H16`, to `M3.5` |

**Round 7**, `2026-10-23`, a new context at `2258c9fb`, read-only and fenced, read against `M3.5.2.3`'s code: ten filters
of the readers' tests, all passing; the inventory and ledger re-measured; five probes in a copy of the workspace.
Verdict: not acceptable, 2 defects and 6 remarks. Every round-6 answer landed in code; the census still read edges
alone, so a test compiling another role's files by `#[path]` or `include!` passed unseen, and the two clarifications
disagreed on generator files. Measured before answering: today's `#[path]` sites point into `eadl-front`'s test
helpers, no role's package, so a rule on what a literal path names changes nothing today.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R7-D1 | defect | a test compiling another role's sources by `#[path]` or `include!`, no dependency edge, is missed, against §3's "whatever its sources name" | a literal `#[path]` or `include!` resolving into a role package counts that package as compiled by its package's test-like units; a path no literal names stated outside (§3, `UNSEEN`, `M3.5.2.4.1`) |
| R7-D2 | defect | the parent's "every passage" makes a joined unit's crate root a generator file, against the generated-sources clarification and the code; that record's §5 restates the old edge rule | the parent's clause excepts that record's §5; its clarification states the edge rule; a fixture refuses such a crate root as a generator file (`M3.5.2.4.2`) |
| R7-R3 | remark | a non-member package's feature joins unseen; `UNSEEN` says "the members' build graph" | stated outside, the line saying what the census reads (`M3.5.2.4.1`) |
| R7-R4 | remark | a `dylib`'s doc tests are counted, which no build makes | a `lib`'s, `rlib`'s or `proc-macro`'s alone (`M3.5.2.4.1`) |
| R7-R5 | remark | "in force" and the orderings name `M3.5.2.2`; `M3.5`'s children omit `.2.3`; two dead branches; `RP-H16` out of order | each names `M3.5.2`'s readers, whole; the children listed; the branches dropped; `RP-H16` after `RP-H15` |
| R7-R6 | remark | `joins`, the remedy by pair and the read failure beside a pair form are in no text; the book's rows lag; a unit joining both pairs is told to take a form either refuses | stated; the rows current; such a unit told no pair form names it (`M3.5.2.4.2`) |
| R7-R7 | remark | the book's "Today and ahead" omits what the census reports today | `rt-core`'s three units and `M3.5.8` named |
| R7-R8 | remark | a test running another role's program as a process is unseen and unsaid | stated outside (`M3.5.2.4.1`) |

**Round 8**, `2026-10-23`, a new context at `b9a070d4`, read-only and fenced, read against `M3.5.2.4`'s code: 27 tests
passing; eight probes in a copy of the workspace, each confirmed by cargo's own dep-info. Verdict: not acceptable, 3
defects and 10 remarks (its verdict line said 11; its findings list ten) — the count rose from round 7's 2, as rounds 5
to 7 had each found the census missing another spelling of a file inclusion. **The method changed**, as after round 4:
the census reads what rustc compiled, one build of every test-like target and its dependency information, not what a
file's text names, so no spelling can hide a file a unit compiles. Measured before answering: a clean build of every
test-like target of this workspace, release, every feature, takes 28 seconds and writes 112 dependency files.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R8-D1 | defect | `include![…]`, an escaped literal, an untokenizable file and an aliased `include!` hide a comparison from the literal-path census | the census reads what each unit's build compiled, its dep-info, the text scan removed (§3, `M3.5.2.5.1`) |
| R8-D2 | defect | a path a dependency's file names, a target's own path outside its package and a chain of paths are missed, unsaid | the same: a unit counts every file its build and its dependencies' builds read (`M3.5.2.5.1`) |
| R8-D3 | defect | the missing-launcher refusal is dropped beside another step-2 refusal | every step-2 refusal reported together (`M3.5.2.5.2`) |
| R8-R4 | remark | `M3.5.8`, *How to apply*, `RP-H5`, `RP-H11` and `M3.5.3.1` still name a single reader leaf | each names `M3.5.2`'s readers, whole |
| R8-R5 | remark | a test joining one pair and compiling a third role's package is offered a pair form that would be refused | offered one only when it compiles no other role's package (`M3.5.2.5.2`) |
| R8-R6 | remark | literal paths count for every program target but not in the generator judgement, and are stated for tests alone | one computation of a program target's role packages, from what its build read, used by both (`M3.5.2.5.1`) |
| R8-R7 | remark | an uncompiled file's `include!` joins | read from the build, it does not (`M3.5.2.5.1`) |
| R8-R8 | remark | the book's census sentence reads as if one role's file by a path suffices | rewritten for the build's census (`M3.5.2.5.1`) |
| R8-R9 | remark | §3's outside list and §4's paragraph drift from `UNSEEN` | §4's conditioned on `M3.5.1.4`; §3's rewritten with the census (`M3.5.2.5.1`) |
| R8-R10 | remark | the book says infrastructure cannot carry a shared mistake, against §5's digest and toolchain rows and §4.4 | narrowed: it seldom shapes an answer, and its residual risk says when it can |
| R8-R11 | remark | `RP-H16` binds `M3.5`'s leaves alone; `M2.21`'s can join units too | `RP-H17`, to `M2.21` |
| R8-R12 | remark | a symbolic-link launcher and the spanning pair-form failures beside every `defgenerated` refusal are untested | a fixture each (`M3.5.2.5.2`) |
| R8-R13 | remark | §3's "but for two things" omits the form's own differences; `M3.5.8`'s obligation and today's census are in no hand-off or §1 | §3 says them; `RP-H18`, to `M3.5.8`; §1 states the measurement |
