<!-- LIVE-DOCUMENT-SIZE-CONTAINMENT-LOCAL-ADOPTION:BEGIN -->
## Local adoption note — archogen

- **Authority:** the director's standing instruction, which names live-document size containment an
  adopted policy. Adopted `2026-09-30` by leaf `PROGRAM.17.1` (`docs/tasks/PROGRAM.md`).
- **Source, read-only, never written to:** the neutral body below is copied verbatim from fsmgen's
  `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` at `0fa794310` (lines 190–529; the body's sha256 is
  `af130de4d7bbeef6db532b0cea4ba25a07ce131c6a44511cbe25d8a91c037aa2`). The adoption route followed is fsmgen's
  `docs/LIVE_DOCUMENT_SIZE_CONTAINMENT_ADOPTION_GUIDE.md` at `727e0d086` (sha256
  `8f77fa39c9bcb9cfc43166259a627a6ced64682030088400b727dccc5d674a53`). The origin is a template, not an
  upstream. A later revision there is adopted only by a new, owned review here. No fsmgen threshold, path,
  surface identifier or conclusion is copied: everything in this note was measured in this repository.
- **Adoption state:** this note and the inventory below (`PROGRAM.17.1`). The bounded snapshots and their
  checker are next (`PROGRAM.17.2`). The lifecycle choices that would change what the director browses go to
  the director first, as the guide's stop condition requires (`PROGRAM.17.3`).

### Inventory, measured `2026-09-30`

Sizes are lines / bytes / longest line in bytes.

