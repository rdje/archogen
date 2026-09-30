- ID: `PROGRAM.3`
  Status: `done`
  Goal: implement the tiered verification runner (`xtask`) for the §14.3 tiers
  (focused / integration / extended / hardware / assurance), including the CI-policy split
  between per-commit focused checks and the pre-push full gate.
  Acceptance: each tier is a named command; a skipped or unavailable required tool is
  reported as skipped, never as a pass.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0029`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — §14.3's five tiers existed only as a table in the roadmap,
    and the director's CI policy (run the full gate before a push, a selected set for ordinary
    commits) had no mechanism at all. Measured at the parent commit:
    `git grep -ci 'tier' HEAD -- Makefile .github/` printed nothing, `rc=1`, and
    `git ls-tree --name-only HEAD` → no `xtask`, though §4.2 names it and
    `.doctrine/code_paths.txt` already reserves `(^|/)xtask/`. WHERE the cost landed: every
    verification decision was a judgement made per commit and recorded nowhere, so the three
    tiers nobody can run today — extended, hardware, assurance — were **invisible** rather than
    incomplete.
  - [x] **ADDRESSED (verified)** — five named commands, and the honest picture they produce:
    `cargo xtask verify --tier focused` → `tier focused: passed — 3 passed, 0 failed, 0
    unavailable, 0 not built`, `exit=0`;
    `--tier integration` → `incomplete — 5 passed, 0 failed, 1 unavailable, 0 not built`,
    `exit=20`, naming QEMU;
    `--tier extended` → `incomplete — 0 passed, 0 failed, 1 unavailable, 2 not built`;
    `--tier hardware` → `incomplete — … 1 not built`, naming `M5.1` and the absent board;
    `--tier assurance` → `incomplete — … 3 not built`, naming `M3.6`, `M4.8`, `M4.7`.
    `cargo test -p xtask` → `test result: ok. 8 passed; 0 failed`.
  - [x] **NO REGRESSION** — `cargo fmt --all -- --check` → `fmt rc=0`;
    `cargo clippy --all-targets --all-features -- -D warnings` → no warnings;
    `cargo test --all` → **317** passed, `0 failed` (309 before this leaf, `+8`);
    `scripts/check_doctrines.sh` → `=== all doctrines green ===`; `mdbook build docs/book` →
    `INFO HTML book written to`. `make check` and `make gate` are unchanged, so nothing that
    referenced them broke.
  - [x] **FIX** — `xtask/` (a workspace member, per §4.2), reached through the committed
    `.cargo/config.toml` alias, because a tier that works only if each developer remembers an
    incantation is a tier nobody runs. Tiers are declared **as data**, the idiom `spec.rs`
    already uses here. The verdict type has **three** states — `passed`, `failed`,
    `incomplete` — and `incomplete` is the whole point: §14.3 says "a required tool skipped or
    unavailable is reported as such, **not a passed check**", which two states cannot express.
    Two kinds of "cannot run" are kept apart because the response differs: **unavailable** (a
    tool is missing from this machine — install it) and **not built** (the step does not exist,
    and names the leaf that owns building it).
  - [x] **REPRODUCE / ISSUE** — the runner found two real defects on its first two runs, which is
    the argument for it. (1) It reported `❌ fmt … FAILED` **with no reason**: `cargo fmt
    --check` writes its diff to stdout while the runner captured only stderr. A failing step
    that cannot say why is one a developer re-runs by hand, which is the same as not having a
    runner — both streams are now captured, the tail is printed, and the exact command to re-run
    is echoed. (2) `every_named_leaf_is_declared_by_a_task_tree` failed on first run —
    `step \`fuzz\` names leaf \`PROGRAM.9\`, which no tree under docs/tasks/ declares` — because
    the shape test above it cannot tell `M9.9` from `M4.8`. `PROGRAM.9` now exists, and the
    existence check stays.
  - [x] **LOCKSTEP** — `Makefile` gains `make focused` / `make integration` / `make tiers`;
    `COMMIT.md` step 2 adopts the director's CI policy explicitly; `TOOLBOX.md` gains the runner;
    `docs/book/src/verification.md` (new chapter) + `SUMMARY.md`;
    `.github/workflows/rust.yml` runs the integration tier so CI and the local gate are the same
    object; `knowledge-map/subsystems.md`; `MEMORY.md`; `LIVE_STATUS.md`; `CHANGELOG.md`;
    `DEV_NOTES.md`; `docs/TASK_TREE.md`.

  ### Lesson promotion

`a verification runner's most useful output is what it cannot run` →
  `promotion: declined (already docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md, applied there to fixtures and here to tiers; a second entry for the same question makes the retrievable layer harder to search)`

  ⛔ **Written on one line deliberately.** `scripts/check_lesson_promotion.sh` matches
  `promotion: declined \(..*\)` with a **line-oriented** grep, so a decline whose parentheses
  wrap across lines is invisible to the gate — which is how a decline can look recorded and not
  be. Found by this very commit: the first draft wrapped, and the gate reported
  `1 new lesson entry in DEV_NOTES.md with NO promotion and NO explicit decline`. The same wrap
  exists in `S0.3`'s decline, where it went unnoticed because that commit also promoted a lesson
  and satisfied the check the other way; it is corrected in its own docs-only commit, since
  staging a second tree beside code is what `PROGRAM.8` tracks.

  ### The measurement that settled `focused`'s contents, recorded because it will expire

  §14.3 defines focused as "format/type checks and **affected** contract tests", and selecting
  affected tests needs change-impact machinery. Measured warm on this tree:
  `cargo fmt` 0.14 s, `cargo clippy` 0.08 s, `cargo test --all` **2.63 s**,
  `scripts/check_doctrines.sh` 1.36 s, `mdbook build` 0.08 s. At 2.9 s for the whole focused
  tier, impact analysis would cost more than it saves and would be one more thing to be wrong,
  so `focused` runs the entire suite. The number is written into the runner's own source so the
  decision can be re-taken against it rather than re-argued from memory.
