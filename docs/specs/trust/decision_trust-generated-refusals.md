# Generated sources: the shapes refused for good and the tool believed, each with its reason and, where one exists, a route

- **Type:** `decision`
- **Date:** `2026-10-10`
- **Status:** `active` — written `2026-10-10` by leaf `M3.6.6.4`; its review open
  ([`decision_trust-generated-refusals-reviews.md`](../../reviews/decision_trust-generated-refusals-reviews.md))
- **Owner / source:** leaf `M3.6.6.4` (`docs/tasks/M3.md`), filed by `M3.6.6.1`'s third review. It discharges `GS-H13`
  of [`decision_trust-generated-sources.md`](decision_trust-generated-sources.md), cited below as "the parent": *"Each
  shape §2 refuses until this leaf — a chain, through a crate-root generator's own build too, and a generator or an
  input at or under a gitlink — is declared with its provenance a shared item or refused with a reason, and the tool a
  script runs that §8 believes is made provenance or kept believed with a reason, under a design reviewed by a context
  that did not write it."*

## The fact / decision

### 1. What is decided

The parent decides one generator step, from committed files to a committed file; what it did not decide it refused until
this leaf, and it believed the tool a script runs (its §1, §2, §8). Each is decided here: the two shapes stay refused,
each for a reason that holds without an instance, and a need they would serve has a route within one step where one
exists; where none does it is said, and a later need is §5's — a marked file (a chain, the parent's §3 reading its
marker) that the generation cannot remake from committed files, and a marked crate root; a program-target generator that
must compile a generated file; a vendored file that cannot be copied, and a vendored generator whose copy the role rules
refuse; the tool stays believed, with a route for when its identity must be compared. Each route and each case with none
is claimed as far as a test of §6 holds it. No behaviour moves: every refusal and belief stays as the parent and its
instrument (`M3.6.6.2`) have it, and the chain's refusals name this record.

**Measured `2026-10-10`** at `614d5c7`: `grep -c defgenerated trust/roots.eadl` → `0`; and `cargo xtask trust-inventory
--commit HEAD` → *"5 program(s), 8 pair(s), 9 shared item(s)"*, its `generated-marked` and `generated-unread` empty and
every program's provenance empty. No instance of either shape exists, and no tool runs for a declared source.

### 2. A chain of generators: refused

What the parent's §2 calls a chain — a live form's generator or input that a `defgenerated` form declares or that the
parent's §3 marks, and a file that a live form's crate-root generator's build reads and that a form declares or §3
marks — is refused, `trust-undeclared-input`, wherever the gate runs: the parent's rule, in every live form, which this
record keeps; the instances its routes rest on are §6's.

