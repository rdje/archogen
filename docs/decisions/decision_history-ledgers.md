# The changelog and the development notes are rolling ledgers, sealed by entry count

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.31` (`docs/tasks/PROGRAM.md`), carrying out the ruling on §8 of
  [[decision_findings-for-director-review]]: the director delegated the decision, and asked for it at the state of
  the art and at sign-off quality. It applies `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s `rolling_ledger` class and its
  atomic transition protocol.

## The fact / decision

`CHANGELOG.md` and `DEV_NOTES.md` are rolling ledgers. Each keeps its newest entries live, and its older entries
move, unchanged, into numbered segments that are never edited again.

- **An entry** is a heading and everything up to the next heading, outside code fences: `## ` for the changelog,
  `## _(` for the development notes. Both files are newest first.
- **The boundary is a count, not a date.** When a live ledger holds twice its window of entries, its oldest window
  of entries is sealed into the next segment. This repeats until fewer than twice the window remain. The window is
  20 entries for the changelog and 10 for the development notes, so the live changelog holds 20 to 39 entries and
  the development notes 10 to 19.
- **A segment** is `docs/history/<ledger>/<NNNN>.md`: the exact bytes of the entries it holds, newest first, with
  numbers rising from `0001` as the sealed content gets newer.
- **`docs/history/INDEX.md`** has one table per ledger and one row per segment: its number, its entry count, its
  newest and oldest entries (a work-unit id, or a date for the development notes), its lines, bytes and sha256, and
  the day it was sealed. Rows are only ever appended.
- **Reading a ledger whole** is reading the live file, then its segments from the highest number down. That
  concatenation is the ledger as it would stand unsealed. It is a file read, so it depends on no tool,
  agent or harness.
- **Sealing is done by a tool**, `bash scripts/check_history_ledgers.sh --seal`. It writes the segments and their rows,
  proves that the live entries followed by the new segments reconstruct the region they came from byte for byte,
  replaces nothing if they do not, and then runs the gate.
- **The gate**, `HISTORY-LEDGERS`, runs on every commit and checks, per ledger:
  1. every segment's lines, bytes, entry count and sha256 against its row;
  2. that segments and rows correspond one to one, numbered without a gap;
  3. across history, that every row any committed version of the index held is unchanged, and that every segment
     is what the commit that added it wrote, so CI catches a forgery too (`PROGRAM.40`). History is read with
     `--full-history`, so a merge that keeps only a side that never sealed cannot hide a seal, and a shallow
     repository is refused (`PROGRAM.42`);
  4. that the live file holds fewer than twice its window, or else that a rollover is required;
  5. that the live header names the index, once a segment exists;
  6. that the order continues across the live file and the segments: work-unit numbers strictly fall in the
     changelog, and dates never rise in the development notes.
- **A rollover is part of the commit that makes it due.** When the gate reports one, the same commit runs the seal,
  as `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` requires: appends stop unless the change performs the rollover.

## Why

- **The pressure was measured.** `CHANGELOG.md` had reached 327 233 bytes and `DEV_NOTES.md` 135 158 bytes in
  seventeen days, at up to 75 commits a day. Neither is a mandatory read, but both are what a reader opens.
- **A count, not a month.** The proposal before the ruling was a monthly boundary. Measured on the day, the whole
  history was one month, so a monthly window would have sealed nothing and bounded nothing. The doctrine allows a
  record boundary as well as a time one. A count bounds the live file whatever the pace. The index still gives each
  segment's range, so "what happened when" stays one lookup away.
- **Byte-exact segments, not rewritten summaries.** A history that is edited on the way into an archive is no
  longer evidence. A segment holds the bytes it had in the live file, and the reconstruction proof shows that
  nothing was dropped or reordered.
- **Digests in an append-only index.** A sealed file that someone edits, even with its row, is caught against
  `HEAD`. A file added without a row, or a row without its file, is caught too.
- **A tool does the seal.** A seal done by hand is where an entry gets lost. The tool proves its own result, and
  refuses to replace the live file otherwise.

## How to apply

- **Writing an entry:** at the top, as before. When the gate says a rollover is required, run
  `bash scripts/check_history_ledgers.sh --seal` and commit its result with the entry.
- **Correcting a sealed entry:** never in place. Write a new entry that supersedes it, in the live file.
- **Reading history:** `docs/history/INDEX.md`, then the segment.
- **Only archogen's own entries.** Both ledgers had been created holding the project template's own entries: its
  release notes at the end of the changelog, and two notes and a header paragraph in the middle of the development
  notes, where the gate's first run found them dated before the notes below them. They were a defect of the
  template, since fixed there, and archogen keeps no reference to it. The first transition, on `2026-09-30`,
  removed them, and proved that the live files and their segments hold every other entry byte for byte.
- Related: [[decision_findings-for-director-review]] §8, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`,
  [[decision_catalog-records]], whose lock is append-only in the same way.
