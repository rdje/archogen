# Decision & Fact Records — Index (memory layer C)

Durable, cross-cutting facts and decisions live here, one record per file (ADR-style). Every
record must be listed below (the MEMORY-ARCH doctrine check enforces it). New record: copy
`TEMPLATE.md` → `<type>_<short-kebab-slug>.md`, fill it in, and add its row.

| Record | Type | One-line hook |
| --- | --- | --- |
| [`decision_eadl-engine-boundary.md`](decision_eadl-engine-boundary.md) | `decision` | eADL describes functionality and contains no implementation — the three tests, the eight worked cases, and what the boundary rules out |
| [`decision_emulator-independence-retained.md`](decision_emulator-independence-retained.md) | `decision` | the independent emulator stays: it is the only thing that can contradict a catalog fact, and no timing claim rests on it — `M2.8` reframed as making the platform facts *checked* rather than asserted |
| [`decision_findings-for-director-review.md`](decision_findings-for-director-review.md) | `project` | six items for the director, **two needing rulings** — §5's five runtime-contract gaps and §6's two behaviour-changing criticisms of the §3.1.1 amendment; §1 (no board) open, §2 (QEMU) resolved, §3–§4 tracked or informational |
| [`decision_priority-comparison-direction.md`](decision_priority-comparison-direction.md) | `decision` | a numerically lower `priority` is a higher priority — `1` is highest; §15 puts the direction under migration discipline |
| [`decision_push-cadence.md`](decision_push-cadence.md) | `decision` | push at **400 commits** ahead of `origin/main` — ruled `2026-09-28`; the recommendation was 25-or-7-days and the trade is recorded, and the check must not be able to deadlock the repo |
| [`decision_repository-boundary-read-only.md`](decision_repository-boundary-read-only.md) | `decision` | never write into another git repository or a vendored submodule — the rule lived only in conversation, upstream published a boundary-violation disclosure, `PROGRAM.11` owns the gate |
| [`decision_runtime-contract-gaps.md`](decision_runtime-contract-gaps.md) | `decision` | five runtime-semantics questions the contract does not decide, found by independent derivation — four need a reviewed decision |
| [`decision_s0-retirement.md`](decision_s0-retirement.md) | `decision` | every hard-coded S0 assumption with the leaf that removes it — marked in the source, checked on every commit |
| [`decision_zero-dependency-engine-core.md`](decision_zero-dependency-engine-core.md) | `decision` | the engine crates depend on `std` and nothing else — §4.4 trust, §10.3 locked builds, §5.5 diagnostic wording |
| [`reference_external-document-source-chipdoc.md`](reference_external-document-source-chipdoc.md) | `reference` | ISA, RISC-V, devicetree and peripheral specifications come from the read-only `chipdoc` corpus, mapped by its `ARCHOGEN.md` and queried through its index — requests are operator-relayed, never a build dependency |
