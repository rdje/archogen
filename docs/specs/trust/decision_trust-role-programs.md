# The checkers' programs: the configuration checker's root, its shared derivation, invoked programs, what roles share

- **Type:** `decision`
- **Date:** `2026-10-23`
- **Status:** `active` — written `2026-10-23` by leaf `M3.5.2`; rounds 1 to 3 answered the same day; under review
  ([`decision_trust-role-programs-reviews.md`](../../reviews/decision_trust-role-programs-reviews.md)). Its rules are in
  force from `M3.5.2.1`'s commit, which builds the reader.
- **External sources:** none beyond the parent's: `ROADMAP.md` §4.4 (its "externally invoked tools", and the classes
  of a shared item, "reference derivation" among them), §7.1's properties and §14.4's cases.
- **Owner / source:** leaf `M3.5.2` (`docs/tasks/M3.md`), as `M3.5`'s and `M3.5.1`'s reviewed designs assign it, under
  the director's ruling of `2026-10-22` (findings §16 (a): each checker an executable of its own, the human command
  line alone running it). It amends [`decision_trust-inventory.md`](decision_trust-inventory.md), "the parent", and
  [`decision_trust-generated-sources.md`](decision_trust-generated-sources.md), whose terms it uses unchanged.

## The fact / decision

### 1. What the parent cannot yet hold, measured

- **One derivation, invisible.** `M3.5` moves the substitutability relation's executable model, byte for byte, into
  `crates/archogen-plan-check` (`M3.5.3`), and keeps the comparison holding the generator's production relation equal
  to it in `crates/eadl-resolve/tests/production.rs`, over `tests/common/universe.rs`, the checker's package a
  development dependency. The two are one derivation, held equal, and share transcribed code, not only a reading of
  one record: the production relation follows the model line for line in places (`side_items`, `side_name_lists`,
  `horizon`; `M3.3`). The parent builds no test target but the harness's (`PROGRAM_KINDS` names no test), and follows
  development edges for the harness and an example's classification alone (parent §2, §3), so that comparison is
  **invisible** — neither reported nor refused.
- **A harness for any pair.** The parent's prose gives a harness to a reference model's pair; the instrument reads a
  `defharness` naming any two roots (`check_pairs`), pairs it with every other root, and the verifier accepts a
  result whose role text is a recorded program's (`trust_verify.rs`).
- **A program that runs another.** The front, `archogen`, will run `archogen-plan-check` and `archogen-analyze` as
  processes (`ROADMAP.md` §10.2; `M3.5.1.4`) and read the install manifest (`archogen-install/1`), no blob of the
  commit. The parent sees what a root compiles and the data its form hands it (§3). §4.4's "externally invoked tools"
  are, in context, a build's; reading them as the programs a root runs is this record's extension, as `M3.5.1`'s.
- **What the roles share.** `cargo xtask trust-inventory --commit 0a5a4b12`, `aarch64-apple-darwin`: exit 0, `5
  program(s), 8 pair(s), 10 shared item(s)`, the tenth `generator+scheduling-checker package crates/archogen-digest`,
  reached through `archogen-outcome` (`archogen-api`'s since `5bc5627d`, `M3.5.1.1`) and through `archogen-evidence`.
  The generator compiles `archogen-json` directly since `e9245a45` (`M3.5.1.7`).
- **The gate is built from the base** (parent `TI-H6`; `.github/workflows/trust-gate.yml`): on a push, the commit
  `main` pointed at before it. Its `read_roots` fails on a form or clause it does not know (exit 2), and its item
  encoding is the base's: `origin/main` is `8cdac117`, which does not hold the structured-key repair `62099a39`
  (`M3.6.3.2.1.1`), so the next push's proposal is in the old encoding.

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

**The form.** `(defderivation <name> (pair generator configuration-checker) (package "<dir>") (test <target>) (reason
"…"))` names one test target comparing a reading of the generator's with the configuration checker's; each form's
class is its own review's. Its text read wrong fails `read_roots` (exit 2), as a harness's does (`check_pairs`): a pair
naming no root, one root twice, or other than the generator's root and the configuration checker's; a target two forms
name. A package or target the commit lacks is `trust-baseline-stale`, from metadata, wherever the gate runs.
**`defharness` is narrowed:** its pair is one root of role `reference-model` and one of role `implementation`, in
either order; today's `differential` is.

**Its build, record and item.** Built as the harness — `cargo test --release --no-run -p <package> --test <target>`,
the parent's flags, in its own target directory, the development edges of its own package followed — and recorded in
the inventory's programs as the harness is, its role `derivation`, its artifact the test binary. The units it
compiles beside its pair's two builds are a shared item of its pair and of no other root: kind `shared-derivation`,
identity `<a>+<b> shared-derivation <name>`, the pair in the inventory's order, one item a form, aspects the harness
item's — content, configuration, provenance. It is a **program** in the generated-sources record's sense, its pair's
two exempt there as the harness's are. Its exemption from `trust-shared-program` is its pair's role packages alone.
Until `M3.4` makes `crates/eadl-resolve` the generator's, the exemption is vacuous and the item reported all the same.
Building it compiles `eadl-resolve`'s library, whose two `include_str!` sites (`src/vocabulary.rs`:
`kinds/deffact.eadl`, `vocabulary/vocabulary.eadl`) enter the default-deny scope and the item's content, each admitted
by a `defadmit`. The verifier refuses a result whose role is not one of `trust::ROLES`, and one whose producer is a
derivation's artifact; a reference model's result produced by its pair's harness stays accepted (`TI-H8`).

