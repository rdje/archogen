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
| `docs/book/src/verification.md` | four members | record | measured `2026-09-30` by `scripts/wasm_build.sh --list`; the sentence gives its date, and the step re-derives the set on every run |
| `docs/book/src/modules.md` | 100 bytes | record | measured `2026-09-30` by leaf `M1.39`'s fan-out probe, before the module limits; the sentence places it before the leaf, and `crates/eadl-front/tests/module_limits.rs` holds the limits that ended it |
| `docs/semantics/reference.md` | 100 bytes | record | measured `2026-09-30` by leaf `M1.39`'s fan-out probe; §6 rule 11 says it was taken before the rule existed |
| `docs/book/src/annex-repository.md` | 633 lines | record | `M1.md`'s 7 633 lines before its seal, measured `2026-09-30` at `940baf1`'s parent with `git show … \| wc -l`, leaf `PROGRAM.32.4`; the sentence names the sealing commit |
| `docs/book/src/annex-repository.md` | two lines | not-a-count | the stub's form, a definition `TASK-HISTORY` enforces, not a carried measurement |
| `docs/book/src/book-index.md` | three kinds | gated | `scripts/check_book_index.sh` refuses an index that differs from the headings it copies — here *Three kinds of success* (the use cases) and *Three kinds of total…* (the scheduling checker) — so each is exactly the heading's figure, which its chapter keeps |
| `docs/book/src/book-index.md` | three tests | gated | `scripts/check_book_index.sh` refuses an index that differs from the headings it copies — here *The three tests* (the boundary) |
| `docs/book/src/annex-repository.md` | 42,110 bytes | record | measured `2026-09-30` by leaf `PROGRAM.17.2` (`e312afc`), the size `LIVE_STATUS.md` had grown to before the status pages were bounded; the sentence says "until", past, and `LIVE-SNAPSHOTS` now bounds the page; moved here from `verification.md` `2026-10-02` (`PROGRAM.47.5.12`) |
| `docs/book/src/annex-repository.md` | 21 lines | record | measured `2026-09-30` by leaf `PROGRAM.17.2` (`e312afc`), with the 42,110 bytes beside it; moved here from `verification.md` `2026-10-02` (`PROGRAM.47.5.12`) |
| `docs/book/src/annex-repository.md` | 30,256 bytes | record | measured `2026-09-30` by leaf `PROGRAM.17.2` (`e312afc`), the longest row before the bound; moved here from `verification.md` `2026-10-02` (`PROGRAM.47.5.12`) |
| `docs/book/src/annex-repository.md` | seven commits | record | found `2026-09-30` by leaf `PROGRAM.20.1` (`f72b50e`) when `STATED-ORDER` was written: the changelog's first entry, then seven commits old; moved here from `verification.md` `2026-10-02` (`PROGRAM.47.5.12`) |
| `docs/book/src/engine-api.md` | 257 diagnostics | record | measured `2026-10-02` by `API.6.5`'s review on a release build, a 1 MB line of `(`; the sentence places it before the excerpt window of leaf `API.6.6`, and `crates/archogen-api/tests/check.rs` holds the bound that replaced it |
