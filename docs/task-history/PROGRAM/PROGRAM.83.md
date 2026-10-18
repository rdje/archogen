- ID: `PROGRAM.83`
  Status: `done` — preparation and verified archival committed separately
  Goal: archive the newly closed PROGRAM/M3 task records and completed design-review histories without changing
  their bytes, retaining all original citations through proven stubs.
  Children: `PROGRAM.83.1`, `PROGRAM.83.2`
  Verification: both children done; archival identity/provenance results recorded below
  Commit: `ARCHOGEN-PROGRAM-0626 (leaf PROGRAM.83.2)`

- ID: `PROGRAM.83.2`
  Status: `done` — published seals verified and all active work retained
  Goal: use the published task/review seal commands for PROGRAM and M3 closed subtrees and the task-tree-sealing
  and generated-adequacy review histories; verify byte/provenance gates and commit the archival result.
  Scope: preserve the committed source bodies, count/hash/copy verification and exact retained stubs; an open
  archival leaf stays live while the tool seals the older committed units. No rewriting of archive evidence.
  Verification: `2026-10-11` — PROGRAM: 15 closed subtrees / 17 leaves; M3: four subtrees / four leaves;
  both full-tree reconstructions read back byte for byte, exit code 0. The two review copies read back exactly
  98 522 and 9 565 bytes; original paths retain proven stubs. Final staged task/history, review/history, count,
  size, book and doctrine gates before commit. No compiler or new test-suite result claimed.
  The PROGRAM.70 parent retains the tool's explicit `not recorded` commit warning; its prior committed body is
  archived unchanged, while its completed children retain their own named commits. No history silently repaired.
  Diff-check scope: the tool-preserved PROGRAM.80 and PROGRAM.81 bodies end with source blank lines, so Git's
  ordinary whitespace check reports their new archived EOF blank lines (exit code 2). They remain byte-identical;
  exclude exactly those two frozen copies for the remaining diff check, not their stubs, indices or other changes.
  Staged provenance gates pass: 194 sealed task files / 356 stubs and 15 archived review histories.
  Commit: `ARCHOGEN-PROGRAM-0626 (leaf PROGRAM.83.2)`

