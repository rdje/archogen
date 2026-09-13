<!-- knowledge-map/subsystems.md — the ONE hand-curated input to the derived Knowledge Map.
     Edit this to give a fast orientation to the project's key subsystems / entry points.
     gen_knowledge_map.sh embeds this section verbatim; the task-tree and decision sections
     are generated automatically. -->

Responsibility boundaries follow `ROADMAP.md` §4.2. A crate is created when a real consumer
justifies the split — the rows below appear as that happens.

- `crates/app/` — the starter binary inherited from the template (package `archogen`).
  Replaced by the `osgen` CLI shell in leaf `PROGRAM.2`.
- `ROADMAP.md` — the program's direction, milestone exit gates, and the F01–F30 acceptance
  matrix. The single source of what "done" means.
- `docs/tasks/PROGRAM.md` — the roadmap-unit → tree map and the fixture-ownership map.
  Start here to find which tree owns a given roadmap item.
- `scripts/check_doctrines.sh` — the doctrine enforcer (git hook + CI). `make gate`.
