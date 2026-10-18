# The trust-dependency inventory and its gate: what each root is built from, and what two roots share

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active` — written, and its review closed `2026-10-05` by `M3.6.1`'s rule: round 11 found no defect;
  rounds 1 to 6 answered `2026-10-03`, rounds 7 to 10 `2026-10-05`, its hand-offs in a ledger since, reviewed beside its instrument since round 8; narrowed after rounds 3 and 4 to what only it decides
- **External sources:** [the pinned Rust toolchain](../../book/src/ledger.md#rust-toolchain) — rustc's dependency
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
checker that compiles them shares them, reported as case 1 (R7 remark j). **A program that would run two roles is
refused** (`trust-shared-program`): a root whose build compiles a role package of another role — a library root too,
which runs its role until an executable does (R8 remark 13) — and the comparison harness compiling one of a role
outside its pair, its exemption being its pair's two, which it compiles by design (R7 remark d; R8 remark 12). The
independence §4.4 asks about is between programs, and one program running both roles has none to disclose; every
package it holds would be shared, and a real case 1 would be lost among them (R2 B5; R4 5). Compiling a role package
counts, whatever the program calls of it: one that needs only another role's types or constants takes them from a
third package, which the gate then reports as shared (R5 remark 15). `ROADMAP.md` §10.2's `archogen analyze`
would make the generator's executable run the scheduling checker; `M2.21` decides how it runs before it is built, and `M3.4` carries the same for `archogen resolve` (R3 C20).

`trust/roots.eadl`, read by the eADL reader as every repository record is, holds one form per root — its role, one
of the five above, the harness being a form of its own (R9 2); its package and target, which fix its artifact; its
role packages, each a package of the commit (R9 remark 5); and its fixed run-time data, a `(data "path" …)` clause of
blobs, hashed and shared as files (§3, §4; R9 remark 6) — and one form per role no root fills yet, naming the leaf that will (the configuration checker, `M3.5`). It also classifies every other
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
its own, its own target directory, and `TMPDIR`/`TMP`/`TEMP` pointing to that target's canonical `tmp`
subdirectory, nothing inherited beyond that — no `RUSTFLAGS`, no inherited `RUSTC_WRAPPER`, no
`CARGO_PROFILE_*`). Inventory compilation additionally sets its own fixed `RUSTC=rustc`, selecting the same
pinned command as the recorded `rustc -vV` identity. The private observer requires that compiler
argument and forwards its native argv; no inherited compiler override is admitted. Unit collection
uses the [production provenance protocol](compiler-provenance-contract.md), with explicit private
settings removed before the real compiler runs. Shared temporary storage is prepared
with checked creation/canonicalization before commands execute. Blocked destinations refuse, while
absent directories are created. The absolute runtime temp path works across child working directories.
These process variables remain refused as compiled environment inputs; process setup grants no admission. It refuses a cargo configuration found
above the repository or under `target/`, or one that is a symbolic link
(`configurations_on_path`), and admits an in-tree configuration only byte for byte as committed and holding
nothing but `[alias]` (`package::check_config`), which closes `build.rustc`, `build.rustc-wrapper`, a linker,
`[env]` and `rustflags` (R1 A6; R3 C18). `CARGO_HOME` is removed and made anew on every run, since `environment` only
creates it and a configuration left in it would reach every later build (R2 B12). Before any build, the packages'
targets are read with `cargo metadata`, which runs no package's code, and a build script or procedural macro is
refused there, as the catalog's `metadata` refuses them, so none runs (R2 B12).
The shared manifest preflight enforces actual metadata target kinds for each reached package before
root/harness or generated-entrypoint compilation, independently of ordinary manifest admissions. A disabled
build script is no dynamic target. Permanent exact-admission script/procedural-macro sentinel controls and
the disabled-script positive case verify this boundary (`flag-normalization-domain.md`, repair `.11.2.5.1`).
Reset removal is checked before metadata, every root/harness build and every separately built generated
target (`trust::reset_cargo_home`, repair `.11.2.6.2`). An absent home is harmless; any other removal error
ends the invocation unable to judge, naming the home and I/O error, before that operation can run. The
environment helper then creates the empty home. Permanent public permission/wrapper and file-state controls
hold this boundary, including writable/absent positive cases and native exact reversal. The earlier gap
and pinned source/caller premises are in `flag-normalization-domain.md` (`.11.2.6.3.1`); local postrepair
integration passes in `.11.2.6.3.2`, with actual-candidate proof controls/E acceptance in `.11.2.4`.

The comparison harness a pair's form
names is built the same way, with `cargo test --release --no-run -p <package> --test <name>`, and recorded as a root
is — its files, its configuration and its artifact — beside the pair's two libraries, in its own target directory,
so the units it shares with them are compiled again with their configuration and recognised by package, crate and
configuration (measured, R4 4; R8 remark 19).

**What it records, for the build as a whole:** the commit and its tree hash; the toolchain (`rustc -vV`,
`cargo -V`); the linker cargo invokes on the host, by `cc --version`'s first line (R6 remark 11); the host triple; and
the sha256 of `Cargo.lock`, the root `Cargo.toml`, `rust-toolchain.toml` and `.cargo/config.toml`. The toolchain's
own `std` and compiler, and the root manifest's profile tables that apply to a root's units — `[profile.release]`
and its overrides for a package some root or the harness compiles, by name or by `*`, never `build-override`, which
applies to build scripts and procedural macros, both refused — read by TOML's meaning (R6 remark 14), are compiled
into every root: they are the **build configuration**, an item every pair shares (§4), recorded by the toolchain's
identity and those tables and never per sysroot file, since the dependency information names no path under the
sysroot. The pin's bytes are in the build's identity, not in it, and the edition reaches rustc only as each unit's
`--edition`, which the unit's configuration holds, so a comment in the pin, a development profile or an override for
a package no root compiles changes no pair's item (R8 6). `Cargo.lock` is recorded and is not
part of it: a package added elsewhere changes it and no root's compilation (case 5) (R1 A14; R4 7).

Historical reader, retired from production by `.15.2.1.3.3.2.1`: Cargo's verbose command was decoded before arguments were identified: single-quoted spans retain literal
backslashes, while a backslash outside them escapes the following character. This preserves Cargo's
close-quote/escaped-apostrophe/reopen-quote path spelling. The public inventory control
`cargo_quoted_output_paths_preserve_inventory_units_and_configuration` exercises the pinned producer with
ordinary, space, apostrophe and literal-backslash output paths and a shared dependency; it is not a general
shell-parser conformance claim.

Producer investigation `M3.6.9.6.3.2.2.10.3.3.1` found that the executable itself is unescaped
([pinned source and evidence](../../book/src/ledger.md#rust-toolchain)). Before the fixed compiler selection,
a space in the toolchain home made collection skip its actual compilation commands; the own-unit guard refused
the measured two-root fixture. `.10.3.3.2` repairs this with the fixed command above. Its isolated Unix public
controls exercise ordinary, space, apostrophe, backslash and combined installation paths, requiring the exact
compiled packages and dependency arguments. Argument quoting evidence does not prove empty-argument absence.

The dependency reader (`M3.6.9.6.3.2.2.10.3.3.3.2.2`, `2026-10-12`) matches rustc's known raw first
.d target before separators, then consumes complete space-escaped scratch/canonical prefixes before splitting
catalog-grammar source suffixes. This preserves literal LF/CR in output targets and absolute in-tree prerequisites;
missing/empty targets refuse. Actual pinned producer controls require independent relative/absolute source sets.
The separate synthetic colon-space target control is not successful Cargo evidence: this host's Cargo refuses
colon output paths when joining its dynamic-library search path. No general Makefile grammar is admitted.

Environment normalization (`M3.6.9.6.3.2.2.10.3.3.3.2.5.2`, `2026-10-12`) encodes known scratch/canonical
prefixes with the [pinned rustc writer's](../../book/src/ledger.md#rust-toolchain) LF/CR/backslash rules before
replacement. Existing encoded record strings, unset spelling, validation and order remain. Actual Cargo controls
require an independently expected relative manifest path and exact description/unset records across ordinary,
line-break, backslash, literal-backslash-n and source-marker roots; a distinct canonical-prefix control passes.
No general record decoder or path/host grammar is admitted.

Historical complete-command framing (`M3.6.9.6.3.2.2.10.3.3.3.2.3`, `2026-10-12`) consumed quoted line breaks
and backticks intact before token decoding. Cargo also prints package paths raw in Compiling/Checking status:
known complete scratch/canonical prefixes there are consumed before physical line breaks, so embedded Running
text cannot become a command. Empty/unclosed framing, unquoted breaks and trailing junk refuse without panics.
`cargo_multiline_output_paths_preserve_complete_inventory` exercises actual public builds, independently
requiring root/dependency units, source/data hashes, environment and retained dependency arguments across
ordinary, line-break, backtick, fake-Running and combined quote/backslash paths; configurations/shared items
remain stable. Synthetic malformed/canonical controls and exact native reversals hold both boundaries.
This repairs the measured public multiline collection defects; full-production adequacy remains separately open.

The retired [source-bound delimiter equivalents](../../book/src/ledger.md#rust-toolchain) (`M3.6.9.6.3.2.2.10.3.3.3.2.4`)
applied only to complete pinned Unix Cargo records with the fixed rustc program and exact outside separators.
Encoded empty operands are included. Compiled actual variants, independent argv expectations and explicit
outside-domain/wrong-oracle falsifiers support the reasons; selected strict native E is not full adequacy.
No production collection behavior changes in this proof slice. After the admitted-dynamic-target repair,
`.11.2.5.2` revalidates root/harness/generated preflight, fixed compiler, modes/environment/configuration and
the unchanged narrow delimiter premises; compiled actual controls and full local integration pass. The later
compiler-program filter cannot supply that premise because decoding precedes it. Flag-source completion and
final whole-production measurements remain separate (`flag-normalization-domain.md`).

Eight lint-level consuming-bit edits are scoped equivalents for the current pinned successful inventory
caller (`M3.6.9.6.3.2.2.11.2.4.1`). Manifest lint flags are joined; excluded extra-argument/environment/
configuration/profile routes and unchanged operand consumers mean the separate keys are not visited.
Permanent compiled actual-edit controls require independent real argv/results and valid wider-domain
cargo-rustc differences, plus wrong-oracle failures, before exact reasons and strict selected native E.
The separate cap-lints consuming-bit proof (`.11.2.4.2`) additionally requires compiled units to be local
path/non-stdlib and show warnings; actual local argv and a successful explicit-cap cargo-rustc counterexample
hold its boundary. The color consuming-bit proof (`.11.2.4.3.2`) binds the pinned formatter and fixed
selector; its successful wider compiler-override control distinguishes raw Cargo requests from actual
forwarded rustc argv, with real artifact consumers and wrong oracles. Plain color/JSON refusal counts as
no successful control. See `flag-normalization-domain.md` for premises/revalidation. Final bounded table replay
`.11.3` binds every original entry: 3 named kills, 10 scoped E, 1 compiler refusal, strict 0 and complete
original/current-copy restoration. Initial outcomes remain historical; the repaired caller supplies current
E premises. Compiler refusal proves no coverage. Milestone integration `.11.4` then passes every current
local tier stage with an exact activation stamp/native Git binding and original/prior-copy restoration;
closure prose has separate checks. The bounded flag group closes; full measurements/review/runner
compatibility remain open. This is no remote CI outcome.

Final bounded helper replay (`M3.6.9.6.3.2.2.10.3.4`) verifies every actual current helper edit with strict
named-kill/scoped-equivalent outcomes and complete original/copy restoration. Original survivor obligations
bind to these corrected-context outcomes; no retroactive equivalence or whole-production adequacy is inferred.
The local helper integration checkpoint (`M3.6.9.6.3.2.2.10.4`) passes every tier step after isolating
process-read permissions; captured inputs restore and the helper group closes. Final full measurements/review
and runner compatibility remain separately owned; this is no remote CI outcome.

Path-boundary controls (`M3.6.9.6.3.2.2.13.2.2`, `2026-10-18`) preserve diagnostic-prefix substrings
inside admitted source arguments: `crates/a/src/kept--json.rs` and
`crates/b/src/kept--error-format.rs` are retained rather than mistaken for diagnostic options.
Dependency arguments split their name/path at the first equals sign; successful ordinary and
`scratch=one` / `scratch=one=two` output builds retain literal `--extern common=libcommon.rlib`
for the binary and `--extern common=libcommon.rmeta` for the library. The public controls
`trust::tests::cargo_source_paths_with_diagnostic_substrings_remain_in_configuration` and
`cargo_dependency_paths_with_equals_preserve_extern_arguments` kill the exact prefix-contains and
last-separator edits in catalogue and systematic native runs. The production helper was already
correct; two parent gaps close. Tracked source names remain subject to §4's path grammar. Initial
outcomes stay historical; the codegen controls below close three further parent gaps.

Codegen operand controls (`M3.6.9.6.3.2.2.13.2.3`, `2026-10-18`) use actual public inventory with
release opt-level/codegen-units 1/3 and a package-b override 2/5. Each literal complete `-C` option
occurs once, and its consumed raw operand is absent as a separate configuration entry. The permanent
`trust::tests::cargo_codegen_arguments_remain_single_normalized_options` parent kills exact
current-token selection, retained-output deletion and one-token advancement edits in both native
harnesses, strict zero. The production helper was correct; these three parent gaps close. Seven
prefix/helper survivors and full production adequacy remain open; initial results stay historical.

**What it records, for each root:**

- the packages its build compiled, from that build's units and not from workspace-wide metadata, each with its
  manifest's sha256 and the kind of every edge that reached it (normal, build), development edges not followed,
  since a test is not the root, but the harness's own (R1 A10, A17; R8 9). A unit's package is the one whose target,
  in `cargo metadata`, names the unit's crate root and crate name — never the directory that prefixes its source,
  which a crate root inside a nested package's directory defeats — and a unit no one target names is refused (R8 2);
- each compilation unit's **configuration**: its whole actual rustc invocation from the private observer record — `--edition`,
  `--cfg` and features, every `-C` option — with every path written relative to the scratch directory, the program path dropped, the `-<hash>` stripped from
  every `--extern` file name, `--out-dir`, `-L` and `-C incremental` dropped, which name the scratch build's own
  directories while each dependency is named by its `--extern` (R8 remark 19; R9 1), and any path still absolute
  after that refused, since it names what the commit does not hold and another checkout spells otherwise (R9 1), and `-C metadata`, `-C extra-filename`, the lint levels (`--warn`, `--allow`, `--deny`,
  `--forbid`, `--cap-lints`), `--check-cfg` and the output-format flags (`--error-format`, `--json`,
  `--diagnostic-width`, `--color`) left out — the first three hash the toolchain, which the build configuration
  holds by identity, the lint levels and `--check-cfg` change only what is warned about, and the format flags only
  how, `--diagnostic-width` differing with the terminal (measured, R4) — and every `# env-dep` line with its value,
  an absolute path written relative — a variable cargo sets from the commit's manifests, `CARGO_PKG_*`,
  `CARGO_CRATE_NAME`, `CARGO_MANIFEST_DIR` or `CARGO_MANIFEST_PATH`, the catalog's own list, and any other refused,
  since its value is not the commit's (R1 A6, A10; R2 B8, B9; R3 C11, C12; R8 remark 22). The root manifest's tables that reach
  rustc — the edition, the profiles — are configuration this way, and its lints are not;
