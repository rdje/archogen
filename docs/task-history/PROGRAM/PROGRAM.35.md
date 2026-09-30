- ID: `PROGRAM.35`
  Status: `done`
  Children: `PROGRAM.35.1`, `PROGRAM.35.2`
  Goal: the README policy the director's standing instruction names — fsmgen's `README_POLICY.md` — adopted at its
  current revision, with caps derived from this repository's README and every route out of the README ending at a
  controlled destination.
  Reproduce / issue: found `2026-09-30` at session start, checking each external policy the standing instructions
  name against what this repository holds (read-only, §12):
  ```text
  $ shasum -a 256 README_POLICY.md ../bedrock/README_POLICY.md ../fsmgen/README_POLICY.md
    091e6922…  README_POLICY.md            (71 lines / 2 920 bytes)
    091e6922…  ../bedrock/README_POLICY.md (identical: the bedrock-scaffold 0.2.0 transfer, the initial commit)
    882682fa…  ../fsmgen/README_POLICY.md  (187 lines / 9 849 bytes; last changed 1f0443b3a, 2026-08-20)
  $ git grep -n 'README_LINE_CAP\|README_BYTE_CAP' -- . ':!vendor'   → only the defaults in the gate itself
  $ bash scripts/check_readme_stability.sh   → README.md is 85/300 lines, 4305/16384 bytes
  ```
  The copy here is an earlier derivative of the same lineage. fsmgen's revision adds four obligations: a fenced local
  adoption note and a stated authority; caps **derived** from the reviewed survivor, never the example values; every
  destination the README, the policy or the guard's hint routes to classified with an owner, a lifecycle and a
  pressure control, transitively, so the cap cannot just displace growth into a neighbouring file; and that closure
  checked unconditionally. ⛔ The gap is real under the copy already here too: its own text says to choose caps
  "after a deliberate review and trim", and `check_readme_stability.sh:25-29` says the defaults are "the policy's own
  published example rather than fitted to this repository's README". Nobody fitted them: 300 lines is 3.5× the page.
  Impact: nothing is over a cap today. But the README could quadruple before anything fired, and `CHANGELOG.md`, the
  guard's own overflow route, has no pressure control — the exact displacement fsmgen's policy measured.
  Acceptance: both children closed.
  Priority: **medium** — a standing instruction not met at the named revision; no present breach.
  Verification: closed `2026-09-30` by its two children — the text and the derived caps (`.35.1`), the routing
  closure and its check (`.35.2`).
  Commit: `ARCHOGEN-PROGRAM-0197` (`.35.1`), `ARCHOGEN-PROGRAM-0198` (`.35.2`)

