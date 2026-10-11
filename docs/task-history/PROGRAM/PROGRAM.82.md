- ID: `PROGRAM.82`
  Status: `done` — filed, repaired and verified `2026-10-11`; the registry description is literal text
  Goal: initialize the project doctrine registry without executing its explanatory text or truncating a description.
  Reproduce / issue: the cleanup's commit hook emitted `syntax error near unexpected token` and a command
  substitution containing `| Round |` from `scripts/check_doctrines.project.sh`, yet every individual project gate
  otherwise ran. `bash -n scripts/check_doctrines.project.sh` exits 0: that syntax check does not execute the array.
  Root location: the REVIEW-COUNTS entry is double-quoted and contains an unescaped backtick-wrapped table header;
  Bash tries to execute the header. Escaping only the backticks would leave pipe characters inside a field whose
  delimiter is itself a pipe; the repair must remove both syntaxes from this description.
  Direction: make this explanatory registry entry inert and delimiter-free; independently execute just the registry
  initialization before/after, require empty diagnostics and the full description, then run the ordinary doctrines.
  The project wrapper is editable; the held native doctrine driver remains untouched. No Rust source change.
  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — Bash initialization of the exact `PROJECT_DOCTRINES` array, extracted by Python:
    `target/p82/before.json` → exit 0 with command-substitution syntax errors and the header removed from the
    REVIEW-COUNTS description; 29 entries. The full commit hook independently emitted the same error.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_doctrines.project.sh`'s double-quoted REVIEW-COUNTS entry
    treats its explanatory backticks as shell code. The extracted-array Bash subprocess → `syntax error near
    unexpected token`, while `bash -n scripts/check_doctrines.project.sh` → exit 0: initialization executes a
    substitution that parse-only validation never runs. The field's resulting `(a  header)` proves text was lost.
  - [x] **FIX** — use words for the header example, removing command-substitution syntax and embedded field
    delimiters; document that registry descriptions are literal data. No check command or verdict rule changes.
  - [x] **ADDRESSED (verified)** — the same exact-array subprocess after repair, `target/p82/after.json` → exit 0,
    empty stderr, 29 entries, each exactly three fields; the REVIEW-COUNTS description contains `a header naming
    its rounds`. `sed -n '/^PROJECT_DOCTRINES=(/,/^)/p' scripts/check_doctrines.project.sh | bash` reproduces the
    initialization directly. The JSON probe outputs are untracked, so no new regression gate is claimed.
  - [x] **NO REGRESSION** — `bash -n scripts/check_doctrines.project.sh` → exit 0; the exact-array probe verifies all
    29 descriptors still have three fields and their script paths. `git diff --exit-code HEAD --
    scripts/check_doctrines.sh .githooks/pre-commit scripts/selftest_spine.sh` → exit 0: held drivers untouched.
    The ordinary doctrine suite runs on the final staged tree in the commit hook. No Rust changed.
  - [x] **LOCKSTEP** — project registry comment and description, book annex, this leaf and both logs, changelog;
    resume/frontier snapshots now name the next repair. README, development notes and product milestones unchanged.
  Verification: `2026-10-11` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0611 (leaf PROGRAM.82)`
