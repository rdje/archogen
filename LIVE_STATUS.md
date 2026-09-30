# LIVE_STATUS.md — authoritative live progress tracker

A snapshot of **now**, never a history. Rows use ONLY these four states: **Done · Mostly Done ·
In Progress · Not Started**. A row says its status, its frontier head and any blocker. Update it when
one of those changes, and only then. What each closed leaf did lives in its tree's Commit Log and in
`CHANGELOG.md` (`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`).

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (`bedrock`) | Done | memory architecture · task-trees · commit workflow · doctrine enforcement · **claim verification** (adopted `2026-09-27`) · mdBook skeleton |
| Roadmap seeded into task-trees | Done | eleven trees: `PROGRAM`, `API`, `M0`, `S0`, `M1`–`M7`; F01–F30 each owned |
| `API` — programmatic interface (§10.4) | In Progress | frontier `API.5`: the wasm binding; the engine API is declared (`1.2`), an instance defined, and a request's cost bounded (`API.3`, `API.4`) |
| `PROGRAM` — workspace, tiers, book, ledger | In Progress | frontier `PROGRAM.34`: nested vendored checkouts off their pins, awaiting the director; the `integration` job awaits its first run (`.10.5`); `.31`/`.32` wait on the director (findings §8) |
| `M0` — charter, boundary, profile, target | Done | all seven leaves closed; F27 green. The board remains a recorded blocker, not a passed gate |
| `S0` — early executable generation (F28) | Done | **F28 green**; every leaf closed — `S0.8` made the book chapter's counts and corpus table measured |
| `M1` — eADL description foundation | Mostly Done | §12 M1's exit gate met; every leaf closed but `M1.29.4`, which waits on the director (findings §7) |
| `M2` — one engine realization + controls | In Progress | **F18 and F29 green**; frontier `M2.6.3`: expected results for the runtime analysis variant, derived independently; the variant is decided and built (`M2.6.1`, `M2.6.2`); `M2.8.3`, the eADL side of the target, remains |
| `M3` — joint resolver + checked plan | Not Started | frontier `M3.1` |
| `M4` — generated system + simulator | Not Started | frontier `M4.1` |
| `M5` — physical execution evidence | Not Started | **blocked: no board procured** (2026-09-13) — director decision |
| `M6` — reuse and extension | Not Started | gated on `M4` |
| `M7` — first supported release | Not Started | gated on `M5`, `M6` |
