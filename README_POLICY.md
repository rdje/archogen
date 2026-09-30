<!-- README-POLICY-LOCAL-ADOPTION:BEGIN -->
## Local adoption note — archogen

- **Authority:** the director's standing instruction, which names this policy. Adopted at this revision on
  `2026-09-30` by leaf `PROGRAM.35.1` (`docs/tasks/PROGRAM.md`). This repository-root file is the authoritative
  copy. `CLAUDE.md` and `AGENTS.md` may point here, but they are not its authority.
- **Source, read-only, never written to:** the neutral body below the rule is copied verbatim from fsmgen's
  `README_POLICY.md` as last changed at `1f0443b3a`: its lines 29–187, 159 lines and 8 279 bytes, sha256
  `77a1e9348ec24d9ec5f0c97ae1ac2d634f7e7e3e150504759af3c0182d6eefec`. The origin is a template, not an upstream,
  so a later revision there is adopted only by a new, owned review here. The body replaces an earlier copy of the
  same lineage, which the `bedrock` scaffold transferred in the initial commit. `scripts/update_scaffold.sh` still
  lists this file as neutral, but since `PROGRAM.26` it offers a merge for a changed file instead of overwriting
  it, so a scaffold update cannot silently undo this adoption.
- **Caps, derived here:** the README that survived `PROGRAM.35.1`'s review measured 84 lines, 4 173 bytes, and
  125 bytes on its longest line. The binding ceilings are **110 lines, 6 144 bytes and 200 bytes per line**,
  which leaves room for one ordinary change to the navigation. They are data in `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`,
  under `### Bounds of the snapshots`, and `LIVE-SNAPSHOTS` holds them on every commit and in CI, whatever is
  staged. `README-STABILITY`'s 300 lines and 16 384 bytes are the scaffold's template defaults. They stay as an
  outer backstop, and as the gate that prints the routing hint, but they are not this project's caps. Raising a
  ceiling needs a decision record showing that the landing page's contract grew.
- **Landing-page identity:** `README.md` is the repository's rendered landing page. Containment moves changing
  detail and chronology out of it; it does not move out the landing function.

### Routed destinations

The registry `README-ROUTES` (`scripts/check_readme_routes.sh`, leaf `PROGRAM.35.2`) holds on every commit. Its
population is derived, not listed here: every link in `README.md` (`navigation`), every path the README's two guards
print when the page is over its caps (`overflow`), every path this policy's body names in a code span, and every onward route below,
followed until nothing new is reached. A path is governed by its own row, or by the row of the deepest registered
directory above it. Ceilings are inclusive. For a file they bound its lines, bytes and longest line. For a directory
they bound its tracked files, its largest file's lines and bytes, its longest line and its total bytes. A doctrine's
name defers that dimension to that doctrine. `debt: <leaf>` records a terminal nothing bounds yet, owned by an open
leaf. "Overflows to" is where that destination's own guard sends what does not fit.

