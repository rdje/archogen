# Decision & Fact Records — Index (memory layer C)

Durable, cross-cutting facts and decisions live here, one record per file (ADR-style). Every
record must be listed below (the MEMORY-ARCH doctrine check enforces it). New record: copy
`TEMPLATE.md` → `<type>_<short-kebab-slug>.md`, fill it in, and add its row.

| Record | Type | One-line hook |
| --- | --- | --- |
| [`decision_eadl-engine-boundary.md`](decision_eadl-engine-boundary.md) | `decision` | eADL describes functionality and contains no implementation — the three tests, the eight worked cases, and what the boundary rules out |
| [`decision_findings-for-director-review.md`](decision_findings-for-director-review.md) | `project` | four items for the director: no board, no QEMU, a spine gate with no seam, and a green gate that was blind to a whole class of error |
| [`decision_priority-comparison-direction.md`](decision_priority-comparison-direction.md) | `decision` | a numerically lower `priority` is a higher priority — `1` is highest; §15 puts the direction under migration discipline |
| [`decision_runtime-contract-gaps.md`](decision_runtime-contract-gaps.md) | `decision` | five runtime-semantics questions the contract does not decide, found by independent derivation — four need a reviewed decision |
| [`decision_s0-retirement.md`](decision_s0-retirement.md) | `decision` | every hard-coded S0 assumption with the leaf that removes it — marked in the source, checked on every commit |
| [`decision_zero-dependency-engine-core.md`](decision_zero-dependency-engine-core.md) | `decision` | the engine crates depend on `std` and nothing else — §4.4 trust, §10.3 locked builds, §5.5 diagnostic wording |
