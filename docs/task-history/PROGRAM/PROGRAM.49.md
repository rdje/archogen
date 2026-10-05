- ID: `PROGRAM.49`
  Status: `done` — `2026-10-03`
  Goal: `docs/reviews/`' total-bytes ceiling raised with its measurement, as `README_POLICY.md` requires of a ceiling:
  the folder stood at 264 219 bytes against 262 144 after the substitutability record's fourth round was answered;
  `PROGRAM.36` set the ceiling from 6 histories at 117 623 bytes, there are 11 now, and the closure rule makes a
  history as long as the rounds it takes — 16 for the catalog's record. A frozen history is never edited, so the
  folder cannot be compacted, and it has no overflow destination of its own.
  Acceptance: the ceiling raised in `README_POLICY.md`'s row to 393 216 bytes (384 KiB) and named in "Ceilings a
  decision fixes", the per-file ceilings unchanged; a decision record, `decision_reviews-folder-ceiling.md`, carrying
  the measurement, the rate and the next step if the rate holds (a terminal archive for closed histories, not another
  raise); the containment inventory's row re-measured; `README-ROUTES` green.
  Verification: `git ls-files docs/reviews | xargs cat | wc -c` → `264219` before, against the refusal
  `docs/reviews/: 264219 bytes in total, over its ceiling of 262144`; `bash scripts/check_doctrines.sh` → `=== all
  doctrines green ===` after.
  Commit: `ARCHOGEN-PROGRAM-0400 (leaf PROGRAM.49)`
