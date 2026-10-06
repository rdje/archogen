- ID: `PROGRAM.61`
  Status: `done` — filed by `M3.1.2.2` and closed `2026-10-06`
  Goal: the mutation catalog runs whole again, and an entry whose source moved is refused at commit time rather
  than found by the next `extended` run.
  Reproduce / issue: `cargo xtask mutate` on `2026-10-06` → exit 2, *"`cli-reaches-the-engine-directly`: its `from`
  text occurs 0 time(s) in crates/archogen-cli/src/check_cmd.rs — it must occur exactly once"*, then *"1 broken
  entr(ies) — fix the catalog"*. So the `extended` tier's `mutation` step cannot pass.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — as above; the new test, run before the fix → `test result: FAILED`, naming
    `cli-reaches-the-engine-directly` and nothing else.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `git show f51bca9 -- crates/archogen-cli/src/check_cmd.rs` (`API.4.2`,
    `2026-09-30 05:10`) turned `use archogen_api::{Closure, Request, Response};` into `use archogen_api::{Closure,
    Limits, Request, Response};`, the line the entry matches. WHY: the catalog runs whole only in the `extended` tier,
    and the commit path and CI run no part of it, so a moved text broke the run and nothing said so.
  - [x] **FIX** — the entry repointed at the line as it now reads. In `xtask/src/mutation.rs`'s tests,
    `every_entry_of_the_real_catalog_names_a_text_its_file_holds_once` parses the real catalog with the harness's own
    `parse` and applies each entry in memory with its own `apply`, so no second reader of the format exists to drift;
    it needs no build. `TOOLBOX.md`'s row says so, and its stale "~10 s" is now the measured figure.
  - [x] **ADDRESSED (verified)** — the test → `test result: ok. 1 passed` after the fix, `FAILED` before;
    `cargo xtask mutate --only cli-reaches-the-engine-directly` → `killed by
    the_cli_judges_nothing_except_through_the_engine_api`; `cargo xtask mutate` → exit 0, *"mutate: OK — 152
    mutation(s), each killed or surviving exactly as the catalog expects"*, in 177 s.
  - [x] **NO REGRESSION** — `make focused` → `tier focused: passed — 3 passed, 0 failed`; `cargo test --all -q` →
    1237 passed, 0 failed over 92 suites; the doctrine gate at commit.
  - [x] **LOCKSTEP** — the book's verification chapter describes the check under the `mutation` step; `TOOLBOX.md`;
    `CHANGELOG.md`.
