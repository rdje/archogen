# The decisions folder's ceiling is raised once, as the director's exception, until it is partitioned

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.38` (`docs/tasks/PROGRAM.md`), on the director's ruling of `2026-09-30`.
  - The question put: "`docs/decisions/` is at 324 162 of its 327 680-byte ceiling (28 of 32 files). The next
    catalog-review answers need ~5 KB more. The policy's structural fix (a sub-folder) would hide those records from
    two held template checks … How should the decisions folder grow?"
  - The options: "Raise, reviewed", "Partition now", and "Pause and hold the ceiling".
  - The answer: "Raise, reviewed".

  The partition this raise defers is `PROGRAM.39`. The review is the last section.

## The fact / decision

`docs/decisions/`'s row in `README_POLICY.md`'s routed destinations changes in three cells:

- **tracked files:** from 32 to **40**;
- **bytes in total:** from 327 680 to **393 216** (384 KiB);
- **owner:** `PROGRAM.39`, the leaf that partitions the folder.

Every other cell stays: 1 200 lines, 98 304 bytes and a 1 536-byte longest line for any one file, and the overflow
route to `docs/reviews/`. The inventory row in `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` changes with it.

**This is the only raise, and only a new explicit ruling by the director can reopen it.** A new record alone
cannot. The next step is the partition (`PROGRAM.39`), and the partition does not add capacity: the folder and every
sub-folder together stay within 40 files and 393 216 bytes. Any net increase is itself a raise, needing a new
reviewed ruling.

## Why

**Measured `2026-09-30`**, at `HEAD` `7a628a0`, with `git ls-files -z docs/decisions | xargs -0 cat | wc -c`: 28
files and 324 165 bytes. This record, with its index row, takes the folder past today's ceiling on its own. So the
raise, the record, its index row and the inventory row land in one commit.

**The policy's test, and how this meets it.** `README_POLICY.md` asks for "an explicit reviewed decision that the …
contract itself expanded". It also says "never raise a cap merely to land new content". Both are answered plainly:

- **The folder's role did grow.**
  - The 327 680 ceiling was set at `38d8634` (12:50 that day), from a folder of 23 files and 250 243 bytes. Its one
    normative design under open independent review, the catalog record, was then a single file of 72 269 bytes.
  - It now holds two such designs, the catalog record's three files and the composition of the runtime inputs, at
    136 093 bytes. That is 42% of the folder, after both designs' review histories moved to `docs/reviews/`.
  - A multi-record design under review is a kind of content the ceiling was not derived for.
- **It is also the director's time-boxed exception.** The immediate need is about 5 KB for `M2.7.1`'s next answers,
  plus this record. By itself that is landing new content, which the policy's clause forbids. The director ruled
  the exception, and it ends at the partition.

**Why not partition today.** It can be done without losing enforcement. The price is a project-owned check, and
`PROGRAM.39` states that price:
- the template's index check (`MEMORY-ARCH`) and knowledge-map generator read the folder flat, and are held
  (findings §10);
- `README-ROUTES`, archogen's own, counts a sub-folder into its parent;
- so a partition needs a project gate that extends index completeness to sub-folders, a curated link to each
  sub-folder's index in `knowledge-map/subsystems.md`, and a change to `README-ROUTES`.

That is a structural leaf of its own. Doing it now would stall `M2.7.1` and `M2.10` while it is built. This is the
reason, cost and timing. It is not that a partition is impossible.

**Alternatives weighed:**
- **Compaction without a raise.** The findings register holds four settled items, 9 749 bytes: §2 resolved, §4
  informational, §8 and §10 ruled. Trimming index rows to 250 bytes would free about 2 900 more. Together that is
  about 12.6 KB, which covers this record and the next answers, and little after. The register's items are cited by
  section number across the task trees, so moving them needs stubs. It is taken up in `PROGRAM.39`, and not relied
  on here.
- **Pause and hold the ceiling.** The policy's own route is debt with growth stopped, the one `PROGRAM.36` and
  `PROGRAM.37` used. Here it would stop `M2.7.1` and `M2.10` until `PROGRAM.39` is done. That cost is the reason it
  was not chosen.

**The headroom.** 393 216 is 69 051 bytes above the measurement. Measured with this record, its index row and
the rows it changes staged, the folder is 29 files and 332 029 bytes: 61 187 bytes of room, about 15% of the new
total, and 11 files. For the horizon: the previous 77 437 bytes of room lasted
from 12:50 to 15:16, two and a half hours, before two moves paid it down. At a pace like that, this room is hours to
days, not a lasting allowance. That is why the partition is opened now, with a warning well short of the ceiling.

**What the folder holds**, so "its canonical home" is shown rather than asserted:
- at `HEAD`, 26 records (23 decisions, two references and one project register), with a template and an
  index. This record is the 27th;
- the catalog record and the composition record are decisions. `M2.7.1`'s acceptance pins the catalog design as a
  decision record, because it decides what `ROADMAP.md` §9 requires before any code;
- the normative formats that are specifications rather than decisions live elsewhere, as
  `docs/analysis/cost-accounting-v1.md` and `docs/profiles/` do.

## How to apply

- **The warning, before the ceiling.** When the folder reaches 36 files or 360 000 bytes, the partition starts
  before anything else adds to the folder.
- **The partition, done by `PROGRAM.39` on `2026-09-30`**, before the warning was crossed:
  - `docs/decisions/catalog/` holds the catalog design's four records: `decision_catalog-records.md`, `-example.md`,
    `-variant-inputs.md` and `decision_runtime-composite-inputs.md`;
  - the sub-folder has a row of its own, reached through the parent's "Overflows to" cell;
  - `README-ROUTES` holds a partition's files to its own row's per-file ceilings. The parent's file count and total
    still count everything beneath it, which is the conservation rule;
  - `DECISION-INDEX` finds every record by its path from the index, partitions included, where the template's
    `MEMORY-ARCH` reads the folder flat;
  - `knowledge-map/subsystems.md` links the partition, which the generated map, reading the folder flat, does not
    reach.
- **A record** still goes in its canonical home, a sub-folder when its subject has one, and a review history still
  goes to `docs/reviews/`.
- **"Only once" is enforced.** `README_POLICY.md`'s table `### Ceilings a decision fixes` holds this record's 40
  files and 393 216 bytes, and `README-ROUTES` refuses a row whose cell is above them.
- **The findings register's settled items** were considered for sealing, and left in place. Their section numbers
  are cited across the task trees, so each would need a stub, and the partition, not a seal, was what the warning
  needed. They are the next compaction when the folder nears its ceiling again.
- Related: `README_POLICY.md` (the row), `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` (the inventory row),
  [[decision_findings-for-director-review]] §10 (the hold), [[decision_history-ledgers]].

## Review

An independent context reviewed the first draft before it landed, and found that the raise could not land as
written. The findings, and the answer to each, are in
[`decision_decisions-folder-ceiling-reviews.md`](../reviews/decision_decisions-folder-ceiling-reviews.md).

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 18 | 4 (the policy's test not applied; a false claim of no append-only content; a trigger its rationale contradicted; a partition that was a second raise) | "cannot land as written"; the ceilings 40 and 393 216 "modest and defensible" |
