- ID: `PROGRAM.114`
  Status: `done` — filed `2026-10-26` by `PROGRAM.113.3`
  Goal: no gate script reads a captured output through `printf … | grep -q` under `pipefail`, where an output larger
    than a pipe turns a match into a failure on Linux.
  Reproduce / issue: the census of `scripts/*.sh` running under `pipefail` finds the idiom 78 times outside
    `check_task_history.sh`, most in self-test arms — `check_frozen_evaluation.sh` 7, `check_task_acceptance.sh` 6,
    `check_feedback_register.sh` 5, and more; each latent, failing only once its output outgrows a pipe, as
    `PROGRAM.113.3` measured.
  Direction: each made a here-string, and a doctrine leg refusing the idiom in a `pipefail` script, its RED arm the
    pipe the padded experiment broke.
  Built: every such read in this repository's `pipefail` scripts made a here-string — seventy-six, in forty files,
    one by hand where its patterns come from a process substitution — and `PIPE-READS`, `scripts/check_pipe_reads.sh`,
    refusing the form in any tracked shell script that sets `pipefail`, a comment not code, registered in
    `scripts/check_doctrines.project.sh`, described in the book's Annex B. The scaffold's eleven sites —
    `check_task_acceptance.sh` six, `check_lesson_promotion.sh` two, `check_routing_evidence.sh` two,
    `check_no_background_jobs.sh` one — are reported, not refused, as `SCRATCH-LOCALITY` reports its own; the change
    they need, for whoever owns the scaffold, never sent there by an archogen agent: each `printf '%s' "$x" | grep -q…`
    or `echo "$x" | grep -q…` made `grep -q… <<< "$x"`. `DOCTRINE_ENFORCEMENT.md` is at its ceiling — twenty-six bytes
    free — so its row waits on that table's compaction, `PROGRAM.115`.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the idiom `PROGRAM.113.3` measured failing a matched check on Linux stood in other gates' self-tests, latent until an output outgrows a pipe; WHERE, `git show 34b6abeb:scripts/check_frozen_evaluation.sh` and the census's forty-four files.
    - [x] FIX — the reads converted; the gate, its registration and the book.
    - [x] ADDRESSED — `bash scripts/check_pipe_reads.sh`: `pipe-reads: OK (74 pipefail script(s) scanned; 11 site(s) in scaffold-owned files reported, …)`; `bash scripts/check_pipe_reads.sh --self-test`: `pipe-reads self-test: 8 pass / 0 fail (8 arms)`; one owned site put back refused, `scripts/check_decision_index.sh:69`; every touched gate's self-test under GNU grep passing, `scripts/run_self_tests.sh`'s two real-tree arms again after the index and the ceiling were answered.
    - [x] NO REGRESSION — `bash scripts/check_doctrines.sh`: every doctrine but `TASK-ACCEPTANCE`, which this leaf answers; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 268 s.
    - [x] LOCKSTEP — the scripts, the gate's registration, the book; the enforcement table's row `PROGRAM.115`'s.
  Commit: `ARCHOGEN-PROGRAM-0980 (leaf PROGRAM.114)`
