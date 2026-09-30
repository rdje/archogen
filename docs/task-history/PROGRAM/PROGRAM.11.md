- ID: `PROGRAM.11`
  Status: `done`
  Goal: state the **repository boundary in both directions** where a resuming agent actually reads
  it, and gate the part that is observable from inside this repository: no archogen agent writes
  into another repository or a vendored submodule's own history (**outbound**), and a change
  delivered into this repository by another project's agent lands through a leaf that records its
  authorization, its file list and what it preserved (**inbound**).
  Acceptance: both directions are stated in `CLAUDE.md`/`AGENTS.md` and `DOCTRINE_ENFORCEMENT.md`,
  not only in a session prompt; a `scripts/check_*.sh` registered in
  `scripts/check_doctrines.project.sh` fails on a seeded local commit and on a seeded local
  modification inside `vendor/`, and passes on a clean pin; the check has RED arms in `--self-test`;
  `docs/decisions/decision_repository-boundary-read-only.md` is linked from it and states the
  incident correctly.
  Priority: **medium** — see the correction below. The outbound half is preventive; the inbound half
  already fired once and was handled correctly.

  ⛔ **The incident, and a correction this leaf owes.** LinkedSpec `8b5b5ffd8` (`2026-09-27`,
  "record publication and repository-boundary violation") discloses "the unauthorized ARCHOGEN
  documentation commit and auxiliary writes". This leaf first read that as an **outbound** crossing
  and called the priority high because "the defect class already fired". **The director corrected it
  on `2026-09-27`: the crossing was inbound** — LinkedSpec's agent modified a few `.md` files *in
  this repository* to deliver its fix notice. LinkedSpec has since made other repositories
  read-only in its own bootstrap; it was a one-time error and is not expected to recur.
  Measured here: every commit carries the single local identity, `git reflog` is linear (no
  `reset`/`rebase`/`amend`), the inbound content entered via `82ee99a` (leaf `M1.18`) confined to
  `docs/feedback/linkedspec/**` plus archogen's own live docs, **no code path was touched**, and
  `make focused` → exit `0` with `cargo test --all` → **421 passed, 0 failed** over 36 suites.
  Full table in `docs/decisions/decision_repository-boundary-read-only.md`.
  ⭐ What survives, and it is the reason this leaf exists at all: `grep -rn 'READ-ONLY' CLAUDE.md
  AGENTS.md` → **no match**. The boundary rule was nowhere in the committed tree, which is why the
  direction was undecidable from inside it — and why the first draft of the durable record got it
  backwards. That wrong record, not a stray write, is the measured damage.

  ⚠️ **Honest limit to state up front:** a gate in *this* repository cannot prevent an outbound write
  into a checkout elsewhere on the filesystem, and cannot prevent an inbound write either — it can
  only require that the inbound one land through a leaf. What it can do is put the rule where every
  agent reads it, and detect the outbound symptom visible from here: a vendored submodule carrying
  local commits or local modifications. The leaf must not claim more than that.
  Scope, decided `2026-09-29` on measurement, before the check was written: the gate covers the pins
  **this repository owns** — the top-level submodules its index records as gitlinks — and not the
  checkouts nested inside them. Measured at `b9f6e22`: `vendor/linkedspec` itself is clean on every count
  (checked out at its pin `2ac834913`, **0** commits reachable from `HEAD` or a local branch and from no
  remote-tracking ref or tag, no modified tracked file, no untracked file), while its nested RGX corpora
  under `rgx/subs/pgen/stimuli/` carry thousands of changed entries and several moved pins. Those are the
  vendor's **own documented bootstrap** at work — `scripts/linkedspec_eval.sh prepare` runs RGX's published
  route — which the director's rule explicitly permits ("normal documented builds and reuse of their outputs
  are permitted"), so flagging them would make the gate refuse every commit for consumption. Two
  measurements shaped the rules: `rev-list --all --not --remotes` **overcounts**, because `--all` includes
  fetched tags (it reported 4 040 "local" commits in one nested checkout), so a local-only commit is one
  reachable from `HEAD` or `refs/heads` and from no remote-tracking ref **and no tag**; and `AGENTS.md` is on
  `scripts/update_scaffold.sh`'s overwrite list while `CLAUDE.md` is project-owned, so the rule is stated in
  `CLAUDE.md`, where `AGENTS.md` already sends every agent — an edit the scaffold would erase is no edit.
  Verification: see the checklist — the rule stated in `CLAUDE.md`, a registered check with nine arms, five
  mutations seen firing, the real tree green.
  Commit: `ARCHOGEN-PROGRAM-0119 (leaf PROGRAM.11)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the rule was nowhere a resuming agent reads, and nothing observed the
    outbound symptom:
    ```text
    $ git grep -n -i "read-only\|repository boundary" b9f6e22 -- CLAUDE.md AGENTS.md
      (no match)                                                          rc=1
    $ git grep -n "vendor/\|submodule" b9f6e22 -- scripts/check_doctrines.project.sh
      (no match)                                                          rc=1 — no doctrine looks at vendor/
      (the bare word `vendor` matches FEEDBACK-SELF-CONTAINED's "vendor register" — a phrase, not the path)
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHY the direction was once recorded backwards: the rule lived only in
    a session prompt, so the tree could not answer "who may write where". WHERE the observable symptom sits,
    measured on the real checkout and its nested ones:
    ```text
    $ git -C vendor/linkedspec rev-parse HEAD ; git ls-files -s vendor/linkedspec
      2ac834913d85c32f532be9b0aab63644838a577a
      160000 2ac834913d85c32f532be9b0aab63644838a577a 0	vendor/linkedspec
    $ git -C vendor/linkedspec rev-list HEAD --branches --not --remotes --tags -- | wc -l   -> 0
    $ git -C vendor/linkedspec/rgx/subs/pgen/stimuli/vhdl/subs/PoC rev-list --all --not --remotes | wc -l
      -> 4040   (fetched tags counted as "local": the naive census is wrong)
    ```
    So a correct check scopes to the pins this repository owns and excludes tags from "local-only".
  - [x] **FIX** — `CLAUDE.md` gains the non-negotiable, both directions, naming the doctrine and the decision
    (the scaffold overwrites `AGENTS.md` but not `CLAUDE.md`, and `AGENTS.md` already routes there);
    `scripts/check_repository_boundary.sh` checks every gitlink the index records — at its pin, no
    local-only commit, no modified tracked file, no created file — and passes an un-checked-out submodule
    with a note; registered as `REPOSITORY-BOUNDARY` in `scripts/check_doctrines.project.sh`; mirrored in
    `DOCTRINE_ENFORCEMENT.md` and `TOOLBOX.md`; the decision record's follow-up item rewritten as what is
    now enforced, with the nested-checkout limit.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/check_repository_boundary.sh
      repository-boundary: OK (1 vendored checkout(s) at their pins, with nothing written into them)  0.16 s
    $ bash scripts/check_repository_boundary.sh --self-test
      repository-boundary self-test: 9 pass / 0 fail (9 arms)
    ```
    Five mutations, each seen failing exactly the arm named for it (restored from a copy, `cmp`-verified):
    ```text
    R-1 tags counted as local (the naive census)  -> 7 pass / 2 fail: the TAG arm and even the CLEAN arm
    R-2 local branches not examined               -> 8 pass / 1 fail: the local-branch arm
    R-3 the pin not compared                      -> 8 pass / 1 fail: the moved-off-its-pin arm
    R-4 modified tracked files not looked for     -> 8 pass / 1 fail: the modified-file arm
    R-5 created files not looked for              -> 8 pass / 1 fail: the created-file arm
    ```
    ⛔ One arm was vacuous as first written — the tag pointed at a commit `origin/main` already reached, so it
    passed with or without the exclusion — found by asking what the arm could fail on, and re-seeded on a
    commit only the tag reaches; R-1 then turned it red.
  - [x] **NO REGRESSION** — `scripts/check_doctrines.sh` → `=== all doctrines green ===` with the new
    doctrine; no Rust changed (`cargo test` not required by the change, run at the commit's focused tier);
    `check_book_anchors.sh` → `OK (19, 3)`; `mdbook build` `rc=0`; every other gate's `--self-test`
    unchanged.
  - [x] **LOCKSTEP** — `CLAUDE.md`, `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`, the decision record, the book's
    `verification.md` ("Other repositories are read-only"), `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md`,
    `docs/TASK_TREE.md`, this tree.
