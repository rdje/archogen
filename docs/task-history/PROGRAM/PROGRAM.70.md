- ID: `PROGRAM.70`
  Status: `done` — started `2026-10-10`, decomposed into `.1`, the bytes, and `.2`, the count, closed the same day with `.2`
  Goal: a new design's review history always has room in `docs/reviews/`.
  Reproduce / issue: `PROGRAM.69` opened `docs/reviews/decision_task-tree-sealing-amendment-reviews.md`, and
  `bash scripts/check_readme_routes.sh` → *"docs/reviews/: 17 tracked files, over its ceiling of 16"*. The folder holds
  `INDEX.md` and fifteen histories, two of them stubs of archived histories (`bash scripts/check_review_history.sh
  --seal` leaves one at each history's path, so a citation still resolves), and an archive never lowers the count.
  So the next design whose review needs a history of its own — `M3.6.6.4`'s, `M3.7`'s — is refused, and `PROGRAM.69`
  reopened a closed history instead (`ARCHOGEN-PROGRAM-0510`).
  Direction, to be decided here with a measurement: stubs that leave the folder, their citations repointed; the count
  ceiling raised by a decision record, with the rate measured; or one index line standing for each archived history.
  Measured `2026-10-10` at `0a7ec7f`, `git ls-files docs/reviews | wc -l` → `16` and `… | xargs cat | wc -c` →
  `391513`, of ceilings 16 and 393 216: the bytes block a new history first, about 1.7 KB left, less than one round. Of
  the 15 histories, 2 are archived stubs and 11 more are closed — `docs/reviews/INDEX.md`'s rows — and only 2 open. A
  stub cannot leave: sealed task-history files cite both archived stubs (`git grep -l` → `docs/task-history/M2/M2.7.1.md`,
  `PROGRAM/PROGRAM.36.md`, `PROGRAM/PROGRAM.50.md`), and a sealed file never changes, so the first and third directions
  would break citations nothing may repoint. Decided: the bytes come back by archiving every closed history, which
  `PROGRAM.55`'s archive exists for (`.1`); the count stops counting what is no history — a stub, three lines
  `REVIEW-HISTORY` proves exact on every commit — so `README-ROUTES` counts the histories a reader may open (`.2`).

- ID: `PROGRAM.70.1`
  Status: `done` — started and closed `2026-10-10`, in two commits
  Goal: every closed review history archived out of `docs/reviews/`, byte for byte, behind its stub.
  Acceptance: `bash scripts/check_review_history.sh --seal <FILE>` for each history whose row reads closed; the gate
  and `README-ROUTES` pass; the folder's bytes measured before and after.
  Step 1, `2026-10-10` (`ARCHOGEN-PROGRAM-0522`): seven archived, each *"read back byte for byte"*; the three rows
  reading *"closed `<date>`: …"* put in the archive's form, *"closed: `<date>`, …"* — the tool archives a row whose
  status begins `closed: `, read at `HEAD`, so their archiving is step 2's; `decision_runtime-analysis-variant-reviews.md`
  refused and kept live — *"08a707bbbd3b changed … while its review's row already read closed"*: `ARCHOGEN-M2-0344`,
  already on `origin/main`, reopened its review in the commit that appended the round, and pushed history is not
  rewritten (the archive's own limit). No link anchors into an archived history (`git grep` → none).
  Step 2 (`ARCHOGEN-PROGRAM-0523`): the three, their rows closed at `HEAD`, archived, each *"read back byte for byte"*.
  Verification: `2026-10-10` — `git ls-files docs/reviews | xargs cat | wc -c` → `391513` at `0a7ec7f`, `262852`
  after step 1, `78575` after step 2, 16 files throughout; `bash scripts/check_review_history.sh` and
  `bash scripts/check_readme_routes.sh` at each commit, in the doctrine gate → green.
  Commit: `ARCHOGEN-PROGRAM-0523 (leaf PROGRAM.70.1)`, with step 1's `ARCHOGEN-PROGRAM-0522`

- ID: `PROGRAM.70.2`
  Status: `done` — started and closed `2026-10-10`; `PROGRAM.70` closes with it
  Goal: `README-ROUTES` counts the histories in `docs/reviews/`, not the stubs `REVIEW-HISTORY` proves.
  Acceptance: an exact archive stub is left out of a directory's file count and of nothing else, and a file that only
  looks like one is counted; an arm for each, failing first; the decision record's dated paragraph, `README_POLICY.md`'s
  row and the book's account changed with it.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `git ls-files docs/reviews | wc -l` at `2493bf8` → `16`, of a ceiling of 16, 12 of them
    stubs: a new design's history is refused, *"docs/reviews/: 17 tracked files, over its ceiling of 16"*, as
    `PROGRAM.69` met it. The first new arm, an exact stub beside four files, run on `2493bf8`'s script → *"expected
    exit 0, got 1 … docs/tasks/: 5 tracked files, over its ceiling of 4"*, `22 pass / 1 fail`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `git show 2493bf8:scripts/check_readme_routes.sh | grep -nF 'n=$((n +
    1))'` → `247`, the directory branch counting every tracked file beneath. WHY: the 16-file ceiling (`PROGRAM.36`) bounded the histories a reader faces, before
    an archive left stubs that can never leave — sealed task-history files cite them — so every history ever reviewed
    would count against it for good.
  - [x] **FIX** — `scripts/check_readme_routes.sh`: `is_archive_stub`, `REVIEW-HISTORY`'s stub exactly — three lines and
    a final newline, the second blank, the third the archive line naming the file itself, its archive tracked — left
    out of a directory's file count alone. `README_POLICY.md`'s adoption note; the ceiling record's dated paragraph and
    How to apply; `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s two rows, re-measured; the book's annex.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_readme_routes.sh --self-test` → *"28 pass / 0 fail (28 arms)"*:
    an exact stub left out; a look-alike counted — no archive, a fourth line, text after the last newline, a second
    line not blank, another file named, another archive linked; a stub's bytes in the total. `python3
    target/p70/mutate.py` → seven mutations of the rule, each `killed` — its runner untracked, so not durable. `bash
    scripts/check_readme_routes.sh` → *"OK (24 destination(s) governed)"*, the folder 4 files against 16.
  - [x] **NO REGRESSION** — `bash scripts/run_self_tests.sh` → *"OK — 47 self-test(s) passed"*; `make focused` →
    `passed — 3 passed, 0 failed`; the doctrine gate at commit.
  - [x] **LOCKSTEP** — the policy's note, the ceiling record, the size inventory, the book's annex; this leaf,
    `PROGRAM.70`'s and both logs; the frontier, `docs/TASK_TREE.md`, `MEMORY.md` and `LIVE_STATUS.md`; `CHANGELOG.md`.
  Verification: `2026-10-10` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0524 (leaf PROGRAM.70.2)`