| Destination | Route | Owner | Lifecycle | Files | Lines | Bytes | Longest line | Total bytes | Overflows to |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `AGENTS.md` | navigation | `PROGRAM` | maintained_reference | — | 24 | 2048 | 200 | — | — |
| `CHANGELOG.md` | navigation + overflow | `PROGRAM.31` | rolling_ledger | — | HISTORY-LEDGERS | HISTORY-LEDGERS | 256 | — | `docs/history/` |
| `CLAUDE.md` | navigation | `PROGRAM` | maintained_reference | — | 90 | 6144 | 200 | — | — |
| `COMMIT.md` | navigation | `PROGRAM` | maintained_reference | — | 160 | 9216 | 200 | — | — |
| `DEV_NOTES.md` | navigation | `PROGRAM.31` | rolling_ledger | — | HISTORY-LEDGERS | HISTORY-LEDGERS | 200 | — | `docs/history/` |
| `DOCTRINE_ENFORCEMENT.md` | navigation | `PROGRAM` | maintained_reference | — | 140 | 36864 | 3072 | — | — |
| `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` | navigation + overflow | `PROGRAM.17` | maintained_reference | — | 480 | 36864 | 512 | — | — |
| `LIVE_STATUS.md` | navigation | `PROGRAM.17.2` | bounded_snapshot | — | LIVE-SNAPSHOTS | LIVE-SNAPSHOTS | LIVE-SNAPSHOTS | — | `CHANGELOG.md`, `docs/tasks/` |
| `MEMORY.md` | navigation | `PROGRAM.17.2` | bounded_snapshot | — | MEMORY-ARCH | MEMORY-ARCH | LIVE-SNAPSHOTS | — | `docs/tasks/`, `docs/decisions/` |
| `MEMORY_ARCHITECTURE.md` | navigation | `PROGRAM` | maintained_reference | — | 540 | 32768 | 512 | — | — |
| `README_POLICY.md` | navigation + overflow | `PROGRAM.35` | maintained_reference | — | 260 | 20480 | 512 | — | — |
| `ROADMAP.md` | navigation + overflow | the director | maintained_reference | — | 1100 | 131072 | 1024 | — | — |
| `TOOLBOX.md` | navigation + overflow | `PROGRAM` | maintained_reference | — | 110 | 28672 | 2048 | — | — |
| `VISIBILITY.md` | navigation | `PROGRAM` | maintained_reference | — | 90 | 5120 | 200 | — | — |
| `docs/TASK_TREE.md` | navigation + overflow | `PROGRAM.17.2` | bounded_snapshot | — | LIVE-SNAPSHOTS | LIVE-SNAPSHOTS | LIVE-SNAPSHOTS | — | `CHANGELOG.md`, `docs/tasks/` |
| `docs/book/` | navigation + overflow | `PROGRAM` | maintained_reference | 32 | 750 | 49152 | 1024 | 294912 | — |
| `docs/history/` | overflow | `PROGRAM.31` | archive_terminal | HISTORY-LEDGERS | 1200 | 131072 | 256 | HISTORY-LEDGERS | — |
| `docs/reviews/` | overflow | `PROGRAM.36` | partitioned_canonical | 16 | 1200 | 131072 | 1024 | 262144 | — |
| `docs/decisions/` | navigation + overflow | `PROGRAM` | partitioned_canonical | 32 | 1200 | 98304 | 1536 | 327680 | `docs/reviews/` |
| `docs/tasks/` | overflow | `PROGRAM.32` | partitioned_canonical | 20 | debt: `PROGRAM.32` | debt: `PROGRAM.32` | debt: `PROGRAM.32` | debt: `PROGRAM.32` | — |
<!-- README-POLICY-LOCAL-ADOPTION:END -->

---

# README Stability Policy

This project- and harness-neutral policy keeps a repository README useful as a
stable landing page instead of letting it grow into a changelog, roadmap, or
documentation catalog.

## Authority and provenance

After adoption, the project owns this policy. Cite its authority by project
owner and adoption or revision date, together with the project-owned
`<repository-root>/README_POLICY.md`. Never cite a vendor-, agent-, or
harness-specific bootstrap file as the authority. Bootstrap files may help
authors and tools discover the policy; they do not make the policy binding.

## Storage location

Store the adopting project's canonical copy as the git-tracked
`<repository-root>/README_POLICY.md`, alongside `README.md`. Keeping the policy
with the file it governs gives contributors, local hooks, and CI one
discoverable, versioned source of truth. A user-home, machine-global, or other
external copy may serve as a reusable template, but it must not replace the
project-owned repository copy. Once copied, the project-owned file is
authoritative and the origin is not an upstream. Do not automatically re-sync
from the origin; adopt later revisions only through deliberate local review.

Keep project-specific adoption metadata—owner, date, decisions, derived caps,
and local enforcement links—in a clearly fenced adoption note above the
neutral policy body. This keeps the reusable body free of project-specific and
harness-vendor-specific tokens without hiding local authority.

## Content contract

Keep only information a first-time visitor needs:

- purpose, audience, and top-level scope;
- prerequisites and one minimal verified quick start;
- stable architecture at a glance;
- links to canonical documentation, support, and contribution guidance;
- license and other essential repository-level notices.

Route changing detail elsewhere:

| Content | Canonical home |
| --- | --- |
| User-facing feature detail and examples | User guide or product manual |
| Current work, priorities, and roadmap status | Roadmap, issue tracker, or task system |
| Release history | Releases, changelog, or git history |
| Design rationale | Decision records or architecture docs |
| Exhaustive file/API/sample inventories | Generated indexes or dedicated references |
| Diagnostics and operational procedures | Troubleshooting or contributor docs |

Change the README only when its purpose, first-use path, top-level architecture,
or canonical navigation changes. Ordinary feature work should update the
canonical destination, not the README.

Before deleting or relocating apparent duplication, prove that it is genuinely
duplicated with a phrase, identity, or content probe against the intended
canonical home. If that home is already richer and maintained, delete the
README copy and retain one link. Relocate only information that is unique and
still belongs in maintained documentation.

