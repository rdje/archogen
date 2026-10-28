# The record folders' ceilings bound what is read, not how much is kept

- **Type:** `decision`
- **Date:** `2026-10-27`
- **Status:** `active` — its review closed in round 5, below
- **Owner / source:** leaf `PROGRAM.117.1` (`docs/tasks/PROGRAM.md`), on the director's ruling of `2026-10-27`, which
  reopens the one-time raise of [[decision_decisions-folder-ceiling]], as only a new explicit ruling could:
  - "I do not understand why there are even ceilings on those directories ... What's the best solution, to you, to
    handle nicely, elegantly those ceiling. I agree their size shall not blow up, shall somehow be contained but
    this shall not hinder progress. Propose a sota, production-grade solution. In any case, please take sound,
    sota, signoff and production-grade solution w.r.t your findings".
  - It supersedes [[decision_decisions-folder-ceiling]], [[decision_specs-folder-ceiling]] and
    [[decision_reviews-folder-ceiling]].

## The fact / decision

In plain words: a record folder may hold as many records as the project needs. What stays bounded is each thing
someone reads, one record or one folder's own records listed. A mistake is refused by the commit hook in the commit
that makes it, and a folder that grows honestly toward its outer guard asks the director for a ruling years ahead.

For `docs/decisions/`, `docs/specs/` and `docs/reviews/`, the rows of `README_POLICY.md`'s routed destinations
read:

- **Per file, unchanged.** Lines, bytes and the longest line of any one file keep their ceilings, so any record
  fits in one reading.
- **Files: `fan-out 64`.** No directory the row governs lists more than 64 entries: its tracked files, archive
  stubs aside, and its sub-folders. A subject's sub-folder takes the growth, and its own listing is bounded the
  same way. A registered partition is one entry of its parent and is bounded by its own row. A maximum a decision
  fixes for a folder also binds every row registered beneath it in that column, through the nearest ancestor the
  table fixes there, and in that column's kind, never a doctrine or a debt. A file row is bound in the per-file
  columns, the only ones it has.
- **Total bytes: `guard 16777216`** (16 MiB). This is a runaway guard, not a budget. `README-ROUTES` refuses three
  things:
  - **a total over the guard.** It is reported as a mistake to find.
  - **a total at three quarters of the guard,** unless an open leaf asks the director for a ruling. The cell then
    reads `guard 16777216, ruling asked by <leaf>`. When the ruling comes, the cell takes the guard it fixes, or, if
    it keeps this one, `guard 16777216, ruled by <record>`, naming the tracked decision record of that ruling under
    `docs/decisions/` or `docs/specs/`, by its literal path, whose type is `decision`; an index or the template is
    no ruling. The check cannot tell that the record rules on this guard; review does. This makes the
    warning a refusal that every commit sees, not a notice the gate would print nowhere.
  - **a commit that grows the folder over `HEAD` by more than its step,** a sixty-fourth of the guard (256 KiB),
    measured in the index, which is what the commit holds. The exception is a commit for which
    `README_ROUTES_GROWTH_LEAF` names the leaf that owns that growth: a leaf open at `HEAD`, or one the commit itself
    files in the index, so a variable left set from an earlier move is refused. This catches a generated or copied tree when it
    is committed. Honest growth comes in records of at most 98 304 bytes each (131 072 for a review history).
  - **What the step cannot see.** The pre-commit hook judges the step, and CI, checking a pushed tip against
    itself, does not. A commit made with `--no-verify` is not seen. An amended commit is measured against the
    commit it replaces. A tree split across several commits passes.
- **Telemetry.** `bash scripts/check_readme_routes.sh --report` prints, for each directory row:
  - its files, its widest listing and its total;
  - its growth since the newest commit at least a fortnight old;
  - when it is refused at that pace: at three quarters of a guard with no ruling clause, at its bound otherwise.

Raising a fan-out or a guard still needs a new ruling. `README_POLICY.md`'s `### Ceilings a decision fixes` holds
both at these values, naming this record. `README-ROUTES` refuses a cell that is above its maximum, or of another
kind than its maximum: another kind of number, a debt or a doctrine. A tighter bound of another kind is refused
too, so a cell a decision fixes changes kind only by a new ruling.

