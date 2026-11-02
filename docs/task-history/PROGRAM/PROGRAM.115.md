- ID: `PROGRAM.115`
  Status: `done` — filed `2026-10-26` by `PROGRAM.114`
  Goal: `DOCTRINE_ENFORCEMENT.md` compacted under its ceiling with room for `PIPE-READS`'s row, or its ceiling raised
    by a decision record.
  Reproduce / issue: `bash scripts/check_readme_routes.sh` with the row added: `DOCTRINE_ENFORCEMENT.md: 37469 bytes,
    over its ceiling of 36864`; without it the table holds 36838, twenty-six bytes free. The table is scaffold-owned,
    so a compaction is the scaffold's change too, written down for its owner.
  Corrected by `PROGRAM.116` (`2026-10-26`): the table is archogen's — on the updater's NEUTRAL list, but changed here,
    and the director's ruling of `2026-09-30` (findings §10) holds only what archogen never changed — so a compaction is
    this repository's alone, nothing owed upstream; `PROGRAM.116`'s row edit left it at 36836 bytes.
  Done `2026-10-28`. `M2.7.6.5` (`ARCHOGEN-M2-1033`) took five clauses of history out of other rows to fit
    `CODE-OWNERS`'s; this leaf shortens `BOOK-ANCHORS`' row to its rule, the rationale it repeated kept in
    `scripts/check_book_anchors.sh`'s header, and adds `PIPE-READS`'s row after `NO-SUBPROCESS`'s, the registry's place.
  Verification: `bash scripts/check_readme_routes.sh` → `exit 0`, nothing on `DOCTRINE_ENFORCEMENT.md`, at 36851 of
    its 36864 bytes; `bash scripts/check_table_arity.sh` → `exit 0`. Thirteen bytes stay free, so the next row comes
    with its own compaction or a ceiling the director rules.
  Commit: `ARCHOGEN-PROGRAM-1036 (leaf PROGRAM.115)`