- **every file its compilation read**, from each unit's dependency information, each name — an absolute one with the
  written tree's prefix stripped first — resolved lexically against the workspace root, and refused if it is then
  not a blob of the commit, or is a symbolic link in it; after the build each file read holds its blob's bytes, as
  the catalog's build checker checks (R4 remark 7; R5 remark 14), as a repository path with its sha256 — the package's sources, and every `include_str!`, `include_bytes!` and `#[path]` input — whatever the
  file is, a build-configuration file a unit reads as data included (R1 A6, A9);
- the root's artifact — the executable, the library's `.rlib`, or the harness's test binary — by its path under the
  root's own target directory and its sha256, from cargo's own report of what it wrote (R8 9);
- the runtime data the pipeline hands it, as its form in `trust/roots.eadl` declares, each by path with its sha256:
  none today, and `M2.7.5`'s catalog records for `rt-analysis` once that leaf hands them over (R1 A20). A root's
  **dependencies** — fixed data, the same for every claim — are what its form declares; the **subject** of a claim —
  the description under check, the plan the generator produced — is not a dependency, and is recorded with the
  claim, not in a form (R6 3).

**An input the inventory cannot account for is refused, not trusted** (`trust-undeclared-input`), and on any
refusal the tool writes no inventory, only the report naming it, so `trust-verify` finds none and refuses the package
(R5 9). Before classifying forms, `clear_previous_inventory` removes an earlier JSON file. A missing file is
harmless; any other removal error ends the invocation unable to judge (exit 2), naming the path and I/O error.
An unremovable old file can remain, so correct the I/O problem and retry. Neither forms judgment nor tree,
metadata or build work starts on that failure. Permanent public deletion-denial and directory-at-output
controls, recovery cases and native ignored-error reversal hold this boundary (`.11.2.7`).
**The rules are the catalog's, and the gate is default-deny** (R7): every rule the catalog applies to one
package's own manifest and sources — `check_manifest` and `scan` in `crates/archogen-catalog/src/package.rs`,
unchanged: a build script or a `build` key, a procedural macro or another crate kind, `rustflags`, an optional
dependency, `[features]`, `cargo-features`, `package.workspace`, a target's `path` leaving its package, `asm`,
`global_asm`, `naked_asm`, `include` and its kin, `no_mangle`, `export_name`, `link_section`, the global hooks,
`macro_rules` and `macro`, `link`, `path` and `used` inside an attribute or a macro's arguments, `extern` other than
`extern crate` or an `extern fn`, a non-ASCII identifier, a file that does not tokenize — and its rules on a
workspace manifest, `check_workspace`, over the root manifest every build reads: no `[patch]` or `[replace]`, no
`cargo-features`, no `rustflags` (R8 remark 16). They apply to every `.rs` file a root's compilation reads, however
it reads it, and to the comparison harness's closure with its development edges followed, since the harness is built
from them (R7 3). The catalog's rules for a package lifted out of its workspace do not apply to a root built in place,
and are not adopted: `packages_below`, since a unit is attributed by its target, not its directory (R8 2); a
dependency written `workspace = true`, since the inventory follows the resolved graph; and where toolchain files may
stand, since the build sets `RUSTUP_TOOLCHAIN` to the root's pin, which overrides every file. Cargo itself refuses a
`links` key without a build script, which is refused (measured, R8 remark 16). **A crate root that is not a `.rs`
file** — a target's `path` naming another extension, which rustc compiles as Rust and no token rule reads — is
refused with the manifest rules, before any build; `mod` finds only `.rs` files and `#[path]` and `include!` are
refused, so every Rust source a root compiles is then a `.rs` file (R8 1). A name in a unit's dependency information
that is not a blob of the commit, or is a symbolic link, is refused. A site the rules refuse passes only when an
**admission** in `trust/roots.eadl` names it — the file, the rule, and the sha256 of the site's **extent**, or, for a
manifest rule, which has none, of the whole manifest, naming the first rule the catalog reports, which the
manifest's review reads past (R10 2) — reviewed as every form is (§5). The extent is the statement or
item the site stands in: at the innermost brace level holding it — the file, a module, a block, a brace-delimited
macro's body — from the token after the previous `;` or brace group at that level through the next `;` or the end of
the next brace group, so parentheses and brackets never end it, nor a brace group between `<` or `,` and `>` or `,`,
which is a const generic argument, `A<{ 1 }>` (R9 3), a macro invocation is whole within it, and an attribute runs
with the item it marks through the item's body; an `.incbin` added on an admitted `global_asm!`'s
second line, or an admitted `#[no_mangle]` function renamed `memcmp`, changes it (R8 4). **An admission admits one
site**: sites with one file, rule and extent pass only as many as the admissions naming them, so a second
`#[no_mangle]` beside an admitted one is a new site (R8 3); a site two roots compile is one site (R8 remark 20). An
`include!` or a `path = "…"` compiles the file it names as Rust, so its admission holds only when it names a `.rs`
file in one string literal, which the rules then read; one naming another file, or written otherwise, is refused
whatever admits it, and every Rust source a root compiles stays a `.rs` file (R9 remark 7). A site whose extent
renames its refused identifier with `use … as`, or defines a macro whose body holds a refused construct, is
inadmissible: the alias's or wrapper's invocations name nothing a rule refuses, so no extent would pin them (R10 1). An
edit anywhere in a site's extent or manifest, or a new site, is refused until a new admission is, and an admission no
site uses is listed with the inventory and refused by the gate as `trust-baseline-stale` on the baseline's host (§6;
R8 remark 15). A manifest rule is applied over each program's closure from `cargo metadata` before any build, so a
build script or a procedural macro is refused before it runs (R2 B12). Measured `2026-10-05` by `cargo xtask trust-inventory`
(`M3.6.2`) over the 55 `.rs` files today's roots and harness compile, the rules refuse 14 sites, each proposed as an
admission: one manifest rule, the wasm module's `crate-type`, which builds the module a browser loads
(`crates/archogen-wasm/Cargo.toml`, `ROADMAP.md` §10.4); eight `include_str!` of data — the two kind modules
(`crates/archogen-api/src/lib.rs`), the profile page (`crates/eadl-model/src/profile.rs`), the roadmap
(`crates/archogen-cli/src/spec.rs`) and four of S0's runtime templates (`crates/archogen-s0/src/emit.rs`); three
`#[no_mangle]` on the wasm module's exports (`crates/archogen-wasm/src/lib.rs`); and two uses of the word `path` in an
ordinary macro call (`crates/eadl-front/src/module.rs`, `crates/rt-analysis/src/cost.rs`). Everything else is
refused. Re-measured by extent after round 8: the same 14 sites, all admitted; 8 pairs, 9 shared items. One rule set so closes the channels five rounds found one at a time — an `.incbin` under the assembler
macros, its bytes in the rlib and in no `.d` (R2 B3; R3 C1), a renaming `use` (R4 8), a link-time binding through an
`extern` block (R6 1), a macro assembling `#[path]` onto a file that is not `.rs` (R7 1), a `#[no_mangle]` interposing
a symbol in the harness, measured taking over `memcmp` (R7 2), a procedural macro on a development edge (R7 3) — and
refuses the next such channel before anyone finds it. A file other than a `.rs` file read through an admitted `include_str!` or
`include_bytes!` is data, hashed and never tokenised (R6 7); a `.rs` file is Rust however it is read, so S0's runtime
templates, compiled as modules and read as text, are tokenised (R8 10). A channel the catalog's rules do not refuse is
outside this coverage until they do, and a fixture of the instrument is where one is shown (R8 remark 17). What remains outside the inventory, and is stated in its report: the linker and the host's C
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
  today `rt-core`'s test target `differential`, which holds the adapter between them — every unit it compiles beside
  the pair's two roots' builds, its test target and the adapters its development dependencies add, with their files
  and configurations (R8 7), built as §3 says, its edge to the reference library of kind `development` (R2 B7; R4 4;
  R6 remark 12); and the harness is paired with every root other than its pair's two as well, whose sharing with it is this item
  (R8 8), as the program that
  produces the reference model's results (`M4.7`), so a package its development dependencies add is shared with any
  other root that compiles it (R5 remark 11);
