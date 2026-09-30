# TEMPLATE-REFS: archogen carries no reference to the project template it was created from

## Metadata

- Tree ID: `TEMPLATE-REFS`
- Status: `blocked` — postponed by the director on `2026-09-30`
- Roadmap lane: `ROADMAP.md` §11 workstream F (engineering operations)
- Created: `2026-09-30`
- Owner: repo-local workflow

## Goal

archogen was created from a project template, and the template left its name, its version pin, its own history
and its updater behind. The director ruled on `2026-09-30` that archogen should be free of references to that
template. This tree owns doing it, once the director says the template's reworked spine can be taken.

## Non-Goals

- **Nothing under `scripts/` is changed before the director says so.** The template's spine is being reworked and
  debugged, and its files here, `scripts/bootstrap.sh` and `scripts/update_scaffold.sh` among them, will soon be
  updatable from it wherever they have not been amended locally. Removing or editing them now would make that
  update harder, so the director ruled them left alone "just for now".
- No reference is removed by this tree before it is unblocked. The one removal already made, the template's own
  entries in the changelog and the development notes, was part of `PROGRAM.31`, on the director's word that they
  were a defect of the template, since fixed there.

## Acceptance Criteria

- A census of every reference, by file and kind, taken when the tree starts rather than copied from this one.
- Each kind handled as the director rules when unblocking: the live documents, the gates' comments and data, the
  functional files under `scripts/` (by the reworked spine's own update route, never by hand-editing files it
  owns), and the history (closed task-tree leaves and sealed ledger segments, which are immutable and would need
  superseding records).
- A check that refuses a new reference, with RED arms, registered like the other doctrines.

## Task Tree

- ID: `TEMPLATE-REFS`
  Status: `blocked`
  Goal: archogen free of references to the project template
  Children: `TEMPLATE-REFS.1` … `TEMPLATE-REFS.5`

- ID: `TEMPLATE-REFS.1`
  Status: `blocked` — postponed by the director
  Goal: the census, taken afresh when the tree starts, with the director's ruling on how far each kind goes.
  Acceptance: every tracked reference listed by file and kind; the ruling recorded here.
  Measured just before this file was added (`git grep -ic bedrock -- . ':!vendor'`, `2026-09-30`), for scale only:
  134 mentions in 30 files — `scripts/` 55 in 9 files, `docs/tasks/` 41 in 4, `docs/decisions/` 15 in 4,
  `docs/history/` 8 in 5 (sealed segments), `docs/book/` 5 in 1 (the ledger's entry), and one to three each in
  `.gitignore`, `COMMIT.md`, `DOCTRINE_VERSION`, `LIVE_STATUS.md`, `Makefile`, `README_POLICY.md` and
  `cargo-generate.toml`.
  Verification: `pending`
  Commit: `pending`

- ID: `TEMPLATE-REFS.2`
  Status: `blocked` — postponed by the director
  Goal: the live documents free of the template's name: the status board, the ledger's entry, the decision records,
  `COMMIT.md`, `README_POLICY.md`'s adoption note, and the gates' comments.
  Acceptance: none remains in a live document; every gate still passes its self-test.
  Verification: `pending`
  Commit: `pending`

- ID: `TEMPLATE-REFS.3`
  Status: `blocked` — on the template's reworked spine, and the director
  Goal: the functional files — `scripts/bootstrap.sh`, `scripts/update_scaffold.sh`, `DOCTRINE_VERSION`,
  `.gitignore`, `cargo-generate.toml`, the `Makefile` target, the spine harness's arms — handled by the reworked
  spine's own update route, as the director rules: taken, replaced or removed.
  Acceptance: the update route run as the template documents it; nothing under `scripts/` edited by hand.
  Verification: `pending`
  Commit: `pending`

- ID: `TEMPLATE-REFS.4`
  Status: `blocked` — postponed by the director
  Goal: the history's references — closed task-tree leaves and sealed ledger segments — left as history, or
  superseded, as the director rules.
  Acceptance: the ruling recorded; a sealed segment is never edited in place (`docs/decisions/decision_history-ledgers.md`).
  Verification: `pending`
  Commit: `pending`

- ID: `TEMPLATE-REFS.5`
  Status: `blocked` — on `TEMPLATE-REFS.2`–`.4`
  Goal: a gate that refuses a new reference.
  Acceptance: registered as a project doctrine, with RED arms; the history the director keeps named as its only
  exemption.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `TEMPLATE-REFS.1` | `blocked` | postponed by the director (`2026-09-30`) until the template's reworked spine can be taken; nothing under `scripts/` is touched before then |

## Decisions

- `2026-09-30`: the director ruled archogen free of references to the project template, then postponed the work,
  and ruled `scripts/bootstrap.sh` and `scripts/update_scaffold.sh` left in place for now.

## Open Questions

- How far each kind goes (`TEMPLATE-REFS.1`): the director's ruling when the tree is unblocked.

## Blockers

- The director: the template's spine is being reworked; this tree waits for the word that it can be taken.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-30` | `TEMPLATE-REFS` | tree filed; the census taken for scale | 134 mentions in 30 files |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `TEMPLATE-REFS` | `ARCHOGEN-TEMPLATE-REFS-0208` | tree filed, postponed by the director |

## Changelog

- `2026-09-30`: created, on the director's ruling, and postponed on the same day.
