- ID: `PROGRAM.23`
  Status: `done`
  Goal: make the ruled push cadence **enforced rather than prose** — one machine-readable threshold, a
  check that reports the live count against it, and `MEMORY.md`'s layer-A field filled in. Ruled
  `2026-09-28` as a number of commits ahead of `origin/main`; see [[decision_push-cadence]], whose **Threshold**
  field is now that number's one copy.
  Reproduce / issue: `MEMORY_ARCHITECTURE.md:193` provides the slot — `(ahead of origin: <N>; push at
  ~<threshold>)` — and archogen never filled it. `grep -rniE 'rev-list --count|origin/main|
  push.threshold|ahead of origin' scripts/*.sh xtask/src/main.rs` → **no match**, so nothing mechanical
  has ever read a threshold. The cadence existed only as an operator batch instruction (BWFSC, default
  100 slices) that the PNT loop can never trigger, because PNT has no fixed BWFSC by definition; the
  branch went unpushed from `32e6b14` (`2026-09-13`) at ≈4.9 commits/day. Fourth instance in a week of
  a rule that lives only in prose — [[a-rule-only-in-the-prompt-is-enforced-nowhere]].
  Acceptance: the threshold has **one** producer, a machine-readable location the check reads, with
  `MEMORY.md` *reporting* it rather than restating it and no document carrying a second copy of the
  number; the check reports the live `git rev-list --count origin/main..HEAD` against the threshold and
  is registered in `TOOLBOX.md` and `DOCTRINE_ENFORCEMENT.md`'s E1 list; it arrives with repeatable
  `--self-test` RED arms to `PROGRAM.18`'s standard rather than joining its backlog, including an arm
  at exactly the threshold and one above it; the **live count appears in no tracked document**, since
  it moves with every commit; and ⛔ it **cannot deadlock the repository** — a blocking verdict at `N`
  while `make integration` fails would leave the tree able neither to commit nor to push, because
  `COMMIT.md` step 2 requires `make integration` before a push and that tier currently fails on the
  emulator step. So this leaf is sequenced **after `PROGRAM.10`** reclassifies that step, or ships as a
  loud non-blocking report until it has.
  Priority: **medium** — it makes a director ruling durable and observable, and it is the mechanism
  that stops the cadence becoming the next rule that exists only in a prompt. Behind `PROGRAM.10` for
  the deadlock reason above.
  Verification: see the checklist — the check's 10 arms (at, above and below the threshold among them) and three
  mutations; the threshold's copies removed from every document but its record; the real distance reported.
  Commit: `ARCHOGEN-PROGRAM-0149 (leaf PROGRAM.23)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — nothing read the threshold, and the number had copies outside its record:
    ```text
    $ git grep -niE 'rev-list --count|push.threshold|ahead of origin|PUSH_AT' HEAD -- 'scripts/*.sh' xtask/src/main.rs
      (no match)
    $ git grep -nw "400" HEAD -- '*.md' ':!CHANGELOG.md' ':!DEV_NOTES.md' ':!docs/decisions/decision_push-cadence.md' | grep -vc "sequences|cases|…"
      8      — the INDEX hook and seven lines of this tree
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — the ruling was recorded as prose in `decision_push-cadence.md`, and the only
    trigger the operator's instructions gave was a batch count the PNT loop never reaches. `MEMORY_ARCHITECTURE.md:193`
    carries the slot (`git show HEAD:MEMORY_ARCHITECTURE.md | sed -n 193p` → `(ahead of origin: <N>; push at
    ~<threshold>)`) and no instrument filled it, so each document that wanted the number retyped it.
  - [x] **FIX** — `scripts/push_cadence.sh`: the threshold from the record's `PUSH_AT_COMMITS_AHEAD=` field, exactly
    one of which must exist and which the title must agree with; the distance from `origin/main`; exit `0` below, `3`
    due, `2` cannot tell. A report, never a gate — in no registry and no tier, so it cannot deadlock. The eight copies
    replaced by pointers; `PROGRAM.10.1` had already removed the deadlock this leaf had to wait on.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/push_cadence.sh; echo "exit=$?"
      push-cadence: 154 commit(s) ahead of origin/main; the ruled threshold is 400 — 246 to go      exit=0
    $ bash scripts/push_cadence.sh --self-test   → push-cadence self-test: 10 pass / 0 fail (10 arms)
      M1 due only past the threshold (-lt → -le)   → "exactly at the threshold, a push is due — expected exit 3", restored
      M2 no origin/main read as level              → "no origin/main is 'cannot tell', not zero", restored
      M3 a second copy tolerated                   → "a second copy of the threshold is refused", restored
    ```
    (The live count above is evidence of this run, not a figure to keep: it is written nowhere else.)
  - [x] **NO REGRESSION** — every self-test, the new one among them:
    ```text
    $ bash scripts/run_self_tests.sh
      ✓ scripts/push_cadence.sh   2s  push-cadence self-test: 10 pass / 0 fail (10 arms)
      self-tests: OK — 26 self-test(s) passed          exit=0
    ```
  - [x] **LOCKSTEP** — the decision record gains its field and loses its "until it lands"; `INDEX.md`, `MEMORY.md`,
    `COMMIT.md` step 2, `DOCTRINE_ENFORCEMENT.md` (why it is not a gate), `TOOLBOX.md`, `verification.md` "When a
    push is due". ⚠️ Honest limit: `FIGURE-REGISTER` refuses a bare `400 commits` added to a live page, but not a
    number inside a code span, which it exempts by design.
