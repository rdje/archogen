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

`defroot`, `defharness`, `defprogram` and `defrole` place a symbol or string name immediately after the head.
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
including in a harness between valid ones.

A program that would run two roles is refused: one program cannot be independent of itself. Every other program the
workspace can build — an example, a diagnostic tool, the browser module — is classified as not a root, with a reason
and the role packages it compiles, so no one can add a second checker beside the generator unseen.

### The inventory

`cargo xtask trust-inventory` writes the commit's tree from git's blobs, builds each root in the release profile,
clean, and reads the compiler's own record of every file it read. Every source a root compiles is held to the
catalog's rules: a construct that could read something the commit does not track — an `include!` of an untracked
file, an environment variable, assembly pulling in bytes — is refused unless a reviewed **admission** names it.

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
| infrastructure | no — a shared allocator cannot make two programs agree on a wrong answer |
| interpretation / normalization | yes — it shapes what both sides *see* |
| semantic analysis | yes — it shapes what both sides *conclude* |
| authoritative data | yes |
| reference derivation | yes |

`cargo xtask trust-baseline --propose` writes the forms with the review's part `unstated`. Nothing in the file can say
a form is accepted: acceptance is read where reviews are protected, never from a form's presence.

### The gate

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
run time was declared. Its manifest and inventory JSON require valid UTF-8.

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

### The run's order

Initial cleanup removes the earlier inventory before forms are judged. Failure ends unable to judge (exit 2),
naming the path; old contents can remain. Fix that I/O problem and retry. Refusals leave no earlier inventory.
The generated-sources record §2 orders the steps; each starts only if earlier steps refused nothing.
Build failures in steps 2 or 4 still finish that step's builds and judgments, then end unable to judge (exit 2),
with refusals reported.

1. Validate all forms before writing or building. Malformed `defgenerated` forms refuse; other malformed
   forms or reader syntax errors leave the invocation unable to judge, reporting generated refusals too.
   Reader recovery admits nothing: an extra `)` reports `read-unexpected-close`; a missing closing `)`
   reports `read-unclosed-list`. Neither starts a build.
2. Build the programs and judge the data their forms hand them, applying every prebuild/build refusal.
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
| `trust-shared-changed` | reported | an item whose content, configuration, edges or readers differ from the base commit's form, or a generated provenance whose content, roles or declared files do, or a comparison harness whose provenance does |
| `trust-unclassified-program` | reported | a program target neither a root nor classified, or whose classification's role packages grew |
| `trust-baseline-stale` | refused, on the baseline's host | a form whose item is no longer shared, a classification whose target is gone, an admission no site uses; and, wherever the gate runs, a form naming a package or target the commit no longer has; a `defgenerated` form whose file no program reads, on the baseline's host |
| `trust-undeclared-input` | refused | an input the commit does not hold, or a construct the catalog's rules refuse, with no admission, in a program's build or a generator's; a `defgenerated` form departing from its shape, or a construction its text alone refuses — a file two forms declare, an entry twice in a clause, a file both a generator and an input, a declared file its own generator or input; a live form's generator or input that is no file of the commit's tree — one at or under a gitlink among them — or is a symbolic link, or that a form declares or a header marks, a chain; an entry point that is not a `.rs` file, in a program's closure or a generator's; a `.rs` generator file that is no program target's entry point; a file a program reads that is marked generated and no form declares; a declared or marked file a generator's build reads |
| `trust-form-missing` | refused, on the baseline's host | a shared item with no form in the commit's own baseline, or with one proposing other digests than the commit measures; a program target with no classification |
| `trust-shared-program` | refused | a root compiling another role's role package, or the comparison harness one of a role outside its pair; a live form's generator file that is the entry point of a program target whose role packages hold another role's than the reading program's — for the harness, than its pair's — or a script lying in a role package of such a role |
| `trust-inventory-stale` | refused, by `trust-verify` | a package's inventory missing or of another build, its report of another inventory, an artifact or a result's producer not the inventory's, a handed dependency undeclared |

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
a route within one step (`M3.6.6.4`, reviewed by a context that did not write it, its review closed `2026-10-10`). Whether the instrument's tests hold the generated-sources rules — what a program reads, the chain, the blob rule,
each generator's role against each reader — wherever the code judges them, not only at the instances they build, is to
be shown under an accepted design (`M3.6.9`, review closed at round 3): a source-linked census of 32 proposed route/parent mutation pairs and
sweeps of the complete production ranges in `xtask/src/trust.rs`, `xtask/src/trust_generated.rs` and
`xtask/src/trust_gate.rs`. Each exact parent mutation must be killed by finishing parent tests; every sweep survivor
needs an equivalence reason, and timeouts fail both strict modes. The design records actual operator reach and binds
the test copy to its source repository. Implementation begins with strict and attributed mutation outcomes: an
invocation error without a failed test summary and failure-list name is unjudged, never a kill or equivalent.
Those modes are built in `M3.6.9.1`; `M3.6.9.2.1` adds source-span replacements and validated multiline equivalents.
`M3.6.9.2` builds all five additional operators and actual per-operator compiler-refusal counts, with compiled
semantic and CLI restoration controls described in `docs/book/src/verification.md`. `M3.6.9.3.1` adds explicit
configuration-root lists: each outer configuration belongs to its innermost supplied root and must still match
the judged tree's tracked bytes and alias-only content rules. Intermediate target configurations and ancestors
above all supplied roots remain refused. The production gate supplies its repository alone. The tests' seam
(`M3.6.9.3.2`) admits `ARCHOGEN_SWEEP_SOURCE` only after canonicalizing it and the test root, verifying that the
source is its own Git top-level, and requiring the test root to be exactly its `target/mutation-sweep/tree` child.
Canonical aliases are accepted; unrelated roots, enclosing ancestors, deeper copies and redirected physical
locations refuse before inventory. The source hint cannot change production's root list. The sweep now supplies
its canonical own source root for every test command, replacing any inherited hint (`M3.6.9.3.3`). All three parent
populations pass in the actual unmutated copy with an invalid inherited hint; that zero-attempt baseline establishes
copy integration, not mutation adequacy. `M3.6.9.4.1.1.1` adds three source/provenance route controls: a live input
link retains its specific refusal, every live form contributes provenance, and middle generators remain present
beside their inputs. Each exact catalogue mutation compiles and is killed by a named finishing route test.
The input-link mutation changes the diagnostic alone: the original refuses a symbolic link, while the mutant still
refuses it as no blob because the regular-file tree excludes symlinks. It does not show changed link acceptance.
The reader/role controls (`M3.6.9.4.1.2`) hold every role package a generator target's build reaches, including a
foreign package after an allowed one; a harness may use both pair roles but refuses a third role's script or
program target; and a reader handed generated runtime data still judges its generators. Each of the four exact
mutations is killed by a named finishing route test. Build-read controls (`M3.6.9.4.1.3`) hold two generators
reaching a shared dependency through another package: plain reads pass; marked and declared reads refuse with
both generators named. Naming the first alone still refuses but loses attribution; dropping transitive units
admits the marked dependency in the native fixture. Both exact mutations produce named finishing kills.
All nine route additions are built. `M3.6.9.4.2.1` adds the 32 exact parent twins, each retaining the expected
killed criterion. The catalogue contract reads the design's census and frozen finding rows, holding exact
population membership, commands/verdicts and each twin's file/from/to identity. Independent falsifications
and a named compiled guard mutation hold those checks. Run the contract with:

