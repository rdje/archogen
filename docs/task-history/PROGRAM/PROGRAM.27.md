- ID: `PROGRAM.27`
  Status: `done`
  Goal: make `LANGUAGE-FREEZE`'s **explicitness** leg able to fail on the real tree, so a migration note
  is an enforced precondition rather than a record nobody checks.
  Reproduce / issue: **a live soundness hole in a registered doctrine gate, reproduced `2026-09-29` by
  `M1.28.2`** — the first leaf to run the migration workflow `M1.13.5` wrote. The leg `M1.13.5`'s own
  leaf called "the one that matters", because it is what stops `--emit` from being the waiver, has never
  been able to fail outside its own scratch directory:

  ```text
  $ mv docs/semantics/migrations/eadl-1-quantity-value-type.md target/tmp/   # NO note at all
  $ scripts/check_language_freeze.sh                                          # baseline amended, 70 -> 72
  language-freeze: OK (72 frozen construct(s) agree with the working tree)     # rc=0 — a FALSE GREEN
  $ grep -rl '^- status:[[:space:]]*pending' docs/semantics/migrations
  docs/semantics/migrations/README.md
  $ grep -n '^- constructs:' docs/semantics/migrations/README.md
  36:- constructs: <construct id>, <construct id>   — or: all
  ```

  Root cause, **two holes that only close together**:
  1. the notes population is `grep -rl '^- status:[[:space:]]*pending' "$NOTES"`, and the directory's own
     README documents the form with the line `- status: pending | applied` — so the README is permanently
     a pending note;
  2. `names_construct`'s first case is `*all*) return 0 ;;`, a **substring** test, and the README's
     template line ends `— or: all` — so that note covers every construct id there is.

  ⛔ **Why nine RED arms did not see it.** All of them run against `$work/notes`, a scratch directory
  holding only the fixture note, so the arms proved the *mechanism* and never the *deployed population*.
  `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md` one level up: the fixture set did not
  contain the thing that would break it, which was a file the gate's own directory ships for a different
  purpose. Impact: any frozen construct of `eadl/1` can be edited and the baseline re-emitted with no note
  and the gate green — leg A still catches a construct edited and the baseline left alone, so the hole is
  exactly the direction `M1.13.5` wrote leg B for.
  Acceptance: leg B fails on the real tree with the baseline amended and no pending note, and passes with
  one that names the movement — both measured on `docs/semantics/migrations/` itself and not on a scratch
  copy of it; the notes population excludes the directory's own documentation **by a rule and not by a
  filename**, so a second form-documenting file cannot reopen it (a status field holds one value, so
  `- status: pending` anchored to the end of the line is the rule, and the README's `pending | applied` is
  not a status); `constructs:` is parsed as a **list of ids compared exactly**, with `all` honoured only
  when it is the whole value; `--self-test` gains an arm whose notes directory contains a README-shaped
  file and requires the movement to stay **uncovered**, which is the arm whose absence is this defect;
  `docs/semantics/migrations/README.md`'s workflow corrected against measurement — ⛔ two of its steps are
  false as written: it says the gate goes green once a pending note exists *before* `--emit`, and leg A
  compares the tracked baseline with a fresh run and does not consult notes at all, so it stays red until
  the emit; and it puts the flip to `applied` in the same sequence as the emit, where leg B would then
  refuse the commit, because an `applied` note cannot cover the movement it is landing in. The lifecycle is
  two commits and the README has to say so, with `M1.28.2`'s note — still `pending` in the tree, because
  leg B reads the working tree — flipped to `applied` here as the first real use of the lifecycle;
  `DOCTRINE_ENFORCEMENT.md`'s `LANGUAGE-FREEZE` row corrected (it states the second leg as though it
  worked); `make focused` exit `0`.
  Priority: **high** — a registered gate that cannot fail is worse than no gate, because the registry is
  what a reader consults to find out what is enforced, and this row says the language definition is
  protected against silent amendment. It is sequenced ahead of `PROGRAM.11` on that reasoning: `.11`'s
  rule is preventive and has fired once and been handled, while this one is inert now.
  Verification: see the checklist — the false green reproduced and closed on the deployed notes directory,
  sixteen arms (seven new), six mutations seen firing.
  Commit: `ARCHOGEN-PROGRAM-0111 (leaf PROGRAM.27)`

  ⛔ **Two further defects found while arming it, both fixed here because both are this gate's.**
  (1) **Two pre-existing arms were vacuous.** "a construct the baseline no longer freezes is refused" and
  "a construct nobody declared is refused" appended a row to the *end* of an otherwise sorted fixture; the
  classifier refuses an unsorted baseline **whole**, as unreadable, so both arms passed on leg A's "not a
  readable baseline file" note and neither ever classified a removal or an addition — from the commit
  that wrote them (`2ae744a`, `M1.13.5`) until now. Found because a new arm of the same shape passed under
  a mutation that should have turned it red. (2) **The classifier blamed the wrong file.**
  `crates/eadl-front/tests/common/baseline.rs`'s `parse` stamped every message with
  `docs/semantics/BASELINE.txt`, whatever it had been given — a fixture, a fresh run, `HEAD`'s copy — so a
  malformed scratch file read as a defect in the tracked baseline. `parse_named(name, text)` now carries
  the real name, and `parse(text)` stays the tracked file's convenience.

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — on the **deployed** notes directory, with the pre-`M1.28.2` baseline
    (`git show 7f46f4d:docs/semantics/BASELINE.txt`) as `HEAD`'s side so a real amendment is in flight:
    ```text
    $ mv docs/semantics/migrations/eadl-1-quantity-value-type.md target/tmp/p27/      # NO note at all
    $ LANGUAGE_FREEZE_HEAD_BASELINE=target/tmp/p27/head-before-m1282.txt bash scripts/check_language_freeze.sh
      language-freeze: OK (72 frozen construct(s) agree with the working tree)       rc=0 — FALSE GREEN
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_language_freeze.sh` at `a6e8972`, three sites:
    ```text
    $ git show a6e8972:scripts/check_language_freeze.sh | grep -n "grep -rl '\^- status\|\*all\*\|\*\"\$id\"\*"
    $ git show a6e8972:scripts/check_language_freeze.sh | grep -nF -e "grep -rl '^- status" -e '*all*)' -e '*"$id"*)'
      152:  done < <(grep -rl '^- status:[[:space:]]*pending' "$NOTES" 2>/dev/null)    <- unanchored
      169:    *all*) return 0 ;;                                                            <- substring
      172:    *"$id"*) return 0 ;;                                                          <- substring
    $ grep -n '^- status\|^- constructs' docs/semantics/migrations/README.md
      35:- status: pending | applied          <- matches line 152, so the README is a pending note
      36:- constructs: <construct id>, <construct id>   — or: all    <- matches line 169: covers everything
    ```
    WHY nine arms missed it: every leg-B arm set `LANGUAGE_FREEZE_NOTES=$work/notes`, a scratch directory
    holding only the arm's own fixture, so no arm ever read the directory the gate reads. And a third
    hole in the same leg, measured rather than supposed: `M1.28.2`'s note stayed `status: pending` from
    `1a43973` to `a6e8972` — five commits in which it silently covered any movement of its five constructs,
    because nothing refuses a note that stays pending after its movement lands.
  - [x] **FIX** — a pending note is a line **exactly** `- status: pending` (`PENDING_LINE`, anchored at
    both ends); `names_construct` parses `constructs:` as a comma-separated list compared **exactly**, with
    `all` honoured only as the whole value; **leg C, spent notes**: a note `HEAD` already carries as pending
    covers nothing and is refused until it says `applied` (read from git, or from `LANGUAGE_FREEZE_HEAD_NOTES`
    in an arm); the breach summary counts breaches and no longer claims every breach is a movement. The
    arms' oracle now also requires the construct or note the refusal is about, and every fixture is
    written sorted. `M1.28.2`'s note flipped to `applied` — the lifecycle's first real use.
  - [x] **ADDRESSED (verified)** — on the deployed directory, the four cases the acceptance names:
    ```text
    (1) the real tree after the flip                                    -> language-freeze: OK   rc=0
    (2) amended vs 7f46f4d, README + the now-applied note only           -> rc=1, 5 constructs uncovered
    (3) + a pending note in docs/semantics/migrations/ naming the five   -> language-freeze: OK   rc=0
    (4) that note rewritten to the README's two template lines           -> rc=1, 5 constructs uncovered
    (0) the real tree BEFORE the flip -> rc=1 "eadl-1-quantity-value-type.md is still `status: pending`
        and HEAD already carries it that way"                          <- leg C on its first real instance
    $ bash scripts/check_language_freeze.sh --self-test
      language-freeze self-test: 16 pass / 0 fail (16 arms)
    ```
    **Six mutations seen firing**, each checked applied and restored from a copy proven identical by `cmp`:
    ```text
    P-A  status rule unanchored (hole 1 alone)   -> 14 pass / 2 fail: the exact-status arm, and "an
                                                     agreeing baseline passes" (leg C then reads the
                                                     committed README as a spent note)
    P-B  constructs as a substring (hole 2 alone) -> 15 pass / 1 fail: the exact-name arm
    P-C  both holes, leg C kept                  -> 12 pass / 4 fail
    P-D  leg C removed                           -> 14 pass / 2 fail: both spent-note arms
    P-E  both holes AND no leg C (as shipped)    -> 10 pass / 6 fail, INCLUDING "the deployed notes
                                                     directory leaves an unnamed amendment uncovered"
                                                     (expected exit 1, got 0) — the defect, on the
                                                     population the gate reads
    P-F  the dropped fixture unsorted (as shipped) -> 14 pass / 2 fail: "refused, but not about
                                                     `suite/examples/s0-heartbeat/system.eadl`, so it
                                                     refused for another reason" — the two vacuous arms
    ```
  - [x] **NO REGRESSION** — `make focused` → `passed — 3 passed, 0 failed, 0 unavailable` (the first run
    failed on `fmt` alone and was fixed, not waived); `cargo test --all` → **575 passed, 0 failed over 41
    suites**, unchanged, `language_baseline.rs` 9 / 0 after the parser change; every gate's `--self-test`
    run: `book-anchors 6/0`, `feedback 6/6`, `gap-claims 10/10`, `language-freeze 16/0`,
    `lesson-promotion 9/9`, `live-doc-currency 3/3`, `routing-evidence 5/5`, `s0-retirement 3/0`,
    `table-arity 8/8`, `task-acceptance 9/0`; `scripts/check_doctrines.sh` → `=== all doctrines green ===`;
    `check_book_anchors.sh` → `OK (19, 3)`; `mdbook build` `rc=0`.
  - [x] **LOCKSTEP** — `docs/semantics/migrations/README.md`: what a pending note is, spent notes, exact
    `constructs:`, and the workflow rewritten as the **two commits** it is, with both false steps
    corrected; the quantity note flipped to `applied`; `DOCTRINE_ENFORCEMENT.md`'s row (three legs, the
    history, no arm count); the book's `verification.md` (legs table, lifecycle, what was broken);
    `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md` gains two lessons and an `answers:` line;
    `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `docs/TASK_TREE.md`, this tree —
    and **`PROGRAM.28`** filed below for the gap measuring the arms exposed: no tier runs any of them.
