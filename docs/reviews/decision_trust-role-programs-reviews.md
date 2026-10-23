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
