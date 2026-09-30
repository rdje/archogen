# Review histories — Index

The independent reviews of a design, one file per design, each round appended and none edited. A design record
under `docs/decisions/` states the design as it stands and keeps one summary row per round. Its review history is
here, because it only grows while the review is open, and it is evidence rather than something to read to resume.
No bootstrap read includes this directory.

- **Adding a round:** append its paragraph and table to the design's file, and its summary row to the design
  record's `## Review`.
- **A new design under review:** a file named as its design record with `-reviews` added, and its row below.
- **When the review closes,** the file is frozen: its last round is the one that found no defect.

| Review history | Design record | Rounds | Status |
| --- | --- | --- | --- |
| [`decision_catalog-records-reviews.md`](decision_catalog-records-reviews.md) | [`decision_catalog-records.md`](../decisions/catalog/decision_catalog-records.md) | 10 | open (`M2.7.1`) |
| [`decision_decisions-folder-ceiling-reviews.md`](decision_decisions-folder-ceiling-reviews.md) | [`decision_decisions-folder-ceiling.md`](../decisions/decision_decisions-folder-ceiling.md) | 1 | closed: answered before the raise landed (`PROGRAM.38`) |
| [`decision_task-tree-sealing-reviews.md`](decision_task-tree-sealing-reviews.md) | [`decision_task-tree-sealing.md`](../decisions/decision_task-tree-sealing.md) | 1 | closed: answered with the tool's hardening (`PROGRAM.32.4`) |
| [`decision_runtime-analysis-variant-reviews.md`](decision_runtime-analysis-variant-reviews.md) | [`decision_runtime-analysis-variant.md`](../decisions/decision_runtime-analysis-variant.md) | 4 | closed: the record is implemented (`M2.6`) |
| [`decision_runtime-composite-inputs-reviews.md`](decision_runtime-composite-inputs-reviews.md) | [`decision_runtime-composite-inputs.md`](../decisions/catalog/decision_runtime-composite-inputs.md) | 3 | open (`M2.10.1`) |
