# The trust-dependency inventory and its gate: what each root is built from, and what two roots share

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active` — written; under independent review (leaf `M3.6.1`'s closure rule: the first round that finds
  no defect closes it)
- **External sources:** [the pinned Rust toolchain](../book/src/ledger.md#rust-toolchain) — rustc's dependency
  information and cargo's metadata, their version and limits in the ledger
- **Owner / source:** leaf `M3.6.1` (`docs/tasks/M3.md`). `ROADMAP.md` §4.4 asks for a machine-readable
  dependency and provenance inventory for each generator, configuration checker, scheduling checker and
  reference-model build, rooted in the actual build, with every item two of them share classified and compared
  against a reviewed baseline; §14.4 makes it fixture F30, with five cases the gate must exercise. The facts below
  were measured `2026-10-03` with `cargo metadata --format-version 1 --offline` and the build's dependency
  information.

## The fact / decision

The integration pipeline builds each **root** — each program whose independence a claim relies on — from a clean
scratch build under the pinned toolchain, reads what the compiler read for it, and writes an **inventory**: the
build's identity, and for each root the packages it reaches and every file its compilation read, with their
content hashes. From the inventories it derives every item two roots **share**. A **baseline**, committed and
reviewed, lists each shared item the project accepts, with its classification, the property it can affect, its
residual common-error risk and the independent controls that remain. The **gate** fails on a shared item the
baseline does not hold, on an accepted one whose content or edges changed, on an input the inventory cannot
account for, and on an inventory that is missing, stale or bound to another build; it reports nothing for a change
outside every root. The tool measures and compares; it never classifies, and it never accepts — a baseline change is
accepted only by a reviewer who is not its author.

### 1. Measured `2026-10-03`

- 12 packages, all workspace members; **no third-party package** (`decision_zero-dependency-engine-core.md`); no
  build script, no procedural macro, no Cargo feature.
- The roots §4.4 names, as they exist today:

| Role | Root | Reaches (normal and build edges) |
| --- | --- | --- |
| generator | `archogen-s0` (until `S0-RETIREMENT` retires it; `archogen-emit` from `M4.2`) | `eadl-front`, `eadl-model` |
| configuration checker | none yet — `archogen-check` is `M3.5`'s | — |
| scheduling checker | `rt-analysis` | `archogen-evidence` |
| reference model | `rt-reference` | nothing |

- **No two roots share a package today.** The first baseline is therefore empty, and the gate passes on it without
  any review.
- Compile-time non-Rust inputs are in the dependency information rustc writes (`*.d`): `archogen-s0` compiles in
  its runtime's sources with `include_str!`, and `archogen-api` the kind modules; the workspace's other
  `include_str!` of documents sit in test modules, which no root's build compiles.
- **Foreseen:** `M2.7.5` makes `rt-analysis` read the catalog through `archogen-catalog`, which reaches `eadl-front`
  and `eadl-model`, the generator's own reader and model. That is §14.4's case 1, a newly shared path, and the
  gate will fail on it until the baseline holds those two packages, reviewed.

### 2. Roots

A root is declared in `trust/roots.eadl`, read by the eADL reader as every repository record is, one form per root:
its role (one of §4.4's four), its package, its kind of artifact (library or binary), its target triple and its
profile. `/1` builds every root for the host triple in the `release` profile with no feature, so the configuration
a claim's binary is built in is the one inventoried. The roots are exactly the programs in §4.4's four roles. The
catalog's gate (`cargo xtask catalog-check`) and the description checker (`archogen check`) are not roots: the
first guards the catalog, not a claim's independence; the second is interpretation the generator and the
configuration checker will both reach, so it appears as a shared item, which is where §4.4 wants it.

### 3. The inventory

`cargo xtask trust-inventory` writes `target/trust/trust-dependencies.json` and `target/trust/report.txt`. For the
build as a whole it records the **build identity**: the commit and its tree hash; the toolchain (`rustc -vV`,
`cargo -V`); the host triple; and the sha256 of `Cargo.lock`, the root `Cargo.toml`, `rust-toolchain.toml` and
`.cargo/config.toml` — the build configuration, which every root reads by construction, recorded once here and never
as a shared item. For each root, from a clean build in `target/trust/<root>/` with `--locked --offline` and the
dependency information enabled:

- the packages it reaches, each with its manifest's sha256 and the kind of every edge that reached it (normal,
  build, procedural macro); development edges are not followed, since a test is not the root;
- **every file the compilation read**, from each unit's dependency information, as a repository path with its
  sha256 — the package's sources, and every `include_str!`, `include_bytes!` and `#[path]` input — and every
  `# env-dep` line with its value;
- every build script and procedural macro, with the files its own compilation read and, for a build script, the
  directory it writes and every file the dependent unit read from there;
- the root's artifact, with its sha256.