**Why.** Provenance is one step (the parent's §4): what a program reaches through a form is that form's generator files
and inputs, its generator files judged by the parent's §5 against that program, its inputs hashed and matched. A chain
makes provenance a closure over steps, and then the roles a file plays, the declared files through which a program
reaches it, and the program each upstream generator is judged against each have a reading per step. The parent's review
met this: a chain lost its first link (its R1-6), answered by making provenance transitive; transitive, its declared
files then had two readings (R3-5), and another role's generator escaped the role judgment through a chain (R3-7) — an
escape of the very refusal the gate exists for — and round 3 closed them only by narrowing to one step. Refused, a chain
the parent's rules can see — through a declared or a marked file — hides nothing: each of its files is seen and named;
one through a file no form declares and no header marks is the parent's §8 limit, a plain file.

**The route.** A file made from another generator's output is declared as the last step of one generation: its form
names every committed generator file of every step — each judged as the parent's §5 judges a generator file against
the programs that read the result, its rule and its instrument's tests — and every committed file any step reads, and
the intermediate file is the generation's own product, never committed. A marked input, or a marked script a form
would name as a generator — a vendored copy among them (§3) — is such an intermediate: the generation remakes it from
committed files, its own generator and inputs, or copies of them for a vendored one, never committing it; one it cannot
remake so has no route (§5). A marked crate root has none: a
program built from it compiles a generated file, a build the gate does not compute. A script that remakes and builds it
is a generator the gate takes, but the program it builds is the script's own dependency, believed with whatever its
build compiles, another role's package among them — the parent's §8 limit, not a route. A program-target generator whose
build would compile a generated file is served so that its build compiles none: one compiling another generator's output
reads it at run time instead, the script that runs both handing it over; one compiling the file its own form declares —
a program that generates its own source — gives way to another program target whose build does not compile that file,
judged by the parent's §5 as every generator is: a second executable of its package when neither a library of the
package nor any package it depends on, directly or not, holds it, or else one of a package that does not depend, directly or through
another, on the package holding it. One that must compile a generated file has no route (§5). If a program also reads
the intermediate, that file is committed and declared, and a later step reading it is a chain: that step remakes it from
the first step's inputs instead. A file one step runs and another reads is named once, as a generator — named in both
clauses it is refused (the parent's §2).

### 3. A generator or an input at or under a gitlink: refused

Neither is a blob of the commit, so the parent's blob rule refuses it in a live form, `trust-undeclared-input`,
wherever the gate runs; a form no program reads is held to the parent's §6.

**Why.** The inventory is the commit's blobs (the inventory record, `decision_trust-inventory.md` §3): every file a
program reads, and every generator and input a live form names, is a blob of the commit — the build's own artifacts aside,
which it makes and hashes. A gitlink is a commit id the commit holds in place of a checkout's files: the files
under it are another repository's blobs, which the gate, reading this commit's alone, never reads, so no form could
propose a digest of them the gate recomputes. The parent's round 2 took the gitlink's pinned commit as the digest
(`359eee3`); its round 3 withdrew it with the chains and the tools' pins it found half-integrated, narrowing to one
step. Taken again, a commit id names the whole checkout: a change anywhere in it would move the item, and which of its
files a generator read would go unseen. This repository keeps every vendored checkout read-only besides
([`decision_repository-boundary-read-only.md`](../../decisions/decision_repository-boundary-read-only.md)).

**The route.** For an input: the file the generator reads is committed — a copy of the vendored file, its source
and pin named in the form's `reason` — and named as an input, a file of the commit, hashed, shared and compared as any
input is; a copy whose header marks it generated is a chain, refused, and served by §2's route. For a generator that
is a file — a script, an executable, a Rust source: a committed, unmarked copy named as the form's generator, beside
the script that runs it, judged by the parent's §5 and hashed as any generator file — a Rust one copied as a program
target of the workspace, built as every program is, offline from the commit's files alone (the inventory record,
§3), whose build then meets the parent's rules, since a `.rs` file that is no program target's
crate root is refused as a generator (the parent's §5). A vendored file that cannot be copied — its licence or its size
keeping it out of the commit, or no file of the checkout, as a build product — has no route (§5), and nor has a
generator whose copy the role rules refuse: an input among them stays refused; a generator among them, run by a committed
script, is that script's dependency, believed — for this section's reason, the gate reading no file under a gitlink,
and, for a build product, §4's too, its bytes the host's build — outside every provenance, so its bytes, their sharing and, for one the role rules refuse, that role
judgment go unseen, as the parent's §8 says of whatever a script runs; its identity alone can be compared, through a
committed version file the script checks it against, by §4's route.

### 4. A tool a script runs: believed

protoc, bindgen, an interpreter: no generator file but the script's own dependency, believed (the parent's §8) and
named in the last section of every report, *what the inventory does not see*. A tool from a vendored checkout is §3's
vendored file: copied, a generator file named beside the script; uncopied, believed for §3's reason, and a build
product for this section's too. This section's
reason is a host tool's, and its route serves both.

**Why.** Its bytes are no file of the commit, and which one runs is the host's: like the linker and the host's C
toolchain, which the inventory records by version alone (`decision_trust-inventory.md` §3), a tool is a fact of the
machine the generation ran on, not of the commit. As provenance it would make a form's digests facts of that machine
that no commit could reproduce. The tool is named in the script the form names as its generator, or in the
`command` that runs it, the script itself a provenance file every program reading the form reaches (the parent's §4).

**The route, when a tool's identity must be compared.** Its pin is committed — a file the script reads to choose or
to check the tool, a lock or a version file — and named as an input: a change made through the pin is then a change of
a provenance file, reported as any is, while the tool that actually ran stays believed to match it. A pin whose
header marks it generated is a chain, refused: §2's route remakes it within the generation, or an unmarked version
file the script checks the tool against is committed and named as an input in its place.

### 5. A later need

An instance none of these routes serves reopens its shape in a leaf of its own, the instance in hand, under a design
reviewed by a context that did not write it; until then the refusal or the belief holds.

### 6. Each route held by a test

What this record decides — the routes above, and the cases with none — is claimed as far as these tests build it and
the gate writes or refuses the inventory. The gate's rules the routes rest on — the chain, the blob rule, the role
judgment of each generator against each reader, in every live form, at every position and for every reader — are the
parent's, decided by its record and held by its instrument's tests; their breadth across every form, entry and reader is
`M3.6.9`'s, filed by this record's round 14. The tests are `route_tests` in `xtask/src/trust_generated.rs`, written when
review rounds had probed the routes in untracked tests and found some the record named fail, and four of the parent's
tests, named with their module — three no-route cases, and the harness as a reader:

| Test | Holds |
| --- | --- |
| `route_a_chain_made_in_one_generation` | §2: a chain refused, through a declared input and through a declared script named as a generator; one form naming both steps of a two-step generation and three committed files the steps read — the later step's own among them — the intermediate never committed, written, each reached |
| `route_a_marked_input_or_script_remade_in_the_generation` | §2, §3, §4: a marked copy, script or pin — an input as a pin is — refused; remade from its own generator and inputs, written |
| `route_a_chain_judges_every_step_s_generator` | §2: every step's generator judged by the parent's §5 — another role's script step named last and named first, refused both ways; a program-target step at every position, in the sweep below |
| `route_a_generator_is_judged_against_every_reader` | §2, §3: a script judged against each of three readers in all six orders — refused for every reader of another role, `chk` and `imp` from `gen`'s package, `gen` and `imp` from `chk`'s |
| `route_a_program_target_generator_is_judged_against_every_reader` | §2, §3: a program-target generator judged against each of three readers in all six orders — refused for every reader of another role, from `gen`'s second role package and from `chk`'s |
| `route_tests_refusals_hold_at_every_position_of_a_form_s_entries` | §2, §3: fifteen entry refusals — a declared, a marked, a generator or input at or under a gitlink, a file named in both clauses, another role's script and program target, a program target whose build reads a declared file, a root's own, one whose build reads a marked undeclared file, a lone `.rs` — each refused alone and first, in the middle and last of three |
| `generator_tests::the_harness_may_use_its_pair_s_roles_and_no_other` | §2: the comparison harness as a reader — a script of its pair's role written; a third root's script refused |
| `no_route_for_a_marked_crate_root_but_the_parent_s_belief_in_a_script` | §2: a marked crate root refused; a script remaking it written — the parent's §8 limit |
| `route_a_program_target_step_reads_the_intermediate_at_run_time` | §2: a program-target step reading the intermediate at run time, written |
| `route_a_program_that_generates_its_own_source` | §2: refused; a second executable of its package, no library holding the file, written |
| `route_a_table_its_package_s_library_holds_is_made_by_another_package` | §2: the package's own executables refused; one of a package not depending on it, written |
| `route_a_vendored_file_copied` | §3: a copied input, script and executable — three generators, the executable in the middle, and an input — written, each reached; a lone `.rs` refused; a program target written |
| `route_a_table_a_dependency_s_library_holds_is_made_by_a_package_not_depending_on_it` | §2: a dependency's library holding the file: the package's executables refused, and a package depending on it through another; one of a package not depending on it, written |
| `route_an_intermediate_a_program_reads_is_remade_by_the_later_step` | §2: an intermediate a program reads, committed and declared: read by the later step, refused; remade by it, written |
| `route_a_file_one_step_runs_and_another_reads_is_named_once` | §2: named in both clauses, refused; once, as a generator, written |
| `a_vendored_executable_a_script_runs_uncopied_is_believed` | §3, §4: written, the gitlinked tool in no provenance; named on the form, refused |
| `route_a_tool_pin` | §4: a version file written; a marked pin refused |
| `order_tests::step_5_refuses_a_chain_through_a_generator_build_and_an_undeclared_marked_file_one_message_a_file` | §2: a generator whose build compiles another generator's output, refused |
| `provenance_tests::step_3_refuses_a_live_form_s_non_blob_and_its_chain_and_holds_a_form_no_program_reads_to_its_text` | §3: an input at a gitlink, refused |
| `generator_tests::another_role_s_program_target_script_or_library_is_refused_as_a_generator` | §3: a generator the role rules refuse, refused |

## Why

- **Refused for good, not until.** A refusal "until" a leaf leaves a decision owed with no case to decide it on; three
  of the parent's review rounds showed that deciding these shapes without one only opens cases. A reason that holds
  without an instance closes the question, and the routes, where they exist, keep the needs open to one step.
- **The routes keep the parent's guarantees.** Each route ends in files of the commit named on a form, so the parent's
  §4 and §5 apply to them whole, as its record decides them and its instrument's tests hold them: its generator files
  judged against their readers, its inputs hashed and shared. What has no route is refused, or, for a tool and a
  vendored generator a script runs, believed and named in every report.

## How to apply

- A generated file a chain would make: one form, every step's committed generator files, every committed file any
  step reads, the intermediate never committed, and a marked input or script a form would name remade so too — a
  marked crate root has no route; a program-target generator whose build would compile a generated file reads another
  generator's output at run time, or, generating its own source, gives way to a program target whose build does not
  compile it. A vendored input: a committed, unmarked copy named as an input; a vendored generator that is a file — a
  script, an executable, a Rust source: a committed, unmarked copy named as the generator, a Rust one as a program
  target; a vendored file that cannot be copied, or a generator whose copy the role rules refuse: no route, an
  input refused, a generator believed when a script runs it, its identity compared through a committed version file.
  A tool's identity to compare: its unmarked pin committed and named as an input; a marked one remade so, or replaced
  by an unmarked version file the script checks. Each is a test of §6.
- `M3.6.6.4` closes when this record's review finds no defect; the parent's dated notes and the trust chapter say
  what is decided here.
- Related: [`decision_trust-generated-sources.md`](decision_trust-generated-sources.md),
  [`decision_trust-inventory.md`](decision_trust-inventory.md).

## Review

`M3.6.6.4`'s acceptance is a review by a context that did not write this record, to a round finding no defect; every
finding answered here. The history is
[`decision_trust-generated-refusals-reviews.md`](../../reviews/decision_trust-generated-refusals-reviews.md).

| Round | Date | Defects | Outcome |
| --- | --- | --- | --- |
| 1 | `2026-10-10` | 8, and 8 remarks | every finding answered: the chain route's later step reads its input at run time, and names every committed file any step reads; a marked copy or pin has no route; a vendored generator's route; the messages, R1-6, the tool's naming and the leaf's figure made true |
| 2 | `2026-10-10` | 5, and 6 remarks | every finding answered: "a route" narrowed everywhere to a route where one exists, the cases with none named; a vendored generator copied as a file, a vendored program believed for §3's reason; the tool's naming as the parent's §4 has it |
| 3 | `2026-10-10` | 3, and 5 remarks | every finding answered: a vendored executable decided once, in §3, §4's version-file route its only comparison; "every file it hashes" narrowed to what a program reads and a form names; How to apply as §3 reads |
| 4 | `2026-10-10` | 3, and 4 remarks | every finding answered: the blob rule's claim narrowed to a live form; a marked file a form would name remade within the generation, §2's route, a marked pin so or replaced by a version file; a program that generates its own source served by a second program target; a vendored file that cannot be copied decided, an input refused, a generator believed |
| 5 | `2026-10-10` | 3, and 3 remarks | answered by method: every route and no-route case held by a committed test (§6); a self-generating program's route narrowed to a target whose build does not compile the file; an executable a file a copy serves; a marked crate root with no route, a script remaking it the parent's §8 limit |
| 6 | `2026-10-10` | 3, and 5 remarks | every finding answered: the self-generating program's route narrowed to the packages it depends on; §6 grown to every route and no-route case, three held by the parent's tests; a build product believed for §4's reason too |
| 7 | `2026-10-10` | 2, and 3 remarks | every finding answered: a declared script named as a generator, refused, held by a test; "the role rules" wherever a copy is refused; the self-generating route judged by §5 as every generator is |
| 8 | `2026-10-10` | 1, and 4 remarks | answered: the parent's §5 judgment of every step's generator against every reader held by two tests; the marked pin's row; a Rust copy built offline, as every program; the catalogue's mutations filed as `M3.6.8` |
| 9 | `2026-10-10` | 2, and 3 remarks | answered: each judgment held both ways — every step's generator named first and last, every reader of a script and of a program-target generator from either role; a later step's own committed input reached; `M3.6.8` widened |
| 10 | `2026-10-10` | 2, and 2 remarks | answered: every refusal of an entry held at every position of its clause, the class closed by one sweep; a package depending on the table's through another refused; `M3.6.8` widened |
| 11 | `2026-10-10` | 3, and 1 remark | answered: the sweep widened to every entry refusal §6 names — another role's program target, a program target whose build reads a declared file, a lone `.rs`, the gitlink itself as a generator; `M3.6.8`'s text; a changelog entry |
| 12 | `2026-10-10` | 5, and 3 remarks | answered: §6's rows say exactly what each test holds — three readers each first, in the middle and last, three inputs, fourteen entry refusals each at every position, another role's script and a root's own crate root among them; round 11's text corrected; "directly or not" |
| 13 | `2026-10-10` | 6, and 3 remarks | answered: every refused reader asserted in all six orders; the harness's test named; a program target compiling a marked module in the sweep; the vendored row's three generators; the completeness claims of rounds 10 and 11 narrowed in the changelog; `M3.6.8` |
| 14 | `2026-10-10` | 5, and 3 remarks | answered by method: the gate's rules the routes rest on said to be the parent's, held by its tests, their breadth across forms, entries and readers filed as `M3.6.9`; §6's opening exact; the round-11 changelog entry and the round-12 leaf text narrowed to what they held |
