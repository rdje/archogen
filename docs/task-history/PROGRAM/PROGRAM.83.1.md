- ID: `PROGRAM.83.1`
  Status: `done` — closed-row grammar and archival ownership prepared
  Goal: prepare the archival activity and normalize the adequacy review's closed-row grammar to the archive tool's
  published contract, then commit that state before any archive is written.
  Scope: task ownership, closed-row metadata, resume/frontier pointers and trailing whitespace in that index;
  no frozen review body, sealed task body or implementation changes.
  Reproduce / impact: the review archive tool refuses the adequacy history because its row begins `closed ` rather
  than `closed: `; exit code 1 explicitly reports nothing written. The source history stays untouched.
  Fix: normalize only its status prefix and trim a trailing blank index line. Prior closure date/result retained.
  Verification: `2026-10-11` — published refusal reproduces; metadata prefix, unchanged source SHA-256, real staged
  review/count and book gates pass before commit. No Rust source changed.
  Commit: `ARCHOGEN-PROGRAM-0625 (leaf PROGRAM.83.1)`