| Surface | Lifecycle | Measured | Bound today, and what enforces it | Scaling term | Decision |
| --- | --- | --- | --- | --- | --- |
| `MEMORY.md` — the resume pointer | `bounded_snapshot` | 41 / 3 980 / 705 | ≤ 50 lines and ≤ 7 168 bytes, by `MEMORY-ARCH`; no line-width bound | its "Closed today" line grows by one entry per closed leaf | done: chronology removed, longest line bounded (`PROGRAM.17.2`) |
| `LIVE_STATUS.md` — the status board | `bounded_snapshot` | 21 / **42 110** / **30 256** | none | a closure note is appended to a tree's row for each closed leaf; one row holds 30 256 bytes | was **transition debt at adoption**; done: current state only, bounded on three axes (`PROGRAM.17.2`) |
| `docs/TASK_TREE.md` — the tree index | `bounded_snapshot` | 73 / 7 199 / 1 158 | none | a "Closed" list inside each row grows per closed leaf | done: current state only, bounded (`PROGRAM.17.2`) |
| `README.md` — the landing page | `bounded_snapshot` | 82 / 4 062 / 125 | ≤ 110 lines, ≤ 6 144 bytes and ≤ 200 bytes per line, by `LIVE-SNAPSHOTS`; `README-STABILITY`'s template 300 / 16 384 is a backstop | none observed | retain; longest line bounded (`PROGRAM.17.2`); lines and bytes derived from the reviewed survivor (`PROGRAM.35.1`, `README_POLICY.md`) |
| `CHANGELOG.md` | `rolling_ledger` | 3 428 / 277 796 / 185 | 20 to 39 entries, by `HISTORY-LEDGERS` | one entry per commit: 1 464 lines when `PROGRAM.17` was filed | sealed by entry count into `docs/history/changelog/` (`PROGRAM.31`, ruled on §8 by delegation); 21 200 bytes live after the first seal |
| `DEV_NOTES.md` | `rolling_ledger` | 1 602 / 135 158 / 132 | 10 to 19 notes, by `HISTORY-LEDGERS` | one entry per lesson, each promoted to `docs/knowledge/` (`LESSON-PROMOTION`) | sealed into `docs/history/dev-notes/` as for the changelog (`PROGRAM.31`) |
| `docs/history/` — the sealed segments and their index | `archive_terminal` | 12 files / 390 301 bytes at the first seal | every segment against its digest, the index append-only, by `HISTORY-LEDGERS` | one segment per 20 changelog entries or 10 notes | retain: immutable, indexed, read as files, in no bootstrap read (`PROGRAM.31`) |
| `docs/tasks/*.md` — 14 task trees | `partitioned_canonical`, one tree per file, closed subtrees sealed out | 698 202 in aggregate; `M1.md` 2 278 / 259 334 after its seal | ≤ 20 files, ≤ 3 000 lines and ≤ 327 680 bytes a file, ≤ 3 072 bytes a line, ≤ 819 200 in total, by `README-ROUTES` | every open leaf and log row | closed subtrees sealed to `docs/task-history/` (`PROGRAM.32`) |
| `docs/task-history/` — 69 sealed subtrees and an index | `archive_terminal` | 756 207 in aggregate; `M1/M1.13.md` 1 511 / 138 922 | every file against its row, the index append-only, every leaf against its stub, by `TASK-HISTORY`; ≤ 1 800 lines, ≤ 163 840 bytes and ≤ 2 048 bytes a line a file, by `README-ROUTES` | one file per closed subtree | retain: immutable, indexed, in no bootstrap read (`PROGRAM.32`) |
| `ROADMAP.md` | `maintained_reference`, owned by the director | 953 / 104 333 / 623 | none; it changes with the program's scope | scope | retain; not archogen's to restructure |
| `KNOWLEDGE_MAP.md` | `generated_projection` | 178 / 14 491 / 113 | freshness, by `KNOWLEDGE-MAP` | one card per promoted lesson | retain; it is generated, never edited |
| `docs/decisions/` — 27 records, a template and an index | `partitioned_canonical` | 4 388 / 332 029; index 36 / 10 008 / 534 | the index lists every record, by `MEMORY-ARCH`; ≤ 40 files and ≤ 393 216 in total, raised once (`decision_decisions-folder-ceiling.md`), with its per-file limits, by `README-ROUTES` | one record per decision | partitioned by subject at 36 files or 360 000 bytes, adding no capacity (`PROGRAM.39`) |
| `docs/reviews/` — 4 review histories and an index | `partitioned_canonical`, one file per reviewed design | 596 / 73 298 / 600 | ≤ 16 files, ≤ 1 200 lines and ≤ 131 072 bytes a file, ≤ 1 024 bytes a line, ≤ 262 144 in total, by `README-ROUTES` | one round per review of an open design, then frozen | retain; in no bootstrap read |
| `docs/knowledge/` — 22 cards and an index | `partitioned_canonical` | 1 891 / 123 527; index 33 / 5 940 / 643 | complete, by `KNOWLEDGE-MAP` | index rows grow with each card's questions | retain; watch the index's row width |
| `docs/book/src/` — 22 chapters | `maintained_reference` | 3 305 / 179 500 | content, by `BOOK-ANCHORS` and `BOOK-COVERAGE`; none on size | product scope | retain; per-chapter bounds are future work |
| Spine documents: `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`, `MEMORY_ARCHITECTURE.md`, `COMMIT.md` | `maintained_reference` | `DOCTRINE_ENFORCEMENT.md` 96 / 21 907 / 2 168; `TOOLBOX.md` 74 / 15 175 / 1 439 | none | one row per doctrine or tool, each row long | retain; watch row width |

### Bounds of the snapshots

The inclusive enforcement ceilings `LIVE-SNAPSHOTS` (`scripts/check_live_snapshots.sh`) holds. A file equal to a
ceiling passes; one byte more fails. Each ceiling was set on `2026-09-30` from the reviewed survivor, after
`PROGRAM.17.2` returned the file to current state, plus room for one ordinary change: a new tree's row, or a
longer frontier sentence. `README.md`'s lines and bytes were set the same way by `PROGRAM.35.1`, from the page
that survived `README_POLICY.md`'s review, and replace the scaffold's template defaults. A dimension another
doctrine already owns is marked with that doctrine and not repeated here. Lowering a ceiling is free. Raising one needs a new decision record that shows the surface's
role grew; editing this table alone does not authorize it.

| Surface | Lines | Bytes | Longest line | Survivor measured |
| --- | --- | --- | --- | --- |
| `LIVE_STATUS.md` | 32 | 4096 | 320 | 22 / 2 324 / 220 |
| `docs/TASK_TREE.md` | 96 | 6144 | 320 | 73 / 4 469 / 189 |
| `MEMORY.md` | MEMORY-ARCH | MEMORY-ARCH | 200 | 26 / 1 770 / 120 |
| `README.md` | 110 | 6144 | 200 | 84 / 4 173 / 125 |

Two decisions are recorded rather than left to drift:

