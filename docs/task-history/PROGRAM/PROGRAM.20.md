- ID: `PROGRAM.20`
  Status: `done`
  Goal: a **carried-figure register**, so a figure no measurement watches is a breach at the commit
  that adds it rather than a defect a later sweep happens to find. Three sweeps with three different
  patterns have now been needed to find one defect class, and a fourth pattern would find a fourth
  spelling: the mechanism, not the sweep, is what is missing.
  Reproduce / issue: the class has fired four times in this repository. `M1.23` found two live
  surfaces publishing a schema reach superseded 47 commits earlier, sweeping
  `grep -rnE '[0-9]+ of (the )?[0-9]+'` → 9 hits, 2 false. `M1.24` found three more live surfaces
  false from the **same commit** (`9030111`), which that pattern could not see because a corpus *size*
  is not an `N of M` figure; its own first replacement pattern (digits followed by a size noun) then
  found 2 of those 3 and could not see `the five ambiguous cases`, because it is spelled out. Two more
  shapes are already measured: `docs/book/src/s0.md` says "Three descriptions" where the directory
  holds four (`S0.8`), and `docs/TASK_TREE.md`'s `S0` row said "all six leaves closed" for one commit
  after `S0.7` added a seventh — `git show 4b7e000^:docs/TASK_TREE.md` and `git show
  4b7e000:docs/TASK_TREE.md` are byte-identical on that row, so the commit that moved the count never
  looked at the index that restates it. Corrected by `M1.24`'s routing edit rather than left. A fifth
  shape, `2026-09-28` (`M1.12.2`): `docs/semantics/grammar.md` headed a list **"Three rules the
  productions above imply"** above *four* numbered items — a **normative document**, and not one of
  the surfaces this leaf's acceptance enumerates (book chapters, corpus indexes, crate module headers),
  so no existing gate would have caught it and no sweep pattern aimed at it. Fixed by deleting the
  count rather than retyping it, which is the register's own prescription.
  A sixth shape, `2026-09-29` — found while recovering a force-quit session, and ⛔ **it is not a
  figure at all**, so a register scoped to figure-shaped text would not see it. Three stale copies of
  an *ordered sequence* and of a *list head*, left by the two commits that closed `PROGRAM.21` and
  `M1.13.3`: `docs/TASK_TREE.md`'s `PROGRAM` row still named `PROGRAM.21` as the frontier one commit
  after that leaf's own status became `done`, and both that row and `LIVE_STATUS.md`'s `M1` row named
  the frontier as its own successor ("Then `.13.4` … `.13.5`"), because a closure rewrote the frontier
  sentence and left the old successor clause behind. Census, re-runnable both ways:

  ```text
  census:   awk -F'|' '/^\| \[`/{print $2, substr($4,1,14)}' docs/TASK_TREE.md, against each
            docs/tasks/<TREE>.md "## Current Frontier" order-1 row
            -> 10 of 12 rows AGREE, pre-correction; PROGRAM DISAGREE (index `PROGRAM.21`,
               tree `PROGRAM.11`). After this commit's correction: 11 of 12, and the only
               DISAGREE left is the artifact below.
               M5's cell carries no head at all, so its DISAGREE is the pattern reading the
               blocker the cell names (`M0.5`) — recorded, not quietly dropped
  census:   git ls-files '*.md' | xargs grep -n -o "[Tt]hen `\.13\.4\`"
            -> LIVE_STATUS.md:15 and docs/TASK_TREE.md:57 only; no other tree's row repeats
               its own head, and MEMORY.md / docs/tasks/M1.md already read `.13.4` then `.13.5`
  evidence: git show --stat 7c59b0b -- docs/TASK_TREE.md | wc -l  -> 0
            the commit that closed PROGRAM.21 staged no line of the index that restates its
            frontier — the same evidence shape as the S0 row above
            (git show --stat 91f329d -- docs/TASK_TREE.md -> 1 insertion, the M1 row it rewrote)
  ```

  Corrected in `ARCHOGEN-PROGRAM-0088`, docs only, no code staged. ⚠️ The paragraph above ended
  "classifying **five** shapes by hand", which made the register's own description carry a figure — the
  class, restated inside the leaf that exists to end it. ⛔ It went stale one shape later, so the count
  is **deleted** here rather than retyped, which is this leaf's own prescription for exactly this.

  A seventh shape, `2026-09-29` — found by `M1.13.4`'s measurement (its M-I: the identifier retrofit
  executed on all 62 suite files and reverted), and ⛔ **not a figure about the repository at all**: a
  line number **inside a quoted transcript**. `grep -rn '^[0-9]\+ |' docs/book/src/*.md` → 14 lines over
  8 chapters (`boundary.md:88`, `checking.md:66` and `:70`, `modules.md:73` and `:77`, `presence.md:67`
  and `:71`, `quantities.md:62`, `reading.md:84`, `:99` and `:103`, `s0.md:218` and `:222`,
  `workload.md:94` and `:98`), each one a diagnostic rendering copied out of a real run. Editing the
  description a transcript quotes moves every line number in it, and nothing compares a transcript to
  the run it claims to be — so the shape is invisible to both existing sweep patterns (`N of M`, digits
  followed by a size noun) and to every gate in the repository. It is the `M1.23`/`M1.24` class in the
  one live surface the register's acceptance already names (book chapters) but no sweep has aimed at.
  ⚠️ **Recorded, not fixed here**: `M1.13.4.2` re-renders the transcripts its own insertion moves and
  states on its leaf that the class stays open. Whether the register should require a transcript to be
  *generated* rather than copied — which is the only fix that cannot rot — is this leaf's design work.

  ⛔ **The seventh shape has a measured false instance now, not only a hypothetical one** (`2026-09-29`,
  found by `M1.13.4.2` re-rendering the transcripts its retrofit moved rather than incrementing them).
  `docs/book/src/s0.md`'s malformed-description block read `--> build/s0/malformed.eadl:41:1` over an
  **empty** source line, and that is not what the fixture produces: `s0_reader.rs` writes the corrupted
  copy with `text.trim_end().strip_suffix(')')`, so there is no trailing newline and the reader's
  "input ends here" label lands at the end of the last real line. Measured both ways on the
  pre-retrofit description, so this leaf's own change is not the cause:

  ```text
  fixture as s0_reader.rs writes it (trim_end)  -> --> build/s0/malformed.eadl:40:36
                                                 40 |   (platform (uses host.playground))
                                                    |                                    ^ input ends here
  fixture keeping its trailing newline          -> --> build/s0/malformed.eadl:41:1
                                                 41 |
                                                    | ^ input ends here        <- byte-for-byte the book
  ```

  So the block was rendered from a fixture shape **no tracked command writes**, and it was false before
  anything moved. The same chapter's `--locked` block was abridging its hint's last sentence ("Re-run
  without `--locked` for an experimental build") with no marker saying so. Both were re-rendered by
  `M1.13.4.2`; the class is untouched.

  ⭐ **What the instance adds to the register's design work.** A moved line number is the *cheap* half of
  this shape. The expensive half is that a transcript quotes a **run**, and a run has an input: this one's
  input (`build/s0/malformed.eadl`) exists nowhere tracked, so a reader who follows the `$` line cannot
  reproduce the output even in principle, and neither can a gate. Six of the fourteen line-numbered
  transcript lines re-rendered cleanly from a tracked command over a tracked file; this one needed the
  fixture reconstructed from the test that writes it. So the register's rule for this shape is probably
  two rules — a figure that moves must be re-derived, and a transcript must name an input a reader can
  rebuild — and the second is the one nothing in the repository asks today.

  Two more instances, `2026-09-29`, of the **older** shapes (a corpus size and a table of counts) —
  recorded because of *how* they were found, which no sweep in this leaf's acceptance would have found.
  `M1.13.4.3` added a thirtieth case to `docs/semantics/cases/` and then asked "which surfaces does this
  change move?", and `docs/book/src/checking.md` answered twice:

  ```text
  census: grep -h '^; expect:' docs/semantics/cases/*.eadl | sort | uniq -c | sort -rn
          -> invalid-description 11, unsupported-profile 7, ok 6, infeasible-configuration 4,
             missing-fact 2   (30 cases)
  figure: docs/book/src/checking.md said "holds 25 worked cases" and tabulated 5 / 10 / 5 / 4 / 1 = 25
  age:    git log --format=%H -S 'holds 25 worked cases' -- docs/book/src/checking.md -> 6df022f
          git ls-tree --name-only 6df022f docs/semantics/cases/ | wc -l            -> 25  (true then)
          git rev-list --count 538fe3b..HEAD                                       -> 71  (false since)
          538fe3b is `M1.9`, which added four cases and touched neither figure
  ```

  So the sentence was false for **71 commits** and **three of the table's five rows** were wrong, with the
  table's own sum contradicting the sentence above it — and the sweep patterns this leaf enumerates would
  not have seen the table at all, because a bare count in a cell is neither an `N of M` figure nor a
  digit followed by a size noun. ⭐ **The mechanism that found them is a fourth one, and it is cheaper
  than any sweep: census the surfaces a change moves, at the commit that moves them.** A change to a
  population knows which population it touched; a sweep has to guess what a figure might be about. Both
  figures are corrected to the measurement **and** gated in that commit — one leg compares the published
  size to the walk, one compares the table in both directions — so the register's *gated* classification
  now covers them, and `M1.24`'s prescription ("retyping the fresh number is not the fix") held rather
  than being quoted.
  An eighth shape, `2026-09-29` — found by censusing a surface this slice was editing, which is the fourth
  mechanism above doing its job, and ⛔ **what it falsifies is the register's own population**, for the
  second time. `CHANGELOG.md` says "Newest first" in its own header and did not: `ARCHOGEN-M1-0092`'s
  entry sat **above six newer ones**, so the surface the director reads for "what just happened" opened
  with work seven commits old. The root cause is pinned to the line and it is a mechanism, not a slip —
  five consecutive commits inserted at the *same* anchor:

  ```text
  census: for c in fc3655c d0692f2 145baab 2ae744a 6a31da5; do
            git show $c -- CHANGELOG.md | grep -m1 '^@@'; done
          -> @@ -52,6 +52,56 @@ · +52,58 · +52,59 · +52,62 · +52,48   ONE anchor, five commits
  census: git log --format=%H -S'every description the repository ships now states its language version' \
            -- CHANGELOG.md
          -> b88812d, the commit that put 0092's entry at the top when it WAS the newest
  census: git rev-list --count b88812d..HEAD   -> 8, so the order was false for the seven between
  census: python3 over CHANGELOG.md's id lines, counting pairs where the later id is the larger one
          -> 74 ids, 1 out-of-order pair before the correction, 0 after
  ```

  ⭐ So the insertion point was "just after the first entry" and not "at the top", and it stayed wrong for
  four commits after the first because **the file looked the same each time** — a wrong anchor that
  produces a plausible file is self-concealing, which is the property the sixth shape had (an index row
  that reads correctly and names a closed leaf). Corrected in `ARCHOGEN-PROGRAM-0101` as a **pure move**,
  verified two ways rather than by eye: the multiset of lines is unchanged
  (`collections.Counter(new) == collections.Counter(lines)` in the script that moved it) and the inversion
  census above went **1 → 0** over all 74 ids, so no second misplacement was left behind.
  ⛔ **What this instance adds, and it is not the sequence.** The acceptance below enumerates three live
  surfaces — book chapters, corpus indexes, crate module headers — and this defect is in a fourth,
  `CHANGELOG.md`, which that acceptance names only as a place a figure may legitimately be *"registered as
  a record"*. The fifth shape was the first falsification of the enumerated population (a normative
  document, `docs/semantics/grammar.md`); this is the second. Two falsifications of a three-item list is
  the measurement that ends the list.

  Each sweep is a population bounded by its pattern, so "nothing else found" has never been a result
  this repository could rely on — and a third sweep, adding `leaves|arms|checks|doctrines|productions|rows`,
  returned a further backlog of structure counts this leaf deliberately does **not** classify, because
  classifying five shapes by hand is the work the register exists to end.
  Acceptance: a registered check enumerates figure-shaped text in the **live** surfaces (book
  chapters, corpus indexes, crate module headers) and classifies each occurrence as *gated* (a named
  test compares it to a measurement), *registered as a record* (history, changelog, closed leaf), or
  *unregistered*; it exits nonzero on an unregistered figure introduced by the **staged diff**, in the
  `TABLE-ARITY-RATCHET` idiom — a per-file ratchet against `HEAD`, so the pre-existing population is a
  reported backlog rather than a reason to bypass; `--self-test` carries RED arms including the two
  shapes already measured (a digits figure and a spelled-out one); the gates `M1.23` and `M1.24` built
  are registered as consumers so they do not trip it; and the honest limit is stated in the check's own
  header — it proves a figure is *watched or listed*, never that a listed one is correct, which is
  `BOOK-ANCHORS`' limit one level down.
  ⚠️ **Widened `2026-09-29` on the sixth shape.** Everything above enumerates *figure-shaped text*, and
  the shape that just fired is an ordered **sequence**: an index row naming a leaf whose own status is
  `done`, and a successor clause naming the frontier as its own successor. Neither contains a number
  that moved, so a digits-or-spelled-out census passes the commit that produces them. The registered
  check must therefore also compare a restated list head against the tree file it indexes (and a
  successor clause against the head it follows), and `--self-test` must carry an arm whose subject is a
  sequence rather than a count — priced here rather than discovered by the seventh shape.
  ⚠️ **Widened again `2026-09-29` on the eighth shape — the population, not the shape.** The three
  enumerated live surfaces above have now been falsified twice from outside (a normative document, then
  the changelog), and both times by a defect that was in the repository's *own* discipline surfaces rather
  than in anything the engine reads. So the check's population is **every live surface the commit stages**,
  derived from the staged diff the way `TABLE-ARITY-RATCHET` and `GAP-CLAIM-CENSUS` derive theirs, and not
  a list of three; a list is the thing that keeps being wrong. The ordered-sequence arm the sixth shape
  priced now has a second subject to be written against — a document that states its own ordering rule in
  its header, where the rule is checkable without knowing what the entries mean.
  ⛔ **Two more instances of the seventh shape, `2026-09-29`, routed in by `M1.31`** (its ROUTING EVIDENCE
  section in `docs/tasks/M1.md`), and with them a **candidate instrument** for the transcript half of the
  register, because it is the first one that has run over the whole population. `M1.31` needed to know which
  book transcripts a renderer change would move, so it re-ran every `error[…]` block in `docs/book/src/*.md`
  whose `-->` names a **tracked** file through `archogen check` and compared it verbatim:

  ```text
  DIFF        docs/book/src/checking.md:93  examples/alternative-timer/system.eadl
              -> the hint is wrapped onto two lines, and the command's final
                 `archogen: infeasible-configuration: 1 diagnostic(s) in …` line is omitted
  no -->      docs/book/src/checking.md:115 "Refused by name" (examples/bounded-queue/system.eadl)
              -> the book shows the error line and the hint; the command also prints the
                 `--> examples/bounded-queue/system.eadl:36:10` location, the source line, the marker
                 line and the final verdict line. Nothing marks the transcript as abridged.
  the whole population, measured at 312b17d: 20 rendered `error[…]` blocks in the book
              5 over a tracked input          (checkable: 4 verbatim, 1 DIFF above)
              7 over an untracked input       (t.eadl, system.eadl, dup.eadl, examples/time.eadl,
                                               examples/sensor.eadl ×2, build/s0/malformed.eadl) — no
                                               command can re-run them
              8 with no `-->` at all          (checking.md:115, modules.md:91 :127, presence.md:36 :53,
                                               refinement.md:32 :63, workload.md:68) — a real diagnostic
                                               always prints one, so each is abridged or was never a run
  ```

  So the instrument measures one rule and exposes the other: **15 of the 20** blocks cannot be checked at
  all, and the rule "name an input a reader can rebuild" is what makes the first rule enforceable. The
  second instance also shows that keying the population on `-->` misses a transcript whose abridgement
  removed the `-->` itself — eight blocks are invisible to it — so the population has to be keyed on the
  `error[` line or the `$ archogen …` line that opens the block, not on a line an abridgement may drop. Not
  fixed here: the two `checking.md` blocks are this leaf's to re-render when the register lands, or sooner by
  any leaf that touches that chapter.
  ⭐ **Re-measured at `0a19c36`, with the population keyed on the `$ archogen check` line as proposed above:**
  **9** blocks over a tracked input (7 verbatim, the two `checking.md` blocks still the only DIFFs), **6** over
  an untracked input, **5** with no location — 11 of 20 uncheckable, down from 15. The movement is `M1.31`'s
  new `reading.md` block and `M1.29.2`'s three `modules.md` transcripts, re-rendered by the command over
  `docs/semantics/modules/` once a command could reach them: they retired one untracked input (`dup.eadl`)
  and two location-free blocks. Keying on the command line also brought `checking.md:115` into the checkable
  population, where it shows as the DIFF it is. The instrument, so it can be rebuilt: for every `error[` block
  in `docs/book/src/*.md`, take the file from the `$ archogen check <file>` line above it (else from its first
  `-->`), keep it if `git ls-files` tracks it, run `archogen check <file>`, and require the block to appear
  verbatim in stderr. It was run by hand from scratch; making it a tracked, armed check is this leaf's work.
  Priority: **medium** — it is the structural fix for the most frequently recurring defect class in
  this repository, and it is what stops `M1.23`/`M1.24`/`S0.8` from being followed by an `M2.x`. It
  gates no milestone and blocks nothing, which is why it is scheduled behind `PROGRAM.11` and
  `PROGRAM.18` rather than ahead of them; `PROGRAM.18` (repeatable RED arms for registered controls)
  should land first, because this check arrives with arms and the older ten do not.
  Verification: closed `2026-09-30` by its three children, each an instrument for a different shape of the class:
  **orders** (`STATED-ORDER`, `.20.1` — its first run found `M2.9` stated two ways), **transcripts** (`book_transcripts.rs`,
  `.20.2` — 5 of 12 checkable transcripts differed, two contradicting their own prose), **figures** (`FIGURE-REGISTER`,
  `.20.3` — a ratchet at the commit that adds one; its first run refused a false "gated" claim). The acceptance's figure
  register exists, registered and armed, with the population the staged live documents rather than a list of three.
  Commit: `ARCHOGEN-PROGRAM-0142` (`.20.1`), `ARCHOGEN-PROGRAM-0143` (`.20.2`), `ARCHOGEN-PROGRAM-0144` (`.20.3`)
  Children: `PROGRAM.20.1`, `PROGRAM.20.2`, `PROGRAM.20.3` — decomposed `2026-09-30`, because the shapes above need three
  different instruments, and one check trying to be all three is the "list that keeps being wrong" again:
  **sequences** (a restated frontier head, a successor clause, a ledger's own ordering rule) are exact and derivable —
  the adopted `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` calls them *verified copies* and requires an executed verifier;
  **transcripts** are exact too, by re-running the command a block quotes; **figures** are the fuzzy residue.

- ID: `PROGRAM.20.1`
  Status: `done`
  Goal: every restated frontier head and successor list is a verified copy of its tree, and the changelog keeps the
  order its own header states — the sixth and eighth shapes, closed by a verifier rather than a sweep.
  Acceptance: a registered check reads each snapshot's restated heads (`LIVE_STATUS.md`, `docs/TASK_TREE.md`,
  `MEMORY.md`) and refuses one that differs from its tree's Current Frontier order-1 leaf or names a `done` leaf; a
  successor list must equal the frontier's following order; `CHANGELOG.md`'s entry ids must be newest-first; RED arms
  include a head left behind by a closure, a successor list naming its own head, and an entry inserted below a newer
  one.
  Verification: see the checklist — a gate over four legs (10 arms, 7 mutations), each real snapshot falsified, and
  one live inconsistency found and corrected in its own commit.
  Commit: `ARCHOGEN-PROGRAM-0142 (leaf PROGRAM.20.1)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the sixth and eighth shapes above, and one more found live on the gate's first run:
    ```text
    $ git grep -n 'Status: `in_progress`' 29c609c -- docs/tasks/M2.md
      29c609c:docs/tasks/M2.md:174:  Status: `in_progress` — checkpointed on branch `wip/m2.9` …
    $ git grep -n '| 4 | `M2.9` | `blocked`' 29c609c -- docs/tasks/M2.md
      29c609c:docs/tasks/M2.md:633:| 4 | `M2.9` | `blocked` | …
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — nothing compared a restated order with its source; the only scripts that read a
    frontier use it as a boundary, not a claim:
    ```text
    $ git grep -ln "Current Frontier" 29c609c -- scripts/
      29c609c:scripts/bootstrap.sh              (seeds a tree)
      29c609c:scripts/check_task_acceptance.sh  (ends a leaf block at it)
    ```
  - [x] **FIX** — `scripts/check_stated_order.sh` (**`STATED-ORDER`**): frontier rows against their leaves' statuses; the
    heads restated in `LIVE_STATUS.md`, `docs/TASK_TREE.md` and `MEMORY.md` against each tree's order-1 leaf (a next action
    may be its child) and never `done`; `Then …` successor lists against the frontier's order and never naming their own
    head; `CHANGELOG.md` strictly newest-first. `M2.9`'s status corrected to `blocked` in its own docs-only commit
    (`ARCHOGEN-M2-0141`, `23febb2`), this leaf's work parked in `git stash` meanwhile so the tree was clean at the pivot.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/check_stated_order.sh          (first run, before the M2.9 correction)
      STATED-ORDER: docs/tasks/M2.md: its frontier says `M2.9` is `blocked`, and the leaf says `in_progress`     exit=1
    $ bash scripts/check_stated_order.sh          (after)
      stated-order: OK (every frontier table, restated head, successor list and the changelog's order agree …)  exit=0
    $ bash scripts/check_stated_order.sh --self-test
      stated-order self-test: 10 pass / 0 fail (10 arms)
    ```
    Each real snapshot falsified and restored by `cmp`: `LIVE_STATUS.md`'s, `docs/TASK_TREE.md`'s and `MEMORY.md`'s head
    moved → each refused naming both sides, so none of the three passes vacuously. Seven mutations O-1–O-7, each restored
    by `cmp`, each fails its own arm.
  - [x] **NO REGRESSION** — the changelog leg on the real file: 103 entries with ids, 0 out-of-order pairs; the doctrine
    driver green at the commit; `BOOK-ANCHORS` exit=0 over the new book section.
  - [x] **LOCKSTEP** — `verification.md` gains "\"Next\" means what the task tree says"; `DOCTRINE_ENFORCEMENT.md`,
    `TOOLBOX.md`.

- ID: `PROGRAM.20.2`
  Status: `done`
  Goal: every rendered diagnostic in the book that names a tracked input is what the command prints — the seventh
  shape, by the instrument recorded above, tracked and armed.
  Acceptance: for each `error[` block, the input from its `$ archogen check` line (else its first `-->`); a tracked one
  re-run and required verbatim; an untracked or location-free block counted as backlog that may not grow; the two
  `checking.md` blocks re-rendered.
  Verification: see the checklist — the transcript test, five blocks re-rendered from runs, the backlog ratchet both
  ways, a permanent catalog entry.
  Commit: `ARCHOGEN-PROGRAM-0143 (leaf PROGRAM.20.2)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the test's first run, before any block was touched:
    ```text
    $ cargo test -q -p archogen-cli --test book_transcripts
      5 of 12 checked transcript(s) disagree with the tool:
      checking.md:44 · checking.md:109 · checking.md:131 · modules.md:152 · reading.md:122
      test result: FAILED. 0 passed; 1 failed
    ```
    Two of them (`checking.md:44`, `modules.md:152`) dropped the secondary "first declared here" label — the very label the
    prose around each says is there ("with the first declaration named"; "with both sites named, in the two files they
    are in"); the other three re-wrapped a hint or dropped the final summary line. ⚠️ The hand instrument (a substring
    match) had counted `reading.md:122` as verbatim; exact equality is what caught its missing summary line.
  - [x] **ROOT CAUSE (WHY + WHERE)** — the tests that read the book read its figures, sources and tables, never its
    transcripts:
    ```text
    $ git grep -ln "docs/book/src" f72b50e -- 'crates/*/tests/*.rs'      (then each file's uses)
      corpus.rs: a corpus size in reading.md · kinds.rs: kinds.md's tables · semantic_corpus.rs: checking.md's corpus size
      module_files.rs: modules.md's opening example source · module_cases.rs: the names modules.md shows · reference.rs: tables
    $ git grep -n "error\[" f72b50e -- 'crates/*/tests/*.rs' | grep -c book        -> 0
    ```
  - [x] **FIX** — `crates/archogen-cli/tests/book_transcripts.rs`: a block with an `error[` line is a transcript; one opened
    by `$ archogen check <file>` must **equal** the in-process run (with the exit code when it echoes `$?`), one keyed by
    `-->` must appear verbatim; a transcript over an input no reader can rebuild is a backlog of 11 that may shrink, not
    grow. The five blocks re-rendered from runs of the current binary.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ cargo test -q -p archogen-cli --test book_transcripts          -> test result: ok. 1 passed; 0 failed
    a new block over nowhere.eadl → "new transcript(s) over an input no reader can rebuild: [\"presence.md:107\"]"
    a stale backlog entry        → "backlog entries that are no longer there: [\"gone.md:1\"]"          (each restored by `cmp`)
    $ cargo xtask mutate --only renderer-drops-secondary-labels
      ✓ renderer-drops-secondary-labels  killed by every_rendered_diagnostic_in_the_book_is_what_the_command_prints
    ```
  - [x] **NO REGRESSION** — `mdbook build docs/book` exit=0; `BOOK-ANCHORS` exit=0; the test takes 0.01 s and is ignored
    under Miri with the other corpus walks; the doctrine driver green at the commit.
  - [x] **LOCKSTEP** — `verification.md` gains "Every diagnostic shown in this book is a real run" — ⚠️ written first with
    "thirteen checkable examples" against the test's own `5 of 12`, and corrected to twelve before the commit: this leaf's
    own class, caught by reading the output back; the catalog's thirteenth entry.

- ID: `PROGRAM.20.3`
  Status: `done`
  Goal: the figure register itself — figure-shaped text in the staged live surfaces classified as gated, recorded or
  unregistered, with an unregistered one introduced by the staged diff refused (`TABLE-ARITY-RATCHET` idiom).
  Acceptance: as the parent's acceptance above, for figures; the population is every live surface the commit stages.
  Verification: see the checklist — a ratchet gate over staged live documents (14 arms, 8 mutations), a register
  that starts empty because its first row was false, and the `S0.8` shape refused on the real tree.
  Commit: `ARCHOGEN-PROGRAM-0144 (leaf PROGRAM.20.3)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — a census of the live surfaces with the gate's own pattern, `2026-09-30`: **96**
    figure-shaped phrases over 28 files, roughly half real carried figures (`five tiers`, `541 tests`, the `S0.8`
    `Three descriptions`, `eleven trees`) and half noise (`§7 rule 5`, `two descriptions that differ`). Then, on the
    real tree, the `S0.8` shape added to a staged chapter:
    ```text
    $ printf '\nThe directory holds three descriptions.\n' >> docs/book/src/s0.md; git add …; bash scripts/check_figure_register.sh
      FIGURE-REGISTER: docs/book/src/s0.md adds a figure no row of docs/figures.md classifies: three descriptions …   exit=1
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — nothing looked at a figure when it was *added*; each fix was a sweep afterwards,
    bounded by its pattern. No gate existed:
    ```text
    $ git grep -n "figure" e9ae521 -- 'scripts/check_*.sh'
      e9ae521:scripts/check_book_anchors.sh:55:# than the language, and its own figures are gated by `corpus.rs`.
      e9ae521:scripts/check_book_anchors.sh:98:  # below has always reported, so the figure does not move when the loop is restructured.
      e9ae521:scripts/check_stated_order.sh:5:# ⭐ WHY THIS EXISTS. The defect class `PROGRAM.20` records has a shape that no figure-shaped …
    ```
    Three mentions, all in comments — none a gate on figure-shaped text. ⚠️ This box was first written as "no match,
    rc=1" before the command was re-run; the run said otherwise, and the box now carries its output.
  - [x] **FIX** — `scripts/check_figure_register.sh` (**`FIGURE-REGISTER`**): per staged live document, unclassified
    figure-shaped phrases may not rise against `HEAD`; `docs/figures.md` classifies a phrase `gated` (a test that must
    exist and read the file), `record` (dated) or `not-a-count`, stale rows refused. Exempt by construction: code fences
    (`PROGRAM.20.2` owns transcripts), code spans, `§N` references, a line carrying its measurement date. The gates
    `M1.23`/`M1.24` built measure figures written in code spans, so they cannot trip it — measured, not assumed: their
    figures are backticked (`corpus.rs:461` "states a corpus of `21`").
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/check_figure_register.sh --self-test
      figure-register self-test: 14 pass / 0 fail (14 arms)
    $ bash scripts/check_figure_register.sh        (this commit staged)
      figure-register: OK (no staged live document adds an unclassified figure; 14 … are backlog)   exit=0
    ```
    ⛔ Its first real run refused this leaf's own first register row — `five tiers` in `verification.md` "gated by"
    `xtask`'s tier test, which compares the runner with the roadmap and never reads the book; the row was a false claim,
    and the register starts empty. And writing the book section, it refused `three rules` — a quotation, moved into a
    code span. Eight mutations, each restored by `cmp`: F-1 spelled-out shape dropped (7 fail), F-2 the dated-line
    exemption (1), F-3 code spans read (1), F-4 the ratchet disabled (3), F-5 a gate that never reads the file accepted
    (1), F-6 stale rows accepted (1), F-7b the `§` lookbehind dropped from the count-noun pattern (1). F-7 (the same on
    the `N of M` pattern) survives: no arm has a `§N of M`, and no live text does.
  - [x] **NO REGRESSION** — the doctrine driver green at the commit, now 13.3 s wall; `BOOK-ANCHORS` exit=0.
  - [x] **LOCKSTEP** — `verification.md` gains "A number added to this book says what keeps it true", itself passing the
    gate; `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`.
