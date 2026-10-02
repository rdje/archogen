- ID: `API.7`
  Status: `done` — `2026-10-02`
  Goal: the book chapter for the programmatic interface.
  Acceptance: a chapter states what an agent and a browser can do, what they cannot, and which verdicts
  they receive; it cites the API, the bindings and the decision record; `BOOK-ANCHORS` and
  `reference.rs`'s legs 8 and 9 pass over it; `PROGRAM.24`'s coverage question is answered for every
  crate this tree adds, so the tree does not create the drift it was filed to end.
  Priority: **medium, and not optional** — the director reads the book and not the code. Filed as its own
  leaf rather than folded into `API.6` so it cannot be quietly skipped when the server lands.
  **Done.** `docs/book/src/engine-api.md` gains *What each consumer can do, and what it receives*: the director's
  ruling cited (`decision_programmatic-interface.md`), and a table of the four doors — the command line, the library,
  the wasm binding and the MCP server — with what each can do and what it receives, each citing its crate and
  record; every one receives §5.5's verdict when judged, none can generate or verify, and only the person's tool
  reads files. And *Today and ahead*: `check` today through all four, held by the parity legs, the `wasm-binding`
  step and the stdio test; the offered commands joining with their leaves (`M3.4`, `M2.6`, `M4.7`); `verify` awaiting
  `PROGRAM.3`'s ruling, `build` outside by §10.4; findings §9 open. A paragraph that sat inside a list moved after it.
  `PROGRAM.24`'s question for this tree's crates: `archogen-api` and `archogen-wasm` are each named in the chapter
  beside a path into them, which `book-coverage` checks for every workspace member.
  Verification: `cargo test -q -p eadl-front --test reference` → `55 passed` (legs 8 and 9 over every chapter);
  `cargo test -q -p archogen-cli --test mcp_stdio` → `3 passed`, the chapter's transcript replayed; `bash
  scripts/check_doctrines.sh` → `=== all doctrines green ===`, `book-anchors` and `book-coverage` among them
  Commit: `ARCHOGEN-API-0353 (leaf API.7)`
