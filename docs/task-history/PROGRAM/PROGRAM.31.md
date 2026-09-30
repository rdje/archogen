- ID: `PROGRAM.31`
  Status: `done`
  Goal: `CHANGELOG.md` and `DEV_NOTES.md` as rolling ledgers, as ruled on `2026-09-30` (§8 of
  `docs/decisions/decision_findings-for-director-review.md`, by delegation): whenever a live ledger holds twice its
  window of entries, its oldest window-full is sealed byte for byte into the next numbered segment under
  `docs/history/<ledger>/`, listed in `docs/history/INDEX.md` with its range, counts and digest. The window is 20
  entries for the changelog and 10 for the development notes. The project template's own entries, which it had
  shipped into both files, are removed, on the director's word that archogen keeps no reference to that template.
  Acceptance: the doctrine's atomic protocol — a deterministic boundary, byte-exact segments, recorded counts and
  digests, and a reconstruction proved byte for byte (`cmp` of the live entries followed by the segments, newest
  first, against the pre-rotation file); a gate, with RED arms, proving each segment unchanged, the index complete,
  append-only against `HEAD` and in order, and the live window bounded; the seal done by a tool, not by hand; the
  ceilings `README-ROUTES` held as debt replaced by what the gate enforces; nothing lost.
  Verification: see the checklist — two ledgers sealed and proved byte for byte twice over, an ordering defect found
  and corrected on the way, 13 RED arms, every gate green.
  Commit: `ARCHOGEN-PROGRAM-0207 (leaf PROGRAM.31)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — nothing bounded either history: `wc -c CHANGELOG.md DEV_NOTES.md` → 327 233 and
    135 158 bytes, 17 days after the first commit; `git log --format=%ad --date=short | uniq -c` → up to 75 commits
    in a day. `README-ROUTES` carried both as `debt: PROGRAM.31`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: both files are `rolling_ledger`s in `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s
    inventory with no containment, waiting on §8's ruling. WHY a count boundary: the same `git log` census shows the
    whole history in one month, so a monthly boundary seals nothing. And WHY the first transition was undone and
    redone: the gate's first run over the sealed result refused (`bash scripts/check_history_ledgers.sh --seal` → rc=1,
    `dev-notes's dates rise somewhere`). An `awk` over the headings found `2026-09-13` below `2026-09-04` at line
    1287, and `git log -S"a template's trial must include the first commit" -- DEV_NOTES.md` → `32e6b14 Initial
    commit`: the project template's two notes and its header paragraph had stayed mid-file, with archogen's notes
    added above and below them. So the defect was in the ledger, not the seal. The director then ruled it template
    residue, a defect since fixed in the template, and archogen free of any reference to it: that block, and the
    template's release notes at the end of the changelog, were removed rather than sealed.
  - [x] **FIX** — `scripts/check_history_ledgers.sh`: `--seal` (rollover with its own reconstruction proof), the gate's six
    legs, and 14 RED arms; named `check_*` so the spine harness's driver arm stubs it (its first name,
    `history_ledgers.sh`, failed `bash scripts/run_self_tests.sh` with rc=1: `the project driver: every gate passing
    exits 0 — expected exit 0, got 1`); `HISTORY-LEDGERS` registered; `docs/history/INDEX.md` and the segments;
    `docs/decisions/decision_history-ledgers.md`; both live headers name the index.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_history_ledgers.sh --seal` → `CHANGELOG.md — sealed 0007 … 0001 and
    kept 20 entries; the reconstruction is byte for byte`, `DEV_NOTES.md — sealed 0004 … 0001 and kept 19 entries`,
    rc=0. An independent rebuild in Python, the live entries then the segments from the highest down, against the
    files saved before the seal with the template's lines cut at their separator: `live + segments == every
    archogen entry before the seal: True` for both, the lines removed being exactly the template's (161 in the
    changelog, 20 in the development notes, the moved block included). The move of that block to the end, before
    its removal, was proved the same way: `block carried verbatim: True`, `the rest in order, unchanged: True`.
    `wc -c` after: `CHANGELOG.md` 21 200, `DEV_NOTES.md` 42 240. `--self-test` → `14 pass / 0 fail (14 arms)`: a
    ledger due for rollover, the seal and its proof, an idempotent second seal, a byte added to a segment, a
    segment without a row, a committed row changed, entries out of order, a note whose date rises, a header
    without the index, and the next rollover in sequence.
  - [x] **NO REGRESSION** — `bash scripts/check_readme_routes.sh` → `19 destination(s) governed; recorded as debt,
    owned by: PROGRAM.32`, the two ledgers moved from debt to `HISTORY-LEDGERS` and `docs/history/` registered as
    an archive; `bash scripts/run_self_tests.sh` → rc=0, `self-tests: OK — 34 self-test(s) passed`, the new gate's
    14 arms and the spine harness's driver arms among them; `check_stated_order.sh`, `check_book_anchors.sh`,
    `check_figure_register.sh`, `check_table_arity.sh` and `check_memory_architecture.sh` → OK; the knowledge map
    regenerated for the new decision record; the doctrine gate on commit.
  - [x] **LOCKSTEP** — the findings record's §8 ruling; the decisions index; `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`,
    `COMMIT.md`; the containment inventory; `README_POLICY.md`'s routes; the book's verification chapter; this
    leaf, `PROGRAM.32`, the frontier and both logs; the snapshots; `CHANGELOG.md`.
