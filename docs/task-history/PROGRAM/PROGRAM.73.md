- ID: `PROGRAM.73`
  Status: `done` — filed `2026-10-10` by `PROGRAM.69`'s review round 9, its P1; done `2026-10-28`
  Goal: the history gate names every failure it meets as a breach, never a traceback.
  Reproduce / issue: `scripts/check_task_history.sh`'s top level catches `Unreadable` and `OSError` alone, so any other
  exception in the gate's own run ends in a traceback, exit 1 — git printing bytes that are not UTF-8 for
  `rev-parse --is-shallow-repository`, a `UnicodeDecodeError` at `shallow.decode()`; the round's probe, a fake `git` on
  the path, its own untracked runner and so not durable; the self-test's arm for a seal stopped that way (R9-2) expects
  the traceback today. Present at `fea69ad`.
  Direction: the top level names any exception as a breach, and that arm expects the named breach; an arm for the gate.
  Built: the gate's top level and the census each catch any other exception and name it, `the gate stopped on TYPE:
    message`, exit 1; a seal stopped so has already rolled back and said so. The seal's arm for git output that is not
    UTF-8 now requires the named breach and no traceback, and a new arm requires the same of a plain run.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `HEAD`'s gate with a fake `git` printing `\377` for `--is-shallow-repository`, a plain run:
    `old rc=1`, `grep -c '^Traceback'` → `1`, ending `UnicodeDecodeError: 'utf-8' codec can't decode byte 0xff`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — the top level caught two exception types alone:
    `git show HEAD:scripts/check_task_history.sh | grep -c 'except (Unreadable, OSError) as e:'` → `2`, and nothing after them.
  - [x] **FIX** — `except Exception` beside each, naming the type and message; the seal arm's condition and comment;
    the plain-run arm.
  - [x] **ADDRESSED (verified)** — the patched gate under the same fake `git`: `new rc=1`, `grep -c '^Traceback'` → `0`,
    `TASK-HISTORY: the gate stopped on UnicodeDecodeError: …`; `bash scripts/check_task_history.sh` → `exit 0` on the
    real tree; `bash scripts/check_task_history.sh --self-test` → `172 pass / 0 fail (172 arms)`.
  - [x] **NO REGRESSION** — no Rust source changed; `bash scripts/run_self_tests.sh` →
    `exit 0`, `self-tests: OK — 58 self-test(s) passed`; `bash scripts/check_doctrines.sh` → `exit 0`.
  - [x] **LOCKSTEP** — the gate's comments, this leaf and the Commit Log; no book page describes the traceback.
  Commit: `ARCHOGEN-PROGRAM-1039 (leaf PROGRAM.73)`
