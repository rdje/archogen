- ID: `PROGRAM.91`
  Status: `done` — historical/pending work-unit presence requires working and indexed records
  Goal: close indexed presence gap while retaining live placement, immutable archive and backlog rules.
  Startup: clean mainfdbcec5, empty message/OS handoff clear; COMMIT durable-row contract, source
    working/indexed readers, PROGRAM.90 original fixture and Annex B reviewed before changes.
  - [x] **ROOT CAUSE** — original actual maintained-gate fixture reported2 work units, exit 0,
    with pending T-0002 only in working T.md. Native git show :docs/tasks/T.md proves it absent.
    Gate collected IDs only from working rows; indexed live validation discarded IDs and archive
    index records were outside that reader. Source and index-presence-probe.json bind reproduction.
  - [x] **ADDRESSED** — collect indexed row IDs in live and archived documents; every HEAD/pending
    subject requires working and indexed presence beyond the unchanged18-item backlog. Indexed-only
    restored backlog records must leave it. bash scripts/check_commit_log_rows.sh in the same
    original fixture: exit 1, naming missing indexed
    T-0002; stage its row, native indexed readback confirms presence and gate exit 0. Retain raw probe.
    Working/archive placement contracts are unchanged; historical records concealed by working copies
    and untracked pending tasks now refuse before commit.
  - [x] **NO REGRESSION** — bash scripts/check_commit_log_rows.sh --self-test, exit 0:
    27 pass / 0 fail (27 arms), including prior21 controls; positive pending/archive preparations now
    explicitly stage their records. Actual missing indexed pending/history/live/archive, untracked/
    staged new tasks and indexed-only restored backlog cases hold. Exact source-function comparison
    preserves live_placement bytes; native byte checks retain250 archive/index files and engine/
    proof/native sources. bash -n passes. Book/doctrine/live sync and actual pinned book/staged gates
    bind closure. No Rust source/focused run warranted; established gate method, no new DEV_NOTES
    lesson. M3 .13.3.2 still owns full caller/integration checkpoint; resume after this clean commit.
  Verification: `2026-10-18`, target/program-index-presence retains original/actual RED/green probe,
    native staged-blob confirmation, full27 self-test and source/archive audit. Ignored adapter is
    supplemental to maintained gate, real Git fixture, native byte checks and permanent controls.
    audit.json SHA256 e0abf927f9f940e7f9210f6bd9e05fd593a6a982bc7f1156a78f5038af1a4c22.
    Initial direct task-acceptance refused prose-only ADDRESSED result syntax; explicit invocation/exit
    results correct that same bullet before commit. Placement validation establishes literal unfenced
    table structure; source evidence proves row content.
  Commit: `ARCHOGEN-PROGRAM-0736 (leaf PROGRAM.91)`
