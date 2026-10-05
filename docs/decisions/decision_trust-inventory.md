# The trust-dependency inventory and its gate: what each root is built from, and what two roots share

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active` — written; under independent review (leaf `M3.6.1`'s closure rule: the first round that finds
  no defect closes it); rounds 1 to 6 answered `2026-10-03`, round 7 `2026-10-05`, its hand-offs in a ledger since; narrowed after rounds 3 and 4 to what only it decides
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
two roots **share**, copies and the build configuration included. A **baseline**, committed, holds one form per
shared item, proposed or accepted — accepted only as `M3.6.5` reads acceptance, never by a form's presence (R7 5) —
with its classification, the property it can affect, its residual common-error
risk and the independent controls that remain. The **gate** refuses what would make the inventory unsound — an input
it cannot account for, a program running two roles, a baseline form whose item is gone, and, at packaging, a stale
inventory — and **reports** in two parts: the **change** against the base commit's forms — the merge base of a
pull request, `main`'s first parent — a shared item new, changed or gone, a root form added, changed or removed, a
program target not classified or whose role packages grew, so forms the commit itself adds or alters never shrink it
(R6 5); and the **standing list** of every shared item, root form and classification not accepted. For a change outside every root and every program target the change is
empty and the gate reports "unchanged"; the standing list is not a loss-of-independence warning (R5 3, 10).

**What this record does not decide** (narrowed after its third and fourth reviews, R3, R4): who reviews a shared
item or a classification, how a review is recorded and protected, what an unreviewed item, an unclassified program or
an unaccepted form costs a claim, a package or a release, and the outcome of the assurance tier's trust step. Those
are the protection the catalog's premise 3 rests on (`M2.7.6`, findings §11), the gate's acceptance (`M3.6.3`,
`M3.6.5`), and the claims and the package that consume the report (`M4.7`, `M4.8`), each of which carries its part
in its acceptance (§5). The tool measures and compares; it never classifies, and it never accepts.

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

- **No two roots share a package, or a file compiled for them, today.** What they share is the build configuration
  every root is compiled under — the toolchain and the root manifest's profiles (§3; R4 7) — and, for the reference
  model and its implementation, the comparison harness through which one validates the other —
  `crates/rt-core/tests/differential.rs` and the adapter it holds, written by `rt-core`'s author — shared by design
  and disclosed by its header (§4; R3 C10).
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
shares (R2 B7), named in the pair's form. Each root's form also names its **role packages**: the packages holding
the role's own logic, its root package always, reviewed with the form — today the generator's `archogen-cli`,
`archogen-api` and `archogen-s0`, the scheduling checker's `rt-analysis`, the reference model's `rt-reference` and the
implementation's `rt-core`; the language's reader and model, `eadl-front` and `eadl-model`, are no role's, so a
checker that compiles them shares them, reported as case 1 (R7 remark j). **An executable that would run two roles is
refused** (`trust-shared-program`): an executable root whose build compiles a role package of another role — the
comparison harness apart, which compiles its pair's two by design (R7 remark d). The
independence §4.4 asks about is between programs, and one program running both roles has none to disclose; every
package it holds would be shared, and a real case 1 would be lost among them (R2 B5; R4 5). Compiling a role package
counts, whatever the program calls of it: one that needs only another role's types or constants takes them from a
third package, which the gate then reports as shared (R5 remark 15). `ROADMAP.md` §10.2's `archogen analyze`
would make the generator's executable run the scheduling checker; `M2.21` decides how it runs before it is built, and `M3.4` carries the same for `archogen resolve` (R3 C20).

`trust/roots.eadl`, read by the eADL reader as every repository record is, holds one form per root — its role, its
package and target, its artifact and the runtime data the pipeline hands it (§3) — and one form per role no root
fills yet, naming the leaf that will (the configuration checker, `M3.5`). It also classifies every other
**program target** of the workspace — a `[[bin]]`, a `src/main.rs`, a `src/bin/*.rs` cargo discovers, an example, and a
`cdylib`, `staticlib` or `dylib` crate type, such as the wasm module `ROADMAP.md` §10.4 makes a consumer of the
engine — as not a root, with a reason and the role packages its build compiles, read from `cargo metadata`'s graph
by normal edges and, for an example, by its development edges too, since an example compiles them (R5 remark 13);
when that set grows, the classification is unreviewed again (R2 B6; R3 C14; R4 5). Programs are targets, not packages: an executable added to a
classified package is still a new program. The gate computes sharing from the roots the commit proposes, never only
from those last reviewed (R3 C19). A program target the file does not classify is reported unreviewed (`trust-unclassified-program`,
§6), and a classification, like every change to the file, is accepted only by the review §5's leaves define, so no author removes a
root or classifies their own checker away to silence a sharing (R1 A3). Libraries are not classified: a library
matters only through the roots that reach it. Every root is built in the release profile with
`--no-default-features` and no `--features` (R1 A19), for **one host triple**, the CI runner's
(`x86_64-unknown-linux-gnu`), which the base commit's baseline names — a change to it is in the commit's change part —
since cargo's unit hashes and a host's `cfg` differ by host; on another host the gate builds, applies every refusal but
`trust-baseline-stale` and `trust-form-missing`, reports every shared item as unreviewed, and its change part says
"not compared", never "unchanged" (R2 B8; R4 10; R7 8).

### 3. The inventory

`cargo xtask trust-inventory` writes `target/trust/trust-dependencies.json`; the gate writes the report (§6).

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
refused there, as the catalog's `metadata` refuses them, so none runs (R2 B12). The comparison harness a pair's form
names is built the same way, with `cargo test --release --no-run -p <package> --test <name>`, and recorded as a root
is — its files, its configuration and its artifact — beside the pair's two libraries, whose compilation units it
reuses (measured, R4 4).

**What it records, for the build as a whole:** the commit and its tree hash; the toolchain (`rustc -vV`,
`cargo -V`); the linker cargo invokes on the host, by `cc --version`'s first line (R6 remark 11); the host triple; and
the sha256 of `Cargo.lock`, the root `Cargo.toml`, `rust-toolchain.toml` and `.cargo/config.toml`. The toolchain's
own `std` and compiler, and the root manifest's tables that reach rustc — the edition it gives, and the profile tables
that apply to a root's units, `[profile.release]` and its overrides for packages a root compiles, read by TOML's
meaning (R6 remark 14) — are compiled into every root: they are the **build
configuration**, an item every pair shares (§4), recorded by the toolchain's identity and those tables and never per
sysroot file, since the dependency information names no path under the sysroot. `Cargo.lock` is recorded and is not
part of it: a package added elsewhere changes it and no root's compilation (case 5) (R1 A14; R4 7).

**What it records, for each root:**

- the packages its build compiled, from that build's units and not from workspace-wide metadata, each with its
  manifest's sha256 and the kind of every edge that reached it (normal, build), development edges not followed,
  since a test is not the root (R1 A10, A17);
- each compilation unit's **configuration**: its whole rustc invocation as cargo reports it with `-v` — `--edition`,
  `--cfg` and features, every `-C` option — with every path written relative to the scratch directory, the program path dropped, the `-<hash>` stripped from
  every `--extern` file name, and `-C metadata`, `-C extra-filename`, the lint levels (`--warn`, `--allow`, `--deny`,
  `--forbid`, `--cap-lints`), `--check-cfg` and the output-format flags (`--error-format`, `--json`,
  `--diagnostic-width`, `--color`) left out — the first three hash the toolchain, which the build configuration
  holds by identity, the lint levels and `--check-cfg` change only what is warned about, and the format flags only
  how, `--diagnostic-width` differing with the terminal (measured, R4) — and every `# env-dep` line with its value,
  an absolute path written relative (R1 A6, A10; R2 B8, B9; R3 C11, C12). The root manifest's tables that reach
  rustc — the edition, the profiles — are configuration this way, and its lints are not;
- **every file its compilation read**, from each unit's dependency information, each name — an absolute one with the
  written tree's prefix stripped first — resolved lexically against the workspace root, and refused if it is then
  not a blob of the commit, or is a symbolic link in it; after the build each file read holds its blob's bytes, as
  the catalog's build checker checks (R4 remark 7; R5 remark 14), as a repository path with its sha256 — the package's sources, and every `include_str!`, `include_bytes!` and `#[path]` input — whatever the
  file is, a build-configuration file a unit reads as data included (R1 A6, A9);
- the root's artifact, the executable or the library, with its sha256;
- the runtime data the pipeline hands it, as its form in `trust/roots.eadl` declares, each by path with its sha256:
  none today, and `M2.7.5`'s catalog records for `rt-analysis` once that leaf hands them over (R1 A20). A root's
  **dependencies** — fixed data, the same for every claim — are what its form declares; the **subject** of a claim —
  the description under check, the plan the generator produced — is not a dependency, and is recorded with the
  claim, not in a form (R6 3).

**An input the inventory cannot account for is refused, not trusted** (`trust-undeclared-input`), and on any
refusal the tool writes no inventory, only the report naming it, so `trust-verify` finds none and refuses the package
(R5 9). **The rules are the catalog's, whole, and the gate is default-deny** (R7): every manifest and token rule the
catalog applies to a recorded package — `check_manifest` and `scan` in `crates/archogen-catalog/src/package.rs`,
unchanged: a build script, a procedural macro, a `links` key, a link argument, `asm`, `global_asm`, `naked_asm`,
`include` and its kin, `no_mangle`, `export_name`, `link_section`, the global hooks, `macro_rules` and `macro`,
`link`, `path` and `used` inside an attribute or a macro's arguments, `extern` other than `extern crate` or an `extern
fn`, a non-ASCII identifier, a file that does not tokenize — applies to every `.rs` file a root's compilation reads,
and to the comparison harness's closure with its development edges followed, since the harness is built from them (R7
3); and a name in a unit's dependency information that is not a blob of the commit, or is a symbolic link, is
refused. A site the rules refuse passes only when an **admission** in `trust/roots.eadl` names it — the file, the
rule, and the sha256 of the line it stands on — reviewed as every form is (§5); an edit to that line, or a new site,
is refused until a new admission is. Measured `2026-10-05` over the 55 `.rs` files today's roots and harness compile,
the rules refuse 13 sites, each proposed as an admission: eight `include_str!` of data — the two kind modules
(`crates/archogen-api/src/lib.rs`), the profile page (`crates/eadl-model/src/profile.rs`), the roadmap
(`crates/archogen-cli/src/spec.rs`) and four of S0's runtime templates (`crates/archogen-s0/src/emit.rs`); three
`#[no_mangle]` on the wasm module's exports (`crates/archogen-wasm/src/lib.rs`); and two uses of the word `path` in an
ordinary macro call (`crates/eadl-front/src/module.rs`, `crates/rt-analysis/src/cost.rs`). Everything else is
refused. One rule set so closes the channels five rounds found one at a time — an `.incbin` under the assembler
macros, its bytes in the rlib and in no `.d` (R2 B3; R3 C1), a renaming `use` (R4 8), a link-time binding through an
`extern` block (R6 1), a macro assembling `#[path]` onto a file that is not `.rs` (R7 1), a `#[no_mangle]` interposing
a symbol in the harness, measured taking over `memcmp` (R7 2), a procedural macro on a development edge (R7 3) — and
refuses the next such channel before anyone finds it. Data a compilation reads through an admitted `include_str!` or
`include_bytes!` is hashed and never tokenised (R6 7). A refusal the catalog has not yet adopted is designs its coverage. What remains outside the inventory, and is stated in its report: the linker and the host's C
toolchain, recorded by version; code a `cfg` gates to a target other than the host (R2 B15); what a root reads
at run time that the pipeline does not hand it; and **generated sources**: a file committed as a generator's output is
judged as a file, and two files generated from one input by one generator, differing in bytes, share nothing the
inventory sees — the generator and its input are a provenance it cannot read, until `M3.6.6` makes committed
generated sources declare them, which §14.4's case 3 names (R3 C13; R4 9).

### 4. Shared items

For each pair of roots, a **shared item** is:

- a package both compiled, named by its manifest path, never by name and version alone;
- a repository file both roots' inventories hold — read by a compilation or handed at run time, at one path or two
  (R6 2) — that no shared package's own compilation reads: two crates including one document, a file a shared
  package's macro makes both roots include, which that package's own dependency information does not name (measured,
  R4 6), or a kind module one root compiles and another is handed;
- a **copy**: a non-empty file whose sha256 appears in both roots' inventories under different paths, so a package
  or file copied byte for byte into a new crate is reported as the original would be (R1 A11);
- for a reference model and the implementation it validates, the **comparison harness** the pair's form names —
  today `rt-core`'s test target `differential`, which holds the adapter between them — every file it compiles beside
  the pair's two libraries, built as §3 says, its edge to the reference library of kind `development` (R2 B7; R4 4;
  R6 remark 12); and the harness is paired with every root other than its pair's as well, as the program that
  produces the reference model's results (`M4.7`), so a package its development dependencies add is shared with any
  other root that compiles it (R5 remark 11);
- for every pair, the **build configuration** of §3: the toolchain's identity and the root manifest's tables that
  reach rustc (R4 7).

An item's **content** is the sha256 of its manifest and of every file it contributes to each side; its
**configuration**, each side's unit configuration for it (§3); and its **edges**, the set of (root, dependent package,
edge kind) for every edge into it, so a new consumer of a package already shared is a change though nothing in the
package is (measured, R5 2); and each file of a package item's content carries its readers, the set of (root,
reading package), so a package that is not shared starting to read a shared package's file is a change too
(measured, R6 4). A copy edited after copying is not seen, and the gate says so: it enforces
disclosure and change control over what is shared verbatim, never semantic independence (§7). Content is taken
over whole files and whole manifests, which over-approximates: an edit to a shared source file's test module, or to
a shared manifest's development dependencies or description, changes the item though neither root's build changes,
and is reported for review (R1 A12).

### 5. The baseline, and what is decided elsewhere

`trust/baseline.eadl` holds one form per shared item, proposed or accepted (R7 5): the item, the pair of roots, its
content, configuration, edges and file readers as proposed (R7 remark b), its classification — infrastructure, interpretation/normalization,
semantic/analysis logic, authoritative data, or reference derivation (§4.4) — the property it can affect, its
residual common-error risk, and the independent controls that remain (§14.4). The gate compares the inventory with
it and reports the difference; a form the tool writes is a proposal, never an acceptance (§14.4).

**Accepting a form is decided where reviews are protected, and what an unreviewed item costs is decided where it is
consumed** (R3 C3–C9, C16). Each hand-off is one sentence in this ledger, quoted word for word beside its identifier
by the leaf that takes it, which `HANDOFF-LEDGER` checks (`PROGRAM.52.2`; R7 remark a); the reasons, and the rounds
that found each, are in the sections above and the review history:

<!-- machine-read: handoffs -->
| Id | Leaf | Obligation |
| --- | --- | --- |
| `TI-H1` | `M2.7.6.5` | `.github/CODEOWNERS` owns `trust/` and the trust gate's code by owners the director named, so a change to either is approved only by an identity other than its author's, on the protected main line with the catalog's checks of `origin`, shown by `M2.7.6`'s scratch-branch tests. |
| `TI-H2` | `M3.6.3` | `cargo xtask trust-gate` writes its report in two parts — the change against the base commit's forms, and the standing list of every shared item, root form, classification and admission not accepted — naming the build identity and the inventory's and the baseline's sha256. |
| `TI-H3` | `M3.6.3` | This leaf commits, unaccepted, the proposed forms of today's shared items, the baseline's host, the classifications of today's non-root program targets and the admissions of today's 13 refused sites, so case 5's change part is empty before any acceptance exists. |
| `TI-H4` | `M3.6.3` | `trust-form-missing` refuses, on the baseline's host, a current shared item, root, program target or refused site with no form in the commit's own `trust/`, and an unrelated commit after a merged sharing reports "unchanged". |
| `TI-H5` | `M3.6.3` | Off the baseline's host the gate applies every refusal but `trust-baseline-stale` and `trust-form-missing`, reports every shared item unreviewed, and says "not compared" in its change part. |
| `TI-H6` | `M3.6.3` | The gate's program is built from the base commit, as the catalog's checker is. |
| `TI-H7` | `M3.6.3` | The `assurance` tier's `trust-inventory` step is a runner action that runs the gate, `Failed` on a refusal and otherwise not built for the acceptance it lacks, owned by `M3.6.5`, so never Passed before it whatever is shared. |
| `TI-H8` | `M3.6.3` | `cargo xtask trust-verify` refuses a package whose inventory is missing, stale or of another build, whose report is of another inventory, in which a result names a program other than its role's inventoried artifact or the pair's harness, or which records a dependency handed to a root that its form does not declare by path and sha256, each tested on package directories made for the purpose. |
| `TI-H9` | `M3.6.3` | F30's five cases are built in scratch workspaces, each asserting the code and outcome the record's §6 gives it, case 5 reporting "unchanged", and a mutation matrix removes each refusal in turn. |
| `TI-H10` | `M3.6.3` | CI runs the gate on every pull request and on `main`, unfiltered. |
| `TI-H11` | `M3.6.5` | A form is accepted only by an identity that authored neither the form nor any commit that changed its item since its last accepted form, or for a first acceptance since the commit that proposed it, authorship being the identity the hosting authenticates for the pull request that merged each commit or a verified signature, and a commit whose author cannot be established counting as every identity's, with a forged-author test. |
| `TI-H12` | `M3.6.5` | Acceptance is read per form, on the protected main line with the catalog's checks of `origin`, so the review of one form accepts that form alone, and the forms `M3.6.3` proposed are accepted rather than proposed again. |
| `TI-H13` | `M3.6.5` | The assurance step is Passed only on the baseline's host with every root form, role package, classification, admission and shared item accepted and no program target unclassified; otherwise it is `Failed` on that host, and off it `Failed` on a refusal and `Unavailable` else. |
| `TI-H14` | `M4.7` | Each result in the assurance package names the sha256 of the program that produced it — a role's executable, the scheduling checker's being the one `M2.21` decides, or for the reference model the comparison harness the trust build inventories — and the manifest records each root's handed dependencies by path and sha256 and each claim's subject apart. |
| `TI-H15` | `M4.7` | Packaging runs `cargo xtask trust-verify` on the package it writes and fails on any refusal, a stale inventory among its tests, and an unaccepted item costs a package nothing beyond `M4.8`'s `not-established`. |
| `TI-H16` | `M4.8` | A property resting on a role's independence is established only when each role it rests on has an inventoried root whose form is accepted, every root form, classification and admission is accepted, no program target is unclassified, and every shared item of those roles is accepted under `M3.6.5`; otherwise it is `not-established`, naming F30. |
| `TI-H17` | `M2.7.5` | The catalog records the scheduling checker reads at run time are declared, by path, in its root's form in `trust/roots.eadl`. |
| `TI-H18` | `M3.4` | `archogen resolve` runs as a program that is the root of one role only, and the fixed data its root reads at run time is declared, by path, in that root's form in `trust/roots.eadl`. |
| `TI-H19` | `M3.5` | `archogen-check`'s executable is classified in `trust/roots.eadl` as the configuration checker's root with its fixed run-time data declared there by path, the description and plan it checks being claim subjects and not dependencies, unreviewed until accepted under `M3.6.5`. |
| `TI-H20` | `M4.2` | The executable that runs the generator is classified in `trust/roots.eadl` as the generator's root with its fixed run-time data declared there by path, the description it generates from being a claim subject and not a dependency, unreviewed until accepted under `M3.6.5`. |
| `TI-H21` | `M2.21` | `archogen analyze` gets an open owner and a program that is the root of the scheduling checker's role alone, decided before it is built, since as a subcommand of the `archogen` executable it would be refused as `trust-shared-program`. |
| `TI-H22` | `M3.6.6` | Committed generated sources declare their generator and input, which become items of the inventory, with a case-3 fixture of a generated-source input on both sides of a pair and a design reviewed by a context that did not write it. |
| `TI-H23` | `M3.6.2` | `cargo xtask trust-inventory` applies the catalog's manifest and token rules whole and unchanged over every `.rs` file a root or the harness compiles, following the harness's development edges, a refused site passing only by an admission naming its file, rule and line's sha256, and writes no inventory on any refusal. |
| `TI-H24` | `M3.6.2` | Every channel a review measured by hand is a fixture of the instrument's tests — an `.incbin` under each assembler macro, a renaming `use`, a macro-made include, a macro assembling `#[path]`, an `extern` block calling another package's `#[no_mangle]` function, a `#[no_mangle]` interposing `memcmp` in the harness, a procedural macro on a development edge, a new consumer edge, a new reader of a shared package's file, a profile edit, per-root against workspace-wide features, two checkout directories and two toolchains. |

Until they land nothing is accepted, and every shared item is reported unreviewed (R2 B2; R3 C4, C5).

### 6. The gate

`cargo xtask trust-gate` builds the inventory as §3 says, compares it with the base commit's forms and the roots, and
writes `target/trust/report.txt`, which names the build identity, the inventory's sha256 and the sha256 of the
baseline it compared against, so a report is bound to the build it describes (R6 9). **Refused**
fails the gate wherever it runs, and is repaired by a change to the code or, for a stale form or a two-role program,
to `trust/`, which §5's review then accepts (R4 remark 4). **Reported** is what only a review can settle: the gate lists it as unreviewed and passes, and what that costs a claim, a package or a release is §5's
leaves' (R2 B1, B10, B17; R3 C3).

| Code | Outcome | When | §14.4 case |
| --- | --- | --- | --- |
| `trust-new-shared` | reported | a shared item the base commit's baseline does not hold — a package, a file compiled or handed, a copy, a comparison harness, the build configuration | 1, and 3 for data |
| `trust-shared-changed` | reported | an item whose content, configuration, edges or file readers differ from the base commit's form — a feature activated, a `cfg` set, an edition changed, a source edited, a new reader — though its name and version are unchanged | 2 |
| `trust-unclassified-program` | reported | a program target `trust/roots.eadl` does not classify, or whose classification's role packages have grown (§2) | — |
| `trust-baseline-stale` | refused, on the baseline's host | a baseline form whose item is no longer shared, or a root form or classification whose package or program target is gone (R7 remark e): removed, so a sharing removed and reintroduced, or a target re-added under an old name, is reviewed again (R1 A15; R3 C9; R5 remark 12) | — |
| `trust-undeclared-input` | refused | a name not a blob of the commit, a symbolic link, a file whose bytes after the build are not its blob's, or a site the catalog's rules refuse, in any `.rs` file a root or the harness compiles, with no admission (§3) | 3 |
| `trust-form-missing` | refused, on the baseline's host | a current shared item, root, program target or refused site with no form, proposed or accepted, in the commit's own `trust/` — repaired by the tool's proposal, committed, so the base commit's forms always cover its inventory and case 5's change part is the commit's own (R7 4) | — |
| `trust-shared-program` | refused | an executable root whose build compiles a role package of another role (§2) | — |
| `trust-inventory-stale` | refused, at packaging | the inventory missing, its build identity not the package's commit and toolchain, an artifact's sha256 not the inventory's, the report not of that inventory, a result naming a program other than its role's inventoried artifact or the pair's harness, or a dependency handed to a root that its form does not declare by path and the inventory's sha256 | 4 |

**Case 4 is judged where an inventory is consumed** (R1 A8). The gate builds its own inventory and cannot find it
stale. `cargo xtask trust-verify <package>` refuses a package whose inventory is missing, whose build identity is not
the package's commit and toolchain, or whose artifact differs from the inventory's; one whose report names another
inventory (R6 9); one in which a result names a
program other than the inventoried artifact of its role's root, or of the pair's named harness, so a result a debug
build or another commit's checker produced does not pass (R5 4); and one whose manifest records a dependency handed
to a root that the root's form does not declare, matched by path and the inventory's sha256 — a claim's subject is
checked against the package's own record of it instead (R5 6; R6 3). The package ships the artifacts the trust build produced, so no
reproducibility between two builds is assumed, and how each result names its producer is `M4.7`'s (§5). `M3.6.3` builds and tests `trust-verify` against package directories made for the
purpose (R3 C15).

**Case 5.** The gate compares shared items only — their content, configuration and edges — on the host the baseline
names (§2), and "unchanged" is the change part of its report (the opening), measured against the base commit's
forms, which `M3.6.3` first commits and `trust-form-missing` keeps whole, so a merged sharing cannot leave a later,
unrelated commit to report it (R7 4); the standing list, which holds every item until `M3.6.5` accepts it, is no
warning (R5 3; R6 5, 6, remark 15). A change outside every root's packages and files changes the build identity — the commit's tree hash
always, `Cargo.lock`'s when a package is added — which the inventory records and the gate does not compare, so it
reports "unchanged", with no warning (R1 A13). A new package with no program target is such a change; one with a
program target is reported for classification, never case 5 (R2 B10). Configuration is taken from each root's own
build, never from workspace-wide feature unification, so a non-root manifest enabling a feature of a shared package
changes no root's item (R1 A10); a lint-level or `check-cfg` edit changes no compared configuration, and a
toolchain bump or a profile edit changes the build configuration, every pair's item, reported for review (§3, §4; R4
7). The
gate's tests build the five cases in scratch workspaces, as the catalog's build checker's tests do, each asserting its
code and nothing else.

### 7. Where it runs, and what it is not

- **Where:** the `assurance` tier's `trust-inventory` step (`xtask/src/main.rs`, owned by `M3.6`), which every
  release runs, its outcome `M3.6.3`'s and `M3.6.5`'s (§5); CI on every pull request and on `main`,
  unfiltered, since case 5 makes it silent on an unrelated change and a path-filtered required check would be left
  pending (R1 A16; R2 B17); and the package verifier of §6. The artifact package (`M4`) carries the inventory, the report and the
  trust build's artifacts.
- **Built from the base commit:** the gate's own program is built from the base commit, as the catalog's checker is
  (`M2.7.6`), so a pull request that changes the gate is judged by the gate it changes nothing of (R7 remark c).
- **What it is not:** a proof of semantic independence. Different crate names do not establish independent
  derivation; a copy edited after copying, or a formula re-derived by hand, is not seen. The gate enforces disclosure
  and change control, and the classification and the controls are the reviewer's (§4.4). The gate reads
  `trust/roots.eadl` and `trust/baseline.eadl` with `eadl-front`, the generator's own reader, so a fault there that
  misreads a form is shared with the generator and unseen by this gate; a form's review reads its text, not the
  gate's reading of it (R4 remark 9).

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

- A new program target is classified in `trust/roots.eadl` — a root in one of §2's roles with its role packages, or
  not a root, with a reason and the role packages its build compiles — under §5's leaves; unclassified, it is
  reported. A comparison harness is named in its pair's form.
- A shared item is accepted only by a reviewed baseline form, under §5's leaves; regenerating the baseline proposes,
  never accepts.
- A site the catalog's rules refuse in a root's or the harness's `.rs` files — an assembler macro, an `extern` block,
  `include!`, `#[path]`, `#[no_mangle]`, a macro definition, a build script, a procedural macro — passes only by a
  reviewed admission naming its file, rule and line (§3; R7).
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
| 4 | 20 | 11 (D1, `M2.7.6.5` with no acceptance, and "per form" given to a code-owner rule; D2, what an unclassified program or an unaccepted form costs carried by no leaf; D3, `M4.8`'s interim rule vacuous with nothing shared; D4, the harness built and inventoried by no rule; D5, `trust-shared-program` with no mechanism; D6, a file a shared package's macro makes both roots include; D7, the build configuration compared nowhere; D8, `M3.6.2` stale and its scan evaded by a renaming `use`; D9, generated sources handed to no leaf; D10, the step off the baseline's host; D11, the step's outcome decided here and not producible by the runner) | "not acceptable as it stands"; §1 re-measured, the normalised configuration stable across directories and toolchains, the harness buildable from the roots' units |
| 5 | 16 | 10 (E1, the assembler scan's files unstated, an `include!` or `#[path]` target unscanned; E2, edges without the dependent, a new consumer unreported; E3, a form held but unaccepted hiding its item, or case 5 never silent; E4, a result's producer unchecked; E5, `M4.8` asking only that programs be classified; E6, runtime data handed but undeclared; E7, acceptance tied to the form's author, not the change's; E8, `M3.6.3`'s step unbuildable by the runner; E9, a refused build's inventory still packaged; E10, "closure grew" beside "role packages grown") | "not acceptable as it stands"; §1 re-measured, the harness's units identical to the roots', every renaming caught, the delegations carried bar four |
| 6 | 15 | 10 (F1, a link-time symbol binding through an `extern` block, unseen; F2, a file one root compiles and another is handed; F3, a claim's subject a declared dependency; F4, a new reader of a shared package's file; F5, the change part measured against forms the commit writes, root forms outside it; F6, the first baseline in the blocked leaf, so case 5 never silent; F7, data tokenised, refusing the generator's kind modules, on round 5's false premise; F8, packaging never failing on `trust-verify`; F9, the report unbound to its build; F10, authorship not an authenticated identity) | "not acceptable as it stands"; §1 re-measured, the harness's units identical, the reused code as the record says, every delegation carried as worded bar one |
| 7 | 18 | 8 (G1, a macro assembling `#[path]` onto a non-`.rs` file, scanned by nothing; G2, a `#[no_mangle]` interposing `memcmp` in the harness; G3, the harness's development edges outside every refusal; G4, a merged sharing without its form leaving later commits to report it; G5, the baseline described as accepted forms only; G6, the off-host outcome read two ways; G7, a first acceptance's window unanchored; G8, the host read from the wrong commit) | "not acceptable as it stands"; §1 re-measured, the subset spared today's roots, the delegations carried bar two |
