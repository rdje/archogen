- ID: `PROGRAM.30`
  Status: `done`
  Goal: every tool a build or the book depends on is pinned to an exact version, so two machines — or one
  machine a month apart — build with the same tools (§10.3 locked builds; §15 "exact upstream source
  versions").
  Reproduce / issue: found `2026-09-29` by `PROGRAM.5`, deriving the ledger's pin population:
  ```text
  rust-toolchain.toml         channel = "stable"            a moving channel, not a version (measured 1.95.0)
  .github/workflows/rust.yml  dtolnay/rust-toolchain@stable a moving ref
  .github/workflows/*.yml     actions/checkout@v4           a moving major tag, not a commit
  mdbook                      v0.5.2 installed              pinned nowhere; the `book` step builds with whatever is on PATH
  ```
  Acceptance: an exact `rustc` version in `rust-toolchain.toml` and the same in CI; CI actions at commit SHAs
  with the tag as a comment; an mdBook version stated and checked by the `book` step; each ledger row
  (`docs/book/src/ledger.md`) updated in the same commit, which `SOURCE-LEDGER` will insist on.
  Priority: **medium** — nothing differs today (one machine, one toolchain), which is exactly when an
  unpinned toolchain is invisible. ⚠️ Moving to an exact `rustc` is the director's call if it constrains
  how CI is provisioned; the leaf proposes, it does not presume.
  ⭐ **The caution above, resolved by measurement rather than deferred.** An exact `rustc` does not constrain how CI
  is provisioned: CI now installs from the same `rust-toolchain.toml` this machine reads (`rustup toolchain
  install`), and `ROADMAP.md` §10.3 already asks for "the same pinned build environment". And the unpinned state was
  not hypothetical: this machine's `stable` was `1.95.0` while `1.98.0` was installed beside it, so CI's `@stable`
  would have built with a newer compiler than every recorded measurement here.
  Verification: see the checklist — the pins derived and held by `SOURCE-LEDGER` (9); the toolchain installed from its
  file; `scripts/build_book.sh`'s 4 arms and 2 mutations; the `integration` tier unchanged; `1.98.0` measured too.
  Commit: `ARCHOGEN-PROGRAM-0151 (leaf PROGRAM.30)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — three moving references, and a measured divergence behind the first:
    ```text
    $ git show HEAD:rust-toolchain.toml | grep channel           → channel = "stable"
    $ git show HEAD:.github/workflows/rust.yml | grep -n "uses:" → actions/checkout@v4 · dtolnay/rust-toolchain@stable ·
                                                                   actions/cache@v4 (and doctrines.yml: actions/checkout@v4)
    $ rustup run stable rustc --version  → rustc 1.95.0 (59807616e 2026-04-14)
    $ rustup run 1.98.0 rustc --version  → rustc 1.98.0 (88d9e12ae 2026-08-18)   — installed beside it; CI's @stable is the newest
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — a channel and a tag name *a pointer*, not a release: `stable` and `v4` resolve to
    whatever they point at on the day of the run, so the build environment was a function of the date. **WHERE:** the
    four refs above, and the `book` step, which ran `mdbook` with no version at all (`git show HEAD:xtask/src/main.rs |
    grep -n 'program: "mdbook"'` → `program: "mdbook",`).
  - [x] **FIX** — `rust-toolchain.toml`: `channel = "1.95.0"`, with its components and the riscv target; every job
    runs `rustup toolchain install --no-self-update`, so the version is written once and `dtolnay/rust-toolchain` is no
    longer used; `actions/checkout` and `actions/cache` referenced by commit (`v4.4.0`, `v4.3.0`, what `v4` resolved
    to by `git ls-remote`), the tag in a comment; `scripts/build_book.sh` builds only with `MDBOOK_VERSION_PINNED` and
    is the `book` step and `make book`; the three ledger entries moved with their pins.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ rustup toolchain install --no-self-update
      info: the active toolchain `1.95.0-aarch64-apple-darwin` has been installed   (the riscv target added by the file)
    $ bash scripts/check_source_ledger.sh      → source-ledger: OK (12 entries; 9 pin(s), each carried by its entry; …)
    $ bash scripts/build_book.sh --self-test   → build-book self-test: 4 pass / 0 fail (4 arms)
      M1 the version matched as a prefix → 1 refused · M2 the version not checked → 2 refused   (both restored)
    ```
  - [x] **NO REGRESSION** — the tier and the suite on the pinned toolchain, and on the newer one for the next bump:
    ```text
    $ cargo xtask verify --tier integration   → incomplete — 7 passed, 0 failed, 0 unavailable, 0 not built, 1 quarantined
    $ cargo +1.95.0 test --all -q  → 625 passed, 0 failed, 48 suites
    $ cargo +1.98.0 test --all -q  → 625 passed, 0 failed, 48 suites   (fmt exit=0, clippy -D warnings exit=0)
    ```
  - [x] **LOCKSTEP** — `ledger.md` (three entries), `verification.md`, `DOCTRINE_ENFORCEMENT.md`, the ledger gate's
    honest-limit comment, `Makefile`. ⚠️ Two things only a real run shows, added to `.10.5`: whether the runner's rustup
    installs from the file (it needs rustup ≥ 1.28), and the pinned actions resolving. ⚠️ Side effect, reported: the
    first `rustup toolchain install` here ran without `--no-self-update` and updated this machine's rustup, `1.29.0` →
    `1.29.1` — outside the repository, and not asked for.
