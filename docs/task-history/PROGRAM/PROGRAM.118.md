- ID: `PROGRAM.118`
  Status: `done` — P3, filed `2026-10-27` by `PROGRAM.117.1`'s third review round, fixed the same day
  Goal: `README-ROUTES`' on-disk measures read every tracked path whole: a NUL-separated `git ls-files -z`, never its
    quoted output or a word-split list.
  Reproduce / issue: `git ls-files` quotes a path holding a tab, a quote or a non-ASCII byte, so `wc` fails on it and
    the file drops out of its row's measures; `longest $own` splits a path holding a space, so awk cannot open it and
    the row's longest line reads 0. The guard's total inherits it; the step, measured from the index, does not.
  Built: the directory branch and `--report` read `git ls-files -z` into an array; `longest` takes the row's files as
    a quoted array; a tracked path holding a newline is refused, named by `%q`; the blob-size sums' book filter takes
    the leading quote `git` writes before an unusual name.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the loops read `git ls-files`' quoted, newline-separated output and passed the row's files to awk word-split; WHERE, `git grep -n "longest \$own" HEAD -- scripts/check_readme_routes.sh`: the unquoted call, measured by the round's reviewer as awk's "can't open file docs/tasks/a".
    - [x] FIX — the NUL-separated reads, the quoted array, the newline refusal, the quote-tolerant filters.
    - [x] ADDRESSED — `bash scripts/check_readme_routes.sh --self-test`: `readme-routes self-test: 116 pass / 0 fail (116 arms)`, three new: a name with a space measured (`250 bytes on its longest line, over its ceiling of 200`), one past ASCII (`2100 bytes in its largest file`), one with a newline refused.
    - [x] NO REGRESSION — `bash -n scripts/check_readme_routes.sh` exit 0; `bash scripts/check_readme_routes.sh`: `readme-routes: OK (25 destination(s) governed)`; `bash scripts/check_doctrines.sh` passes; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 376 s
    - [x] LOCKSTEP — the check alone; its header's honest limits unchanged, the defect being no limit but a fault.
  Commit: `ARCHOGEN-PROGRAM-1028 (leaf PROGRAM.118)`