**Outside it, stated in the report:** its run-time reads of the kind modules (`std::fs`); from `M3.4`, the
transcriptions themselves, `eadl-resolve`'s units then being the generator's build, so an edit to either moves the
generator's and the checker's items, not this one; and every test the parent builds no form for — `rt-core`'s
`lifecycle`, `eadl-resolve`'s `checker` and `corpus` among them — since a test produces no claim's result and no
root compiles it, so it puts no sharing between roots; a comparison a claim rests on is named by a form, and a missing
one is not found mechanically.

**Its class** (§5), §4.4's **reference derivation.** The comparison holding the two equal is the coupling itself,
not a control: a fault both carry passes it. Controls independent of the transcription: the model's exhaustive
checker for the properties the relation record states, and the falsification corpus written from the record (both
built); F12's hand-written corruptions and accepted plans (`M3.5.4`, `M3.5.5`, *pending*). The frozen verdicts, once
`M3.4` wires the relation into `archogen check`, are change control. The record's 31 review rounds, and amending the
checker's model before the generator's relation, are procedures. The comparison of the checker's fenced joint reading
with `joint::System::read` (`M3.5.5`) compares two readings written apart: a form of its own, its class its review's.

### 4. Invoked programs and installed data

**`(invokes <root> …)` and `(installed "archogen-install.json")`**, together, on the front's form alone — the one
root of role `generator`, whose target is an executable. Each read wrong fails `read_roots` (exit 2): either clause on
another root's form, or on a front not an executable, or while two roots have role `generator`; either without the
other; an empty clause or a root named twice; an invoked name that is no root, the generator's, a root whose target is
not an executable, the harness or a derivation; `(installed …)` holding more than one name, or a name other than
`archogen-install.json` (`archogen_install::FILE`). An invocation is a shared item of the two roots: kind
`invocation`, identity `<a>+<b> invocation`, the pair in the inventory's order; its content the invoked root's name,
role and target and the sha256 of the launcher, the commit's blob `crates/archogen-cli/src/sibling.rs`, whose
acceptance contract carries its risk — no such blob is `trust-undeclared-input` — so a new or changed invocation, or
launcher, is reported. A process is not a compilation: an invocation is never `trust-shared-program`. The installed
name is no blob and no `(data …)`: the blob rule of `trust-undeclared-input` does not reach it, and the inventory
hashes nothing for it. Today the install places every executable root but the generator's and derives its digests
from the inventory (`M3.5.1.3`); from `M3.5.1.4` it places exactly the invoked roots. The package records the manifest
beside the artifacts it describes, never as a handed dependency (`M4.7`).

**What stays outside, stated in the report:** that the front runs only, or all, the programs its form invokes.
`NO-SUBPROCESS` admitting `sibling.rs` alone, declared from the front's `main.rs` (`M3.5.1.4`), the launcher running
only what the manifest names, and the install placing only invoked roots stand there.

### 5. What the roles share

Proposed for each form's review (parent §5): one class, the §7.1 property resting on the pair's independence —
configuration validity for the generator and the configuration checker, timing for the generator and the scheduling
checker — its residual risk and controls; *pending* names the owner; change control and procedures are named as such.

