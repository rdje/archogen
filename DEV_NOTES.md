# DEV_NOTES.md

## _(2026-09-04)_ — a template's trial must include the first commit

- Every gate was green on the generated project and the first commit still failed: the doctrines judge STAGED
  code, and nothing had been staged until the user tried. Trial the path a user walks, to its end.
- `grep -c` prints `0` and exits 1. `$(grep -c … || echo 0)` therefore yields `0⏎0` — a second line — which
  here started a flush-left line inside a checklist bullet and hid its evidence from the box-scoped extractor.
  Capture the count, then default the empty case; never append a fallback to grep's own output.

## _(2026-09-04)_ — a green gate that judges nothing is the class a template must not ship

- Two of the four doctrine ports in `.2.6` were wrong on first run and their own RED self-test arms said so:
  a `python3 - <<'PY'` detector whose stdin was the heredoc (every arm read 0 rows), and a `grep -c … | grep -qx 0`
  control under `pipefail` (`grep -c` prints 0 and exits 1). A self-test with only GREEN arms would have passed both.
- The neutrality bar is measured, not felt: `grep -ciE 'grammar|parser|…'` over each ported script → 0, after the
  generic uses of "corpus" and "grammar" were re-worded ("tree", "syntax") so the count means what it says.

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

## _(2026-09-13)_ — a commit carries one owning leaf

- `TASK-ACCEPTANCE` requires a complete checklist from EVERY staged `docs/tasks/*.md`, not
  from the leaf that owns the staged code. Propagating a blocker into a second tree alongside
  code is therefore refused, with three "no 'ROOT CAUSE' box" lines for a tree that landed no
  code. Unstaging that one file and changing nothing else → `=== all doctrines green ===`.
- It is the conservative closure of a measured hole (a co-staged tree supplying another leaf's
  evidence), so the answer is a convention, not an edit to the check: split the commit, same
  work-unit id on both. Tracked as `PROGRAM.8` with routing evidence.
- Promoted to `docs/knowledge/cross-tree-lockstep-and-commit-scope.md`.

## _(2026-09-13)_ — a portable gate that misfires is a seam question, not a fork question

- `TASK-ACCEPTANCE` blocked this project's first real commit with thirty refusal lines,
  because its neutral `(^|/)src/` arm matches `docs/book/src/introduction.md` — an mdBook
  page, not Rust. Measured, not guessed:
  `git diff --cached --name-only | grep -E '(^|/)(crates|src|scripts)/|\.(rs|sh)$'` →
  `docs/book/src/introduction.md`.
- The fix was `.doctrine/code_paths.txt`, the seam `.doctrine/README.md` already documents —
  not an edit to the check. Rust sources stay covered by the `\.rs$` arm wherever they live,
  so dropping the bare `src/` arm narrows nothing real.
- Promoted to `docs/knowledge/doctrine-seams-vs-forking-a-check.md`.

## _(2026-09-13)_ — bootstrap

Repo created from the `bedrock` template: durable 4-layer memory, task-tree tracking, the
strict commit workflow, and the mechanical doctrine enforcer are in place and enforced by
git hooks + CI. `ROADMAP.md` revision 2.0 adopted and seeded into ten task-trees. No engine
code yet.

- Promoted: nothing to promote; this entry records a state, not a lesson.