- ID: `PROGRAM.35.1`
  Status: `done`
  Goal: the policy text adopted, and the README's caps derived from it.
  Acceptance: `README_POLICY.md` is a fenced archogen adoption note followed by fsmgen's neutral body, byte-identical
  to its source (`cmp`), with the source revision and the body's digest recorded; the README reviewed against the
  content contract, each apparent duplication probed against its canonical home; line and byte ceilings derived from
  the survivor with modest stated headroom and enforced unconditionally by `LIVE-SNAPSHOTS`, with a RED run on the
  real tree; nothing written outside this repository.
  Verification:
  - **The copy.** fsmgen read with `sed -n`, `shasum`, `wc` and `git log` only; its `README_POLICY.md` is clean at
    `HEAD` `0d01ca3fc` and last changed at `1f0443b3a`. Lines 1–25 are fsmgen's own fenced note, and the neutral body
    is lines 29–187.
    ```text
    $ sed -n '29,$p' ../fsmgen/README_POLICY.md | cmp - <body of README_POLICY.md from "# README Stability Policy">
      cmp: body identical
    $ shasum -a 256 <that body>    → 77a1e9348ec24d9ec5f0c97ae1ac2d634f7e7e3e150504759af3c0182d6eefec (159 lines / 8 279 bytes)
    $ grep -niE 'fsmgen|claude|codex|gemini|cursor' <body>   → no match: the body carries no project or vendor token
    ```
  - **The review of the page.** Each content class against the contract. Kept: purpose and boundary, the quick
    start (`make focused` passed this session, and `make tiers` lists the tiers), the architecture sketch (the
    responsibility names are `ROADMAP.md` §4.2's, so they are architecture at a glance and not a crate inventory),
    the navigation table, the contribution links, and the licence. **One duplication probed and removed:** the
    Status paragraph's "the roadmap is seeded into task-trees and the discipline spine is enforced" restated
    `LIVE_STATUS.md`'s rows 10 and 11, which are richer and linked two lines below. The link stays, and so does
    the notice that nothing here claims a verified OS. Every link resolves: 19 of 19 targets exist.
  - **The caps.** The survivor is 84 lines, 4 173 bytes and 125 bytes on its longest line. The ceilings are 110,
    6 144 and 200 (lines +31 %, bytes +47 %), the headroom practice of the other snapshots: `docs/TASK_TREE.md` went
    73 → 96 and 4 469 → 6 144. They are data in `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s bounds table, and
    `check_live_snapshots.sh` reads them on every run whatever is staged. Measured on the real tree, one below the
    survivor on each axis:
    ```text
    lines<=83  bytes<=6144 → rc=1  LIVE-SNAPSHOTS: README.md: 84 lines, over its ceiling of 83 …
    lines<=110 bytes<=4172 → rc=1  LIVE-SNAPSHOTS: README.md: 4173 bytes, over its ceiling of 4172 …
    lines<=84  bytes<=4173 → rc=0  (equality passes)
    lines<=110 bytes<=6144 → rc=0  live-snapshots: OK (4 snapshot(s) within their ceilings …)
    ```
  - `README-STABILITY` → `OK — README.md is 84/300 lines, 4173/16384 bytes`. Its defaults stay as the outer
    backstop, and the adoption note says they are not this project's caps.
  - ⚠️ **Found for `.35.2`:** a README over its ceiling now gets `LIVE-SNAPSHOTS`' hint as well as
    `README-STABILITY`'s. That hint says "move history to its tree's Commit Log and CHANGELOG.md", so both guards
    route overflow to `CHANGELOG.md`, and `.35.2` must derive its hint paths from both.
  Commit: `ARCHOGEN-PROGRAM-0197 (leaf PROGRAM.35.1)`

- ID: `PROGRAM.35.2`
  Status: `done`
  Goal: the routing-pressure closure — a registry of every destination the README, the policy and the guard's emitted
  hint route to, each with its route class (`reader_navigation` or `author_overflow`), owner, lifecycle class and
  pressure control; and a check, run on every commit, that derives the destinations from the README's links and the
  guard's **actual** hint and fails on one the registry does not govern.
  Acceptance: the registry complete against the derived population; the check registered as a project doctrine with
  RED arms (an unregistered README link, an unregistered hint path, a row with no pressure control, an empty
  registry); `CHANGELOG.md` and `DEV_NOTES.md`, the uncontrolled terminals, recorded as measured debt routed to
  `PROGRAM.31` and the director's §8 ruling rather than silently counted as controlled.
  Verification: see the checklist — 18 destinations governed, 16 of 16 arms, six mutations of the real registry.
  Commit: `ARCHOGEN-PROGRAM-0198 (leaf PROGRAM.35.2)`
  promotion: declined (no `DEV_NOTES.md` lesson is added; the one transferable observation, that a heading matched as
  a bare substring can land an edit inside a leaf body, is already recorded at `PROGRAM.20.1`'s placement repair)

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — before this leaf nothing registered where the README routes, and the guards' shared
    overflow terminal had no bound: `git show HEAD:README_POLICY.md | grep -c "### Routed destinations"` → `0`;
    `README_LINE_CAP=0 bash scripts/check_readme_stability.sh` → rc=1, printing eight destinations, none of them
    checked by anything; `git ls-files CHANGELOG.md DEV_NOTES.md | xargs wc -c` → 316 413 and 135 158 bytes.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: the policy the scaffold transferred has no routing-pressure section
    (`git show 32e6b14:README_POLICY.md | grep -c "Routing pressure"` → `0`), so no route was ever registered.
    `scripts/check_readme_stability.sh:55` (`routing_hint`) and `scripts/check_live_snapshots.sh:59` both send an
    over-cap README's detail on, the second to "its tree's Commit Log and CHANGELOG.md". WHY: a cap on one file
    was the only control, and fsmgen's revision is the one that follows the pressure the cap displaces; `.35.1`
    adopted the text, and this leaf gives it a registry and a mechanism.
  - [x] **FIX** — the table `### Routed destinations` in `README_POLICY.md`'s fenced note: 18 rows, each with a
    route, an owner, a lifecycle, a control on every dimension and its onward routes. `scripts/check_readme_routes.sh`
    derives the population, from `README.md`'s links, the paths `README-STABILITY` and `LIVE-SNAPSHOTS` actually
    print for an over-cap README (the second on a scratch copy of its doctrine), the policy body's code spans, and
    each reached row's onward routes. It then holds legs 1–5 of its header. It is registered as `README-ROUTES`, and
    `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md` and the book's verification chapter mirror it. Ceilings were set from a
    measurement of each destination, with the snapshots' headroom practice. `CHANGELOG.md` and `DEV_NOTES.md`
    (lines, bytes) and `docs/tasks/` (all but its file count) are `debt: PROGRAM.31` and `debt: PROGRAM.32`, both
    open and both waiting on the director's §8 ruling.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_readme_routes.sh` → exit 0, `readme-routes: OK (18
    destination(s) governed; recorded as debt, owned by: PROGRAM.31 PROGRAM.32)`. Six mutations of the real
    registry, each restored with `cp` and re-run green:
    ```text
    AGENTS.md lines ceiling 13, one below 14   → rc=1  AGENTS.md: 14 lines, over its ceiling of 13 …
    VISIBILITY.md row removed                  → rc=1  … routes readers to `VISIBILITY.md`, which no row … governs
    CHANGELOG.md route written 'navigation'    → rc=1  … derives 'navigation + overflow'
    docs/tasks/ debt on the closed PROGRAM.30  → rc=1  … `PROGRAM.30` is done — a closed leaf owns no debt
    docs/book/ total 200000, below 221 710      → rc=1  docs/book/: 221710 bytes in total, over its ceiling of 200000
    a LIVE_STATUS ⇄ TASK_TREE onward loop       → rc=1  onward routes form a cycle: docs/TASK_TREE.md → LIVE_STATUS.md → docs/TASK_TREE.md
    ```
    `--self-test` → `readme-routes self-test: 16 pass / 0 fail (16 arms)`, stub guards in scratch repositories.
  - [x] **NO REGRESSION** — `bash scripts/run_self_tests.sh` → rc=0, `self-tests: OK — 32 self-test(s) passed`,
    the new gate among them at 16 pass / 0 fail; `bash scripts/check_scratch_locality.sh` → exit 0 (170 files);
    `bash scripts/check_doctrines.project.sh` → every project doctrine OK, `README-ROUTES` included;
    `bash -n scripts/check_readme_routes.sh` → syntax OK. No Rust changed.
    ⚠️ The awk runs as BWK `awk version 20200816` here, and the runner has `mawk`. Recursion and local arrays are
    POSIX, and the first real run on the runner's userland (`PROGRAM.10.5`) is where that is observed.
  - [x] **LOCKSTEP** — `README_POLICY.md`, `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`, `docs/book/src/verification.md`,
    `scripts/check_doctrines.project.sh`; this leaf and `PROGRAM.35`, the frontier and both logs; `LIVE_STATUS.md`,
    `docs/TASK_TREE.md`, `MEMORY.md`, `CHANGELOG.md`.

