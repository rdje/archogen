# The checkers' programs: the configuration checker's root, its shared derivation, invoked programs, what roles share

- **Type:** `decision`
- **Date:** `2026-10-23`
- **Status:** `active` — written `2026-10-23` by leaf `M3.5.2`; round 1 answered the same day; under independent review
  ([`decision_trust-role-programs-reviews.md`](../../reviews/decision_trust-role-programs-reviews.md))
- **External sources:** none beyond the parent's: `ROADMAP.md` §4.4 (its "externally invoked tools", and the classes
  of a shared item, "reference derivation" among them) and §14.4's cases.
- **Owner / source:** leaf `M3.5.2` (`docs/tasks/M3.md`), as `M3.5`'s and `M3.5.1`'s reviewed designs assign it, under
  the director's ruling of `2026-10-22` (findings §16 (a): each checker an executable of its own, the human command
  line alone running it, as git runs a sibling). It amends [`decision_trust-inventory.md`](decision_trust-inventory.md),
  cited below as "the parent", whose terms — root, role package, the comparison harness, program target, shared item,
  form, the baseline's host — it uses unchanged.

## The fact / decision

### 1. What the parent cannot yet hold, measured

- **One derivation, invisible.** `M3.5` moves the substitutability relation's executable model, byte for byte, into
  the configuration checker's package, `crates/archogen-plan-check` (`M3.5.3`), and keeps the comparison holding the
  generator's production relation equal to it where it is, `crates/eadl-resolve/tests/production.rs` over
  `tests/common/universe.rs`, taking the checker's package as a development dependency. The two relations are then one
  derivation, held equal, and more than a shared reading of one record: the production relation follows the model
  line for line in places (`side_items`, `side_name_lists`, `horizon`; `M3.3`). The parent builds no test target but
  the harness's and follows no development edge but the harness's (parent §3; `PROGRAM_KINDS` in
  `xtask/src/trust.rs` names no test), so the comparison, and every other `eadl-resolve` test that compiles the
  checker's package, is **invisible** to the gate — neither reported nor refused.
- **A harness for any pair.** The parent's prose gives a harness to a reference model and the implementation it
  validates (parent §2, §4), and the instrument reads a `defharness` naming any two roots (`check_pairs`). The same
  comparison could be written as a harness, which is paired with every other root and accepted as a result's producer
  (`trust_verify.rs`, `TI-H8`) — wrong of a comparison that produces no claim's result.
- **A program that runs another.** The front, `archogen`, will run `archogen-plan-check` and `archogen-analyze` as
  processes (`ROADMAP.md` §10.2; `M3.5.1.4`). The parent sees what a root compiles and the data its form hands it
  (parent §3), not a program it executes, nor the install manifest it reads (`archogen-install/1`, `M3.5.1.3`), which
  is no blob of the commit. §4.4's "externally invoked tools" are, in their context, the tools a build executes;
  reading them as the programs a root runs is this record's extension, as it was `M3.5.1`'s.
- **What the roles now share.** `cargo xtask trust-inventory --commit 0a5a4b12`, on `aarch64-apple-darwin`: exit 0,
  `5 program(s), 8 pair(s), 10 shared item(s), 14 refused site(s) each admitted`, the tenth
  `generator+scheduling-checker package crates/archogen-digest` — reached on the generator's side through
  `archogen-outcome`, a dependency of `archogen-api` since `5bc5627d` (`M3.5.1.1`), and on the scheduling checker's
  through `archogen-evidence`. The generator compiles `archogen-json` directly since `e9245a45` (`M3.5.1.7`) and
  through `archogen-outcome` since `5bc5627d`. The digest item's form waits on the baseline's host (`M3.5.1.8`).

### 2. The configuration checker's root

