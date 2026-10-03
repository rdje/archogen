# The trust-dependency inventory and its gate: what each root is built from, and what two roots share

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active` — written; under independent review (leaf `M3.6.1`'s closure rule: the first round that finds
  no defect closes it); rounds 1 to 3 answered `2026-10-03`; narrowed after round 3 to what only it decides
- **External sources:** [the pinned Rust toolchain](../book/src/ledger.md#rust-toolchain) — rustc's dependency
  information and cargo's metadata, their version and limits in the ledger
- **Owner / source:** leaf `M3.6.1` (`docs/tasks/M3.md`). `ROADMAP.md` §4.4 asks for a machine-readable
  dependency and provenance inventory for each generator, configuration checker, scheduling checker and
  reference-model build, rooted in the actual build, with every item two of them share classified and compared
  against a reviewed baseline; §14.4 makes it fixture F30, with five cases the gate must exercise. The facts below
  were measured `2026-10-03` with `cargo metadata --format-version 1 --offline` and the build's dependency
  information.

## The fact / decision

The integration pipeline builds each **root** — each program whose independence a claim relies on — from the
commit's own files, under the pinned toolchain and a cleared environment, reads what the compiler read for it, and
writes an **inventory**: the build's identity, and for each root the packages it reaches, every file its compilation
read with its content hash, and each compilation unit's configuration. From the inventories it derives every item
two roots **share**, copies included. A **baseline**, committed and reviewed, lists each shared item the project
accepts, with its classification, the property it can affect, its residual common-error risk and the independent
controls that remain. The **gate** refuses an input the inventory cannot account for and a program running two
roles, which an author repairs alone, and **reports** every shared item that is new, or changed against the baseline,
and every program not yet classified, as unreviewed. It reports nothing for a change outside every root.

**What this record does not decide** (narrowed after its third review, R3): who reviews a shared item, how a review
is recorded and protected, and what an unreviewed item does to a claim, a package or a release. Those are the
protection the catalog's premise 3 rests on (`M2.7.6`, findings §11) and the claims and the package that consume the
report (`M4.7`, `M4.8`), each of which carries its part in its acceptance (§5). Until they land nothing is accepted:
every shared item is unreviewed, no claim rests on a role's independence, and the assurance tier's trust step
reports **Unavailable**, never Passed. The tool measures and compares; it never classifies, and it never accepts.

### 1. Measured `2026-10-03`

- 12 packages, all workspace members; **no third-party package** (`decision_zero-dependency-engine-core.md`); no
  build script, no procedural macro, no Cargo feature (`cargo metadata --format-version 1 --offline`).
- The roots of §2, as they exist today, and what each reaches (normal and build edges, the root's own package
  included):

| Role | Root | Reaches |
| --- | --- | --- |
| generator | the `archogen` executable (`crates/archogen-cli`), which `archogen build` runs: it checks a description through `archogen-api` and hands it to `archogen-s0` (`crates/archogen-cli/src/build_cmd.rs`) | `archogen-cli`, `archogen-api`, `archogen-s0`, `archogen-wasm`, `eadl-front`, `eadl-model`, and through `archogen-api` the kind modules `docs/semantics/kinds/core.eadl` and `os-rt.eadl` |
| configuration checker | none yet — `archogen-check` is `M3.5`'s | — |
| scheduling checker | `rt-analysis`, a library no executable runs yet | `rt-analysis`, `archogen-evidence` |
| reference model | `rt-reference` | `rt-reference` |
| implementation a reference model validates | `rt-core`, which `rt-reference` validates (`ROADMAP.md` §6.3) | `rt-core` |

- **No two roots share a package, or a file compiled for them, today.** The one shared item is the comparison
  harness through which the reference model validates its implementation — `crates/rt-core/tests/differential.rs`
  and the adapter it holds, written by `rt-core`'s author — shared by design and disclosed by its header (§4; R3
  C10).
- **Dependency information names a file as rustc opened it, unnormalised:** `archogen-api`'s names its kind module
  `crates/archogen-api/src/../../../docs/semantics/kinds/core.eadl`. The inventory resolves every name lexically
  before using it (§3).
- **Foreseen:** `M2.7.5` makes `rt-analysis` read the catalog through `archogen-catalog`, which reaches `eadl-front`
  and `eadl-model`, the generator's own reader and model — §14.4's case 1, reported unreviewed (§5).

### 2. Roots

A root is a program that runs a role. The roles are §4.4's four — generator, configuration checker, scheduling
checker, reference model — and, for each reference model, the implementation it validates, since §6.3 makes their
independence the point of having one (R1 A2). A role's root is the executable that runs it (R1 A1), with two
exceptions stated rather than left: a role no executable runs yet is rooted at its library — the scheduling checker
today — and replaced by the first executable that runs it; and a reference model and the implementation it
validates are rooted at their libraries, since what runs them is a test harness, which §4 makes an item the pair
shares (R2 B7). **An executable that would run two roles is refused** (`trust-shared-program`): the independence
§4.4 asks about is between programs, and one program running both roles has none to disclose; every package it
holds would be shared, and a real case 1 would be lost among them (R2 B5). `ROADMAP.md` §10.2's `archogen analyze`
would make the generator's executable run the scheduling checker; `M2.21` decides how it runs before it is built, and `M3.4` carries the same for `archogen resolve` (R3 C20).

`trust/roots.eadl`, read by the eADL reader as every repository record is, holds one form per root — its role, its
package and target, its artifact and the runtime data the pipeline hands it (§3) — and one form per role no root
fills yet, naming the leaf that will (the configuration checker, `M3.5`). It also classifies every other
**program target** of the workspace — a `[[bin]]`, a `src/main.rs`, a `src/bin/*.rs` cargo discovers, an example, and a
`cdylib`, `staticlib` or `dylib` crate type, such as the wasm module `ROADMAP.md` §10.4 makes a consumer of the
engine — as not a root, with a reason and the role libraries its closure reaches; when that set grows, the
classification is unreviewed again (R2 B6; R3 C14). Programs are targets, not packages: an executable added to a
classified package is still a new program. The gate computes sharing from the roots the commit proposes, never only
from those last reviewed (R3 C19). A program target the file does not classify is reported unreviewed (`trust-unclassified-program`,
§6), and a classification, like every change to the file, is accepted only by the review §5's leaves define, so no author removes a
root or classifies their own checker away to silence a sharing (R1 A3). Libraries are not classified: a library
matters only through the roots that reach it. Every root is built in the release profile with
`--no-default-features` and no `--features` (R1 A19), for **one host triple**, the CI runner's
(`x86_64-unknown-linux-gnu`), which the baseline names: cargo's unit hashes and a host's `cfg` differ by host, so on
another host the gate builds and reports but compares nothing (R2 B8).

### 3. The inventory

`cargo xtask trust-inventory` writes `target/trust/trust-dependencies.json` and `target/trust/report.txt`.

**Where it builds.** Each root is built clean from the commit's own blobs, written into `target/trust/<root>/` as
the catalog's build checker writes a judged tree (`xtask/src/catalog_build.rs`, `write_tree`, which refuses a path
that differs from another only in ASCII case, a special name spelt in another case, and any tracked path outside the
catalog's path grammar — none today, and the catalog's own check refuses such a tree already (R3 C17)); never from the working tree, so an uncommitted or untracked file
cannot stand in for a committed one (R1 A7). It builds with `--locked --offline` under the catalog's cleared,
allowlisted environment (`environment`: `PATH`, `HOME`, the rustup home and the pinned toolchain, a `CARGO_HOME` of
its own, its own target directory, nothing else — no `RUSTFLAGS`, no `RUSTC_WRAPPER`, no `CARGO_PROFILE_*`), and
refuses a cargo configuration found above the repository or under `target/`, or one that is a symbolic link
(`configurations_on_path`), and admits an in-tree configuration only byte for byte as committed and holding
nothing but `[alias]` (`package::check_config`), which closes `build.rustc`, `build.rustc-wrapper`, a linker,
`[env]` and `rustflags` (R1 A6; R3 C18). `CARGO_HOME` is removed and made anew on every run, since `environment` only
creates it and a configuration left in it would reach every later build (R2 B12). Before any build, the packages'
targets are read with `cargo metadata`, which runs no package's code, and a build script or procedural macro is
refused there, as the catalog's `metadata` refuses them, so none runs (R2 B12).

**What it records, for the build as a whole:** the commit and its tree hash; the toolchain (`rustc -vV`,
`cargo -V`); the host triple; and the sha256 of `Cargo.lock`, the root `Cargo.toml`, `rust-toolchain.toml` and
`.cargo/config.toml` — the build configuration. The toolchain's own `std` and compiler are code every root shares
by construction, accepted as infrastructure by the toolchain's identity, whose bump is reviewed where the toolchain
is ledgered; the dependency information names no path under the sysroot, so the toolchain is recorded by identity
and never per file (R1 A14).

**What it records, for each root:**

- the packages its build compiled, from that build's units and not from workspace-wide metadata, each with its
  manifest's sha256 and the kind of every edge that reached it (normal, build), development edges not followed,
  since a test is not the root (R1 A10, A17);
- each compilation unit's **configuration**: its whole rustc invocation as cargo reports it with `-v` — `--edition`,
  `--cfg` and features, every `-C` option — with every path written relative to the scratch directory, the program path dropped, the `-<hash>` stripped from
  every `--extern` file name, and `-C metadata`, `-C extra-filename` and the lint levels (`--warn`, `--allow`,
  `--deny`, `--forbid`, `--cap-lints`) left out — the first three hash the toolchain, which is compared by identity,
  and the last change no compiled code — and every `# env-dep` line with its value, an absolute path written
  relative (R1 A6, A10; R2 B8, B9; R3 C11, C12). The root manifest's tables that reach rustc — the edition, the
  profiles — are configuration this way, and its lints are not;
- **every file its compilation read**, from each unit's dependency information, each name resolved lexically
  against the workspace root and refused if it is a symbolic link in the commit, as a repository path with its
  sha256 — the package's sources, and every `include_str!`, `include_bytes!` and `#[path]` input — whatever the
  file is, a build-configuration file a unit reads as data included (R1 A6, A9);
- the root's artifact, the executable or the library, with its sha256;
- the runtime data the pipeline hands it, as its form in `trust/roots.eadl` declares, each with its sha256: none
  today, and `M2.7.5`'s catalog records for `rt-analysis` once that leaf hands them over (R1 A20).

**An input the inventory cannot account for is refused, not trusted** (`trust-undeclared-input`): a path in a
unit's dependency information outside the written tree; and, in any root's closure, a build script, a procedural
macro, a `#[link]` attribute, a `links` key, a link argument in any configuration, or an assembler directive —
`global_asm!`, `asm!` or `naked_asm!`, whose `.incbin` reads a file no dependency information names: measured with
the pinned rustc, a `.incbin` file's bytes are in the rlib and not in its `.d`, for `naked_asm!` too, which needs no
feature gate in 1.95.0 (R2 B3; R3 C1); the catalog's token list holds all three. The catalog refuses each of them in a
recorded package for the same reason (`decision_catalog-records-hashes.md`) (R1 A5). Each is admitted when a leaf
designs its coverage. What remains outside the inventory, and is stated in its report: the linker and the host's C
toolchain, recorded by version; code a `cfg` gates to a target other than the host (R2 B15); what a root reads
at run time that the pipeline does not hand it; and **generated sources**: a file committed as a generator's output is
judged as a file, and two files generated from one input by one generator, differing in bytes, share nothing the
inventory sees — the generator and its input are a provenance it cannot read, until a leaf makes committed generated
sources declare them (R3 C13).

### 4. Shared items

For each pair of roots, a **shared item** is:

- a package both compiled, named by its manifest path, never by name and version alone;
- a repository file both compilations read outside every shared package — two crates including one document;
- a **copy**: a non-empty file whose sha256 appears in both roots' inventories under different paths, so a package
  or file copied byte for byte into a new crate is reported as the original would be (R1 A11);
- a **runtime-data file** both roots' forms declare, at one path or two (R2 B4);
- for a reference model and the implementation it validates, the **comparison harness** — today
  `crates/rt-core/tests/differential.rs`, which holds the adapter between them — every file it compiles beside the
  pair's two libraries (R2 B7).

An item's **content** is the sha256 of its manifest and of every file it contributes to each side; its
**configuration**, each side's unit configuration for it (§3); and its **edges**, the set of (root, edge kind)
through which each side reaches it. A copy edited after copying is not seen, and the gate says so: it enforces
disclosure and change control over what is shared verbatim, never semantic independence (§7). Content is taken
over whole files and whole manifests, which over-approximates: an edit to a shared source file's test module, or to
a shared manifest's development dependencies or description, changes the item though neither root's build changes,
and is reported for review (R1 A12).

### 5. The baseline, and what is decided elsewhere

`trust/baseline.eadl` holds one form per shared item a review has accepted: the item, the pair of roots, its content,
configuration and edges as reviewed, its classification — infrastructure, interpretation/normalization,
semantic/analysis logic, authoritative data, or reference derivation (§4.4) — the property it can affect, its
residual common-error risk, and the independent controls that remain (§14.4). The gate compares the inventory with
it and reports the difference; a form the tool writes is a proposal, never an acceptance (§14.4).

**Accepting a form is decided where reviews are protected, and what an unreviewed item costs is decided where it is
consumed** (R3 C3–C9, C16). Each of these leaves carries its part in its acceptance:

- `M2.7.6`'s protection, under findings §11: who may accept a form or a classification — an identity other than the
  author's, by the code-owner rule `M2.7.6.5` writes, `trust/` among its paths — on which main line, with the
  catalog's checks of `origin` (`decision_catalog-records.md` §4), and per form, so a review of one line accepts that
  line alone;
- `M3.6.5`, blocked on the director: wiring that acceptance into the gate once the named commit exists, the first
  baseline, its comparison harness form included, among what is proposed;
- `M4.8`'s report: every property resting on a role's independence `not-established` while any shared item of that
  role is unreviewed, naming F30;
- `M4.7`'s package: each result bound to the program that produced it — a role's executable, or for the reference
  model the comparison harness the trust build inventories — so no role's results are refused for want of a
  program (R3 C2).

Until then every shared item is unreviewed, the assurance tier's trust step reports **Unavailable**, on every host,
and nothing an author writes is accepted by its author (R2 B2; R3 C4, C5).

### 6. The gate

`cargo xtask trust-gate` builds the inventory as §3 says and compares it with the baseline and the roots. **Refused**
is what its author can repair alone, and fails the gate wherever it runs. **Reported** is what only a review can
settle: the gate lists it as unreviewed and passes, and what that costs a claim, a package or a release is §5's
leaves' (R2 B1, B10, B17; R3 C3).

| Code | Outcome | When | §14.4 case |
| --- | --- | --- | --- |
| `trust-new-shared` | reported | a shared item the baseline does not hold — a package, a file, a copy, a runtime-data file, a comparison harness | 1, and 3 for data |
| `trust-shared-changed` | reported | an item whose content, configuration or edges differ from its baseline form — a feature activated, a `cfg` set, an edition changed, a source edited — though its name and version are unchanged | 2 |
| `trust-unclassified-program` | reported | a program target `trust/roots.eadl` does not classify, or whose classification's role libraries have grown (§2) | — |
| `trust-baseline-stale` | refused | a baseline form whose item is no longer shared: removed, so a sharing removed and reintroduced is reviewed again (R1 A15; R3 C9) | — |
| `trust-undeclared-input` | refused | a path outside the written tree, a symbolic link, or a build script, procedural macro, native link input or assembler directive in a root's closure (§3) | 3 |
| `trust-shared-program` | refused | an executable that would run two roles (§2) | — |
| `trust-inventory-stale` | refused, at packaging | the inventory missing, its build identity not the package's commit and toolchain, or an artifact's sha256 not the inventory's | 4 |

**Case 4 is judged where an inventory is consumed** (R1 A8). The gate builds its own inventory and cannot find it
stale. `cargo xtask trust-verify <package>` refuses a package whose inventory is missing, whose build identity is not
the package's commit and toolchain, or whose artifact differs from the inventory's; the package ships the artifacts
the trust build produced, so no reproducibility between two builds is assumed, and how each result is bound to its
producer is `M4.7`'s (§5). `M3.6.3` builds and tests `trust-verify` against package directories made for the
purpose (R3 C15).

**Case 5.** The gate compares shared items only — their content, configuration and edges — on the host the baseline
names (§2). A change outside every root's packages and files changes the build identity — the commit's tree hash
always, `Cargo.lock`'s when a package is added — which the inventory records and the gate does not compare, so it
reports "unchanged", with no warning (R1 A13). A new package with no program target is such a change; one with a
program target is reported for classification, never case 5 (R2 B10). Configuration is taken from each root's own
build, never from workspace-wide feature unification, so a non-root manifest enabling a feature of a shared package
changes no root's item (R1 A10); and a toolchain bump or a lint-level edit changes no compared configuration (§3). The
gate's tests build the five cases in scratch workspaces, as the catalog's build checker's tests do, each asserting its
code and nothing else.

### 7. Where it runs, and what it is not

- **Where:** the `assurance` tier's `trust-inventory` step (`xtask/src/main.rs`, owned by `M3.6`), which every
  release runs and which reports Unavailable until §5's leaves land; CI on every pull request and on `main`,
  unfiltered, since case 5 makes it silent on an unrelated change and a path-filtered required check would be left
  pending (R1 A16; R2 B17); and the package verifier of §6. The artifact package (`M4`) carries the inventory, the report and the
  trust build's artifacts.
- **What it is not:** a proof of semantic independence. Different crate names do not establish independent
  derivation; a copy edited after copying, or a formula re-derived by hand, is not seen. The gate enforces disclosure
  and change control, and the classification and the controls are the reviewer's (§4.4).

## Why

- §4.4 and §14.4 make independence a disclosed, reviewed property rather than a crate-name claim. A dependency graph
  alone over-approximates what is compiled and misses what is read, so the inventory takes what the compiler read
  for the one configuration a claim's program is built in, from the commit's own files, and refuses what it cannot
  see rather than trusting it.
- A root is the program a claim's independence is about — the executable that runs a role — because a library
  inventoried alone misses what its caller compiles in beside it; and a reference model's counterpart is a root,
  because §6.3's independence is between the two.
- Accepting a shared item needs what the catalog's protection needs — an identity other than the author's on a
  protected main line — and what it costs a claim is the report's to say; this record's three review rounds found
  twenty defects in the parts that decided those things here, so it hands them to the leaves that own them and
  decides what only it can: what is built, what is read, and what is shared.
- Today's empty intersection lets the mechanism be built and its five cases tested now, while the first real shared
  item, which `M2.7.5` will create, is the moment F30 exists for.

## How to apply

- A new executable target is classified in `trust/roots.eadl` — a root in one of §2's roles, or not a root, with a
  reason — under §5's leaves; unclassified, it is reported.
- A shared item is accepted only by a reviewed baseline form, under §5's leaves; regenerating the baseline proposes,
  never accepts.
- A build script, procedural macro, native link input or assembler directive (`global_asm!`, `asm!`, `naked_asm!`)
  in a root's closure needs a leaf that designs its coverage first (§3; R3 C1).
- Related: [[decision_zero-dependency-engine-core]], [[decision_findings-for-director-review]] (§11),
  [[decision_catalog-records]].

## Review

`M3.6.1`'s acceptance is a review, by a context that did not write this record, finding that the gate as specified
exercises each of §14.4's five cases, that no input a root's build reads can be shared without being reported, and
that the baseline cannot be accepted by its author; every finding answered here. The history is
[`decision_trust-inventory-reviews.md`](../reviews/decision_trust-inventory-reviews.md).

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 20 | 11 (A1, the generator rooted at a library, not the program that generates; A2, the reference model's counterpart not a root; A3, the root set the author's to change; A4, baseline acceptance not mechanical; A5, build-script, macro and link inputs unseen; A6, the environment and outside configuration uncontrolled; A7, sources not bound to the commit; A8, case 4 without a consumer; A9, one file under two paths; A10, no feature set, workspace-unified features; A11, verbatim copies unreported) | "not acceptable as it stands"; §1's facts confirmed, cases 1, plain 2 and 5 working |
| 2 | 18 | 12 (B1, the required check's acceptance unreachable from the pull request that proposes a form; B2, no bootstrap; B3, a file read by the assembler unseen; B4, runtime data at one path unshared; B5, one executable running two roles; B6, programs are targets; B7, the reference model's harness outside every inventory; B8, host-dependent configuration; B9, an edition change uncompared; B10, case 5 stated two ways; B11, results unbound to the program that produced them; B12, `CARGO_HOME` not emptied) | "not acceptable as it stands"; §1 re-measured, the copy rule silent on today's tree, the reused rules real |
| 3 | 22 | 16 (C1, `naked_asm!` unrefused; C2, the reference model's results refused for ever; C3–C5, pending items reaching packages, claims and releases, and off the named host; C6–C8, the roots accepted whole, `trust/` owned by no leaf, a fork's main; C9, a stale form stated two ways; C10, the harness shared today; C11–C12, the toolchain and lint flags in the compared configuration; C13, generated sources; C14, a `cdylib` and growing closures; C15–C16, `M3.6.3` and findings §11 stale) | "not acceptable as it stands"; the narrowing that followed hands acceptance and its costs to `M2.7.6`, `M3.6.5`, `M4.7` and `M4.8` |
