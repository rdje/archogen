- ID: `PROGRAM.137`
  Status: `done` — `2026-11-13`; filed at session start: fsmgen's `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` changed its neutral body at `58d5edef7` (`2026-11-04`, "remove all book content size limits"), after the `0fa794310` adopted here; the adoption note admits a later revision only by an owned review.
  Goal: review that revision against this repository's own rules and adopt it verbatim, the note's and the ledger's provenance moved with it, or record why not.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the source moved and the copy did not; WHERE, `git -C <fsmgen> log 0fa794310..HEAD -- LIVE_DOCUMENT_SIZE_CONTAINMENT.md` → `58d5edef7` and `c83948125`, the second reviewed by `PROGRAM.57` as fsmgen's local part alone; the body here hashed `af130de4…7aa2`, the source's `f5fbff05…1e4b`. The other three sources are unchanged: fsmgen's README policy body `77a1e934…` and adoption guide `8f77fa39…` as adopted, and pgen's claim-verification policy 285 lines, 18 166 bytes, last changed `178251cce`, before its adoption.
    - [x] FIX — `python3 -I -B target/p137/adopt.py .`: the body replaced by the source's lines 191–549 at `58d5edef7`, read by `git show` and nothing written there; the adoption note and the ledger's `fsmgen` entry name the revision, the span and the digest. The revision adds fsmgen's user-facing book exception, which agrees with this repository's own ruling of `2026-10-18` that the book's growth is unbounded (the inventory's `docs/book/src/` row, `README_POLICY.md`'s `docs/book/` row): no limit here moves.
    - [x] ADDRESSED — `diff` of the body against `git show 58d5edef7:LIVE_DOCUMENT_SIZE_CONTAINMENT.md` lines 191–549 → identical; its sha256 `f5fbff0506537e23ca602e0006588ff1e76bfe4b4c056628d527ef8752261e4b`, 359 lines, 21 415 bytes.
    - [x] NO REGRESSION — `bash scripts/check_doctrines.sh` on the staged tree → exit 0. No code changed.
    - [x] LOCKSTEP — the doctrine's copy and note, the ledger, the tree and the Commit Log.
  Verification: the diff against the source, the gate.
  Commit: `ARCHOGEN-PROGRAM-1273 (leaf PROGRAM.137)`
