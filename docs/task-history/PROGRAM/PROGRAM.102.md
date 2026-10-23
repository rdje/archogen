- ID: `PROGRAM.102`
  Status: `done` — P2: `docs/decisions/` at its byte ceiling blocks `M3.3.6`
  Reproduce / issue: `2026-10-22`, `find docs/decisions -maxdepth 1 -type f -exec cat {} + | wc -c`: 392915, against the
    ruled 393216 (`README_POLICY.md`'s routed destinations; `docs/decisions/decision_decisions-folder-ceiling.md`), with 40
    files, the file ceiling. `M3.3.6`'s bundles need vocabulary rows in `decision_substitutability-relation.md` §1.1, which
    the folder cannot hold. Sealing settled sections frees about 1 KiB: the findings record's §10 and §12 are already
    stubs and §14 is 1292 bytes, its other long sections still with the director; the largest record,
    `decision_substitutability-relation.md` (72961 bytes), is live normative text.
  Goal: headroom by the folder's own routes — moving records that are specifications to `docs/specs/`
    (`docs/decisions/decision_specifications-home.md`, as the catalog's were), every link to them rewritten and the
    decisions index and `docs/specs/`'s ceiling held — never by raising a ceiling, which needs the director's ruling.
    Candidates to judge, each with its reason: `decision_task-tree-sealing.md` (33245 bytes), `decision_runtime-analysis-
    variant.md` (38794), `decision_runtime-contract-gaps.md` (29963).
  Measured `2026-10-22`, before any move, `git ls-tree -r -l HEAD -- docs/specs`: 524246 of its 524288 bytes, 13 of 16
    files. The goal's "`docs/specs/`'s ceiling held" was written unmeasured: no record fits in 42 bytes, so the ruled
    route needs that folder raised. Its ceiling is not the director's: `PROGRAM.56` and `PROGRAM.79` raised it under
    `README_POLICY.md`'s rule, each by a dated paragraph in `decision_specs-folder-ceiling.md` with the measurement, as
    that record asks; `docs/decisions/`' is the director's one-time raise, and stays.
  Decision (`2026-10-22`, under the director's delegation of that day): `decision_runtime-analysis-variant.md` moves to
    `docs/specs/analysis/`, by the ruling of `2026-10-01` — an accepted design, its review closed with `M2.11`'s fifth
    round, unchanged since `2026-10-03`, the specification `crates/rt-analysis/src/runtime.rs` implements; `docs/specs/`'
    total rises one 64 KiB step, 524288 → 589824, by a dated paragraph. Judged and set aside:
    - `decision_task-tree-sealing.md` — amended `2026-10-20` (`PROGRAM.95`), and cited from
      `scripts/check_task_acceptance.sh`, which `scripts/update_scaffold.sh` overwrites from upstream: a citation there
      could not follow the move.
    - `decision_runtime-contract-gaps.md` — `resolved`, a decision's rationale and reviews, not a design: its normative
      text already lives in `docs/profiles/rt-static-up-v1-faults.md`.
    - `decision_substitutability-relation.md` — live normative text `M3.3.6` amends next.
    - sealing the findings record's settled §14 and §15 — about 2.3 KiB, which the next record would spend; left for
      when they are needed.
  Acceptance:
    - [x] ROOT CAUSE — at `757c49a6`, `find docs/decisions -maxdepth 1 -type f -exec cat {} + | wc -c`: 392950 of the
      director's 393216, 266 bytes to spare, so `M3.3.6`'s rows and `M3.4`'s `SR-H18` amendment cannot land; `git ls-tree
      -r -l HEAD -- docs/specs`: 524246 of 524288, so the ruled route was closed too.
    - [x] FIX — `git mv` of the record to `docs/specs/analysis/`, its eight relative links one level deeper; every
      path-form citation repointed — `runtime.rs`'s header and `MODEL`'s citation, three test headers, the test's
      `record_path!`, `analysis.md`, `versions.md`, `model.md`, both indexes — bare names left, as they still name it,
      and sealed history untouched; `decision_specs-folder-ceiling.md`'s dated paragraph, `README_POLICY.md`'s two cells,
      `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s two rows re-measured; the knowledge map regenerated.
    - [x] ADDRESSED — `docs/decisions/`: 355040 of 393216, 38176 to spare, 37 files; `docs/specs/`: 563064 of 589824, 14
      of 16 files; `bash scripts/check_readme_routes.sh`: `readme-routes: OK (24 destination(s) governed)`, which refused
      the move first ("docs/specs/: 563064 bytes in total, over its ceiling of 524288"). `PROGRAM.104`'s pin held the
      move: the record moved alone, `couldn't read … docs/decisions/decision_runtime-analysis-variant.md`; the test's path
      alone, `test result: FAILED. 0 passed; 1 failed`, "does not cite"; both, `test result: ok. 1 passed`.
    - [x] NO REGRESSION — `cargo test -p rt-analysis`: 87 passed / 0 failed; `decision-index: OK (35 record(s) …)`;
      book-anchors, source-ledger, version-register and knowledge-map OK; `cargo test -p eadl-front --test reference`:
      `55 passed`; `make focused` on this staged tree: exit code 0, `tier focused: passed — 3 passed, 0 failed, 0
      unavailable, 0 not built, 0 quarantined`.
    - [x] LOCKSTEP — the ceiling record, the policy, the containment inventory, both indexes, the book's analysis and
      versions chapters, the map; the snapshots and `M3`'s frontier, the blocker gone.
  Verification: the routes gate refusing first; the pin refusing each half-done move; the crate's suite; the gates; focused.
  Commit: `ARCHOGEN-PROGRAM-0866 (leaf PROGRAM.102)`
