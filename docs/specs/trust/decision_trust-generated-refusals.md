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

The parent decides one generator step, from committed files to a committed file; what it did not decide it refused
until this leaf, and it believed the tool a script runs (its §1, §2, §8). Each is decided here: the two shapes stay
refused, each for a reason that holds without an instance, and a need they would serve has a route within one step
where one exists; where none does it is said, and a later need is §5's — a file a route names whose header marks it
generated (a chain, the parent's §3 reading its marker), a later step that is a program and must compile the
intermediate, and a vendored program that cannot be copied; the tool stays believed, with a route for when its
identity must be compared. No behaviour moves: every refusal
and belief stays as the parent and its instrument (`M3.6.6.2`) have it, and the chain's refusals name this record.

**Measured `2026-10-10`** at `614d5c7`: `grep -c defgenerated trust/roots.eadl` → `0`; and `cargo xtask trust-inventory
--commit HEAD` → *"5 program(s), 8 pair(s), 9 shared item(s)"*, its `generated-marked` and `generated-unread` empty and
every program's provenance empty. No instance of either shape exists, and no tool runs for a declared source.

### 2. A chain of generators: refused

What the parent's §2 calls a chain — a live form's generator or input that a `defgenerated` form declares or that the
parent's §3 marks, and a file that a live form's crate-root generator's build reads and that a form declares or §3
marks — is refused, `trust-undeclared-input`, wherever the gate runs.

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
names every generator file of every step — each judged by the parent's §5 against every program that reads the
result — and every committed file any step reads, and the intermediate file is the generation's own product, never
committed. A program-target generator that would compile another generator's output reads it at run time instead, the
script that runs both handing it over, so its build, made from the commit's files, compiles no generated file; one
that must compile it has no route (§5). If a program also reads the intermediate, that file is committed and declared,
and a later step reading it is a chain: that step remakes it from the first step's inputs instead. A file one step
runs and another reads is named once, as a generator — named in both clauses it is refused (the parent's §2).

### 3. A generator or an input at or under a gitlink: refused

Neither is a blob of the commit, so the parent's blob rule refuses it, `trust-undeclared-input`, wherever the gate
runs.

**Why.** The inventory is the commit's blobs (the inventory record, `decision_trust-inventory.md` §3): every file it
hashes is a blob of the commit. A gitlink is a commit id the commit holds in place of a checkout's files: the files
under it are another repository's blobs, which the gate, reading this commit's alone, never reads, so no form could
propose a digest of them the gate recomputes. The parent's round 2 took the gitlink's pinned commit as the digest
(`359eee3`); its round 3 withdrew it with the chains and the tools' pins it found half-integrated, narrowing to one
step. Taken again, a commit id names the whole checkout: a change anywhere in it would move the item, and which of its
files a generator read would go unseen. This repository keeps every vendored checkout read-only besides
([`decision_repository-boundary-read-only.md`](../../decisions/decision_repository-boundary-read-only.md)).

**The route.** For an input: the file the generator reads is committed — a copy of the vendored file, its source
and pin named in the form's `reason` — and named as an input, a file of the commit, hashed, shared and compared as any
input is; a copy whose header marks it generated is a chain, refused, with no route (§5). For a generator that is a
file: a committed, unmarked copy named as the form's generator, judged by the parent's §5 and hashed as any generator
file. A vendored program that cannot be copied has no route (§5): run by a committed script, it is that script's
dependency, believed — for this section's reason, the gate reading no file under a gitlink, not §4's — outside every
provenance, so its sharing goes unseen.

### 4. A tool a script runs: believed

protoc, bindgen, an interpreter: no generator file but the script's own dependency, believed (the parent's §8) and
named in the last section of every report, *what the inventory does not see*.

**Why.** Its bytes are no file of the commit, and which one runs is the host's: like the linker and the host's C
toolchain, which the inventory records by version alone (`decision_trust-inventory.md` §3), a tool is a fact of the
machine the generation ran on, not of the commit. As provenance it would make a form's digests facts of that machine
that no commit could reproduce. The tool is named in the script the form names as its generator, itself a
provenance file every program reading the form reaches (the parent's §4).

**The route, when a tool's identity must be compared.** Its pin is committed — a file the script reads to choose or
to check the tool, a lock or a version file — and named as an input: a change made through the pin is then a change of
a provenance file, reported as any is, while the tool that actually ran stays believed to match it. A tool from a
vendored checkout has its gitlink for a pin, which is no blob: its route is a committed version file the script checks
the tool against. A pin whose header marks it generated is a chain, refused, with no route (§5).

### 5. A later need

An instance none of these routes serves reopens its shape in a leaf of its own, the instance in hand, under a design
reviewed by a context that did not write it; until then the refusal or the belief holds.

## Why

- **Refused for good, not until.** A refusal "until" a leaf leaves a decision owed with no case to decide it on; three
  of the parent's review rounds showed that deciding these shapes without one only opens cases. A reason that holds
  without an instance closes the question, and the routes, where they exist, keep the needs open to one step.
- **The routes keep the parent's guarantees.** Each route ends in files of the commit named on a form, so the parent's
  §4 and §5 apply to them whole: every generator file judged against every reader, every input hashed and shared. What
  has no route is refused, or, for a tool and a vendored program a script runs, believed and named in every report.

## How to apply

- A generated file a chain would make: one form, every step's generator files, every committed file any step reads,
  the intermediate never committed. A vendored input: a committed, unmarked copy named as an input; a vendored
  generator: a committed script that runs it. A tool's identity to compare: its unmarked pin committed and named as an
  input.
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
