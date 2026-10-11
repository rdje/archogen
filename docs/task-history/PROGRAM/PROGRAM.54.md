- ID: `PROGRAM.54`
  Status: `done` — started and closed `2026-10-10`: the project's second sample; the scaffold's census itself is
  `PROGRAM.75`'s proposal
  Goal: the handoff census (`scripts/check_no_background_jobs.sh`) tells a background job from a transient helper
  without an allowlist.
  Reproduce / issue: found `2026-10-05` by three review readers (`M3.1.1` rounds 18 and 19, `M3.6.1` round 9), each
  told to end on `handoff: OK`: the census named iTerm's `pidinfo --git-state <repo> 4 1`, whose command line names
  the checkout — the census's PROJECT-WORK arm — respawned every few seconds by the terminal (PIDs 2171, 6514, 10743;
  38355, 42120, each 4–5 s old), while a run between two respawns prints `handoff: OK`. So the verdict depends on when
  it runs. Direction, to be designed here: a process counts only if a second sample a few seconds later still holds
  it, since a job that can rewrite tracked files outlives that and a helper does not — a property, as the census's
  own header asks, not a list of names. The script is the scaffold's (`scripts/update_scaffold.sh`'s `NEUTRAL` list),
  so an edit here would be overwritten by the next update: the fix is proposed to the scaffold's owner, never written
  into another repository (`decision_repository-boundary-read-only.md`), with a project-side second sample meanwhile.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — a helper whose command line names the checkout, `bash -c 'sleep "$1"; :'
    "$PWD/handoff-census-arm" 4`, alive: `bash scripts/check_no_background_jobs.sh` → exit `1`, naming it; once it
    ended, the same run names it no more (`grep -c handoff-census-arm` → `0`) — the verdict depends on the instant.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: the scaffold census's one `ps -Ao` snapshot, `git grep -n 'SNAP=' --
    scripts/check_no_background_jobs.sh` → `scripts/check_no_background_jobs.sh:80`. WHY: one instant cannot tell a process that lives seconds from
    one that lives on, and the script is the scaffold's, so it is not this project's to change.
  - [x] **FIX** — `scripts/handoff_census.sh`: the scaffold's census, then, when it names anything, a second after
    `HANDOFF_RESAMPLE_SECONDS` (8 unless set); a process counts only when both name it by PID and command line, the
    rest reported as transient. `TOOLBOX.md`'s row. `PROGRAM.75` filed for the scaffold's owner.
  - [x] **ADDRESSED (verified)** — `bash scripts/handoff_census.sh --self-test` → *"2 pass / 0 fail (2 arms)"*: a
    process the first sample names and the second does not, reported transient and not counted; one both hold, named
    and refused. A copy without the second sample (`target/p54/mut/`, untracked, so not durable) → *"1 pass / 1 fail"*.
  - [x] **NO REGRESSION** — `bash scripts/run_self_tests.sh` → *"OK — 48 self-test(s) passed"*, the new one among
    them; `make focused` → `passed — 3 passed, 0 failed`; the doctrine gate at commit.
  - [x] **LOCKSTEP** — `TOOLBOX.md`; this leaf, `PROGRAM.75` and both logs; `docs/TASK_TREE.md`; `CHANGELOG.md`.
  Verification: `2026-10-10` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0533 (leaf PROGRAM.54)`
