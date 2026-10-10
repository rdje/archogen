# Annex B: The checks that keep the repository honest

[Verifying the toolchain](verification.md) explains how archogen's code is tested. This annex is about the
repository itself — its book, its records, its task trees — and the checks that stop them drifting from the code or
from the truth. Each is a script under `scripts/`, run before every commit by the doctrine enforcer
(`scripts/check_doctrines.sh`) and again in CI, and each has its own RED arms proving it can refuse; the full
register is `DOCTRINE_ENFORCEMENT.md`. The sections below are each check's account, moved here from the chapter so
that the chapter can stay readable; the move changed nothing in them but two links to the
[ledger](ledger.md) the source-ledger gate asked for.

## Every crate is in this book

`BOOK-ANCHORS` checks that what a chapter cites exists; `BOOK-COVERAGE` checks the other direction — that every
crate of the workspace appears in some chapter **beside a path into it**, so no capability exists that this
book never shows you. The crates are read from `Cargo.toml` rather than listed, so a new one is covered the day
it is added. A name alone does not count: a list of crate names would pass a name-only rule and tell you
nothing about where anything lives. When it was written, the scheduling checker — the `rt-analysis` crate —
was in no chapter at all; [What the scheduling checker establishes](analysis.md) now says which file holds what.

```console
$ bash scripts/check_book_coverage.sh
```

## Other repositories are read-only

`REPOSITORY-BOUNDARY` runs with the other doctrines on every commit. It checks the one place this
repository can see another: each vendored checkout it pins — today `vendor/linkedspec` — must be **at
its pin**, with **no commit made there**, **no file changed** and **no file created**. Reading a vendor,
building it by its documented route and moving our own pin to a commit it has published are all fine;
writing into it is not, and neither is accepting a change another project's agent wrote into this one
without a task-tree leaf that records who authorized it (`CLAUDE.md`, and
`docs/decisions/decision_repository-boundary-read-only.md` for the one time it happened).

```console
$ bash scripts/check_repository_boundary.sh              # the gate
$ bash scripts/check_repository_boundary.sh --self-test  # its RED arms, on scratch repositories
```

⚠️ It does not look inside the checkouts *nested* in a vendor. The vendor's own published bootstrap
moves and dirties those — thousands of entries, measured — and running a documented build is
consumption, so a gate that counted them would refuse every commit for doing what the rules allow.

## Scratch stays on this volume

Every temporary file this repository's scripts create lives under its own `target/`, never in the
system's temporary directory. `SCRATCH-LOCALITY` reads every tracked shell script and Rust source and
refuses a `mktemp` with no template under `target/`, a line naming the system temporary directory, and a
Rust `temp_dir()` call. When it was written, one ordinary run of the doctrines and their self-tests made
**26** temporary directories and files, **every one** of them off this volume, from 18 places in 15
files. Four of those places are in files this project takes from its scaffold. A local edit there would
be erased by the next sync, so the check lists them without refusing them, and the change they need is
written down for the scaffold's owner in `docs/decisions/decision_scratch-on-the-repository-volume.md`.

