- ID: `PROGRAM.75`
  Status: `done` — filed, started and closed `2026-10-10`
  Goal: the scaffold's handoff census judges by a property that tells a background job from a respawned helper.
  Reproduce / issue: `PROGRAM.54`'s reproduction — `scripts/check_no_background_jobs.sh`, the scaffold's, names a
  helper alive for seconds, and its verdict depends on the instant it runs; archogen samples twice in a wrapper of its
  own, which the census's next update cannot undo, but every other project the scaffold serves meets the same.
  Direction: an outbound report to the scaffold's owner, `bedrock`, as a tracker under `docs/feedback/bedrock/`
  shaped as the LinkedSpec one is — a self-contained issue whose `repro.sh` exits 0 while the census names a helper of
  a few seconds' life — proposing the second sample; nothing written into `bedrock` itself
  (`decision_repository-boundary-read-only.md`).

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `bash docs/feedback/bedrock/issues/BR-001-handoff-census-instant/repro.sh
    /Volumes/SSD/Documents/github/bedrock` at bedrock `835547e` → exit 0: *"census while it lived ......... exit 1,
    names it: yes"*, *"census once it had exited ..... exit 0, names it: no"*; bedrock's working tree after it, `git -C
    … status --short` → nothing.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: bedrock's census takes one snapshot, and `835547e` answers helpers with
    an ignore list — in the bedrock checkout, `git show 835547e:scripts/check_no_background_jobs.sh | grep -c
    handoff_ignore` → `2`, the census last changed in `a860103`. WHY: one instant cannot tell a helper from a job, and
    a list must name each helper ahead of time.
  - [x] **FIX** — `docs/feedback/bedrock/`: the register, its README and state model, and `BR-001` — its README with
    the proposal, `SETUP.md`, `repro.sh`, `evidence/OBSERVED.txt`; the cross-vendor row in `docs/feedback/README.md`;
    the ledger's `bedrock` scope.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_feedback_self_contained.sh` → exit 0;
    `bash scripts/check_feedback_register.sh` → *"OK (2 vendor register(s) agree with their issues, their totals and
    the cross-vendor index)"*; `bash scripts/check_source_ledger.sh` → *"OK (22 entries …)"*.
  - [x] **NO REGRESSION** — `make focused` → `passed — 3 passed, 0 failed`, on the staged tree; the doctrine gate at
    commit.
  - [x] **LOCKSTEP** — the tracker, the cross-vendor index, the ledger; this leaf and both logs; `docs/TASK_TREE.md`;
    `CHANGELOG.md`.
  Verification: `2026-10-10` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0537 (leaf PROGRAM.75)`
