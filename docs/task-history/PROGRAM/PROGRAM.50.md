- ID: `PROGRAM.50`
  Status: `done` — `2026-10-03`
  Goal: `docs/decisions/` back under its ceiling of 393 216 bytes without raising it: the fault contract's review
  rounds R3–R15, appended to `decision_runtime-contract-gaps.md` while the contract was reviewed, moved byte for byte
  into a review history of their own, `docs/reviews/rt-static-up-v1-faults-reviews.md`, as `docs/reviews/INDEX.md`
  places a design's review history and `PROGRAM.36` moved the catalog's.
  Why: on `2026-10-03` `M3.1.1`'s thirteenth step took the folder to 395 564 bytes, which `README-ROUTES` refused.
  The ceiling is the director's one-time raise (`decision_decisions-folder-ceiling.md`); `docs/specs/` has 8 KB of
  room, too little for an accepted design to move there (`decision_specifications-home.md`); the record's sections
  are unnumbered, so `DECISION-HISTORY` cannot seal them. The thirteen sections are review history — rounds of an
  independent review of `docs/profiles/rt-static-up-v1-faults.md`, each with its findings and answers — which
  belongs in `docs/reviews/`.
  Acceptance: the block moved unchanged, its sha256 the same before and after; a stub where it stood naming the new
  file; the profile's header and the review index pointing at it; `README-ROUTES` green, `docs/decisions/` under its
  ceiling.
  Verification: `git show HEAD:docs/decisions/decision_runtime-contract-gaps.md | sed -n 186,549p | shasum -a 256` → `5785eb49…57cc9`, and the same digest over
  the new file from its first round's heading; `git ls-files docs/decisions | xargs cat | wc -c` → 395 564 before,
  352584 after; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  Commit: `ARCHOGEN-PROGRAM-0412 (leaf PROGRAM.50)`
