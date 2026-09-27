# LIVE_STATUS.md — authoritative live progress tracker

Rows use ONLY these four states: **Done · Mostly Done · In Progress · Not Started**.
Review and update before every commit whenever actual closure or remaining scope changes;
summarize the snapshot in every commit-workflow completion message.

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (`bedrock`) | Done | memory architecture · task-trees · commit workflow · doctrine enforcement · mdBook skeleton |
| Roadmap seeded into task-trees | Done | ten trees: `PROGRAM`, `M0`, `S0`, `M1`–`M7`; F01–F30 each owned |
| `PROGRAM` — workspace, tiers, book, ledger | In Progress | `.1` `.1.1` `.2` `.2.1` `.3` `.4` `.12` done — §14.3 tiers run as `make focused` / `make integration`; the book is structured and its citations checked; harness-local scratch is ignored, so a clean `git status` means handoff-ready. Frontier `.11` (the repository boundary in **both** directions — the rule existed only in a session prompt, and an inbound crossing got recorded backwards), then `.5`; `.6`–`.10` and `.13`–`.14` pending |
| `M0` — charter, boundary, profile, target | Done | all seven leaves closed; F27 green. The board remains a recorded blocker, not a passed gate |
| `S0` — early executable generation (F28) | Done | all six leaves closed; **F28 green** end to end, provenance included; the prototype carries an expiry enforced by `S0-RETIREMENT` |
| `M1` — eADL description foundation | Mostly Done | §12 M1's exit gate met; **the surface syntax is now normative** (`docs/semantics/grammar.md`) and token-checked against the reader. The LinkedSpec pin is the vendor's **latest published head** `2ac834913` (`M1.19` → `M1.19.1`), and the re-measurement is four fifths done: **LS-005, LS-001, LS-004 and LS-002 are `verified`** on archogen's own reruns, each through a new property instrument carrying RED arms — **both reported blockers are closed**, and the tracker's "single most valuable ask" is measured as delivered: the document route returns all four top-level forms of the real S0 description and rejects trailing, intervening and leading junk with no partial value. Three standing consequences: the workspace root carries `exclude = ["vendor/linkedspec"]`; the vendored parser was regenerated at this pin (it had arrived with the previous pin's); and `M1.14`'s closure of the independent-recognizer question no longer rests on anything, so `M1.22` reopens it. **421 tests pass**. Frontier `.20.6` (LS-003, the last row on the vendor's word), then `.12` (language reference), `.13` (freeze), `.10`, `.21`, `.22` |
| `M2` — one engine realization + controls | In Progress | `.1`–`.5` done — **F18 and F29 green**; an independent reference agrees over 16 000 events and exposed 5 contract gaps. Frontier `.8` (pin QEMU); `.9` blocked on a director decision |
| `M3` — joint resolver + checked plan | Not Started | frontier `M3.1` |
| `M4` — generated system + simulator | Not Started | frontier `M4.1` |
| `M5` — physical execution evidence | Not Started | **blocked: no board procured** (2026-09-13) — director decision |
| `M6` — reuse and extension | Not Started | gated on `M4` |
| `M7` — first supported release | Not Started | gated on `M5`, `M6` |
