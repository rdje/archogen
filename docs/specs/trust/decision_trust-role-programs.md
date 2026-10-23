# The checkers' programs: the configuration checker's root, its shared derivation, invoked programs, what roles share

- **Type:** `decision`
- **Date:** `2026-10-23`
- **Status:** `active` — written `2026-10-23` by leaf `M3.5.2`; rounds 1 to 5 answered the same day; under review
  ([`decision_trust-role-programs-reviews.md`](../../reviews/decision_trust-role-programs-reviews.md)), from round 5 read
  against its readers' code and fixtures (`M3.5.2.1`, `M3.5.2.2`), its rules in force as each lands.
- **External sources:** none beyond the parent's: `ROADMAP.md` §4.4 (its "externally invoked tools", and the classes
  of a shared item, "reference derivation" among them), §7.1's properties and §14.4's cases.
- **Owner / source:** leaf `M3.5.2` (`docs/tasks/M3.md`), as `M3.5`'s and `M3.5.1`'s reviewed designs assign it, under
  the director's ruling of `2026-10-22` (findings §16 (a): each checker an executable of its own, the human command
  line alone running it). It amends [`decision_trust-inventory.md`](decision_trust-inventory.md), "the parent", and
  [`decision_trust-generated-sources.md`](decision_trust-generated-sources.md), whose terms it uses unchanged.

## The fact / decision

### 1. What the parent cannot yet hold, measured

- **One derivation, invisible.** `M3.5` moves the substitutability relation's executable model, byte for byte, into
  `crates/archogen-plan-check` (`M3.5.3`); the tests that hold the generator's production code equal to it stay in
  `crates/eadl-resolve/tests`, the checker's package a development dependency. A census of their sources, comments and
  literals stripped and each `use` group read whole, finds two naming both the model and production modules:
  `production.rs` (the relation, over `common/universe.rs`) and `vocabulary.rs` (the typed table, entry by entry);
  `checker.rs`, `corpus.rs` and `common/universe.rs` name the model alone. Each pair is one derivation held equal,
  sharing transcribed code — the production relation follows the model line for line in places (`side_items`,
  `side_name_lists`, `horizon`; `M3.3`). The parent builds no test target but the harness's (`PROGRAM_KINDS` names no
  test), and follows development edges for the harness and an example's classification alone (parent §2, §3), so
  neither comparison is seen.
- **A harness for any pair.** The parent's prose gives a harness to a reference model's pair; the instrument reads a
  `defharness` naming any two roots (`check_pairs`), pairs it with every other root, and the verifier accepts a
  result whose role text is a recorded program's (`trust_verify.rs`).
- **A program that runs another.** The front, `archogen`, will run `archogen-plan-check` and `archogen-analyze` as
  processes (`ROADMAP.md` §10.2; `M3.5.1.4`) and read the install manifest (`archogen-install/1`), no blob of the
  commit. The parent sees what a root compiles and the data its form hands it (§3). §4.4's "externally invoked tools"
  are, in context, a build's; reading them as the programs a root runs is this record's extension, as `M3.5.1`'s.