From `M3.5.3` the role is rooted at its library, as the parent §2 roots a role no executable runs yet:
`(defroot configuration-checker (role configuration-checker) (package "crates/archogen-plan-check") (target lib)
(role-packages "crates/archogen-plan-check"))`, replacing `(defrole configuration-checker (leaf "M3.5"))`; from `M3.5.6`
its target is the executable, `(target bin archogen-plan-check)`. The kind modules, `docs/semantics/kinds/core.eadl`
and `os-rt.eadl`, are compiled in by `include_str!` from `M3.5.4`, when check (2) re-checks the description — the
model moved at `M3.5.3` holds its own vocabulary and reads no kind module — as `archogen-api` compiles them, so the two
roots' compilations read the same two files: shared items of the parent §4's second kind, each `include_str!` site
admitted by a `defadmit` of its own (parent §3). They are not `(data …)`; any other fixed data the checker comes to
read at run time — an adapter's or a catalog's knowledge, which `M3.5` foresees — is declared there by path, as the
parent §2 asks. The description and the assignment record it checks are claim subjects, never dependencies (`TI-H19`).

### 3. The shared derivation

**The form.** `(defderivation <name> (pair generator configuration-checker) (package "<dir>") (test <target>) (reason
"…"))` names one test target. Its reading fails, as the parent's does for a harness's shape, so the gate cannot judge
(`read_roots`, exit 2): a pair naming no root or one root twice, a pair other than the generator's root and the
configuration checker's, a package the commit lacks, a test target its package lacks, two forms naming one target.
**`defharness` is narrowed to match:** its pair is a reference model and the implementation it validates, refused
otherwise, so a comparison of the generator and a checker is written one way.