- **`CHANGELOG.md` and `DEV_NOTES.md` are rolling ledgers** (`PROGRAM.31`, `2026-09-30`). They first grew
  deliberately while the choice waited on the director, since rotating them changes what the director browses. The
  director delegated the ruling, and the boundary chosen is a count of entries, not a month, because the whole
  history was one month (`docs/decisions/decision_history-ledgers.md`). The task trees' closed leaves follow in
  `PROGRAM.32`.
- **The three snapshots were transition debt, now paid** (`PROGRAM.17.2`, `2026-09-30`). Each had grown a
  chronology duplicating the canonical record, the tree's Commit Log and the changelog. Before anything was
  removed, every leaf the snapshots mentioned (60, 43 and 39 distinct ids) was checked to be declared by its
  tree, and every closed leaf was checked to have its Commit Log row. The removed text remains exactly
  retrievable: `git show 2ae5559:LIVE_STATUS.md`, sha256 `4910305c…`, 42 110 bytes, with the same for the other
  two, recorded in the leaf. `COMMIT.md` now says a snapshot changes only when the state it shows changes.
<!-- LIVE-DOCUMENT-SIZE-CONTAINMENT-LOCAL-ADOPTION:END -->

---

# Live-Document Size-Containment Doctrine

This project-neutral, project-agnostic, and harness-neutral doctrine keeps
long-lived documentation useful as a bounded working set while preserving
durable information in addressable storage. It applies to human-authored
documents, generated indexes, work records, manuals, ledgers, and historical
archives. It does not prescribe a product domain, authoring tool, agent, build
system, or repository layout.

## Authority and adoption

After adoption, the adopting project owns its copy. Cite the project's owner
and adoption or revision date together with the project-owned doctrine file.
Never cite a vendor-, agent-, or harness-specific bootstrap file as authority.
Bootstrap files may make the doctrine discoverable; they do not make it
binding.

Keep project-specific authority, paths, thresholds, measurements, storage
choices, and migration decisions in a clearly fenced local adoption note or a
separate data registry. Keep them out of this neutral body. The copied doctrine
is authoritative; its origin is a template rather than an upstream, so later
changes are adopted only through explicit local review.

## Core invariant

A bounded live view and a durable history are different products. The live
view answers what a reader needs now; the durable store preserves what must
remain recoverable. No surface may be both an indefinitely growing history and
a mandatory current read.

Every governed surface must declare:

- its stable identifier, owner, audience, and canonical authority;
- its lifecycle and storage topology;
- independent reviewed health targets and inclusive enforcement ceilings for
  lines, bytes, and maximum content-line bytes, plus file-count and aggregate
  dimensions for collections unless a product-sized maintained-reference
  contract replaces fixed aggregate caps with exact per-change authority;
- warning and rollover-required milestones measured against health targets;
- the operation that bounds it: overwrite, partition, regenerate, seal,
  rotate, archive, supersede, or freeze;
- how a reader finds current material and retrieves retained history; and
- the mechanical check that rejects missing, stale, cyclic, or over-limit
  declarations.

Routing is transitive. A bounded file that sends overflow to an unbounded
neighbor has not contained anything.

Reader navigation and author overflow are different route kinds. A reader may
legitimately navigate to immutable history. Author guidance may also identify
exact history already captured by the durable version-history workflow, but it
must not redirect appendable prose into an immutable or frozen terminal.
Inventory both without forcing them to have the same destinations. Derive
author candidates from enforcers' emitted failure guidance as well as
hand-authored route data; an undeclared path-shaped hint is a real pressure
edge and must fail closed.

A collection front door must state what completeness means. A literal table of
contents proves every member, a generated index names its reproducible source,
or an explicit query contract defines the complete target expansion. Presence
of an index file alone proves none of these. Likewise, a human evidence map is
only trustworthy when every fenced repository-relative path resolves
mechanically in the resulting tree.

## Boundedness, currency, and truth

Boundedness does not imply currency, and currency does not imply unrestricted
semantic truth. Size and route checks prove only their declared structural
properties. A current surface may opt into a named, lifecycle-specific
currency contract backed by a calibrated local verifier. Only that declaration
authorizes the currency claim; the verifier must execute in the unconditional
doctrine path and its failure must fail closed.

