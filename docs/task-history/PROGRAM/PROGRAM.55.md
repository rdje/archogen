- ID: `PROGRAM.55`
  Status: `done` — filed and closed `2026-10-05`
  Goal: the archive `decision_reviews-folder-ceiling.md` names as the step after its raise: a closed review history
  leaves `docs/reviews/` for `docs/review-history/`, byte for byte, behind a stub at its path so every citation
  resolves, proven against an index as `TASK-HISTORY` and `DECISION-HISTORY` prove theirs.
  Acceptance: `REVIEW-HISTORY` registered, with RED arms in `--self-test`, each leg proven by a mutation; the closed
  catalog history archived; the folders' rows in `README_POLICY.md` and `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`; every
  check green.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `M3.1.1` step 20's commit → `README-ROUTES: docs/reviews/: 395098 bytes in total, over
    its ceiling of 393216`; the ceiling record: a frozen history "cannot be compacted", and "if the rate holds, the
    next step is that archive … not another raise"; 264 219 bytes on `2026-10-03`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `docs/reviews/` keeps closed histories for good, 75 383 bytes of them the
    catalog's, closed; WHY: no mechanism let a frozen file leave (`rc=1` from `README-ROUTES`), which the record left
    to a `PROGRAM` leaf.
  - [x] **FIX** — `scripts/check_review_history.sh`: `--seal <FILE>`, six legs, a self-test; registered in
    `scripts/check_doctrines.project.sh` and `DOCTRINE_ENFORCEMENT.md` (the `DECISION-HISTORY` row tightened to fit);
    `docs/review-history/` in `README_POLICY.md` and `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`; the record's dated line; the
    reviews index's note → `bash -n scripts/check_review_history.sh` → `rc=0`.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_review_history.sh --self-test` → `38 pass / 0 fail (38 arms)`;
    three legs broken in turn → provenance `37 pass / 1 fail`, closed status `37 pass / 1 fail`, history `36 pass /
    2 fail`; `--seal decision_catalog-records-reviews.md` → `archived, 75383 bytes, byte for byte`, `cmp` against
    `HEAD`'s copy → `rc=0`; `docs/reviews/` → 317 648 bytes. ⛔ **Corrected by `PROGRAM.55.1`:** at this commit the
    folder held 2 589 lines and 317 779 bytes; 317 648 was measured before the reviews index's note.
  - [x] **NO REGRESSION** — `make focused` → `tier focused: passed — 3 passed, 0 failed`; `bash scripts/run_self_tests.sh` → `self-tests: OK — 46 self-test(s) passed`; `bash
    scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — the doctrine's row, the policy's two rows, the inventory's two rows, the ceiling record, the
    reviews index's note, this leaf and its log rows, `CHANGELOG.md`; the book states no folder's ceiling.
  Verification: `2026-10-05` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0438 (leaf PROGRAM.55)`

- ID: `PROGRAM.55.1`
  Status: `done` — filed and closed `2026-10-05`
  Goal: `REVIEW-HISTORY` hardened after its independent review (`docs/reviews/review-history-reviews.md`, 17
  findings, 5 defects), as `PROGRAM.41.1` hardened `DECISION-HISTORY`: every finding answered, every construction a
  RED arm, the claim of proof measured by a mutation matrix.
  Acceptance: each of E1–E17 answered in the gate, its header, the documents or the book; the self-test green; every
  mutation of the matrix killed, or its survival stated; `PROGRAM.55`'s figure corrected; every check green.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the review's constructions on the gate at `49c44ed`, each `rc=0` where a refusal was
    due: an open review archived by `closedness not reached`, a fifth cell, a hidden row; a merged round lost; a file
    ignored or unstaged passing the hook; 14 of 30 mutations surviving the 38 arms.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `scripts/check_review_history.sh`'s `statuses` read any line opening
    `| [` by its first word; leg 5 looked no later than the archiving commit; legs 1 and 4 read the working tree;
    the stub was any line matching. WHY: `PROGRAM.55` ported `DECISION-HISTORY`'s legs for files but not the
    review's own row, merge, staging or quoting cases, and its claim "each leg proven by a mutation" rested on three
    mutations (`36 pass / 2 fail` at most).
  - [x] **FIX** — the gate rewritten: the review's row read from the table alone, four cells, `closed: `, linked
    once; every later commit at the stub's path its stub; staged copies read; a closed history frozen; stubs by shape;
    merges refused; the seal's read-back; the self-test from 38 arms to 65; `.gitattributes`; the doctrine rows
    (D11's clause restored); the book; the review history and its index row → `bash -n
    scripts/check_review_history.sh` → `rc=0`.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_review_history.sh --self-test` → `65 pass / 0 fail`; a matrix
    of mutations, one per refusal and leg, each run against the self-test → 38 of 38 killed; `bash
    scripts/check_review_history.sh` on this repository → `OK (1 archived history(ies)`.
  - [x] **NO REGRESSION** — `make focused` → `tier focused: passed — 3 passed, 0 failed`; `bash scripts/run_self_tests.sh` → `self-tests: OK — 46 self-test(s) passed`; `bash
    scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — the gate's header; `DOCTRINE_ENFORCEMENT.md`'s two rows; `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s
    reviews row re-measured, with E12's decision; `.gitattributes`; `annex-repository.md` and `catalog.md`, the book's
    index regenerated; the review history and its index row; this leaf and its log rows; `CHANGELOG.md`.
  Verification: `2026-10-05` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0442 (leaf PROGRAM.55.1)`
