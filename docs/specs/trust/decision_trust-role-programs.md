# The checkers' programs: the configuration checker's root, its shared derivation, invoked programs, what roles share

- **Type:** `decision`
- **Date:** `2026-10-23`
- **Status:** `active` — written `2026-10-23` by leaf `M3.5.2`; rounds 1 and 2 answered the same day; under review
  ([`decision_trust-role-programs-reviews.md`](../../reviews/decision_trust-role-programs-reviews.md))
- **External sources:** none beyond the parent's: `ROADMAP.md` §4.4 (its "externally invoked tools", and the classes
  of a shared item, "reference derivation" among them) and §14.4's cases.
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
  `horizon`; `M3.3`). The parent builds no test target but the harness's and follows no development edge but the
  harness's (parent §3; `PROGRAM_KINDS` names no test), so that comparison, and every other test compiling the
  checker's package, is **invisible** — neither reported nor refused.
- **A harness for any pair.** The parent's prose gives a harness to a reference model's pair; the instrument reads a
  `defharness` naming any two roots (`check_pairs`), pairs it with every other root, and the verifier accepts a
  result whose role text is a recorded program's (`trust_verify.rs`), so a comparison producing no claim's result
  could be written as one and pass as a producer.
- **A program that runs another.** The front, `archogen`, will run `archogen-plan-check` and `archogen-analyze` as
  processes (`ROADMAP.md` §10.2; `M3.5.1.4`) and read the install manifest (`archogen-install/1`), no blob of the
  commit. The parent sees what a root compiles and the data its form hands it (§3). §4.4's "externally invoked tools"
  are, in context, a build's; reading them as the programs a root runs is this record's extension, as `M3.5.1`'s.