**The lifecycle that keeps the working set small.** `docs/decisions/` is the working set: records under review, or
still being amended. An accepted design moves to `docs/specs/<subject>/`, as [[decision_specifications-home]]
rules. The move is one scripted, verified step, `bash scripts/move_record.sh [--dry-run] <record> <destination>`
(`PROGRAM.117.2`). It rewrites the record's own links and every live citation, and lists what it leaves: a
revision's path, the archives, the rolling ledgers, the verbatim proofs and the files held for the scaffold.

A superseded record is marked `superseded by` its successor and stays where its citations find it. Sealed
history cites records by path and cannot be repointed. This is the practice of the architecture-decision-record
method ([Nygard](../book/src/ledger.md#nygard-adr)): "If a decision is reversed, we will keep the old one around,
but mark it as superseded."

**What does not change.**
- The snapshots read at every resume — `MEMORY.md`, `LIVE_STATUS.md`, `docs/TASK_TREE.md`, `README.md` — keep their
  hard ceilings.
- `docs/tasks/` keeps its file count and total: its trees are read to resume.
- The archives keep their own doctrines.
- The book stays unbounded ([[decision_book-unbounded]]).

**Where the bound is still one file, and what is owed.**
- `docs/decisions/INDEX.md` lists every record of `docs/decisions/` and of `docs/specs/`, their sub-folders
  included. It is one file under the folder's per-file ceilings. At `220eb078` it held 18185 of 98304 bytes, 16118 at
  `d00c3dee` fifteen days before: about 140 bytes a day, so it fills in about a year and a half.
- Discharged by `PROGRAM.117.3` (`2026-10-28`), so that the index never binds the folders it lists:
  - `DECISION-INDEX` reads a sub-folder's own `INDEX.md` once an index it already reached links it, and refuses an
    index nothing reaches; a partition's rows can move there before the root fills;
  - it holds `docs/specs/` to the same rule. The three specifications that were in no decision index, two of
    `docs/specs/trust/` in none at all, are indexed.
- `docs/reviews/` is flat. `REVIEW-HISTORY` refuses a stub in a sub-folder and archives a history only from the
  folder itself, and an archive stub stays where sealed history cites it. So that folder's own listing grows by a
  stub for each closed review. The fan-out
  counts its open histories and its index alone, 6 of 64 today.

## Why

**A total bounds no read.** `MEMORY_ARCHITECTURE.md`'s resume reads the snapshots and a tree's frontier. No reader's
context loads a whole record folder. Gates scan every record, but a gate reads files and keeps none of them. A reader
opens one record by its path, found from an index or a search. The per-file ceilings bound that read. The listing
bound and the index bound the way a reader gets to it.

**The totals were a wall that stopped work.**
- `docs/specs/`' total was raised four times in eighteen days: by `PROGRAM.56` (`2026-10-05`), `PROGRAM.79`
  (`2026-10-11`), `PROGRAM.102` (`2026-10-22`) and `PROGRAM.108` (`2026-10-23`). Each raise cost a dated paragraph,
  a measurement and a review of the paragraph.
- At `220eb078`, `git ls-tree -r -l HEAD -- docs/specs`: 649155 of 655360 bytes. The next accepted design would
  have needed a fifth raise.
- `docs/decisions/` held 392564 of the director's 393216. That is 652 bytes, less than this record.
- Each new design adds records, so a total turns the product's growth into accounting work. The director ruled the
  same of the book on `2026-10-18`: a ceiling "creates unnecessary compaction work"
  ([[decision_book-unbounded]]).

**The neutral policy is still met.** `README_POLICY.md`'s body asks a partitioned collection for a "Bounded index
plus per-part, file-count, and aggregate ceilings". Each is still here:
- per-part: the per-file ceilings;
- file count: the fan-out, now per directory;
- aggregate: the guard.

The index is bounded as one file, and `PROGRAM.117.3` keeps it bounded when it splits. The body's "Raising any
destination threshold needs the same explicit review as raising the README cap" is answered by the director's
ruling, by this record, and by its review.

**The numbers.**
- **64 entries** is a judgement, not a measurement of a read.
  - With this record, `docs/decisions/` lists 40 entries, so the working set has room for about two dozen records
    before a subject takes a sub-folder of its own.
  - The widest listing elsewhere is `docs/specs/catalog/`, at 7 entries.
  - A fan-out alone would let a deep tree of sub-folders grow without end; the guard closes that.
- **16 MiB** is about 26 times `docs/specs/`, the largest of the three folders.
  - `--report`'s window to `2026-10-27`, fifteen days from `d00c3dee`, measured `docs/specs/` growing 14554 bytes a
    day. That figure includes the moves of accepted designs into it, and it is the fastest of the three folders.
  - At that pace, `docs/specs/` reaches three quarters of its guard in about two years and the guard in about
    three.
  - So honest growth does reach the guard, but only after the three-quarters refusal has asked the director for a
    ruling.
- **The step, 256 KiB,** is a sixty-fourth of the guard.
  - The largest growth of any of the three folders in one commit since `6d61f65` is 38818 bytes: `efedf6d6`,
    `docs/specs/` taking the runtime analysis design.
  - The largest batch the records document is older: `PROGRAM.43` moved the catalog's six records out of
    `docs/decisions/` on `2026-10-01`, 388 884 bytes before and 202 267 after ([[decision_specifications-home]]).
    That is 186 617 bytes, under the step; a larger batch names its leaf.
  - A commit adding a tree of records past the step is refused when it is made. Without the step, a mistake could
    land unseen: 64 files of 98304 bytes in a new sub-folder pass the per-file ceilings, the fan-out and, at 6.3 MB,
    the guard.

**Alternatives weighed.**
- **Raising totals by measured steps,** as before: this is the friction measured above.
- **No aggregate bound at all:** a generator writing a tree of small files would land unnoticed.
- **A bound on the number of records:** the same wall as a total, counted in files.
- **A per-directory bound alone,** with no total: this is the deep tree the guard closes.
- **A notice at three quarters, with no refusal:** `scripts/check_doctrines.sh` prints nothing from a gate that
  passes. It is held for the scaffold, so no notice would reach anyone.

## How to apply

- **A folder listing 64 entries.** Give a subject with several records a sub-folder of its own, or move accepted
  designs on to `docs/specs/<subject>/`. `DECISION-INDEX` finds a record in a sub-folder by the path its index
  gives. Link the sub-folder from `knowledge-map/subsystems.md`: the generated map reads `docs/decisions/*.md` flat.
  Register the sub-folder as a row of its own only when it needs other per-file ceilings. Move each record with
  `bash scripts/move_record.sh`, a dry run first; a batch past a folder's step names its leaf in
  `README_ROUTES_GROWTH_LEAF`.
- **A row beneath a column a decision fixes** writes a number of that column's kind there, even a file another
  doctrine bounds: a deferral would leave the decision's bound.
- **Three quarters of a guard.**
  - File a leaf that asks the director for a ruling, and name it in the cell.
  - Read `--report` for the pace.
  - When the ruling comes, write the guard it fixes, in the policy's row and in its maximum; a ruling that keeps
    the guard is written `guard N, ruled by <record>`.
- **Reaching a guard, or a commit refused past its step.** Treat either as a defect, with a root cause, under a
  leaf. A commit that honestly grows a folder past its step is split into smaller commits. A move of several
  accepted designs at once names its leaf in `README_ROUTES_GROWTH_LEAF`.
- **A superseded record.** Its status reads `superseded by <record>`, its index row says so, and it stays in place.
- **The three records this supersedes** keep their text. Their rules about totals, the one-time raise and "a
  partition adds no capacity" no longer bind these folders; the last still holds where a row keeps a plain total,
  as `docs/tasks/` does. Their measurements remain the history of how the folders grew.
- Related: `README_POLICY.md` (the rows and the maxima); `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` (the inventory);
  `scripts/check_readme_routes.sh` (the check, its `--report` and its self-test); [[decision_book-unbounded]];
  [[decision_specifications-home]].

## Review

Independent contexts reviewed the record and the check before they landed. Their findings and the answers are in
leaf `PROGRAM.117.1`.

| Round | Findings | Defects |
| --- | --- | --- |
| 1 | 15 | 5: numbers past 64 bits and leading zeros failing a comparison open; a decision-fixed cell turned into a debt or a doctrine; a notice that reached no one; an index split `DECISION-INDEX` refuses; "only a mistake reaches the guard", false |
| 2 | 15 | 2: the step measured the disk, not the commit; a partition's row lifting a fan-out a decision fixes for its parent |
| 3 | 7 | 3: a ruling clause accepting any pathspec; a leaf "new in the commit" read from the disk; a parent's fixed maximum reaching file rows that cannot comply |
| 4 | 8 | 2: file rows freed in the per-file columns too, lifting a decided per-file maximum; the header and a message still stating round 2's rule |
| 5 | 3 | 0 — the review closed; its three remarks taken |
