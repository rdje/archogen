- ID: `PROGRAM.2`
  Status: `done`
  Goal: establish the workspace skeleton and crate boundaries actually needed by S0/M1,
  replacing the bedrock starter crate with the `archogen` CLI shell.
  Acceptance: `cargo test --all` green; `archogen --help` lists the §10.2 command surface as
  implemented-or-unimplemented, with unimplemented commands exiting with a clear diagnostic.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0003`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — the workspace still held only the template's starter
    crate, so the project had no entry point and the §10.2 interface target existed nowhere in
    code. Measured at the parent commit: `git ls-tree --name-only HEAD crates/` → `crates/app`
    (one crate); `git grep -c 'bin name = "archogen"' HEAD -- crates/` → no match, `grep rc=1`;
    `git show HEAD:crates/app/src/main.rs | grep -n 'println!'` →
    `7:    println!("bedrock: replace this crate with your project — start from ROADMAP.md.");`.
    WHERE: `crates/app/src/main.rs:7` — the only executable behavior in the repository was a
    template placeholder.
  - [x] **ADDRESSED (verified)** — after: `cargo run --quiet --bin archogen -- --help` prints all
    seven §10.2 commands with their owning leaves and the eleven-row exit-code table,
    `exit=0`; `cargo run --quiet --bin archogen -- check examples/periodic-three/system.eadl
    --profile rt-static-up-v1` prints
    `archogen: unimplemented: \`archogen check\` is not implemented yet` with
    `hint: … tracked by task-tree leaf M1.8`, `exit=20`. Before, the same binary name did not
    resolve at all (evidence B above, `grep rc=1`).
  - [x] **NO REGRESSION** — `cargo fmt --all -- --check` → `fmt rc=0`; `cargo clippy
    --all-targets --all-features -- -D warnings` → `Finished \`dev\` profile`, no warnings;
    `cargo test --all` → `test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0
    filtered out`; `scripts/check_doctrines.sh` → `=== all doctrines green ===`, `rc=0`;
    `mdbook build docs/book` → `INFO HTML book written to`.
  - [x] **FIX** — removed `crates/app`; added `crates/archogen-cli` (lib `archogen_cli` + bin
    `archogen`) with the §10.2 surface declared once as data in `src/spec.rs`, a hand-written
    parser in `src/cli.rs`, and the §5.5 outcome vocabulary with stable exit codes in
    `src/status.rs`. Zero external dependencies.
  - [x] **LOCKSTEP** — `docs/book/src/cli.md` (new chapter) and `SUMMARY.md`;
    `docs/decisions/decision_zero-dependency-engine-core.md` + its index row;
    `knowledge-map/subsystems.md`; `MEMORY.md`; `LIVE_STATUS.md`; `CHANGELOG.md`.

