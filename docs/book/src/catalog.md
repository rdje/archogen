# Where the engine's knowledge comes from: the catalog

## The idea, in plain words

A description says *what* a system must do. To build it, archogen needs to know *how* — how a scheduler behaves,
what a timer on a particular chip does, how long switching between tasks takes. That knowledge lives in the
**catalog**: a collection of reviewed parts archogen assembles systems from.

Think of a cookbook in which every recipe has been tested by someone, signed and dated, and sealed so that you would
notice if one word had changed since. Each part in the catalog is a **record**: what it is, what it promises, what it
needs, the code that does it, and the facts and timings it states. Each record is sealed by a **hash**, a fingerprint
of its exact content, so a review is a review of exactly those bytes; change one character and the review no longer
counts, without anyone having to remember to say so. When a report relies on a part, it says which version of which
record it relied on, and whether that record was reviewed.

> **In one minute, for engineers.** `ROADMAP.md` §9's catalog, designed in `docs/specs/catalog/decision_catalog-records.md`
> and accepted after its independent review rounds. A record under `catalog/` has four facets — contract,
> implementation, behavioral model, timing model — each with an own and a bound hash over a normative byte grammar.
> Evidence status (`production`, `rejected`, `stale`, `unreviewed`) is derived from an append-only review ledger,
> never written; claims cite exactly what their lookups read; a production claim needs production inputs, a board's
> timings and a protected main line. The `archogen-catalog` crate implements it; the gate, the check's protection and
> the first records come next, so `catalog/` is empty today.

## How it works

1. **A part is written as a record**, with its four facets, in the same syntax eADL is read with.
2. **Its hashes are computed**, never written, so the record cannot claim a version its content does not have.
3. **A reviewer reviews it** by naming a hash; the review is ledgered, and holds only for that content.
4. **Its status is derived** from the reviews: `production` only where a review names its current hash.
5. **A claim reads it** through a lookup that returns what it found and where from, so the claim's citations are
   exactly what it used, and a later change to any of it flags the claim.

## The precise rules

### What the crate does today

This chapter explains the design. It was reviewed independently sixteen times before it was accepted, and the
`archogen-catalog` crate implements all of it (`M2.7.3`, `M2.12.4`). It reads records and refuses what breaks the
rules, each refusal with its one code; computes every hash, reproducing the worked example's 23 values; checks the
lock over one tree and over the history; derives each facet's evidence status; names what a later change
invalidates; answers the analysis's lookups; holds the production namespace; admits claims, judging premise 3; and
holds the code a record points at to the package rules, the port's assembly to its dialect. The gate that runs it,
and the records, are *Today and ahead*'s.

### The port's record

The port is the small part of the runtime written in the processor's own language: the code that runs when an
interrupt arrives, that switches from one task to another, and that turns interrupts off and on. Rust cannot say
those things, so they are assembly, and the catalog at first refused assembly outright, since assembly can reach code
no review sees. The port's record admits it narrowly (the design's §14,
`docs/specs/catalog/decision_catalog-records-port.md`):

- **Where it may be.** A record declares which of its packages hold assembly, and for which processor family. Only
  there, and only inside a function built for the bare-metal target, may assembly appear.
- **What it may say.** Each line is one instruction from a short list, each operand checked by its position. Other
  code is reached only through a Rust name the compiler resolves; there is no assembler directive; and an inline
  block declares to the compiler every register it touches.
- **What the port must state.** The fault contract leaves the port choices: how it catches a primitive called from
  outside a job, whether a trap serves one interrupt or several, how a failed check reports what it found, how a
  stack's guard is noticed. Its record states each as a fact beside the code, `unknown` where nobody knows yet, and
  the ones the contract requires are never `no`. One choice is shared: every record whose code makes a check depends
  on the same *check-passing convention* record as the port, which the catalog matches mechanically.
- **What it rests on.** The format leans on what the pinned compiler and assembler do — how they align the trap
  entry, how they read labels and numbers — and each such premise is re-checked on every integration run.

### The records that hold it