- for every pair, the **build configuration** of §3: the toolchain's identity and the root manifest's tables that
  reach rustc (R4 7).

An item's **content** is the sha256 of its manifest and of every file it contributes to each side. Its
**configuration** holds each side's unit configuration, `# env-dep` lines and their values included, so a value
compiled in through `env!` changes the item as a flag would (§3, R8 5). Clarified `2026-10-11` by
`M3.6.9.6.3.2.2.9.2`: each unit's comparison key is deterministic JSON of two ordered string arrays,
`[command_tokens, environment_records]`. This preserves token/record boundaries, empty values, order and
escaped characters even when a value contains the former separator. Per-program raw configuration/env arrays
retain their shape. Changed key encodings change configuration digests; proposals are regenerated on their own
host and remain subject to §5/§6's review. Its **edges** are the set of (root, dependent package,
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
| `TI-H3` | `M3.6.3` | This leaf commits, unaccepted, the proposed forms of today's shared items, the baseline's host and the classifications of today's non-root program targets, so case 5's change part is empty before any acceptance exists. |
| `TI-H4` | `M3.6.3` | `trust-form-missing` refuses, on the baseline's host, a current shared item, root or program target with no form in the commit's own `trust/`, `trust-baseline-stale` refuses there an admission no current site uses, and an unrelated commit after a merged sharing reports "unchanged". |
| `TI-H5` | `M3.6.3` | Off the baseline's host the gate applies every refusal but `trust-baseline-stale` and `trust-form-missing`, reports every shared item unreviewed, and says "not compared" in its change part. |
| `TI-H6` | `M3.6.3` | The gate's program is built from the base commit, as the catalog's checker is. |
| `TI-H7` | `M3.6.3` | The `assurance` tier's `trust-inventory` step is a runner action that runs the gate, `Failed` on a refusal and otherwise not built for the acceptance it lacks, owned by `M3.6.5`, so never Passed before it whatever is shared. |
| `TI-H8` | `M3.6.3` | `cargo xtask trust-verify` refuses a package whose inventory is missing, stale or of another build, whose report is of another inventory, in which a result names a program other than its role's inventoried artifact or the pair's harness, or which records a dependency handed to a root that its form does not declare by path and sha256, each tested on package directories made for the purpose. |
| `TI-H9` | `M3.6.3` | F30's five cases are built in scratch workspaces, each asserting the code and outcome the record's §6 gives it, case 5 reporting "unchanged" over a change outside every root's units — a README, a development profile, an override for a package no root compiles, a comment in the pin — and a mutation matrix removes each refusal in turn. |
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
| `TI-H23` | `M3.6.2` | `cargo xtask trust-inventory` applies the catalog's package and workspace rules unchanged over every `.rs` file a root or the harness compiles, however read, following the harness's development edges, refuses a crate root that is not a `.rs` file and the manifest rules before any build, attributes each unit by the target naming its crate root, passes a refused site only by an admission of its own naming its file, rule and extent's sha256, records each root's artifact, packages and declared run-time data, writes no inventory on any refusal, and commits `trust/roots.eadl`'s root forms and the admissions of today's 14 refused sites, unaccepted. |
| `TI-H24` | `M3.6.2` | Every channel a review measured by hand is a fixture of the instrument's tests — an `.incbin` under each assembler macro, a renaming `use`, a macro-made include, a macro assembling `#[path]`, an `extern` block calling another package's `#[no_mangle]` function, a `#[no_mangle]` interposing `memcmp` in the harness, a procedural macro on a development edge, a new consumer edge, a new reader of a shared package's file, a profile edit, per-root against workspace-wide features, two checkout directories and two toolchains, a crate root that is not `.rs`, a crate root in a nested package's directory, two sites of one text, a multi-line site, a value compiled in through `env!`, an unrelated profile edit, the harness's adapter, and a harness or library root compiling another role's package. |

