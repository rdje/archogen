---
slug: a-restored-file-keeps-its-age
answers:
  - "I reverted the source to prove a test fails, put it back, and the build still behaves like the reverted code — why?"
  - "How do I restore files after a RED run so the next cargo build compiles what is on disk?"
type: knowledge
date: 2026-11-15
---

# A restored file keeps its age: put sources back with a new modification time, or cargo builds the reverted code

## The question

To show a test fails first, a leaf reverts its source files to `HEAD`, runs the test, and copies its saved files
back. The tree is byte for byte what it was. Does the next `cargo` build compile it?

## The answer

**Not when the copy keeps the saved file's modification time.** Cargo decides whether a local crate is fresh by
comparing timestamps: what it built from the reverted files is newer than a file put back with `cp -p`, so it reads
the crate as up to date and reuses the build of the reverted code.

It happened here in leaf `M4.1.7.5.4.4`, on `2026-11-15`. The RED run reverted the checker's sources
(`crates/archogen-plan-check/src/`) and the front's, then put them back with `cp -p`. The next RED run reverted only
`main.rs`, written fresh. It did not fail at run time as it should have: its test failed to compile,
`error[E0432]: unresolved import archogen_plan_check::target::digest`. The library's `target.rs` on disk held that
function. The build in use was still the reverted one. After `touch` on the restored files, the same command built
the library again and failed the way it should, at run time: four of the five tests in
`tests/executable_on_target.rs`, on `` `--target` is no option of this checker``.

Both runs read the same bytes on disk. They differed only in the files' times, and the outcome changed. That is
measured here. Cargo's own account of its timestamp check, the module documentation of its `fingerprint` module, was
not reachable from this host when this was written, so this card does not quote it.

It can go the other way. Had a GREEN run come next instead, it would have tested the reverted code while the tree held
the fixed code. A failure there would be false. A pass would be false too, if the test could not see the revert.

## How to apply

- **Put sources back with a plain `cp`** (a new modification time), or `touch` every restored file before the next
  build.
- **Never `cp -p`, `rsync -t`, or `tar -x` that keeps times, to undo a revert in a cargo tree.**
- **After a revert-and-restore, `cmp` each file with its saved copy and then build once more.** A result that does not
  match what is on disk means the build is stale. It does not prove the code is wrong.
- **`git stash` and `git stash pop` write each file they restore anew**, since git keeps no modification times, so
  they leave no such hazard. It comes from copying a file back by hand, keeping its time.