| Item | Pairs | Class | Property | Residual risk | Controls |
| --- | --- | --- | --- | --- | --- |
| `crates/eadl-front` | generator, configuration checker | interpretation/normalization | configuration validity | a misreading both make | the conformance suite; the frozen verdicts, change control |
| `crates/eadl-model` | generator, configuration checker | semantic/analysis logic | configuration validity | a wrong quantity, comparison or pass both apply | its tests; the profile's classified rules; the frozen verdicts, change control |
| the kind modules, two files | generator, configuration checker | authoritative data | configuration validity | a wrong definition both read | `LANGUAGE-FREEZE` and the migration notes, change control |
| `crates/archogen-digest` | every pair it joins | infrastructure | the pair's | a wrong digest in every role's records | three oracles: derived constants, published examples, the system tool's boundaries |
| `crates/archogen-outcome` | every pair it joins | interpretation/normalization | the pair's | a refusal read as an acceptance for every role | one text per record, its consistency rules, the damage sweep; the exit code's second channel *pending* `M3.5.1.4` |
| `crates/archogen-json` | every pair it joins | interpretation/normalization | the pair's | a parsing fault common to every record and server | RFC 8259 strictness, a member named twice refused, for records and messages alike; the records' readers re-write what they read |
| `crates/archogen-transport` | every pair it joins | interpretation/normalization | the pair's | a framing fault common to every server | *pending* `M3.5.1.6` |
| the shared derivation | generator, configuration checker | reference derivation | configuration validity | as §3 | as §3 |
| an invocation | the front and each invoked root | interpretation/normalization | the pair's | the front reporting a refused or unchecked result as checked | *pending* `M3.5.1.4`: the exit code agreeing with the record, the subject's digests, the program's sha256 |
| the build configuration | every new pair | infrastructure | the pair's | a toolchain, profile or flag fault common to every role | the pinned toolchain (`SOURCE-LEDGER`); the item compared on the baseline's host, change control |

`crates/archogen-install` is the front's alone; `xtask` is no root. `eadl-front` and `eadl-model` join the scheduling
checker's pairs when it reads descriptions (`M2.7.5`); each new pair's rows are this table's.

### 6. What the parent, and the generated-sources record, read with it

Each passage below reads with this record from `M3.5.2.1`'s commit, which states so in both records.

- **The parent's opening:** "a change outside every root and every program target" reads "… and every derivation's
  units"; **the change part, the standing list and `TI-H2`** take each `defderivation` form and invocation; **case 5**
  and `M3.6.7`'s list of what the gate reads beyond the roots take a derivation's units as the harness's; **`TI-H9`**'s
  sets likewise.
- **§2:** the checker's root (§2 above); `defderivation`; `defharness` narrowed; the front's two clauses; the
  exemption extended to a derivation's pair; the sentence on `archogen analyze` and `archogen resolve` answered — each
  checker an executable the front invokes.
- **§3:** the manifest preflight "before root/harness … compilation" and the Cargo home's reset "before … every
  root/harness build" take a derivation's; "the comparison harness a pair's form names is built" and the build
  configuration's "overrides for a package some root or the harness compiles" take a derivation; "development edges
  not followed … but the harness's own" and the artifact bullet take a derivation's; the default-deny scope and
  admissions cover its closure; what is recorded per root gains the front's invocations and installed name; what
  remains outside gains §3's and §4's lines.
- **§4, §5:** the two kinds of item, their identities and aspects, and their forms' digests.
- **§6:** the new read failures (§3, §4); `trust-new-shared`, `trust-shared-changed` and `trust-form-missing` take both
  kinds; `trust-undeclared-input`'s "a root or the harness compiles" takes a derivation, its blob rule not the
  installed name, and it takes a missing launcher; `trust-shared-program`'s exemption, and the clarification of
  `2026-10-10`'s "for the harness, than its pair's two", take a derivation's pair; `trust-baseline-stale` a derivation
  whose package or target is gone; **case 4 and `trust-inventory-stale`** the verifier's refusal of a derivation's or
  no role's result, and the install manifest as `M4.7` records it; §7's package contents the manifest.
- **How to apply:** "a comparison harness is named in its pair's form" gains a derivation; "a root's or the harness's
  `.rs` files" gains a derivation's; **`TI-H24`** its fixtures.
