- ID: `PROGRAM.108`
  Status: `done` — P2: `docs/specs/` at its byte ceiling blocks `M3.5.2`'s round-8 answers
  Reproduce / issue: `2026-10-23`, `git ls-files docs/specs | xargs cat | wc -c`: 589 151, against 589 824
    (`README_POLICY.md`; `docs/decisions/decision_specs-folder-ceiling.md`), 673 to spare; the trust role-programs
    record's open review amends it and the two records it amends at each round.
  Goal: the total one 64 KiB step higher, 589 824 → 655 360, by a dated paragraph in the ceiling's record, as
    `PROGRAM.102` raised it; the routes table and the size-containment row re-measured.
  Verification: the folder measured, 589 151 of 655 360; `bash scripts/check_doctrines.sh` green on the staged tree.
  Commit: `ARCHOGEN-PROGRAM-0908 (leaf PROGRAM.108)`