- **What the roles share.** `cargo xtask trust-inventory --commit 0a5a4b12`, `aarch64-apple-darwin`: exit 0, `5
  program(s), 8 pair(s), 10 shared item(s)`, the tenth `generator+scheduling-checker package crates/archogen-digest`,
  reached through `archogen-outcome` (`archogen-api`'s since `5bc5627d`) and through `archogen-evidence`.
- **The gate is built from the base** (parent `TI-H6`; `.github/workflows/trust-gate.yml`): on a push, the commit
  `main` pointed at before it, which the report names. Its `read_roots` fails on a form or clause it does not know
  (exit 2), and its item encoding is the base's: `origin/main` is `8cdac117`, which does not hold `62099a39`, the
  structured-key repair (`M3.6.9.6.3.2.2.9.2`; its runner follow-up `M3.6.3.2.1.1`).

### 2. The configuration checker's root

From `M3.5.3` the role is rooted at its library, as the parent §2 roots a role no executable runs yet: `(defroot
configuration-checker (role configuration-checker) (package "crates/archogen-plan-check") (target lib) (role-packages
"crates/archogen-plan-check"))`, replacing `(defrole configuration-checker …)`; from `M3.5.6` its target is `(target bin
archogen-plan-check)`. The kind modules, `docs/semantics/kinds/core.eadl` and `os-rt.eadl`, are compiled in by
`include_str!` from `M3.5.4`, when check (2) re-checks the description — the model moved at `M3.5.3` reads none — as
`archogen-api` compiles them: shared files of the parent §4's second kind, each site admitted by a `defadmit` of its
own. They are not `(data …)`; any other fixed data it comes to read at run time is declared there by path. The
description and the assignment record it checks are claim subjects, never dependencies (`TI-H19`).

### 3. The shared derivation

**A derivation is a comparison harness for the generator and a checker, but for two things:** it is
paired with no root — its pair takes its item, as a harness's pair does — and no result may name it as its producer.
Wherever the parent, the generated-sources record or their ledgers name the comparison harness as a program — what it
compiles, reads and reaches; how it is built, recorded, scanned, admitted, exempted, judged and reported; its item,
its provenance and its forms — a derivation is read the same way, but for those two things. That is complete by
construction: no passage needs listing, and a passage added later reads so too.

**The form.** `(defderivation <name> (pair generator <checker>) (package "<dir>") (test <target>) (reason "…"))`
names one test target comparing a reading of the generator's with a checker's; each form's
class is its own review's. Its text read wrong fails `read_roots` (exit 2), as a harness's does: a pair naming no root,
one root twice, or other than the generator's root and a checker's; a target another form names, a
harness's or a derivation's. A package or target the commit lacks is `trust-baseline-stale`, from metadata, wherever
the gate runs. **`defharness` is narrowed:** its pair is one root of role `reference-model` and one of role
`implementation`, in either order; today's `differential` is.

**Its record and item.** Recorded in the inventory's programs as the harness is, its role `derivation`, its artifact
the test binary; its item kind `shared-derivation`, identity `<a>+<b> shared-derivation <name>`, the pair in the
inventory's order, one item a form, aspects the harness item's. Its exemption from `trust-shared-program` is its pair's
role packages: the checker's from `M3.5.3.1`, when the comparison compiles `archogen-plan-check`, and the generator's
from `M3.4`, when `crates/eadl-resolve` becomes one. Building it compiles `eadl-resolve`'s library, whose two
`include_str!` sites (`src/vocabulary.rs`: `kinds/deffact.eadl`, `vocabulary/vocabulary.eadl`) enter the scope and the
item, each admitted by a `defadmit`. The verifier refuses a result whose role is not one of `trust::ROLES`, and one
whose producer is a derivation's artifact; a reference model's result produced by its pair's harness stays accepted.

**Found by the build, not by names.** Each test-like unit — an integration test, a bench, a library's or an
executable's unit tests, a library's doc tests — of a package whose build, its own development edges followed, compiles
role packages of the two roles of a pair a property rests on (§5: the generator and either checker, the reference model
and the implementation) is a program target of the parent's §2, whatever its sources name: an integration test named by
a pair form if it compares them, any unit else classified by `defprogram`, `(target test|bench|lib-test|bin-test|doctest
<name>)`, its reason the claim that it compares nothing they hold equal. Unnamed, it is `trust-unclassified-program`,
and `trust-form-missing` on the baseline's host. §1's census of names found the two comparisons `RP-H11` names.

**Outside it, stated in the report:** its run-time reads of the kind modules (`std::fs`); and the transcriptions
themselves — the model's sources are in the checker's package and the production relation's, from `M3.4`, in the
generator's, so an edit to either moves no shared item and is reported nowhere; the comparison failing is what holds
them together; and whether a classified unit compares what two roles hold equal, which its review decides.

**Its class** (§5), §4.4's **reference derivation.** The comparison holding the two equal is the coupling itself,
not a control: a fault both carry passes it. Controls independent of the transcription: the model's exhaustive
checker for the properties the relation record states, and the falsification corpus written from the record (both
built); F12's hand-written corruptions and accepted plans (`M3.5.4`, `M3.5.5`, *pending*). The frozen verdicts, once
`M3.4` wires the relation into `archogen check`, are change control; the record's 31 review rounds, and amending the
checker's model before the generator's relation, are procedures. The comparison of the checker's fenced joint reading
with `joint::System::read` (`M3.5.5`) compares two readings written apart: a form of its own, its class its review's.

### 4. Invoked programs and installed data

**`(invokes <root> …)` and `(installed "archogen-install.json")`**, together, on the front's form alone — the one
root of role `generator`, whose target is an executable. Read wrong, either fails `read_roots` (exit 2): either clause
on another root's form, or on a front not an executable, or while two roots have role `generator`; either without the
other; an empty clause or a root named twice; an invoked name that is no root, the generator's, a root whose target is
not an executable, the harness or a derivation; `(installed …)` holding more than one name, or a name other than
`archogen-install.json` (`archogen_install::FILE`). An invocation is a shared item of the two roots: kind
`invocation`, identity `<a>+<b> invocation`, the pair in the inventory's order; its content the invoked root's name,
role and target and the sha256 of the launcher, `crates/archogen-cli/src/sibling.rs`, the one file `NO-SUBPROCESS`
is to admit; a front with `(invokes …)` whose commit holds no such blob is `trust-undeclared-input`, decided in step 2 of
the generated-sources record's order, when the tree is read. A process is not a compilation: an invocation is never
`trust-shared-program`. The installed name is no blob and no `(data …)`, so the blob rule of `trust-undeclared-input`
does not reach it. Today the install places the generator's executable as the front and every other executable root
as a sibling (`M3.5.1.3`); from `M3.5.1.4` its siblings are exactly the roots the front's form invokes. The package
records the manifest beside the artifacts it describes, never as a handed dependency (`M4.7`).

**What stays outside, stated in the report:** that the front runs only, or all, the programs its form invokes.
`NO-SUBPROCESS` admitting `sibling.rs` alone, declared from the front's `main.rs` (`M3.5.1.4`), the launcher running
only what the manifest names, and the install placing only invoked roots stand there.

### 5. What the roles share

Proposed for each form's review (parent §5): one class, one §7.1 property, the residual risk and the controls;
*pending* names the owner; a package's own tests, change control and procedures are named as such. **The property is
the pair's:** configuration validity for the generator and the configuration checker; timing for the generator and the
scheduling checker; runtime functional behavior for the reference model and the implementation; for any other pair,
none rests on its independence, and its form says so.

| Item | Pairs | Class | Residual risk | Controls |
| --- | --- | --- | --- | --- |
| `crates/eadl-front` | generator, configuration checker | interpretation/normalization | a misreading both make | the third reader (`scripts/third_opinion.sh`); its own conformance suite; the frozen verdicts, change control |
| `crates/eadl-model` | generator, configuration checker | semantic/analysis logic | a wrong quantity, comparison or pass both apply | the profile's classified rules; its own tests; the frozen verdicts, change control |
| the kind modules, two files | generator, configuration checker | authoritative data | a wrong definition both read | `LANGUAGE-FREEZE` and the migration notes, change control |
| `crates/archogen-digest` | every pair it joins | infrastructure | a wrong digest in every role's records | three oracles independent of it: derived constants, published examples, the system tool's boundaries |
| `crates/archogen-outcome` | every pair it joins | interpretation/normalization | a refusal read as an acceptance for every role | its own tests — one text per record, its consistency rules, the damage sweep; the exit code's second channel *pending* `M3.5.1.4` |
| `crates/archogen-json` | every pair it joins | interpretation/normalization | a parsing fault common to every record and server | its own tests — RFC 8259 strictness, a member named twice refused; the records' readers re-write what they read |
| `crates/archogen-transport` | every pair it joins | interpretation/normalization | a framing fault common to every server | *pending* `M3.5.1.6` |
| the relation's derivations, `production` and `vocabulary` | generator, configuration checker | reference derivation | as §3 | as §3 |
| an invocation | the front and each invoked root | interpretation/normalization | the front reporting a refused or unchecked result as checked | *pending* `M3.5.1.4`: the exit code agreeing with the record, the subject's digests, the program's sha256 |
| the build configuration | every new pair | infrastructure | a toolchain, profile or flag fault common to every role | the pinned toolchain (`SOURCE-LEDGER`) and the item compared on the baseline's host, change control |

`crates/archogen-install` is the front's alone; `xtask` is no root. `eadl-front` and `eadl-model` join the scheduling
checker's pairs when it reads descriptions (`M2.7.5`), with that pair's property.

### 6. What else reads with it

Beyond §3's reading rule: a new kind of item, the invocation, is read by the parent's §4–§6 as every shared item is;
the parent's §2 gains this record's forms and clauses and its sentence on `archogen analyze` and `archogen resolve` is
answered; case 4, `trust-inventory-stale` and §7's package contents gain the install manifest as `M4.7` records it;
`TI-H19` is amended in the parent's ledger in this change and `TI-H18` stands. Ledger rows owned elsewhere that the
reading rule extends — `GS-H10`'s acceptance of live `defgenerated` forms among them — are quoted below for their
leaves. Both records state, from `b788ec4e`, that these read with it as its readers land (`M3.5.2.1`, `M3.5.2.2`).

## Why

- **Disclosed, not hidden.** A derivation two roles share is the loss of independence §4.4 exists to surface; a form
  of its own puts it before a reviewer with its class, where today the gate sees nothing of it.
- **One way to write a comparison.** A harness and a derivation differ in what they may produce and whom they are
  paired with; a comparison writable as either would take whichever its author found convenient.
- **A process is an input.** The program a root runs is another root's artifact, and the invocation names it.

## How to apply

**A reader before its form.** The gate is built from the base (§1), so a form's reader lands first (`M3.5.2.1`),
reaches `main` through a cadence push, and only a commit of a later push writes the form. **A form from the runner.**
The forms of the shared items a commit adds, or whose measured aspects it moves, are the proposal of a runner run whose
base holds the encoding the judging gate uses — today `62099a39` and, for the new kinds, `3f0a27e0`, decided
by ancestry against the base the report names — committed in a leaf of their own that waits on the runner, writing §5's
row and recording that base. A proposal holds for the item as the run measured it, so the gate on the baseline's host
refuses the item until a push holds the form and nothing after it moves the item; each push's report says which.

<!-- machine-read: handoffs -->
| Id | Leaf | Hand-off |
| --- | --- | --- |
| `RP-H1` | `M3.5.3` | The configuration checker's library is its root in `trust/roots.eadl` from the commit that creates `crates/archogen-plan-check`, replacing `(defrole configuration-checker …)`. |
| `RP-H2` | `M3.5.2.1` | The gate reads `defderivation`, the narrowed `defharness`, `(invokes …)` and `(installed …)` as `decision_trust-role-programs.md` §3 and §4 state, reads a derivation as the harness but for its two differences, records and reports both new kinds of item, reports a test naming crates of the generator's and a checker's role packages that no form names, and refuses in `trust-verify` a result whose role is no role of `trust::ROLES` or whose producer is a derivation's artifact; its report's list of what stays outside gains §3's and §4's lines; it is tested on fixtures — each read failure, case 5 inside and outside a derivation's units, a derivation compiling a role package outside its pair, a derivation's target gone, a `#[no_mangle]` and a procedural macro on a derivation's development edge, an unnamed comparison reported, a missing launcher, an invocation moved by a launcher edit, a harness-produced reference-model result accepted, a derivation's and a no-role result refused — the existing pairs of a generator and a checker rewritten; the book's trust chapter says so; and it writes none of these forms. |
| `RP-H3` | `M3.5.4` | Each `include_str!` site of `crates/archogen-plan-check` is admitted in `trust/roots.eadl` by a `defadmit` of its own. |
| `RP-H4` | `M3.5.5` | The comparison of the checker's fenced joint reading with `joint::System::read` is a `defderivation` form of its own, written after its reader reaches `main`, its class written by its review. |
| `RP-H5` | `M3.5.1.4` | The generator's form gains `(invokes configuration-checker)` and `(installed "archogen-install.json")` with the launcher, after `M3.5.6` roots the checker at its executable and in a push after the one that brings `M3.5.2.1`'s reader to `main`, a test holding the installed name to `archogen_install::FILE` and the launcher's path to the file `NO-SUBPROCESS` admits, and `cargo xtask install` places the front and, as siblings, exactly the roots its form invokes. |
| `RP-H6` | `M2.21.5` | The generator's form invokes `scheduling-checker` in the commit in which `archogen analyze` runs `archogen-analyze` through the front's launcher, after `M2.21.3` and `M3.5.1.4`, and the forms of the items that commit adds or moves come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, in a leaf of their own. |
| `RP-H7` | `M3.4` | `crates/eadl-resolve` joins the generator's role packages in `trust/roots.eadl` in the commit that makes the generator compile it, its two `include_str!` sites admitted unless `M3.5.3.1` has admitted them, each of its test-like units no `defderivation` names classified by a `defprogram`, and the forms of the items its new edges add or move come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, in a leaf of their own. |
| `RP-H8` | `M4.7` | The package records the install manifest of its own trust build beside the artifacts it describes, never as a handed dependency, and `cargo xtask trust-verify` refuses one whose digests are not the inventory's, `TI-H14` amended to say so. |
| `RP-H9` | `M3.5.1.8` | The form of `generator+scheduling-checker package crates/archogen-digest` comes from the runner as `decision_trust-role-programs.md`'s *How to apply* says — the first run whose base holds `62099a39` — writing its §5's class, property, residual risk and controls. |
| `RP-H10` | `M3.5.1.6` | `crates/archogen-transport`'s controls are written into `decision_trust-role-programs.md` §5's row, through that record's review, before a checker compiles it. |
| `RP-H11` | `M3.5.3.1` | `trust/roots.eadl` holds a `defderivation` form for each comparison §1 names — `production` and `vocabulary` in `crates/eadl-resolve` — after `M3.5.2.1`'s reader reaches `main`, `crates/eadl-resolve`'s two `include_str!` sites admitted unless `M3.4` has admitted them. |
| `RP-H12` | `M3.5` | Each leaf of `M3.5` that adds a shared item, or moves one's measured aspects, has its forms come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, in a leaf of their own. |
| `RP-H13` | `M2.21.3` | The forms of the shared items the scheduling checker's executable root adds, or whose measured aspects it moves, come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, in a leaf of their own. |
| `RP-H14` | `M3.5.3.2` | The forms of the shared items `M3.5.3`'s root and `M3.5.3.1`'s derivations add come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, each writing its §5's row. |
| `RP-H15` | `M4.8` | `GS-H10`'s acceptance of the live `defgenerated` forms a root or the comparison harness reaches takes those a derivation reaches, as `decision_trust-role-programs.md` §3 reads a derivation. |

## Review

| Round | Date | Defects | Remarks | Verdict |
| --- | --- | --- | --- | --- |
| 1 | `2026-10-23` | 23 | 10 | not acceptable; every finding answered |
| 2 | `2026-10-23` | 14 | 6 | not acceptable; every finding answered |
| 3 | `2026-10-23` | 10 | 10 | not acceptable; every finding answered |
| 4 | `2026-10-23` | 10 | 10 | not acceptable; every finding answered, and the method changed: §6's enumeration replaced by a reading rule complete by construction, a missing comparison found by machine, and from round 5 the record read against `M3.5.2.1`'s code and fixtures |
| 5 | `2026-10-23` | 7 | 10 | not acceptable, read against the code; every finding answered, the census read from the build (`M3.5.2.2`) |