**Its build and item.** The gate builds the named test target as it builds the harness's — `cargo test --no-run
--test <target>`, its development edges followed — and records the units it compiles beside its pair's two builds.
That set is a shared item of its pair and of no other root, of kind `shared-derivation`, its aspects the harness
item's: content, configuration and provenance. It produces no claim's result, so no result may name it as its
producer (`TI-H8` unchanged: "its role's inventoried artifact or the pair's harness"). Its exemption from
`trust-shared-program` is its pair's role packages alone; one of any other role is refused, as for the harness.
Until `M3.4` makes `crates/eadl-resolve` a role package of the generator the exemption is vacuous — the comparison
compiles the checker's role package and no other role's — and the item is reported all the same, which is the
disclosure. **Outside it:** the comparison reads the kind modules at run time (`production.rs`, `std::fs`), a test's
reading, not a compiled input; the report states it, as the parent §3 states what remains outside.

**Its class** (§5): §4.4's **reference derivation**. The property: the checker's relation is the generator's,
transcribed once — the model from the relation record, the production relation from the model, line for line in
places — and held equal. The residual risk: a misreading of the record, or a fault in transcribed code, that both
carry passes the checker. The controls: the record's 31 review rounds, the model's exhaustive checker and its corpus,
and the comparison over the bounded universe on every commit, each mechanized; the order of every amendment — the
checker's model first, the generator's relation after — a procedure, not a check. What the checker adds of its own is
the plan's identity, its completeness and the joint rules (`M3.5`). The comparison of the checker's fenced joint
reading with `joint::System::read` (`M3.5.5`) compares two readings written apart: a form of its own, whose class
its review writes, never this one's.

### 4. Invoked programs and installed data

**`(invokes <root> …)`**, on the front's form alone — the root whose role is the generator's and whose target is an
executable. Refused as `trust-invocation`, decided from the forms' text before any build, wherever the gate runs:
`invokes` on any other root's form; on a front whose target is not an executable; an empty clause; a root named twice;
a name that is no root, or is the generator's root, a root whose target is not an executable, the harness or a
derivation. Otherwise each invocation is a shared item of the two roots, of kind `invocation`, its identity
`<front>+<invoked> invocation`, its one aspect the content — the sha256 of the invoked root's name, role and target —
so a new or changed invocation is reported for review, as a new consumer is. A process is not a compilation: the
invoked root's packages are not the front's, and an invocation is never `trust-shared-program`.

**`(installed "archogen-install.json")`**, on the front's form alone: a bare file name, the manifest
`cargo xtask install` writes beside the front (`archogen_install::FILE`). It is no blob and no `(data …)`, so the
blob rule of `trust-undeclared-input` does not apply and the inventory hashes nothing for it: the install derives every
digest it holds from the inventory (`M3.5.1.3`), and places exactly the roots the front's form invokes. The package
records it beside the artifacts it describes, never as a handed dependency, and its verifier holds its digests to the
inventory's (`M4.7`).

**What stays outside, stated in the report.** The gate cannot see that the front runs only, or all, the programs its
form invokes. `NO-SUBPROCESS` admitting one file, `sibling.rs`, declared from the front's `main.rs` alone
(`M3.5.1.4`), the launcher running only what the manifest names, and the install placing only invoked roots stand
there; a test holds the form's `(installed …)` name to `archogen_install::FILE`.

### 5. What the roles share, each with its class and residual risk

Proposed for each form's review (parent §5), one class an item; a control marked *pending* has its owner.

| Item | Pairs | Class | Residual risk | Controls |
| --- | --- | --- | --- | --- |
| `crates/eadl-front` | generator, configuration checker | interpretation/normalization | a misreading of a description both make | the conformance suite, the frozen verdicts |
| `crates/eadl-model` | generator, configuration checker | semantic/analysis logic | a wrong quantity, comparison or pass both apply | its tests, the frozen verdicts, the profile's classified rules |
| the kind modules, two files | generator, configuration checker | authoritative data | a wrong definition both read | `LANGUAGE-FREEZE` and the migration notes |
| `crates/archogen-digest` | every pair it joins | infrastructure | a wrong digest in every role's records | three oracles: the derived constants, the published examples, the system tool's boundaries |
| `crates/archogen-outcome` | every pair it joins | interpretation/normalization: its reader decides acceptance | one fault reading a refusal as an acceptance for every role | one text per record, the consistency rules, the damage sweep; the exit code as a second channel, *pending* `M3.5.1.4` |
| `crates/archogen-json` | every pair it joins | interpretation/normalization | a parsing fault common to every record and server | RFC 8259 strictness, a member named twice refused; the records' readers re-write what they read |
| `crates/archogen-transport` | every pair it joins | interpretation/normalization | a framing fault common to every server | *pending* `M3.5.1.6`, which writes this row's controls before a checker compiles it |
| the shared derivation (§3) | generator, configuration checker | reference derivation | as §3 | as §3 |
| an invocation (§4) | the front and each invoked root | interpretation/normalization | the front reporting a refused or unchecked plan as checked | *pending* `M3.5.1.4`: the exit code agreeing with the record, the subject's digests, the program's sha256 |
| the build configuration | every new pair | infrastructure | as the parent's build-configuration items | as theirs |

`crates/archogen-install` is the front's alone, and `xtask`, which writes the manifest, is no root. `eadl-front` and
`eadl-model` join the scheduling checker's pairs when it reads descriptions (`M2.7.5`, parent §1); each new pair's rows
are this table's.

### 6. What the parent reads with it

- **The opening, the change part, the standing list and `TI-H2`** take each `defderivation` form and each invocation:
  added, changed or removed against the base commit's in the change part, named in the standing list until accepted.
- **§2:** the configuration checker's root, as §2 above; the forms gain `defderivation`; `defharness`'s pair is a
  reference model and its implementation; the front's form gains `(invokes …)` and `(installed …)`; the exemption from
  `trust-shared-program` extends to a derivation's pair; and its sentence on `archogen analyze` and `archogen
  resolve` is answered — each checker is an executable of its own, which the front invokes.
- **§3:** "Where it builds" builds a derivation's test target as the harness's; the default-deny scope, the
  admissions and their counting cover its closure as the programs'; what is recorded for each root gains the front's
  invocations and installed file; what remains outside gains a derivation's run-time reads and §4's last paragraph.
- **§4 and §5:** two kinds of shared item, the shared derivation and the invocation, with the aspects above; a form
  proposes their digests as it does any item's.
- **§6:** `trust-invocation`, refused, as §4; `trust-new-shared`, `trust-shared-changed` and `trust-form-missing` take
  both kinds; the blob rule of `trust-undeclared-input` exempts `(installed …)`; `trust-baseline-stale` takes a
  derivation whose package or test target is gone, and an invocation whose root is gone.
- **`TI-H9`**'s case-5 sets take a derivation's units; **`TI-H24`**'s fixtures gain the derivation's and the
  invocation's; **`TI-H19`** is amended in the parent's ledger in this change; `TI-H18` stands.

## Why

- **Disclosed, not hidden.** A derivation two roles share is the loss of independence §4.4 exists to surface; a form
  of its own puts it before a reviewer with its class, where today the gate sees nothing of it.
- **One way to write a comparison.** A harness and a derivation differ in what they may produce and whom they are
  paired with; a comparison that could be written as either would take whichever its author found convenient.
- **A process is an input.** The program a root runs is another root's artifact, and the invocation item names it.

## How to apply

Each form's reading is built in the leaf where the form first has a subject (the hand-offs below). Until then a form
of these kinds fails `read_roots`, so the gate cannot judge (exit 2), and none is written early.

<!-- machine-read: handoffs -->
| Id | Leaf | Hand-off |
| --- | --- | --- |
| `RP-H1` | `M3.5.3` | The configuration checker's library is its root in `trust/roots.eadl` from the commit that creates `crates/archogen-plan-check`, replacing `(defrole configuration-checker …)`, and the forms of the items it adds are the runner's proposal at the next authorized cadence push, each carrying `decision_trust-role-programs.md` §5's class. |
| `RP-H2` | `M3.5.3` | The gate reads `defderivation` as `decision_trust-role-programs.md` §3 states and refuses a `defharness` whose pair is not a reference model and the implementation it validates, each tested on fixtures; `trust/roots.eadl` holds the form for the relation's comparison; the book's trust chapter lists the form and its item. |
| `RP-H3` | `M3.5.4` | Each `include_str!` site of `crates/archogen-plan-check` is admitted in `trust/roots.eadl` by a `defadmit` of its own. |
| `RP-H4` | `M3.5.5` | The comparison of the checker's fenced joint reading with `joint::System::read` is a `defderivation` form of its own, its class written by its review. |
| `RP-H5` | `M3.5.1.4` | The gate reads `(invokes …)` and `(installed …)` on the front's form alone as `decision_trust-role-programs.md` §4 states, refusing `trust-invocation` and reporting each invocation as a shared item, tested on fixtures; the generator's form gains `(invokes configuration-checker)` and `(installed "archogen-install.json")` with the launcher, a test holding the name to `archogen_install::FILE`; `cargo xtask install` places exactly the roots the front's form invokes; the book's trust chapter lists both clauses and the code. |
| `RP-H6` | `M2.21.3` | The generator's form invokes `scheduling-checker` from the commit that roots the scheduling checker at its executable. |
| `RP-H7` | `M3.4` | `crates/eadl-resolve` joins the generator's role packages in `trust/roots.eadl` in the commit that makes the generator compile it. |
| `RP-H8` | `M4.7` | The package records the install manifest beside the artifacts it describes, never as a handed dependency, and `cargo xtask trust-verify` refuses one whose digests are not the inventory's, `TI-H8` and `TI-H14` amended to say so. |
| `RP-H9` | `M3.5.1.8` | The form of `generator+scheduling-checker package crates/archogen-digest` carries `decision_trust-role-programs.md` §5's class, risk and controls. |
| `RP-H10` | `M3.5.1.6` | `crates/archogen-transport`'s row of `decision_trust-role-programs.md` §5 has its controls written before a checker compiles it. |

## Review

| Round | Date | Defects | Remarks | Verdict |
| --- | --- | --- | --- | --- |
| 1 | `2026-10-23` | 23 | 10 | not acceptable; every finding answered |
