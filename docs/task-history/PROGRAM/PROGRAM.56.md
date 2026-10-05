- ID: `PROGRAM.56`
  Status: `done` — filed and closed `2026-10-05`
  Goal: room in `docs/decisions/` by the director's ruling of `2026-10-01`
  (`docs/decisions/decision_specifications-home.md`): the trust gate's design, its review closed by `M3.6.1`, moves to
  `docs/specs/trust/`, every live reference following it; `docs/specs/`' ceiling raised to hold it, by a decision
  record with the measurement (`docs/decisions/decision_specs-folder-ceiling.md`); `docs/decisions/`' ceiling untouched.
  Acceptance: the record moved byte for byte but for its two relative links; every reference outside sealed history and
  dated rows repointed; both folders under their ceilings; every check green.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `git ls-files docs/decisions | xargs cat | wc -c` → 393 061 of 393 216 after
    `ARCHOGEN-M3-0444`, the substitutability review still open; `docs/specs/` → 253 792 of 262 144, so the 46 125-byte
    record would breach it.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `README_POLICY.md`'s `docs/specs/` row, sized by `PROGRAM.43` for one
    subject (186 667 bytes); WHY: the ruling sends every design whose review closes to `docs/specs/`, and the trust
    design is the second (`rc=1` from `README-ROUTES` without the raise).
  - [x] **FIX** — `git mv` of the record, its two links one level deeper; the path repointed in `crates/archogen-catalog/src/package.rs`, `xtask/src/catalog_build.rs`, `xtask/src/trust.rs`, `trust/roots.eadl`, `docs/semantics/conformance.md`, `docs/tasks/M3.md` and both indexes; the
    decision record, its index row, the policy's two rows, the inventory's two rows; the knowledge map regenerated →
    `bash knowledge-map/scripts/check_knowledge_map.sh` → `OK`.
  - [x] **ADDRESSED (verified)** — `docs/decisions/` → 4 154 lines, 349 217 bytes; `docs/specs/` → 3 311 lines,
    300 288 bytes; `bash scripts/check_readme_routes.sh` → `readme-routes: OK`; `git grep` for the old path outside
    sealed history and dated rows → 0 lines.
  - [x] **NO REGRESSION** — `make focused` → `tier focused: passed — 3 passed, 0 failed`; `bash scripts/check_doctrines.sh` → `=== all doctrines
    green ===`.
  - [x] **LOCKSTEP** — the book cites the record by name only; both indexes, the policy, the inventory, this leaf and
    its log rows, `CHANGELOG.md`.
  Verification: `2026-10-05` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0445 (leaf PROGRAM.56)`
