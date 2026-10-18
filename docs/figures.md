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
| `docs/book/src/trust-adequacy.md` | 91 tests | record | measured `2026-10-11` by the exact native parent baselines of `M3.6.9.4.2.2.2`, `M3.6.9.5.2.2` and `M3.6.9.5.3.2`; their paragraphs date the measurements, whose outputs are `target/m369-pairs/parent-population-baseline.log` , `operator-generated-parent-baseline.log` and `operator-trust-parent-baseline.log` in that directory |
| `docs/book/src/trust-adequacy.md` | 30 entries | record | measured `2026-10-11` by `M3.6.9.4.2.2.2`'s strict native exact parent run, raw exit 1; the dated paragraph and per-entry table retain these initial survivors as repair obligations |
| `docs/book/src/annex-trust-proofs.md` | 91 tests | record | the initial native parent baseline of `2026-10-11`, `M3.6.9.4.2.2.2`; the linked measurement chapter dates it and the leaf records its native output |
| `docs/book/src/annex-trust-proofs.md` | 2 of the 32 | record | the initial exact parent outcomes of `2026-10-11`, `M3.6.9.4.2.2.2`; the linked dated measurement chapter records each outcome and actual killing test, without claiming final adequacy |
| `docs/book/src/trust-adequacy.md` | 269 row | record | measured `2026-10-11` by `M3.6.9.5.1`’s tracked operator planner, summing census-to-candidate memberships; the paragraph dates this overlap-planning count and disclaims semantic coverage |
| `docs/book/src/trust-adequacy.md` | Four entries | record | measured `2026-10-11` by `M3.6.9.5.4.1` using tracked scripts/trust_witness.py and four initial native datasets; dated syntax table lists all four formatted-source matches, explicitly separate from semantic reach |
| `docs/book/src/trust-adequacy.md` | three file | record | reviewed `2026-10-11`, `M3.6.9.5.4.2`; the compiled contrast compares all three file-position candidates against the depth gate, as the dated semantic section records |
| `docs/book/src/annex-trust-proofs.md` | twelve steps | record | measured `2026-10-12`, `M3.6.9.6.3.2.2.10.4`: complete local `make integration` rerun passes every actual step; native ordered outcomes and activation-tree stamp retained, not a current or remote CI assertion |
| `docs/book/src/annex-trust-proofs.md` | 52 gate | record | measured `2026-10-12`, `M3.6.9.6.3.2.2.10.4`: the self-test runner discovers 52 entrypoints, independently recounted by Python and rg; unchanged population passes through full integration, including tool controls and the external spine harness |
| `docs/book/src/annex-repository.md` | 928 files | record | measured `2026-10-18`, leaf `PROGRAM.86`: targeted Python metadata and independent native find/stat agree before removal of only main/empty wasm incremental caches; the book dates this cleanup, and target/artifact-cleanup/2026-10-18 retains supplementary manifests and maintained restoration/workflow receipts |
| `docs/book/src/annex-trust-proofs.md` | twelve steps | record | measured `2026-10-18` by `PROGRAM.89.2`: actual local integration tier passes all twelve declared stages; official/native activation and exact scope in its task leaf |
| `docs/book/src/annex-repository.md` | 1600001 bytes | record | measured `2026-10-18` by `PROGRAM.92`: protected Bash input descriptor metadata records a regular file on the repository volume; the book dates the observation and limits its scope, with fd-here-string.json retained in target/handoff-control-repair |
| `docs/book/src/annex-trust-proofs.md` | three diagnostic | record | measured `2026-10-18` by `M3.6.9.6.3.2.2.14.2`: the complete actual guarded native cohort has exactly three surviving diagnostic matcher edits; the dated section and table name each, with raw strict exit and scope retained |
| `docs/book/src/annex-trust-proofs.md` | 1526 production | record | native complete listing measured `2026-10-18` by leaf `M3.6.9.6.3.2.2.14.3.2.2.1`; the paragraph dates the bounded assessment and distinguishes listing from attempted coverage |
| `docs/book/src/annex-trust-proofs.md` | two entries | record | repaired helper source inspected `2026-10-18` by leaf `M3.6.9.6.3.2.2.14.3.2.2.1`: cwd plus optional target, and raw spelling plus optional canonical spelling; E proof/replay remains owned by its successor |
| `docs/book/src/annex-trust-proofs.md` | seven diagnostic | record | open obligations on `2026-10-18`: the complete51 assessment had nine survivors and the finite-iterator proof resolved two; `M3.6.9.6.3.2.2.14.3.2.2.2.2.2.1` preserves the remaining set while repairing its caller, without claiming a fresh seven-candidate measurement |
| `docs/book/src/book-index.md` | Seven diagnostic | record | index of the dated `2026-10-18` assessment `M3.6.9.6.3.2.2.14.3.2.2.2.2.3`; all seven exact remaining raw edits are listed in the linked annex, with no fresh native outcome or equivalence claim |
| `docs/book/src/annex-trust-proofs.md` | 4639 entries | record | pinned Cargo source archive census on `2026-10-18`, `M3.6.9.6.3.2.2.14.3.2.2.2.2.6.1`: tar metadata counts4639 entries and2906 regular files; complete file bytes and30 prior source pins agree, with no dependency build/source amendment |
| `docs/book/src/annex-trust-proofs.md` | 153 tests | record | measured `2026-10-18`, `M3.6.9.6.3.2.2.14.4.1`: original/restored copied full-parent CLI lists agree on153 names, with no ignored parent attributes; actual complete51 replay uses the controlling full command and passes strict0 |
| `docs/book/src/annex-trust-proofs.md` | 26 entries | record | parsed `2026-10-18`, `M3.6.9.6.3.2.2.14.4.1`: native equivalents reader finds26 records in the unchanged per-file source; the bounded replay judges only4 applicable reasons, with no whole-file validity claim |
| `docs/book/src/annex-trust-proofs.md` | 463 rows | record | measured `2026-10-18`, `M3.6.9.6.3.2.2.15.1`: literal pre-completion ownership table in committed8d92d39; every ID binds the initial native plan, then all receive concrete source-lane owners; not a current survivor count |
| `docs/book/src/annex-trust-proofs.md` | 26 records | record | measured `2026-10-18`, `M3.6.9.6.3.2.2.15.2.1.1`: native equivalents parser before complete compiler83 replay counts26 exact records with zero applicable selected keys; no unrelated reason-validity claim |
| `docs/book/src/annex-trust-proofs.md` | 155 tests | record | measured `2026-10-18`, `M3.6.9.6.3.2.2.15.2.1.2.1`: actual original/restored-copy parent CLI names agree after exactly two added Rust parents; all153 previous names retained and selected native4 replay has intended finishing names |
