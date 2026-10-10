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

A program that would run two roles is refused: one program cannot be independent of itself. Every other program the
workspace can build — an example, a diagnostic tool, the browser module — is classified as not a root, with a reason
and the role packages it compiles, so no one can add a second checker beside the generator unseen.

### The inventory

`cargo xtask trust-inventory` writes the commit's tree from git's blobs, builds each root in the release profile,
clean, and reads the compiler's own record of every file it read. Every source a root compiles is held to the
catalog's rules: a construct that could read something the commit does not track — an `include!` of an untracked
file, an environment variable, assembly pulling in bytes — is refused unless a reviewed **admission** names it.

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
under two generators, a generator both use, a data file one reads that the other's source was made from, or a copy of
an input under another name. Two generated tables that differ in every byte, written by one script from one data file,
are seen to share both. A generator or input the commit does not hold as a file is refused. So are a **chain**, a
generator or input that a form declares or a header marks as generated, and a generator or input inside a vendored
checkout, which the commit holds as a pointer to another repository rather than as files: each is refused for a reason,
and the need it would serve is met in one step instead where a route exists (below).

A generator must lie outside the reading program's other roles: a table the generator's own executable wrote, read by
a checker, is refused, since the checker would then trust the very logic it checks. A generator that is a program —
an executable or an example — is built as a root is, its sources held to the same rules and its admitted sites counted
with the roots', unless it is a root, whose build it already is; a script is judged by the package that holds it; a
Rust file that is no program's entry point is refused, since only a program's build can be computed.

A generated file that does not declare itself is recognised by its header: its opening comments saying `@generated`, *do
not edit*, *generated by* and their kin, read by the comment syntax of the file's name or extension
(`xtask/src/generated_header.rs`). A program that reads such a file with no form declaring it is refused, and so is a
generator whose own build reads a declared or marked file — a generator made by another generator, a chain again. The
recogniser sees only what a generator chose to mark, in a kind of file it knows: a table written with no marker, a
`.json` or `.lock` file, a binary, or a marker below the file's first content passes as a hand-written file, and is
judged as one. So a marker belongs in a generated file's very first comment, where a later copy that nobody declared is
still caught.

A form whose file no program reads any more is stale on the baseline's host, and is removed, so a generated source
read again is reviewed again.

For each pair of roots it then lists what they share, each item with the sha256 of its **content**, its
**configuration** (features, `cfg`, edition, as each root's own build sets them), its **edges** (who depends on it)
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
run time was declared.

## The precise rules

The design is `docs/specs/trust/decision_trust-inventory.md`, reviewed round by round until a round found no defect;
its section numbers are used below. The code is `xtask/src/trust.rs` (the inventory), `xtask/src/trust_gate.rs` (the baseline and the
gate) and `xtask/src/trust_verify.rs` (the verifier); committed generated sources are
`docs/specs/trust/decision_trust-generated-sources.md`, `docs/specs/trust/decision_trust-generated-refusals.md` and
`xtask/src/trust_generated.rs`.

### The run's order

The inventory decides in steps, each running only when those before it refused nothing (the generated-sources record
§2): first the forms' text, nothing built; then the programs' builds and the data their forms hand them, with every
refusal the rules below make before or in those builds. A step that refuses ends the run and leaves no inventory — not
even one an earlier run wrote, which would read as this commit's. A step in which a build fails still runs to its end,
every build in it made, and then ends unable to judge (exit 2), the refusals it found printed beside the failure. A
form of `trust/roots.eadl` other than a `defgenerated` one that departs from its shape also leaves the gate unable to
judge, the `defgenerated` refusals read before it printed beside it. The third step judges every form a program
reads: a generator or input that is a symbolic link or no blob of the commit is refused, and so is one a form declares
or whose header marks it generated, a chain; a form no program reads is held to its
text alone. The fourth judges each such form's generator files against each program that reads it: a program
target's entry point whose role packages, or the union of them when it is the entry point of more than one target, hold
another role's is refused and not built, and is otherwise built and checked as a root is, the manifest rules first;
a `.rs` file that is no program target's entry point is refused; a script lying in another role's package is refused.
A build that fails there ends the run unable to judge, as in the second step. The fifth refuses a file a generator's
build reads that a form declares or whose header marks it generated, then a file a program reads whose header marks it
generated and that no form declares — one message for one file. Only when all five refuse nothing is the inventory
written, and only then can the gate find a form stale.

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
  generator files and every committed file any step reads, the intermediate file never committed — a program-target
  step reads it at run time rather than compiling it, and one that must compile another generator's output has no
  route;
- a generator or input **at or under a gitlink**, the pointer by which the commit holds a vendored checkout: it is no
  file of the commit's tree, so the rule above refuses it — the inventory reads a program's files, generators and
  inputs from the commit's blobs alone. The need is met by committing a copy of the file, named as an input — or, for a
  generator, as the generator, a Rust one as a program target; a vendored executable that cannot be copied is believed
  when a script runs it, its bytes and their sharing unseen. A copy, or a pin below, whose header marks it generated is
  a chain, with no route.

A tool a script runs — protoc, bindgen, an interpreter — is no generator file but the script's own dependency, and is
believed: a file a tool wrote is declared with the script that ran the tool as its generator. A file a tool writes
outside any committed script, as cargo writes `Cargo.lock`, has nothing a declaration could truthfully name;
undeclared, it is refused if a program reads it marked, and is a plain file otherwise. A tool stays believed: which
one runs is the host's, as the linker is, and as provenance it would make a form's digests facts no commit reproduces.
When a tool's identity must be compared, its pin — a file the script reads to choose or check it — is committed and
named as an input; a tool from a vendored checkout is a vendored executable, as above.

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
reference model and the implementation share. The configuration checker has no root yet (`M3.5`). The gate passes with
*not compared*, since no baseline is committed yet. The runner's baseline is proposed by the `trust-gate` workflow's
first run, and committed then (`M3.6.3.2.1`). Acceptance — who may accept a form, read on the protected main line —
waits on the director's protection of `main` and a second reviewer (`M3.6.5`). A committed generated source, which
shares its generator's mistakes with whatever reads it, is declared, recognised, judged and shared as this chapter
states (`M3.6.6.2`). Measured `2026-10-10`: `trust/roots.eadl` declares none, no program reads a file the recogniser
marks, and every program's provenance is empty. A chain of generators and a generator or input in a vendored checkout
stay refused, and a tool a script runs believed and named in every report, each for a reason and, where one exists, with
a route within one step (`M3.6.6.4`, its review by a context that did not write it under way). The package the verifier
reads is fixed provisionally until `M4.7` writes real ones.
