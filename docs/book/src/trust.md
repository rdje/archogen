# Checks that must not share a mistake: the trust gate

## The idea, in plain words

archogen does not ask you to trust one program. The **generator** turns a description into a system; separate
programs then check its work — a **configuration checker** that the plan is valid, a **scheduling checker** that the
deadlines hold — and a **reference model**, written apart from the runtime, says what the runtime should have done.
Each check is worth something only because it was reached another way. Two checks built from the same code can share a
mistake, and then they agree on a wrong answer.

So sharing has to be **visible**. Programs share a great deal, and most of it is harmless: a shared way to allocate
memory cannot make two programs agree on a wrong answer. A shared way to evaluate a constraint can. The trust gate
does not forbid sharing. It lists what each checked program is built from, finds everything two of them have in
common, and makes every such item wait for a person to look at it — and it says plainly when something new is shared,
or something already shared has changed behind an unchanged name.

An everyday comparison: two accountants auditing the same books are worth two opinions only if they did not both copy
their totals from the same spreadsheet. Nobody minds them using the same calculator. The gate is the list of
everything they used, kept current, with each shared spreadsheet flagged for review.

> **In one minute, for engineers.** `trust/roots.eadl` names each **root** — the program that runs a role — and
> classifies every other program target of the workspace. `cargo xtask trust-inventory` builds each root clean from
> the commit's blobs, reads what [rustc](ledger.md#rust-toolchain) read, and lists every **shared item** two roots have in common: a package, a
> file, a byte-for-byte copy, the comparison harness, the build configuration, what a committed generated source was
> made from. `trust/baseline.eadl` holds one
> **form** per shared item, proposed by the tool and accepted only by a protected review (`M3.6.5`).
> `cargo xtask trust-gate` compares a commit's inventory with its base commit's forms and writes a report in two
> parts, the change and the standing list; `cargo xtask trust-verify` checks a package that carries an inventory. It
> enforces disclosure and change control. It does not prove independence.

## How it works

### The roots

A **root** is the program that runs one role: the `archogen` executable for the generator, the scheduling checker's
library until an executable runs it, and, for the reference model and the implementation it validates, their
libraries, since a test harness runs them. Each is a form in `trust/roots.eadl`, naming its role, its package and
target, the **role packages** holding its role's own logic, and any fixed data it is handed at run time:

```text
(defroot generator
  (role generator)
  (package "crates/archogen-cli")
  (target bin archogen)
  (role-packages "crates/archogen-cli" "crates/archogen-api" "crates/archogen-s0"))
```

The root's own package always belongs to its role packages, even if the clause omits it. With
`(package "crates/archogen-cli")` and `(role-packages "crates/archogen-s0")`, both packages belong to the root's set;
without a role-packages clause, its set still contains `crates/archogen-cli`. Every declared member is retained.

`defroot`, `defharness`, `defderivation`, `defprogram` and `defrole` place a symbol or string name immediately after the
head.
A missing name, integer or clause in that position refuses before a build. `defadmit` deliberately has no name.
A role without a root names its future task: `(defrole configuration-checker (leaf "M3.5"))`.
Its `leaf` clause requires one textual value; absent, empty or multiple-value clauses refuse.
This placeholder creates no program.
Clauses may appear in any order, but each clause is allowed only once and only if the form takes it.
A misspelling such as `(role-pakages "crates/other")` refuses at the beginning, middle or end of a root.
Two `(data …)` clauses also refuse wherever the second occurs: put several paths in one clause,
for example `(data "data/one.txt" "data/two.txt")`. A valid clause after a departure does not hide it.

The comparison harness in `trust/roots.eadl` declares:

```text
(defharness differential (pair reference-model implementation)
  (package "crates/rt-core") (test differential))
```

Every harness must name distinct declared roots. Unknown names or another harness in either slot refuse,
including in a harness between valid ones. A harness compares a reference model with the implementation it
validates — one root of role `reference-model` and one of role `implementation`, in either order — and any other
pair refuses, so a comparison of the generator with a checker cannot be written as one.

