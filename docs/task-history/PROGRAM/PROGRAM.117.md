- ID: `PROGRAM.117`
  Status: `done` — filed `2026-10-27` on the director's ruling of that day; closed `2026-10-28` with `.3`, its three
    children done
  Goal: the record folders' ceilings bound what is read, not the corpus, so their growth never stops work.
  Ruling (`2026-10-27`, the director, reopening the one-time raise of `decision_decisions-folder-ceiling.md`): "I do
    not understand why there are even ceilings on those directories ... What's the best solution, to you, to handle
    nicely, elegantly those ceiling. I agree their size shall not blow up, shall somehow be contained but this shall
    not hinder progress. Propose a sota, production-grade solution."
  Reproduce / issue: `docs/specs/`' total was raised by `PROGRAM.56`, `.79`, `.102` and `.108`, four times in
    eighteen days; at `220eb078`, `git ls-tree -r -l HEAD -- docs/decisions docs/specs`: 392564 of 393216 and 649155
    of 655360. No reader's context loads a whole folder, so a total bounds no read; the per-file caps do.
  Children: `.1` the ceilings, the check and the record; `.2` the move tool the lifecycle needs; `.3` the index that
    still binds as one file.

- ID: `PROGRAM.117.1`
  Status: `done` — `2026-10-27`
  Goal: `README-ROUTES` reads a directory row's Files cell `fan-out N` — no directory it governs lists more than N
    entries, its tracked files (archive stubs aside) and sub-folders — and its Total bytes cell `guard N`, a runaway
    guard, with a `--report` of each folder's totals and growth; `docs/decisions/`, `docs/specs/` and `docs/reviews/`
    take both, under a new decision record that supersedes the three ceiling records; the policy, the inventory and
    the book follow.
  Built: the check's two cells (`fanned`, `guarded`) and its `--report` (`scripts/check_readme_routes.sh`); a guard
    refuses a total over it, three quarters of it unless the cell names an open leaf asking for a ruling or a tracked
    decision record that kept it, and a commit growing the folder in the index over `HEAD` past a sixty-fourth of it
    unless `README_ROUTES_GROWTH_LEAF` names a leaf open at `HEAD` or new in the index; every ceiling a whole number,
    compared fail-closed; `### Ceilings a decision fixes` compares a cell with its maximum by kind, a debt or a
    doctrine refused, a directory's maximum binding the rows beneath it through the nearest fixed ancestor; `docs/decisions/decision_ceilings-bound-reads.md`, quoting the
    ruling and the `nygard-adr` ledger entry; the three ceiling records marked superseded; the three rows at
    `fan-out 64` and `guard 16777216`.
  Review (`2026-10-27`, rounds by independent contexts): round 1 on the first draft, five defects and ten remarks;
    round 2 on the answers, two defects and thirteen remarks; round 3, three defects and four remarks; round 4, two
    defects and six remarks; round 5, no defect and three remarks, closing the review; all answered here.
    - D1, a number past 64 bits or with a leading zero failing a comparison open: `whole()`, every comparison
      `[ a -le b ] || note`; arms for each.
    - D2, a decision-fixed cell turned into a debt or a doctrine escaping its maximum: refused; arms.
    - D3, the notice at three quarters printed nowhere, `scripts/check_doctrines.sh` being held and silent on a pass:
      a refusal unless an open leaf asks for a ruling.
    - D4, the record's index split refused by `DECISION-INDEX`: the record states the single index binds; `.3` filed.
    - D5, "only a mistake reaches the guard", false by the record's own numbers, and a 6.3 MB dump passing: reworded,
      and the per-commit step added, sized against the largest honest growth since `6d61f65`, 38818 bytes
      (`efedf6d6`).
    - Remarks taken: the numbers (26 times, fifteen days, "no reader's context"); `docs/reviews/`' stubs and flat
      layout stated; `docs/specs/` has no index of its own; `knowledge-map/subsystems.md` for a new sub-folder;
      superseded records stay in place, `.2`'s goal narrowed; the inventory's widest-listing label; the two stale
      script comments; the self-test gaps.
    - Round 2, defect 1, the step measuring the disk while the commit holds the index — growth staged and deleted
      from the disk passed: `index_bytes()`, the index's blob sizes against `HEAD`'s; an arm.
    - Round 2, defect 2, a partition's row lifting a fan-out a decision fixes for its parent: a maximum fixed for a
      directory bounds every registered row beneath it unless the table fixes that row's own; three arms.
    - Round 2's remarks taken: `README_ROUTES_GROWTH_LEAF` open at `HEAD` or new; an empty ruling clause refused;
      `guard N, ruled by <record>` for a ruling that keeps a guard; the report's date the three-quarter refusal; the
      step's honest limits in the record and the book; the documented batch of 186617 bytes (`PROGRAM.43`); 131072
      for a review history; the index figures from `220eb078`; one reason for a malformed number under a maximum;
      the inventory and the failing-first figures re-measured on the final tree.
    - Round 3, defect 1, a `ruled by` clause satisfied by any pathspec — `.`, `*`, `README.md`: a decision record
      under `docs/decisions/` or `docs/specs/`, by its literal path; arms for each.
    - Round 3, defect 2, a leaf "new in the commit" read from the disk: `leaf_status_in`, at `HEAD` or in the index;
      an arm with a leaf on the disk alone.
    - Round 3, defect 3, a directory's fixed maximum reaching file rows and doctrine cells beneath it that could never
      comply: only directory rows, through the nearest fixed ancestor; arms for a file row and a nested partition
      (narrowed by round 4: a file row is bound in the per-file columns, and a doctrine cell beneath is refused).
    - Round 3's remarks taken: the book's and the report arm's dates the three-quarter refusal, pinned; `index_bytes`
      refusing an object it cannot size; the header rewrapped, `control`'s unused locals gone; this text. Its
      remark on path quoting in the on-disk measures predates this change, filed as `PROGRAM.118`.
    - Round 4, defect 1, file rows freed in every column, so a file row lifted a decided per-file maximum (a row for
      a file of `docs/task-history/` above `decision_task-history-line-ceiling.md`'s 3072): a file row is skipped in
      Files and Total bytes alone; three arms.
    - Round 4, defect 2, the header and `within_fixed`'s message stating round 2's rule: both rewritten.
    - Round 4's remarks taken: a row beneath a fixed column takes its kind, stated and pinned by an arm; an index or
      the template is no ruling; arms reaching the literal-path test (`docs/decisions/../../README.md`, a record on
      the disk alone) and the `unreadable` refusal; the table's rows matched with or without backticks.
    - Round 5's remarks taken: `within_fixed`'s messages read in order; an arm for a file row's doctrine cell under a
      fixed per-file column; a ruling clause names a record whose type is `decision`, the template and a reference
      note refused, and the header states what the check cannot tell.
    - Fence: the round-1 reviewer ran `git log -G'655360|589824|524288|458752|393216' -- README_POLICY.md`, which searched that
      file's diffs in commits older than `6d61f65` and printed commit subjects alone, and disclosed it. No path under
      `docs/evaluation/` was searched and no case text was printed, so no case was read and nothing joins the
      evaluation's *Exposures*; a brief's fence now names `log -G` and `log -S` with `-p`.
  Acceptance:
    - [x] ROOT CAUSE — WHY: `README_POLICY.md` bounded the record folders' totals, which no read loads, so the product's growth met a wall raised four times in eighteen days; WHERE, `git show 220eb078:README_POLICY.md`, the rows of `docs/decisions/`, `docs/specs/` and `docs/reviews/` and their maxima; at `220eb078`, `git ls-tree -r -l HEAD -- docs/specs`: 649155 of 655360.
    - [x] FIX — the check's cells, step and report; the policy's rows, note and maxima; the record; the three records superseded; the inventory, the enforcement table, `TOOLBOX.md`, the book's repository annex and glossary, the ledger, `docs/figures.md`; the review's answers above.
    - [x] ADDRESSED — `bash scripts/check_readme_routes.sh --self-test`: `readme-routes self-test: 113 pass / 0 fail (113 arms)`; `bash scripts/check_readme_routes.sh`: `readme-routes: OK (25 destination(s) governed)`. Failing first, this staged tree under `220eb078`'s policy: `docs/decisions/: 407037 bytes in total, over its ceiling of 393216`; under the new rows before the record existed: six breaches, `fixed by docs/decisions/decision_ceilings-bound-reads.md, which does not exist`.
    - [x] NO REGRESSION — `bash -n scripts/check_readme_routes.sh` exit 0; `bash scripts/check_doctrines.sh` passes; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 300 s
    - [x] LOCKSTEP — the check, the policy, the record, the inventory, the enforcement table, the toolbox, the book, the snapshots, two script headers; `eadl-1-a-plan-names-engine-adapters.md` flipped to `applied`, `M3.2.5`'s note.
  Commit: `ARCHOGEN-PROGRAM-1020 (leaf PROGRAM.117.1)`

- ID: `PROGRAM.117.2`
  Status: `done` — `2026-10-27`
  Goal: one scripted, verified step moves records — an accepted design from `docs/decisions/` to
    `docs/specs/<subject>/`, or a subject's records into a sub-folder of their own — their relative links rewritten,
    every path-form citation in a live file repointed, sealed history and scaffold-held files left and reported, the
    index rows updated, a new sub-folder linked from `knowledge-map/subsystems.md`; a batch past a folder's step,
    262144 bytes, names its leaf in `README_ROUTES_GROWTH_LEAF`; a superseded record stays in place
    (`decision_ceilings-bound-reads.md`).
  Built: `scripts/move_record.sh [--dry-run] <record> <destination>`. It refuses a record that is not a tracked `.md`
    under the two folders, an index or the template, a destination outside them or existing, and — for a move —
    unstaged changes. It `git mv`s the record, rewrites its own relative links, and repoints every written-out path
    and every relative markdown link that resolved to it in a live file. It leaves and lists a `<revision>:<path>`,
    the four archives, `CHANGELOG.md` and `DEV_NOTES.md`, `annex-trust-proofs.md`'s verbatim proofs and the held
    files; it never lists or opens `docs/evaluation/`. It stages the move only when no live file names the old path,
    runs `DECISION-INDEX`, and names a new folder for the knowledge map and a rewritten Rust source or trust root for
    `cargo xtask trust-inventory`. `TOOLBOX.md`'s README-routes row, the record and the book's repository annex say so.
  Acceptance:
    - [x] ROOT CAUSE — WHY: a record's move was a census by reading, its citations found and judged by hand (`PROGRAM.43`, `.56`, `.102`); WHERE, `PROGRAM.102`'s move, `git show --stat --format= efedf6d6`: `20 files changed, 94 insertions(+), 39 deletions(-)`, each edit chosen by reading, its FIX "every path-form citation repointed … bare names left … sealed history untouched".
    - [x] FIX — `scripts/move_record.sh`; `TOOLBOX.md`; the record's lifecycle and how-to; the book's repository annex.
    - [x] ADDRESSED — `bash scripts/move_record.sh --self-test`: `move-record self-test: 24 pass / 0 fail (24 arms)`, the sealed fixture unreadable throughout. On this tree, `bash scripts/move_record.sh --dry-run docs/decisions/decision_engine-api.md docs/specs/api/`: `move-record: planned, nothing changed (--dry-run)`, 12 citations and 2 links rewritten, 9 left as verbatim proofs — `git grep -c docs/decisions/decision_engine-api.md` counts 21 = 12 + 9.
    - [x] NO REGRESSION — `bash -n scripts/move_record.sh` exit 0; `bash scripts/check_doctrines.sh` passes; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 318 s
    - [x] LOCKSTEP — the tool, `TOOLBOX.md`, the record, the book, the snapshots.
  Commit: `ARCHOGEN-PROGRAM-1022 (leaf PROGRAM.117.2)`

- ID: `PROGRAM.117.3`
  Status: `done` — filed `2026-10-27` by `PROGRAM.117.1`'s review (D4); done `2026-10-28`
  Goal: an index never binds the folder it lists: `DECISION-INDEX` accepts a record linked from its sub-folder's own
    `INDEX.md` when the parent index links that index, and the specifications are indexed — two records of
    `docs/specs/trust/` are in no index, a third only in the reviews' index.
  Reproduce / issue: `scripts/check_decision_index.sh` requires every record under `docs/decisions/` to be linked from
    `docs/decisions/INDEX.md` itself; at `220eb078`, 18185 of its 98304 bytes, about 140 a day since `d00c3dee`
    (`2026-10-12`, 16118), so about a year and a half before it binds.
  Built: `DECISION-INDEX` walks the root index and every `INDEX.md` under `docs/decisions/` or `docs/specs/` that an
    index it already reached links, each link resolved against its own index's folder; a record is found by that path,
    and an index nothing reaches is refused, since what it lists would be listed nowhere. An index outside the two
    folders, the reviews', is never read as one. Its population is both folders, so the specifications are held too;
    the three it then found are indexed in `docs/decisions/INDEX.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the widened gate on the tree before the rows: `bash scripts/check_decision_index.sh` →
    `exit 1`, `3 breach(es)`, naming `docs/specs/trust/compiler-provenance-contract.md`,
    `docs/specs/trust/decision_trust-role-programs.md` and `docs/specs/trust/flag-normalization-domain.md`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — the gate read one folder and one index, every record's path taken relative to it
    and compared with the root index's links alone:
    `git show HEAD:scripts/check_decision_index.sh | grep -c 'DIR = "docs/decisions"'` → `1`.
  - [x] **FIX** — the walk over sub-indexes and the two folders in `scripts/check_decision_index.sh`, its header,
    seven new arms; the three rows; the registry's text, `DOCTRINE_ENFORCEMENT.md`'s row, the ceilings record's owed
    clause marked discharged, and the book's Annex B.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_decision_index.sh` → `exit 0`, `decision-index: OK (40
    record(s), 0 in partitions none, and 16 specification(s), each linked by its path from 1 index(es); every link
    resolves)`; `bash scripts/check_decision_index.sh --self-test` → `12 pass / 0 fail (12 arms)`.
  - [x] **NO REGRESSION** — no Rust source changed; `bash scripts/run_self_tests.sh` →
    `exit 0`, `self-tests: OK — 58 self-test(s) passed`; `bash scripts/check_doctrines.sh` → `exit 0`.
  - [x] **LOCKSTEP** — the registry, the enforcement table, `docs/decisions/decision_ceilings-bound-reads.md`, the
    book's `annex-repository.md`, this leaf and the Commit Log.
  Commit: `ARCHOGEN-PROGRAM-1037 (leaf PROGRAM.117.3)`