```bash
cargo test -p xtask mutation::adequacy_tests
```

This verifies the catalogue, not semantic adequacy. The full strict route run now has 32 named finishing kills
and exact source restoration (`M3.6.9.4.2.2.1`); the [measurement chapter](trust-adequacy.md) records each actual
killing test and the source hashes. The initial parent baseline passed 91 tests; 2 of the 32 exact mutations had named
kills and 30 survived (`M3.6.9.4.2.2.2`). After the required initial operator measurements, parent controls repaired
those gaps. On `2026-10-11`, the combined final exact run kills all 32 entries with finishing named parent tests, strict exit 0
(`M3.6.9.6.3.1`). These historical measurements remain separate from full production-sweep adequacy.

The first full-range `trust.rs` attempt lists 1,420 candidates, lines 144–2493. Its baseline passes; it stops after 150: 78 named kills, 62 survivors, five compiler refusals, four strict timeouts and one unjudged result, raw exit 2. The incomplete-result auditor refuses; all captured original/copied inputs restore. This is not a full measurement or adequacy pass.

The five non-finishing candidates remove index progress in configuration normalization. A compiler-backed diagnostic
uses each actual edit with a trace that stops before a fourth loop-body evaluation: the baseline visits index 1 once,
while each mutant revisits it; the final mutant also appends the same flag repeatedly. Both deliberately wrong trace
oracles fail. This establishes non-progress, without assigning an OS crash cause to the unjudged command.

```console
bash scripts/trust_adequacy.sh --progress --output target/trust-progress.json
bash scripts/trust_adequacy.sh --self-test
```

The parent tests now execute their unchanged bodies in bounded children selected by the same exact libtest names
(`M3.6.9.6.3.2.1.2`). Each parent requires a successful child exit, one named passed test and a finishing single-test
summary. Zero selected tests, assertions, abrupt exits and a child that exceeds its limit become parent failures.
Child diagnostics are escaped so their failure lists cannot supply names to the outer native result. The test-only
worker in `xtask/src/trust_test_process.rs` reuses the mutation runner's direct system-call group cleanup.