## Routing pressure closure

Moving content out of the README is not sufficient if the destination can
become an unbounded neighboring sink. Inventory every destination named by the
README, this policy, or the guard's failure guidance. Give each route an owner,
lifecycle class, and pressure control, and follow routes transitively until
they end at a controlled terminal. An unclassified destination, routing cycle,
or chain that merely moves the same append pressure again is a failed adoption.

Classify routes as `reader_navigation` or `author_overflow`. The sets may
legitimately differ: readers may inspect immutable change history, while
authors must not be told to append new status prose there. Derive overflow
candidates from path-shaped destinations in the guard's actual emitted
guidance, not only from a hand-maintained table, and fail when an emitted hint
has no governed destination.

Use controls appropriate to the destination:

| Destination class | Required pressure control |
| --- | --- |
| Hot/live file | Derived line and byte ceilings plus overwrite, review, or staleness semantics |
| Partitioned manual or task collection | Bounded index plus per-part, file-count, and aggregate ceilings |
| Generated index | Size ceilings plus a reproducible freshness check against canonical sources |
| Append-only history | Query-first access plus a shard, rotation, or archival threshold; never a mandatory bootstrap read |
| External service | Named authority, retention/lifecycle owner, and a stable query/link contract |
| Frozen legacy record | Content identity or another write prohibition; never an overflow destination |

A legacy destination that is already too large is not exempt. Record its
current measured ceiling as debt, stop further growth there, and open a
separately owned partition/compaction task. Do not describe a measured legacy
ceiling as an ideal reusable default. Raising any destination threshold needs
the same explicit review as raising the README cap.

In one measured adoption, README status/history guidance routed overflow into
an otherwise unchecked neighboring status file. That file reached 1,547,057
bytes, and 94.7% of it was dated changelog content. The README cap had displaced
the pressure rather than removing it. A destination registry and unconditional
closure check make that failure visible before it becomes another megabyte-
scale bootstrap surface.

## Mechanical growth guard

Enforce both a line cap and a byte cap. Derive both from the landing page that
survives a deliberate review and trim, leaving only modest explicit headroom.
Do not copy example values from this policy. Never raise a cap merely to land
new content; move the detail to its canonical home. A cap increase requires an
explicit reviewed decision that the landing-page contract itself expanded.

A minimal deterministic check is:

```sh
line_cap=__DERIVED_LINE_CAP__
byte_cap=__DERIVED_BYTE_CAP__
lines=$(wc -l < README.md | tr -d ' ')
bytes=$(wc -c < README.md | tr -d ' ')
test "$lines" -le "$line_cap"
test "$bytes" -le "$byte_cap"
```

Replace both placeholders with the adopting project's reviewed values before
enabling the check. Keep it non-mutating, return nonzero with a routing hint on
failure, and run it unconditionally on every commit and CI build. Landing-page
size is a property of the resulting tree, so the guard must not short-circuit
merely because `README.md` is absent from a staged or changed-path set; this
also catches over-budget merge and revert results.

The same unconditional check must validate the routed-destination inventory
and each declared pressure control. A commit that does not touch the README can
still overgrow, unfreeze, remove, or silently retarget one of its destinations.

Line and byte checks are independent. In one real adoption, the retained README
was 141 lines yet already 10,297 bytes; a numbered prose list measured roughly
118 bytes per line while a path list measured roughly 57. A line budget alone
therefore cannot constrain prose density, and a byte budget alone cannot
constrain vertical sprawl.

## Adoption checklist

1. Add and commit `<repository-root>/README_POLICY.md` beside `README.md`.
2. Fence local owner/date, authoritative-copy, independence, decision, and cap
   metadata above the neutral policy body.
3. Prove apparent status, history, inventory, and deep-reference duplication
   against its canonical home; delete-with-link when that home is richer, and
   relocate only genuinely unique maintained content.
4. Verify the retained quick start and links.
5. Record where each excluded content class belongs, then inventory every
   actual route through a controlled terminal; reject cycles and unclassified
   neighboring sinks.
6. Give hot/live files line and byte caps; give partitioned, generated,
   historical, external, and frozen terminals the class-specific controls
   above. Treat measured legacy ceilings as debt, not examples.
7. Derive reviewed line and byte caps from the trimmed survivor with modest
   explicit headroom; do not copy illustrative values.
8. Commit the deterministic README and routing-closure check and wire it
   unconditionally into every local commit and CI build, independent of
   changed-path scope.
9. Require an explicit decision before the README cap or any routed-destination
   threshold can increase.
