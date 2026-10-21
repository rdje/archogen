- ID: `PROGRAM.98`
  Status: `done` — the mutation-adequacy lane's pace surfaced and ruled: mutation testing waits for completion
  Goal: the director asked whether the project was progressing. Measured: from `2026-10-11` to `2026-10-21`, 158 of
    173 commits went to `M3.6.9`, the mutation adequacy of the trust gate's tests, and none to the product. Record it
    as findings §14 with the director's ruling of the same day — mutation testing stops until the functionality is
    complete and locked, every decision advances the project first — as a decision record, and move the trees to it:
    every pending `M3.6.9` leaf `blocked` on the ruling, `M3.3` the frontier head. No code changes.
  Verification: `git log --oneline --since=2026-10-11 | wc -l` → 173; with `--grep='leaf M3.6.9'` → 158; the other
    15 `PROGRAM` upkeep; `decision_mutation-testing-after-completion.md` and its index row; findings §14, the register
    at fourteen; 27 `M3.6.9` leaves `blocked` on the ruling, `.8.2` kept as a P2 defect. The new record put
    `docs/decisions/` 1 314 bytes over its ceiling (`README-ROUTES`), so the settled §12, approved `2026-10-02`, is
    sealed (`bash scripts/check_decision_history.sh --seal … 12`, byte for byte); §14 can be sealed once committed.
    The register's header named §2, §4, §8 and §10 as sealed where §6 was too: corrected. The doctrine gate green.
  Commit: `ARCHOGEN-PROGRAM-0840`

