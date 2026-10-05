- ID: `PROGRAM.52`
  Status: `done` — filed and closed `2026-10-05` on the director's word that a stuck review loop is an engineering
  problem
  Goal: the method of `docs/decisions/decision_executable-design-reviews.md` in place for every design reviewed under
  the closure rule — a reference model or measuring instrument, a falsification corpus, and the hand-off ledger — and
  the doctrine `HANDOFF-LEDGER`, `scripts/check_handoff_ledger.sh`: a record's `<!-- machine-read: handoffs -->` table
  holds each hand-off's identifier, receiving leaf and obligation sentence; the commit is refused when a row's
  sentence is not quoted verbatim beside its identifier in that leaf, or a leaf quotes an identifier no ledger holds.
  Acceptance: the decision record and its index row; the doctrine with RED arms run by `scripts/run_self_tests.sh`,
  registered and mirrored; `decision_substitutability-relation.md` and `decision_trust-inventory.md` holding their
  hand-offs in ledgers, every receiving leaf quoting them, the doctrine green; `M3.1.1.1` and `M3.6.2` carrying the
  model and the instrument.
  Children: `PROGRAM.52.1` the doctrine; `PROGRAM.52.2` the two records moved to ledgers, after their running rounds
  Verification: `2026-10-05` — through `.1` and `.2`
  Commit: step 1, `ARCHOGEN-PROGRAM-0425 (leaf PROGRAM.52)`

- ID: `PROGRAM.52.1`
  Status: `done` — `2026-10-05`
  Goal: `HANDOFF-LEDGER`, `scripts/check_handoff_ledger.sh`, registered and mirrored.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the review histories' late defects: `M3.1.1` R10–R15 and `M3.6.1` R3–R6 each found a
    hand-off its leaf did not carry or carried in weaker words (`docs/reviews/decision_substitutability-relation-reviews.md`,
    `docs/reviews/decision_trust-inventory-reviews.md`); `git ls-files scripts | grep -c handoff` → 0 before this leaf.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `bash scripts/check_doctrines.sh` → `=== all doctrines green ===` over a tree
    whose hand-offs had been found uncarried by a reader: each hand-off was two independent prose statements, the
    record's and the leaf's, and nothing compared them.
  - [x] **FIX** — a ledger table per record and `[<id>] <sentence>` in the leaf, compared after whitespace
    normalisation, in both directions; registered in `scripts/check_doctrines.project.sh`, mirrored in
    `DOCTRINE_ENFORCEMENT.md`.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_handoff_ledger.sh --self-test` → `12 pass / 0 fail (12 arms)`:
    a quote across a line wrap passes; a dropped quote, weaker words, an unheld identifier, another leaf's quote, one
    identifier twice, a leaf found nowhere, a malformed identifier and a quote outside every block are refused; a
    sealed leaf's quote passes; no ledger is clean. The first run failed one arm, which found a quote outside every
    leaf's block passing unchecked; the gate now refuses it.
  - [x] **NO REGRESSION** — `bash scripts/check_handoff_ledger.sh` on the tree → `handoff-ledger: OK (0 hand-off(s)
    in 0 ledger(s) …)`; `bash scripts/run_self_tests.sh` → every self-test passed; `bash scripts/check_doctrines.sh`
    → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md`; this leaf and both logs; `CHANGELOG.md` → one entry.
  Verification: `2026-10-05` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0426 (leaf PROGRAM.52.1)`

- ID: `PROGRAM.52.2`
  Status: `done` — `2026-10-05`: step 1 the trust record's ledger, step 2 the substitutability record's
  Goal: `decision_substitutability-relation.md` and `decision_trust-inventory.md` holding every hand-off in a ledger,
  prefixes `SR` and `TI`, and every receiving leaf quoting its sentences.
  Acceptance: `bash scripts/check_handoff_ledger.sh` green with both ledgers populated; no hand-off left in prose
  that the ledger does not hold, by a census of each record's "is `M…`'s" sentences.
  Verification: `bash scripts/check_handoff_ledger.sh` → `handoff-ledger: OK (46 hand-off(s) in 2 ledger(s), each
  quoted word for word by its leaf; 46 quote(s), each held)`; a census of every "`M…`'s" in each record's prose
  against its ledger's leaves → the names absent from a ledger are the record's own leaf, the closed `M1.40`, and
  two named inside ledger sentences, so no hand-off is left in prose; each rewritten leaf read against its previous
  text, one dropped item restored (step 1) and one retracted corpus claim left out of `M3.4` (step 2)
  Commit: step 1, `ARCHOGEN-PROGRAM-0429 (leaf PROGRAM.52.2)`; step 2, `ARCHOGEN-PROGRAM-0430 (leaf PROGRAM.52.2)`
