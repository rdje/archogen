- ID: `PROGRAM.69`
  Status: `done` — independent round 37 closes the review at zero new defects, additional actionable arm gaps
  or remarks, `2026-10-11`; tool, gate and earlier seal committed, all preceding findings answered.
  **Closure, round 37:** report `target/review-p69-r37/report.txt` read in full, reviewing `2cef1a9`.
  The retained 347 checker invocations comprise 260 core and 87 full pinned-shell evaluations: 341 pinned-reader
  outcomes (217 expected normal, 124 named refusals) and six valid local-mutant sensitivity witnesses (four wrong
  census totals, two fail-closed diagnostic tracebacks). Every outcome matches its expectation. Separately, 4 096
  finite unit/oracle comparisons agree. Root independently recounts retained results and authenticates 17 source
  snapshots with `python3 target/review-p69-r37/probes/verify_results.py` → exit code 0.
  Before closure, staged `check_review_counts.sh` → OK, 1 listed review and all 37 rounds judged; the closed
  review then leaves the counted list. Final gates judge the committed closure state.
  This review does not replay the complete 146/156-arm suites or establish native path production, sealing
  transactions, signals, Linux behavior or particular real historical byte aggregates. Reports/producers are
  untracked evidence, not durable enforcement; R36's distinguishing controls remain maintained in both scripts.
  No real task-history body or sealed evaluation case read, native Git fixture created, compiler/shared suite run,
  external repository written or own review job left. PROGRAM.71/.72/.73/.78 retain their separately owned limits.
  **Acceptance checklist, round 36:**
  - [x] **REPRODUCE / ISSUE** — unchanged production readers with new controls give history `133 pass / 13 fail
    (146 arms)` and counts `153 pass / 3 fail (156 arms)`, retained in `target/p69r36/`. The independent review's
    98 invocations and 2 187 unit comparisons are separately scoped, not a replay of the complete native suites.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `census()` always appends LF to a leaf span, including physical EOF, and
    filters suffixes on non-z Git display names, silently dropping quoted paths. New EOF and single/partial filename
    controls exit code 0 with wrong totals: `133 pass / 13 fail (146 arms)`. Both count parsers access `cells[0]`
    when an empty pipe row has no cells: `153 pass / 3 fail (156 arms)`; the new arms explicitly exclude tracebacks.
  - [x] **FIX** — derive source-byte boundaries from real line separators; consume NUL-delimited Git paths with
    explicit UTF-8 decoding and terminator validation; guard zero-cell rows with existing named diagnostics.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_task_history.sh --self-test` → `146 pass / 0 fail (146 arms)`;
    `bash scripts/check_review_counts.sh --self-test` → `156 pass / 0 fail (156 arms)`. Fourteen new census controls
    hold zero/one/two final LFs and an internal boundary, Unicode/tab/backslash/quote names singly and in partial
    totals, invalid listing UTF-8 and absent NUL terminator. Six count controls hold empty/zero-cell rows in findings
    and both summary shapes. Literal fixture UTF-8 byte lengths are the independent oracle, not the span algorithm.
  - [x] **NO REGRESSION** — complete suites above; `bash -n scripts/check_task_history.sh` and
    `bash -n scripts/check_review_counts.sh` → exit code 0. Staged count/history, book and doctrine gates before
    commit. No Rust changed; no new Rust-suite or real historical byte aggregate claimed.
  - [x] **LOCKSTEP** — script contracts, review record/history/index, book annex, owning leaf/logs and resume pointers.
    Compact four older design-summary verdicts to keep the decisions byte cap, preserving counts/key results;
    full review rounds remain unchanged. No real sealed evaluation text or external repository writes.
  Round 35 answered: one production defect, no additional actionable arm gaps and one remark; report read in full at
  `target/review-p69-r35/report.txt` (untracked). Answer: census converts a failed selected-tree
  blob read into an empty tree, so a successful zero/partial aggregate can omit completed work; a recognized summary
  header at end of input or a table-ending non-row can bypass its required separator. Both repaired: failed selected
  reads name commit/path and refuse without a final aggregate; each recognized summary header requires its next
  separator. Eight census and twelve summary-boundary controls distinguish the repairs. Round 36 next.
  **Acceptance checklist, round 35:**
  - [x] **REPRODUCE / ISSUE** — new controls on unchanged readers: history `129 pass / 3 fail (132 arms)`;
    counts `140 pass / 10 fail (150 arms)`. Failed selected reads and ten incomplete summary boundaries exit 0.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `census()` uses `leaves(at(commit, path) or "")`, conflating failed Git
    evidence with a valid empty tree: `SELF-TEST: selected-tree census last-error — rc 0`. `summary()` clears shape
    before judging a non-row and overwrites a pending header; it never checks EOF: `SELF-TEST: a recognized summary
    header before eof requires its separator: | Round | Findings | Defects | Verdict | (R35 R1) — rc 0`.
    Full before runs: `129 pass / 3 fail (132 arms)` and `140 pass / 10 fail (150 arms)`;
    `target/p69r35/history-before.log` and `target/p69r35/count-before.log`.
  - [x] **FIX** — refuse selected `None` reads with commit/path; judge pending separators before every later line
    and after input ends. Both shapes held at EOF, newline, blank, prose, another header and malformed row.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_task_history.sh --self-test` →
    `132 pass / 0 fail (132 arms)`; `bash scripts/check_review_counts.sh --self-test` → `150 pass / 0 fail (150 arms)`.
    The eight finite census cases include successful one/two-tree totals, genuine empty content, first/last read
    failures, listing failure and invalid blob UTF-8; failures cannot print a final successful aggregate.
  - [x] **NO REGRESSION** — complete suites `132 pass / 0 fail (132 arms)` and `150 pass / 0 fail (150 arms)`;
    `bash -n scripts/check_task_history.sh` and `bash -n scripts/check_review_counts.sh` → exit 0;
    staged `REVIEW-COUNTS` → `OK (1 review(s) listed; 35 round(s) judged, every one)`; task history →
    `OK (175 sealed file(s)` and 335 stubs; book anchors/glossary/index → OK (30/4, 67/25, 316). Real HEAD census
    exits 0; final doctrines in hook. No Rust changed; no new Rust-suite claim. Logs in `target/p69r35/`, untracked.
  - [x] **LOCKSTEP** — checker contracts, review history/record, book, task evidence and resume/frontier snapshots.
    Commit gate found the decisions folder 261 bytes above its cap after this answer; compact three older review
    summary cells, preserving counts and key results. Full historical detail stays in the unchanged review rounds.
  **Acceptance checklist, round 34:**
  - [x] **REPRODUCE / ISSUE** — removing unconditional validation and rejecting parentheses in angle destinations
    each give `126 pass / 0 fail (126 arms)` under native Git. The independent report
    `target/review-p69-r34/report.txt` found zero production defects and these two arm gaps; read in full.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_review_counts.sh`, self-test: all unsupported-record controls
    were open/unlisted, so the later completeness path still parsed them without global validation; no angle matrix
    path contained parentheses. Both valid mutations survive `126 pass / 0 fail (126 arms)` before these controls.
  - [x] **FIX** — listed/closed record and empty-list closed-history refusals; an all-closed passing control;
    parenthesized angle destinations across both primary states and all four title spellings. Production core unchanged.
  - [x] **ADDRESSED (verified)** — deleting validation now gives `135 pass / 3 fail (138 arms)`; restrictive angle
    syntax gives `130 pass / 8 fail (138 arms)`. Each embedded mutant parses before running, and its log has no
    syntax error or traceback. Producers in `target/p69r34/` are untracked; all distinguishing arms are tracked.
  - [x] **NO REGRESSION** — `bash scripts/check_review_counts.sh --self-test` → `138 pass / 0 fail (138 arms)`;
    shell syntax clean; book anchors 30/4, glossary 67/25, index 316 OK. Staged counts 1 review / 34 rounds, history
    175 files / 335 stubs; stated-order, task-acceptance and figure-register OK; final doctrines in the commit hook.
    No Rust changed; the prior M3 focused claim is not restated.
  - [x] **LOCKSTEP** — controls, review/record summary, book, task evidence and current resume pointers.
  **Acceptance checklist, round 33:**
  - [x] **REPRODUCE / ISSUE** — the 35 new tracked controls beside the committed reader give
    `96 pass / 30 fail (126 arms)`; the independent report is `target/review-p69-r33/report.txt` (untracked).
  - [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_review_counts.sh`, `LINK` and `linked`: the regex recognized
    only a whitespace-free bare destination ending immediately at `)`, and `LINK.search` skipped a first link it
    could not parse. The pre-repair suite gives `96 pass / 30 fail (126 arms)`; the pinned independent shell reads
    no primary record for angle/title links, or reads only the secondary explanation, and falsely exits 0.
  - [x] **FIX** — parse the first unescaped balanced label and its explicit inline destination/title grammar;
    refuse unsupported first syntax, including reference/shortcut links; validate both file cells before counting.
    Paths remain literal without decoding; a bracket-free cell names no file; counted pairs remain authoritative.
  - [x] **ADDRESSED (verified)** — repaired suite `126 pass / 0 fail (126 arms)`; all five original shell cases
    refuse the primary table and read no explanation. Selecting the last supported link gives `93 pass / 33 fail`;
    omitting unsupported-first refusal gives `120 pass / 6 fail`, both of 126. The first selection mutant's syntax
    error is invalid evidence, excluded; the corrected producer parses the Python core before running it.
  - [x] **NO REGRESSION** — `bash scripts/check_review_counts.sh --self-test` → `126 pass / 0 fail (126 arms)`,
    including all earlier 91 arms. Shell syntax clean; final staged counts OK (2 reviews / 35 rounds), history OK
    (175 files / 335 stubs); book anchors 30 chapters / 4 normative documents, glossary 67 acronyms / 25 terms,
    index 316 entries; stated-order OK. Task-acceptance and final doctrine checks run again in the commit hook.
  - [x] **LOCKSTEP** — script contract, leaf, review history and summary, book, changelog and resume pointers.
  Goal: a closed subtree below an open top-level subtree is sealed too, so a tree whose top-level subtree stays open
  for long — on a blocked leaf, or a long feature — does not keep its finished leaves live.
  Reproduce / issue: `M3.6.6.2.1`'s commit was refused by `README-ROUTES`, *"docs/tasks/: 823457 bytes in total, over
  its ceiling of 819200"*. The seal releases only a top-level subtree whose every leaf is `done`
  (`decision_task-tree-sealing.md`, "The unit is a closed subtree"), and on `2026-10-10` the only ones left were
  `PROGRAM.66` and `.68`, sealed by that commit to bring the folder to 815 443 bytes before this leaf was written. Measured the same day by a census
  of every leaf's span: 271 393 bytes of `done` leaves sit under top-level subtrees with an open leaf — `M1` 84 363,
  `M2` 104 992, `M3` 82 038 — and 258 649 of them in 28 subtrees below the top level whose every leaf is `done`, which
  a finer unit would seal — *corrected the same day by the review's R2-3 and R3-1: those were character counts of a
  working tree; in bytes at `fea69ad`, by `bash scripts/check_task_history.sh --census fea69ad`, 274 483, `M1` 85 113,
  `M2` 105 946, `M3` 83 424, and 261 610 in the 28; and, by R4-4, the 823 457 and 815 443 above were measured on the
  working tree, `M3.6.6.2.1`'s edits staged, and reproduce at no commit* — `M3.6.1`, `M3.6.2`, `M3.6.4`, `M3.6.6.1` among them, under `M3.6`, which stays open while
  `M3.6.5` waits on the director. Each sub-leaf of `M3.6.6.2` adds kilobytes to `M3.md`, so the next commit crosses
  the ceiling again.
  Direction, to be designed here: the unit becomes the outermost closed subtree — a subtree at any depth whose every
  leaf is `done` and whose parent is open or the tree itself — sealed to `docs/task-history/<TREE>/<SUBTREE>.md`; a
  sealed file still never changes, since its subtree is closed, and when its parent later closes, the parent's
  remaining leaves seal to the parent's own file beside it. The record, the tool, the gate's legs 4–6 and its
  self-test change together, under a review by a context that did not write them, as the record's own was; the option
  the director ruled, (C), is "closed leaves sealed out of the task trees", which this keeps.
  Acceptance: a seal of the real trees releases what the census measures, each reconstruction byte for byte; the gate
  refuses a live leaf inside a sealed subtree at any depth, a sealed file that changes, and a nested seal whose parent
  is closed; every arm fails first; the record amended and reviewed.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `M3.6.6.2.1`'s first commit attempt, the doctrine gate → *"README-ROUTES: docs/tasks/:
    823457 bytes in total, over its ceiling of 819200"*. `bash scripts/check_task_history.sh --census fea69ad` →
    *"census: fea69ad — 274483 byte(s) of `done` leaves under open top-level subtrees, 261610 of them in 28 closed
    subtree(s) below the top level"*: `M1` 85 113, `M2` 105 946, `M3` 83 424, which no seal could take.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `git show fea69ad:scripts/check_task_history.sh | grep -n 'return
    ".".join(parts\[:2\])\|key = subtree(lid, tree_name)'` → `122`, `subtree` fixing a unit at two components, and
    `324`, the seal grouping every leaf by it; leg 6 at `277` and leg 5's whole-subtree rule at `312` judged by the same
    key. WHY it bites now: the unit assumed a top-level subtree closes soon after its parts, and `M3.6` stays open while
    `M3.6.5` waits on the director, as `M2.7` does on `M2.7.4.5` and `M1.29` on `M1.29.4`.
  - [x] **FIX** — `scripts/check_task_history.sh`: `under` and `units`, each live leaf's outermost closed subtree, a
    top-level key always a candidate and a deeper one only as a leaf of the tree; the seal groups by `units`, refuses a
    tree holding a leaf not under its name before it writes, and on a rollback removes the folders it made; leg 4 by
    `under`; leg 5 rebuilds each file's bytes from its leaves' spans in the tree before the seal, requires each sealed
    leaf's outermost closed subtree there to be its file's; leg 6 refuses
    a live leaf under any sealed key, at any depth, in any tree file; the index's layout, one table per tree with its
    rows under its header, checked; `--census <COMMIT>`. The record amended sentence by sentence; its review reopened
    (`ARCHOGEN-PROGRAM-0510`) and rounds 2–5 and the sweep appended; `DOCTRINE_ENFORCEMENT.md`'s row, `TOOLBOX.md`, the
    book's annex, the index's header. After round 6: a row twice in its tree's table and a leaf named twice in a tree
    refused by name, the seal refusing the second before it writes; a file the gate cannot read, met after the seal
    wrote, rolled back as any refusal is; the check of a stub's commit, added after round 5, withdrawn, and the two
    first-seal stubs it had "corrected" restored. `PROGRAM.70` filed for `docs/reviews/`' file ceiling.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_task_history.sh --self-test` → *"66 pass / 0 fail (66 arms)"*.
    `python3 target/m369/mutate.py` → twenty named mutations of the tool, each `killed` by the arm for its rule;
    `target/review-p69-r3/mutate2.py`, round 3's twenty → 16 killed, the four left equivalent as rounds 4 and 5
    argued — both runners untracked, so these counts are not durable. The sweep of the core, `target/m369/sweep.py`,
    untracked and not durable → its third run, on round 5's tool, *"196 mutant(s) … 182 killed, 14
    not"*, each of the 14 reasoned in the review history. The real seal: `--seal M1` → *"sealed 7 subtree(s), 7
    leaves"*, `M2` → *"10 subtree(s), 27 leaves"*, `M3` → *"11 subtree(s), 12 leaves"*, `PROGRAM` → *"nothing to
    seal"*, each *"the reconstruction is byte for byte"*; the seal takes 255 871 bytes out of `docs/tasks/`, which held
    818 985 at `72bd446`; the 28 files hold
    261 582 bytes, the census's 261 610 less the blank line each subtree leaves in its tree. Rounds 2 to 5 re-derived the
    seal with code of their own, byte for byte `fea69ad`'s leaves and exactly the outermost closed set there; round 4
    fuzzed 840 seals against an oracle and 890 hand seals, the gate accepting exactly the tool's — its runner
    untracked, so those counts are not durable. After round 7: a sealed file holding no leaf refused; every write of a
    seal and its proof in one guard, so any exception that stops the run rolls the seal back first — a kill signal
    excepted, a stated limit (R8-2); a column-0 line below the top
    level refused, by an arm; `--self-test` → *"69 pass / 0 fail (69 arms)"*. After round 8: a refused
    seal never reported sealed, the separator and an empty table checked → *"71 pass / 0 fail (71 arms)"*, each new rule
    failed by a mutation. After round 9: the tree and the index written whole or not at all; the rollback undoing what
    was written alone, each step on its own, naming what it could not undo; a sealed file's path already taken refused
    before any write — the file-size and taken-path arms failing first on `7ad8e6b`'s tool, then `--self-test` →
    *"77 pass / 0 fail (77 arms)"*; `python3 target/p69r9/mutate.py` → nine mutations of round 9's rules, each `killed`,
    its runner untracked, so not durable. After round 10: each write noted before it is made; any entry at a sealed
    file's path refused, a unit the index rows skipped; a linked tree or index refused — the tree, index and create
    interrupt arms, the taken-file and the linked-tree arms failing first beside `0a7ec7f`'s core, the
    just-before-creation arm holding the existence guard (corrected by R11-D2), on the working tree and so not
    durable, then `--self-test` → *"85 pass / 0 fail (85 arms)"*; `python3 target/p69r10/mutate.py` → eleven mutations
    of round 10's rules, each `killed`, its runner untracked, so not durable. After round 11: a stop recorded and
    answered by a rollback that runs to its end; a linked history folder refused — `target/p69r11/hybrid.sh`, these
    arms beside `b87adbc`'s core → *"82 pass / 9 fail (91 arms)"*, the seven stop arms and the linked folder failing,
    then `--self-test` → *"90 pass / 0 fail (90 arms)"*; `python3 target/p69r11/mutate.py` → six mutations of round
    11's rules, each `killed`; both runners untracked, so not durable — the hybrid's tally with SIGINT at its default, in
    the foreground (R12-R3). After round 12: a recorded stop answered once the handlers are given back, a later one
    finding the seal kept; a link on any written path refused; "rolled back" said of a whole rollback alone; a signal
    ignored on entry left ignored — `target/p69r12/hybrid.sh`, these arms beside `76d8775`'s core → *"88 pass / 11 fail
    (99 arms)"*, the given-back interrupt, the two linked folders and the two not-whole rollbacks among them, then
    `--self-test` → *"98 pass / 0 fail (98 arms)"*; `python3 target/p69r12/mutate.py` → seven mutations, each
    `killed`; both runners untracked, so not durable. After round 13: a stop held by the signal mask until the seal is
    done, the recorded-stop route removed; the temporary file created new — `target/p69r13/hybrid.sh`, the arms as
    committed beside `1ad87e4`'s core → *"88 pass / 10 fail (98 arms)"*, the held-stop arms, the refused seal's rollback
    and the temporary path failing, then `--self-test` → *"98 pass / 0 fail (98 arms)"*; `python3
    target/p69r13/mutate.py` → five mutations, each `killed`; both runners untracked, so not durable. After round 14:
    every signal but nine held from before the first write, the outcome recorded and the interrupt's message by
    it, an error said before it goes on, the temporary paths refused first — `python3 target/p69r14/hybrid.py`, the
    arms as committed beside `6c79b36`'s core → *"97 pass / 9 fail (106 arms)"*, then `--self-test` → *"106 pass / 0
    fail (106 arms)"*; `python3 target/p69r14/mutate.py` → *"7 of 7 killed"*; both runners untracked, so not durable.
    After round 15: the seal records that it started, and the stop's message is by that and by its said outcome; the
    texts name the nine signals left unheld; arms for the mask at the first write, a lost report, the index's
    temporary path, the error path's breach and its rollback's wording — `python3 target/p69r15/hybrid.py`, the arms
    as committed beside `421ce47`'s core → *"108 pass / 1 fail (109 arms)"*, the lost report's, the four arm gaps
    passing there as rules already kept; then `--self-test` → *"109 pass / 0 fail (109 arms)"*; `python3
    target/p69r15/mutate.py` → *"6 of 6 killed"*; both runners untracked, so not durable. After round 16: the
    interrupt alone says which outcome, exiting 130 with its output closed; SIGABRT held, six fault signals left out;
    arms for the mask's eight and for the start before the first write — `python3 target/p69r16/hybrid.py`, the arms
    as committed beside `755c151`'s core → *"109 pass / 2 fail (111 arms)"*, the mask's and the lost report's exit;
    then `--self-test` → *"111 pass / 0 fail (111 arms)"*; `python3 target/p69r16/mutate.py` → *"5 of 5 killed"*; both
    runners untracked, so not durable. After round 17: an outcome said only to a stream that exists; one exit,
    `quiet_exit`, whatever became of the outputs; an abort and the C library's own signals named; arms with each output
    closed or broken and for the git that inherits the mask — `python3 target/p69r17/hybrid.py`, the arms as committed
    beside `4105124`'s core → *"112 pass / 4 fail (116 arms)"*, the four output arms; then `--self-test` → *"116 pass /
    0 fail (116 arms)"*; `python3 target/p69r17/mutate.py` → *"4 of 4 killed"*; both runners untracked, so not
    durable. After rounds 18 and 19: texts and comments alone, `--self-test` → *"116 pass / 0 fail (116 arms)"*. After
    round 20: texts narrowed, and a race arm planting a link at the temporary path as `put` opens it — `--self-test` →
    *"117 pass / 0 fail (117 arms)"*; with `put`'s exclusive creation or its creator-only removal broken, the arm fails,
    by an untracked runner, so not durable. After round 21: an arm sealing past an uncommitted edit that only reopens a
    subtree the seal leaves live — `--self-test` → *"118 pass / 0 fail (118 arms)"*; it fails a seal that refuses any
    uncommitted tree, by the round-22 reviewer's untracked runner. After round 22: arms for a sealed file a file-size
    limit stops halfway, for a `pending` field with a log row and for no warning where the log names the commit, and the
    column-0 arm requiring each line named — `--self-test` → *"120 pass / 0 fail (120 arms)"*; `python3
    target/p69r17/mutate22.py` → *"4 of 4 killed"*, one each — untracked, so not durable. After round 23: a folder
    the seal made that holds another writer's entry named, the rollback not called whole; a second torn leaf in another
    unit; `REVIEW-COUNTS` — `--self-test` → *"121 pass / 0 fail (121 arms)"*; `bash scripts/check_review_counts.sh
    --self-test` → *"9 pass / 0 fail (9 arms)"*, and before row 22's fix it refused exactly it; `python3
    target/p69r17/mutate23.py` → *"2 of 2 killed"*, each arm failing alone — untracked, so not durable. After round 24:
    `REVIEW-COUNTS` reads the reviews index's row and keys rounds on their lines, an archived record named, not read
    (not so, review round 25 found: eight unnamed) —
    `bash scripts/check_review_counts.sh --self-test` → *"18 pass / 0 fail (18 arms)"*; `python3
    target/p69r17/mutate24.py` → *"8 of 8 killed"*, each arm failing alone — untracked, so not durable; the
    `TASK-HISTORY` row's review narrative was cut in round 23 for the doctrine file's 36 864-byte ceiling. After round
    25: `REVIEW-COUNTS` narrowed to what it holds exactly — the index status, prose, unread; duplicates refused; every
    row it judges nothing of named (not so, review round 26 found) —
    `bash scripts/check_review_counts.sh --self-test` → *"20 pass / 0 fail (20 arms)"*;
    `python3 target/p69r17/mutate25.py` → *"6 of 6 killed"* — untracked, so not durable. After round 26, by method:
    `REVIEW-COUNTS` judges the reviews its script lists and refuses any it cannot judge, nothing derived or skipped (not
    so, review round 27 found: a round with another table header passed unjudged) —
    `bash scripts/check_review_counts.sh --self-test` → *"25 pass / 0 fail (25 arms)"*; `python3
    target/p69r17/mutate26.py` → *"8 of 8 killed"* — untracked, so not durable. After round 27, strict: every round of
    a listed review keeps a findings table headed exactly, its separator row and an id in each row; the summary table
    its separator; an open review whose record keeps a summary table listed from its first round —
    `bash scripts/check_review_counts.sh --self-test` → *"35 pass / 0 fail (35 arms)"*; `python3
    target/p69r17/mutate27.py` → *"12 of 12 killed"* — untracked, so not durable. Found while answering `M3.6.9`'s
    round 1: the list kept in the reviews index, under *Counted by `REVIEW-COUNTS`*, so a review in another tree joins
    it with no change to the script — `bash scripts/check_review_counts.sh --self-test` → *"38 pass / 0 fail (38
    arms)"*; `python3 target/p69r17/mutate_list.py` → *"4 of 4 killed"* — untracked, so not durable. After round 28:
    every index row read by its cells and links, an unreadable open row refused; the list's rule a record keeping a
    table of its rounds, any shape; a listed review's row open; the findings table first, no dash or kindless row;
    every stated count judged in either shape (not so, review round 29 found: indented or malformed index rows
    skipped, and only the first count of a kind judged) —
    `bash scripts/check_review_counts.sh --self-test` → *"51 pass / 0 fail (51 arms)"*; `python3
    target/p69r17/mutate28.py` → *"13 of 13 killed"*, `mutate27b.py` → *"11 of 11 killed"* — untracked, so not durable.
  - [x] **NO REGRESSION** — `bash scripts/check_task_history.sh` at `e624001` → *"OK (170 sealed file(s) … every sealed
    leaf proven against its tree before its seal)"*: the 142 seals before this change judged by every generalised leg, the byte
    rebuild included. `bash scripts/run_self_tests.sh` → *"OK — 47 self-test(s) passed"*; the
    gates that read what this change touched, each re-run → `stated-order` *"10 pass / 0 fail"*, `review-history` *"65
    pass / 0 fail"*, `readme-routes` *"20 pass / 0 fail"*, `handoff-ledger` *"14 pass / 0 fail"*.
    `handoff-ledger` reads the sealed quotes; `review-history` holds the reopened history; the doctrine gate at commit.
  - [x] **LOCKSTEP** — `docs/decisions/decision_task-tree-sealing.md`, its review history and `docs/reviews/INDEX.md`'s
    row, `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`, `docs/book/src/annex-repository.md`, `docs/task-history/INDEX.md`;
    this leaf and both logs, `PROGRAM`'s changelog; the frontier, `docs/TASK_TREE.md`, `LIVE_STATUS.md`
    and `MEMORY.md`; `CHANGELOG.md`.
  **Round 29 acceptance:**
  - [x] **REPRODUCE / ISSUE** — added synthetic regression arms before repair: the count check, 53 pass / 13 fail
    (66 arms); the history check, 122 pass / 2 fail (124 arms). Logs under `target/p69r29/` are untracked; the arms
    are tracked. No real sealed text was read and no real history was changed.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `check_review_counts.sh` selected only column-0 pipe rows, skipped short
    rows and used `re.search` for each stated count; `check_task_history.sh` treated any failed historical index
    `git show` as a path deletion. Those shortcuts hid obligations instead of refusing unreadable evidence.
  - [x] **FIX / ADDRESSED** — an explicit four-column index table parser accepts indentation, optional edge pipes
    and escaped pipes; malformed rows and headers refuse. Every stated count is judged in both summary shapes
    (not so, round 30 found: an escaped summary-cell pipe hid later counts).
    Historical absence requires a successful independent `git ls-tree` read. The stale Status is overwritten.
    The tracked arms pass: 69 / 0 and 124 / 0 respectively, including three additional table-grammar arms and
    a real historical deletion. The duplicate-row fixture now puts its row inside the actual table.
  - [x] **LOCKSTEP** — record, review history and index; book annex; leaf, frontier, both logs, changelog and resume
    snapshots. No Rust source changed; shell syntax, real gates, census and book gates pass. The broader self-test
    runner was confirmatory and still in flight at that commit; its final result is recorded below.
  **Round 30 acceptance:**
  - [x] **REPRODUCE / ISSUE** — eight tracked synthetic arms put a false second count after a Markdown-escaped
    pipe, four counted kinds in each summary shape: `--self-test` → `70 pass / 8 fail (78 arms)` before repair.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `summary` split on every pipe instead of calling the index's escaped-pipe
    parser; the independent report's plain/escaped counterparts isolate that split (`target/review-p69-r30/`,
    untracked). The rest of the cell disappeared before `stated` could judge it.
  - [x] **FIX / ADDRESSED** — one `cells_of` parser now serves all three tables; summary cells keep escaped pipes.
    `bash scripts/check_review_counts.sh --self-test` → `78 pass / 0 fail (78 arms)`, both shapes covered by a
    correct-count control too. No separate parser can drift between these tables.
  - [x] **NO REGRESSION / LOCKSTEP** — shell syntax, staged real count gate, history gate, book gates and doctrine
    checks; review record/history/index, book annex, this tree's logs, changelog and snapshots. Round 29's broader
    runner finished: 47 suites passed in the restricted environment; the two process-dependent suites passed with
    OS access, 2/2 handoff arms and 39/39 spine arms, in the same runner's CI-like Git environment. These are two
    runs covering all 49 suites, before the round-30 parser repair; the parser's focused 78-arm run is current.
    The sandbox failure also exposed a real handoff false-green, filed and owned as `PROGRAM.80`, next.
  **Round 31 acceptance:**
  - [x] **REPRODUCE / ISSUE** — three tracked first-link arms on the previous core → `79 pass / 2 fail (81 arms)`:
    a secondary round table and a missing secondary link both falsely refused; the primary-table control passed.
  - [x] **ROOT CAUSE (WHY + WHERE)** — the unlisted-row rule iterated `LINK.findall(c[1])`, while the declared
    grammar selects the first link. The independent probe changed only that traversal and reversed the result
    (`target/review-p69-r31/report.txt`, untracked). Counted pairs remain their own declared authority.
  - [x] **FIX / ADDRESSED** — the rule uses the existing `linked` helper, just as the history cell does;
    `bash scripts/check_review_counts.sh --self-test` → `81 pass / 0 fail (81 arms)`, the first record's table
    still requiring listing. The review's 42 core evaluations and seven parity checks found no escaped-pipe
    regression; those probe measurements are untracked, not durable.
  - [x] **NO REGRESSION / LOCKSTEP** — shell syntax, real staged count/history gates, book gates and doctrines;
    record, history, index, book annex, this leaf and logs, changelog and resume snapshots. No Rust changed.
  **Round 32 acceptance:**
  - [x] **REPRODUCE / ISSUE** — `bash target/p69r32/baseline.sh --self-test` → `83 pass / 8 fail (91 arms)`, the
    new tracked arms with `5d5daf1`'s unchanged reader; the baseline producer/output is untracked. Present staged
    reads and failed proofs falsely pass with good disk copies, including a primary record hiding a round table.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `check_review_counts.sh`'s `read(path)` treated any nonzero `git show` as
    index absence. The reviewer ran the unchanged pinned shell with a synthetic Git shim: normal bad staged record
    → exit 1, the same present record's read returning 128 → exit 0 via its good disk copy. The eight tracked
    counterexamples → exit 0 on the baseline, proving the shared fallback branch hides obligations.
  - [x] **FIX** — after a failed staged read, a successful `git ls-files --full-name -z` census must prove the complete
    path's bytes absent before the disk fallback. It passes no pathspec to Git, so inherited pathspec settings cannot
    reinterpret the query. A failed proof or present unreadable entry exits 1 with a reason;
    the successful staged-read path and genuine-absence fallback retain their meaning.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_review_counts.sh --self-test` → `91 pass / 0 fail (91 arms)`.
    Index, history and record read faults are separate; absence-proof faults cover both present and absent paths;
    genuine absence passes; an unreadable unlisted primary record refuses; bracket characters are compared literally.
    An inherited `GIT_LITERAL_PATHSPECS=1` control also refuses a present unreadable entry.
    The first nonliteral-query mutation passed 89 arms: Git also matches a present file's exact spelling. The new
    genuine-absence control distinguishes another staged file matching that spelling as a glob. That first query draft also failed under `GIT_LITERAL_PATHSPECS=1`: the new environment arm
    gave `90 pass / 1 fail (91 arms)` before the census repair. A read-only Git probe on a known tracked path gave
    one entry normally, zero with that setting and the magic prefix. Census without a pathspec finds the same
    945 entries under each of the four Git pathspec settings separately and all together. Final exact-comparison
    mutation → `89 pass / 2 fail (91 arms)`: both complementary filename controls refuse glob comparison.
    Probe/mutation logs are untracked; the filename/environment arms are durable.
  - [x] **NO REGRESSION / LOCKSTEP** — `bash -n scripts/check_review_counts.sh` → exit 0; real count gate → OK,
    2 reviews / 32 rounds before appending this answer; final staged count → OK, 2 reviews / 33 rounds; history gate
    → OK, 175 files; book anchors/glossary/index → OK. Final doctrines run in the commit hook. Checker contract, record/history/index, book annex, this leaf and logs,
    changelog and resume/frontier snapshots agree. No Rust changed; no full-suite or final-tree Rust claim is made.
  Verification: `2026-10-11` — the Verification Log's round-29 to round-37 rows
  Commit: `ARCHOGEN-PROGRAM-0510 (leaf PROGRAM.69)`, the review reopened; `ARCHOGEN-PROGRAM-0511 (leaf PROGRAM.69)`;
  `ARCHOGEN-PROGRAM-0514 (leaf PROGRAM.69)`, round 7 answered; `ARCHOGEN-PROGRAM-0517 (leaf PROGRAM.69)`, round 8;
  `ARCHOGEN-PROGRAM-0521 (leaf PROGRAM.69)`, round 9; `ARCHOGEN-PROGRAM-0525 (leaf PROGRAM.69)`, round 10;
  `ARCHOGEN-PROGRAM-0531 (leaf PROGRAM.69)`, round 11; `ARCHOGEN-PROGRAM-0535 (leaf PROGRAM.69)`, round 12;
  `ARCHOGEN-PROGRAM-0539 (leaf PROGRAM.69)`, round 13; `ARCHOGEN-PROGRAM-0543 (leaf PROGRAM.69)`, round 14;
  `ARCHOGEN-PROGRAM-0546 (leaf PROGRAM.69)`, round 15;
  `ARCHOGEN-PROGRAM-0548 (leaf PROGRAM.69)`, round 16;
  `ARCHOGEN-PROGRAM-0552 (leaf PROGRAM.69)`, round 17;
  `ARCHOGEN-PROGRAM-0556 (leaf PROGRAM.69)`, round 18;
  `ARCHOGEN-PROGRAM-0559 (leaf PROGRAM.69)`, round 19;
  `ARCHOGEN-PROGRAM-0563 (leaf PROGRAM.69)`, round 20;
  `ARCHOGEN-PROGRAM-0564 (leaf PROGRAM.69)`, round 21;
  `ARCHOGEN-PROGRAM-0567 (leaf PROGRAM.69)`, round 22; `ARCHOGEN-PROGRAM-0576 (leaf PROGRAM.69)`, round 23; `ARCHOGEN-PROGRAM-0582 (leaf PROGRAM.69)`, round 24; `ARCHOGEN-PROGRAM-0586 (leaf PROGRAM.69)`, round 25; `ARCHOGEN-PROGRAM-0590 (leaf PROGRAM.69)`, round 26;
  `ARCHOGEN-PROGRAM-0598 (leaf PROGRAM.69)`, round 27; `ARCHOGEN-PROGRAM-0599 (leaf PROGRAM.69)`, the list in the index;
  `ARCHOGEN-PROGRAM-0602 (leaf PROGRAM.69)`, round 28; `ARCHOGEN-PROGRAM-0605 (leaf PROGRAM.69)`, round 29;
  `ARCHOGEN-PROGRAM-0606 (leaf PROGRAM.69)`, round 30; `ARCHOGEN-PROGRAM-0608 (leaf PROGRAM.69)`, round 31;
  `ARCHOGEN-PROGRAM-0612 (leaf PROGRAM.69)`, round 32; `ARCHOGEN-PROGRAM-0615 (leaf PROGRAM.69)`, round 33; `ARCHOGEN-PROGRAM-0618 (leaf PROGRAM.69)`, round 34;
  `ARCHOGEN-PROGRAM-0620 (leaf PROGRAM.69)`, round 35; `ARCHOGEN-PROGRAM-0622 (leaf PROGRAM.69)`, round 36; `ARCHOGEN-PROGRAM-0624 (leaf PROGRAM.69)`, closed at round 37
