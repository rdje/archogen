- ID: `PROGRAM.105`
  Status: `done` — P2, found `2026-10-22` by `M3.5.1`'s review (remark 10), re-measured: `NO-SUBPROCESS` misses ordinary
    Rust that starts a process
  Reproduce / issue: run through `scripts/check_no_subprocess.sh`'s own `SPAWN` pattern, each of these passes: `use
    std::{process::{Command as C}};` with `C::new("x").output()`; `extern "C" { fn execve(…) -> i32; }`; a call to
    `posix_spawn(…)`; a call to `system(…)`. The gate reads lines for names, so an alias, a nested `use` group, a
    `use` split over lines, a libc function declared in an `extern` block, or a system call through inline assembly
    escapes it. Impact: §10.4's safety argument — the product executes nothing — rests on a gate that a one-line alias
    defeats; nothing in the tree does it today, which `M3.5.1.4`'s one admitted spawn site must not change silently.
  Goal: the three routes a zero-dependency workspace has to a process closed structurally — `std::process` admitted
    only as `ExitCode` and `exit`, however imported; no `extern` block and no `#[link]`; no `asm!`, `global_asm!` or
    `naked_asm!` — over production code with comments and string literals stripped and each `use` read whole; an
    `extern "C" fn` definition, the wasm module's export, still admitted; arms for each.
  Acceptance:
    - [x] ROOT CAUSE — the gate's `SPAWN` pattern at `b3043190` (`git show HEAD:scripts/check_no_subprocess.sh`), applied
      line by line: `use std::{process::{Command as C}};`, `C::new("x").output()`, `extern "C" { fn execve(…) }`,
      `posix_spawn(…)` and `system(…)` each passed it, `Command::new("x")` alone refused. It matched names on single
      lines, so an alias, a split `use`, a foreign declaration and assembly were outside what it could see.
    - [x] FIX — `scripts/check_no_subprocess.sh`: `stripped`, a light lexer removing comments — nested block comments
      among them — and string, raw-string and character literals, a lifetime told from a character by its byte length,
      a file ending inside a literal refused; `joined`, each `use` onto its first line; `process_paths`, `std::process`
      admitted as `ExitCode` and `exit` alone, never aliased or glob-imported; `FOREIGN`, an `extern` block or `#[link]`;
      `ASSEMBLY`, `asm!`, `global_asm!`, `naked_asm!`; thirteen arms. The doctrine row, the driver's line, the
      interface record and the book's verification chapter say what it closes.
    - [x] ADDRESSED — `bash scripts/check_no_subprocess.sh --self-test`: `no-subprocess self-test: 22 pass / 0 fail (22
      arms)`. Failing first, each new rule disabled in a copy (`target/p105/variant.sh`): process paths unchecked `18
      pass / 4 fail`; foreign functions `19 pass / 3 fail`; assembly `21 pass / 1 fail`; nothing stripped `17 pass / 5
      fail`, the real tree among them; uses not joined `21 pass / 1 fail`; a character read as a lifetime `20 pass / 2
      fail`; every quote a character `21 pass / 1 fail`; an unterminated file accepted `21 pass / 1 fail`.
    - [x] NO REGRESSION — `bash scripts/check_no_subprocess.sh` on the real tree: `no-subprocess: OK (89 production source
      file(s) …)`, every file lexed to a clean end and the code it keeps checked by eye on `main.rs`, the wasm exports
      and the catalog's assembly reader; `bash -n scripts/check_no_subprocess.sh`: clean; `bash scripts/run_self_tests.sh`:
      51 of 52 passing, the one the stated-order gate's real-tree arm catching this leaf's own unmoved snapshots, then
      `stated-order self-test: 10 pass / 0 fail (10 arms)` once they moved.
    - [x] LOCKSTEP — `DOCTRINE_ENFORCEMENT.md`'s row, rewritten within its cap (36 842 → 36 838 bytes); the driver's
      line; `decision_programmatic-interface.md`; the book's "The product runs nothing".
  Verification: the four evasions re-measured; the self-test armed rule by rule; every gate's self-test.
  Commit: `ARCHOGEN-PROGRAM-0878 (leaf PROGRAM.105)`
