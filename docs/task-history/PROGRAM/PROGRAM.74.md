- ID: `PROGRAM.74`
  Status: `done` — filed, started and closed `2026-10-10`
  Goal: the book has room to grow with the code it describes.
  Reproduce / issue: `PROGRAM.70.2`'s first commit attempt, the doctrine gate → *"README-ROUTES: docs/book/: 458863 bytes
  in total, over its ceiling of 458752"*, a two-sentence addition to the annex; replacing a sentence instead, and the
  index regenerated, left `git ls-files docs/book | xargs cat | wc -c` → `458517`, 235 bytes of room. `M3.6.6.3`'s chapter took most of the rest
  the same day, and `M3.6.6.4`, `M3.3` and `M3.5` each owe the book their account.
  Direction: the measurement, and the book's layering (`docs/decisions/decision_book-in-layers.md`) — chapters and
  annexes — weighed against a dated raise of its total in that record, as the reviews folder's was.
  Decided `2026-10-10`: a raise, to 589 824 bytes, by the record's dated paragraph. Measured at `bfabefa`, `git
  ls-files docs/book | xargs cat | wc -c` → `458750`, two below the ceiling; `git ls-tree -r -l a274131 docs/book`
  summed → `395389`, the first book commit after the last raise; `git log a274131..bfabefa -- docs/book/src` → 38
  commits: 63 361 bytes in a week, about 1.7 KB a commit, as the record foresaw. Annexes move text, not remove it, so
  no layering frees room; the index, generated, is 44 785 bytes of it. The record asks the next raise to weigh a split.
  Verification: `2026-10-10` — `bash scripts/check_readme_routes.sh` → *"OK (24 destination(s) governed)"*, and its
  self-test → *"28 pass / 0 fail"*, the policy's cell and its decision's maximum agreeing; the doctrine gate at commit.
  Commit: `ARCHOGEN-PROGRAM-0526 (leaf PROGRAM.74)`