| File | What it holds |
| --- | --- |
| `docs/specs/catalog/decision_catalog-records.md` | the design: what a record holds, how status and claims work, what it defends against |
| `docs/specs/catalog/decision_catalog-records-hashes.md` | its §3, the byte grammar every hash is computed over, and what the gate builds |
| `docs/specs/catalog/decision_catalog-records-example.md` | a worked example of records and the files they name, with every hash they have, which the catalog crate must reproduce |
| `docs/specs/catalog/decision_catalog-records-variant-inputs.md` | its §12, what the runtime analysis takes from the catalog, and from whom |
| `docs/specs/catalog/decision_catalog-records-limits.md` | its §13, what the design does not do |
| `docs/specs/catalog/decision_runtime-composite-inputs.md` | how four of the analysis's inputs are put together from catalog, application and plan parts |
| `docs/reviews/decision_catalog-records-reviews.md` | every review round, every finding, and the answer to each |
| `docs/specs/catalog/decision_catalog-records-port.md` | its §14, the port's assembly and what the port's record must state |
| `crates/archogen-catalog/src/record.rs` | one record read, and every rule of the design that needs only the file, each refusal with its one code |
| `crates/archogen-catalog/tests/record.rs` | one valid record and one change of it per rule, each refused with that rule's code |
| `crates/archogen-catalog/src/hash.rs` | every hash: each facet's own and bound hash, the record's, a review's, over the files, packages, targets and ledger sections they rest on |
| `crates/archogen-catalog/tests/hash.rs` | the worked example, read from its own file, with all 23 of its values |

### A record, and its four parts

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

### Hashes: what a review is a review of

Every facet has two hashes, and neither is written in the record; both are computed.

- Its **own** hash covers the facet's own text and files. It pins the facet's version in the catalog's lock, so a
  facet that changes without its version moving is refused.
- Its **bound** hash adds everything the facet rests on: the code under a model, the dependencies' contracts, the
  targets it applies to, the workspace and toolchain files the code is built with.

A review names a bound hash. So a review is of exactly the content that hash covers, and any change to any of it,
a comment in the code under a model included, leaves the review stale. Nobody has to remember to lower a status.

### Status is derived, never written

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

### Claims cite what they read

The analysis consults the catalog by name. Every lookup returns what it found and the record it came from, so a
claim's citations are exactly what it read, never a list someone wrote. Its closure follows the hashes: every facet
whose hash enters one it read. When anything in that closure later changes, is rejected or is demoted, the claim is
found and flagged.

A **production claim** is admitted only when every catalog input comes from production records, no input came from
the caller, the timings it read are a board's, measured on the very image the claim is about, and that image was
built only from reviewed code. Before `M4` builds images, no production claim can be made at all, and the design
says so rather than pretending otherwise.

### What it defends against

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

### How it was reviewed

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
`docs/specs/catalog/decision_runtime-composite-inputs.md`, was reviewed the same way under the same closure
rule. Its rounds found defects in which platform facts the composition needs and how they are worded, several of
them live on the emulator, whose interrupt order [QEMU](ledger.md#qemu)'s source showed differs from the
specification's. Round 11 found none live, and the record was accepted. Its history is kept in
`docs/reviews/decision_runtime-composite-inputs-reviews.md`.

## Today and ahead

- **The gate loads the catalog at every commit, and it is empty.** `cargo xtask catalog-check` reads the history
  from git and judges the pending commit, a commit as made, or a pull request's merge against its base (`M2.7.4.2`),
  building every package a record names, for each profile and target, and reading what the compiler read
  (`M2.7.4.3`), run from the hooks on every commit and blessing the lock on request (`M2.7.4.4`); the records
  themselves wait on the director (`M2.7.4.5`). The crate behind it is complete:
  it reads records, hashes them, checks the lock over one tree and a history, derives status and admits claims.
- **No port's record exists yet.** Its format, its statement and the loader's checks are done (`M2.12`). Until a
  port's record is written, every analysis of the runtime variant over the catalog is inconclusive.
- **No surface makes a production claim.** That needs images, which are `M4`'s, and `M4.10` holds everything the
  design leaves to it.
- **Premise 3 is unmet** until the director turns on the hosting settings and names a second reviewer. Its
  repository half has begun: every workflow holds a read-only token and keeps no credentials (`M2.7.6.1`), and
  the harness that builds the checker from the base and runs it on the judged tree is written and tested against
  the constructions the reviews found (`M2.7.6.2`).
