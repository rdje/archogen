- ID: `PROGRAM.18`
  Status: `done` — closed by its two children `2026-09-29`; **decomposed `2026-09-29` into two children on a re-run census**, as the leaf itself asks.
  Measured at `61e5f09`: `check_task_acceptance.sh` is **already armed** (nine arms, `PROGRAM.21`), so the
  "first" this leaf names is spent. Still without a `--self-test`: **seven registered controls** — six
  universal (`MEMORY-ARCH`, `DOCPATH`, `TASK-TREE-OWNERSHIP`, `README-STABILITY`, `WAIVER-ROUTING`,
  `KNOWLEDGE-MAP`) and one project (`FROZEN-EVALUATION`) — plus the two drivers and the handoff tool
  `check_no_background_jobs.sh`. ⛔ **The six universal scripts are on `scripts/update_scaffold.sh`'s
  overwrite list, and their upstream is another repository**, so an arm written *into* them is erased by the
  next scaffold sync and cannot be sent upstream from here (the repository boundary); `PROGRAM.21`'s edit of
  the scaffold-owned `check_task_acceptance.sh` already carries that hazard, which is `PROGRAM.26`'s. So the
  children split by ownership: `PROGRAM.18.1` arms the project-owned `FROZEN-EVALUATION` in place;
  `PROGRAM.18.2` arms the scaffold-owned gates and the drivers **from outside**, in a project-owned harness
  that runs each unmodified script inside a scratch repository holding a seeded breach.
  Children: `PROGRAM.18.1`, `PROGRAM.18.2`.
  Goal: give the registered doctrine controls the repeatable RED arms they lack, so that "is this gate
  known to work?" stops depending on a validation somebody ran once while writing it.
  Reproduce / issue: measured by the §7.4 sweep of the claim-verification adoption (`PROGRAM.16`,
  `2026-09-27`): `scripts/check_*.sh` → **18** files, **8** carry `--self-test` with RED arms and all
  8 pass, **10 do not** — `check_docpaths`, `check_doctrines`, `check_doctrines.project`,
  `check_frozen_evaluation`, `check_memory_architecture`, `check_no_background_jobs`,
  `check_readme_stability`, `check_task_acceptance`, `check_task_tree_ownership`,
  `check_waiver_routing`. THE GAP: ten of eighteen controls have never been observed failing by a
  repeatable arm, and they include the most load-bearing gate in the repository.
  `check_task_acceptance.sh`'s own header records that its box-scoping was "priced against a real
  corpus" and validated against two measured leakage holes — real validation, and **one-off**: nothing
  re-fires it, so an edit could silently break the property and every commit would still pass. Census
  command: `for s in scripts/check_*.sh; do grep -q -- '--self-test' "$s" && echo yes || echo "$s"; done`.
  Acceptance: each of the ten either gains a `--self-test` carrying at least one RED arm that fails on
  a seeded breach and passes on the real tree, or is recorded in `DOCTRINE_ENFORCEMENT.md` as needing
  none with the reason — the two drivers are candidates, since they only run the others;
  `check_task_acceptance` is done **first**, and its arms must include the two leakage holes its header
  names, so the property it was written for is the property under test; every arm is run and its output
  recorded in the leaf; the doctrine count and the registry stay accurate; `make gate` is green before
  and after.
  Priority: **medium-high** — a gate nobody has seen fail is a gate whose failure mode is unknown, and
  this repository's whole claim is that its discipline is mechanical rather than remembered.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.18.1`
  Status: `done`
  Goal: `FROZEN-EVALUATION` gains a `--self-test` whose RED arms each seed one breach of one of its three legs
  in a scratch repository — integrity, completeness, non-contamination — and require the refusal to name the
  case it is about, with a passing arm for a clean set, an unsealed set and an untracked mention.
  Acceptance: every arm uses **synthetic** case names, because writing a real sealed slug anywhere tracked is
  itself the contamination the gate exists to catch; one arm runs the real tree; mutations seen firing; the
  registry and `DOCTRINE_ENFORCEMENT.md` say it is armed.
  Priority: **medium-high** (the parent's).
  Verification: see the checklist — nine arms, seven mutations each failing its own arm.
  Commit: `ARCHOGEN-PROGRAM-0120 (leaf PROGRAM.18.1)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the gate had never been seen failing by a repeatable arm:
    ```text
    $ git grep -c -- "--self-test" 61e5f09 -- scripts/check_frozen_evaluation.sh    -> no match, rc=1
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — it was written with its three legs validated once, by hand, at
    `M0.6`, and nothing re-fires them; its legs are observable only against a *set*, so an arm needs a
    whole repository to seed:
    ```text
    $ git log --diff-filter=A --format='%h %s' -- scripts/check_frozen_evaluation.sh | cut -c1-60
      b05fadb ARCHOGEN-M0-0007 (leaf M0.6): four use cases, and a
    ```
    (the add commit, and no commit since has added an arm), and the line at its head is the seam:
    ```text
    $ git grep -n 'ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"' 61e5f09 -- scripts/check_frozen_evaluation.sh
      61e5f09:scripts/check_frozen_evaluation.sh:31:ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
    ```
    — run it with its working directory inside a scratch repository and it checks that repository's set.
  - [x] **FIX** — `--self-test` in the script: a scratch repository under `target/doctrine_scratch/` with a
    sealed set of two synthetic cases and one other tracked file, re-created per arm; eight seeded arms
    (intact, edited, removed, unlisted, named in a tracked file, named only in an untracked file, unsealed,
    no seal line) and the real tree; every refusing arm must name the case or line it refuses.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/check_frozen_evaluation.sh --self-test
      frozen-evaluation self-test: 9 pass / 0 fail (9 arms)
    ```
    Seven mutations, each checked applied (the file differs from its copy) and restored by `cmp`:
    ```text
    F-1 hashes never compared                -> 8 pass / 1 fail: the integrity arm
    F-2 a missing case not reported          -> 8 pass / 1 fail: the removed-case arm
    F-3 an unlisted file not reported        -> 8 pass / 1 fail: the unlisted arm
    F-4 the contamination leg removed        -> 8 pass / 1 fail: the tracked-mention arm
    F-5 untracked files searched too         -> 6 pass / 3 fail, the untracked-mention arm among them
    F-6 contamination not lifted if unsealed -> 8 pass / 1 fail: the unsealed arm
    F-7 the seal line not required           -> 8 pass / 1 fail: the no-seal arm
    ```
  - [x] **NO REGRESSION** — `bash scripts/check_frozen_evaluation.sh` on the real tree → exit `0`;
    `scripts/check_doctrines.sh` → `=== all doctrines green ===`; no Rust changed.
  - [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md`'s row and `TOOLBOX.md`'s; the parent's census note; the live
    docs; and **`PROGRAM.29` filed** for the `mktemp` scratch this script (and five others) puts in `/tmp`.

- ID: `PROGRAM.18.2`
  Status: `done`
  Goal: arm the six scaffold-owned universal gates and the two doctrine drivers **without editing them** — a
  project-owned harness that copies nothing of theirs, runs each unmodified script with its working
  directory inside a scratch repository holding a seeded breach, and requires its refusal to name the
  subject; and record `check_no_background_jobs.sh`'s disposition (armed, or needing none with the reason).
  Acceptance: each arm names what it refuses; each gate has a clean-fixture arm so a refusal cannot pass for a
  second reason; a mutation of each gate seen failing its own arm; `DOCTRINE_ENFORCEMENT.md` and `TOOLBOX.md`
  updated. ⚠️ Registration in a tier moved to **`PROGRAM.28`**, measured rather than assumed: the harness takes
  **~30 s** (the handoff tool walks every process with `lsof`), which is a tier's budget and not the pre-commit
  path's, and `PROGRAM.28` exists to run every `--self-test` in one — registering this one alone would be a
  second mechanism for the same job.
  Priority: **medium-high**.
  Verification: see the checklist — 34 arms, nine mutations each failing exactly its own arm.
  Commit: `ARCHOGEN-PROGRAM-0121 (leaf PROGRAM.18.2)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the re-run census on the parent: six scaffold-owned universal gates, the two
    drivers and the handoff tool had never been seen failing by a repeatable arm (`grep -q -- '--self-test'`
    → absent in all nine).
  - [x] **ROOT CAUSE (WHY + WHERE)** — nobody could arm them in place without the arm being erased by the
    scaffold, and two of them could not even be pointed at a scratch repository by working directory,
    because they find their root by their own path:
    ```text
    $ git grep -n 'ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"' 6bf578f -- scripts/check_waiver_routing.sh
      6bf578f:scripts/check_waiver_routing.sh:38:ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
    $ git grep -n 'REPO_ROOT="$(cd "$(dirname …' 6bf578f -- scripts/check_no_background_jobs.sh
      6bf578f:scripts/check_no_background_jobs.sh:53:REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)" || exit 2
    ```
    — so those two are run as byte-for-byte copies placed inside the scratch repository (the harness checks
    the copy is identical), and the other four by working directory.
  - [x] **FIX** — `scripts/selftest_spine.sh`, project-owned: a fresh scratch git repository per arm under
    `target/doctrine_scratch/`, one seeded breach, the real gate run there, and a refusal required to carry
    the text naming its subject. The drivers get stub gates at every path they register (their property is
    "run each, propagate a failure"); the handoff tool gets a real `sleep` holding a file in the scratch
    repository, killed and reaped before the next arm. Removes its scratch on success.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/selftest_spine.sh
      spine self-test: 34 pass / 0 fail (34 arms)                          real 0m28.5s
    ```
    ⛔ The first run was **33 / 1**, and the failing arm was a *clean* one: the stub regex `[a-z_.]` missed
    the digit in `check_s0_retirement.sh`, so that gate was absent — which meant the project driver's two
    refusal arms were refusing for a second reason as well. The clean arm is what exposed it; fixed to
    `[a-z0-9_.]`. Nine mutations, one per script, each applied to the real file, run, and restored from a
    copy verified by `cmp` — every one fails **exactly** the arm named for it:
    ```text
    S-1 MEMORY-ARCH line cap never compared        -> 33 / 1: the line-cap arm
    S-2 DOCPATH pattern matches nothing            -> 33 / 1: the checkout-path arm
    S-3 TASK-TREE-OWNERSHIP lets an unowned change -> 33 / 1: the unowned-code arm
    S-4 README-STABILITY dated lines ignored       -> 33 / 1: the release-history arm
    S-5 WAIVER-ROUTING any line names an owner     -> 33 / 1: the unowned-waiver arm
    S-6 KNOWLEDGE-MAP never compared               -> 33 / 1: the out-of-sync arm
    S-7 universal driver does not count a failure  -> 33 / 1: its failing-gate arm
    S-8 project driver does not count a failure    -> 33 / 1: its failing-gate arm
    S-9 handoff tool never reports a holder        -> 33 / 1: the holding-process arm
    ```
  - [x] **NO REGRESSION** — `bash scripts/check_no_background_jobs.sh` → `handoff: OK` after the run (the
    holder was reaped); `git status` after the mutations shows only the new harness, every script
    byte-identical to its copy; no Rust changed; and the doctrine driver over the real tree with the harness
    in place:
    ```text
    $ bash scripts/check_doctrines.sh | tail -1
      === all doctrines green ===
    ```
  - [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md` (how the universal gates are armed, and why from outside),
    `TOOLBOX.md`, the parent closed, `PROGRAM.28` widened to register this harness, the live docs. ⚠️ One
    observation recorded, not fixed here: `TASK-TREE-OWNERSHIP` hard-codes "code" as Rust and Cargo
    (`6bf578f:scripts/check_task_tree_ownership.sh:27`) instead of reading `.doctrine/code_paths.txt`, so a
    staged script alone passes it — `TASK-ACCEPTANCE` reads the seam and still refuses one, so nothing is
    unguarded, and the arm pins today's scope. The script is the scaffold's; `PROGRAM.26` owns what a
    scaffold sync may and may not change here.