The neutral doctrine never infers staleness from the newest date, the number of
distinct dates, file age, or a universal threshold. Closure facts legitimately
put old dates in current roadmaps, while archive and frozen dates are expected
to remain old. Historical terminals and frozen records are therefore exempt
from current-state contracts. A local verifier may detect a document's exact
self-contradiction or compare a projection with its canonical source, but its
grammar and false-positive calibration belong to the adopting project.

## Derived-state containment

Mechanically owned current state is a separate truth boundary. If a canonical
system can answer a field's question exactly, deterministically, and cheaply,
the field is derived state rather than independent information. Classify each
such maintained current-state field in one of two ways:

1. **Derive on read.** Do not store the value. Keep the exact command or
   accessor at the reader's point of need so the answer is computed from its
   authority when requested. A value invalidated by the commit or write that
   records it must always use this class; periodic correction cannot make a
   self-invalidating copy coherent.
2. **Verified copy.** Retain a value only because the copy itself is a
   deliberate contract, published baseline, or bounded projection. Name its
   canonical authority and recomputation method, and execute a verifier that
   fails whenever the stored copy disagrees. Declaring a verifier without
   running it is not verification.

Judgement, intent, rationale, ownership, blockers, and a deliberately selected
next action are not mechanically derivable and remain ordinary authored
content. An immutable evidence snapshot is also distinct from a mutable claim
about now: it must name an exact capture boundary such as a revision, digest,
invocation, or externally owned observation and remain under the applicable
evidence, retention, archive, or frozen-identity contract. Removing the
boundary or relabeling the snapshot as current turns it back into an
unverified copy.

Field discovery is declared, not guessed. The adopting project keeps an
explicit bounded list of governed paths and exact field markers; the neutral
checker contains no project-specific names and does not infer semantics from
dates, number shapes, or prose. Generated projections with declared canonical
inputs and executed freshness already satisfy the verified-copy rule at
surface scope.

Before demoting a duplicate, compare it with its authority and inspect the
authority for divergence the convenient copy may have concealed. Preserve the
reader's question in place through the derivation, repair the canonical source
first if it is wrong, and only then remove the duplicate. A smaller document
that silently loses the answer or preserves a defective authority is not a
successful containment migration.

## Lifecycle classes

| Class | Purpose | Required containment |
| --- | --- | --- |
| `bounded_snapshot` | Current state, resume pointer, or concise landing/index view | Overwrite semantics, no embedded chronology, line/byte limits, and a stale-state check where derivable |
| `partitioned_canonical` | Finite or contract-bounded canonical material whose full content remains directly browsable | Stable semantic partitions, bounded table of contents, per-part/file-count/aggregate limits, and link/reconstruction checks |
| `maintained_reference` | Unique maintained product/specification prose whose aggregate follows legitimate product scope | Auditable audience/role/rationale, stable semantic parts, bounded complete mandatory index and navigation depth, per-part limits, and exact fresh authority for every aggregate change |
| `generated_projection` | Search map, catalog, or index derived from smaller canonical units | Reproducible generation, freshness proof, bounded root view and shards, and no unique facts in generated output |
| `rolling_ledger` | Ordered recent entries with historical value | Bounded current window, deterministic seal/rotation boundary, immutable segments, bounded index, and an archive transition before aggregate growth becomes unbounded |
| `archive_terminal` | Exact historical evidence not needed in ordinary reading | Immutable locator, identity and size proof, tool-neutral retrieval procedure, retention owner, and exclusion from mandatory bootstrap reads |
| `external_terminal` | History retained by an independently managed system | Named authority, retention commitment, stable query/export contract, and a failure policy if that contract disappears |
| `frozen_legacy` | Existing record awaiting an owned lifecycle decision | Exact content identity or equivalent write prohibition; it cannot accept new content or act as an overflow destination |

A local adoption may define additional classes, but each must make growth stop,
become predictably partitioned under fixed bounds, or govern product-scope
change through exact fresh authority. Renaming an append-only blob is not a new
lifecycle.

## Choose the storage topology from the information role

Do not shard mechanically by arbitrary line count. Classify the information
first:

1. Current state belongs in an overwritten bounded snapshot.
2. Unique maintained material that people browse belongs in semantic,
   navigable partitions. If its aggregate follows legitimate product scope,
   classify it as maintained reference and bound the read path plus each part
   while authorizing every aggregate change exactly.
3. A projection that can be recreated belongs in generated bounded shards; its
   smaller canonical inputs remain authoritative.
