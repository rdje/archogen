# LIVE_STATUS.md — authoritative live progress tracker

A snapshot of **now**, never a history. Rows use ONLY these four states: **Done · Mostly Done ·
In Progress · Not Started**. A row says its status, its frontier head and any blocker. Update it when
one of those changes, and only then. What each closed leaf did lives in its tree's Commit Log and in
`CHANGELOG.md` (`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`).

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (`bedrock`) | Done | memory architecture · task-trees · commit workflow · doctrine enforcement · **claim verification** (adopted `2026-09-27`) · mdBook skeleton |
| Roadmap seeded into task-trees | Done | eleven trees: `PROGRAM`, `API`, `M0`, `S0`, `M1`–`M7`; F01–F30 each owned |
| `API` — programmatic interface (§10.4) | In Progress | `API.5` done: the wasm binding answers as the CLI does, and its page ran in Chrome 154 as the book says; frontier `API.6`, the MCP server, behind the main line |
| `PROGRAM` — workspace, tiers, book, ledger | In Progress | frontier `PROGRAM.32`: `M1` and `PROGRAM` sealed to `docs/task-history/`, its review next; then `.39`, the decisions folder's partition; `.34` awaits the director |
| `M0` — charter, boundary, profile, target | Done | all seven leaves closed; F27 green. The board remains a recorded blocker, not a passed gate |
| `S0` — early executable generation (F28) | Done | **F28 green**; every leaf closed — `S0.8` made the book chapter's counts and corpus table measured |
| `M1` — eADL description foundation | Mostly Done | §12 M1's exit gate met; every leaf closed but `M1.29.4`, which waits on the director (findings §7) |
| `M2` — one engine realization + controls | In Progress | **F17 (analysis half), F18 and F29 green**; `riscv-virt-up` verified, and the spike runs on it (`M2.8`); the integration tier passes; frontier `M2.7.1`, the catalog record: round 9 answered, round 10 next; `M2.10.1`'s composite inputs, written |
| `M3` — joint resolver + checked plan | Not Started | frontier `M3.1` |
| `M4` — generated system + simulator | Not Started | frontier `M4.1` |
| `M5` — physical execution evidence | Not Started | **blocked: no board procured** (2026-09-13) — director decision |
| `M6` — reuse and extension | Not Started | gated on `M4` |
| `M7` — first supported release | Not Started | gated on `M5`, `M6` |
| `TEMPLATE-REFS` — no reference to the project template | Not Started | frontier `TEMPLATE-REFS.1`: **postponed by the director** (2026-09-30); the 17 template files archogen has not changed are held until the template's reworked spine can be taken (findings §10) |