- **The generated-sources record:** §1's "a program is a root or the comparison harness", and "for the harness, both
  what it reads and what it reaches count only through the units it compiles beside its pair's two builds", gain a
  derivation; §2's departures found once the whole file is read gain §3's and §4's; §5's pairing ("the harness with
  every root but its pair's two"), its "for the harness's own pair, the harness's provenance is an aspect", and its
  "other roles … than its pair's two" each take a derivation, paired with its pair alone; §7's case-5 list its units.
- **`TI-H19`** is amended in the parent's ledger in this change; `TI-H18` stands.

## Why

- **Disclosed, not hidden.** A derivation two roles share is the loss of independence §4.4 exists to surface; a form
  of its own puts it before a reviewer with its class, where today the gate sees nothing of it.
- **One way to write a comparison.** A harness and a derivation differ in what they may produce and whom they are
  paired with; a comparison writable as either would take whichever its author found convenient.
- **A process is an input.** The program a root runs is another root's artifact, and the invocation names it.

## How to apply

**Two pushes for every new form.** The gate is built from the base (§1), so a form's reader lands first (`M3.5.2.1`),
reaches `main` through a cadence push, and only a commit of a later push writes the form. **A form from the runner, in
the judging gate's encoding.** The forms of the shared items a commit adds, or whose measured aspects it moves — the
parent's clarifications of `2026-10-06` and `2026-10-10`: a form proposing other digests than the commit measures is
`trust-form-missing` on the baseline's host — come from the proposal of the first run whose base gate holds the
encoding the judging gate uses (the structured-key repair, `M3.6.3.2.1.1`; `M3.5.2.1` for the new kinds), its base
identity recorded, committed in a leaf of its own that waits on the runner and writes §5's row. Until then the gate on
the baseline's host refuses the item, which the push's report states.

<!-- machine-read: handoffs -->
| Id | Leaf | Hand-off |
| --- | --- | --- |
| `RP-H1` | `M3.5.3` | The configuration checker's library is its root in `trust/roots.eadl` from the commit that creates `crates/archogen-plan-check`, replacing `(defrole configuration-checker …)`. |
| `RP-H2` | `M3.5.2.1` | The gate reads `defderivation`, the narrowed `defharness`, `(invokes …)` and `(installed …)` as `decision_trust-role-programs.md` §3 and §4 state, records and reports both new kinds of item, and refuses in `trust-verify` a result whose role is no role of `trust::ROLES` or whose producer is a derivation's artifact; its report's list of what stays outside gains §3's and §4's lines; it is tested on fixtures — each read failure, case 5 inside and outside a derivation's units, a `#[no_mangle]` and a procedural macro on a derivation's development edge, a harness-produced reference-model result accepted, a derivation's and a no-role result refused — the existing pairs of a generator and a checker rewritten; the parent and the generated-sources record say their passages read with it from its commit, and the book's trust chapter says so; and it writes none of these forms. |
| `RP-H3` | `M3.5.4` | Each `include_str!` site of `crates/archogen-plan-check` is admitted in `trust/roots.eadl` by a `defadmit` of its own. |
| `RP-H4` | `M3.5.5` | The comparison of the checker's fenced joint reading with `joint::System::read` is a `defderivation` form of its own, written after its reader reaches `main`, its class written by its review. |
| `RP-H5` | `M3.5.1.4` | The generator's form gains `(invokes configuration-checker)` and `(installed "archogen-install.json")` with the launcher, in a push after the one that brings `M3.5.2.1`'s reader to `main`, a test holding the name to `archogen_install::FILE`, and `cargo xtask install` places exactly the roots the front's form invokes. |
| `RP-H6` | `M2.21.5` | The generator's form invokes `scheduling-checker` in the commit in which `archogen analyze` runs `archogen-analyze` through the front's launcher, after `M2.21.3` and `M3.5.1.4`, and that invocation's form comes from the runner as `decision_trust-role-programs.md`'s *How to apply* says. |
| `RP-H7` | `M3.4` | `crates/eadl-resolve` joins the generator's role packages in `trust/roots.eadl` in the commit that makes the generator compile it, its two `include_str!` sites admitted unless `M3.5.3.1` has admitted them, and the forms of the items its new edges move come from the runner as `decision_trust-role-programs.md`'s *How to apply* says. |
| `RP-H8` | `M4.7` | The package records the install manifest of its own trust build beside the artifacts it describes, never as a handed dependency, and `cargo xtask trust-verify` refuses one whose digests are not the inventory's, `TI-H14` amended to say so. |
| `RP-H9` | `M3.5.1.8` | The form of `generator+scheduling-checker package crates/archogen-digest` comes from the runner as `decision_trust-role-programs.md`'s *How to apply* says, writing its §5's class, property, residual risk and controls. |
| `RP-H10` | `M3.5.1.6` | `crates/archogen-transport`'s controls are written into `decision_trust-role-programs.md` §5's row, through that record's review, before a checker compiles it. |
| `RP-H11` | `M3.5.3.1` | `trust/roots.eadl` holds the `defderivation` form for the relation's comparison, after `M3.5.2.1`'s reader reaches `main`, `crates/eadl-resolve`'s two `include_str!` sites admitted unless `M3.4` has admitted them. |
| `RP-H12` | `M3.5` | Each leaf of `M3.5` that adds a shared item, or moves one's measured aspects, has its forms come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, in a leaf of their own. |
| `RP-H13` | `M2.21.3` | The forms of the shared items the scheduling checker's executable root adds come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, in a leaf of their own. |
| `RP-H14` | `M3.5.3.2` | The forms of the shared items `M3.5.3`'s root and `M3.5.3.1`'s derivation add come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, each writing its §5's row. |

## Review

| Round | Date | Defects | Remarks | Verdict |
| --- | --- | --- | --- | --- |
| 1 | `2026-10-23` | 23 | 10 | not acceptable; every finding answered |
| 2 | `2026-10-23` | 14 | 6 | not acceptable; every finding answered |
| 3 | `2026-10-23` | 10 | 10 | not acceptable; every finding answered |
