# Committed generated sources: declared, recognised, and shared through their provenance

- **Type:** `decision`
- **Date:** `2026-10-06`
- **Status:** `active` — written `2026-10-06` by leaf `M3.6.6.1`; rounds 1 to 4 answered the same day; under review,
  by a context that did not write it, until a round finds no defect (`decision_executable-design-reviews.md`)
- **External sources:** [the pinned Rust toolchain](../../book/src/ledger.md#rust-toolchain) — rustc's dependency
  information, which, with a root's declared run-time data, is what the inventory knows of the files a program reads;
  and how the pinned rustc reads a `#!` line, which §3's recogniser follows (measured below)
- **Owner / source:** leaf `M3.6.6` (`docs/tasks/M3.md`), filed by `M3.6.1`'s fourth review; the gap is
  `decision_trust-inventory.md` §3's last paragraph, and `ROADMAP.md` §14.4's case 3, "a shared transitive semantic
  helper, generated-source input, or authoritative data source used on both sides". The trust inventory's terms — root, the comparison harness, role package, program target,
  shared item, form, the baseline's host — are that record's, cited below as "the parent".

## The fact / decision

### 1. The gap, measured

The inventory judges every file a program reads as a file: a path and its bytes (parent §3, §4). A source committed as
a generator's output is such a file, and two sources one generator wrote for two roots — differing in bytes, perhaps
only in a name — share nothing the inventory sees, though a fault in the generator, or in an input it read, is in both.
The generator and its inputs are a provenance no build reads.

A **program** is a root or the comparison harness; a **generator file** is a file a form names as its generator —
a script, or a program target's crate root (§5) — and "program" never means one. A program **reads** a file when its
compilation reads it (rustc's dependency information) or its form hands it at run time (the parent's `(data …)`
clause, §2, §3). For the harness, both what it reads and what it reaches (§4) count only through the units it compiles
beside its pair's two builds — the units that make its files part of the pair's comparison-harness item (parent §4).

**Measured `2026-10-06`**, as the instrument's own output: the inventory lists, as `generated-marked`, every file a
program reads that §3 marks, and on the real tree that list is empty — its roots read 56 workspace files; the harness
reads 1 beside its pair's builds and 5 in its whole build; 57 in all. The recogniser's reach is narrower than "every generated file": of the four tracked files that
say in their first lines that a tool wrote them, it marks none — `Cargo.lock` and `targets/riscv-virt-up/spike/Cargo.lock`
(no comment syntax), `docs/semantics/BASELINE.txt` (none) and `KNOWLEDGE_MAP.md` (its comment follows a heading). No
program reads any of the four. So no *recognised* generated source reaches a program today; what the recogniser does
not reach is stated in §8.

This design is deliberately narrow: it decides one generator step, from committed files to a committed file. What it
does not decide it refuses (§2), and leaf `M3.6.6.4` owns deciding it.

### 2. The declaration

A committed generated source is declared in `trust/roots.eadl`, read by the same strict reader as every form there:

```text
(defgenerated "crates/x/src/table.rs"
  (generator "scripts/gen_table.sh")
  (inputs "docs/data/table.csv")
  (command "bash scripts/gen_table.sh docs/data/table.csv")
  (reason "the instruction table, generated from the vendor's CSV"))
```

- The form's name is the generated file's repository path, and the form **declares** that file. One form per path.
- `generator`: the generator files that wrote it, one or more — a script, a program target's crate root, both when a
  script drives a program target; each judged by §5.
- `inputs`: the tracked files the generator read, none or more; `(inputs)` may be omitted when there are none, so a
  table computed from a formula in the generator is declared without one.
- `command`, how it was run, and `reason`, why, each exactly once; nothing executes either.

A form is **live** when a program reads the file it declares. The reader refuses, `trust-undeclared-input`, wherever
the gate runs, each a coded refusal and never an uncoded failure: a declared path twice; an entry twice in one clause; a
file in both `generator` and `inputs`; a declared file that is its own form's generator or input; and, for a live form,
a generator or input that is not a blob of the commit or is a symbolic link. A form that is not live is held to its
syntax alone and is stale (§6), so a deleted file has one code, not two; a declared file that is a symbolic link is
refused as every symbolic link a program reads is (the parent's §3).

**Refused until `M3.6.6.4` decides them**, `trust-undeclared-input`, wherever the gate runs:
- **a chain**: a generator or input of a live form that a `defgenerated` form declares, or that §3 marks; and a file
  that a live form's crate-root generator's build reads (§5) and that a form declares or §3 marks;
- **an input in a vendored checkout**: a gitlink is no blob, so an input that is one is refused by the blob rule.

**A tool a script runs** — protoc, bindgen, an interpreter — is no generator file: it is the script's own dependency,
believed (§8). So a file a tool wrote is declared with the script that ran the tool as its generator, and is then
decided like any other; undeclared and marked it is refused (§3); unmarked it is judged as a plain file (§8). A file a
tool writes outside any committed script, such as cargo writing `Cargo.lock`, cannot be declared, and is refused if a
program reads it marked, and a plain file otherwise.

A `defgenerated` form is a form of `trust/roots.eadl` like a root's: added, changed or removed against the base
commit's, it is in the change part, as the parent's opening puts every root form and `trust_gate.rs`'s `root_forms`
keys every form; the standing list names it; and it is accepted only where reviews are protected (`M3.6.5`), a change
to it needing a new acceptance.

### 3. Recognition: a generated source that does not declare itself

A file is **marked as generated** when its **header** — the comments that come before its first character that is
neither whitespace nor inside a comment — holds, in any case, `@generated`, `do not edit`, `automatically generated`,
`auto-generated`, `autogenerated` or `generated by`.

**The rule is stated as code:** `xtask/src/generated_header.rs` — `TABLE`, `syntax`, `header`, `marked` — with its
corpus in its tests. Its comment syntax is by a file's exact name first, then by its extension in any case, and this
table is the whole of its reach; a test holds it equal to the code's `TABLE`:

<!-- machine-read: comment-syntax -->
| Syntax | Extensions | Names |
| --- | --- | --- |
| `//` and `/* */`, nesting | `rs` `kt` `swift` `scala` | |
| `//` and `/* */` | `c` `h` `cc` `cpp` `cxx` `hh` `hpp` `hxx` `js` `ts` `go` `java` `proto` | |
| `/* */` | `css` | |
| `#` | `sh` `bash` `zsh` `py` `rb` `pl` `r` `toml` `yml` `yaml` `cmake` `mk` `env` `conf` | `Makefile` `makefile` `GNUmakefile` `Dockerfile` `CMakeLists.txt` |
| `;` | `eadl` `el` `lisp` `clj` `s` `asm` `ini` | |
| `--` | `sql` `lua` `hs` `ada` `adb` `ads` `vhd` `vhdl` | |
| `<!-- -->` | `md` `html` `htm` `xml` `svg` | |

An extensionless file whose first line is a `#!` is read with `#`. An XML declaration may come before an `<!-- -->`
file's comments. A `#!` first line is a comment where `#` is, and in Rust unless what follows it is `[`, whitespace and
comments aside — plain block comments nesting, a plain line comment ending at a line feed, and a doc comment (`///`,
`//!`, `/**`, `/*!`) ending the look-ahead, so that line is a shebang. Measured on rustc 1.95.0, `2026-10-06`, against a
control file whose unused function draws a warning: `#! [allow(dead_code)]`, `#! /* c */ [allow(dead_code)]`,
`#! /* a /* b */ c */ [allow(dead_code)]` and `#! // c` followed by `[allow(dead_code)]` on the next line silence it,
so each is an inner attribute; `#! /*! d */ [allow(dead_code)]` and `#! /** d */ [allow(dead_code)]` do not, and
`#! //! d` or `#! /// d` followed by the attribute fail to compile, so each such line is a shebang. Outside that
look-ahead, a header's line ends at a line feed or a carriage return; an unclosed block comment runs to the end of the
file. The
corpus holds every case the reviews raised, each with its outcome. Where this text and the code differ, both are
reviewed together; a case the corpus does not hold is added to it before it is decided.

A marked file that a program reads, or that is a generator or an input of a live form, or that a live form's
crate-root generator's build reads (§5), and that no `defgenerated` form declares, is refused, `trust-undeclared-input`: *"a generated source with no declaration"*, wherever the gate runs,
since a header does not depend on the host. A marked file a live form names as a generator or input is refused as a
chain even when declared (§2).

A marker belongs in a generated source's first comment, at the very top of the file, so a later undeclared copy of it
is recognised; it is advised, not required of a declared one. A hand-written file whose header uses a marker phrase in
ordinary prose — "the code generated by the emitter" — has no honest declaration and is repaired by rewording its
header. A template an emitter copies verbatim into what it emits is hand-written, and its copies are build outputs,
never committed.

### 4. Provenance

A program's **provenance** is, for each live form whose declared file it reads, the form's generator files and
inputs: each a **provenance file**, recorded with its path, its sha256, its **roles** — the set of `generator` and
`input` it plays across those forms — and the declared files through which the program reaches it, each with its
sha256. The inventory records each program's provenance beside its files; a form a program **reaches** is a live form
whose declared file it reads.

### 5. What two programs share, and the program a generator must be

For each pair of programs the parent pairs (§4: every two roots, and the harness with every root but its pair's two), a
new kind of shared item, the **generated provenance**: one per file that one side's provenance holds and the other
side's provenance holds or the other side reads — matched by path, and by sha256 under different paths, a non-empty
file only, as the parent matches a copy (§4, R1 A11). Its identity is the path, or the sorted paths joined by a space,
as the parent writes a copy's. A sha256 match makes an item only between paths that match no path on the other side, so
one file makes one item. Its aspects are the file's sha256, its roles on each side — `read` among them for a side that
reads it, beside any it plays in that side's provenance — and each side's declared files through which it is reached,
with their sha256. So a shared input through two
generators, a shared generator over two inputs, a generator with no input, and a data file one side reads that the
other's generated source was made from each make an item. It is compared, proposed, reported and accepted as every
shared item is: `trust-new-shared` when new, `trust-shared-changed` when an aspect moves, `trust-form-missing` on the
baseline's host when the commit's own baseline holds no form proposing its digests; its §4.4 classification is the
review's. A file both sides read directly is also a shared **file** item, as today, and a generated-provenance item
too if a side's provenance holds it. For the harness's own pair, the harness's provenance is an aspect of the pair's
comparison-harness item, as its files are. A third program reaching the same provenance makes new items in its own
pairs and moves none of the others'.

**A generator file must lie outside the reading program's other roles.** Each generator file of each live form a
program reads, once §2 admits it, is judged; for the harness, "other roles" means roles other than its pair's two:

- the crate root of a program target is judged by that target's role packages, as the parent computes them for every
  program target (§2: `cargo metadata`'s graph by normal edges, and an example's development edges too), the union of
  them when one file is the crate root of two targets — holding one of another role, it is refused,
  `trust-shared-program`. The target is also **built as a root is** (the parent's §3), and every file that build reads
  is subject to §3 and to §2's chain rule, so a marked module compiled into the generator cannot pass unseen;
- a `.rs` file that is no program target's crate root — a library's, a test's, a module — is refused,
  `trust-undeclared-input`: a generator file is a script or an executable's crate root, and only a program target's
  build is the instrument's to compute;
- a file that is not `.rs` — a script — lies in the innermost workspace member whose directory contains it, by whole
  path components, this record's own rule, or in none; lying in a role package of another role, it is refused,
  `trust-shared-program`.

This is a deliberate tightening of `ROADMAP.md`'s words, which ask that such a dependency be visible and reviewed —
§4.4's "a checker acquiring a dependency on the generator's constraint-evaluation implementation must be visible and
block automatic acceptance", §14.4's case 3, "its role and shared provenance become visible". The parent already
refuses its kind: a program running another role's logic is refused, whatever it calls of it, and what it needs from
that role comes through a third package the gate reports as shared (§2). A table another role's program wrote carries
that role's logic the same way, and its route is the same: a generator outside both roles, whose provenance the gate
then reports.

**A case this foresees.** `cargo xtask catalog-check --index --bless` writes `catalog/catalog.lock`, and `xtask`'s
build compiles `crates/archogen-api`, the generator's; should the scheduling checker read the lock at run time, as
`TI-H17`'s catalog records lead toward, its form declares it, and none of its generator files is one §5 refuses for the
scheduling checker (`GS-H12`). A lock has no comment syntax, so undeclared it would not be recognised (§8): the obligation
is carried by its leaf rather than left to the recogniser.

### 6. A declaration no program reads

A `defgenerated` form that is not live is stale, `trust-baseline-stale`, on the baseline's host, as an admission no
site uses is (the parent's §6): removed, so a generated source that stops being read and comes back is reviewed again.

### 7. Case 5, and the parent's text

A change outside every root, program target and provenance file reports "unchanged", as §14.4's case 5 asks
("outside the recorded roots and provenance"). An edit to a provenance file moves an item when two paired programs both
hold or read it, and is otherwise "unchanged" like any file one side alone reads; an edit to a script no live form
names as a generator or an input changes nothing. The parent says otherwise in places, each amended by a dated
clarification when the instrument lands (`GS-H8`):
- its opening's "a change outside every root and every program target";
- §2's list of the forms `trust/roots.eadl` holds, and its definition of `trust-shared-program` — "a root whose build
  compiles a role package of another role … and the comparison harness compiling one of a role outside its pair" —
  which §5 joins with a generator file of another role;
- §3's paragraph on generated sources; its list of what is recorded for each root, which gains provenance; and its
  sentence that `Cargo.lock` is recorded and is not part of the build configuration, which stands, beside this record's
  refusal of a marked file no form can declare;
- §4's list of shared items, and its paragraph on an item's aspects — content, configuration, edges, readers — beside
  which the generated provenance's are digest, roles and declared files;
- §5's description of a form, "its content, configuration, edges and file readers as proposed";
- §6's table: `trust-new-shared`'s list of what is new, `trust-shared-changed`'s list of what moves,
  `trust-undeclared-input`'s, `trust-shared-program`'s and `trust-baseline-stale`'s new triggers; and its case 5;
- `TI-H9`'s "a change outside every root's units", which then reads "outside every root's units and every provenance
  file".

### 8. What stays outside, stated in the report

The gate's report states what the inventory does not see — the parent's list (§3), which no report states today, and
these:

- **The declaration is believed, not verified.** The instrument runs no package's code, so it does not run the
  generator to check that the committed file is its output, that it read only the declared inputs, or that the
  generator files named are all there are — a script that drives another role's program target, declared with the
  script alone, escapes §5's refusal, and so does a tool a script runs. A wrong
  declaration is the review's to catch, reading the form and its command, which the standing list names and
  `M3.6.5` requires accepted; regenerating in CI is a check of its own, not this design's.
- **A generated file the recogniser does not mark**, judged as a plain file: one whose generator writes no marker;
  one with no comment syntax the table names — a `.txt`, `.lock`, `.json` or `.csv`, a binary, a linker script, a
  `.cs`, `.tsx`, `.jsx` or `.mjs`; one whose marker comes after content — a Markdown heading, YAML's `---`; one marked
  in a comment syntax the table does not read — a marker on a later line of Lua's `--[[ ]]`, SQL's `/* */`, GNU
  assembler's `#` and `/* */`; a file whose name or extension the table does not hold.
- **A generator's own dependencies** — a script's interpreter, the modules a crate root's program compiles beside it —
  are not provenance files: a crate-root generator is hashed by its crate root alone, so an edit elsewhere in its
  program changes no item until its output is regenerated, which then does.

### 9. Hand-offs

Each obligation is one sentence, quoted word for word beside its identifier by the leaf that takes it, which
`HANDOFF-LEDGER` checks.

<!-- machine-read: handoffs -->
| Id | Leaf | Obligation |
| --- | --- | --- |
| `GS-H1` | `M3.6.6.2` | `trust/roots.eadl` takes `defgenerated` forms read strictly as §2 states — one per declared path, one or more generator files, zero or more inputs, a command and a reason exactly once — refusing as `trust-undeclared-input` wherever the gate runs, each a coded refusal, every construction §2 lists, a live form's chain, a non-blob or a symbolic link among a live form's generators and inputs included, and holding a form that is not live to its syntax alone. |
| `GS-H2` | `M3.6.6.2` | A file that `xtask/src/generated_header.rs`'s `marked` marks, that a program reads, that is a generator or an input of a live form, or that a live form's crate-root generator's build reads, and that no `defgenerated` form declares, is refused as `trust-undeclared-input` wherever the gate runs, the recogniser, its corpus and its `TABLE`, held equal to §3's machine-read table, being the rule §3 states. |
| `GS-H3` | `M3.6.6.2` | Each program's record carries its provenance as §4 defines it, the harness's reads and provenance counted through the units it compiles beside its pair's two builds, and each pair the parent pairs shares a generated-provenance item per file one side's provenance holds and the other side's provenance holds or the other side reads, matched by path, or by a non-empty file's sha256 between paths that match no path on the other side, its identity the path or the sorted paths joined by a space, its aspects the file's sha256, its roles on each side — `read` among them for a side that reads it — and each side's declared files with theirs, the harness's own pair taking the harness's provenance as an aspect of its comparison-harness item, compared and proposed as every shared item is. |
| `GS-H11` | `M3.6.6.2` | For each generator file §2 admits of each live form a program reads, a program target's crate root whose role packages, by the parent's §2 computation and their union for a crate root of two targets, hold one of a role other than the program's own — for the harness, other than its pair's two — is refused as `trust-shared-program`, the target being built as a root is and every file that build reads subject to §3 and to §2's chain rule; a `.rs` file that is no program target's crate root is refused as `trust-undeclared-input`; and a file not `.rs` lying, by the innermost workspace member whose directory contains it, in a role package of such a role is refused as `trust-shared-program`. |
| `GS-H4` | `M3.6.6.2` | A `defgenerated` form whose declared file no program reads is `trust-baseline-stale` on the baseline's host. |
| `GS-H5` | `M3.6.6.2` | The instrument's tests hold a case-3 fixture of two roots whose differently-named generated sources share an input through two generators, and others for a generator shared over two inputs, a generator with no input, a data file one side reads that the other's generated source was made from, a copy matched by sha256, the harness, a third program, a script running a tool declared and decided, a chain refused, a marked module of a crate-root generator's build refused, a gitlink input refused, a marked file no form can declare refused, another role's program target and script refused as generator files, a library's crate root refused as one, a `defgenerated` form added, changed and removed in the change part, an edit to a declared generator reported and to an undeclared script "unchanged", and each refusal, each removed in turn by a catalogued mutation. |
| `GS-H7` | `M3.6.6.2` | The gate's standing list names every `defgenerated` form, its change part every one added, changed or removed against the base commit's, and its report states the parent's §3 list of what the inventory does not see and §8's three exclusions. |
| `GS-H8` | `M3.6.6.2` | `decision_trust-inventory.md` gains a dated clarification amending each passage §7 lists to say what this record decides. |
| `GS-H9` | `M3.6.5` | A `defgenerated` form is accepted per form as every form in `trust/` is, a change to it needs a new acceptance, and the assurance step's Passed requires every live form accepted. |
| `GS-H10` | `M4.8` | A property resting on a role's independence is established only when every live `defgenerated` form reached, as §4 defines reaching, by a root of a role it rests on or by the comparison harness of a pair holding such a role is accepted, beside `TI-H16`'s conditions. |
| `GS-H12` | `M2.7.5` | Should the scheduling checker read `catalog/catalog.lock` or any other generated file, that file is declared by a `defgenerated` form none of whose generator files §5 refuses for the scheduling checker. |
| `GS-H6` | `M3.6.6.3` | The trust chapter states what a generated source must declare, how an undeclared one is recognised and what the recogniser does not reach, what is refused until `M3.6.6.4`, the new shared item and the new triggers in the chapter's codes table, and §8's three exclusions. |

## Why

- **A form, not a header the tool trusts.** A header is written by the generator, which is the thing under suspicion,
  and a form in `trust/` is reviewed and accepted where reviews are protected. The header's job is narrower: to make an
  undeclared generated file recognisable.
- **The header as code.** Two rounds of prose left cases two readers would decide differently; a recogniser with a
  corpus decides each once (`decision_executable-design-reviews.md`: a decision procedure ships as an executable model).
- **One step, the rest refused.** Three rounds found that each feature added to answer the last — a tool outside the
  commit, a gitlink input, a chain of generators, an item reached through role packages — brought its own open cases,
  for a design with no instance in the tree. What a single committed generator step needs is decided here; the rest is
  refused, so it cannot pass unseen, and decided by `M3.6.6.4` when a generated source needs it.
- **Provenance per file, matched against what the other side reads too.** §14.4's case is a shared *input*, and §4.4's
  is a shared formula: either alone is a common source of error, and so is a data file one side reads directly that the
  other's generated source came from.
- **A generator judged by the build the parent already computes.** A program target's role packages are §2's set,
  reviewed with its classification; no new reading of what a build compiles is introduced.
- **Declined, round 1's R1-21:** `.gitattributes`' `linguist-generated` as a second recognition signal. It is a
  hosting's display hint, read by no build, a second, weaker way to say what the reviewed form says.

## How to apply

- Generating a source a program reads: commit it with a marker in its first comment and its `defgenerated` form in the
  same commit; the gate on the baseline's host then asks for the proposed forms of any provenance two programs share.
- `M3.6.6.2` implements §2–§8 against the hand-offs; `M3.6.6.3` puts it in the trust chapter; `M3.6.5`, `M4.8` and
  `M2.7.5` take theirs; `M3.6.6.4` decides chains and vendored inputs, and whether a tool a script runs becomes
  provenance.
- Related: `decision_trust-inventory.md`, `decision_executable-design-reviews.md`.

## Review

`M3.6.6.1`'s acceptance is a review by a context that did not write this record, finding no defect; every finding
answered here. The history is [`decision_trust-generated-sources-reviews.md`](../../reviews/decision_trust-generated-sources-reviews.md).

| Round | Date | Defects | Outcome |
| --- | --- | --- | --- |
| 1 | `2026-10-06` | 19, and 6 remarks | every finding answered: the census by the rule itself, harness and handed data included; provenance per file, transitive, through role packages, matched by sha256 too; the header any length, its markers widened; the parent's lists amended by hand-off; forms reviewed and required accepted; R1-21 declined |
| 2 | `2026-10-06` | 12, and 11 remarks | every finding answered: the header stated as code with its corpus; another role's generator refused, the role-package route withdrawn; a file one side reads matched against the other's provenance; tool and gitlink provenance; several generator files; stale on the host only; the parent's amendments listed whole; the gate's change part fixed to the parent's opening (`M3.6.3.7`) |
| 3 | `2026-10-06` | 12, and 8 remarks | answered by narrowing: one generator step decided, chains, tools and vendored inputs refused until `M3.6.6.4`; a generator judged by the parent's own computation of a program target's role packages; the recogniser's text and code agreed — a spaced shebang, nesting in Kotlin, Swift and Scala, a carriage return, an XML declaration, an extensionless script — and its reach stated, recall measured; the parent's amendment list completed; a hand-off to `M2.7.5` for the catalog lock |
| 4 | `2026-10-06` | 9, and 6 remarks | every finding answered: a tool a script runs a believed dependency, not a refused shape; a crate-root generator built as a root, its build's files under §3 and the chain rule; the shebang look-ahead nesting and stopping at doc comments, measured on rustc 1.95.0; the table of comment syntax machine-read and held equal to the code's; the parent's `trust-shared-program` definition and per-root list added to the amendments; `program` and `generator file` kept apart |
