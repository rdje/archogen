# Figures in the live documents, and what keeps each true

`FIGURE-REGISTER` (`scripts/check_figure_register.sh`) refuses a commit that adds a figure-shaped phrase to a
live document without a row here saying what keeps it true (leaf `PROGRAM.20.3`). A row classifies one phrase
in one file:

- `gated` — the named test reads the file and compares the figure with a measurement;
- `record` — a measurement true of its date, never re-read as current;
- `not-a-count` — figure-shaped, but not a carried figure.

The phrases already in the live documents when the check was written are a backlog it reports and does not
refuse; a commit may only lower their number in a file it touches. ⛔ The first row written here claimed
`five tiers` in `docs/book/src/verification.md` was gated by `xtask`'s tier test. The check refused it,
because that test compares the runner's table with the roadmap and never reads the book. The table starts
empty rather than with a false claim.

| File | Phrase | Class | Why |
| --- | --- | --- | --- |
| `docs/book/src/s0.md` | four descriptions | gated | `crates/archogen-cli/tests/s0_chapter.rs` compares it, and the corpus table row by row, with the `.eadl` files in `examples/s0-heartbeat/` |
| `docs/book/src/s0.md` | five files | gated | `crates/archogen-cli/tests/s0_chapter.rs` compares it, and the file table beneath it, with what `archogen build` writes |