- **What the roles share.** `cargo xtask trust-inventory --commit 0a5a4b12`, `aarch64-apple-darwin`: exit 0, `5
  program(s), 8 pair(s), 10 shared item(s)`, the tenth `generator+scheduling-checker package crates/archogen-digest`,
  reached through `archogen-outcome` (`archogen-api`'s since `5bc5627d`, `M3.5.1.1`) and through `archogen-evidence`.
  The generator compiles `archogen-json` directly since `e9245a45` (`M3.5.1.7`). That item's form waits on the
  baseline's host (`M3.5.1.8`).
- **The gate is built from the base** (parent `TI-H6`; `.github/workflows/trust-gate.yml`): on a push, the commit
  `main` pointed at before it. Its `read_roots` fails on a form or clause it does not know, so a commit writing a new
  kind of form is unjudged (exit 2) until a push has brought the reader to `main` first.

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
"…"))`, one test target a form. Its text read wrong fails `read_roots` (exit 2), as a harness's does (`check_pairs`):
a pair naming no root, one root twice, or other than the generator's root and the configuration checker's; a target
two forms name. A package or target the commit lacks is `trust-baseline-stale`, from metadata, wherever the gate runs.
**`defharness` is narrowed:** its pair is one root of role `reference-model` and one of role `implementation`, in
either order; today's `differential` is.

**Its build, record and item.** Built as the harness — `cargo test --release --no-run -p <package> --test <target>`,
the parent's flags, in its own target directory, the development edges of its own package followed — and recorded in
the inventory's programs as the harness is, its role `derivation`, its artifact the test binary. The units it
compiles beside its pair's two builds are a shared item of its pair and of no other root: kind `shared-derivation`,
identity `<a>+<b> shared-derivation <name>`, the pair in the inventory's order, one item a form, aspects the harness
item's — content, configuration, provenance. It is a **program** in the generated-sources record's sense, its pair's
two exempt there as the harness's are. Its exemption from `trust-shared-program` is its pair's role packages alone.
Until `M3.4` makes `crates/eadl-resolve` the generator's, the exemption is vacuous and the item reported all the
same. Building it compiles `eadl-resolve`'s library, whose two `include_str!` sites (`src/vocabulary.rs`:
`kinds/deffact.eadl`, `vocabulary/vocabulary.eadl`) enter the default-deny scope and the item's content, each
admitted by a `defadmit`. No result may name it as its producer: the verifier refuses a result whose producer's role
is `derivation` or no role of `trust::ROLES`. **Outside it:** its run-time reads of the kind modules (`std::fs`),
stated in the report. A test or bench target whose package's normal and development closure, read from metadata,
holds role packages of two roles and that no form names is **reported**, so no comparison stays unseen.

**Its class** (§5), §4.4's **reference derivation.** The comparison holding the two equal is the coupling itself,
not a control: a fault both carry passes it. Controls independent of the transcription: the model's exhaustive
checker for the properties the relation record states, and the falsification corpus written from the record (both
built); F12's hand-written corruptions and accepted plans (`M3.5.4`, `M3.5.5`, *pending*); the frozen verdicts once
`M3.4` wires the relation into `archogen check` (*pending*). The record's 31 review rounds, and amending the checker's
model before the generator's relation, are procedures. The comparison of the checker's fenced joint reading with
`joint::System::read` (`M3.5.5`) compares two readings written apart: a form of its own, its class its review's.

### 4. Invoked programs and installed data

**`(invokes <root> …)` and `(installed "archogen-install.json")`**, on the front's form alone — the root of role
`generator` whose target is an executable. Each read wrong fails `read_roots` (exit 2): either clause on another
root's form or on a front not an executable; an empty clause or a root named twice; an invoked name that is no root,
the generator's, a root whose target is not an executable, the harness or a derivation; `(installed …)` holding more
than one name, a name other than `archogen-install.json` (`archogen_install::FILE`), or no `(invokes …)` beside it.
An invocation is a shared item of the two roots: kind `invocation`, identity `<a>+<b> invocation`, the pair in the
inventory's order; its content the invoked root's name, role and target and the sha256 of the launcher, `crates/
archogen-cli/src/sibling.rs`, whose acceptance contract carries its risk; so a new or changed invocation, or launcher,
is reported. A process is not a compilation: an invocation is never `trust-shared-program`. The installed name is no
blob and no `(data …)`: the blob rule of `trust-undeclared-input` does not reach it, and the inventory hashes nothing
for it; the install derives its digests from the inventory and places exactly the invoked roots (`M3.5.1.3`), and the
package records it beside the artifacts it describes, never as a handed dependency (`M4.7`).

**What stays outside, stated in the report:** that the front runs only, or all, the programs its form invokes.
`NO-SUBPROCESS` admitting `sibling.rs` alone, declared from the front's `main.rs` (`M3.5.1.4`), the launcher running
only what the manifest names, and the install placing only invoked roots stand there.

### 5. What the roles share

Proposed for each form's review (parent §5): one class, the property it can affect, its residual risk and controls;
*pending* names the owner.

| Item | Pairs | Class | Property | Residual risk | Controls |
| --- | --- | --- | --- | --- | --- |
| `crates/eadl-front` | generator, configuration checker | interpretation/normalization | configuration validity | a misreading both make | the conformance suite, the frozen verdicts |
| `crates/eadl-model` | generator, configuration checker | semantic/analysis logic | configuration validity | a wrong quantity, comparison or pass both apply | its tests, the frozen verdicts, the profile's classified rules |
| the kind modules, two files | generator, configuration checker | authoritative data | configuration validity | a wrong definition both read | `LANGUAGE-FREEZE`, the migration notes |
| `crates/archogen-digest` | every pair it joins | infrastructure | every checked result's identity | a wrong digest in every role's records | three oracles: derived constants, published examples, the system tool's boundaries |
| `crates/archogen-outcome` | every pair it joins | interpretation/normalization | every checked result | a refusal read as an acceptance for every role | one text per record, its consistency rules, the damage sweep; the exit code's second channel *pending* `M3.5.1.4` |
| `crates/archogen-json` | every pair it joins | interpretation/normalization | every checked result | a parsing fault common to every record and server | RFC 8259 strictness, a member named twice refused; the records' readers re-write what they read |
| `crates/archogen-transport` | every pair it joins | interpretation/normalization | every served result | a framing fault common to every server | *pending* `M3.5.1.6` |
| the shared derivation | generator, configuration checker | reference derivation | configuration validity | as §3 | as §3 |
| an invocation | the front and each invoked root | interpretation/normalization | every checked result | the front reporting a refused or unchecked plan as checked | *pending* `M3.5.1.4`: the exit code agreeing with the record, the subject's digests, the program's sha256 |
| the build configuration | every new pair | infrastructure | every result | a toolchain, profile or flag fault common to every role | the pinned toolchain (`SOURCE-LEDGER`), the item compared on the baseline's host |

`crates/archogen-install` is the front's alone; `xtask` is no root. `eadl-front` and `eadl-model` join the scheduling
checker's pairs when it reads descriptions (`M2.7.5`); each new pair's rows are this table's.

### 6. What the parent, and the generated-sources record, read with it

- **The parent's opening:** "a change outside every root and every program target" reads "… every program target and
  every derivation's units"; **the change part, the standing list and `TI-H2`** take each `defderivation` form and
  invocation, added, changed or removed; **case 5** and `M3.6.7`'s list of what the gate reads beyond the roots take a
  derivation's units as the harness's; **`TI-H9`**'s sets likewise.
- **§2:** the checker's root (§2 above); `defderivation`; `defharness` narrowed; the front's two clauses; the
  exemption extended to a derivation's pair; the sentence on `archogen analyze` and `archogen resolve` answered — each
  checker an executable the front invokes.
- **§3:** "the comparison harness a pair's form names is built" and the build configuration's "overrides for a
  package some root or the harness compiles" take a derivation; "development edges not followed … but the harness's
  own" and the artifact bullet take a derivation's; the default-deny scope and admissions cover its closure; what is
  recorded per root gains the front's invocations and installed name; what remains outside gains §3's and §4's lines.
- **§4, §5:** the two kinds of item, their identities and aspects, and their forms' digests.
- **§6:** the derivation's and the clauses' read failures (§3, §4); `trust-new-shared`, `trust-shared-changed` and
  `trust-form-missing` take both kinds; `trust-undeclared-input`'s "a root or the harness compiles" takes a derivation,
  its blob rule not the installed name; `trust-shared-program`'s exemption a derivation's pair; `trust-baseline-stale`
  a derivation whose package or target is gone.
- **Case 4, `trust-inventory-stale` and §7's package contents** gain the install manifest, as `M4.7` records it;
  **How to apply**'s "a comparison harness is named in its pair's form" gains a derivation; **`TI-H24`** its fixtures.
- **The generated-sources record:** §1's "a program is a root or the comparison harness" gains a derivation; §5's
  `trust-shared-program`, "for the harness, than its pair's two", gains a derivation; §7's case-5 list its units.
- **`TI-H19`** is amended in the parent's ledger in this change; `TI-H18` stands.

## Why

- **Disclosed, not hidden.** A derivation two roles share is the loss of independence §4.4 exists to surface; a form
  of its own puts it before a reviewer with its class, where today the gate sees nothing of it.
- **One way to write a comparison.** A harness and a derivation differ in what they may produce and whom they are
  paired with; a comparison writable as either would take whichever its author found convenient.
- **A process is an input.** The program a root runs is another root's artifact, and the invocation names it.

## How to apply

**Two pushes for every new form.** Because the gate is built from the base (§1), a form's reader lands first
(`M3.5.2.1`), reaches `main` through a cadence push, and only a commit of a later push writes the form. The forms of
the shared items a commit adds are its push's runner proposal, committed in a leaf of their own that waits on the
runner, writing §5's row — as `M3.5.1.8`, and `M3.6.3.2.1.1` for a refreshed encoding; until then the gate on the
baseline's host refuses `trust-form-missing`, which the push's report states.

<!-- machine-read: handoffs -->
| Id | Leaf | Hand-off |
| --- | --- | --- |
| `RP-H1` | `M3.5.3` | The configuration checker's library is its root in `trust/roots.eadl` from the commit that creates `crates/archogen-plan-check`, replacing `(defrole configuration-checker …)`. |
| `RP-H2` | `M3.5.2.1` | The gate reads `defderivation`, the narrowed `defharness`, `(invokes …)` and `(installed …)` as `decision_trust-role-programs.md` §3 and §4 state, records and reports both new kinds of item, reports a test or bench target compiling two roles' role packages that no form names, and refuses in `trust-verify` a result whose producer is a derivation or no role's; its report's list of what stays outside gains §3's and §4's lines; each tested on fixtures, `TI-H9`'s and `TI-H24`'s among them, the existing pairs of a generator and a checker rewritten; the book's trust chapter says so; and it writes none of these forms. |
| `RP-H3` | `M3.5.4` | Each `include_str!` site of `crates/archogen-plan-check` is admitted in `trust/roots.eadl` by a `defadmit` of its own. |
| `RP-H4` | `M3.5.5` | The comparison of the checker's fenced joint reading with `joint::System::read` is a `defderivation` form of its own, written after its reader reaches `main`, its class written by its review. |
| `RP-H5` | `M3.5.1.4` | The generator's form gains `(invokes configuration-checker)` and `(installed "archogen-install.json")` with the launcher, in a push after the one that brings `M3.5.2.1`'s reader to `main`, a test holding the name to `archogen_install::FILE`, and `cargo xtask install` places exactly the roots the front's form invokes. |
| `RP-H6` | `M2.21.3` | The generator's form invokes `scheduling-checker` from the commit, after both this leaf and `M3.5.1.4`, in which `archogen analyze` runs the sibling. |
| `RP-H7` | `M3.4` | `crates/eadl-resolve` joins the generator's role packages in `trust/roots.eadl` in the commit that makes the generator compile it, its two `include_str!` sites admitted unless `M3.5.3.1` has admitted them. |
| `RP-H8` | `M4.7` | The package records the install manifest of its own trust build beside the artifacts it describes, never as a handed dependency, and `cargo xtask trust-verify` refuses one whose digests are not the inventory's, `TI-H14` amended to say so. |
| `RP-H9` | `M3.5.1.8` | The form of `generator+scheduling-checker package crates/archogen-digest` writes `decision_trust-role-programs.md` §5's class, property, residual risk and controls. |
| `RP-H10` | `M3.5.1.6` | `crates/archogen-transport`'s row of `decision_trust-role-programs.md` §5 has its controls written before a checker compiles it. |
| `RP-H11` | `M3.5.3.1` | `trust/roots.eadl` holds the `defderivation` form for the relation's comparison, after `M3.5.2.1`'s reader reaches `main`, `crates/eadl-resolve`'s two `include_str!` sites admitted unless `M3.4` has admitted them, and the forms of the items `M3.5.3`'s root adds are its push's runner proposal, each writing `decision_trust-role-programs.md` §5's row. |
| `RP-H12` | `M3.5` | Each leaf of `M3.5` that adds a shared item commits its forms from its push's runner proposal in a leaf of their own that waits on the runner, each writing `decision_trust-role-programs.md` §5's row. |
| `RP-H13` | `M2.21.3` | The forms of the shared items the scheduling checker's executable root adds are its push's runner proposal, committed in a leaf of their own that waits on the runner, each writing `decision_trust-role-programs.md` §5's row. |

## Review

| Round | Date | Defects | Remarks | Verdict |
| --- | --- | --- | --- | --- |
| 1 | `2026-10-23` | 23 | 10 | not acceptable; every finding answered |
| 2 | `2026-10-23` | 14 | 6 | not acceptable; every finding answered |
