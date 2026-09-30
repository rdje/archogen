- ID: `PROGRAM.33`
  Status: `done`
  Goal: the book-transcript backlog is keyed by what a transcript *is*, not by the line it sits on.
  Reproduce / issue: routed from `S0.8` (its ROUTING EVIDENCE has the measurement). `BACKLOG` in
  `crates/archogen-cli/tests/book_transcripts.rs` names each unreproducible transcript as `chapter:line`, so a
  paragraph added above one fails the test twice — a "new" transcript and a "vanished" one — for bytes that did not
  change. `S0.8` moved its entry by hand; every other entry is exposed the same way.
  Acceptance: an entry names the chapter and the block's own content (its first `error[` line and first `-->`
  location, or a digest), so an edit above a block does not move it; the two ratchet directions still hold (a new
  unreproducible transcript refused, a fixed one's entry required to go); arms for an edit above a backlog block
  (passes) and for a changed block (its entry no longer matches).
  Priority: **low** — a false failure that names its own fix, never a false pass.
  Verification: see the checklist — the eleven entries re-keyed by content, five arms, two catalogued mutations.
  Commit: `ARCHOGEN-PROGRAM-0153 (leaf PROGRAM.33)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — measured by `S0.8`, whose paragraph above `s0.md`'s backlog block failed the test for
    a block whose bytes had not changed:
    ```text
    $ cargo test -q -p archogen-cli --test book_transcripts
      new transcript(s) over an input no reader can rebuild: ["s0.md:224"] — name a file in the repository
      backlog entries that are no longer there: ["s0.md:214"] — delete them from BACKLOG
      test result: FAILED. 0 passed; 1 failed
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — the key was the block's position, `format!("{name}:{}", i + 1)`, and
    positions move with every edit above them: `git show HEAD~1:crates/archogen-cli/tests/book_transcripts.rs | grep -n
    '"s0.md:'` → `40:    "s0.md:214",` in the version before `S0.8`, all eleven entries in that form.
  - [x] **FIX** — the key is `<chapter>: <first error[ line>` plus ` @ <first --> location>` when the block has one;
    the eleven entries re-keyed (unique, checked); the ratchet is a pure `backlog_problems()` comparing multisets.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ cargo test -q -p archogen-cli --test book_transcripts   → test result: ok. 6 passed; 0 failed
      arm 1 an edit above a backlog block → its line moves, its key does not, no problem
      arm 2 a changed block → 2 problems (new, and no longer there) · arm 3 a new block → 1 · arm 4 a fixed block → 1
      arm 5 a second copy of a listed block → 1 (a set would have let it through)
    $ cargo xtask mutate --only transcript-backlog-keyed-by-line transcript-backlog-as-a-set
      mutate: OK — 2 mutation(s), each killed or surviving exactly as the catalog expects
    ```
  - [x] **NO REGRESSION** — the real book still passes, every checked transcript still compared exactly:
    `cargo test -q -p archogen-cli --test book_transcripts` → `test result: ok. 6 passed; 0 failed`, the main test among
    them. ⚠️ Corrected after the commit: this box first said "the whole suite at the commit", which had not been run.
    Run afterwards, it gave `636 passed, 1 failed` — a race in `S0.8`'s new gate, not in this leaf, fixed by
    `ARCHOGEN-S0-0154`; three runs after the fix, `637 passed, 0 failed` each.
  - [x] **LOCKSTEP** — `verification.md`'s transcript section says how an entry is named, and why.