Until they land nothing is accepted, and every shared item is reported unreviewed (R2 B2; R3 C4, C5).

### 6. The gate

When the inventory refuses, the gate removes any earlier `baseline-proposal.eadl` before writing the
refusal's `report.txt`. Only successful deletion or a missing proposal permits continuation. Removal
failure returns an attributed filesystem error (exit 2), naming the path, before replacing the report;
old proposal/report contents may remain and must be treated as earlier output. Fix the I/O problem and
retry. No deletion restriction is overridden and a directory's contents are preserved. Public controls
in `M3.6.9.6.3.2.2.11.2.8` cover a readable protected proposal with writable sibling output (macOS), a
directory at the proposal path (portable), refusal retry and later successful proposal generation.

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
| `trust-baseline-stale` | refused, on the baseline's host | a baseline form whose item is no longer shared, a root form or classification whose package or program target is gone (R7 remark e), or an admission no current site uses (R8 remark 15): removed, so a sharing removed and reintroduced, or a target re-added under an old name, is reviewed again (R1 A15; R3 C9; R5 remark 12) | — |
| `trust-undeclared-input` | refused | a name not a blob of the commit, a symbolic link, a file whose bytes after the build are not its blob's, a crate root that is not a `.rs` file, a unit no one target names, an environment variable outside cargo's own, a path left absolute in a unit's configuration, declared run-time data that is not a blob, an admitted `include!` or `path` naming a file that is not `.rs`, an alias or wrapper of a refused construct, or a site the catalog's rules refuse, in any `.rs` file a root or the harness compiles, with no admission of its own (§3) | 3 |
| `trust-form-missing` | refused, on the baseline's host | a current shared item, root or program target with no form, proposed or accepted, in the commit's own `trust/` — a refused site with no admission is `trust-undeclared-input`'s alone (R8 remark 14) — repaired by the tool's proposal, committed, so the base commit's forms always cover its inventory and case 5's change part is the commit's own (R7 4) | — |
| `trust-shared-program` | refused | a root whose build compiles a role package of another role, an executable or a library alike, or a harness compiling one of a role outside its pair (§2; R8 remarks 12, 13) | — |
| `trust-inventory-stale` | refused, at packaging | the inventory missing or not valid UTF-8 JSON, its build identity not the package's commit and toolchain, an artifact's sha256 not the inventory's, the report not of that inventory, a result naming a program other than its role's inventoried artifact or the pair's harness, or a dependency handed to a root that its form does not declare by path and the inventory's sha256 | 4 |

**Clarified `2026-10-06`, by the gate's leaf (`M3.6.3.3`).** A root form or classification naming a package the commit
no longer has, or a root naming a target its package no longer has, is `trust-baseline-stale` wherever the gate runs,
not on the baseline's host alone: no inventory can be built without what the form names, so off that host there would
be nothing else to report. *Corrected the same day, by `M3.6.3.7`:* this paragraph first said a `trust-unclassified-program` was reported in the
standing list, against the opening, which puts it in the change part; the change part holds, as the opening says,
every root form added, changed or removed against the base commit's `trust/roots.eadl` — compared by what each form
says, so a comment is no change — and every program target not classified or whose role packages grew, on any host. And,
by F30's second case (`M3.6.3.6`): a form proposes its item's digests, so on the baseline's host a commit whose own form
for a shared item proposes other digests than the commit measures holds no form for the item as it is, and is
`trust-form-missing` until the tool's proposal is committed; otherwise a change to a shared item, merged once, would be
reported by every later commit, which R7 4 forbids for a new sharing alike.

**Clarified `2026-10-10`, by `M3.6.6.2`, the instrument of
[`decision_trust-generated-sources.md`](decision_trust-generated-sources.md)** (its `GS-H8`). That record decides what
committed generated sources add to this one, and each passage its §7 lists reads with it:
- the opening's "a change outside every root and every program target" reads "outside every root, program target,
  provenance file, unit a live form's generator build compiles and form of `trust/`"; the change part, the standing list,
  `TI-H2`'s standing list and the clarification of `2026-10-06` above take every `defgenerated` form — added, changed or
  removed against the base commit's in the change part, named in the standing list until accepted;
