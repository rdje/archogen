- ID: `PROGRAM.15`
  Status: `done`
  Goal: make the outbound feedback register's **states and totals** mechanically consistent with the
  issue sub-trees that are their source — the half of "keeping this index true" that no check
  performs today.
  Reproduce / issue: `FEEDBACK-SELF-CONTAINED` leg 4 checks only that every issue directory is *named*
  in the vendor's `INDEX.md` — `grep -n 'INDEX' scripts/check_feedback_self_contained.sh` → lines 17,
  47, 49, 75, all registration. Nothing compares a register row's `State` cell with the `**State**`
  field in that issue's own `README.md`, and nothing recomputes the totals table from the rows:
  `git grep -ln 'State' -- scripts/` → only `check_waiver_routing.sh`, which is about waivers.
  THE GAP: a register row that contradicts its own sub-tree is undetectable by any gate. Measured
  consequence: five state transitions landed across six commits on `2026-09-27` (leaves `M1.20.1` –
  `M1.20.7`), and consistency held only because each leaf hand-edited both files and the last one ran
  a census by hand — 7 of 7 rows matched, but a commit that got one wrong would have passed every
  check. Impact: the register is the file a vendor reads first, and a row that disagrees with its own
  sub-tree is worse than no register at all.
  Acceptance: a check — a new `scripts/check_*.sh` or a fifth leg of the existing feedback check —
  fails when a row's State differs from its sub-tree's State field, when a `verified` row carries no
  dated archogen re-measurement behind it, or when the totals table does not equal a recount of the
  rows; it is registered in `scripts/check_doctrines.project.sh` and mirrored in
  `DOCTRINE_ENFORCEMENT.md`; it carries RED arms in `--self-test`; it is scoped to staged files, so an
  unrelated vendor's register cannot fail a commit; and its honest limit is stated in the header — it
  proves the two records agree, never that the measurement behind them was right.
  Priority: **medium** — no behaviour depends on it, but this repository's whole claim to a vendor is
  that its records are consistent, and that claim currently rests on hand-editing.
  Verification: see the checklist — a new gate over five legs (15 arms, 8 mutations), whose first run found a
  live inconsistency.
  Commit: `ARCHOGEN-PROGRAM-0137 (leaf PROGRAM.15)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — found live, one level above the register the leaf named: the cross-vendor index
    still counted five open bugs and two blockers after the register had closed all five (`0a98c66`,
    `2026-09-27`), and nothing noticed:
    ```text
    $ git grep -n "| 7 | 5 | 2 |" bf1eef8 -- docs/feedback/README.md
      bf1eef8:docs/feedback/README.md:14:| LinkedSpec | [`linkedspec/`](linkedspec/INDEX.md) | Rust backend + shipped Lispish grammar | 7 | 5 | 2 |
    $ bash scripts/check_feedback_register.sh            (its first run, on the real tree)
      FEEDBACK-REGISTER: docs/feedback/README.md says linkedspec has 5 open, and its register has 0 unresolved …
      FEEDBACK-REGISTER: docs/feedback/README.md says linkedspec has 2 blockers, and its register has 0 unresolved Blockers
      exit=1
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — `FEEDBACK-SELF-CONTAINED`'s only register leg is registration: that every
    issue is *named* in its `INDEX.md`; nothing read what a row, a total or the cross-vendor index *says*:
    ```text
    $ git grep -n "INDEX" bf1eef8 -- scripts/check_feedback_self_contained.sh
      bf1eef8:scripts/check_feedback_self_contained.sh:17:#   4. REGISTRATION   — the vendor's INDEX.md names every issue dire…
      bf1eef8:scripts/check_feedback_self_contained.sh:77:        note "$id exists but is not named in ${vendor}INDEX.md"; …
    ```
    And "open only" was never defined — the data at `792bb9d` (all five `open`) cannot tell "State is `open`"
    from "unresolved"; the register now says which.
  - [x] **FIX** — `scripts/check_feedback_register.sh` (**`FEEDBACK-REGISTER`**): rows against their issues (State,
    Severity's leading token, the linked directory), a `verified` row backed by a dated archogen re-measurement
    whose date is its `Last verified`, totals by state (counts and IDs) and by severity (unresolved = `open`,
    `acknowledged`, `fixed-upstream`, now stated in the register and the index) as recounts, and the cross-vendor
    index's Bugs / Open / Blockers; scoped to staged vendors, all when nothing is staged. The index corrected to
    `| 7 | 0 | 0 |`.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/check_feedback_register.sh
      feedback-register: OK (1 vendor register(s) agree with their issues, their totals and the cross-vendor index)   exit=0
    $ bash scripts/check_feedback_register.sh --self-test
      feedback-register self-test: 15 pass / 0 fail (15 arms)
    ```
    Eight mutations R-1–R-8, each restored by `cmp`, each fails its own arms (R-7 — `verified` counted as open —
    fails five). The staged-scope arm and the nothing-staged arm both hold.
  - [x] **NO REGRESSION** — `FEEDBACK-SELF-CONTAINED` exit=0 on the edited register; `SOURCE-LEDGER` caught the new
    book paragraph naming LinkedSpec uncited, and it cites the entry now; the doctrine driver green at the commit.
  - [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`, `verification.md` ("A bug report says what its issue
    says"); the definition of "open" in `docs/feedback/README.md` and `linkedspec/INDEX.md`.
