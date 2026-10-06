- ID: `PROGRAM.59`
  Status: `done` — filed and closed `2026-10-06`
  Goal: no live document still says, in the present tense, that the emulator step is quarantined or that
  `integration` is incomplete everywhere.
  Reproduce / issue: found by `PROGRAM.58`'s run, `make integration` → `tier integration: passed — 12 passed, 0
  failed, 0 unavailable, 0 not built, 0 quarantined`. Yet `COMMIT.md` step 2 read *"Today `integration` is
  incomplete everywhere — … the step is **quarantined** under `M2.8`"*. The book, `verification.md:128`, read
  *"`integration` is too — the emulator quarantined, as above"*, fifteen lines under a transcript showing it passing.
  `decision_incomplete-blocking-policy.md` read *"Today the one gap is the emulator step, quarantined under
  `M2.8`"*, and this tree's Blockers read *"`M2.8` still owns removing the cause"*.
  Root cause: `git show --stat 6096906` (`ARCHOGEN-M2-0193`, `M2.8.3.4`, which lifted the quarantine) touches
  `verification.md`'s transcript and its "Until then" paragraph, and none of the four passages. `git log -L128,129`
  names `8a9e98a` (`PROGRAM.10.1`) as their author. The commit that changed the state took no census of the
  sentences restating it: the class `docs/knowledge/a-moved-measurement-needs-a-census-of-its-copies.md` describes.
  Census: `git grep -n -i 'quarantin\|incomplete everywhere'` and `git grep -n -i
  'integration.\{0,40\}incomplete'` over every tracked `*.md` outside sealed history and the changelog, plus
  `*.rs` and `*.sh`. Each hit was read in context. Four were stale and are corrected. The rest stay: they are
  dated history (the rehearsal of `2026-09-30` in `verification.md:254`, `decision_push-cadence.md`'s amendment
  of the same day, `docs/targets/first-target.md:48`), the definition of a quarantine, the CI script's arms, or
  rows of closed leaves.
  Fix: `COMMIT.md` step 2 now says what holds today: `integration` passes where its tools are installed and is
  incomplete where one is missing, and the quarantine is past tense with its dates. The book's sentence is
  corrected; the decision record gains a dated amendment rather than a rewrite; so does this tree's Blockers entry.
  Verification: both census commands re-run afterwards, every present-tense hit now true; the book builds;
  `make focused`; the doctrine gate.
  Lockstep: the decision's amendment and the book move in the same commit, as a normative edit must. No snapshot
  moves.
