# Accepted designs move out of the decisions folder, to docs/specs/

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.43` (`docs/tasks/PROGRAM.md`), on the director's ruling of `2026-10-01`.
  - The question put: "`docs/decisions/` is at 388 884 of its 393 216-byte ceiling (about 4 KB left), and that
    ceiling was your one-time raise. The next decision record won't fit. How should the folder make room?"
  - The options: "Move accepted designs out", "Raise the cap again, reviewed", and "Hold the ceiling".
  - The answer: "Move accepted designs out".

## The fact / decision

A design record that is accepted, its independent review closed under its closure rule, moves from
`docs/decisions/` to `docs/specs/<subject>/` when the decisions folder needs the room. The first move is the catalog
design and the composition of the runtime variant's inputs: the six records of `docs/decisions/catalog/`, accepted
by `M2.7.1` and `M2.10.1`, now in `docs/specs/catalog/`. `docs/specs/` is a routed destination of its own, with its
own ceilings, reached from `docs/decisions/`'s onward cell. No ceiling is raised.

## Why

**Measured `2026-10-01`**, with `git ls-files docs/decisions | xargs cat | wc -c`: 388 884 bytes before the move,
202 267 after it. The ceiling of 393 216 was the director's one-time raise
([[decision_decisions-folder-ceiling]]), which only a new ruling reopens, and the next decision record would not
have fit.

An accepted design is a specification its code is built against, as `docs/analysis/cost-accounting-v1.md` and
`docs/profiles/` already are, and it changes from then on only through review. The decisions folder keeps what is
still being decided. The alternatives were a second reviewed raise, which only moves the pressure, and holding the
ceiling, which would have stopped every new decision until something was sealed.

## How to apply

- A moved record keeps its name, its `decision` type and its review history in `docs/reviews/`. The decisions
  index still lists it, by its new path, so it is found where decisions are looked for.
- Every reference follows the move, apart from sealed history and dated notes, which say what was true then.
- `docs/specs/` is held by `README-ROUTES` to its own row in `README_POLICY.md`, and listed in
  `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`.
- Related: [[decision_decisions-folder-ceiling]], [[decision_catalog-records]],
  [[decision_runtime-composite-inputs]].