A file outside the repository is accounted for only when it is inside the pinned toolchain's sysroot, recorded by
the toolchain's identity rather than per file. Any other path in a root's dependency information is an
**undeclared input**. The inventory declares its coverage: the linker and the host's C toolchain, if a root links
one, are recorded by version and not hashed; what a root reads at run time — a catalog record, a kind module it is
handed — is not in any dependency information, and is the pipeline's to record when it hands it over (`M4`, which
first runs a generator and a checker over a description), until when the inventory states that no root is given
runtime data by the pipeline.

### 4. Shared items

For each pair of roots, a **shared item** is a package both reach, or a repository file both compilations read
outside every shared package — two crates including one document, for example. An item's **content** is the sha256
of its manifest and of every file it contributes to each side, and its **edges** the set of (root, edge kind,
features) through which each side reaches it. Packages are named by their manifest path, never by name and version
alone, so a renamed copy is a new item and an unchanged name hides no change (§14.4).

### 5. The baseline

`trust/baseline.eadl` holds one form per accepted shared item: the item, the pair of roots, its content and edges
as last reviewed, its classification — infrastructure, interpretation/normalization, semantic/analysis logic,
authoritative data, or reference derivation (§4.4) — the property it can affect, its residual common-error risk,
and the independent controls that remain (§14.4). Each form names the review that accepted it. A review is accepted
under the rule findings §11 asks the director to set for the catalog checker's closure: a code-owner approval by an
identity other than the change's author, on a protected `main` (`decision_findings-for-director-review.md` §11). Until
that rule is set, no baseline form can be accepted, so the gate fails on any shared item at all; with today's empty
intersection it passes, and the first shared item (`M2.7.5`, §1) waits on the director, as F30 intends a checker's
new dependency on the generator's code to wait. A baseline the tool writes is never accepted: regenerating it
proposes, and only the review accepts (§14.4).

### 6. The gate

`cargo xtask trust-gate` builds the inventory and compares it with the baseline. Each refusal names the item, the
pair of roots and what changed:

| Code | When | §14.4 case |
| --- | --- | --- |
| `trust-new-shared` | a shared item the baseline does not hold | 1, and 3 for a build script, a procedural macro, a generated input or a data file |
| `trust-shared-changed` | an accepted item whose content or edges differ from the baseline's — a feature activated, a source edited, a new edge kind — though its package name and version are unchanged | 2 |
| `trust-undeclared-input` | a path in a root's dependency information outside the repository and the sysroot, or a build script or procedural macro whose outputs the inventory cannot account for | 3 |
| `trust-inventory-stale` | the inventory missing, or its build identity, or a root's artifact sha256, differing from the build it is attached to | 4 |
| `trust-baseline-unreviewed` | a baseline form whose review is not accepted under §5 | — |

A change outside every root's packages and files changes no inventory section but the commit, and the gate reports
"unchanged", with no warning (§14.4 case 5). The gate's own tests build the five cases in scratch workspaces, as
the catalog's build checker's tests do, each asserting the code above and nothing else.

### 7. Where it runs, and what it is not

- **Where:** the `assurance` tier's `trust-inventory` step (`xtask/src/main.rs`, owned by `M3.6`), which every
  release runs, and CI on a change to a root's packages, the build configuration or `trust/`. The artifact package
  (`M4`) carries the inventory and the report, bound to the build by §3's identity.
- **What it is not:** a proof of semantic independence. Different crate names do not establish independent
  derivation, and a copied formula in an unshared package is not seen; the gate enforces disclosure and change
  control, and the classification and the controls are the reviewer's (§4.4).

## Why

- §4.4 and §14.4 make independence a disclosed, reviewed property rather than a crate-name claim; a dependency
  graph alone over-approximates what is compiled and misses what is read, so the inventory takes what the compiler
  read for the one configuration a claim's binary is built in, and states what it cannot see.
- The baseline's acceptance rides on the catalog's reviewer rule rather than a second mechanism, because both need
  the same thing — an identity other than the author's on a protected main line — and only the director can supply
  it.
- Today's empty intersection lets the mechanism be built and its five cases tested now, while the first real shared
  item, which `M2.7.5` will create, is the moment F30 exists for.

## How to apply

- A new root is a form in `trust/roots.eadl`, in one of §4.4's four roles; a new kind of input the compiler reads
  extends §3.
- A shared item is accepted only by a reviewed baseline form (§5); regenerating the baseline proposes, never accepts.
- Related: [[decision_zero-dependency-engine-core]], [[decision_findings-for-director-review]] (§11),
  [[decision_catalog-records]].

## Review

`M3.6.1`'s acceptance is a review, by a context that did not write this record, finding that the gate as specified
exercises each of §14.4's five cases, that no input a root's build reads can be shared without being reported, and
that the baseline cannot be accepted by its author; every finding answered here. The history is
[`decision_trust-inventory-reviews.md`](../reviews/decision_trust-inventory-reviews.md).

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