4. Exact chronology or evidence that is rarely read belongs in a query-first
   archive after its current window closes.
5. Content already present in a richer canonical source is proved duplicate,
   then removed with a link rather than copied into another store.

Sharding controls per-read and per-file pressure, but it does not by itself
control aggregate storage. An ordinary partitioned collection keeps aggregate
limits. A maintained product reference instead measures aggregate files/lines/
bytes and requires an exact, newly owned change record whenever they move;
fixed aggregate targets are explicitly inapplicable rather than silently
unlimited. A rolling ledger must still declare when sealed segments leave the
live collection for an archive terminal. The chosen topology must therefore
bound the reader's working set and either bound or exactly govern the
collection's long-term aggregate change according to information role.

## Derive pressure limits from the retained surface

Measure the deliberately reviewed live survivor and set independent line,
byte, and maximum content-line-byte health targets. For collections, also set
per-part, file-count, and aggregate targets. Maximum line width is a separate
pressure axis: a generated table or dense record can remain pathological while
the file still passes total-line and total-byte limits. Measure raw content
bytes deterministically, excluding LF and an optional preceding CR. Do not copy
illustrative numbers from another adoption, and do not treat a current legacy
size as healthy merely because it was measured.

The exception is unique maintained product/specification prose. A fixed
aggregate target there is dishonest because legitimate scope changes with the
product. Require fixed per-part limits, a complete mandatory index with its own
line/byte bounds, bounded direct navigation, and exact aggregate baseline plus
signed per-change delta. A revision-aware adapter must reject stale, inexact,
reused, or banked authority. Classification alone never waives a monolith's
existing debt; semantic partition and complete navigation land first.

For every measured dimension, declare two different values:

- the **health target** describes the reviewed steady-state working set and is
  the denominator for warning and rollover pressure; and
- the **inclusive enforcement ceiling** rejects only `actual > ceiling`. It is
  a quarantine boundary, not evidence that content below it is healthy.

Each local registry selects two ordered milestones against the health target:

- **warning**: investigation and an owned remediation become mandatory;
- **rollover required**: ordinary appends stop unless the same change performs
  the declared rollover.

The warning must leave enough capacity for the largest normal update plus the
rollover transaction. A ceiling increase requires a new, separate, reviewed
authority record proving that the surface's user contract expanded; editing
the surface declaration alone cannot authorize itself. Lowering is free.

At first adoption, a surface already beyond warning or rollover may be entered
as explicit transition debt only with its exact measured baseline, named
remediation owner, deadline or ordered frontier, and unchanged ceiling.
Only records required to complete the containment transition may extend a
rollover-debt surface; ordinary unrelated growth remains prohibited. The debt
exception ends when the migration lands and can never excuse ceiling overflow.
Its baseline cannot increase across revisions, but an atomic content reduction
may lower it so the ceiling can ratchet down. The current baseline plus owned
allowance must fit below the ceiling. A
declared ratchet band must also reject a ceiling that remains materially above
both actual use and the health target after pressure falls.

## Atomic partition, rollover, and archive protocol

A transition is complete only when one change performs and verifies all
applicable steps:

1. Stop writes to the source at a stable semantic, record, or time boundary.
2. Classify retained information as canonical, derived, duplicate, or archival.
3. Write the new partition, sealed segment, or archive record without altering
   record order or identity.
4. Record source and destination line counts, byte counts, and content digests;
   when exact reconstruction is promised, prove it byte for byte.
   Report complete-source identity, semantic closure, the live working-set
   dimensions, and any truly unretained residue as four independent products.
   Declare whether products overlap; never force disjoint arithmetic onto
   retained source, semantic extracts, and live navigation views.
5. Update the bounded current view, manifest, table of contents, predecessor/
   successor links, and query route.
6. Run link, freshness, ordering, uniqueness, retrieval, and pressure checks;
   a declared executable must actually run successfully, not merely exist.
7. Compare each duplicate with its authority, repair any divergence the copy
   concealed, and only after those checks pass remove a live duplicate whose
   derivation, retained copy, or archive retrieval has been proved.
8. Commit the transition atomically so no durable state exposes half a move.

