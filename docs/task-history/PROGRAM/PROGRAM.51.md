- ID: `PROGRAM.51`
  Status: `done` — filed and closed `2026-10-03` from `M2.22`'s root cause
  Goal: `COMMIT.md` step 2's format step enforced at commit time, a project doctrine `RUST-FORMAT`: every Rust source
  a commit stages is in canonical format, read from the index. `ARCHOGEN-M2-0394 (leaf M2.10.2)` committed two
  unformatted files after running the tests and clippy alone, and the `focused` tier's `fmt` step failed at HEAD
  until `M2.22`; nothing at commit time held anyone to the step, and the format step is the one of that tier cheap
  enough for the pre-commit path.
  Acceptance: the gate refuses an unformatted staged file by name and passes a canonical one; judges the staged
  bytes, not the working tree's; follows no `mod`; refuses a file `rustfmt` cannot read; judges every tracked file
  when nothing is staged; leaves `vendor/` out; an empty population a breach; its RED arms run by
  `scripts/run_self_tests.sh`; registered, mirrored in `DOCTRINE_ENFORCEMENT.md`, the toolchain fact it rests on in
  the ledger.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `ARCHOGEN-M2-0394`'s `compose.rs` through `rustfmt --emit stdout` and `cmp` against
    itself → `cmp rc=1`, differing; yet that commit passed the pre-commit hook, whose doctrines hold no format check
    (`grep -c fmt scripts/check_doctrines.project.sh` → 0 before this leaf).
  - [x] **ROOT CAUSE (WHY + WHERE)** — `printf 'fn  main( ){}' | rustfmt --edition 2021 --check; echo rc=$?` →
    its diff, then `rc=0`, on rustc 1.95.0's rustfmt; and no doctrine ran a format check before this leaf.
    WHERE: `COMMIT.md` step 2 asks for `make focused`, whose first step is `fmt`, and nothing at commit time runs it. WHY a staged-file check and not `cargo fmt --check`: the latter reads the
    working tree, so a fix left unstaged passes over the bytes committed, and it follows `mod` declarations; on
    stdin, `rustfmt --check` prints its diff and exits 0 (`printf 'fn  main( ){}' | rustfmt --edition 2021
    --check; echo $?` → `0`), so the gate compares `--emit stdout` with its input.
  - [x] **FIX** — `scripts/check_rust_format.sh`, `RUST-FORMAT`, registered in `scripts/check_doctrines.project.sh`,
    mirrored in `DOCTRINE_ENFORCEMENT.md`, named in `COMMIT.md` step 2; the `rustfmt` fact in the ledger's
    `rust-toolchain` entry → `grep -c RUST-FORMAT` over the four files → 1 each.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_rust_format.sh --self-test` → `10 pass / 0 fail (10 arms)`:
    a canonical file passes; an unformatted one refused by name; the staged bytes judged over a working-tree fix,
    and the reverse; no `mod` followed; an unreadable file refused; `vendor/` outside; a commit staging no Rust
    passes; nothing staged judges every tracked file; an empty population a breach.
  - [x] **NO REGRESSION** — `bash scripts/check_rust_format.sh` on the tree → `159 tracked Rust source file(s), each
    in canonical format`, in about 5 s; `bash scripts/run_self_tests.sh` → `self-tests: OK — 44 self-test(s)
    passed`; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md`, `COMMIT.md`, the ledger; this leaf and both logs; `CHANGELOG.md`
    → one entry.
  Verification: `2026-10-03` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0421 (leaf PROGRAM.51)`
