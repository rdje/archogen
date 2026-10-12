# Inventory compiler flags: producer evidence and caller premises

Observed `2026-10-12`, owner `M3.6.9.6.3.2.2.11.2.3` in `docs/tasks/M3.md`.
Source: [the pinned Rust toolchain](../../book/src/ledger.md#rust-toolchain).
This is a source/caller assessment, not an equivalence declaration. The remaining table obligations stay
open. The confirmed prebuild metadata defect below is repaired before further classifications.

## Pinned producer

Installed `cargo -Vv` identifies Cargo 1.95.0, commit
`f2d3ce0bd7f24a49f8f72d9000448f8838c4e850`. Official source files at that exact commit were retrieved
read-only into `target/m369-flags-domain/`; Python and system SHA-256 agree for each retained file.
The source table below identifies those bytes, rather than relying on moving branch documentation.

- `compiler/mod.rs` constructs a compiler command in `prepare_rustc`: base arguments, dependency arguments,
  lint capping, explicit extra arguments, then unit rustflags. `build_base_args` appends manifest lint flags
  and profile rustflags. `add_error_format_and_color` emits JSON/diagnostic formatting arguments; its body
  does not emit a separate color flag.
- `util/toml/mod.rs::lints_to_rustflags` maps each manifest lint level to one `--level=name` token, with a
  tool prefix when appropriate. Custom unexpected-cfg settings append separate `--check-cfg`/operand pairs.
  Ordinary/allow/warn/deny/forbid successful actual producer and public-inventory controls agree with this
  construction (`.11.2.1`, permanent parent `.11.2.2`). Those finite samples alone prove no absence theorem.
- `compiler/mod.rs::add_cap_lints` emits a separate cap only when warnings are suppressed for the unit,
  or the unit is non-local. `unit.rs` makes a path package local unless it is a standard-library unit;
  local units show warnings. This conditional needs a caller premise about package sources and standard
  library construction, not merely the absence of a cap in the sampled fixtures.
- `build_context/mod.rs::extra_args_for` obtains trailing rustc/rustdoc arguments. `target_info.rs` collects
  environment and target/host/build configuration flags. `compilation.rs` selects the process and fills
  its environment. `manifest.rs` exposes the already converted lint flags and replaces hyphens in crate
  names with underscores; `workspace.rs::path_args` supplies the source path and working directory.

## Caller restrictions and explicit admissions

`xtask/src/trust.rs::compile` selects build/bin, build/lib, or test/no-run/test target arguments, then
release/locked/offline/no-default-features/JSON/verbose/package arguments. It supplies no trailing rustc
arguments. `run_cargo` clears the environment, copies the catalog allowlist, and fixes `RUSTC=rustc`.
`catalog_build::environment` supplies PATH/HOME/rustup/toolchain and fresh project-local Cargo/target homes.
Configuration admission is alias-only, with tracked bytes and ancestor checks.

Manifest rules are different: `Rules.workspace` and `Rules.manifests` allow a default rule refusal when
an exact file/rule/whole-manifest hash admission matches. Thus default rustflags/cargo-features refusal is
not an unconditional premise. The stable Cargo feature gate must also be considered.

Pinned `util/toml/mod.rs` requires the unstable profile-rustflags feature for profile rustflags;
`features.rs` marks that feature unstable and refuses activation on the stable channel. Copied diagnostics
with exact manifest admissions confirm both boundaries through actual Cargo and the unchanged public
inventory entrypoint:

| Workspace input | Actual Cargo build | Public inventory entrypoint |
| --- | --- | --- |
| profile rustflags without opt-in | exit 101, feature required/not stabilized | metadata returns an error before compilation |
| profile rustflags plus explicit feature opt-in | exit 101, nightly Cargo required | metadata returns an error before compilation |

These are expected producer refusals, not successful flag-production examples. The first diagnostic oracle
incorrectly expected the same error wording in both cases; saved raw output and pinned feature branches
root-cause the distinction. Inputs were restored before correcting the ignored oracle; the corrected
probe passes. No production defect follows from these expected stable-channel refusals.

A proof must reason about tokens actually visited by normalization. An arbitrary manifest check-cfg operand
may itself resemble another flag, but the preceding check-cfg entry consumes it. Output/search paths,
codegen operands and extern operands likewise have their own consumption rules. Source-root `.rs` admission
and crate-name normalization constrain other values. A blanket claim that a bare flag string occurs nowhere
in every argv would be stronger than necessary. Permanent actual-candidate/wider-domain falsifiers remain
owned by `.11.2.4`; no new E record is justified by this outline alone.

## Confirmed defect: an admitted build script executes

The canonical inventory design §3 promises metadata refusal of build scripts and procedural macros before
package code runs. The inventory's own metadata loop collects targets, but does not apply the catalog
metadata helper's corresponding hard checks. Its later default manifest refusal can be admitted.

Reproduction on unchanged code at `3a2a7b22`:

1. Create the ordinary generator `a` and checker `b` fixture, with `crates/b/build.rs` writing only an owned
   fixture-local `RAN` sentinel. With no admission, public inventory refuses and the sentinel is absent.
2. Add the exact `crates/b/Cargo.toml` admission for the build-script rule and its system-verified manifest
   digest. Leave package code and dependency graph unchanged.
3. Invoke the unchanged public inventory entrypoint again. It returns a written inventory, the sentinel
   exists, and the checker includes both `b` and `build_script_build` compilation units. The latter's
   recorded source hash agrees with the independent literal script bytes.

This confirms an execution-boundary defect, not just a missing test. The diagnostic wrote only inside its
owned synthetic fixture on the repository volume. Ignored copied diagnostic/adapters are qualified evidence;
no maintained regression is claimed yet. Independent wrong-manifest-hash, missing-script-unit and false
no-execution records refuse. All captured original/copied regular inputs restore under the maintained census.
Evidence and digests are in the owning task leaf, with raw records in `target/m369-admitted-script/`.

Priority P1, immediate owner `.11.2.5.1`: enforce the metadata target boundary before builds regardless of
manifest admissions, with permanent public sentinel/refusal controls and actual reversal evidence.
`.11.2.5.2` then revalidates the caller/equivalent premises and full integration. Existing delimiter proof
scope and proposed flag proofs must be checked against the repaired no-dynamic-target caller; do not assume
that every current verbose command is a compiler command while this defect remains. Other source-policy
premises also require explicit evidence before conditional cap-lints equivalence.

## Revalidation

Revalidate on toolchain/channel/producer changes, compile-mode/argument/environment/configuration changes,
manifest/admission/metadata policy changes, or changes to token consumption and source attribution. After
repair, maintained compiled controls must distinguish every actual table edit outside its scoped caller
using successful explicit cargo rustc flags and reject wrong oracles before precise E records are considered.
No final whole-production adequacy or remote CI result follows from this assessment.

## Retrieved source identities

Every link below names the installed Cargo commit. These are source digests, not toolchain binary digests.

| Official source | SHA-256 |
| --- | --- |
| [src/cargo/core/compiler/mod.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/compiler/mod.rs) | `0676b172894d2d0e8536bb5197019f2c147f9aa3e4587e13909e73512fa9d023` |
| [src/cargo/core/compiler/unit.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/compiler/unit.rs) | `edcc3fb5d266cd1de90b2e630cafea34c0bbda6722a860d3614a0f60b0862d8b` |
| [src/cargo/util/toml/mod.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/util/toml/mod.rs) | `0b7e604cabcedb7d9e62942f5004625930220b2b1eef85cc1c71b0c4dfb7bb30` |
| [src/cargo/core/manifest.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/manifest.rs) | `a4ab9a2e3f7177322b659e21dace551f784b709e1041757412eb107e690fc5f9` |
| [src/cargo/core/compiler/compilation.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/compiler/compilation.rs) | `143263b660cd89e6ae2a0ecca33333e23d4254d134f6336a2ae7f36621435753` |
| [src/cargo/core/compiler/build_context/mod.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/compiler/build_context/mod.rs) | `26ad05905afb9d4946d1087d3291c3bd398468082f77420ed79f2e0610585898` |
| [src/cargo/core/compiler/build_context/target_info.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/compiler/build_context/target_info.rs) | `6ea904f8285cda1f8014b4eb2ed92abc61bdf5abc74101da3b802f2f95b6431c` |
| [src/cargo/util/mod.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/util/mod.rs) | `0d11f8f8643282099151a2a28a8505adf7b5bbbcd1760d59e295adcc64fae9cf` |
| [src/cargo/util/workspace.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/util/workspace.rs) | `a8ab44391e698a50d1b8f57a33ff6205866560e7f49520c1334aa8db2f419f83` |
| [src/cargo/core/profiles.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/profiles.rs) | `df4b7e2d13f25efa7bb892a7efd31c82e549f0c80a138af361f7ab4e4252cfb7` |
| [src/cargo/core/features.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/features.rs) | `e07a9570fde7d272c5205579855b9942011275bb1730f56db0b686d6cfe168d6` |
