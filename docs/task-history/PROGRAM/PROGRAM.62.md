- ID: `PROGRAM.62`
  Status: `done` — filed and closed `2026-10-06`
  Goal: a tree's Current Frontier holds its frontier, not the closure narratives of leaves it sealed long ago, so
  `docs/tasks/` keeps room for live work.
  Reproduce / issue: after `ARCHOGEN-M3-0463`, `cat $(git ls-files docs/tasks) | wc -c` → 819 056 of the 819 200-byte
  ceiling, and `bash scripts/check_task_history.sh --seal <TREE>` → *"nothing to seal"* for every tree: each closed
  leaf left sits under a subtree with an open one. `M1.md`'s Current Frontier was 31 877 bytes, of which its table
  400: the rest, 28 paragraphs headed *"`M1.x` is closed"* and four more of the same kind, each a closed leaf's story.
  `PROGRAM.md`'s held three such paragraphs, one stale: *"`M1` is open with its frontier at `M1.13`"*.
  Root cause: `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` says a snapshot takes current state and a closure narrative belongs
  in the tree's Commit Log and `CHANGELOG.md`, but `LIVE-SNAPSHOTS` bounds four snapshot files and no tree's frontier
  section, so closures accumulated there as each leaf closed.
  Fix: both frontiers cut to their table and one paragraph of current state. Census first: every leaf a removed
  paragraph narrates as closed, 28 in `M1` and 3 in `PROGRAM`, has its row in its tree's Commit Log (a script over
  both sections → `missing rows []`), and each leaf's full record is sealed under `docs/task-history/`; the four other
  paragraphs narrate `M1.28`, `M1.26`, `M1.12` and `M1.1`–`M1.9`, all closed and sealed.
  Verification: `docs/tasks/` 819 056 → 786 004 bytes (`M1` −31 087, `PROGRAM` −1 965); `stated-order: OK`; the
  doctrine gate at commit. A gate that bounds a tree's frontier section is not built here: a size cap would be
  arbitrary, and the narrative shape has no clean signature. Left to review.
