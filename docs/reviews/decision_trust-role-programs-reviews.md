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
