- ID: `PROGRAM.48`
  Status: `done` — `2026-10-03`
  Goal: the book's total-bytes ceiling raised with its measurement, as `README_POLICY.md` requires of a ceiling:
  `docs/book/` stood at 393 208 of 393 216 bytes, eight bytes of headroom, after 44 commits touched the book in two
  days; the lockstep rule moves a chapter with every normative change, so the book grows by about a kilobyte a leaf
  by design, and four of the last six book changes needed a compaction of a stale or redundant passage first.
  Acceptance: the ceiling raised in `README_POLICY.md`'s two rows to 458 752 bytes (448 KiB), the per-file ceilings
  unchanged; the decision record that owns the ceiling, `decision_book-in-layers.md`, carries the dated paragraph
  with the measurement and the rate; `README-ROUTES` green.
  Verification: `git ls-files docs/book | xargs wc -c` → `393208 total` before; `bash scripts/check_doctrines.sh` →
  `=== all doctrines green ===` after.
  Commit: `ARCHOGEN-PROGRAM-0392 (leaf PROGRAM.48)`
