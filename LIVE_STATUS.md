# LIVE_STATUS.md — authoritative live progress tracker

A snapshot of **now**, never a history. Rows use ONLY these four states: **Done · Mostly Done ·
In Progress · Not Started**. A row says its status, its frontier head and any blocker. Update it when
one of those changes, and only then. What each closed leaf did lives in its tree's Commit Log and in
`CHANGELOG.md` (`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`).

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (`bedrock`) | Done | memory architecture · task-trees · commit workflow · doctrine enforcement · **claim verification** (adopted `2026-09-27`) · mdBook skeleton |
| Roadmap seeded into task-trees | Done | eleven trees: `PROGRAM`, `API`, `M0`, `S0`, `M1`–`M7`; F01–F30 each owned |
| `API` — programmatic interface (§10.4) | In Progress | frontier `API.5`: the wasm binding built, checked against the CLI in the integration tier, and given its page; the director saw its Check answer match the book in a browser; the load answer and the browser's name remain (`API.5.5`) |
| `PROGRAM` — workspace, tiers, book, ledger | In Progress | frontier `PROGRAM.32`: closed leaves sealed out of the task trees (§8, ruled), blocked on the director's hold on `scripts/`; review histories under `docs/reviews/` (`.36`); `.34` awaits the director, `.10.5` the next push |
| `M0` — charter, boundary, profile, target | Done | all seven leaves closed; F27 green. The board remains a recorded blocker, not a passed gate |
| `S0` — early executable generation (F28) | Done | **F28 green**; every leaf closed — `S0.8` made the book chapter's counts and corpus table measured |
| `M1` — eADL description foundation | Mostly Done | §12 M1's exit gate met; every leaf closed but `M1.29.4`, which waits on the director (findings §7) |
| `M2` — one engine realization + controls | In Progress | **F17 (analysis half), F18 and F29 green**; `riscv-virt-up` verified, and the spike runs on it (`M2.8`); the integration tier passes; frontier `M2.7.1`, the catalog record: round 7 answered, round 8 next; `M2.10.1`'s composite inputs, written |
| `M3` — joint resolver + checked plan | Not Started | frontier `M3.1` |
| `M4` — generated system + simulator | Not Started | frontier `M4.1` |
| `M5` — physical execution evidence | Not Started | **blocked: no board procured** (2026-09-13) — director decision |
| `M6` — reuse and extension | Not Started | gated on `M4` |
| `M7` — first supported release | Not Started | gated on `M5`, `M6` |
| `TEMPLATE-REFS` — no reference to the project template | Not Started | frontier `TEMPLATE-REFS.1`: **postponed by the director** (2026-09-30); `scripts/` left alone until the template's reworked spine can be taken |
