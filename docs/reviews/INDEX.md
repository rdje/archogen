# Review histories — Index

The independent reviews of a design, one file per design, each round appended and none edited. A design record
under `docs/decisions/` states the design as it stands and keeps one summary row per round. Its review history is
here, because it only grows while the review is open, and it is evidence rather than something to read to resume.
No bootstrap read includes this directory.

- **Adding a round:** append its paragraph and table to the design's file, and its summary row to the design
  record's `## Review`.
- **A new design under review:** a file named as its design record with `-reviews` added, and its row below.
- **When the review closes,** the file is frozen: its last round is the one that found no defect. It may then move
  to `docs/review-history/`, byte for byte, behind a stub (`bash scripts/check_review_history.sh --seal <FILE>`).

| Review history | Design record | Rounds | Status |
| --- | --- | --- | --- |
| [`decision_catalog-records-reviews.md`](decision_catalog-records-reviews.md) | [`decision_catalog-records.md`](../specs/catalog/decision_catalog-records.md) | 16 | closed: the record met its closure rule on round 16 (`M2.7.1`) |
| [`decision_decisions-folder-ceiling-reviews.md`](decision_decisions-folder-ceiling-reviews.md) | [`decision_decisions-folder-ceiling.md`](../decisions/decision_decisions-folder-ceiling.md) | 1 | closed: answered before the raise landed (`PROGRAM.38`) |
| [`decision-history-reviews.md`](decision-history-reviews.md) | `DECISION-HISTORY`: the header of `scripts/check_decision_history.sh`, and [`decision_decisions-folder-ceiling.md`](../decisions/decision_decisions-folder-ceiling.md)'s sealing bullet | 1 | closed: answered with the gate's hardening (`PROGRAM.41.1`) |
| [`review-history-reviews.md`](review-history-reviews.md) | `REVIEW-HISTORY`: the header of `scripts/check_review_history.sh`, and [`decision_reviews-folder-ceiling.md`](../decisions/decision_reviews-folder-ceiling.md)'s dated line | 1 | closed: answered with the gate's hardening (`PROGRAM.55.1`) |
| [`decision_task-tree-sealing-reviews.md`](decision_task-tree-sealing-reviews.md) | [`decision_task-tree-sealing.md`](../decisions/decision_task-tree-sealing.md) | 1 | closed: answered with the tool's hardening (`PROGRAM.32.4`) |
| [`decision_runtime-analysis-variant-reviews.md`](decision_runtime-analysis-variant-reviews.md) | [`decision_runtime-analysis-variant.md`](../decisions/decision_runtime-analysis-variant.md) | 9 | closed: `M2.11`'s change met its closure rule on its fifth review (`M2.11`) |
| [`catalog-check-protection-reviews.md`](catalog-check-protection-reviews.md) | the repository half of `M2.7.6`: the headers of `scripts/check_workflow_tokens.sh`, `scripts/catalog_check.sh` and `.github/workflows/catalog-check.yml` | 1 | open: rounds 2 and 3 not completed; the next is the director's reviewer's (`M2.7.6.4`, findings §11) |
| [`decision_catalog-records-port-reviews.md`](decision_catalog-records-port-reviews.md) | [`decision_catalog-records-port.md`](../specs/catalog/decision_catalog-records-port.md), §14 of the catalog record | 7 | closed: round 7 found no defect (`M2.12.2`) |
| [`decision_catalog-records-port-statement-reviews.md`](decision_catalog-records-port-statement-reviews.md) | §14.4 of [`decision_catalog-records-port.md`](../specs/catalog/decision_catalog-records-port.md), the port's statement | 7 | closed: round 7 found no defect (`M2.12.3`) |
| [`decision_runtime-composite-inputs-reviews.md`](decision_runtime-composite-inputs-reviews.md) | [`decision_runtime-composite-inputs.md`](../specs/catalog/decision_runtime-composite-inputs.md) | 11 | closed: the record met its closure rule on round 11 (`M2.10.1`) |
| [`decision_substitutability-relation-reviews.md`](decision_substitutability-relation-reviews.md) | [`decision_substitutability-relation.md`](../decisions/decision_substitutability-relation.md) | 31 | closed `2026-10-05`: round 31 found no defect; rounds 1–31 found 7, 7, 8, 3, 3, 3, 5, 1, 2, 8, 7, 8, 9, 9, 8, 3, 6, 2, 2, 2, 3, 3, 3, 2, 1, 2, 1, 2, 1, 1 and 0, rounds 17–31 against the executable model |
| [`decision_trust-inventory-reviews.md`](decision_trust-inventory-reviews.md) | [`decision_trust-inventory.md`](../specs/trust/decision_trust-inventory.md) | 11 | closed `2026-10-05`: round 11 found no defect; rounds 1–11 found 11, 12, 16, 11, 10, 10, 8, 11, 4, 1 and 0, the last four against the instrument |
| [`decision_trust-generated-sources-reviews.md`](decision_trust-generated-sources-reviews.md) | [`decision_trust-generated-sources.md`](../specs/trust/decision_trust-generated-sources.md) | 1 | open: round 1 found 19 defects, every one answered; round 2 next |
| [`rt-static-up-v1-faults-reviews.md`](rt-static-up-v1-faults-reviews.md) | [`rt-static-up-v1-faults.md`](../profiles/rt-static-up-v1-faults.md), rounds R3–R15 (R1 and R2 stay in `decision_runtime-contract-gaps.md`) | 13 | closed: the fifteenth round found no defect, `2026-10-02`; moved here `2026-10-03` (`PROGRAM.50`) |
| [`rt-static-up-v1-faults-observation-reviews.md`](rt-static-up-v1-faults-observation-reviews.md) | [`rt-static-up-v1-faults-observation.md`](../profiles/rt-static-up-v1-faults-observation.md), the fault paths' observation events | 7 | closed: round 7 found no defect (`M2.15`) |
