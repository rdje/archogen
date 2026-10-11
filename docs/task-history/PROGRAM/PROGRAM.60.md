- ID: `PROGRAM.60`
  Status: `done` — filed `2026-10-06` by `M3.1.2.1`; started and closed `2026-10-10`
  Goal: the third reader's record, `docs/semantics/third-opinion.txt`, cannot fall behind the tracked descriptions
  unseen.
  Reproduce / issue: `bash scripts/third_opinion.sh` on `2026-10-06` → `125 document(s) — 125 agree`, then exit 1:
  four files *"agree, and not in the record"*. They were added by `M2.13` (`49638bf`, `2026-10-01`), `M2.14`
  (`d65d836`, `2026-10-01`, two files) and `M3.6.2` (`1d6415d`, `2026-10-05`), and the record was last written by
  `947cdc2` on `2026-09-30`. Each leaf's commit passed every gate.
  Root cause: the script needs LinkedSpec's `sexpr_file`, built under `.app-data/`, and exits 2 without it. So it
  runs only by hand: census `git grep -n third_opinion -- scripts xtask Makefile .github .githooks
  ':!scripts/third_opinion.sh'` → no match, so no tier, hook, doctrine or workflow invokes it. A description added after the record
  was last written goes unrecorded until somebody runs the script by hand. `M3.1.2.1` blessed the four (all
  `agree`) beside its own two files.
  Direction, to be designed here: the frozen verdict table (`verdicts.txt`) and the record share one population, so
  a check needing no vendor build can refuse a tracked description that the record lacks. The reader's verdict on a
  new file then still needs the build, but its absence from the record becomes visible at commit time.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the new test run against the record as `947cdc2` left it (`git show
    947cdc2:docs/semantics/third-opinion.txt`) → *"6 difference(s) …"*: the four this leaf names and two more since,
    `docs/semantics/kinds/deffact.eadl` and `docs/semantics/vocabulary/vocabulary.eadl`; against today's record, `bash
    scripts/third_opinion.sh` → *"127 document(s) — 127 agree … OK"*, `M3.1.2.1`'s bless having caught up by hand.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `git grep -n third_opinion -- scripts xtask Makefile .github .githooks
    crates ':!scripts/third_opinion.sh'` at `3d9a70c` → one match, `crates/eadl-front/examples/reader_shape.rs:1`, a
    comment naming the script whose shape that example prints: no tier, hook, test or workflow ran the census. WHY:
    the script checks the census and the classification together, and the classification needs LinkedSpec's build.
  - [x] **FIX** — `crates/archogen-cli/tests/verdicts.rs`: `the_third_reader_s_record_names_every_description`, the
    record's paths against the verdict table's own population, every description and only those, with no vendor
    build; the two populations one, as the direction asks. The script's header and the book's *A third reader*.
  - [x] **ADDRESSED (verified)** — `cargo test -p archogen-cli --test verdicts the_third_reader` → `1 passed` on the
    tree; against `947cdc2`'s record → `1 failed`, six descriptions named; against a record naming `docs/nowhere.eadl`
    → `1 failed`, it named — `target/p60/falsify.py`, untracked and so not durable, the record restored byte for byte.
  - [x] **NO REGRESSION** — `make focused` → `passed — 3 passed, 0 failed`; the doctrine gate at commit.
  - [x] **LOCKSTEP** — the script's header; the book's *A third reader*; this leaf and both logs; `docs/TASK_TREE.md`;
    `CHANGELOG.md`.
  Verification: `2026-10-10` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0529 (leaf PROGRAM.60)`