- §2's forms of `trust/roots.eadl` gain `defgenerated`; `trust-shared-program` gains a generator file of another role
  than the reading program's — for the harness, than its pair's two — a program target's crate root judged by its role
  packages, the union of them for a crate root of two targets, and a script by the innermost workspace member holding it;
- §3's "Where it builds", its default-deny scope and "a manifest rule is applied over each program's closure" extend to
  each program target a live form's generator file roots, built and checked as a root is, unless it is a root, whose
  build it is; §3's sentences on admissions count such a build's sites and uses with the programs', one admission a
  site, a site both compile one site;
- §3's paragraph on generated sources is answered — a committed generated source declares its generator and inputs, and
  one a program reads whose header marks it generated and that no form declares is refused — and what is recorded for
  each program gains its provenance;
- §4's shared items gain the **generated provenance**, by path and by content, whose aspects are the content's sha256,
  each side's roles and each side's declared files with theirs; §5's form proposes those digests as it does any item's;
- §6's table: `trust-new-shared` counts a new generated provenance, and `trust-shared-changed` one whose content, roles
  or declared files move; `trust-undeclared-input` gains a `defgenerated` form departing from its shape or declaring a
  path another declares, a live form's generator or input that is no blob or is itself generated, a `.rs` generator file
  that is no program target's crate root, a marked file a program reads that no form declares, and a declared or marked
  file a generator build reads; `trust-shared-program`, a generator file of another role; `trust-baseline-stale`, on the
  baseline's host, a `defgenerated` form whose file no program reads; and case 5 is "unchanged" outside the provenance
  files and the generator builds' units too;
- `TI-H9`'s "a change outside every root's units" reads "outside every root's units, every provenance file and every
  unit a live form's generator build compiles".

Its sentence that `Cargo.lock` is recorded and is not part of the build configuration stands, unamended.

*Completed `2026-10-10`, by `M3.6.6.3`, its review's round 1, D2:* §4's comparison-harness item gains the harness's
provenance as an aspect, as its files are, and §6's `trust-shared-changed` one whose provenance moves — passages the
record's §7 list omitted, corrected there the same day.

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
toolchain bump or an edit to a profile table that applies to a root's units changes the build configuration, every
pair's item, reported for review, while a development profile, an override for a package no root compiles or a
comment in the pin changes none (§3, §4; R4 7; R8 6). The
gate's tests build the five cases in scratch workspaces, as the catalog's build checker's tests do, each asserting its
code and nothing else.

*Clarified `2026-10-10`, by `M3.6.7`, reading the code beside this text:* this record's sentences that a change outside
the roots reports "unchanged" — the opening's "for a change outside every root and every program target", this
paragraph's "a change outside every root's packages and files", and the case-5 bullet of the clarification of
`2026-10-10` above — hold on the baseline's host alone, where any change off it reads "not compared" (§2), and for a
change to nothing the gate reads beyond the roots. It reads more: the pin, whose channel is in the build
configuration and which admits a few keys, any other leaving the gate unable to judge; every cargo configuration on
the build's path, a key other than an alias refused; each member's manifest, a program target it adds unclassified;
the commit's tree, a symbolic link or submodule entry under `catalog/` refused; and the comparison harness's units
beside its pair's builds, the comparison-harness item's. A change to any of these is judged as §6 says, and reads
"unchanged" only when the change part names nothing. Case 5's examples above stand.

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

