- ID: `PROGRAM.41`
  Status: `done`
  Goal: the next compaction of `docs/decisions/`, named by `docs/decisions/decision_decisions-folder-ceiling.md`. The
  findings register's settled items, §2, §4, §8 and §10, about 9.7 KB, are sealed out of it the way closed leaves
  are: moved byte for byte, with a stub per item keeping its section number, since the numbers are cited across the
  trees.
  Why: after the partition (`PROGRAM.39`), the folder was 366 543 bytes on `2026-09-30` against a cap of 393 216.
  Each review round of the catalog design adds several kilobytes, and only a new ruling can raise the cap.
  Acceptance: the items moved and stubbed, every citation of their section numbers still resolving; a digest and
  a check that the moved text never changes, reusing `TASK-HISTORY`'s pattern or a register of its own; the
  folder's measurement in the inventory.
  Verification: see the checklist.
  Commit: `ARCHOGEN-PROGRAM-0234 (leaf PROGRAM.41)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE** — after `ARCHOGEN-M2-0233`, `git ls-tree -r --name-only b8448c6 docs/decisions | xargs -I{}
    git show b8448c6:{} | wc -c` → 376904, short of the trigger by 3 096 bytes, with catalog round 13's and
    composition round 6's answers still to come.
  - [x] **ROOT CAUSE** — the folder grows by every review round's answers, and the ceiling cannot rise without a new
    ruling. The compaction was named in advance: `git show b8448c6:docs/decisions/decision_decisions-folder-ceiling.md
    | grep -n "next compaction"` → line 105, the findings register's settled items. They are cited by number across
    the trees, so they could not move without a stub under each heading.
  - [x] **FIX** — `scripts/check_decision_history.sh`, `DECISION-HISTORY`: `TASK-HISTORY`'s pattern for numbered
    sections. `--seal` moved §2, §4, §8 and §10 into `docs/decision-history/decision_findings-for-director-review/`,
    each heading kept above a one-line stub, with the reconstruction proven before writing. The gate's five legs:
    rows, one to one, across history, stubs, provenance. `README_POLICY.md` routes the folder, `archive_terminal`.
  - [x] **ADDRESSED** — the seal printed `sealed section(s) 02 04 08 10, 9745 bytes; the reconstruction is byte for
    byte`; `bash scripts/check_decision_history.sh` → rc=0, `decision-history: OK (4 sealed section(s) …)`;
    `bash scripts/check_decision_history.sh --self-test` → rc=0, `decision-history self-test: 20 pass / 0 fail (20
    arms)`. The folder is 368 643 bytes, the register's intro and the ceiling record included. Mutations on a copy:
    removing leg 3's file check, leg 5's comparison, the history rows or the live-again check each turns its own arm
    red (`19 pass / 1 fail`). Removing the seal's reconstruction proof turns none red: no arm reaches it without a
    fault in the seal's own code, as for `TASK-HISTORY`.
  - [x] **NO REGRESSION** — `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`; `bash
    scripts/run_self_tests.sh` → rc=0, `self-tests: OK — 37 self-test(s) passed`. No held file changed; every
    citation of §2, §4, §8 and §10 still finds its heading.
  - [x] **LOCKSTEP** — `docs/decisions/decision_decisions-folder-ceiling.md` and the register's intro,
    `README_POLICY.md`, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, `DOCTRINE_ENFORCEMENT.md`,
    `docs/book/src/verification.md`; this leaf, the frontier and both logs; `LIVE_STATUS.md`, `docs/TASK_TREE.md`,
    `MEMORY.md`, `CHANGELOG.md`.


- ID: `PROGRAM.41.1`
  Status: `done`
  Goal: `DECISION-HISTORY` hardened after its independent review (`2026-09-30`), which accepted the first seal as
  correct and lossless and found the gate not yet sound enough to seal more. Its findings, D1–D18, and the answer to
  each are in `docs/reviews/decision-history-reviews.md`. The two it names first:
  - D1: history simplification hides a seal behind a merge (`git merge -s ours`), and the gate then passes an edit
    to the settled text;
  - D2: a fence model that toggles on backticks alone lets a seal swallow a live section or tear one.
  Acceptance: each finding answered in the script, its header or the records; a RED arm for each construction the
  review gave, each failing for its stated reason; every leg the review's mutation matrix found unarmed armed, or
  said why it cannot be; the gate and every self-test green.
  Verification: see the checklist.
  Commit: `ARCHOGEN-PROGRAM-0238 (leaf PROGRAM.41.1)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE** — each defect the review built, rebuilt as an arm and run against the gate with the fix
    mutated out on a copy (`target/tmp/p411/mutate.py`): without `--full-history` on the rows read, the `-s ours`
    merge passes (`decision-history self-test: 53 pass / 1 fail (54 arms)`); with backtick-only fences, a tilde
    fence's `## ` line is sealed (`53 pass / 1 fail`); with any closing run accepted, a fence holding a shorter one
    tears the section (`52 pass / 2 fail`).
  - [x] **ROOT CAUSE** — `git show 4973e33:scripts/check_decision_history.sh | grep -n 'git("log"'` shows both
    history reads without `--full-history`, so git's default simplification follows one side of a merge that is
    TREESAME to it and never visits the seal. The same file's `sections()` toggled a fence on any line starting
    with three backticks, so a longer fence, or a tilde one, was misread. The rest were legs with no arm and limits
    left unsaid.
  - [x] **FIX** — the gate rewritten: `--full-history` on both reads; CommonMark fences; the seal refusing a section
    with another `## ` line or ending inside a fence, a record with a carriage return or no final newline, a
    malformed or repeated number; rollback on any refusal; a shallow repository and a failed history read refused;
    the index's header, every stub line outside a fence, and the row's date checked; the folder listed by git; the
    honest limits stated. `.gitattributes` keeps the folder from line-ending conversion.
  - [x] **ADDRESSED** — `bash scripts/check_decision_history.sh --self-test` → rc=0, `decision-history self-test: 54
    pass / 0 fail (54 arms)`; the mutation matrix → 41 of 42 mutations turn an arm red, the 42nd the added-files
    read whose only construction the rows leg refuses first; `bash scripts/check_decision_history.sh` → rc=0,
    `decision-history: OK (4 sealed section(s) …)`.
  - [x] **NO REGRESSION** — `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`;
    `bash scripts/run_self_tests.sh` → rc=0, `self-tests: OK — 37 self-test(s) passed`. The first seal is unchanged;
    no held file changed.
  - [x] **LOCKSTEP** — `docs/reviews/decision-history-reviews.md` and its index row, `DOCTRINE_ENFORCEMENT.md`,
    `docs/decisions/decision_decisions-folder-ceiling.md`, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, `.gitattributes`;
    this leaf, the frontier and both logs; `LIVE_STATUS.md`, `docs/TASK_TREE.md`, `MEMORY.md`, `CHANGELOG.md`.

