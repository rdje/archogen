- ID: `PROGRAM.21`
  Status: `done`
  Goal: **`TASK-ACCEPTANCE` examines only the first acceptance checklist in a tree file, so for every
  leaf after the first it verifies nothing** — and prints a success message asserting the opposite.
  Make the check leaf-scoped, and arm it.
  Reproduce / issue: `scripts/check_task_acceptance.sh` runs one `awk` per staged `docs/tasks/*.md`.
  That awk sets `inbox=1` on the first bullet matching the keyword and `exit`s at the **next** box
  bullet, so exactly one box per keyword per file is ever read. Measured rather than inferred from the
  source: `docs/tasks/M1.md` carries **24** ticked ROOT CAUSE boxes
  (`grep -cE '^[[:space:]]*-[[:space:]]*\[[xX]\][[:space:]]*\*\*ROOT CAUSE' docs/tasks/M1.md` → `24`),
  and the check's own awk against that file captures **line 52** — leaf `M1.1`, written `2026-09-13`.
  Then, with `crates/eadl-front/tests/corpus.rs` staged and leaf `M1.24` carrying **zero** boxes
  (`awk '/^- ID: `M1.24`/,/^## Current Frontier/' docs/tasks/M1.md | grep -cE '^\s*- \[[ x]\] …'` →
  `0`), the gate printed
  `task-acceptance: OK (every staged code-change leaf carries a ticked, evidence-backed checklist)`
  and `exit=0`.
  ⭐ **Severity as first measured: latent, not active.** Auditing every leaf that records a
  commit found five with no ticked ROOT CAUSE box — `M0.1`, `M0.2`, `M1.18`, `M1.20`, `PROGRAM.1` — and
  `git show --stat` on each of their commits reports **0** code files: `M1.18` staged twelve `.md`
  files, `M0.1`/`M0.2`/`PROGRAM.1` none, and `M1.20` is an aggregation node whose seven sub-leaves
  each carry their own checklist. So no code change had landed unboxed. The defect was that the gate
  **could not have stopped one** on any leaf but the first in its file, while telling the author it had.

  ⛔ **That severity claim is SUPERSEDED `2026-09-28`: the defect is active, and this is measured rather
  than inferred.** Commit `cd355ef` (`ARCHOGEN-M1-0080`, leaf `M1.13.1`) staged five Rust files —
  `crates/eadl-front/src/form.rs`, `crates/eadl-front/src/reader.rs` and three test files — and the gate
  printed `task-acceptance: OK (every staged code-change leaf carries a ticked, evidence-backed
  checklist)` with `exit=0`. What it actually read was **`docs/tasks/M1.md` line 53**, inside leaf
  **`M1.1`**, written `2026-09-13`. Three measurements, all re-runnable:

  ```text
  census: grep -cE '^[[:space:]]*-[[:space:]]*\[[xX]\][[:space:]]*\*\*ROOT CAUSE' docs/tasks/M1.md
          → 31 ticked ROOT CAUSE boxes in the file (24 when this leaf was written); exactly one is read
  census: the check's own awk over docs/tasks/M1.md with kw="root.?cause"
          → captures line 53, ticked=1 — leaf M1.1's box, not M1.13.1's at line ~1470
  mutation: M1.13.1's ROOT CAUSE box unticked in a scratch copy, the same awk re-run over both files
          → identical capture, line 53 ticked=1 in BOTH, so M1.13.1's boxes cannot affect the verdict
  ```

  ⚠️ **The mutation is the finding, so read it before reading the reassurance.** Nothing bad happened in
  `cd355ef`: `M1.13.1`'s checklist was written, ticked and evidence-backed, so the commit was honest and
  the verdict was right **for the wrong reason**. Had those boxes been empty the verdict would have been
  byte-identical, which means the gate supplied that commit no protection and told its author it had.
  Every code commit on every leaf of `M1.md` after `M1.1` sits in the same position — 30 of the file's
  31 boxes belong to leaves the gate never reads — and `M1.md` is the tree the project is working in.
  ⛔ Interim mitigation, stated so it is not mistaken for a fix: there is none inside the gate. The only
  thing standing between this and an unboxed code commit is the author's own discipline, which is
  precisely what a gate exists to not depend on.
  ⛔ That is the same failure mode the check's own header says box-scoping was introduced to end. It
  closed cross-**file** leakage and incidental-**prose** leakage, and left cross-**leaf** leakage open —
  a co-staged tree file supplying another leaf's evidence, one directory level down from the hole it
  was written for.
  Acceptance: the check verifies the boxes of the leaf that **owns** the staged change rather than the
  first leaf in the file; its success message is true of what it actually examined; a staged code change
  whose owning leaf carries no ticked, evidence-backed boxes is refused **with that leaf's ID in the
  message**; `--self-test` carries RED arms including (a) a *second* leaf in a file with no boxes while
  the first is complete — the exact shape measured here, (b) a file whose only leaf is complete, and
  (c) the `TEMPLATE.md` exclusion still holding; and the honest limit is stated in the header,
  including that the owning leaf is **not always identifiable from staged paths alone** (the commit
  message names it, and `pre-commit` does not have one) and what the check does when it cannot tell.
  Priority: **high** — this is the gate enforcing root-cause / effect / no-regression evidence on every
  code change; it is unsound for the majority of leaves in every multi-leaf tree file (`M1.md` 33
  leaves, `PROGRAM.md` 23, `M2.md` 10, `S0.md` 9); and a false success message is worse than a silent
  one, because it teaches an author that the box they skipped did not matter. Scheduled **ahead of
  `PROGRAM.18`**, which would otherwise spend effort arming a control that reads the wrong boxes —
  `PROGRAM.18` keeps the other nine.

  ### Design pinned by measurement `2026-09-29` — implementation NOT started, and this is the resume point

  ⛔ **The obvious design is dead, measured rather than argued.** Identifying the owning leaf from
  *which leaf sections a commit's tree-file diff touches* was priced against the last seven code
  commits, comparing the touched set with the `(leaf X)` token in each subject:
  ```text
  3a6bbb9 M1.13.2 → touched M1.13,M1.13.2                  DISAGREE
  cd355ef M1.13.1 → touched M1.13.1,M1.26                   DISAGREE
  e4212c4 M1.12.5 → touched M1,M1.12,M1.12.5,M1.26,PROGRAM.24  DISAGREE
  64e35c4 M1.12.4 → touched M1.12.4                         agree
  951f6c0 M1.12.3 → touched M1.12.3,M1.12.4,M1.12.5         DISAGREE
  690f5c6 M1.12.2 → touched M1.12.2,PROGRAM.20              DISAGREE
  29154da M1.12.1 → touched M1.12.1,M1.12.2                 DISAGREE
  → agree 1 / disagree 6 / no-map 0
  ```
  A commit legitimately touches its own leaf, its parent, the frontier, and every leaf it routes a
  finding to, so the touched set is not the owner and narrowing it (e.g. "touched sections that carry a
  checklist") still leaves `M1.13.2` and `M1.13` both live. **Do not re-derive this; it is measured.**

  ⭐ **The chosen identification chain, and why it fails closed:**
  1. `TASK_ACCEPTANCE_LEAF=<ID>` — for a commit made with no message file (an IDE dialog, `-m`).
  2. the **subject** of the pending commit message, via a new seam `.doctrine/commit_message_file`
     (one line, default `git_message_brief.txt`) so the template stays project-neutral. `COMMIT.md`
     step 4 already writes that file before step 6's `git commit -F`, so it exists at `pre-commit` time.
  3. ⛔ **otherwise REFUSE.** Do not fall back to the first checklist in the file — that fallback *is*
     the defect. A green verdict about a leaf nobody claimed is worse than no verdict, because the
     author reads it as protection. The refusal must name what it tried and both ways to satisfy it.
     Consequence to state in the header: `make gate` with staged code and no message file now fails,
     which enforces the mandated workflow rather than quietly excusing its absence.

  **Scoping mechanism:** slice the leaf by *exact string* match on ``- ID: `<owner>` `` (not a regular
  expression — an id's `.` would match another leaf's), running to the next flush-left `- ID: ` or
  `## `, then run the existing three-box awk over that slice alone. **Success message must name what it
  read** — `leaf <ID> in <file>` — because criterion 2 is that the message be true of what was examined.
  Also refuse when the named leaf is in **no staged** tree file, which catches a stale message file
  left over from a previous commit.

  **`--self-test` arms, each in a throwaway `git init` repo under `mktemp -d` so the real index is never
  touched** (the house pattern in `check_book_anchors.sh` cannot be copied directly, because this check
  reads `git diff --cached`): (1) first leaf complete, second leaf owns the change and has no checklist
  — the measured defect — must **fail** naming the second; (2) same fixture, complete leaf named, must
  **pass**, proving arm 1 fails for the right reason; (3) named leaf with unticked boxes fails;
  (4) no owner identifiable fails closed; (5) a stale message file naming an unstaged leaf fails;
  (6) the env override beats the message file; (7) `TEMPLATE.md` staged beside a complete leaf still
  passes; (8) code with no leaf staged still fails with `NO owning task-tree leaf`; (9) the OK message
  names the leaf it read.

  ⚠️ **Two traps for whoever resumes.** (a) The new check gates **its own** commit: the hook runs the
  working-tree script, so this leaf must carry ticked, evidence-backed `ROOT CAUSE` / `ADDRESSED` /
  `NO REGRESSION` boxes before the commit can land — which is the right self-test, not an obstacle.
  (b) `ADDRESSED` should re-run the two historical scenarios (`cd355ef`, `3a6bbb9`) as fixtures rather
  than asserting from memory, since those are the commits the defect was measured on.

  ⛔ **A full draft was written and then REVERTED, deliberately, rather than committed unverified.**
  It parsed (`bash -n` clean) but its `--self-test` was never run, and committing an untested gate that
  fails closed would have blocked every subsequent code commit. A copy sits at
  `target/tmp/p21/check_task_acceptance.draft.sh` — **scratch, and the next artifact cleanup will
  delete it**, so the design above is the durable artifact and the draft is only a head start.
  `scripts/check_task_acceptance.sh` is byte-identical to `HEAD` (`diff -q` against
  `git show HEAD:…` silent). Lockstep owed with the fix: `DOCTRINE_ENFORCEMENT.md` §4's description of
  this control, `.doctrine/README.md` for the new seam, `TOOLBOX.md`, and `PROGRAM.18`'s note that ten
  controls lack a RED arm (this one gains nine).
  ✅ **That checkpoint is superseded the same day: the draft was installed, its arms were run, and the
  fix landed.** The lockstep listed above is discharged in this commit, and the design was followed
  except where measurement corrected it — see the NO REGRESSION box, where the arms' own oracle turned
  out to be unsound in the same way the check was.

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE` — which, from this commit, enforces it *here*)

  - [x] **ROOT CAUSE (WHY + WHERE)** — **WHERE:** `scripts/check_task_acceptance.sh`, the per-keyword
    `awk` that extracts a box's bullet: `if (inbox) exit` stops at the *next* box bullet, and the awk is
    run once over the whole tree **file**, so a file holding N leaves yields exactly one box per keyword
    — whichever leaf comes first. **WHY it survived:** the check had already been hardened twice, for
    cross-**file** and incidental-**prose** leakage, and its own header claimed box-scoping was "the
    soundness property"; cross-**leaf** leakage is the same hole one directory level down, and the
    success message asserted the opposite of what was examined, so a green run taught the author the
    box they skipped did not matter. Measured at `HEAD`, not recalled:
    ```text
    $ git show HEAD:scripts/check_task_acceptance.sh | grep -n "if (inbox) exit"
    111:          if (inbox) exit
    $ grep -cE '^[[:space:]]*-[[:space:]]*\[[xX]\][[:space:]]*\*\*ROOT CAUSE' docs/tasks/M1.md
    32                                        ← 32 ticked boxes in the file; the awk reads ONE
    $ <HEAD's own awk, kw="root.?cause", over docs/tasks/M1.md>
    53:   - [x] **ROOT CAUSE (WHY + WHERE)** — §12 M1's exit gate is that "invalid examples produce
                                                ↑ leaf M1.1, written 2026-09-13
    ```
    ⛔ **And the fix's premise was priced before it was chosen.** Identifying the owner from the leaf
    sections a commit touches — the only signal inside the staged paths — was measured against seven
    real code commits and agrees with the `(leaf X)` subject token on **1 of 7** (`3a6bbb9` touched
    `M1.13`+`M1.13.2`; `e4212c4` touched five leaves across two trees). So the owner comes from the
    author's own declaration and the check refuses when there is none; the table is in the design
    section above.
  - [x] **FIX** — `scripts/check_task_acceptance.sh` rewritten leaf-scoped, plus the new seam
    `.doctrine/commit_message_file`. (1) The owner is `TASK_ACCEPTANCE_LEAF`, else the `(leaf <ID>)`
    token in the pending message's **subject**, else **refuse** — no fallback to the first checklist,
    because that fallback *is* the defect. (2) The leaf's section is sliced by **exact string** match on
    ``- ID: `<owner>` `` (not a regular expression, so an id's `.` cannot match another leaf's) running
    to the next flush-left `- ID: ` or `## `, and the three-box awk now runs over that slice alone.
    (3) A named leaf found in **no staged** tree file is refused, which catches a stale message file.
    (4) The OK message names the leaf and the file it read. (5) `--self-test` with nine RED arms, each in
    a throwaway `git init` repo under `mktemp -d`, because the house pattern in `check_book_anchors.sh`
    cannot be copied by a check that reads `git diff --cached`.
  - [x] **ADDRESSED (verified)** — both commits the defect was measured on, **replayed as fixtures**
    rather than asserted from memory: each commit's own `docs/tasks/M1.md` and one of its staged source
    files extracted with `git show <commit>:<path>` into a throwaway repo, its `.doctrine/` seams copied,
    and a brief naming the leaf that commit claimed. Then the same fixture with that leaf's ROOT CAUSE
    box unticked. `HEAD`'s check and the new one run over both:
    ```text
    cd355ef  pristine  OLD exit=0 OK | NEW exit=0  → "OK (leaf M1.13.1 in docs/tasks/M1.md — …)"
    cd355ef  mutated   OLD exit=0 OK | NEW exit=1  → "leaf M1.13.1 — the 'ROOT CAUSE' box is present
                                                      but NOT ticked."
    3a6bbb9  pristine  OLD exit=0 OK | NEW exit=0  → "OK (leaf M1.13.2 in docs/tasks/M1.md — …)"
    3a6bbb9  mutated   OLD exit=0 OK | NEW exit=1  → "leaf M1.13.2 — the 'ROOT CAUSE' box is present
                                                      but NOT ticked."
    ```
    ⭐ Read the `mutated` rows as the finding: the OLD verdict is **byte-identical** pristine and
    mutated, so those two commits' own boxes provably could not change it, while the NEW verdict moves
    and names the right leaf. Both leaves were honest, which is why `pristine` passes on both — the
    defect was never a bad commit, it was a gate that supplied no protection and said it had.
    `bash scripts/check_task_acceptance.sh --self-test` → **9 pass / 0 fail**, `exit=0`.
    `bash -n` clean; `make gate` → `=== all doctrines green ===` over 13 checks; `make focused` run
    below. ⚠️ This commit is the new check's first real exercise: it gates itself, so the verdict it
    prints is about *this* leaf's boxes and not `PROGRAM.1`'s.
    ⭐ **And it refused this very commit before the brief was written** — the fail-closed path exercised
    live rather than only in a fixture, with the real staged set (11 files, `scripts/…` among them):
    ```text
    $ git add <the 11 files> && bash scripts/check_doctrines.sh      # git_message_brief.txt still empty
      ❌ TASK-ACCEPTANCE
       TASK-ACCEPTANCE: a CODE change is staged but the check CANNOT TELL WHICH LEAF owns it.
         It does not guess. Falling back to the first checklist in the file is how this check
         came to print OK having read a different leaf's boxes, and a green verdict about a leaf
         nobody claimed is worse than no verdict: the author reads it as protection.
         Name the owning leaf either way:
           • write the commit message to 'git_message_brief.txt' with '(leaf <ID>)' in its SUBJECT, or
           • TASK_ACCEPTANCE_LEAF=<ID> <your commit command>
         tried: TASK_ACCEPTANCE_LEAF (unset), 'git_message_brief.txt' (exists but its subject names no leaf)
    ```
    `make focused` → `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built`.
  - [x] **NO REGRESSION** — three mutations, each restoration proven byte-identical with `diff -q`
    against the saved copy (`target/tmp/p21/new.sh.orig`), per
    `docs/knowledge/verify-the-mutation-applied.md`:
    ```text
    mutation A: replace the fail-closed refusal with the OLD behaviour — take the first leaf in the file
      → self-test `8 pass / 1 fail`: arm 4 (no owner identifiable) goes green, so the refusal is
        load-bearing and an arm proves it
    mutation B: point the arms' invocation at a file that does not exist, exact oracle kept
      → self-test `0 pass / 9 fail`: every arm reports that the check never ran
    mutation B2: the same break, with the oracle weakened from `rc -eq 1 && grep TASK-ACCEPTANCE` to
      `rc -ne 0`
      → self-test `4 pass / 5 fail`, and the four ✅ are arms 1/3/4/5 **reported as passes on
        `exit 127`** — the false green reproduced deliberately
    restored: `diff -q` silent; self-test `9 pass / 0 fail`, `exit=0`
    ```
    ⛔ **B2 is the reason this leaf found a second defect, in its own new code.** The arms as first
    written used `rc -ne 0` as the pass condition, and their first run scored **4 passes on `exit 127`**
    — `$0` was a relative path and every arm `cd`s into a throwaway repo, so the check was never found.
    A refusal and a failure to exec are different claims, and an oracle that accepts "not success"
    cannot tell them apart; the tally still read like partial success. Fixed by requiring the subject's
    **own** exit code and its **own** identifying output, and by resolving the invocation path against
    the directory the caller stood in. Promoted into `docs/knowledge/verify-the-mutation-applied.md`,
    which now names all three parts of an arm that can be weaker than the property: the needle, the
    mutation, and the oracle.
  - [x] **LOCKSTEP** — `scripts/check_task_acceptance.sh` (header now names three leakage holes, the
    leaf-scoping rationale, the fail-closed rule and its three honest limits);
    `.doctrine/commit_message_file` (new seam, with the 1-of-7 measurement as its reason);
    `.doctrine/README.md` (the seam table, "in both" → "in all three"); `DOCTRINE_ENFORCEMENT.md` §4's
    `TASK-ACCEPTANCE` row (two holes → three, the identification rule, the refusal, nine arms);
    `TOOLBOX.md` (a row for the check and its `--self-test`);
    `docs/knowledge/verify-the-mutation-applied.md` (new section + `answers:` line + a How-to-apply
    bullet); this leaf, this tree's frontier and both logs; `MEMORY.md`, `LIVE_STATUS.md`,
    `CHANGELOG.md`, `DEV_NOTES.md`. ⛔ `KNOWLEDGE_MAP.md` was regenerated and came out **unchanged** —
    `git diff --stat HEAD -- KNOWLEDGE_MAP.md` empty, and `grep -c` for the new `answers:` line in the
    map returns 0, so the map does not index a card's `answers:` and adding one moves nothing. Recorded
    rather than claimed, because "regenerated the map" reads as "the map changed". ⭐ `PROGRAM.18`'s
    population moves: it counted ten registered controls with no repeatable RED arm,
    `check_task_acceptance.sh` among them — this one now has nine, so `PROGRAM.18` keeps the other nine
    and must re-run its census rather than reuse the figure.
  Verification: see the acceptance checklist above.
  Commit: `ARCHOGEN-PROGRAM-0086 (leaf PROGRAM.21)`
