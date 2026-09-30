- ID: `PROGRAM.24`
  Status: `done`
  Goal: the **mirror direction** of `BOOK-ANCHORS`. That doctrine walks the book and asks whether what
  it cites exists; nothing walks the codebase and asks whether the book describes it. The standing
  instruction is that the roadmap, the codebase and the mdBook stay in lockstep, and the book is the
  director's only window into the project — so an undocumented capability is not a documentation nit.
  It is a feature that, as far as the only reader of it is concerned, does not exist.
  Reproduce / issue: **one live instance, measured `2026-09-28` by `M1.12.5`.** The `rt-analysis`
  crate is named nowhere in `docs/book/`, and `docs/book/src/analysis.md` — the chapter titled *"What
  the scheduling checker establishes"*, which is precisely what that crate does — cites only
  `ROADMAP.md` and `docs/analysis/cost-accounting-v1.md`. It passes `BOOK-ANCHORS` on both legs, and
  that is the finding rather than an excuse: leg 1 asks for *a* repository path and the chapter has
  two, leg 2 asks whether they exist and they do. A chapter can be perfectly anchored and still never
  tell its reader where the thing it describes is implemented.

  ```text
  census: for c in crates/*/; do n=$(basename "$c"); printf '%-22s %s\n' "$n" \
            "$(grep -roF "$n" docs/book/src/ | wc -l | tr -d ' ')"; done
          → archogen-cli 2 · archogen-evidence 1 · archogen-s0 2 · eadl-front 1 · eadl-model 3 ·
            rt-analysis **0** · rt-core 4 · rt-reference 1     (7 of 8 workspace crates named)
  census: grep -rn "rt-analysis" docs/book/                     → no match, exit 1
  census: grep -noE '`(crates|docs|scripts)/[A-Za-z0-9_./-]+`|`ROADMAP\.md`' docs/book/src/analysis.md
          → 4 citations, none inside the crate the chapter is about
  census: grep -rln "docs/book" scripts/ crates/*/tests/*.rs xtask/src/
          → check_readme_stability.sh · check_book_anchors.sh · corpus.rs · reference.rs · kinds.rs ·
            differential.rs · xtask/src/main.rs. Every one starts FROM the book and asks about the
            code; none starts from the code and asks about the book.
  census: grep -n '^members' Cargo.toml                         → members = ["crates/*", "xtask"]
  ```

  Acceptance: the population is **derived from the root manifest's workspace members**, never listed
  inside the check, so a crate added tomorrow is in scope without anyone editing a gate; the rule is
  the stronger of the two shapes below, or the weaker one with the reason for stopping there recorded
  in this leaf —
  1. *weak*: every workspace member is named in at least one chapter. Cheap, and it catches the live
     instance, but a list of crate names in an appendix satisfies it and is worth nothing — the same
     token-citation trap `check_book_anchors.sh`'s own header refuses to set.
  2. *strong*: every workspace member is named in a chapter that **also cites a repository path inside
     that member**. Mechanical, and not satisfiable by an appendix, because a bare name carries no path
     with it. ⭐ This is the shape to build unless measurement says it cannot be met honestly.

  Plus: the `rt-analysis` instance fixed in the same commit, with `analysis.md` naming the crate and
  the fixtures that establish its claims; RED arms in the `--self-test` idiom, each pinning its
  violation count, including one that proves a name-without-a-path does **not** satisfy the strong
  shape; `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md` and `scripts/check_doctrines.project.sh` updated if
  this becomes a doctrine rather than a test; `make focused` exit `0`.

  ⛔ **Deliberately out of scope: `ROADMAP.md` → book.** Nothing gates that direction either, and this
  leaf must not "complete" itself by adding one. The roadmap is direction and exit criteria; the book
  is a description of what the toolchain does for a user. Requiring every roadmap section to appear in
  the book would produce a chapter per milestone and teach exactly the token-mention habit the strong
  shape exists to refuse. Recorded so the absence stays a decision. Where the roadmap *is* load-bearing
  as data, it is already read as data and gated: `crates/rt-analysis/tests/f18_baseline.rs` and
  `f29_preemption.rs` parse §13.2's table out of `ROADMAP.md` and fail loudly if it will not parse,
  because a baseline that quietly shrank would still be green.
  Priority: **medium-high** — a live drift in the one artifact the director reads, and no mechanism in
  the tree that could have found it. Sequenced behind `PROGRAM.21` and `PROGRAM.18` only because those
  are about gates reporting that they checked something they did not, which corrupts every other
  gate's evidence including this one's.
  Verification: see the checklist — the strong shape built, measured on the real book (2 of 9 members
  failing, both fixed), eight arms, four mutations.
  Commit: `ARCHOGEN-PROGRAM-0122 (leaf PROGRAM.24)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the leaf's own census, re-run at `4c6a715`:
    ```text
    $ git grep -c "rt-analysis" b9f6e22 -- docs/book/src/        -> no match, rc=1
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — every book gate starts from the book; none starts from the code:
    ```text
    $ git grep -l "docs/book" 4c6a715 -- scripts/ xtask/src/ 'crates/*/tests/*.rs'
      crates/archogen-cli/tests/module_cases.rs   crates/archogen-cli/tests/module_files.rs
      crates/eadl-front/tests/corpus.rs           crates/eadl-front/tests/reference.rs
      crates/eadl-model/tests/kinds.rs            crates/eadl-model/tests/semantic_corpus.rs
      crates/rt-core/tests/differential.rs        scripts/check_book_anchors.sh
      scripts/check_readme_stability.sh           scripts/selftest_spine.sh      xtask/src/main.rs
    ```
    Each reads a chapter and asks about the code; none walks the workspace and asks about the book. And
    the leaf's census **under-counted**: measured with the strong shape (a chapter naming the member beside a
    path into it, `BOOK-ANCHORS`' notion of a citation), **2 of 9** members fail — `rt-analysis` (named
    nowhere) and **`xtask`**, which `verification.md` names in `cargo xtask verify` and never locates:
    ```text
    $ bash scripts/check_book_coverage.sh      (with the two chapter fixes stashed)
      BOOK-COVERAGE: `rt-analysis` (crates/rt-analysis) is named in no chapter of docs/book/src
      BOOK-COVERAGE: `xtask` (xtask) is named in verification.md without a citation of a path inside xtask
      BOOK-COVERAGE: 2 of 9 workspace member(s) described by no chapter — name each beside a path into it
    ```
    ⛔ A first census with the pattern `` `crates/<name>/<something>` `` reported **5** failures: it missed a
    citation of the member's *directory* (`crates/archogen-s0`), which `BOOK-ANCHORS` accepts and which does
    tell a reader where the crate lives — so the rule accepts the directory or anything beneath it.
  - [x] **FIX** — the strong shape as a doctrine, `BOOK-COVERAGE` (`scripts/check_book_coverage.sh`, registered
    in the project slot): the population is the root `Cargo.toml`'s `members` with globs expanded and each
    package name read from its own manifest; a member passes only when **one** chapter both names it and cites
    a path inside it; `SUMMARY.md` never counts; an empty population is a breach. `analysis.md` gains "Where it
    lives", naming the crate and the file behind each claim, **F18** and **F29** included; `verification.md`
    names `xtask/src/main.rs` as where the tiers are declared.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/check_book_coverage.sh
      book-coverage: OK (9 workspace member(s), each named in a chapter beside a path into it)
    $ bash scripts/check_book_coverage.sh --self-test
      book-coverage self-test: 8 pass / 0 fail (8 arms)
    ```
    ⛔ One arm was unfailable as first written — "name in one chapter, path in another" used a member whose
    directory equals its package name, so the path carried the name — found when it failed on a correct
    check, and re-staged with a member whose name differs from its directory. Four mutations, restored by
    `cmp`: **C-1** the weak, name-only shape → 5 / 3, the **appendix arm** among them; **C-2** the table of
    contents counted → 4 / 4; **C-3** a prefix match on the directory → 7 / 1, the longer-name arm; **C-4** a
    fixed population → 5 / 3, the glob arm and the real tree.
  - [x] **NO REGRESSION** — `cargo test --all --no-fail-fast` → **602 passed, 0 failed over 42 suites**
    (several legs read the book); `scripts/check_doctrines.sh` → `=== all doctrines green ===` with the new
    doctrine; `check_book_anchors.sh` → `OK (19, 3)`; `mdbook build` `rc=0`.
  - [x] **LOCKSTEP** — `analysis.md`, `verification.md` (the two fixes, and a section on the doctrine),
    `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`, `scripts/check_doctrines.project.sh`, the live docs.
