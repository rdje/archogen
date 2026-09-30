# Where the engine's knowledge comes from: the catalog

A description says what a system must do. To say anything about how a real system does it, the engine needs
knowledge that no description holds: what a scheduler guarantees, what a timer on a given platform does, how long a
kernel path takes. `ROADMAP.md` §9 puts that knowledge in a **catalog**, and asks for every entry to carry "an ID,
semantic version, content hash, source/license metadata, maintainer, dependencies, supported profiles,
preconditions, guarantees, implementation source, model source, cost evidence, and evidence status".

This chapter explains the design that answers it. The design is decided, and it was reviewed independently sixteen
times before it was accepted. Its code is being built in the `archogen-catalog` crate (`M2.7.3`). So far that crate
reads a record and refuses what breaks the design's rules for a single file, and computes every hash the design
defines, reproducing the worked example's 23 values. It also reads the catalog's lock and checks it against the
records of one tree: a changed facet without a version bump, a version going backwards, a missing line, and a
review line that disagrees with its review or was taken out of its record. Over a history given in memory, it
holds the lock append-only, recomputes every commit's new lines as blessing would write them, verifies each review
at the commit that ledgered it, and accepts a waiver only where §9 allows one. From that history it derives each
facet's evidence status: rejected while a rejection reaches it unanswered, by its own record, its lineage or
content it shares, production only where a review names its current hash, stale or unreviewed otherwise. At each
commit that recorded a review it checks the review's date against the commit's and that every answer names a
rejection the facet inherits, and it keeps a deleted record's rejection from being shed. Given what a claim
recorded, it names every line and lookup that no longer holds, even when the catalog no longer loads. The rest of
the crate, the
gate, the check that protects it and the first records are the next leaves (`M2.7.3` to `M2.7.6`). Until they
land, nothing loads a catalog, and `catalog/` is empty.

## The records that hold it

| File | What it holds |
| --- | --- |
| `docs/decisions/catalog/decision_catalog-records.md` | the design: what a record holds, how status and claims work, what it defends against |
| `docs/decisions/catalog/decision_catalog-records-hashes.md` | its §3, the byte grammar every hash is computed over, and what the gate builds |
| `docs/decisions/catalog/decision_catalog-records-example.md` | a worked example of records and the files they name, with every hash they have, which the catalog crate must reproduce |
| `docs/decisions/catalog/decision_catalog-records-variant-inputs.md` | its §12, what the runtime analysis takes from the catalog, and from whom |
| `docs/decisions/catalog/decision_catalog-records-limits.md` | its §13, what the design does not do |
| `docs/decisions/catalog/decision_runtime-composite-inputs.md` | how four of the analysis's inputs are put together from catalog, application and plan parts |
| `docs/reviews/decision_catalog-records-reviews.md` | every review round, every finding, and the answer to each |
| `crates/archogen-catalog/src/record.rs` | the code so far: one record read, and every rule of the design's §1 and §2 that needs only the file, each refusal with its one code |
| `crates/archogen-catalog/tests/record.rs` | one valid record and one change of it per rule, each refused with that rule's code |
| `crates/archogen-catalog/src/hash.rs` | every hash: each facet's own and bound hash, the record's, a review's, over the files, packages, targets and ledger sections they rest on |
| `crates/archogen-catalog/tests/hash.rs` | the worked example, read from its own file, with all 23 of its values |

## A record, and its four parts

A record is one small text file under `catalog/`, written in the same datum syntax eADL is read with, and read by
that reader alone. It has four parts, called facets:

- its **contract**: its identity, version, maintainer, dependencies, the profiles and targets it supports, and the
  guarantees and preconditions it states;
- its **implementation**: the Rust packages that realize it, or a statement that there are none;
- its **behavioral model**: facts about what the code or the hardware does, such as "the timer compare has level
  semantics", each pointing at the code or file it is about;
- its **timing model**: costs, such as how long a context switch takes on a target, each with its evidence.

The two models are versioned apart, because a change to one invalidates different claims than a change to the
other, which `ROADMAP.md` §9 asks for in so many words.

## Hashes: what a review is a review of

Every facet has two hashes, and neither is written in the record; both are computed.

- Its **own** hash covers the facet's own text and files. It pins the facet's version in the catalog's lock, so a
  facet that changes without its version moving is refused.
