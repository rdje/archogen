# Catalog records: hashes, a normative byte grammar

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **External sources:** [the Rust toolchain](../../book/src/ledger.md#rust-toolchain) — `rustc`, `cargo` and
  `rustup`, whose version, scope and limits are in the ledger
- **Owner / source:** leaf `M2.7.1` (`docs/tasks/M2.md`). This is §3 of [[decision_catalog-records]], moved out of it
  on `2026-09-30`, in the change that answered its fourteenth review, with that review's T1 and T15 edits and
  nothing else, when that record held 95 522 of the
  98 304 bytes `README-ROUTES` allows a file. It is part of that record: normative, numbered as its §3, and reviewed
  with it. That change's edits to §3 are in the review history. Its worked example is
  [`decision_catalog-records-example.md`](decision_catalog-records-example.md).

## The fact / decision

### 3. Hashes: a normative byte grammar

Every hash is SHA-256 (FIPS 180-4), written `sha256:` followed by 64 lowercase hexadecimal digits. Every hash
input is ASCII text, one item per line. Every line ends in one line feed (`0x0A`), the last one included, and
there is nothing else.

**The encoding of a form, `E`**, is independent of any printer, so no change elsewhere moves a hash:

- a symbol is its text;
- an integer is its decimal value, with no leading zeros. A hexadecimal literal is encoded in decimal, so `0x28`
  is `40`;
- a string is `"`, then its decoded value with `\` written `\\` and `"` written `\"`, then `"`. §1 has made the
  decoded value printable ASCII;
- a list is `(`, then its items' encodings joined by one space, then `)`.

**The forms of each facet:**

- the contract's are the `version`, `catalog`, `source`, `maintainer`, `depends`, `supersedes`, `profiles`,
  `targets`, `preconditions` and `guarantees` forms, each whole, in record order;
- each other facet's is its one form, whole.

**Source sets.** Each entry in `sources` is a tracked file, a directory, or a package: a directory whose
`Cargo.toml` has a `[package]` table. A manifest with only a `[workspace]` table does not make a package.

- **An entry inside a package stands for the whole package**, so a named file or subdirectory cannot narrow what
  is covered.
- **A directory entry with a package below it is refused.** Name the package instead.
- **A package with another package below it is refused**, so an entry stands for exactly one package.
- **The facet's own set:**
  - a file entry is that file;
  - a directory entry is every tracked file under it;
  - a package entry is every tracked file under the package's directory.
- **The facet's reached set** is what a package brings in beyond its directory, and it enters the bound hash
  only, as `file` lines:
  - the packages its manifest names as dependencies or build dependencies, transitively, each with every tracked
    file under its directory. **A dependency table** is read by TOML's meaning, not its spelling: a table named
    `dependencies`, `build-dependencies` or `dev-dependencies`, or by Cargo's aliases `build_dependencies` and
    `dev_dependencies`, at the root or under `target.<triple>` whose triple is a bare key, however the dialect's
    headers, dotted keys and inline tables write it. Each of its keys is a dependency. A quoted `cfg(…)` key is
    outside the dialect below. Each reached package is held to every rule of this section, and its own workspace
    manifest is in the reached set too. Dev dependencies are not followed, because they do not enter the built
    code;
  - its workspace's manifest: the nearest `Cargo.toml` with a `[workspace]` table, in the package's directory
    or an ancestor. The package must be that manifest's own package, or match an entry of its `members` and no
    entry of its `exclude`. An entry is a relative path whose segments are literal or exactly `*`, which matches
    one segment. Anything else is refused. So is a package that workspace does not include, which Cargo would walk
    past to an outer workspace (the ledger's `rust-toolchain` scope): `/1` does not follow it;
  - from the package's directory and every ancestor up to the repository root, each of `.cargo/config.toml`,
    `.cargo/config` and `rust-toolchain.toml` that exists. A `rust-toolchain` file without the extension there is
    refused, and so is a `rust-toolchain.toml` anywhere but the repository root: the pin is one file, and rustup
    reads the legacy one too.

  A change to the workspace manifest or the toolchain therefore makes reviews stale, but moves no version.

Both sets are sorted bytewise by path, without duplicates, and a file in the own set is not repeated in the
reached set. A path under `catalog/` is refused, in these sets and among a target's files alike, since a record
could otherwise hash its own reviews, and a target file there would move with every bless.

**A package is refused (`catalog-source`), fail-closed,** when:

- a line of its manifest, or of its workspace manifest, is outside the dialect below;
- it has a build script: a `build.rs` at its root, or a `build` key other than `false`;
- any table of its manifest holds a `proc-macro`, `proc_macro`, `crate-type` or `crate_type` key, whatever its
  value, since each can make it a procedural macro or another kind of crate; or it has a `package.workspace` key;
- it declares a `[features]` table or an optional dependency. `/1` builds one configuration per profile and
  target, so what a review saw is that configuration;
- a dependency in any dependency table, `[dev-dependencies]` included, is not a path dependency, or is
  `workspace = true`;
- a `path` key of `[lib]`, `[[bin]]`, `[[test]]`, `[[example]]` or `[[bench]]` leaves its directory;
- its workspace manifest has a `[patch]` or `[replace]` table;
- a config or toolchain file on its ancestor path is present but untracked, or a config file there is outside the
  manifest dialect below or holds a key whose path does not begin with `alias`, whether written under a header,
  dotted or as an inline table. Build flags, linkers and linker scripts are the build's, and `/1` admits none;
- its manifest or its workspace manifest has a `cargo-features` key, or a `rustflags` key in any table. Either
  passes the compiler flags that `/1` admits none of;
- a Rust source file of the package holds, **as tokens** (comments and the contents of literals are not tokens):
  - anywhere: the identifier `global_asm`, `include`, `include_str`, `include_bytes`,
    `debugger_visualizer`, `no_mangle`, `export_name`, `link_section`, `panic_handler`, `global_allocator`,
    `alloc_error_handler` or `macro_rules`, or the keyword `macro`; and `asm` and `naked_asm` anywhere but as §14.2
    admits them, in a package an `assembly` declaration names (`M2.12`). So a renamed import of an assembly or include
    macro, or of an attribute macro such as `global_allocator`, is refused too, and the package defines no macro of
    its own;
  - inside an attribute, `#[…]` or `#![…]`, at any depth, so within `cfg_attr(…)` and `unsafe(…)` too, and inside
    the arguments of any macro invocation: the identifier `path`, `link` or `used`. These three are common words,
    refused only where they could become an attribute;
  - the identifier `extern`, except in `extern crate`, and when followed by an optional string literal and `fn`
    outside the arguments of a macro invocation. So a foreign block is refused, and so is one a macro could
    assemble from its arguments, while an `extern "C" fn` definition and a function-pointer type are not;
  - an identifier that is not ASCII.

  An identifier is compared by its name, so `r#asm` is `asm`. Rust normalizes identifiers to NFC, and a non-ASCII
  identifier could normalize to a refused name, so none is admitted.

  **The Rust source files** of a package are every tracked `*.rs` file under its directory, and every source path
  the compiler's dependency information lists for its builds (below). Each listed path must end in `.rs`. A listed
  path that does not is a file an attribute or macro made the compiler read, and it is refused, whatever read it.
  So the scan covers every file the compiler read, not the files a name suggests. No package in the build defines a
  macro: `macro_rules` and `macro` are refused as words, and a procedural macro is refused by its manifest and by
  what cargo resolves (below). A macro invocation's arguments hold none of the refused words. So nothing compiled
  holds a refused form that its source does not show, beyond what the pinned toolchain's own macros expand to
  (premise 1). What the refused forms
  reach — code a symbol reaches only at link time, a native library, a file an assembler directive reads, and a
  global hook or symbol another package could supply in its place — is in no source set and, for some, in no
  dependency information, so `/1` admits none of them. `#![feature]` needs a nightly compiler, which the pin's
  form and the environment below rule out. Architecture code that needs assembly is admitted only as §14.2 states
  ([`decision_catalog-records-port.md`](decision_catalog-records-port.md)): code reached only by `sym` operands or
  addresses the code computes, a closed dialect, no directive.

  The rules bind the packages a record's sets reach, and no other. Measured on `2026-09-30`: `crates/rt-core`, the
  package the slice `M2.7.4` records names, reaches no other package and passes them. Each refused word appears in
  its tracked files only in comments, in literals or in its manifest; its attributes are `test`, `must_use`,
  `derive` of standard traits, `cfg(test)` and `#![cfg_attr(not(test), no_std)]`; its only non-ASCII bytes are in
  comments; and the dependency information a plain `cargo build -p rt-core` writes names only its three `.rs`
  sources. A clippy run adds `Cargo.toml` to it, so the gate reads a plain build's. Packages no record reaches are
  not held to these rules: `archogen-wasm`'s exports need `no_mangle`, and the spike under `targets/` declares
  `[features]`.

**The manifest dialect** is the part of TOML these manifests use. A line is blank; a comment; a table header,
which is `[` or `[[`, then one or more bare keys joined by `.`, then `]` or `]]`, where a bare key is ASCII
letters, digits, `-` and `_`; or `key = value`,
where the key is a bare key or bare keys joined by `.`. A value is, on the same line, a basic string with no
escape, an integer, `true` or `false`, an array of those, or an inline table `{ key = value, … }` of those. A
quoted key, a literal or multi-line string, a value continued on the next line, a float or a date is outside it.
Every tracked manifest is inside it as of `2026-09-30`.

These rules are necessary, not sufficient: what the compiler reads is decided by `cfg_attr`, `env!` and the
module rules, and no lexical rule sees all of it. So **the gate (`M2.7.4`) builds every package in every source
set** and reads the compiler's dependency information:

- **What it builds from.** The git index, each entry of mode `100644` or `100755` written file by file from its
  blob's bytes into a directory of its own under the repository's `target/`, with `GIT_NO_REPLACE_OBJECTS` set. A
  gitlink is not written, and no set may reach it. A tree with any tracked path outside §4's grammar, reached or
  not, is refused, and that grammar is ASCII, so no file system's Unicode folding can make one name another. It is
  not a checkout, so no filter, attribute or line-ending setting of git's can change a byte. Two tracked paths that
  differ only in ASCII case are refused, since a case-insensitive file system cannot hold both. So is a tracked path
  whose last segments equal `.cargo/config`, `.cargo/config.toml`, `rust-toolchain`, `rust-toolchain.toml`,
  `Cargo.toml`, `Cargo.lock` or `build.rs` in ASCII case but are spelled otherwise, since such a file system would
  let cargo or rustup read it under the name it expects. Each build runs in its package's own directory.
- **What configuration it lets cargo read.** Cargo reads every `.cargo/config` and `.cargo/config.toml` from the
  build's directory up to the file system's root, and those under `CARGO_HOME`. So before any cargo command runs
  on a tree, `cargo metadata` included, the gate lists every such file on that command's directory path, and holds
  each to the content rules above, and refuses (`catalog-source`), running nothing:
  - one inside the written index that is not a tracked file held to the content rules above;
  - one outside the written index, except the tracked copies of the tree being judged, whose bytes on disk must
    equal that tree's. `target/.cargo` is outside the written index and is refused, and so is anything above the
    repository.

  `cargo metadata` itself runs `rustc` through any `build.rustc` or wrapper a configuration names, and the content
  rules admit only `alias` keys, so no program a configuration names ever runs.
- **Its environment is an allowlist:** `PATH`, `HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN` set to the pin,
  `CARGO_HOME` set to an empty directory of the gate's own, since a workspace of path dependencies needs no
  registry, and the gate's own `CARGO_TARGET_DIR`, and nothing else. A list of variables to clear would miss the
  next one that changes a build. `RUSTUP_TOOLCHAIN` outranks every rustup override, and `rustc -vV` must then name
  the pinned release, or the gate refuses (premise 1).
- **The pin** is the `channel` of the `rust-toolchain.toml` at the written index's root, which holds only a
  `[toolchain]` table whose keys are `channel`, which is required, and any of `components`, `targets` and `profile`,
  so no `path` toolchain is named. The channel must be a release number: three decimal numbers, `MAJOR.MINOR.PATCH`,
  each `0` or without a leading zero. A named channel such as `stable`, `beta` or `nightly`, a dated or suffixed
  one, or an index with no `rust-toolchain.toml`, is refused. A moving channel would change the compiler under an
  unchanged hash, and a nightly one admits `#![feature]`.
- **What cargo resolves, before anything is built.** `cargo metadata --offline --locked --format-version 1`, in
  the environment above, must report the root of the written index as its `workspace_root`. It resolves the whole
  workspace, so every member's dependencies must be path ones, a package no record reaches included, dev
  dependencies too. That tightens `decision_zero-dependency-engine-core.md`, which admits an external crate
  through a reviewed decision; the tightening is recorded there. A member with any other dependency is refused on its
  own (`catalog-source`), naming that member, rather than failing the offline resolution with a message about the
  registry. In the graph of normal and build dependencies of every package in a source set, the gate also refuses:
  - a package with a target of kind `proc-macro` or `custom-build`;
  - a package whose `source` is not null, which is any dependency that is not a path one;
  - a package the graph holds that §3's reading of the manifests did not reach, or one it reached that the graph
    does not hold.

  So cargo's own reading of the manifests checks the dialect's reading, both ways, and no manifest spelling adds a
  package, a build script or a procedural macro unseen. `cargo metadata` runs no package's code. Every build runs
  with `--offline --locked` as well, so nothing is fetched and `Cargo.lock` is not rewritten.
- **Which workspace cargo sees.** The workspace root that `cargo metadata` reports is the root of the written
  index. An `exclude` entry excludes every package whose directory is the entry or lies under
  it, as cargo's does, and §3's membership rule reads it so.
- **The build matrix.** The dev and release profiles. An implementation package is built for the host and for
  each target's `RUST_TARGET`, for every target the record names in its contract or costs, where `any` means
  every target under `targets/`. A package
  that is only in a model's sets is host code, and is built for the host alone. There is no feature axis,
  because `/1` refuses features.
- **Who runs these checks.** The gate, on the commit being made. CI, on every commit it replays that changes
  anything under `catalog/`, a tracked file under the directory of any package in a source set or a reached set,
  a manifest,
  `Cargo.lock`, a cargo configuration file, a toolchain file or a target file. Records under `catalog/` decide the
  source sets, targets and costs, so this covers every change to what is built, and a commit made with the gate
  bypassed is built too. A production claim runs them on the
  commit it reads. Each reads the dependency information of every unit the build compiles, not only the packages a
  record names.
- **What it refuses:**
  - a package that does not build under the matrix;
  - every source path in the compiler's dependency information that is in neither set. A path is resolved
    lexically against the written index's root, `.` and `..` included, and must then satisfy §4's grammar. One
    outside the written index is refused, the toolchain's sysroot included: no package's own sources live there,
    and measured on `2026-09-30` no workspace crate's dependency information names one. `M2.7.4` measures that
    again before relying on it;
  - a source path whose bytes after the build differ from its blob's;
  - every environment dependency except `CARGO_PKG_*`, `CARGO_CRATE_NAME`, `CARGO_MANIFEST_DIR` and
    `CARGO_MANIFEST_PATH`.

The compiler, not a scan, decides completeness.

**Own hash** of facet `F` of record `R`:

```text
archogen-catalog/1
own F R
E(form)                                          one line per form of the facet, in record order
file <path> sha256:<hash of the file's bytes>    one line per file of the own set, in its order
```

**Bound hash** of `F`:

```text
archogen-catalog/1
bound F R
own sha256:<own hash of F>
<derived lines: a set, without duplicates, sorted bytewise>
```

Each derived line takes one of four forms:

- `<facet> <id> sha256:<that facet's bound hash>`;
- `file <path> sha256:<file hash>`, for the reached set;
- `target <path> sha256:<file hash>`;
- `ledger <anchor> sha256:<hash of the section's bytes>`.

The **target files** of a target `t` are `targets/<t>.env`, `targets/<t>.eadl`, and every path the `.env` gives
as a value. A value that holds `/` is a path from the repository's root: it must be tracked and in §4's normal
form, or the target is refused. A value without `/`, such as a program's name or a triple, is not a path.

**A `.env` is lines** of three kinds: blank; a comment, whose first character is `#`; or `KEY=value`. A key is one
of the prefixes `TARGET_`, `QEMU_`, `RUST_`, `REQUIRES_`, `DEVICE_` and `PLATFORM_`, then uppercase ASCII letters,
digits and `_`. No variable that bash or a POSIX shell sets, reads or holds read-only begins with one of them, so no
key is one the shell gives a meaning to. The value is the rest of the line: ASCII letters,
digits, `.`, `_`, `-`, `/`, `+`, `:` and `,`, and nothing else, so it holds no character a shell treats specially.
There is no `export`. Each key appears at most once. Any other line refuses the target (`catalog-field`), so no
reader can take a different value from the one the shell that sources the file takes. A value without `/` that
names a tracked file (a file, not a directory with tracked files under it), from the repository's root or from
`targets/`, is refused: a file a target reads is named
with its path, so a word such as `yes` never turns a file added later into a target file. A `.env` or `.eadl`
directly under `targets/` without its pair is refused too, so every target under `targets/` is a named target (§2).

The **dependency closure** of a record is its dependencies, transitively.

| Facet | Derived lines |
| --- | --- |
| contract | `contract <D>` for each direct dependency `D`, so a dependency's changed guarantee makes its dependents' contract reviews stale, whatever its version says |
| implementation | `file` lines of its reached set |
| behavior-model | `contract <R>`, because the facts are stated under its profiles, targets, preconditions and dependencies; `implementation <R>` and `implementation <X>` for each `describes` record `X`, because a fact about code is voided when the code changes; `behavior-model <D>` for each direct dependency; `target` lines for the target files of each target the contract names, or of every target under `targets/` when it names `any`; `ledger` lines for its ledger locators; `file` lines of its reached set |
| timing-model | `contract <R>`, for the same reason; `implementation <X>` for `R`, every record in `R`'s dependency closure, and every `measured-with` record with its own dependency closure; `timing-model <D>` for each direct dependency and each `measured-with` record; `target` lines for the target files of each target its costs name; `ledger` lines; `file` lines of its reached set |

- **Independence.** No behavioral line names a timing model, and no timing line names a behavioral model. That is
  what keeps them independent (§8).
- **The record's hash**, `ROADMAP.md` §9's "content hash", is over:

  ```text
  archogen-catalog/1
  record R
  contract sha256:<bound>
  implementation sha256:<bound>
  behavior-model sha256:<bound>
  timing-model sha256:<bound>
  ```

- **A review's ledger hash** is over its three lines, `archogen-catalog/1`, `review R` and `E(review form)`.
- **Excluded from every facet hash** are the reviews, since a review cannot cover itself, and the namespace.
  Promotion changes no hash, and it needs production verdicts (§6).
- **A `none` facet is hashed like any other.** A `none` timing model still gets its `contract` and
  `implementation` lines, so going from `none` to present, or back, is a change.
- **Line 2 names the facet or review and the id**, so a review cannot be copied between records or facets whose
  text is the same.
- **`archogen-catalog/1` versions this grammar.** From the first lock on, changing any part of this section, or
  §14.2 and §14.3, which hold the assembly its token rules admit, or §14.4's fact and cost names and its id prefix `convention.check-passing.`, changes it. Before it, `/1` was amended once, by
  §14.2, for the reason given there.

**Worked example.** [`decision_catalog-records-example.md`](decision_catalog-records-example.md) fixes two
records and three files, byte for byte, and every hash this section gives them: each facet's own and bound hash,
each record hash, a review's ledger hash and lock lines, and a forms digest. `M2.7.3` must reproduce every one.

## Why

§3 is the record's longest section, and the worked example pins it byte for byte, so it changes least often. Kept
apart, like §12 and §13, it lets the record grow with its reviews without passing its ceiling, and a reviewer of
the record is given it too.

## How to apply

- A bare section number here is the catalog record's. One that is `ROADMAP.md`'s says so.
- A change to §3 that moves a hash input recomputes the worked example.