For a task-tree ledger, bound live narrative fields independently from the
file and collection ceilings. When repeated optional verification/checklist
history becomes the scaling term, retain the complete live node graph, stop at
one exact predecessor, and replace only the historical suffix with a finite
manifest pointer. The verifier must retrieve and authenticate both the complete
predecessor and the exact removed suffix, require a unique live pointer, count
embedded markup as source bytes, and reject string-encoded numeric evidence.
Structural child indexes remain governed by graph closure rather than the
narrative-field cap.

Sealed units are immutable. Corrections create a superseding record or segment
rather than silently editing archived evidence.

## Archive descriptor contract

When bytes leave the live collection, a small tracked descriptor must preserve
at least:

- schema version and stable surface identifier;
- former logical path and covered record, topic, or time range;
- immutable revision or object locator;
- line count, byte count, and content digest before removal;
- a repository-root-relative or otherwise portable retrieval procedure;
- the current-view, manifest, and replacement pointers;
- the sealing reason, date, and verifier identity; and
- an executable proof that retrieval reproduces the declared content.

A version object is a conditional retention mechanism, not a self-proving
archive. Every version-object use must name a bounded retention contract with
an owner, an explicit reachability/backup guarantee, and an actionable
recovery procedure for shallow history or rewritten objects. If evidence must
remain recoverable without that condition, prefer a content-addressed file on
the repository volume.

Retrieval must not depend on a particular AI agent, editor, or harness. An
archive descriptor is a controlled terminal, not permission to route new live
content into an opaque dump.

## Registry and mechanical enforcement

Keep local declarations in data-only registries consumed by one deterministic
checker. Each registry must itself be finite: begin with schema-versioned
metadata declaring positive maximum data-record count, total file bytes, and
raw JSON bytes per record; impose portable fail-safe ceilings as well as the
adopter's tighter limits. Arrays need finite cardinalities, scalar fields need
byte limits, identifiers need closed domains, and unknown fields fail closed.
The registries are the local authority for class, paths, limits, milestones,
generation or retrieval checks, and dependency routes. The checker must run
on the resulting tree for every commit and continuous-integration build,
independent of which paths changed.

At minimum, fail on:

- an undeclared live surface or routing destination;
- missing, malformed, oversized, or over-populated control-plane registry
  metadata, record, scalar, or array;
- an absolute, escaping, or otherwise forbidden persisted path;
- a missing owner, lifecycle, limit, or retrieval/freshness control;
- a route cycle or a route ending at an uncontrolled neighbor;
- a stale generated projection or broken current/history link;
- a declared derive-on-read field marker that remains stored, a missing reader
  derivation, an off-surface field contract, or a verified copy whose authority
  verifier is absent, unexecuted, degraded, or failing;
- a mutable sealed/frozen unit or failed archive digest/retrieval proof;
- a version object without a named retention owner, guarantee, and recovery
  action, or migration evidence that conflates overlapping products;
- warning without an owned remediation, rollover-required without the atomic
  transition, actual usage above an inclusive ceiling, or stale debt headroom;
- an unauthorized ceiling increase or rewritten debt baseline; and
- an unclassified maintained reference, oversized part or mandatory index,
  incomplete direct navigation, aggregate mismatch, or stale/reused/banked
  aggregate-change authority.

Generated caches may accelerate the checker or search, but they are disposable
and never the canonical copy.

## Adoption checklist

1. Add a project-owned copy and fence all local metadata away from this body.
2. Inventory every live document, generated view, collection, route, and
   historical terminal; follow routes transitively.
3. Classify each surface by lifecycle and identify its actual canonical source.
4. Classify mechanically owned current-state fields as derive-on-read or
   verified copies; publish exact local markers, authorities, derivations, and
   verifier contracts without heuristic field discovery.
5. Measure lines, bytes, maximum content-line bytes, file counts, aggregates,
   structure, and read path.
6. Derive health targets from reviewed survivors and set separate inclusive
   ceilings with only transaction-sized headroom.
7. Open an owner for every surface already at warning or structurally
   monolithic even if it remains below a numeric threshold.
8. Choose bounded snapshot, semantic partitions, maintained reference,
   generated shards, rolling ledger, archive, external, or frozen topology
   from the information role.
9. Prove any duplicate and compare its authority before deletion; prove any
   archive before removing its live copy.
10. Add the data registry, unconditional checker, positive/fail-closed tests,
   and commit/CI wiring.
11. Re-audit after each migration and periodically thereafter; lower limits to
    the retained steady-state surface instead of preserving legacy headroom.
