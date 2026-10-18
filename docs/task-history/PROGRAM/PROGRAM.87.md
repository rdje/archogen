- ID: `PROGRAM.87`
  Status: `done` — native provenance and complete headed log rendering verified
  Goal: repair PROGRAM log placement/rendering while preserving every record. Reproduce the existing
    PROGRAM.85/0707 row after Changelog using native source/heading positions, trace its introduction
    and any other affected rows through native documentation history, assess blank-separated tables,
    and move only proven misplaced/detached records into contiguous headed tables. Verify actual
    pinned rendering, row counts/order/multiplicity/bytes and public row-presence gate scope.
  Reproduction: tail docs/tasks/PROGRAM.md shows the 0707 row after ## Changelog, outside ## Commit Log.
  Impact: the rendered headed log omits the ninth cleanup record. No source/evidence row may be lost.
  Schedule: after PROGRAM.86/.88, before resuming M3 .13.2.4. Own any further confirmed log defects.
  Startup: clean main 85434b9, empty/untracked message and OS-readable handoff census. Read task-tree
    log/template contracts, COMMIT.md, row-presence gate, prior M3 layout repair and book repository
    section. Own current PROGRAM record layout/history/rendering only; source and frozen archives stay.
  - [x] **ROOT CAUSE** — complete current source census: 204 prior commit rows (203 headed-section,
    one after Changelog), only 182 in the contiguous table; 154 prior verification rows, only 135
    contiguous. `python3 -I -B target/program-log-layout/audit-repair.py`, exit 0: all 41 affected
    current row versions traced through 53 native task-doc commits after fence 6d61f65; all absent at
    that boundary, exact introductions have no matching row in their direct parent. First detached
    commit/verification row is c4c61209ea8418b729b1a0151628fecaa7dfe9af (PROGRAM-0604, round13), with
    a blank immediately before each. Misplaced 0707 begins at 112f6973274d2997d711348a032f47b52fd7d549,
    after Changelog. Native ancestry places it after 0687 and before 0724; no guessed lexical sorting.
    Actual pinned rendering confirms 182/135 before, 204/154 after relocation, all cells present.
    Supplementary renderer first assumed PROGRAM-area subjects; corrected one legitimate M1 filing
    reference before final complete subject/multiplicity comparison. No source defect in that record.
    Task collection measured 817912 bytes before this evidence (native indexed path census/Python stat),
    1288 headroom. Own published sealing of only committed closed PROGRAM bodies here to fit closure;
    current .87 remains active, all prior rows and frozen archives must stay exact.
  - [x] **ADDRESSED** — preserve all 204 prior commit-record and 154 verification-record rows,
    exact bytes/multiplicity, with all old headed-row and verification order unchanged. Stable-merge
    0707 between native ancestors 0687/0724; remove blank data-row separators and nothing from the
    records. `python3 -I -B target/program-log-layout/render.py`, exit 0, actual pinned mdBook: before
    182/135 headed rows, relocation 204/154, complete 3/4 cells and all subject multiplicities,
    including the legitimate M1 filing reference. Blank-after-first controls render one row in only
    the affected table. Independent structure refuses wrong location, changed/missing/duplicate row
    and blank separator. Append this unit's records contiguously only after all prior records.
    `bash scripts/check_task_history.sh --seal PROGRAM`, exit 0: five committed closed subtrees,
    six leaves, exact reconstruction; gate 241 sealed files / 489 stubs. Current .87 stayed active.
    Native HEAD/system-hash corroboration holds all 236 prior archive files exact and old index rows
    in order; new bodies' core lines match HEAD. Supplemental comparison normalizes only terminal
    inter-leaf blanks; the published reconstruction/history gate proves exact source preservation.
  - [x] **NO REGRESSION** — actual unchanged public check_commit_log_rows.sh accepts proper,
    after-Changelog, blank-separated and changed-note controls, exit 0; deleting the 0707 row exits 1,
    specifically its missing record. This holds the gate's defined presence-only scope, not layout/
    truth validation. Every controlled file write is restored exactly in finally. Maintained
    `python3 -I -B scripts/trust_adequacy.py --snapshot-inputs --output target/program-log-layout/inputs.json`
    then `--check-inputs target/program-log-layout/inputs.json --output target/program-log-layout/restoration.json`,
    both exit 0: all 1034 regular inputs, Python/system hashes/bytes/modes/sizes/index and opaque pin
    metadata restore before closure prose. No source/script/old-archive change; new seals owned here.
    Native Git history/ancestry, actual pinned renderer, template and maintained input/history/presence
    gates are primary; extraction/orchestration scripts/receipts are ignored supplemental adapters.
    Final 205/155-row render with new unit, book/index/anchors/glossary/source/figure/routes/history/
    order/live/staged gates precede commit. No new Rust-focused/full integration/remote claim.
    Sealed PROGRAM.83/.84/.88 retain their original terminal blank lines. Git whitespace checking
    covers authored paths; exact archival bytes are validated by TASK-HISTORY/native hashes.
  - [x] **LOCKSTEP** — current book explains complete log rendering and retained legacy references;
    live/resume/index return to committed M3 .13.2.4 matcher assessment, ordinary PROGRAM.71 next.
    All old global log/changelog records retained, no new DEV_NOTES lesson or product status category.
  Verification: `2026-10-18` — target/program-log-layout retains committed before source, 41 native
    introduction/break records, minimal merge ancestry, actual before/after/separator book HTML,
    preserved seals/index, public gate boundary/restoration and independent rejected record controls.
    History and row/renderer claims are bounded to PROGRAM logs; no whole-repository history audit.
    Retained receipts, Python/system SHA-256 agree; restoration predates closure edits and rendering
    below is the final 205/155-row run, superseding the intermediate render hash after this unit:
    history-audit.json 39bbc61731b557565572217a0167856e65947d1d51a026b53b210c1652e6de5a;
    render-audit.json 6871247c893aedd3fbced231b1d9fa6a7a6ad78652040cbe0f2b61da0e35654d;
    seal-audit.json 7334f1b38d962f656bdae2562d32700a67cf4a1b6179c80153dbd830df12bd66;
    boundary.json dbc40c9f673e2bf94eacb8aefc63a79d76129e725959eb63d0defd815cfa7885;
    inputs.json a80ad2d22346c71beea8d61763f04e4eb953d75e7c39c169f6a7c11aefc72fe4;
    restoration.json 45706ac3ca05e1cba356fb7a3b2367287a72f4c125ef28fef47400b97bd288c1;
  Commit: `ARCHOGEN-PROGRAM-0726 (leaf PROGRAM.87)`