- Its **bound** hash adds everything the facet rests on: the code under a model, the dependencies' contracts, the
  targets it applies to, the workspace and toolchain files the code is built with.

A review names a bound hash. So a review is of exactly the content that hash covers, and any change to any of it,
a comment in the code under a model included, leaves the review stale. Nobody has to remember to lower a status.

## Status is derived, never written

A facet's evidence status is `production`, `rejected`, `stale` or `unreviewed`, and it is computed from reviews,
never written by hand:

- a **production** review holds only at the hash it names;
- a **rejection** holds until a later review answers it by name, whatever the content becomes. It follows the
  content it was about, the same fact, cost, file bytes or text, into any other record, so copying rejected content
  under a new name does not shed it;
- every review is kept in an append-only ledger in the lock, and is checked where it was ledgered, on every load.

A record may sit in the **production** namespace only while all four of its facets are `production`. Anything may
sit in the **experimental** one, which is how unreviewed knowledge "may exist ... but cannot silently satisfy a
stronger production claim" (`ROADMAP.md` §9).

## Claims cite what they read

The analysis consults the catalog by name. Every lookup returns what it found and the record it came from, so a
claim's citations are exactly what it read, never a list someone wrote. Its closure follows the hashes: every facet
whose hash enters one it read. When anything in that closure later changes, is rejected or is demoted, the claim is
found and flagged.

A **production claim** is admitted only when every catalog input comes from production records, no input came from
the caller, the timings it read are a board's, measured on the very image the claim is about, and that image was
built only from reviewed code. Before `M4` builds images, no production claim can be made at all, and the design
says so rather than pretending otherwise.

## What it defends against

The design's threat model, which the director ruled, takes in every change that reaches the catalog through the
repository: a hand-edited lock, a bypassed local check, a forged review. Each is refused, or makes the affected
reviews stale and the affected claims flagged. It rests on four premises, each with the cheap checks it gets:

1. the toolchain is the pinned release;
2. git reports the repository as it is;
3. the published main line is protected;
4. the machine running a check is not working against it.

The third premise needs settings only the director can turn on, which the findings record lists (§11). They
include a second person to approve changes to the checking tool, since a sole author approving their own change
protects nothing. How the check itself is protected from what it judges is the leaf `M2.7.6`, with a test for
every attack the reviews found.

## How it was reviewed

Each round was a new context that had not written the design, reading it with the repository's code and history
to hand, and trying to break it. Rounds 1 to 11 found defects in the catalog's own mechanics, fewer each time. The
director then let the engineer judge the loop, and a closure rule was set: the design closes on the first round
that finds nothing live for the first version's slice, and a defect that bites only later (a physical board, built
images, the port's assembly, a second rules version) is answered in the design and owned by that later leaf.

Rounds 12 to 15 found holes only in how the check that guards the catalog is protected, so that mechanism became a
leaf of its own. Round 16 found no defect live for the slice, and the design was accepted in the change that
answered it. Every round recomputed the worked example's 23 hashes by two independent routes, and every round
found them right.

The composition of four of the analysis's inputs from catalog, application and plan parts,
`docs/decisions/catalog/decision_runtime-composite-inputs.md`, was reviewed the same way under the same closure
rule. Its rounds found defects in which platform facts the composition needs and how they are worded, several of
them live on the emulator, whose interrupt order [QEMU](ledger.md#qemu)'s source showed differs from the
specification's. Round 11 found none live, and the record was accepted. Its history is kept in
`docs/reviews/decision_runtime-composite-inputs-reviews.md`.

## What it does not do yet

- **Nothing loads a catalog.** `catalog/` is empty. The crate reads records, computes their hashes and checks the
  lock, over one tree and over a history, derives evidence status, checks each review where it was recorded and
  traces invalidation so far; the namespaces, lookups and claims are the rest of `M2.7.3`, and the gate that gives it the history is `M2.7.4`'s.
- **The port's facts are unknown.** The architecture port is assembly, which no record can hold yet, so every
  analysis of the runtime variant over the catalog is inconclusive until `M2.12` gives the port's code a record
  format.
- **No surface makes a production claim.** That needs images, which are `M4`'s, and `M4.10` holds everything the
  design leaves to it.
- **Premise 3 is unmet** until the director turns on the hosting settings and names a second reviewer.
