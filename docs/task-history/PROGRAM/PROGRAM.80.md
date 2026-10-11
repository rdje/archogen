- ID: `PROGRAM.80`
  Status: `done` — filed, started and verified `2026-10-11`; the guarded wrapper is the project handoff command.
  Goal: the archogen handoff wrapper refuses when the required OS process census cannot run, rather than reporting
  the workspace safe to clear. The scaffold's neutral `check_no_background_jobs.sh` stays untouched.
  Reproduce / issue: in the restricted execution environment, `/bin/ps -p "$$" -o pid=` exits 126 (Operation not
  permitted), `lsof -p "$$" -Fp` exits 0, and `bash scripts/handoff_census.sh` exits 0 with `handoff: OK`. The native
  census suppresses `ps` errors and judges an empty snapshot, which the wrapper accepts. Probe output in
  `target/p80/restricted-probe.log` is untracked. Two process-dependent self-test suites fail under that restriction
  and pass with OS process access. This is a pre-existing false-green, independent of the review-count repair.
  Direction: keep the native script read-only; make the archogen wrapper establish that its actual required census
  commands succeeded, and refuse an unreadable sample. Add deterministic synthetic failures without OS denial;
  preserve both sampling and normal process detection. Investigate transient ancestry reads separately from the
  required process snapshot so a process that exits normally does not cause a false refusal.
  Acceptance: failed required process or open-handle reads exit 2 with a reason and never print handoff OK;
  ordinary empty, transient and persistent samples retain their documented outcomes; tracked synthetic arms and
  OS-access controls verify the fix; the book and toolbox name the authoritative wrapper and its limits.
  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — six tracked refusal arms on the previous core, with three passing controls:
    `bash target/p80/baseline-tree/scripts/handoff_census.sh --self-test` → `3 pass / 6 fail (9 arms)`.
    The old core and byte-identical native census run in a correctly rooted fixture; its producer and log are
    untracked. The original seven-arm run also gave `3 pass / 4 fail`, before adding second-sample controls.
  - [x] **ROOT CAUSE (WHY + WHERE)** — the native census suppresses a failed `ps` snapshot and judges the empty
    result; the wrapper accepted that verdict. `target/p80/restricted-probe.log` → `ps exit: 126`, `lsof exit: 0`,
    `wrapper exit: 0`. A successful preflight would not prove the later actual read, so the wrapper observes the
    census's actual commands. The held native file is unchanged, checked by `git diff --exit-code c153604 --
    scripts/check_no_background_jobs.sh` → exit 0.
  - [x] **FIX** — `sample` records successful required reads through exported functions scoped to its subprocess,
    with private same-volume receipts; absent receipts and an empty process snapshot refuse with exit 2. Both
    samples use it. A transient ancestry lookup is distinct from the required full process snapshot. The guarded
    wrapper, `CLAUDE.md` and `TOOLBOX.md` now agree on the project handoff command.
  - [x] **ADDRESSED (verified)** — `bash scripts/handoff_census.sh --self-test` with OS access → `9 pass / 0 fail
    (9 arms)`: failures in either sample, a missing census read and an empty snapshot, plus successful reads,
    transient and persistent processes. The real restricted run exits 2 with `required process snapshot did not
    complete; outcome unknown`, without OK; the real OS-access run exits 0 with `handoff: OK`. Zero receipt
    directories remain. The tracked arms are durable; the probe logs under `target/p80/` are not.
  - [x] **NO REGRESSION** — `bash -n scripts/handoff_census.sh` → exit 0; book anchors → 30 chapters / 4 normative
    documents; glossary → 67 acronyms / 25 terms; regenerated book index → 316 entries, its gate OK; doctrines at
    commit. No Rust source changed; these focused shell, actual OS and documentation checks cover the change.
  - [x] **LOCKSTEP** — wrapper, bootstrap, toolbox, book annex and its generated index; this leaf and both logs;
    changelog and resume/frontier snapshots. Removed stale hand-copied arm totals and duration estimates from
    the toolbox's adjacent self-test descriptions. README and product milestones unchanged.
  Verification: `2026-10-11` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0607 (leaf PROGRAM.80)`

