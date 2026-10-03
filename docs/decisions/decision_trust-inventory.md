# The trust-dependency inventory and its gate: what each root is built from, and what two roots share

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active` — written; under independent review (leaf `M3.6.1`'s closure rule: the first round that finds
  no defect closes it); rounds 1 and 2 answered `2026-10-03`
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
controls that remain. The **gate** gives each item one of three outcomes: **refused** — an input the inventory
cannot account for, an executable running two roles, an inventory stale at packaging — which an author repairs
alone and which fails the gate wherever it runs; **pending** — a shared item the baseline does not hold, an accepted
one whose content, configuration or edges changed, an unclassified program, a change to the roots or the baseline
not yet reviewed — which only a review settles, and which fails the release's assurance step and nothing else; and
**accepted**. It reports nothing for a change outside every root. The tool measures and compares; it never
classifies, and it never accepts — the roots and the baseline are accepted only by a review that is not their
author's, on the protected main line the catalog's premise 3 defines, and before the director names its commit no
claim of independence rests on the gate at all.

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

- **No two roots share a package or a file today**, so the first baseline is empty and the gate passes on it with
  no review.
- **Dependency information names a file as rustc opened it, unnormalised:** `archogen-api`'s names its kind module
  `crates/archogen-api/src/../../../docs/semantics/kinds/core.eadl`. The inventory resolves every name lexically
  before using it (§3).
- **Foreseen:** `M2.7.5` makes `rt-analysis` read the catalog through `archogen-catalog`, which reaches `eadl-front`
  and `eadl-model`, the generator's own reader and model — §14.4's case 1, which will wait on the director's
  reviewer (§5).

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
would make the generator's executable run the scheduling checker; `M2.21` decides how it runs before it is built.

`trust/roots.eadl`, read by the eADL reader as every repository record is, holds one form per root — its role, its
package and target, its artifact and the runtime data the pipeline hands it (§3) — and one form per role no root
fills yet, naming the leaf that will (the configuration checker, `M3.5`). It also classifies every other
**executable target** of the workspace — a `[[bin]]`, a `src/main.rs`, a `src/bin/*.rs` cargo discovers, an example —
as not a root, with a reason. Programs are targets, not packages: an executable added to a classified package is
still a new program (R2 B6). An executable target the file does not classify is pending (`trust-unclassified-program`,
§6), and a classification, like every change to the file, is accepted only by §5's review, so no author removes a
root or classifies their own checker away to silence a sharing (R1 A3). Libraries are not classified: a library
matters only through the roots that reach it. Every root is built in the release profile with
`--no-default-features` and no `--features` (R1 A19), for **one host triple**, the CI runner's
(`x86_64-unknown-linux-gnu`), which the baseline names: cargo's unit hashes and a host's `cfg` differ by host, so on
another host the gate builds and reports but compares nothing (R2 B8).

### 3. The inventory

`cargo xtask trust-inventory` writes `target/trust/trust-dependencies.json` and `target/trust/report.txt`.

**Where it builds.** Each root is built clean from the commit's own blobs, written into `target/trust/<root>/` as
the catalog's build checker writes a judged tree (`xtask/src/catalog_build.rs`, `write_tree`, which refuses a path
that differs from another only in ASCII case); never from the working tree, so an uncommitted or untracked file
cannot stand in for a committed one (R1 A7). It builds with `--locked --offline` under the catalog's cleared,
allowlisted environment (`environment`: `PATH`, `HOME`, the rustup home and the pinned toolchain, a `CARGO_HOME` of
its own, its own target directory, nothing else — no `RUSTFLAGS`, no `RUSTC_WRAPPER`, no `CARGO_PROFILE_*`), and
refuses a cargo configuration found above the repository or under `target/`, or one that is a symbolic link
(`configurations_on_path`) (R1 A6). `CARGO_HOME` is removed and made anew on every run, since `environment` only
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
  `--cfg` and features, every `-C` option — with every path written relative to the scratch directory, and `-C
  metadata` and `-C extra-filename`, which hash the package and the toolchain, left out; and every `# env-dep` line
  with its value, an absolute path written relative (R1 A6, A10; R2 B8, B9);
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
`global_asm!` or `asm!`, whose `.incbin` reads a file no dependency information names: measured with the pinned
rustc, a `.incbin` file's bytes are in the rlib and not in its `.d` (R2 B3). The catalog refuses each of them in a
recorded package for the same reason (`decision_catalog-records-hashes.md`) (R1 A5). Each is admitted when a leaf
designs its coverage. What remains outside the inventory, and is stated in its report: the linker and the host's C
toolchain, recorded by version; code a `cfg` gates to a target other than the host (R2 B15); and what a root reads
at run time that the pipeline does not hand it.

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

### 5. The baseline, and who accepts it

`trust/baseline.eadl` holds one form per accepted shared item: the item, the pair of roots, its content,
configuration and edges as last reviewed, its classification — infrastructure, interpretation/normalization,
semantic/analysis logic, authoritative data, or reference derivation (§4.4) — the property it can affect, its
residual common-error risk, and the independent controls that remain (§14.4). A form whose item is no longer shared
is refused until it is removed, so a sharing removed and later reintroduced is reviewed again (R1 A15).

**Acceptance is the catalog's, and the gate only reports it.** `trust/` and the gate's code join the paths findings
§11 asks the director to put under code-owner review by an identity other than the author's
(`decision_findings-for-director-review.md` §11, amended `2026-10-03`). A baseline form, and the content of
`trust/roots.eadl`, is **accepted** when the commit that last changed it is on `origin/main`'s first-parent chain at
or after the named commit the catalog's premise 3 records, and is a merge commit the hosting made and signed
(`docs/specs/catalog/decision_catalog-records.md`, premise 3); the review is the hosting's code-owner approval, which
the protection `M2.7.6` designs makes the merge's condition, and the gate trusts that design for `trust/` from the
named commit on (R1 A4; R2 B16). On a pull request a form or a roots change is therefore never yet accepted: the gate
reports it pending, and its merge under that protection is what accepts it — so the review is the merge, and the
pull request's run does not wait on itself (R2 B1).

**Before the named commit, nothing is accepted and nothing waits for ever** (R2 B2). The first `trust/roots.eadl`,
every classification and every form are pending; the assurance step, which alone fails on pending items, reports F30
`not-established` instead, naming findings §11, as the catalog's production claims are refused before the same commit.
No independence claim rests on the gate until then, so nothing its author wrote is accepted by its author. The first
acceptance is the first pull request after the named commit that proposes `trust/roots.eadl` and
`trust/baseline.eadl` whole, approved under that protection. A new program before then — `M3.5`'s checker, `M4`'s
generator — is a pending classification, which blocks no merge. A baseline the tool writes is never accepted:
regenerating it proposes, and only the review accepts.

### 6. The gate

`cargo xtask trust-gate` builds the inventory as §3 says, compares it with the baseline and the roots with their
accepted content, and gives each item one of three outcomes. **Refused** is what its author can repair alone, and
fails the gate wherever it runs. **Pending** is what only a review settles: on a pull request and on `main` the gate
reports it and passes, and the release's assurance step fails on it — before the named commit reporting F30
`not-established` instead (§5) — so a pull request that proposes a form or classifies a program is never blocked by
its own review (R2 B1, B10, B17). Each item names the pair of roots and what changed:

| Code | Outcome | When | §14.4 case |
| --- | --- | --- | --- |
| `trust-new-shared` | pending | a shared item the baseline does not hold — a package, a file, a copy, a runtime-data file, a comparison harness | 1, and 3 for data |
| `trust-shared-changed` | pending | an accepted item whose content, configuration or edges differ from the baseline's — a feature activated, a `cfg` set, an edition changed, a source edited — though its name and version are unchanged | 2 |
| `trust-unclassified-program` | pending | an executable target `trust/roots.eadl` does not classify (§2) | — |
| `trust-baseline-unreviewed` | pending | a baseline form, or `trust/roots.eadl`'s content, not accepted under §5; a baseline form whose item is no longer shared (R1 A15) | — |
| `trust-undeclared-input` | refused | a path outside the written tree, a symbolic link, or a build script, procedural macro, native link input or assembler directive in a root's closure (§3) | 3 |
| `trust-shared-program` | refused | an executable that would run two roles (§2) | — |
| `trust-inventory-stale` | refused, at packaging | the inventory missing, its build identity not the package's commit and toolchain, an artifact's sha256 not the inventory's, or a result not produced by the inventory's program for its role | 4 |

**Case 4 is judged where an inventory is consumed** (R1 A8). The gate builds its own inventory and cannot find it
stale; the assurance package (`M4.7`'s identity step) consumes one. The package ships the artifacts the trust build
produced, so their equality with the inventory holds by construction and no second build is compared — this record
assumes no reproducibility between two builds. Each result the package holds names the sha256 of the program that
produced it, and `cargo xtask trust-verify <package>` refuses a result whose producer is not the inventory's artifact
for its role, and any result of a role rooted at a library, since no inventoried program produced it (R2 B11); it
refuses too a package whose inventory is missing, whose build identity is not the package's commit and toolchain, or
whose artifact differs from the inventory's. `M3.6.3` tests it against package directories built for the purpose,
and `M4.7`'s acceptance carries the binding.

**Case 5.** The gate compares shared items only — their content, configuration and edges — on the host the
baseline names (§2), and the roots' and baseline's acceptance. A change outside every root's packages and files
changes the build identity — the commit's tree hash always, `Cargo.lock`'s when a package is added — which the
inventory records and the gate does not compare, so it reports "unchanged", with no warning (R1 A13). A new package
with no executable target is such a change; one with an executable target is a pending classification, never case 5
(R2 B10). Configuration is taken from each root's own build, never from workspace-wide feature unification, so a
non-root manifest enabling a feature of a shared package changes no root's item (R1 A10). The gate's tests build the
five cases in scratch workspaces, as the catalog's build checker's tests do, each asserting its code and nothing
else.

### 7. Where it runs, and what it is not

- **Where:** the `assurance` tier's `trust-inventory` step (`xtask/src/main.rs`, owned by `M3.6`), which every
  release runs and which alone fails on pending items; CI on every pull request and on `main`, unfiltered, since case
  5 makes it silent on an unrelated change and a path-filtered required check would be left pending (R1 A16; R2
  B17); and the package verifier of §6. The artifact package (`M4`) carries the inventory, the report and the
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
- The roots and the baseline are accepted by the catalog's reviewer rule rather than a second mechanism, because both
  need the same thing — an identity other than the author's on a protected main line — and only the director can
  supply it.
- Today's empty intersection lets the mechanism be built and its five cases tested now, while the first real shared
  item, which `M2.7.5` will create, is the moment F30 exists for.

## How to apply

- A new executable target is classified in `trust/roots.eadl` — a root in one of §2's roles, or not a root, with a
  reason — under §5's review; unclassified, it is pending.
- A shared item is accepted only by a reviewed baseline form (§5); regenerating the baseline proposes, never accepts.
- A build script, procedural macro or native link input in a root's closure needs a leaf that designs its coverage
  first (§3).
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
