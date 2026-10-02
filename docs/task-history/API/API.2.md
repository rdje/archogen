- ID: `API.2`
  Status: `done`
  Goal: state and gate the invariant that makes handing archogen to an arbitrary agent safe — **no
  product code spawns a subprocess or executes anything.**
  Reproduce / issue: measured `2026-09-28`. `git grep -niE 'spawns? no|no subprocess|does not
  execute|never executes|no child process'` over tracked files returns **nothing**, so the property is
  written down nowhere. It holds today: `Command::new` appears in no production half in the workspace,
  the only `std::process` use in `crates/archogen-cli/src` is `use std::process::ExitCode;`, and the one
  place that compiles a generated artifact is `crates/archogen-cli/tests/s0_oracle.rs:609` — a test.
  census: `for f in $(git ls-files 'crates/*/src/*.rs'); do awk '/#\[cfg\(test\)\]/{exit} {print
  FILENAME":"NR": "$0}' "$f"; done | grep -E 'Command::new|std::process'` → one hit, `ExitCode`.
  Acceptance: the invariant is stated in a durable record and in the book; a gate fails if a production
  half anywhere in `crates/*/src` spawns a subprocess, with the test half excluded by construction and
  the exclusion itself armed; RED arms proving the gate fires on a real spawn and does **not** fire on
  `ExitCode`, on a test-half `Command::new`, or on the `xtask` and `scripts/` surfaces that legitimately
  drive a toolchain; `make focused` exit `0`.
  Priority: **medium-high** — cheap, unblocked by the freeze, and it is the property §10.4's safety
  argument rests on. A `Command::new` added to a product crate tomorrow passes every gate in the tree
  today.
  Verification: see the checklist — the gate green on 44 production files, nine arms, two mutations; the same
  production-half rule adopted by `wasm_build.sh`, with its own arm.
  Commit: `ARCHOGEN-API-0158 (leaf API.2)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the property held and nothing held it:
    ```text
    $ git show HEAD:scripts/check_doctrines.project.sh | grep -c "NO-SUBPROCESS"   → 0
    $ git grep -niE 'spawns? no|no subprocess|does not execute|never executes|no child process' HEAD
      → only CHANGELOG.md and the decision record describing its absence — no statement of it, and no gate
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — §10.4's safety argument was reasoned from a measurement
    (`decision_programmatic-interface.md`, `2026-09-28`) and never turned into a check, so it lived at the level of
    "true today". **WHERE the rule's first shape would have leaked:** "production code ends at the first
    `#[cfg(test)]`" — the rule `API.1`'s `wasm_build.sh` shipped with — lets one attribute on an early helper silence
    every line below it (`git show HEAD:scripts/wasm_build.sh | grep -n 'cfg\\(test'` →
    `43:    hit="$(awk '/^[[:space:]]*#\[cfg\(test\)\]/ { exit } …`, an exit at the first match).
  - [x] **FIX** — `scripts/check_no_subprocess.sh` (**`NO-SUBPROCESS`**, registered in the project slot): every
    tracked `crates/*/src/**/*.rs`, derived; the test half starts only at a `#[cfg(test)]` opening a `mod`; refuses
    `Command::new`, `process::Command` (imports too), `.spawn(`, `exec`, `fork`, and passes `ExitCode` and
    `process::exit`. The invariant stated in the decision record and in `verification.md`. `wasm_build.sh` adopts
    the same production-half rule.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/check_no_subprocess.sh → no-subprocess: OK (44 production source file(s) under crates/*/src; …)
      (44 = git ls-files 'crates/*/src/*.rs' | wc -l = find crates/*/src -name '*.rs' | wc -l)
    $ bash scripts/check_no_subprocess.sh --self-test → no-subprocess self-test: 9 pass / 0 fail (9 arms)
      a real spawn → refused with its line · an import of Command → refused · ExitCode, process::exit → pass
      a spawn in the test module → pass · cfg(test) on a lone fn → the spawn below it refused
      tests/ and xtask/ → outside the population · .spawn( → refused · no product source → a breach
    M1 the naive rule (the first cfg(test) ends production) → 1 refused: "a cfg(test) on a lone function…", restored
    M2 .spawn( dropped from the pattern → 1 refused, restored
    $ bash scripts/wasm_build.sh --self-test → wasm-build self-test: 6 pass / 0 fail (6 arms), the new arm among them
    ```
  - [x] **NO REGRESSION** — `bash scripts/check_doctrines.project.sh` → `no-subprocess: OK …`, slot rc=0; the
    `wasm_build.sh --list` set unchanged (the five crates, the four exclusions with the same lines).
  - [x] **LOCKSTEP** — the decision record, `DOCTRINE_ENFORCEMENT.md`, `verification.md` "The product runs nothing".