Mutation preflight `M3.6.9.6.3.2.2.14.1` (`2026-10-18`) binds the complete `run_cargo` operator
region and observes actual environment-omission edits with a write-free compiled child. Production
setup remains unchanged. An installed-host, parameterized OS write guard and real parent baseline
precede mutation writers; [the book records the reproducible boundary and its limits](../../book/src/annex-trust-proofs.md#run_cargo-locality-preflight).
No current mutant classification follows from that preflight.

The following complete guarded measurement `.14.2` judges all twelve current edits: nine named kills
and three diagnostic matcher survivors, strict exit one. The end-elements edit now drops target
isolation: a successful mutated inventory omits common from the second root after Cargo reuses its
artifact. An independent omission-only public Cargo intervention confirms fresh/no-command behavior
while all home/temp settings remain local. Original production retains isolated targets; the existing
shared-package parent catches this altered behavior. The book records full results and restoration;
matcher assessment `.14.3.1` is recorded below, with no cohort or full-production adequacy claim.


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
  reviewed admission of its own naming its file, rule and the sha256 of its extent, an `include!` or `#[path]` only
  onto a `.rs` file (§3; R7; R8 3, 4; R9 4, remark 7).
- Related: [[decision_zero-dependency-engine-core]], [[decision_findings-for-director-review]] (§11),
  [[decision_catalog-records]].

## Review

`M3.6.1`'s acceptance is a review, by a context that did not write this record, finding that the gate as specified
exercises each of §14.4's five cases, that no input a root's build reads can be shared without being reported, and
that the baseline cannot be accepted by its author; every finding answered here. The history is
[`decision_trust-inventory-reviews.md`](../../reviews/decision_trust-inventory-reviews.md).

Defects per round, oldest first: 11, 12, 16, 11, 10, 10, 8, 11, 4, 1, 0. Each round's findings, its reader's measurements and every answer are in
the review history linked above, which holds them whole; this record keeps only the count (`PROGRAM.52.2`, the
folder's ceiling).


## Cargo failure diagnostic assessment

Assessment `M3.6.9.6.3.2.2.14.3.1` (`2026-10-18`) uses the supported multiline scratch-prefix
contract and actual pinned Cargo, retaining the admitted alias configuration. Successful controls
pass. An ordinary type error supplies a literal independent diagnostic and structured E0308; exact
compiled original/three selector edits differ as expected. These are witnesses, not fresh native kills.

The original helper has a confirmed framing defect: a physical path continuation beginning with
`error` can replace the actual diagnostic in its failure text. A standalone helper control with
`error: decoy` and an ordinary separate target also reproduces it. This is not a successful inventory
artifact-layout witness: colon artifacts cause a Unix Cargo library-path setup refusal. The following repair
`.14.3.2.1` reproduces the issue through public inventory and adds permanent regressions before
remaining selector rebinding/classification. Source/controls/reasons are unchanged
by the assessment. The [book](../../book/src/annex-trust-proofs.md#diagnostic-selector-assessment)
records producer evidence and limitations; at that assessment, the last complete native result was nine K/three S.


## Cargo failure framing repair

`M3.6.9.6.3.2.2.14.3.2.1` (`2026-10-18`) excludes physical error candidates inside known workspace/
artifact path spans, including ancestors, canonical aliases and Unix quote spellings. Public inventory
compiler failures preserve both roots’ literal messages, no-inventory state and recovery. Operational
colon-artifact refusals retain the actual Cargo path-joining reason despite admitted dual-config
warnings. The initial unsupported colon-positive baseline is explicitly excluded; the earlier separate-
target helper witness is qualified. Setup/compiler stores are local; targeted Cargo experiments and reversal runs are guarded.

The final parent group passes; the native catalogue reversal kills the exact old selector with every
new parent and restores the file. All earlier code/tests reconstruct exactly, the catalogue appends one
entry and prior E records stay exact. Final focused/staged/book gates bind publication. Current global
listing1526 and helper region51 were planning only at the repair commit; three old raw selector rows are textually retired,
not converted to E. Fresh binding/measurement `.14.3.2.2.1` follows. The
[book](../../book/src/annex-trust-proofs.md#cargo-failure-framing-repair) records behavior and scope.


## Complete repaired diagnostic-helper assessment

`M3.6.9.6.3.2.2.14.3.2.2.1` (`2026-10-18`) measures all 51 current helper/invocation edits through
the maintained native CLI under the revalidated installed-host write guard: 38 finishing named kills,
nine survivors and four E0599 compiler refusals, strict exit 1. No E, timeout or unjudged outcome.
The 1526-entry global listing is a scope census only. Eleven initial obligations bind through the
prior twelve; nine prior invocation edits remain and are killed, three raw selector rows are retired
by production repair, and all 42 new helper candidates are attempted. Retirement is not equivalence.

All 1047 original/current copied regular inputs restore; seven false receipts reject. Code, tests,
catalogue, native producer, previous reasons and opaque submodule identity stay exact. The nine
survivors are scheduled under `.14.3.2.2.2`, finite-iterator proofs first, then evidence-directed
assessment of the remaining seven. The
[complete book table](../../book/src/annex-trust-proofs.md#complete-repaired-cargo-diagnostic-measurement)
records each actual outcome and a finishing parent or refusal reason; no whole-file adequacy is claimed.


## Finite diagnostic-path iterator equivalences

`M3.6.9.6.3.2.2.14.3.2.2.2.1` (`2026-10-18`) proves the actual end-elements edits over
cargo_error_line's owned path vector and raw/canonical spelling chain. The complete vector constructor
has cwd plus at most one last-selected target. The chain is once(path) plus a precomputed
Option<PathBuf> from canonicalization. Both have one or two positions; every position is an end.
The counter cannot overflow, peeking moves concrete paths without user/FS callbacks, and the
unchanged loop work observes the same values and order.

Permanent controls bind both exact constructors and compile the original/actual edits with literal
helper outputs and ordered traces. Successful wider Vec-backed constructors lose a middle value;
wrong positive/negative/wider oracles fail, and widened source-premise controls refuse. Add only the
two scoped reasons after proof. Native2 E/strict0 and the full33 operator controls pass; all1051
original/current copied inputs restore and seven altered receipts reject. Existing source/test/reason
prefixes stay exact. The other seven current survivors remain owned in .14.3.2.2.2.2; no broad adequacy.
The [book proof](../../book/src/annex-trust-proofs.md#finite-cargo-path-iterator-proofs) records examples
and revalidation triggers: changed constructors, types, iterator or body require a fresh judgment.


## Relative public gate caller defect

`M3.6.9.6.3.2.2.14.3.2.2.2.2.1` (`2026-10-18`) confirms a P2 original CLI defect before
classifying the remaining seven helper survivors. On the same main commit, absolute --repo produces
an inventory/exit0 (other-host baseline comparison explicitly skipped), while --repo . falsely calls
the own .cargo/config.toml an above-repository configuration, exits1 and writes no inventory.

Root: relative repo/output spellings pass unchanged into lexical configuration-root comparisons.
The exact compiled ownership closure and OS same-file/byte identity distinguish spelling failure
from a foreign or edited configuration. Held1051 regular inputs restore; an early intentional task
finding edit was correctly refused, then checked in its held epoch and reapplied exactly. Code,
controls and E reasons remain unchanged. Immediate repair/regression owner is .14.3.2.2.2.2.2; survivor
classification waits. The [book assessment](../../book/src/annex-trust-proofs.md#relative-gate-caller-assessment)
records commands, actual outcomes and the temporary absolute-path invocation.


## Repository-path caller repair

`M3.6.9.6.3.2.2.14.3.2.2.2.2.2.1` (`2026-10-18`) resolves the runtime repository before deriving
output/configuration roots. Relative and aliased inputs match absolute ones; absolute output selection
and missing/non-directory errors retain their contracts. The public wrapper still authorizes only its
selected repository. The private fixture command worker receives only already validated test roots;
the production wrapper passes none, held by a strict refusal parent.

Permanent command controls go from2 pass/1 fail to3 pass/0 fail, retain expected identities/programs,
and compare the same physical output. The exact maintained defect reversal is killed/restored.
Actual frozen-source public absolute/relative/alias calls produce byte-identical inventories; invalid
repository calls return2/no output. Other-host baseline comparison remains skipped and qualified.
A rejected different-directory byte oracle is explained by embedded source paths in rlib metadata,
with originals retained. All1051 held public inputs restore and older source/controls/reasons preserve.
Final exact-stage focused pass/stamp is required before publication; the full integration/caller
checkpoint remains .14.3.2.2.2.2.2.2, before the seven remaining judgments. The
[book repair](../../book/src/annex-trust-proofs.md#repository-path-caller-repair) records behavior and scope.


## Repaired caller integration closure

`M3.6.9.6.3.2.2.14.3.2.2.2.2.2.2.2` (`2026-10-18`) closes the caller repair after a complete
local integration12/0, no unavailable/not-built/quarantined steps, with native/official activation stamp
180b081792ea1770b6ca8e79efcf11487e97cb21. All1055 held regular inputs restore; five false bindings refuse.
Completion documents receive separate pinned-book/staged checks in a later epoch. No Rust, operator,
catalogue or reason change occurs in this checkpoint.

Current source/pinned/tool metadata bind unchanged environment/allowlist,28 source identities,33
operator controls and exact historical59/14/51/2 edits. The repaired default caller maps relative and
aliased inputs to runtime absolute geometry without new outer-root authorization. Historical results
are not new measurements, and the full pass establishes no whole-file E validity or mutation adequacy.
Both caller false refusal and stale timing rationale are resolved; the seven remaining helper judgments
resume. The [book checkpoint](../../book/src/annex-trust-proofs.md#repaired-caller-integration-checkpoint)
records every stage and the local/host-comparison/physical-board limits.


## Seven remaining helper domains

Assessment `M3.6.9.6.3.2.2.14.3.2.2.2.2.3` (`2026-10-18`) binds exact current helper edits
and the unchanged environment. Actual guarded Cargo witnesses distinguish target-key inequality,
last-path-only processing and removal of the span's lower bound. The first two independently retain
E0308 and the literal compiler failure; the third is a direct failed-command manifest control,
not a public inventory fixture. No fresh native mutation outcome is claimed.

Relative wider controls distinguish the unconditional-prefix and strict-begin edits; their finite
absolute-path agreement only schedules universal caller/prefix proofs. Constructed shorter-literal
and closed-upper-bound differences require producer evidence rather than an absolute-path assumption.
Immediate owned .4 adds three permanent regression parents; .5 proves the absolute-prefix proposals;
.6 resolves both producer-dependent boundaries before complete .14.4 replay. All1055 held inputs
restore and four false bindings refuse; production/operator/catalogue/reasons stay unchanged.
The [book assessment](../../book/src/annex-trust-proofs.md#seven-diagnostic-helper-obligations-domain-assessment)
records every edit and its evidence boundary.


## Three Cargo diagnostic regression parents

`M3.6.9.6.3.2.2.14.3.2.2.2.2.4` (`2026-10-18`) adds isolated actual-Cargo parents for
independent raw target alias, separate cwd/configuration warning and error-before-multiline-path.
Successful controls/recovery pass; compiler cases independently assert E0308 and literal diagnostics.
The leading manifest control exercises failed Cargo directly, not public inventory admission.

The unmodified three-parent baseline passes; native exact replay reports3 K/strict0, each with its
intended finishing parent. All1056 original/current copied regular inputs restore and seven falsified
receipts refuse. Exact reconstruction preserves every earlier source byte after removing only the
new test block; production/catalogue/operator/reasons remain unchanged. Four obligations remain:
two absolute-prefix proofs and two producer boundaries before complete .14.4 replay. Final focused/
staged gates and the [book](../../book/src/annex-trust-proofs.md#real-cargo-regression-parents-and-exact-replay)
bind publication; no fresh whole-file adequacy follows.


## Absolute Cargo diagnostic-prefix proof

Proof leaf `M3.6.9.6.3.2.2.14.3.2.2.2.2.5` resolves only the unconditional prefix insertion
`L690.cond.244` and strict span-begin `L717.cmp.257` proposals. Its domain is the supported Unix
Cargo-built xtask callers with absolute cwd, any supplied environment values and any stderr string.
Relative cwd is outside this proof; permanent wider controls distinguish both edits there.

Caller premise: the inventory/baseline CLI uses the parent of Cargo's absolute manifest directory;
the gate resolves its selected repository before joining output. Inventory derives cwd as out/tree,
and compile is the sole production run_cargo caller. Joining an absolute cwd with either a relative
target or retaining an absolute target supplies absolute raw paths. Successful canonicalization also
supplies absolute paths; failure only omits that spelling. Unix ancestors of an absolute path retain
the leading slash. Lossy UTF-8 conversion and apostrophe quoting preserve that slash.

The pinned Cargo source at f2d3ce0bd7f24a49f8f72d9000448f8838c4e850 supplies the build premise:
[workspace construction](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/workspace.rs)
rejects a relative manifest path; [package roots](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/package.rs)
derive from that manifest's parent, and the already held compilation.rs exports pkg.root() as
CARGO_MANIFEST_DIR. The supported build uses a normal workspace, not the ephemeral install/package
constructor. Native Cargo also compiles the actual repository-root function through a relative
manifest argument and directory alias, requiring its absolute result and physical identity.

A candidate physical line starts at byte zero or immediately after newline, and its first bytes
are error. For an added prefix containing no newline, its match cannot straddle such a line start:
straddling would include the preceding newline. At equality with the match begin, the slash and
error disagree. Therefore adding those no-newline prefixes cannot suppress any candidate, and the
unconditional insertion edit preserves every selected result, including empty stderr.

The strict-begin edit changes span membership only when begin equals the candidate start. Every
recorded prefix begins with slash, whereas that candidate begins with error. They cannot occupy
the same byte position. Thus any original containing span already has begin strictly before start;
the edited predicate has the same value. All other work, ordering and returned bytes remain exact.

Permanent test test_cargo_absolute_prefix_edits_preserve_line_selection binds the complete current
trust/gate production slices and repository-root/CLI entry functions. These deliberately broad
premise guards require proof revalidation after any change in those guarded slices. Three altered
source/caller premises refuse. Actual original/two edited helpers compile; independent literal
records, aliases, missing/relative targets, both line endings and4681 generated strings support the
universal argument. Two relative wider witnesses and three wrong oracles fail as required. These
finite examples do not define the proof's universe. The operator test requires separate precise
reasons only after reaching those controls; its first pre-reason run refuses at that final guard.

The two additional pinned source identities are package.rs c69d85227b154d36e828acdbc3705ef921a4919775dd2a9e1c0eed912287cff7
and workspace.rs fa321dfbd2857f42b4a1dd37054dd749a860f664baa89fc55e76978da32abbd2.
Their complete copies and system-hash bindings are retained under target/cargo-absolute-prefix-proofs.
They supplement, not replace, the earlier28 pinned sources. Revalidate after caller/root/constructor,
prefix conversion, line iteration, predicate or toolchain/platform contract changes. No arbitrary
relative-cwd equivalence or judgment on the final two producer-dependent edits follows.


Proof verification: the full operator suite passes34 controls, and the selected guarded native replay
reports2 E/strict0 with original baseline passing. Only these2 reason records are appended; prior
reasons/source/tests remain byte-exact. All1058 original/current copied regular inputs restore;
six falsified bindings refuse. The copy borrows index metadata only. Native selected E does not
establish unrelated reason validity. Two producer-dependent obligations remain before .14.4.
The [book proof](../../book/src/annex-trust-proofs.md#absolute-diagnostic-prefix-proofs)
carries the argument and boundary examples.


## Final diagnostic-producer boundaries

Assessment `M3.6.9.6.3.2.2.14.3.2.2.2.2.6.1` (`2026-10-18`) separates the shorter-literal
helper witness from inventory reachability. The maintained parser refuses both quoted multiline-key
fixtures before program compilation; admissions cannot override parse failure. Initial warning-data
misselection is an outside-dialect control, not an admitted public defect or claimed repair.
The direct command helper's literal-prefix behavior gets a permanent parent with this explicit limit.

An ordinary-manifest/configuration, inventory-shaped colon layout proves the closed upper bound
can suppress a genuine Cargo path-joining error. A known ancestor text matches a containing
configuration warning's closing backtick/newline; its end is exactly the following error's start.
Original retains the error. Permanent inventory reproduction belongs to next .6.2. Two lexical-alias
hypotheses are refuted by actual path spellings and equal original/edited results.

Five guarded cases, two compiled syntax refusals, exact helper/environment and raw/structured
output bind the assessment. All1058 inputs restore, five false bindings refuse, code/reasons remain
unchanged and all30 prior pins match the read-only existing-commit source cache. No new native K/E,
public quoted-key failure, original repair or complete producer-grammar claim. The
[book](../../book/src/annex-trust-proofs.md#remaining-diagnostic-producer-assessment)
retains cases and limits before permanent parents and final .14.4 replay.


## Final diagnostic regression parents

`M3.6.9.6.3.2.2.14.3.2.2.2.2.6.2` (`2026-10-18`) adds actual bounded Cargo parents for
the final literal/upper-bound edits. The direct helper case also requires inventory syntax refusal,
no program target/inventory and ordinary recovery. The actual inventory boundary case requires
ordinary successes, distinct file/directory identity, genuine operational errors for both roots,
no inventory and recovery, using only the existing authorized fixture roots.

Original parents pass; selected guarded native replay reports2 K/strict0 with intended finishing
names. All1060 original/current copied regular inputs restore and seven false bindings refuse.
Only two test functions are added; earlier source/tests/operators/catalogue/reasons remain exact.
Final held focused/staged/book gates bind publication. Individual nine-survivor resolution is
complete, but final current-region measurement .14.4.1 and full integration .14.4.2 remain before
bounded cohort closure. The [book](../../book/src/annex-trust-proofs.md#final-diagnostic-regression-parents)
records behavior and limits; no new original repair or whole-file E/adequacy claim.


## Complete current diagnostic-region replay

`M3.6.9.6.3.2.2.14.4.1` (`2026-10-18`) actually reruns every current helper/run_cargo edit:
43 K/4 E/4 U, strict0, no survivor/timeout/unjudged. Full controlling parent arguments select153 tests
in both original and restored copied trees; no reduced finishing-only population. All51 identities/
raw edits bind current069d8e598fd9d6c12306fbe0f091635b60f942b747e78e6167e92b41e0ca01ba source.

The four selected reasons retain their exact proofs; all five new intended finishing names remain
actual kills. Four E0599 Cow first/last compiler refusals establish no coverage. All1061 original/
current copied regular inputs restore, including modes/Python-system hashes; ten false receipts
refuse. Copied index metadata is borrowed and the gitlink remains opaque. All30 pinned source
copies and tool metadata revalidate; metadata is not executable-digest evidence. Native held tree
6306e488b8c4e06ce5fc1b39acb53c74586b3028 is distinct from the later completion-doc epoch.

The [book](../../book/src/annex-trust-proofs.md#complete-diagnostic-region-replay) publishes every row,
operator totals, full command and limits. Only four applicable E reasons are judged, not the
whole26-record file. Full integration/cohort closure .14.4.2 remains before the complete trust.rs
sweep. This is fresh bounded evidence, not a projected sum, remote CI or whole-file adequacy.


## Diagnostic cohort integration closure

`M3.6.9.6.3.2.2.14.4.2` (`2026-10-18`) binds final current source to the actual complete51 native
43K/4E/4U strict0 result/full153 parents, then passes actual local integration12/0 with no unavailable,
not-built or quarantined step. Native/official held activation 5fc854ec08a322af168a2615934a988a6e9e1b47 agrees;
all1062 regular inputs restore with modes/Python-system hashes and one opaque gitlink identity.
Five false bindings refuse. All30 pins,34 operator controls and tool metadata bind; production,
caller/environment/allowlist, catalogue and reasons remain exact. Completion prose is a later
separately gated epoch. No unrelated E validity, whole-file adequacy, remote CI or physical-board claim.

The [book checkpoint](../../book/src/annex-trust-proofs.md#diagnostic-cohort-integration-checkpoint)
closes all bounded diagnostic containers. P2 resume-order drift is also owned/corrected here:
the earlier checkpoint goal jumped to the final trust.rs sweep although its containing assessment
parent still owns historical rows. A literal table census finds463 general-parent rows, not463
current survivors. Next `.15.1` must reconcile historical/current raw identities and completed
evidence, then own the next assessment; only then may the controlling final-sweep prerequisite close.


## Complete initial survivor ownership census

`M3.6.9.6.3.2.2.15.1` (`2026-10-18`) reconciles all540 actual initial survivor rows:
512 retain exact source-span/raw edits (only constructor-derived fresh iterator line numbers
normalized),28 have no current exact edit, zero ambiguity. Maintained old native auditor retains
802K/540S/78U strict1 at36154d6; actual current native listing/discovery agree on1526.
No new current verdict, semantic equivalence or adequacy is inferred from identity or disappearance.

All463 previously general-parent rows gain concrete .15.2–.11 assessment owners: compiler-input
path, build, source extents, compiled rules, tool output, target/program closure, Rules/refusals,
inventory orchestration, command and harness-relative units. These contain442 retained edits/21
without exact current edits. Earlier77 rows retain completed owners, with70 exact edits bound to
audited actual historical measurements and seven absent. Prior result epochs/test populations
remain explicit. All1062 held regular inputs restore before the completion-doc epoch; six false
censuses refuse. Published .14 sealing preserves all267 older archives and reconstructs the live
tree byte-exactly; nine committed leaves move to one immutable archive.

Next .15.2.1 measures all83 current compile candidates against the full parent population and
strict timeout/unjudged rules. Separate command/dependency helpers and later source lanes follow
actual evidence-directed resolution, with fresh whole-file criterion only after required repairs/
assessments. The [book](../../book/src/annex-trust-proofs.md#survivor-ownership-reconciliation)
publishes the source-scope summary and every initial/current/owner row. No production/test/reason change.


## Complete compiler-function measurement

`M3.6.9.6.3.2.2.15.2.1.1` (`2026-10-18`) actually measures all83 current compile candidates
on source069d8e59 with the full153-parent population:49 named K/28 S/6 U, zero E/timeout/unjudged,
native strict1. The copied unmutated baseline passes. Controller0 means audited completion, not
adequacy. Applicable reasons0/file records26; no unrelated E validity inferred.

Actual identities/raw replacements, finishing names and operator totals agree with maintained
discovery/auditor. All1063 original/copied held regular inputs restore including modes/Python-system
hashes; restored copied parent names agree. Opaque gitlink/borrowed copied index metadata only.
Held native21450e7 activation is distinct from completion documents and not an integration stamp.
All30 pinned source copies/tool metadata and code/operator/catalogue/reasons remain exact.
Seven falsified receipts refuse. Book/staged gates check the later completion epoch.

All28 S gain exact owned source groups under .15.2.1.2–.9: environment admission4, command filter4,
artifact suffix2, tail continue1, absolute dependencies4, absolute configuration10, own package1,
owner cardinality2. First .2 assesses actual environment positions/prefixes; no S is preclassified.
The [book](../../book/src/annex-trust-proofs.md#complete-compiler-function-measurement) publishes every
outcome, one actual finishing name per K, precise compiler diagnostics, operator totals and full command.


## Environment admission parent closure

`M3.6.9.6.3.2.2.15.2.1.2.1` (`2026-10-18`) closes four actual compile survivors:
L841.guard.292/.293 and L845.dir.301/lit.302. The old HOME fixture has its foreign record last.
Two new permanent parents hold a nonterminal actual foreign dependency record and the foreign
CARGO_PKGNAME/PREFIX_CARGO_PKG_NAME lookalikes, with allowed value/absence, literal refusals,
no inventory after refusal and successful recovery. Production behavior is unchanged.

Actual full155-parent selected native replay4K/strict0 has each required intended finishing name;
old153 names remain. All1063 original/copied inputs restore, seven false bindings reject (including
another valid but unintended parent),30 pinned copies/tool metadata unchanged. Removing only the
two added functions reconstructs earlier Rust bytes exactly; production/global1526 raw listing and
all operators/catalogue/26 reason records stay exact. Native heldb423bf43 epoch precedes later
completion documents and final staged focused gates. No E, original production repair or projected
fresh83 outcome. The other24 earlier survivors remain owned; command filtering .15.2.1.3 follows.
The [book](../../book/src/annex-trust-proofs.md#compiler-environment-admission-regressions) explains
the real producer position, namespace controls and exact four verdict/name bindings.


## Command-filter producer assessment and owned phantom-unit defect

`M3.6.9.6.3.2.2.15.2.1.3.1` (`2026-10-18`) refutes JSON-only equivalence for the four original
command-filter survivors. Compiled actual environment/parser/split helpers and real pinned Cargo
show deprecated-note warnings routed to stdout in fixed JSON mode, while a wider plain-mode
control parses the warning as a command. Actual multiline global_asm warnings also yield raw
stderr command-shaped records under fixed JSON. Pinned Cargo forwards non-JSON stderr lines.

The maintained actual inventory fixture, with exact global_asm extent admission, writes for
ordinary source and warning text naming not-a-compiler. Changing only the warning name to rustc
(with refreshed admission hash) refuses gen/chk for empty-name phantom units; real builds succeed.
P2 defect `.3.3` owns this false refusal and is scheduled immediately after `.3.2` finishing
parents. The parser cannot treat warning text as compiler-invocation provenance. Preserve existing
admitted-source behavior, bind actual invocations independently of warning data, and rebind every
affected mutation/producer/caller/equivalence premise after the repair; full milestone integration
and independent review requirements remain.

All1063 original regular inputs restore; only a temporary copied test was added and its Rust bytes
restore exactly. Six false observations refuse. Seven new source copies match official Cargo
archive/cache,37 total identities/system hashes rechecked. Ten literal writer call sites across
seven production modules are a bounded syntax census, not all possible parsed input origins.
All original Rust/operators/catalogue/reasons stay exact HEAD. The [book assessment](../../book/src/annex-trust-proofs.md#compiler-command-provenance-assessment)
records the exact macro/admission and all producer/inventory outcomes. No E or new finishing
parent, full-region result, root repair or universal-host claim is made in this assessment.


## Nonterminal command-warning parent closure

`M3.6.9.6.3.2.2.15.2.1.3.2` (`2026-10-18`) adds one bounded actual-inventory parent.
Two explicit admitted non-compiler warning records hold a nonterminal case independent of
consumer scheduling, with ordinary/admitted success and recovery. An independent cold producer
observation replaces an initially overstrict ordering assertion: consumer compilation can start
before final dependency diagnostics. Preserve the initial exit101 and corrected actual observations.

Fresh full156-parent selected native4K/strict0 requires the new parent for every exact L797.guard.
273/.274/.275 and L799.cond.277 edit. All1066 original/copied regular inputs restore with modes/
Python-system hashes; copied156 names agree,37 pinned copies/tool metadata unchanged. Seven false
receipts reject, including another valid but unintended parent. Only one Rust function is added;
all previous Rust bytes reconstruct exactly and production/global1526 raw edits/operators/catalogue/
26 reason entries remain unchanged. Held nativeff8c428 epoch precedes final staged focused/book gates.

No projected full83 result follows;20 earlier survivors remain owned. The original P2 phantom-unit
defect is open, with .3.3.1 contract/prototype immediately next and bounded repair/full milestone
verification afterward. The [book](../../book/src/annex-trust-proofs.md#admitted-nonterminal-warning-regression-parent)
records exact intended names, fixture correction and the distinction between coverage and repair.

## Compiler provenance repair contract and prototype

The complete pending repair contract, tracked prototype, actual outcomes and bounded
implementation obligations are in [compiler-provenance-contract.md](compiler-provenance-contract.md).
Unit collection/artifact binding implement the contract; compatibility/source checks and independent verification remain open.