- ID: `PROGRAM.2.1`
  Status: `done`
  Goal: rename the toolchain's command and crate family from `archogen` to `archogen`, across the
  roadmap, the mdBook, the task-trees, the examples and the code.
  Acceptance: the old name survives **only** in the documents that record the rename — this
  leaf, `ROADMAP.md`'s migration note, the `CHANGELOG.md` entry and `MEMORY.md`'s latest-commit
  line — and nowhere else in tracked files; `archogen check`
  behaves exactly as the old command did; every test, gate and book build stays green.

  ⛔ The first draft of this criterion said "`git grep -ci osgen` returns nothing on tracked
  files", which is **self-defeating**: the record of a rename necessarily names the old thing,
  so writing the evidence down would have broken the criterion it was evidence for. The
  criterion is scoped to exclude those records instead, and the exclusion is verified rather
  than assumed.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0021`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — the project is `archogen` and its command announced
    itself as `osgen`, so the toolchain carried two names. Measured before the change:
    `git grep -o -i 'osgen' | wc -l` → `169` across 33 tracked files, and
    `git grep -oh -E '[A-Za-z_-]*osgen[A-Za-z_-]*' | sort | uniq -c` → six distinct forms
    (`osgen` 128, `osgen-cli` 14, `osgen-evidence` 12, `osgen_cli` 6, and
    `osgen-plan`/`osgen-emit`/`osgen-check` 3 each). WHERE: `crates/osgen-cli/Cargo.toml`'s
    `[[bin]] name = "osgen"` was the source, and `ROADMAP.md` §4.2 and §10.2 propagated it.
  - [x] **ADDRESSED (verified)** — after the change, the scoped census

    ```
    git grep -c -i 'osgen' -- . ':(exclude)Cargo.lock' ':(exclude)docs/tasks/PROGRAM.md' \
      ':(exclude)ROADMAP.md' ':(exclude)CHANGELOG.md' ':(exclude)MEMORY.md'
    ```

    returns **no output**, `rc=1`. The four excluded documents are exactly the four that *record*
    the rename: this leaf, the §15 migration note, the changelog entry, and the resume pointer's
    "latest commit" line. `Cargo.lock` is excluded because it was regenerated rather than edited:
    `grep -c 'osgen' Cargo.lock` → `0`, `rc=1`.
    ⛔ An earlier version of this box pasted the same command with only **three** exclusions,
    which does not reproduce — it returns `MEMORY.md:1`. A pasted command that does not reproduce
    is the failure `TOOLBOX.md` exists to prevent, so it was corrected rather than left as a
    near-miss.
    `crates/archogen-cli/Cargo.toml` now carries `[[bin]] name = "archogen"`, and the command
    behaves identically: `archogen check examples/periodic-three/system.eadl` →
    `accepted against profile \`rt-static-up-v1\` (8 declaration(s))`, `exit=0`;
    `archogen check examples/bounded-queue/system.eadl` →
    `archogen: unsupported-profile: 1 diagnostic(s)`, `exit=12` — the same verdicts and the same
    exit codes as before, with only the program name changed.
  - [x] **NO REGRESSION** — `cargo fmt --all -- --check` → `fmt clean`; `cargo clippy
    --all-targets --all-features -- -D warnings` → 0 errors; `cargo test --all` → **246** tests
    passing, `0 failed` — the same count as before the rename, so no test was lost to a renamed
    path; `scripts/check_doctrines.sh` → `=== all doctrines green ===`; `mdbook build docs/book`
    → `INFO HTML book written to`.
  - [x] **FIX** — `git mv` on both crate directories so history follows the files, then one
    substitution across every tracked text file except `Cargo.lock`. All six spellings share the
    `osgen` prefix, so a single replacement covers the hyphenated, underscored and bare forms
    without a per-form rule.
    ⛔ `cargo fmt` was **not** optional here: the four extra characters pushed several lines past
    the width limit, and `--check` caught it before the commit rather than CI catching it after.
  - [x] **LOCKSTEP** — `ROADMAP.md` gains a dated **migration note**, which §15 requires of a
    rename ("maintain migrations for renamed fields and kinds"), stating that nothing but the
    name changed; `README.md`, all 15 book chapters, every task-tree, `examples/README.md`,
    `TOOLBOX.md`, `knowledge-map/subsystems.md`, `MEMORY.md`, `LIVE_STATUS.md` and `CHANGELOG.md`
    carry the new name.

  ### Scope decision, recorded because it was a judgement call

  The director's words were "rename the CLI from archogen to archogen **in the entire project**".
  Read narrowly that is the binary alone; read broadly it is every `archogen`-prefixed name. The
  broad reading was taken, because the narrow one **preserves the inconsistency that prompted
  the request**: a project called `archogen` whose evidence crate is `archogen-evidence` and whose
  §4.2 component names are `archogen-plan`, `archogen-emit`, `archogen-check` is still a project with two
  names in it.

  So the rename covers, in one substitution:

  | Form | Count | Becomes |
  | --- | --- | --- |
  | `archogen` (the command, and prose) | 128 | `archogen` |
  | `archogen-cli` (crate) | 14 | `archogen-cli` |
  | `archogen-evidence` (crate) | 12 | `archogen-evidence` |
  | `archogen_cli` (Rust identifier) | 6 | `archogen_cli` |
  | `archogen-plan` / `archogen-emit` / `archogen-check` (§4.2 names) | 9 | `archogen-*` |

  Narrowing it back to the binary alone is a small, mechanical revert of the crate directories
  and the §4.2 table; nothing depends on the broad reading being right.