**Read and built, written by no commit yet** (`M3.5.2`'s readers, `decision_trust-role-programs.md`).
The gate reads two forms no commit
writes yet, since the gate that judges a push is built from the commit before it and must already know a form:

- `(defderivation <name> (pair generator <checker>) (package "<dir>") (test <target>) (reason "…"))`, a test
  comparing a reading of the generator's with either checker's, its pair the generator's root and that checker's, in
  either order, and a reason;
- on the front's form alone — the one root of role `generator`, an executable — `(invokes <root> …)`, the other
  executable roots it runs, each once, and `(installed "archogen-install.json")`, together.

Each departure refuses before a build, as a misnamed harness does — a clause holding anything but names among them,
and a fault within one form stopping the reading at that form (leaf `M3.5.2.2.1`) — and one test target belongs to one
comparison only, never classified beside it (leaf `M3.5.2.3.2`). A derivation is built, recorded, scanned and held to its pair's role packages as the harness is, but paired with
no root: its pair takes its item, `shared-derivation`, one item a form (leaf `M3.5.2.1.2`). The front and each root it
invokes share an `invocation` item: the invoked root's name, role and target, and the sha256 of the launcher — the
file the constant `LAUNCHER` in `xtask/src/trust.rs` names, which the front gains with leaf `M3.5.1.4` — so an edit to
the launcher, whose acceptance contract carries the risk, is reported; a front that invokes while its commit holds no
launcher is refused (leaf `M3.5.2.1.3`). A comparison no form names is found by the build, not by what its sources
say: every test-like unit — an integration test, a bench, a library's or an executable's unit tests, a library's doc
tests — that some build of its package makes, whatever its flags, and whose build, by normal and build edges over every
feature and platform and its package's development edges, compiles role packages of both roles of a pair a property
rests on (the generator and either checker, the reference model and the implementation) is a program target. What a
unit compiles is read from the compiler itself: each package's targets are built as `cargo test` builds them — its
test profile, default features — once as it selects them, once with `--all-targets`, and by name, with its required
features, each target neither build made (leaf `M3.5.2.6.2`), and a unit counts every file its dependency information names and its closure's libraries' do, each
for the innermost package holding it (leaf `M3.5.2.6.1`), so a role package's file reached by a path, an `include!` or a dependency's own path joins it however its source spells it
(leaf `M3.5.2.5.1`; a scan of the text before it, `M3.5.2.4.1`, missed spellings). It counts what some build compiles, more than the default build makes, by
design (leaf `M3.5.2.3.1`). The inventory records the pairs each joins, so the gate's remedy names its pair's form —
a harness for the reference model's, a derivation for the generator's and a checker's — or, for a unit joining two
pairs or compiling a role package of a root beside its pair's two, which no pair form can name without its build being
refused, says to split or classify it (leaf `M3.5.2.5.2`).
An integration test that compares the two is named by its pair form; any other such unit is classified by a
`defprogram` whose target is `(target test|bench|lib-test|bin-test|doctest <name>)`, its reason saying it compares
nothing the two hold equal. Unnamed, it is reported as an unclassified program is, and refused on the baseline's host
(leaf `M3.5.2.2.2`; a census of names before it, `M3.5.2.1.5`, missed a comparison spelt another way).

A program that would run two roles is refused: one program cannot be independent of itself. Every other program the
workspace can build — an example, a diagnostic tool, the browser module — is classified as not a root, with a reason
and the role packages it compiles, so no one can add a second checker beside the generator unseen.

### The inventory

`cargo xtask trust-inventory` writes the commit's tree from git's blobs, builds each root in the release profile,
clean, and reads the compiler's own record of every file it read. Every source a root compiles is held to the
catalog's rules: a construct that could read something the commit does not track — an `include!` of an untracked
file, an environment variable, assembly pulling in bytes — is refused unless a reviewed **admission** names it.

Build subprocesses use a cleared environment with repository-derived Cargo, target and temporary
storage. `TMPDIR`, `TMP` and `TEMP` point to the canonical `tmp` directory inside each build target;
for example, metadata uses `<output>/target-metadata/tmp`. The directory is prepared before commands
run, and a blocked destination stops the run. Its runtime path remains correct when a child changes
working directory. Inherited temp overrides do not redirect these builds. These process settings do
not grant compiled source permission to read them: the compiler dependency check refuses source-level
`env!` or `option_env!` dependencies on these names.

Compiler invocations are observed through the pinned Cargo wrapper interface in a separate session
for each root, harness and generator build. The embedded observer forwards native arguments and
stdio unchanged, removes its private settings before the real compiler, and publishes completed
records without replacement. The consumer requires the current schema, role, session, working
directory and successful status. Compiler queries yield no units. Metadata ownership, dependency
files, source admissions and environment/configuration rules still judge every actual unit.
See the [protocol and examples](annex-trust-proofs.md#production-compiler-invocation-collection-repair).

Failure text retains Cargo’s actual compiler or setup diagnostic when an output path continues across
physical lines. Known workspace/artifact paths are excluded from diagnostic selection, including quoted
command arguments and canonical aliases. [The verified framing repair](annex-trust-proofs.md#cargo-failure-framing-repair)
shows compiler failures, operational refusals, empty inventory state and successful recovery.
The [initial repaired-helper measurement](annex-trust-proofs.md#complete-repaired-cargo-diagnostic-measurement)
records the dated 51-edit assessment: 38 named kills, nine survivor obligations and four compiler refusals.
[Individual controls and proofs](annex-trust-proofs.md#final-diagnostic-regression-parents) now resolve those
nine obligations. The [complete region replay](annex-trust-proofs.md#complete-diagnostic-region-replay)
records 43 kills, four proven equivalents and four compiler refusals, strict exit 0. The
[diagnostic integration checkpoint](annex-trust-proofs.md#diagnostic-cohort-integration-checkpoint)
passes all local stages and closes this cohort. The [ownership reconciliation](annex-trust-proofs.md#survivor-ownership-reconciliation)
accounts for every initial survivor and schedules the remaining source lanes. The [complete compiler measurement](annex-trust-proofs.md#complete-compiler-function-measurement)
records the initial 49 kills, 28 owned survivors and six compiler refusals, strict exit 1.
[Environment regression parents](annex-trust-proofs.md#compiler-environment-admission-regressions)
resolve four of those obligations with real refusal/recovery controls. Command-filtering assessment
now has [verified warning controls](annex-trust-proofs.md#admitted-nonterminal-warning-regression-parent). A
[reproduced parser defect](annex-trust-proofs.md#compiler-command-provenance-assessment) used to
interpret an admitted rustc-looking warning as empty-name compilation units. The
[production unit-collection repair](annex-trust-proofs.md#production-compiler-invocation-collection-repair)
now reads actual compiler arguments from private per-build records. Missing, stale or malformed
provenance stops the inventory; warning text cannot add a unit.
[Observed artifact binding](annex-trust-proofs.md#observed-compiler-artifact-binding) now selects each
role's metadata-owned compiler output through an empty-input naming query. Artifacts name direct
files under `release/deps/`, with observed suffixes, rather than Cargo root aliases. Existing
inventory path/hash fields remain the consumer contract. Missing, ambiguous, foreign or linked
outputs stop the inventory. The
[role-warning matrix](annex-trust-proofs.md#compiler-warning-data-across-harness-and-generator-roles)
holds admitted warning data, source refusal and actual compiler failure/recovery through both
harness and separate generator builds.
[Path/feature/copy controls](annex-trust-proofs.md#compiler-artifact-path-and-feature-compatibility)
also hold mixed-byte paths, canonical aliases, actual output hashes and target default-feature
suppression. The
[compatibility checkpoint](annex-trust-proofs.md#compiler-provenance-compatibility-checkpoint)
also accounts for record validation, writer failures, signals and overlapping actual compilers,
with explicit source and I/O limits. Native-tool hardening and full source/integration verification
remain before F30 signoff.

An admission's `defadmit` form names the file, rule and `sha256` of the site's extent (the whole manifest for a
manifest rule), plus one textual `reason` value.
For example, `(reason "the reviewed template")` has one value. An absent reason, `(reason)` or
`(reason "first" "second")` leaves the inventory unable to judge before any build. A well-shaped form still
needs the review that admits the site; reading it is not acceptance.

A file committed as a generator's output — a table a script wrote from a data file — is declared in the same roots
file, by a `defgenerated` form naming the file, the generator files that wrote it, the inputs they read, the command
that ran and why:

```text
(defgenerated "crates/x/src/table.rs"
  (generator "scripts/gen_table.sh")
  (inputs "docs/data/table.csv")
  (command "bash scripts/gen_table.sh docs/data/table.csv")
  (reason "the instruction table, generated from the vendor's CSV"))
```

The inventory reads these forms before it writes or builds anything, and refuses one that departs from that shape — a
clause the form does not take, no generator, a command that is not one string — or two forms declaring one file.
Nothing in a form is run: the command is there for the person who reviews it, and the declaration is believed, not
checked by generating the file again.

Once a program reads a declared file, the form's generator files and inputs become that program's **provenance**, and
two programs whose provenance meets share a **generated-provenance** item the gate reports like any other: one input
under two generators, a generator both use, a data file one reads that the other's source was made from, or a copy of an
input under another name. Two generated tables that differ in every byte, written by one script from one data file, are
seen to share both. A generator or input of such a form that the commit does not hold as a file is refused. So are a
**chain**, a generator or input that a form declares or a header marks as generated, and a generator or input inside a
vendored checkout, which the commit holds as a pointer to another repository rather than as files: each is refused for a
reason, and the need it would serve is met in one step instead where a route exists (below).

A generator must lie outside the reading program's other roles: a table the generator's own executable wrote, read by
a checker, is refused, since the checker would then trust the very logic it checks. A generator that is a program —
an executable or an example — is built as a root is, its sources held to the same rules and its admitted sites counted
with the roots', unless it is a root, whose build it already is; a script is judged by the package that holds it; a
Rust file that is no program's entry point is refused, since only a program's build can be computed.

A generated file that does not declare itself is recognised by its header: its opening comments saying `@generated`, *do
not edit*, *generated by* and their kin, read by the comment syntax of the file's name or extension
(`xtask/src/generated_header.rs`). A program that reads such a file with no form declaring it is refused, and so is a
generator whose own build reads a declared or marked file — a generator built from generated code, another's or its
own, a chain again. The recogniser sees only what a generator chose to mark, in a kind of file it knows: a table
written with no marker, a `.json` or `.lock` file, a binary, or a marker below the file's first content passes as a
hand-written file, and is judged as one. So a marker belongs in a generated file's very first comment, where a later
copy that nobody declared is still caught.

A form whose file no program reads any more is stale on the baseline's host, and is removed, so a generated source
read again is reviewed again.

For each pair of roots it then lists what they share, each item with the sha256 of its **content**, its
**configuration** (features, `cfg`, edition and compiled `env!` values, as each root's build sets them), its **edges** (who depends on it)
and, for a package's files, its **readers**. A new consumer of a package already shared is a change, though nothing in
the package moved.

### The baseline and its forms

`trust/baseline.eadl` holds one form per shared item: the item, the digests the inventory measured, and the review's
part — its classification, the property it can affect, its residual risk, the controls that remain:

| Classification | Can a shared item carry a shared mistake? |
| --- | --- |
| infrastructure | seldom — a shared allocator does not shape an answer, though a toolchain's or a digest's fault reaches every role, which its residual risk says |
| interpretation / normalization | yes — it shapes what both sides *see* |
| semantic analysis | yes — it shapes what both sides *conclude* |
| authoritative data | yes |
| reference derivation | yes |

`cargo xtask trust-baseline --propose` writes the forms with the review's part `unstated`. Nothing in the file can say
a form is accepted: acceptance is read where reviews are protected, never from a form's presence.

### The gate

The gate resolves `--repo` against the process's current directory before deriving its output and
configuration roots. From the repository root, `cargo xtask trust-gate --repo . --out target/trust`
uses the same repository and destination as their absolute spellings. Relative `--out` values are
relative to the selected repository; absolute values keep their destination. Repository aliases resolve
to their physical root. Missing or non-directory repository paths return 2 and create no output.
The [verified caller repair](annex-trust-proofs.md#repository-path-caller-repair) records the controls.
Its [local integration checkpoint](annex-trust-proofs.md#repaired-caller-integration-checkpoint)
passes the complete tier; the diagnostic helpers' mutation assessment is postponed with the rest (below).

`cargo xtask trust-gate` builds the commit's inventory and compares it with the forms its **base** commit holds. Its
report, `target/trust/report.txt`, names the build it describes and comes in two parts — here as it would read had the
scheduling checker come to compile the language's model, which is no role's own:

```text
trust-gate report — docs/specs/trust/decision_trust-inventory.md §6
commit: …
host: x86_64-unknown-linux-gnu
inventory: sha256 …
baseline: sha256 … — trust/baseline.eadl of the base commit …
verdict: passed

== the change, against the base commit's forms ==
trust-new-shared: `generator+scheduling-checker package crates/eadl-model`

== the standing list: nothing is accepted before M3.6.5, so every entry is unreviewed ==
shared item `generator+scheduling-checker build-configuration`: proposed, classification unstated
…

== what the inventory does not see ==
- the linker and the host's C toolchain, recorded by version alone (decision_trust-inventory.md §3)
…
```

The **change** says what this commit shares that its base did not, or shares differently, which forms of
`trust/roots.eadl` — a `defgenerated` one among them — it added, changed or removed against its base's, a comment being
no change, and which program targets are not classified or have outgrown their classification; or *unchanged*. The
**standing list** names everything still awaiting review, each declared generated source with its generators and
inputs among it. The report ends with what the inventory does not see, the same list every time ([What it is
not](#what-it-is-not)). A commit that shares something new must carry the form the
tool proposes for it, so the sharing is reported once, by the commit that makes it, and the next unrelated commit
reads *unchanged*.

### The package verifier

A report that a property holds rests on results produced by particular programs. `cargo xtask trust-verify <package>`
checks a package that carries the trust build's inventory, report and programs: that they are one commit's and one
toolchain's, that each result was produced by the program its role names, and that every file handed to a program at
run time was declared. A result's role is a root's — never the harness's or a derivation's own — and no derivation
produces one, while a reference model's result produced by its pair's harness is accepted (leaf `M3.5.2.1.4`). Its
manifest and inventory JSON require valid UTF-8.

## The precise rules

The design is `docs/specs/trust/decision_trust-inventory.md`, reviewed round by round until a round found no defect;
its section numbers are used below. The code is `xtask/src/trust.rs` (the inventory), `xtask/src/trust_gate.rs` (the baseline and the
gate) and `xtask/src/trust_verify.rs` (the verifier); committed generated sources are
`docs/specs/trust/decision_trust-generated-sources.md`, `docs/specs/trust/decision_trust-generated-refusals.md` and
`xtask/src/trust_generated.rs`.

Metadata preflight refuses reached build-script/procedural-macro targets. Before metadata or any root,
harness or separate generated-target build, checked removal clears the old Cargo home. Missing homes
are safe; other removal failures end unable to judge (exit 2), naming the home and I/O error. For example,
an undeletable readable home reports `cannot reset inventory Cargo home` before reusing its configuration.
Writable homes are recreated normally. Permanent public controls cover a wrapper that must never execute,
a file instead of a directory, and clean recovery. Repair, caller and local integration evidence:
`docs/specs/trust/flag-normalization-domain.md`.

Eight lint-level consuming-bit edits are scoped E for the pinned caller.
`cargo rustc -- -W unused_variables` distinguishes the changed helper outside it.
The [source ledger](ledger.md#rust-toolchain) gives actual argv and falsifiers. Other edits stay open.

### Source paths and dependency arguments

A compiler argument can contain text that resembles an option while still naming a source file.
Diagnostic options are removed by their option prefix; a substring inside a source filename remains
path data. The tests create a synthetic repository, whose root is denoted here by `<fixture>`.
Its binary source `<fixture>/crates/a/src/kept--json.rs` and library source
`<fixture>/crates/b/src/kept--error-format.rs` remain inventoried, and their configurations retain
those paths relative to `<fixture>`. Treating every argument that merely contains `--json` as a
diagnostic option would erase a real input identity.

A dependency argument separates the dependency name from its path at the **first** equals sign.
Further equals signs belong to the path. For example, a build under `scratch=one=two` still records
`--extern common=libcommon.rlib` for the binary and `--extern common=libcommon.rmeta` for the library.
The scratch directory and Cargo's artifact hash are removed from that identity; the dependency name
and library kind are retained. The ordinary `plain` output path is a positive control for the same rule.

Two permanent public inventory controls use real successful Cargo builds and these literal expected
arguments: `trust::tests::cargo_source_paths_with_diagnostic_substrings_remain_in_configuration` and
`trust::tests::cargo_dependency_paths_with_equals_preserve_extern_arguments`. Run either with
`cargo test -p xtask <test-name> -- --exact`. The tracked-source path grammar still applies: an equals
sign in a tracked source filename is refused before compilation; equals-bearing untracked build-output
paths are admitted. These examples establish the tested boundaries, while full mutation adequacy remains open.

### Inputs outside the written tree

Admitting an `include_str!` site lets compilation reach the input inventory. It does not make
external data part of the commit. Every file the compiler reads must still belong to the written
tree. The inventory reports each outside-tree dependency and writes no inventory on refusal.

For example, let `<run>` denote a generated test directory. Its written tree is `<run>/out/tree`,
while three admitted include sites read `<run>/external/a.txt`, `m.txt` and `z.txt`. The library
root named `chk` must produce the complete refusal list:

```text
trust-undeclared-input: `chk` reads `<run>/external/a.txt`, outside the written tree
trust-undeclared-input: `chk` reads `<run>/external/m.txt`, outside the written tree
trust-undeclared-input: `chk` reads `<run>/external/z.txt`, outside the written tree
```

The permanent parent `trust::tests::every_absolute_dependency_outside_the_written_tree_is_refused`
uses successful real Cargo compilation and this complete literal oracle. The actual dependency
record lists the ordinary source first, followed by those three external files; retaining just
one outside-tree refusal is insufficient. The existing admitted-site control still establishes
that a tracked include inside the written tree can be accepted. Run the new control with:

```bash
cargo test -p xtask trust::tests::every_absolute_dependency_outside_the_written_tree_is_refused -- --exact
```

The [full witness record](annex-trust-proofs.md#absolute-dependency-refusal-witness) binds the
producer order and the four initial branch mutations killed by this parent. Other path quoting,
raw/canonical geometry and the complete dependency-block checkpoint remain separately owned;
this bounded result does not establish full mutation adequacy.

Ambiguous records are refused (`2026-10-20`): the pinned compiler writes a prerequisite's spaces as
backslash-space and every other byte literally, so one dependency record can spell two prerequisite
lists. With a space in the written tree's path, an outside input whose name ends in a backslash,
followed by an in-tree prerequisite, reads as one escaped in-tree path. The inventory marks only the
written tree's known raw and canonical prefixes before splitting a rule, refuses a backslash anywhere
else, and requires the compiler's per-file targets, one per line, to re-spell exactly the list it
read. A record that fails either test is refused naming the raw rule, and no inventory is written:

```text
trust-undeclared-input: `chk` reads `crates/b/src/lib.rs <run>/prefix\ crates/b/src/tree/data.txt` ambiguously: its per-file targets spell the prerequisites otherwise
trust-undeclared-input: `chk` reads `crates/b/src/lib.rs <run>/external/a\ with\ space.txt <run>/external/m\ with\ space.txt <run>/external/z\ with\ space.txt` ambiguously: a backslash outside the written tree's prefixes
```

The permanent controls `trust::tests::a_trailing_backslash_before_an_in_tree_prerequisite_is_refused_as_ambiguous`,
`trust::tests::outside_inputs_with_spaces_are_refused_by_their_raw_rule`, under the raw output root and
under a symbolic link to it, and `trust::tests::a_symbolic_output_root_accepts_raw_and_canonical_in_tree_inputs`
hold these with real Cargo builds; `trust::tests::an_ambiguous_dependency_record_is_refused_by_its_raw_rule`
holds the record grammar on synthetic records. The
[repair record](annex-trust-proofs.md#dependency-path-domain-repair) retains the measurement.

A final prerequisite ending in a backslash never reaches the reader (`2026-10-21`): rustc writes the
backslash literally, and the pinned Cargo's own dependency-record reader takes a trailing backslash as
the escape of a next token that is not there, so the build fails as Cargo's internal error before the
inventory reads anything. The run reports that as the error it is, never as a refusal: a refusal is the
inventory's judgment of a record it read, and no input can be named without reading past the producer's
own refusal. Nothing is written, no refusal is listed, and the outcome is unjudged, not passed:

```text
`cargo build --lib --release --locked --offline --no-default-features --message-format=json -v -p b` failed: error: could not parse/generate dep info at: <run>/out/target/chk/release/deps/b-20572edba3a82d76.d
```

The permanent control `trust::tests::a_final_prerequisite_ending_in_a_backslash_is_the_builds_own_error`
holds that exact message naming the record, and holds that the record rustc left is one the reader
refuses as ambiguous, so the inventory does not rest on Cargo's refusal: a Cargo that stopped refusing
would fail the control, and the reader would refuse the record. The
[control record](annex-trust-proofs.md#dependency-terminal-backslash-control) retains Cargo's message,
its reader's text at the pinned commit, and the decision.

The whole dependency block is then held by its tests, not only the lines the repair changed (measured
`2026-10-21`): every mutation the sweep finds in the reader, its result type, the environment records'
normalisation and the unit reader's handling of the record — `64` of them — was run against every
current parent. `58` are killed, each by at least one of the block's own controls, and the compiler
refuses the other `6` as written, each with its actual error; nothing survives, nothing times out, and
no equivalence reason is used. The four edits the first assessment saw survive are among the killed. The
[checkpoint record](annex-trust-proofs.md#complete-dependency-block-checkpoint) lists every outcome. The
catalog builder reads its own builds' dependency records with a separate reader; that reader is
measured on its own (`M3.6.9.6.3.2.2.15.2.1.6.2.1`).

A unit's configuration is checked the same way: once normalised, an entry that still names a path
outside the written tree — one that begins with `/`, or one that holds `=/` — refuses the unit, each
entry once and in order:

```text
trust-undeclared-input: `b` in `chk` is configured with `-C link-arg=/outside/lib.o`, a path outside the written tree
```

No way of building a commit reaches this today (measured `2026-10-21`): the cargo configuration, the
workspace's `rustflags`, an inherited `RUSTFLAGS` and build scripts are each refused before anything is
compiled, and the pinned Cargo does not offer `trim-paths`. So the check is defence in depth, and it is
held directly: `checked_configuration` in `xtask/src/trust.rs` is the only way a unit's configuration is
had, and `trust::tests::a_configuration_naming_an_outside_path_is_refused_at_every_position` gives it
each form first, in the middle and last of a configuration, two at once, and none. Every mutation of it
is killed by that control or refused by the compiler; ten had survived every earlier test. The
[route record](annex-trust-proofs.md#configuration-absolute-path-routes) and the
[control record](annex-trust-proofs.md#configuration-refusal-held-directly) keep the measurements.

A package the commit does not hold is refused before anything is compiled (`2026-10-21`). A member
may depend on a path package outside the written tree — by an absolute path, or by one climbing out —
and Cargo's metadata lists it with its own manifest. The inventory used to read such a package's
directory as the empty one, the workspace root's, and let it through the manifest rules; the build then
stopped on compiler provenance, Cargo having compiled the outside crate from its own directory, and the
run ended unjudged, exit 2, as if the gate could not tell. It is the commit's own manifest that reaches
outside it, so it is now a refusal, exit 1, naming each such package:

```text
trust-undeclared-input: the commit reaches the package `x` at `<run>/outside-dependency-external/x/Cargo.toml`, outside the written tree
```

The permanent control
`trust::tests::a_path_dependency_outside_the_written_tree_is_refused_before_any_compilation` holds it
with real Cargo builds — an absolute path, a climbing one, two outside packages at once, and one under a
directory whose name repeats the tree's own path — and that nothing is compiled. The
[repair record](annex-trust-proofs.md#outside-path-dependencies-refused) keeps the measurement.

### Codegen options retain their operands

A `-C` compiler option and its following operand are recorded together as one configuration entry.
The operand is consumed once. It also carries information: removing the option would lose part of
what the compiler was asked to do, while retaining a second unprefixed copy would add a spurious argument.
The metadata, extra-filename and incremental options keep their separate normalization rules.

For example, a fixture's release profile sets `opt-level=1` and `codegen-units=3`. Its binary
configuration must contain `-C opt-level=1` and `-C codegen-units=3`, each once. A package override for
its library sets `opt-level=2` and `codegen-units=5`, so that library must retain the corresponding
`-C opt-level=2` and `-C codegen-units=5` entries instead. None of the four bare operands, such as
`opt-level=1` without its `-C`, is a second configuration entry for the unit it belongs to.

The permanent public control is
`trust::tests::cargo_codegen_arguments_remain_single_normalized_options`. It builds the committed
fixture with the pinned Cargo producer and checks literal profile expectations for both roots.
The current-token selection, retained-option deletion and one-token advancement edits each give
its finishing named failure. These three controls close measured parent gaps; they do not establish
whole-file mutation adequacy.

### The run's order

Initial cleanup removes the earlier inventory before judging forms. Failure ends unable to judge (exit 2),
naming path/I/O; old contents may remain. Fix the I/O error and retry. Ordinary refusals leave no inventory.
When inventory refuses, the gate removes its earlier `baseline-proposal.eadl` before publishing `report.txt`.
Proposal removal failure also ends exit 2, naming the path and preserving earlier files; no new report is written.
For example, a protected old proposal needs its deletion restriction repaired before retrying the gate.
The generated-sources record §2 orders the steps; each starts only if earlier steps refused nothing.
Build failures in steps 2 or 4 still finish that step's builds and judgments, then end unable to judge (exit 2),
with refusals reported.

1. Validate all forms before writing or building. Malformed `defgenerated` forms refuse; other malformed
   forms or reader syntax errors leave the invocation unable to judge, reporting generated refusals too.
   Reader recovery admits nothing: an extra `)` reports `read-unexpected-close`; a missing closing `)`
   reports `read-unclosed-list`. Neither starts a build.
2. Build the programs and judge the data their forms hand them, applying every prebuild/build refusal.
   Its refusals are reported together: a root's data and the launcher are judged first; a link or a submodule
   under `catalog/`, or a refused cargo configuration, ends the step at once, nothing after it being safe to
   judge; a package outside the tree, a stale form and the manifest rules are judged together, and none is
   followed by a build. A failure that leaves the gate unable to judge reports beside it the refusals already
   found (leaf `M3.5.2.5.2`).
3. Judge each live generated form's generators and inputs: each must be a commit blob, never a symbolic
   link, declared generated file or header-marked generated file. An unread form receives text checks only.
4. Judge every generator against every program reading its output. For a program-target entry point, use
   the union of its targets' role packages. A foreign role package refuses before building; otherwise,
   apply the manifest rules and build as for a root. Refuse a `.rs` generator with no program target,
   and a script in another role's package.
5. Refuse generated files read by generator builds (declared or header-marked), then marked files read by
   programs with neither a declaration nor a generator-read attribution: one message per file.

Only after all steps pass is an inventory written; only then can the gate find a form stale.

### Generated sources

These rules are the generated-sources record's, and its section numbers are used here.

**What a declaration says** (§2). One `defgenerated` form per generated file, named by the file's path. `generator`
names one or more generator files — scripts, entry points of program targets, or both when a script drives a program
target. `inputs` names the tracked files they read, none or more, and may be left out when there are none, so a table
computed from a formula in its generator needs none. `command` and `reason` each hold exactly one non-empty string,
and nothing executes either. Every path is the one the commit's tree spells, compared literally: `./scripts/gen.sh`
names no file. From the text alone, nothing built, the reader refuses every departure from that shape and these
constructions: a file two forms declare, both forms refused; an entry twice in one clause; a file both a generator and
an input; a declared file that is its own form's generator or input.

A form is **live** when a program — a root or the comparison harness, never a generator — reads the file it declares,
by its build or as data its form hands it at run time; the harness counts only through the units it compiles beside
its pair's builds. A form that is not live is held to its text alone. A live form's generators and inputs must each
be a file of the commit's tree and not a symbolic link; a declared file that is a symbolic link is refused as every
symbolic link a program reads is. A `defgenerated` form is a form of `trust/roots.eadl` like a root's: the change part
names it when it is added, changed or removed, the standing list names it, and it is accepted where reviews are
protected (`M3.6.5`), a change to it needing a new acceptance.

**How an undeclared one is recognised** (§3). A file is **marked as generated** when its header — the comments before
its first character that is neither whitespace nor inside a comment — holds, in any case, one of these markers:

<!-- machine-read: markers -->
`@generated` `do not edit` `automatically generated` `auto-generated` `autogenerated` `generated by`

The comments are read by the syntax this table gives the file's exact name, or else its extension — what follows the
name's last `.`, in any case, a leading `.` opening none, so `.env` is a name and `.gitlab-ci.yml`'s extension `yml`:

<!-- machine-read: comment-syntax -->
| Syntax | Extensions | Names |
| --- | --- | --- |
| `//` and `/* */`, nesting | `rs` `kt` `swift` `scala` | |
| `//` and `/* */` | `c` `h` `cc` `cpp` `cxx` `hh` `hpp` `hxx` `js` `ts` `go` `java` `proto` | |
| `/* */` | `css` | |
| `#` | `sh` `bash` `zsh` `py` `rb` `pl` `r` `toml` `yml` `yaml` `cmake` `mk` `env` `conf` | `Makefile` `makefile` `GNUmakefile` `Dockerfile` `CMakeLists.txt` `.env` |
| `;` | `eadl` `el` `lisp` `clj` `s` `asm` `ini` | |
| `--` | `sql` `lua` `hs` `ada` `adb` `ads` `vhd` `vhdl` | |
| `<!-- -->` | `md` `html` `htm` `xml` `svg` | |

A file with no extension whose first line is a `#!` is read with `#`; any other file the table does not reach is never
marked. A file is read as UTF-8, an invalid sequence as `U+FFFD`, a leading byte-order mark dropped; an unclosed block
comment runs to the end of the file; in an `<!-- -->` file the XML declaration may come before the comments. In a `.rs`
file, a `#!` first line rustc reads as a shebang is a comment, and one it reads as an inner attribute, `#![…]`, is code:
the rule is the pinned rustc's, held by a test that compiles each case with it (`the_shebang_rule_is_rustc_s`). The
recogniser is code, `xtask/src/generated_header.rs`. Its tests hold the markers and the table above, and the record's,
equal to its `MARKERS` and `TABLE`; every rule it follows is a row of the record's §3, each broken by a catalogued
mutation the tests must kill; and `scripts/mutation_sweep.sh` mutates its functions every way its operators know, every
mutant killed, timed out or failing to compile, but those listed, each with its reason, in
`xtask/generated_header.equivalents`.

A marked file that a program reads and that no form declares is refused, `trust-undeclared-input`: *"a generated
source with no declaration"*. A marker is advised, not required, in a declared file. A hand-written file whose header
uses a marker phrase in ordinary prose — "the code generated by the emitter" — has no honest declaration, and is
repaired by rewording its header.

**Refused, each with its reason** (§2; `docs/specs/trust/decision_trust-generated-refusals.md`, `M3.6.6.4`). Each is
refused, `trust-undeclared-input`, wherever the gate runs:

- a **chain** of generators: a live form's generator or input that a form declares or whose header marks it
  generated; and a declared or marked file that the build of a live form's program-target generator reads. Provenance
  is one step, each generator file judged against each program that reads its output; over a chain, which roles a
  file plays and which program each upstream generator answers to have a reading per step, and the design's review
  found another role's generator escaping through one. The need is met in one step: one form names every step's
  generator files and every committed file any step reads, the intermediate file never committed, and a marked input
  or script a form would name is remade so too, while a marked crate root has no route. A program-target generator
  whose build would compile a generated file reads another generator's output at run time instead, and a program that
  generates its own source gives way to another program target whose build does not compile it — a second executable
  of its package when neither a library there nor a package it depends on, directly or not, holds the file, else one
  of a package that does not depend on the package holding the file, directly or through another; one that must
  compile a generated file has no route;
- a live form's generator or input **at or under a gitlink**, the pointer by which the commit holds a vendored
  checkout: it is no file of the commit's tree, so the rule above refuses it — the inventory reads a program's files,
  and a live form's generators and inputs, from the commit's blobs alone. The need is met by committing a copy of the
  file, named as an input — or, for a generator, a script, an executable or a Rust source, as the generator, a Rust
  one as a program target; a copy whose header marks it generated is a chain, remade within the generation as above. A
  vendored file that cannot be copied, or a generator whose copy the role rules refuse, has no route: an input stays
  refused, and a generator is believed when a script runs it, its bytes, their sharing and its role unseen. Each route
  is held by a test the record names.

A tool a script runs — protoc, bindgen, an interpreter — is no generator file but the script's own dependency, and is
believed: a file a tool wrote is declared with the script that ran the tool as its generator. A file a tool writes
outside any committed script, as cargo writes `Cargo.lock`, has nothing a declaration could truthfully name;
undeclared, it is refused if a program reads it marked, and is a plain file otherwise. A tool stays believed: which
one runs is the host's, as the linker is, and as provenance it would make a form's digests facts no commit reproduces.
When a tool's identity must be compared, its pin — a file the script reads to choose or check it — is committed and
named as an input, and a marked pin is remade within the generation or gives way to an unmarked version file the
script checks; a tool from a vendored checkout is a vendored file, as above — copied, a generator file; uncopied,
believed, its identity compared the same way.

**Provenance, and the generated-provenance item** (§4, §5). A program's **provenance** is, for each live form whose
declared file it reads, the form's generator files and inputs, each recorded with its path, its sha256, its roles —
`generator`, `input` or both, across those forms — and the declared files through which the program reaches it, with
theirs. For each pair the inventory pairs, there is one generated-provenance item per file that one side's provenance
holds and the other side's provenance holds or the other side reads, its identity the path; and, as a copy is matched,
one per non-empty content of which each side has a path its provenance holds or it reads, when the two sides' sets of
such paths differ and one of the paths is a provenance file of its side, its identity the sorted union of both sets,
joined by a space. Its aspects are the content's sha256, each side's roles over its paths — `read` among them for a side
that reads one — and each side's declared files with their sha256. For the harness's own pair, the harness's provenance
is an aspect of the pair's comparison-harness item, as its files are. A third program reaching the same provenance makes
items in its own pairs and moves none of the others'. A file both sides read directly is a shared file item as before,
and a generated-provenance item too when a side's provenance holds it.

**What a generator file may be** (§5). Each generator file of each live form is judged against each program that reads
the form's file, and must lie outside that program's other roles — for the harness, outside every role but those of
its pair:

- the entry point of a program target, whatever its extension, is judged by that target's role packages, the union of
  them for the entry point of more than one target. Holding another role's, it is refused, `trust-shared-program`, and
  not built. Otherwise its target is built as a root is: the manifest and workspace rules over its closure first, then
  `cargo build --release --locked --offline --no-default-features -p <package>` with `--bin <name>`, `--example <name>`
  or, for a `cdylib`, `staticlib` or `dylib`, `--lib`, in the roots' environment; every `.rs` file it compiles held to
  the catalog's rules, a site it shares with a program admitted once; every file it reads held to recognition and the
  chain rule. Nothing of that build is recorded but its sites, counted with the programs', the admissions they use, and
  what recognition and the chain rule find, and no artifact is required of it; a build that fails leaves the gate unable
  to judge. A generator target that is itself a root is that root's own build. An entry point that is not a `.rs` file
  is refused before the build, `trust-undeclared-input`, as one in any program's closure is;
- a `.rs` file that is no program target's entry point is refused, `trust-undeclared-input`: only a program target's
  build is the inventory's to compute;
- any other file — a script — belongs to the innermost workspace member whose directory holds it, by whole path
  components, or to none; one lying in a role package of another role is refused, `trust-shared-program`.

**A form no program reads** (§6). When the run's steps all refuse nothing, a `defgenerated` form that is not live is
`trust-baseline-stale` on the baseline's host, and is removed. A form whose file a live form names as a generator or an
input, or a generator's build reads, is not stale: its run refuses, and that refusal is its one code.

**Case 5** (§7). Generated sources widen the sets case 5 is measured outside of ([§14.4's cases](#144s-cases)): beside
every root, program target and form of `trust/`, every provenance file and every unit a generator's build compiles. A
change inside them is judged by every rule above, as any change is, and reads *unchanged* only when the change part
names nothing. An edit to a provenance file, for one, can move an item the two sides share or make a copy between them,
move the harness's own pair's comparison-harness item by its content or by its provenance, change a generator's build,
judged as above, be refused — removed, made a symbolic link, marked generated — or leave the gate unable to judge when a
build it needs fails.

### The codes

| Code | Outcome | When |
| --- | --- | --- |
| `trust-new-shared` | reported | an item the base commit's baseline does not hold, a generated provenance among them |
| `trust-shared-changed` | reported | an item whose content, configuration, edges or readers differ from the base commit's form — an invocation's launcher among them — or a generated provenance whose content, roles or declared files do, or a comparison's, a harness's or a derivation's, whose provenance does |
| `trust-unclassified-program` | reported | a program target neither a root nor classified, or whose classification's role packages grew — a test-like unit whose build joins two roles a property rests on among them, unless a pair form names it |
| `trust-baseline-stale` | refused, on the baseline's host | a form whose item is no longer shared, a classification whose target is gone or no longer joins two roles, an admission no site uses; and, wherever the gate runs, a form naming a package or target the commit no longer has; a `defgenerated` form whose file no program reads, on the baseline's host |
| `trust-undeclared-input` | refused | an input the commit does not hold, or a construct the catalog's rules refuse, with no admission, in a program's build or a generator's; a `defgenerated` form departing from its shape, or a construction its text alone refuses — a file two forms declare, an entry twice in a clause, a file both a generator and an input, a declared file its own generator or input; a live form's generator or input that is no file of the commit's tree — one at or under a gitlink among them — or is a symbolic link, or that a form declares or a header marks, a chain; an entry point that is not a `.rs` file, in a program's closure or a generator's; a `.rs` generator file that is no program target's entry point; a file a program reads that is marked generated and no form declares; a declared or marked file a generator's build reads; a front that invokes while its commit holds no launcher |
| `trust-form-missing` | refused, on the baseline's host | a shared item with no form in the commit's own baseline, or with one proposing other digests than the commit measures; a program target with no classification — a joined unit among them, unless a pair form names it, the remedy naming its pair's form |
| `trust-shared-program` | refused | a root compiling another role's role package, or a comparison — the harness or a derivation — one of a root outside its pair, by an edge or by a file its own build read (leaf `M3.5.2.5.3`); a live form's generator file that is the entry point of a program target whose role packages hold another role's than the reading program's — for the harness, than its pair's — or a script lying in a role package of such a role |
| `trust-inventory-stale` | refused, by `trust-verify` | a package's inventory missing or of another build, its report of another inventory, an artifact or a result's producer not the inventory's, a result's role no root's or its producer a derivation, a handed dependency undeclared |

**Reported** passes and lists the item unreviewed; **refused** fails the gate wherever it runs, and is repaired by a
change to the code or to `trust/`.

### One host

A form's digests hold only on the host they were taken on: the build configuration every pair shares holds the
toolchain's identity, and `rustc -vV` names its host. The baseline names its host, the CI runner's
(`x86_64-unknown-linux-gnu`). On another host the gate builds and applies every refusal but `trust-baseline-stale` and
`trust-form-missing`, lists every item unreviewed, and its change part says *not compared*, never *unchanged*.

### §14.4's cases

Each is a test of the gate in a scratch workspace, judged commit by commit by the real instrument (`M3.6.3.6`):

| Case | What changes | What the gate says |
| --- | --- | --- |
| 1 | a package both roots come to compile | `trust-new-shared`; `trust-form-missing` until its form is committed |
| 2 | a shared package's source edited, a feature of it activated | `trust-shared-changed`, naming the aspect |
| 3 | a data file both roots are handed; an input declared but absent | `trust-new-shared`; `trust-undeclared-input` |
| 4 | a package carrying an earlier commit's inventory | `trust-inventory-stale` |
| 5 | a README, a development profile, an override for a package no root compiles, a comment in the pin | *unchanged*, nothing refused |

Case 3 for a generated source is a test of the inventory: two tables of different names and bytes, made by two
generators from one input, share that input as a generated-provenance item, the only item beside the build configuration
(`case_3_two_generated_sources_sharing_an_input_through_two_generators_share_it`). The gate's tests take it on: an edit
to a generator both tables' forms name is `trust-shared-changed`, and one to a script that no form names and no program
reads is *unchanged* (`an_edit_to_a_declared_generator_is_reported_and_to_an_undeclared_script_is_unchanged`); and a
`defgenerated` form added, changed or removed is in the change part, every one standing.

Case 5 is a change to nothing the gate reads beyond the roots, on the baseline's host: the pin, the cargo
configurations on the build's path, each member's manifest, the tree's entries under `catalog/` and the harness's
units are read too, and judged (`M3.6.7`).

Every code is removed in turn by a catalogued mutation the tests must kill, and a test holds the catalogue to the
record's table, so a code the record gains with nothing removing it fails.

### Where it runs

- In CI, `.github/workflows/trust-gate.yml`, on every pull request, the merge queue and every push to `main`, with no
  path filter. The gate is built from the base commit by the catalog's harness, so a change to the gate is judged by
  the gate it changes nothing of ([Verifying the toolchain](verification.md)).
- In the `assurance` tier's `trust-inventory` step, which fails on a refusal and is otherwise *not built*: a pass
  would claim forms accepted, and none can be before `M3.6.5`.
- In packaging, `trust-verify`, once `M4.7` writes packages.

### What it is not

A proof of semantic independence. Different crate names do not establish independent derivation; a copy edited after
copying, or a formula re-derived by hand, is not seen. The gate reads `trust/` with the generator's own reader, so a
fault there that misreads a form is shared with the generator, which is why a form's review reads its text.

Every report ends with what the inventory does not see, the same list each time, each entry in short (`UNSEEN`,
`xtask/src/trust_gate.rs`); here it is in full:

- the linker and the host's C toolchain, recorded by their versions alone;
- code a `cfg` gates to a target other than the host;
- what a root reads at run time that the pipeline does not hand it;
- **a declaration is believed, not verified.** No generator is run to check that a declared file is its output, that
  it read only its declared inputs, or that the generator files named are all there are: a script that drives another
  role's program target, declared with the script alone, escapes the generator judgments, and so does a tool a script
  runs. A wrong declaration is the review's to catch, reading the form and its command: the standing list names
  every form, and `M3.6.5` requires it accepted; generating again in CI would be a check of its own;
- **an undeclared generated file the recogniser does not mark**, judged as a plain file: one whose generator writes no
  marker; one whose name or extension the table does not hold, unless it has no extension and its first line is a `#!`
  — a `.txt`, `.lock`, `.json` or `.csv`, a binary, a linker script, a `.cs`, `.tsx`, `.jsx` or `.mjs`; one whose
  marker comes after content, a Markdown heading or YAML's `---`; one marked in a comment syntax the table does not
  read — SQL's `/* */`, GNU assembler's `#` and `/* */`, on any line, or a later line of Lua's `--[[ ]]` or CMake's
  `#[[ ]]`;
- **a generator's own dependencies** — a script's interpreter, the modules a generator's build compiles beside its
  entry point — which are no provenance: a generator target is hashed by its entry point alone, so an edit elsewhere in
  its build changes no item until its output is generated again, which then does;
- **what a program reads on another host.** A module compiled only under another target's `cfg` is read on that host
  alone, so on the baseline's host its form is stale, and off it the file, marked and undeclared, is refused.
  Admissions meet the same limit.
- **what a derivation reads at run time**, as a comparison test reads the kind modules from disk;
- **the transcriptions a derivation holds together.** The checker's model lives in the checker's package and, from
  `M3.4`, the generator's relation in the generator's, so an edit to either moves no shared item — before `M3.4` the
  relation, compiled beside the pair's builds, is in the derivation's item: only the comparison failing joins them,
  which is why the relation's derivations are reference derivations, their controls the ones independent of the
  transcription;
- **whether the front runs only, or all, the programs its form invokes.** From leaf `M3.5.1.4`, `NO-SUBPROCESS`
  admitting the launcher alone, the launcher running only what the install manifest names, and the install placing only
  the invoked roots stand there; until then no front invokes;
- **whether a unit two roles' packages join compares them.** The build finds the joining; a classification's review
  decides that the unit compares nothing the two hold equal;
- **an edit inside a classified program target.** A classification is reviewed again only when the role packages its
  build compiles grow, so an edit that makes a unit start comparing two roles reads "unchanged" (`M3.6.10` owns it);
- **what the census build does not compile.** It builds each package's targets as `cargo test` builds them, on this
  host, and each target that skips by name with its required features: a file only another profile, platform or
  feature set compiles — `cargo test --release`, `cargo bench` — what a doc test's own snippet includes, a package
  holding or reaching a build script or a procedural macro — left out, so none runs, and judged by its edges — a
  non-member's features and another workspace's tests are outside it. A census build that fails stops the run, unable
  to judge, naming its package;
- **a program a test runs.** A test that runs another role's program as a process compiles none of it, so the census
  does not see it.

## Today and ahead

Measured `2026-10-10`: the roots are the generator, the scheduling checker, the reference model and the implementation,
with the comparison harness beside them, and what they share is every pair's build configuration and the harness the
reference model and the implementation share. The configuration checker has no root yet (`M3.5`). The runner's baseline
is committed (`M3.6.3.2.1`): the proposal of the `trust-gate` workflow's first run that judged, a form for each of
the nine shared items, unaccepted, its host `x86_64-unknown-linux-gnu`. On any other host the gate passes with *not
compared*, since the digests compare only on the baseline's. The workflow's very first run could not judge: the gate
is built from the base, and that run's base was older than the gate's command, a case no later push meets. Acceptance — who may accept a form, read on the protected main line —
waits on the director's protection of `main` and a second reviewer (`M3.6.5`). A committed generated source, which
shares its generator's mistakes with whatever reads it, is declared, recognised, judged and shared as this chapter
states (`M3.6.6.2`). Measured `2026-10-10`: `trust/roots.eadl` declares none, no program reads a file the recogniser
marks, and every program's provenance is empty. A chain of generators and a generator or input in a vendored checkout
stay refused, and a tool a script runs believed and named in every report, each for a reason and, where one exists, with
a route within one step (`M3.6.6.4`, reviewed by a context that did not write it, its review closed `2026-10-10`).
Measured `2026-10-23`: the census of test-like units finds `crates/rt-core`'s `lifecycle` test, its library's unit tests
and its doc tests joining the reference model and the implementation beside the harness; their classifications land
after the first push that brings the census to `main` and before the one after it (`M3.5.8`).

The accepted mutation criterion, measured controls and exact command examples are in
[Annex C](annex-trust-proofs.md#mutation-measurements-and-controls). Historical results retain their
source and limits; final production-sweep adequacy remains open. **Mutation testing is postponed** by the director
on `2026-10-21` until the project's functionality is complete and locked, then done extensively in one campaign
(`docs/decisions/decision_mutation-testing-after-completion.md`): a result is bound to the exact source it ran against, so measuring code
that `M3.3` to `M7` will still change would pay for most of it twice. Until then each feature keeps its own tests,
a commit that changes Rust runs the focused tier, and every push to `main` runs the full CI and this gate, which
judges every commit the push brings; a real defect is still fixed.

The package the verifier
reads is fixed provisionally until `M4.7` writes real ones.