Moving a fixture changes what surrounds it. Two self-tests broke when their scratch moved under
`target/`: one because git ignores that whole directory, and one because [Cargo](ledger.md#rust-toolchain), walking up from the
fixture, now found this workspace. The second is why the root `Cargo.toml` excludes `target`. For the same reason
the CI rehearsal checks out beside this repository rather than under it, the one exception the director ruled, and
the repository's own data store is a git-ignored `.archogen-data/`
(`docs/decisions/decision_ci-rehearsal-beside-the-repository.md`).

```console
$ bash scripts/check_scratch_locality.sh              # the gate
$ bash scripts/check_scratch_locality.sh --self-test  # its RED arms, on scratch repositories
```

## A commit holds what was run

Every check before a commit reads the working tree, and a commit records the index. A new file left untracked is
in the first and not the second, so a commit can record a tree nobody ran: one did, a module declared and its file
not committed, so the committed tree did not build. `UNTRACKED-CODE` refuses a commit while a file in a code path —
what `.doctrine/code_paths.txt` says is code — is untracked; an ignored file, or a note outside every code path,
passes. A leaf's evidence can be stale the same way: a box that cites a passing `make focused` run, made before the
last file changed. So the focused tier, when it passes on a tree that did not move during the run, writes a stamp
naming that tree, and the same check refuses a staged leaf that cites the run unless the staged tree has its stamp
(`PROGRAM.53`).

```console
$ bash scripts/check_untracked_code.sh              # the gate
$ bash scripts/check_untracked_code.sh --self-test  # its RED arms, on scratch repositories
```

## What comes from outside is written down

`SOURCE-LEDGER` keeps [What this project relies on from outside](ledger.md) true to the repository.
Every version this repository pins must appear in that chapter at the same version, and a chapter or
decision that names an outside source must link to its entry. When the check was written, no chapter
and no decision linked to anything outside, and the pins it found showed that the Rust compiler,
[mdBook](ledger.md#mdbook) and the CI's actions are not pinned to exact versions at all (`PROGRAM.30`).

```console
$ bash scripts/check_source_ledger.sh              # the gate
$ bash scripts/check_source_ledger.sh --self-test  # its RED arms, on scratch repositories
```


Three of those pins were once moving targets: the Rust channel `stable`, the CI actions' `v4` tags and whatever
mdBook was installed. Each now names one release (leaf `PROGRAM.30`). `rust-toolchain.toml` says `1.95.0`, with
the components and the bare-metal target the tiers use, and CI installs exactly that file with
`rustup toolchain install`, so the version is written once. The actions are referenced by commit. The `book` step
builds only with the pinned mdBook, because every `ledger.md#…` link depends on how mdBook derives a heading's
anchor. The day the Rust pin was set, `1.98.0` passed the same format check, lints and test suite, so a bump
starts from evidence rather than from hope.

## What is versioned is written down

`VERSION-REGISTER` keeps [What is versioned, and what changing it costs](versions.md) true to the code.
Every version the code declares must be on that page at the same value: each format identifier, each
version constant, each profile id, and the engine version, which every crate must agree on. When the
check was written, it found one version that the census planning it had missed, the model the
scheduling analysis reasons in, because the census looked for version constants by name and the gate
looks at what the value is.

```console
$ bash scripts/check_version_register.sh              # the gate
$ bash scripts/check_version_register.sh --self-test  # its RED arms, on scratch repositories
```

## A bug report says what its issue says

The bugs this project reports to the tools it uses live in `docs/feedback/`, one register per vendor
and one directory per bug. `FEEDBACK-REGISTER` checks that each register row states what its bug's
own page states, that a `verified` bug was re-measured on the date the register gives, and that every
total is a recount. When it was written, the index above the registers still said [LinkedSpec](ledger.md#linkedspec) had five
open bugs and two blockers, three days after all five had been verified.

```console
$ bash scripts/check_feedback_register.sh              # the gate
$ bash scripts/check_feedback_register.sh --self-test  # its RED arms, on scratch repositories
```

## The status pages stay short

`LIVE_STATUS.md` and `docs/TASK_TREE.md` show where each part of the work stands now. They grew into
something else. Each finished task appended a note to its area's row, until `LIVE_STATUS.md` held
42,110 bytes in 21 lines, one row alone being 30,256 bytes long. What each task did already lives in
its tree's Commit Log and in `CHANGELOG.md`, so the rows were cut back to status, next task and any
blocker, after checking that every task they mentioned had its own record. `LIVE-SNAPSHOTS` now
bounds these pages, `MEMORY.md` and `README.md` on lines, bytes and longest line. The longest line is
counted separately because a page can keep a short line count while one row grows without limit. The
landing page's limits were measured from the page itself, after a review against `README_POLICY.md`,
rather than taken from the generous defaults the project's template started with.

```console
$ bash scripts/check_live_snapshots.sh              # the gate
$ bash scripts/check_live_snapshots.sh --self-test  # its RED arms, on scratch repositories
```

## The histories are sealed as they grow

`CHANGELOG.md` and `DEV_NOTES.md` gain an entry with nearly every commit. They are rolling ledgers: once one holds
twice its window of entries, its oldest window is moved, byte for byte, into the next numbered file under
`docs/history/`, and `docs/history/INDEX.md` lists each such file with its range, its size and a fingerprint.
Nothing is rewritten. Reading the live file and then the sealed files, newest first, gives the whole history
exactly as it was written (`docs/decisions/decision_history-ledgers.md`). `HISTORY-LEDGERS` checks on every commit
that no sealed file has changed since it was sealed, that the index lists every one and has lost or changed no row it ever held, that the order runs on
unbroken, and that neither live file has outgrown its window.

```console
$ bash scripts/check_history_ledgers.sh              # the gate
$ bash scripts/check_history_ledgers.sh --seal       # a rollover the gate asks for, with its proof
$ bash scripts/check_history_ledgers.sh --self-test  # its RED arms, on scratch repositories
```

## Finished work leaves the task trees

A task tree records every leaf of its work, with its checklist and evidence. Most of it is finished work: on
`2026-09-30`, 77% of `docs/tasks/M1.md` and 79% of `docs/tasks/PROGRAM.md`.
- **What moves.** Once every leaf of one of a tree's top-level parts is done, those leaves move, byte for byte, into
  one file under `docs/task-history/`. A top-level part can stay open for long — one of its leaves waiting on a
  decision — so, below an open one, each finished part moves the same way, the largest it can, into a file of its own;
  when the open part closes, the rest of it follows into its own file (`PROGRAM.69`).
- **What stays.** Each leaf leaves two lines in the tree, its name and a link to where its text now is. The live
  work, the list of what is next and the logs stay where they were.
- **The proof.** Before anything is written, the tool checks that putting every leaf back would give the tree as it
  was, byte for byte (`docs/decisions/decision_task-tree-sealing.md`). After writing, it runs the check below on the
  result, and undoes every write if the check refuses or an error stops it — each file written whole or not at all,
  and whatever it could not undo, as on a full disk, named. A request to stop waits until the tool is done, so the
  history is whole either way; only a process killed outright — by `SIGKILL` or one of the few fault signals left
  unheld — aborting or crashing leaves its writes, for the next check to name. So the history admits every line a tree may hold:
  its line ceiling is the trees' own (`docs/decisions/decision_task-history-line-ceiling.md`).
- **What it still carries.** A sealed leaf keeps the hand-offs it quoted; the check that each design's hand-off is
  quoted word for word by its leaf reads the sealed text, never the two-line placeholder (`PROGRAM.63`).
- **The first seal**, on `2026-09-30`: 112 leaves left `M1` and `PROGRAM`. At the sealing commit, `M1.md` went
  from 7 633 lines to 2 278, and `PROGRAM.md` from 4 280 to 978.

`TASK-HISTORY` checks on every commit, in CI as well as before a commit:
- that no sealed file has changed since the commit that sealed it;
- that `docs/task-history/INDEX.md` lists every sealed file and has lost or changed no row it ever held;
- that every sealed leaf has exactly one two-line placeholder in its tree, linking the file that holds it;
- that every sealed leaf is, byte for byte, what its tree held just before it was sealed, so a seal made by hand or
  edited is refused when its files, its placeholders' links or its parts differ from what the tool makes — each file
  the largest finished part, whole. A placeholder's commit, a row's date and the rule against a line at column 0 are
  the tool's alone, which the check does not read, and so are where a placeholder stands in its tree, any text after
  the name on its first line, a seal of only some finished parts, and prose in the index outside its tables;
- that no live leaf sits inside a part that is sealed, in any tree directly under `docs/tasks/` (one in a sub-folder
  is `PROGRAM.72`'s), and that each tree holds only its own leaves, each
  named once; that every sealed file holds a leaf; and that the index keeps one table per tree, its rows under it.

```console
$ bash scripts/check_task_history.sh                  # the gate
$ bash scripts/check_task_history.sh --seal M1        # seal what has closed in a tree, with its proof
$ bash scripts/check_task_history.sh --census HEAD    # done leaves under open top-level parts, in bytes
$ bash scripts/check_task_history.sh --self-test      # its RED arms, on scratch repositories
```

The decisions folder is kept the same way. A numbered section of a decision record that is settled, such as a
finding the director has ruled on, moves byte for byte into `docs/decision-history/`. Its heading stays in the
record, above one line linking the sealed file, so a citation of the section by its number still finds it.
`DECISION-HISTORY` checks what `TASK-HISTORY` checks, for sections: every sealed file against its row and the
commit that sealed it, the index append-only, one placeholder per sealed section, and every sealed section what its
record held just before. The first seal, on `2026-09-30`, took four settled items of the director's findings
register out of the decisions folder (`PROGRAM.41`).

Each tree also keeps a Commit Log, one row per commit that worked on it, and `COMMIT-LOG-ROWS`
(`scripts/check_commit_log_rows.sh`) holds every commit whose subject names a work unit to having its row —
the pending commit included, so a commit that forgot its row is refused before it lands rather than found by
hand a commit later, as two were on `2026-10-01` (`PROGRAM.46`). The rows missing when the gate landed are a
backlog listed in the script, which may only shrink.

```console
$ bash scripts/check_decision_history.sh                                   # the gate
$ bash scripts/check_decision_history.sh --seal <RECORD> <N>...            # seal settled sections, with the proof
$ bash scripts/check_decision_history.sh --self-test                       # its RED arms, on scratch repositories
```

A design's review history only grows while its review is open. Once the review closes, the whole file can move, byte for
byte, into `docs/review-history/`, leaving a short stub at its old path that links to it, so every citation of it still
resolves. `REVIEW-HISTORY` checks every archived file against its row and the history it came from, and holds the
review's row in `docs/reviews/INDEX.md` to `closed`, the history to what it was when it closed, and every later commit
at that path to the stub, so a round merged in afterwards cannot be lost. It reads the staged copies, so a file left out
of the commit is refused before it lands. Every closed history it accepts is archived (`PROGRAM.55`, `PROGRAM.70.1`); a
stub is no history, so the folder's file ceiling skips it, not its bytes (`PROGRAM.70.2`).

```console
$ bash scripts/check_review_history.sh                     # the gate
$ bash scripts/check_review_history.sh --seal <FILE>       # archive a closed review's history, with the proof
$ bash scripts/check_review_history.sh --self-test         # its RED arms, on scratch repositories
```

## Where the landing page sends things

A size limit on `README.md` does not remove the need to write things down; it moves it somewhere else. So
`README_POLICY.md` also asks that every place the README points a reader to, and every place its checks tell an
author to move detail to, is itself kept in bounds. `README-ROUTES` works that list out from the README's links and
from what its two checks actually print. It follows each place on to wherever that place's own check sends overflow,
and holds each one to the ceiling registered for it in the policy. The changelog and the development notes are
bounded as rolling ledgers (above). A design's review history, which grows by a round at a time, overflows from the
decisions folder to `docs/reviews/`, which has ceilings of its own, and a closed review's history overflows on to
`docs/review-history/` (above). The task trees are bounded too, and their
finished parts overflow to `docs/task-history/`, as settled sections of a decision record do to
`docs/decision-history/` (above). A folder grown too large is split by subject into sub-folders, each with ceilings
of its own, while the folder's own limits still count everything in them, so a split adds no room; and no ceiling
may rise above what a decision fixes, which the policy lists and the check enforces. The decisions folder was split
this way on `2026-09-30`, and `DECISION-INDEX` keeps every record in a sub-folder listed in its index.

```console
$ bash scripts/check_readme_routes.sh              # the gate
$ bash scripts/check_readme_routes.sh --self-test  # its RED arms, on scratch repositories
```

## "Next" means what the task tree says

A page that names the next task is repeating what that task's tree says, and a repeat goes stale the
moment the tree moves without it. It has happened here several times: a page naming a task already
finished, a list of what comes next naming the current task as its own successor, and a changelog whose
header says "newest first" opening with an entry seven commits old. `STATED-ORDER` compares every such
repeat with its source on every commit. On its first run it found one more: a task whose own status said
it was in progress while its tree's list said it was blocked.

```console
$ bash scripts/check_stated_order.sh              # the gate
$ bash scripts/check_stated_order.sh --self-test  # its RED arms, on scratch repositories
```

## A number added to this book says what keeps it true

The error this project has made most often is a number written into a page that later stopped being
true: a count of descriptions over a directory that had grown, a list introduced as `Three rules` above
four. `FIGURE-REGISTER` stops a commit from adding such a number to a live page without saying what keeps
it true. The number must either be compared with a measurement by a named test that reads the page, carry
the date it was measured on the same line, or be marked as not a count (`docs/figures.md`). Numbers
already on these pages are counted and reported, and a commit may only lower their count in a page it
touches. Its first run refused a claim written while the check was being built, which named a test that
never reads the page.

```console
$ bash scripts/check_figure_register.sh              # the gate
$ bash scripts/check_figure_register.sh --self-test  # its RED arms, on scratch repositories
```

## Every diagnostic shown in this book is a real run

When this book shows `archogen check` refusing a description, a test re-runs that command and requires
the book to show exactly what it prints: every label, the final summary line and the exit code, with
nothing shortened or re-wrapped (`crates/archogen-cli/tests/book_transcripts.rs`). Before that test,
five of the twelve checkable examples differed from a real run. Two had silently dropped the "first
declared here" label that the text around them said was there. All five are re-rendered from runs.
Eleven older examples use a file that is not in the repository, so no one can re-run them. They are
listed in the test, and the list may shrink but never grow. Each is listed by what it shows, its first
error and its first location, and not by the line it sits on. The first version used line numbers, and a
paragraph added above one of them made the test fail for an example that had not changed (leaf
`PROGRAM.33`).