The command's deadline stays active until its child exits and both captured output pipes close. An exited child
whose descendant retains stdout or stderr cannot bypass the deadline: descendants still in its process group
are stopped and the command reports a timeout. The leader stays unreaped while its pipes are open, retaining its
identity for group cleanup. A descendant that creates another group or session leaves this cleanup's scope and
can retain a pipe; independently grouped parent workers must finish under their own limits.

Exact guard reversal bypasses a one-second limit; reapplication times out near one second (`M3.6.9.6.3.2.1.5`, `2026-10-11`). Run the actual runner controls:

```bash
cargo test -p xtask mutation::tests::an_exited_command -- --nocapture
```

All 103 original parent names/bodies remain apart from the guards (`2026-10-11`). Workers have 30s bounds in their own groups. The selected native regressions use 600s per candidate on this host to allow successive waves; observed budgets do not relax strict timeout rules.

```bash
cargo test -p xtask trust_test_process::tests
ARCHOGEN_PARENT_TEST_TIMINGS=1 cargo test -q -p xtask -- \
  trust:: trust_generated:: trust_gate:: --skip route_tests --nocapture
```

Measured `2026-10-11`: the five non-progress mutations gain named finishing kills, strict 0; all 32 exact entries retain their killing-name sets. Inputs restore, production prefixes stay unchanged and focused gates pass. These repair regressions precede the complete measurement below.

Measured `2026-10-11` at source 36154d6: the complete `trust.rs` retry attempts all 1,420 production candidates with every operator, the original 103-name parent population and no equivalents file. Its native baseline passes in 17s. Results are 802 finishing named kills, 540 survivors and 78 compiler refusals, with no timeout or unjudged result. The native strict exit is 1: this is a complete measurement, not an adequacy pass (`M3.6.9.6.3.2.1.3`).

```bash
bash scripts/mutation_sweep.sh xtask/src/trust.rs --package xtask \
  --timeout 600 --timeouts-fail --filter=trust:: \
  --filter=trust_generated:: --filter=trust_gate:: --filter=--skip --filter=route_tests
```

Measured `2026-10-11`: actual identities, replacements, killing names and counts agree; altered evidence refuses. The first attempt's measured prefix is retained, except its five non-finishing cases now have named kills. Compiler refusals prove no coverage. All 540 initial survivors are owned in `docs/tasks/M3.md`; each needs a repair or justified per-file equivalence before the final strict sweep.

Initial `.1`–`.7` assessments (`2026-10-11`) repair malformed names/placeholder leaves and reader/role/pair test gaps; committed records hold named kills, reversals and source boundaries.

Measured `2026-10-11`, `.8.2`: the ends rewrite preserves order/early returns for both borrowed String references, with no user callback/destructor effect. Alias, wider-array and wrong-oracle controls support the scoped E: native survivor (strict 1), then reviewed equivalent (strict 0). Production stays unchanged; classification adds no kill.

Historical selected runs restore 988 captured inputs under Python/system hashes; the new E record is separately hashed. Commands/identities live in committed leaves. Listing `2026-10-11`: 1,416 candidates, production lines 144–2498. IDs require recorded source or fresh discovery; final exact/full current-source outcomes remain open.

Measured `2026-10-11`, `.9.1`: four environment survivors gain named kills, strict 0. Public controls hold three compiled values and record boundaries; production/old tests stay unchanged and all 990 captured inputs restore.

Measured `2026-10-11`, `.9.2`: marker-bearing values exposed a configuration-key collision. Keys now use
JSON `[command_tokens, environment_records]`, e.g. `[["--crate-name","common"],["CARGO_PKG_DESCRIPTION=alpha env-dep:omega"]]`,
preserving boundaries/order/empty values and public raw-array shapes. The key-only native reversal gains a
named public kill; all 990 inputs restore and domain/focused controls pass. Changed configuration digests
require Linux proposal compatibility in `M3.6.3.2.1.1`, pending an authorized runner/base checker and review.
The reproducer also changes content; it proves the field defect, not a complete-gate bypass.

The [tooling JSON contracts](annex-repository.md#json-used-by-the-tooling) separate encoding from reader diagnostics. Number grammar, raw controls, Unicode escapes and inventory-byte admission are repaired; `.9.4` integration is next. These defects do not establish an artifact-hash bypass.

The [focused-stamp guard](annex-repository.md#a-commit-holds-what-was-run) now holds explicit successful-exit claims and large diffs; `.9.5` records its RED controls and repair.

Timeout and unjudged results remain failures. Full adequacy and independent review stay open.
Recorded native killing names remain accessible through the original `docs/tasks/M3.md` leaf stubs, which now
route completed evidence to sealed task-history records. The stubs retain commit identities; the archive index
holds content digests, and the task-history gate proves reconstruction and provenance.
The package the verifier
reads is fixed provisionally until `M4.7` writes real ones.
