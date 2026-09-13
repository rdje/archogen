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

## _(2026-09-13)_ — prose sitting beside enforced data goes unenforced

- `Profile` carries `exclusions` (read by the admission pass) and `decisions` (thirteen rows of
  concern/decision). Both are `&'static [..]`, both look equally authoritative, both are
  published on the same page. `git grep -n '\.decisions' -- crates/` → **one** hit, and it is the
  drift test comparing the table to the documentation. So the table tracked the docs perfectly
  and meant nothing to the checker.
- ⭐ The trap is that the row had a guard that made it look guarded. A prose-to-prose drift test
  reads like enforcement. Consequence: `archogen check` accepted two tasks at priority 1 —
  `exit=0` — under a profile that admits unique priorities only.
- Fixed for the Workload row (`crates/eadl-model/src/workload.rs`, `0 → 12` on the same input).
  The other twelve rows are now **counted** by a test that fails if the number moves, with the
  classifying leaf named in the assertion message. Counting is not enforcing, and it is what
  stops the gap being invisible.
- Verdict choice matters as much as detection: duplicate priorities and `D > T` are
  `unsupported-profile`, not `invalid-description`. Neither description is wrong about anything;
  both describe systems a different profile could analyze, and §3.1 requires the refusal to name
  what admitting them would cost rather than silently weakening the guarantee.
- Promoted to `docs/knowledge/prose-beside-data-goes-unenforced.md`.

## _(2026-09-13)_ — a leaf is closed by its acceptance, not by its implementation existing

- `S0.2` described a reader `M1` had already built better. Three tempting answers — delete the
  leaf, tick it because the work exists, write a second reader — each lose something: the
  acceptance, the measurement, and the point of the retirement clause respectively.
- Re-verifying the acceptance clause by clause found the one nobody had checked. "The three
  fixtures parse" was already implied; "a malformed fixture reports a **span-localized** error"
  was asserted nowhere for this corpus:
  `git grep -c 'read-unexpected-close\|read-unclosed-list' HEAD -- 'crates/archogen-cli'` printed
  nothing, `rc=1`.
- The clause is only meaningful with a negative case. A reader reporting every error at
  end-of-input passes "an error was produced". So the test injects the corruption near the TOP
  of the fixture and demands the caret there, with the line computed from the fixture rather than
  pinned — otherwise the test becomes a maintenance tax the next fixture edit pays.
- Red arms run against the **subject**: `read-unexpected-close` pointed at offset zero →
  `left: 1 / right: 25`; the `read-unclosed-list` secondary label moved to EOF →
  `left: 40 / right: 32`. Restored from a copy and confirmed byte-identical with
  `git diff --stat HEAD -- crates/eadl-front crates/eadl-model` (empty).
- Promoted to `docs/knowledge/closing-a-leaf-whose-work-landed-elsewhere.md`.

## _(2026-09-13)_ — an oracle is independent by construction, or it is not independent

- F28 asks for an expected-output assertion written *before* the generator. Intent cannot carry
  that: once the generator exists, the cheapest "expectation" is its own output pasted back, and
  the resulting test is a transcript that passes forever, including on every future version that
  is wrong in the same way.
- Three construction choices, none of which rely on anyone remembering. The fixtures land in
  `S0.1` and the emitter in `S0.3`, so the ordering is `git log`, not a comment. The oracle lives
  in `crates/archogen-cli/tests/`, which Rust cannot link into a library — no emitter can call it.
  And each expectation has two legs: frozen literal bytes, plus a re-derivation from the
  description through an implementation of the published contract, so an edit on either side
  fails.
- Measured in both directions rather than asserted: mutating
  `examples/s0-heartbeat/expected/system.txt` (`releases 4` → `releases 5`) →
  `test result: FAILED. 7 passed; 2 failed`; mutating the description instead (`chime` 30 ms →
  15 ms, expectation untouched) → `test result: FAILED. 6 passed; 3 failed`.
- What cannot be asserted yet is a **tripwire, not a skip**: `build_is_not_yet_assertable` pins
  `archogen build` at `unimplemented` and fails the moment `S0.3` makes it real, so `S0.4` cannot
  inherit a green test that checks nothing.
- Deriving the expectations needed a semantic nobody had written down — which way `(priority N)`
  compares. Recorded as `docs/decisions/decision_priority-comparison-direction.md` rather than
  inferred from three example files.
- Second seam, same instinct as `doctrine-seams-vs-forking-a-check`: `TASK-ACCEPTANCE` refused
  this commit because its default evidence signatures bless `git ls-files`, `git cat-file` and
  `git show` but not `git grep` or `git ls-tree` — the two verbs a census over a tree is actually
  written with, so the ROOT CAUSE box read as prose. Declared in
  `.doctrine/evidence_tokens.txt`, not patched into the portable script.
- Promoted to `docs/knowledge/an-oracle-is-independent-by-construction.md`.

## _(2026-09-13)_ — a presence check cannot see an extra key

- The eADL reader silently truncated a boundary-corpus rationale: it wrapped onto
  `;   implementation-independence: a different timer …`, which is exactly a bare key plus a
  colon, so a spurious seventh header opened. Two green suites were blind to it — the unit
  tests never wrapped onto a key-shaped line, and the corpus suite asserted the five keys it
  wanted were present, which they were. Found by running `examples/diagnose` over a real file
  and reading the output.
- The fix needs TWO discriminators, unindented AND bare-key, and each had already been tried
  alone and failed on a real corpus file. Tests now assert the exact key set per case.
- Promoted to `docs/knowledge/presence-checks-cannot-see-an-extra-key.md`.

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
