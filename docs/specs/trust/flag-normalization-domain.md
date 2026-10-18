# Inventory compiler flags: producer evidence and caller premises

Initial assessment observed `2026-10-12`, owner `M3.6.9.6.3.2.2.11.2.3` in `docs/tasks/M3.md`.
Source: [the pinned Rust toolchain](../../book/src/ledger.md#rust-toolchain).
That initial source/caller assessment left the table obligations open. Later sections record the scoped
proofs, final table replay and `.11.4` checkpoint closing that group. The prefix/helper cohort later closes in `.13.3.2.2`; each subsequent proof states its domain and revalidation conditions. The confirmed prebuild metadata defect below is repaired by `.11.2.5.1`; caller/E premise and local
integration revalidation pass in `.11.2.5.2`. Further assessment `.11.2.6.1` confirms a Cargo-home reset
execution gap below; `.11.2.6.2` repairs it before `.6.3` completion and further classifications.

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
arguments. `run_cargo` clears the environment, uses the catalog base allowlist plus explicit owned observer
settings, and fixes `RUSTC=rustc`. The observer forwards immutable native argv without adding flags
and removes its private settings before the real compiler. Inherited wrappers remain excluded.
`catalog_build::environment` supplies PATH/HOME/rustup/toolchain and fresh project-local Cargo/target homes.
After `PROGRAM.89.1`, it also prepares the target-local canonical temporary directory and seeds
TMPDIR/TMP/TEMP; these are process settings, not new compiled-input admissions.
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

## Permanent lint-table proof controls

Completed `2026-10-17`, `.11.2.4.1`, on unchanged Rust production from `d223d8d`. The eight short/long
lint-level consuming-bit switches are scoped equivalents for the current pinned successful inventory
caller, under the source/caller premises above and below. This is not an arbitrary-argv claim or a
claim about every numeric toolchain release the generic inventory can select; a producer/pin change
requires revalidation. These eight reasons do not classify cap-lints or color.

`DiagnosticFlagTests.test_actual_lint_table_candidates_hold_only_the_inventory_caller_domain` discovers
each actual native table edit and compiles its constants and unmodified normalization helper in a separate
module beside the original. Real pinned Cargo builds cover ordinary/allow/warn/deny/forbid manifests.
The control independently requires the complete producer argv shape and a literal normalized vector;
only two nondeterministic hash values are matched by their fixed hexadecimal form. It decodes these
simple Unix records independently with shlex, including the known quoted cfg operands. This is finite
producer evidence; the source construction and caller restrictions supply the visited-key absence argument.

All eight `cargo rustc -- <separate-flag> unused_variables` cases compile successfully outside the
inventory's command contract. The original drops the separate flag and operand; each actual variant
leaks `unused_variables`, matching independently specified output. Synthetic consumed-value controls show
that a bare key consumed by an unchanged out-dir entry is not visited; those helper inputs are not claimed
as successful Cargo builds. Deliberately wrong positive and negative runtime oracles both refuse.

Permanent controls run before adding reasons: all compiled checks finish, then only the missing per-file
reason assertion fails. Eight exact reasons are appended to the existing E prefix. Fresh native selected
measurement with the unchanged parent filters moves eight survivors (strict 1) to eight scoped equivalents
(strict 0), with explicit reason snapshots. The maintained audit rejects an unlisted E, wrong exit and
missing result. Both runs restore all captured originals and current copied regular inputs. This adds no
kill and does not establish full-table/full-file adequacy or whole-file stale-E handling in selected mode.
Commands, exact identities, input populations and qualified supplemental receipts are in the owning leaf.

### Conditional cap-lints consuming bit

Completed `2026-10-17`, `.11.2.4.2`, on unchanged Rust production from `428a3cd`. Pinned
`add_cap_lints` emits a separate cap only when `show_warnings` is false or `is_local` is false.
`Unit::is_local` is path-source and non-stdlib; local units always show warnings. The repaired fresh
offline homes, source-resolution and no-build-std caller premises in `.11.2.6.3.1` make both branches
false for compiled inventory units. Extra-argument/environment/configuration/profile and dynamic-target
routes remain excluded. Another unchanged consumer can consume a bare-key operand without visiting it.
This justifies only the current pinned single-edit domain, with revalidation on changes to these premises.

`DiagnosticFlagTests.test_actual_cap_lints_candidate_requires_local_unit_premises` compiles the original
helper and actual native cap consuming-bit edit. Independently expected real local producer argv and
normalized vectors cover ordinary and warning-lint manifests. Successful `cargo rustc -- --cap-lints allow`
is a wider-domain falsifier: the changed helper retains `allow`, independently expected. Synthetic
consumed-value and wrong positive/negative controls pass. The producer helper adds only an optional operand
with its old lint default; all earlier test bodies and expected vectors remain exact.

The permanent control completes every compiled check before failing only for missing E; after one exact
reason is appended, it passes. Fresh selected native outcome moves one survivor/strict 1 to one scoped E/
strict 0, with explicit reason binding and altered-evidence refusal. Original/current-copy restoration
and all 27 operator controls pass. No kill, color classification, full-table/full-file adequacy or general
numeric-release equivalence follows. The owning leaf records actual commands, identities and populations.

### Color compatibility and permanent proof controls

Observed `2026-10-18`, `.11.2.4.3.1`, on unchanged production `e4fc5a0`. Pinned
`add_error_format_and_color` always appends `--error-format=json` and `--json=…`; its human/short display
choices do not remove those compiler arguments. It emits no separate color key. Actual `cargo rustc`
probes across human/short/json and always/never/auto color choices all fail with the attributed rustc
color/JSON conflict. These expected producer refusals are neither defects nor successful wider-domain
controls. Direct pinned rustc without JSON accepts the color flag and writes metadata, confirming that
the conflict is the combination rather than an invalid color value.

The [pinned public RUSTC contract](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/doc/src/reference/environment-variables.md)
allows selecting another compiler program. An exploratory test-owned forwarding compiler removes only
JSON/error-format options, records the exact argv it forwards, then execs pinned rustc. `cargo rustc --
--color always` succeeds through that published override and produces a real rlib. Raw Cargo argv still
contains the formatter options; actual forwarded compiler argv does not. The records stay distinct.
Independent Python/system hashes agree for the artifact; another rustc invocation consumes it and runs
its exported function successfully. All files stay in the owned repository-volume fixture.

The override remains outside the inventory caller, which fixes `RUSTC=rustc`; no production selector or
admission changes. Initial `.11.2.4.3.1` diagnostics remain qualified exploratory evidence. Permanent
`DiagnosticFlagTests.test_actual_color_candidate_requires_the_fixed_inventory_formatter`, completed
`2026-10-18` in `.11.2.4.3.2`, now discovers and compiles the actual consuming-bit edit beside the original
helper. Ordinary, raw Cargo and actual forwarded compiler argv have independently specified shapes and
normalized vectors. The forwarded prefix is explicit and its unchanged tail comes from the already
checked raw expectation, rather than from applying the normalizer or forwarding filter to actual data.

For always/never/auto, plain JSON-plus-color compilation must refuse and produce no library in a fresh
target. The published override then compiles, logs exactly one physical compiler invocation and produces
the expected archive. Another rustc compilation consumes that library and runs its exported function.
A fixed owned compiler basename on PATH avoids unescaped executable-path ambiguity. These are valid
wider Cargo builds outside the inventory selector; each actual variant retains the color operand while
the original drops it. Synthetic consumed-value controls and wrong positive/negative/forwarded-argv
oracles hold the narrow claim and reject conflating raw requests with executed compiler arguments.

All compiled checks precede the E-presence assertion. Before the reason that assertion alone fails;
after one exact reason is appended it passes. Native selected survival/strict 1 becomes scoped E/strict 0,
with explicit reason binding and altered-evidence refusal. All previous test/helper bodies and E prefix
remain exact; original/current-copy restoration and the complete operator suite pass. Source construction
and caller restrictions justify visited-key absence, not finite sample absence or compiler-refused input.
Changed producer/pin, formatter/selector or other caller premises require revalidation. Final table replay,
whole-file stale-E handling and full production adequacy remain separate; no general numeric-release or
arbitrary-argv equivalence follows.

## Final bounded table replay

`M3.6.9.6.3.2.2.11.3` (`2026-10-18`) rediscovers every actual table edit and uniquely binds the unchanged
original table/operator/from/to keys: three named finishing kills (`--out-dir`, `-L`, `--check-cfg`), ten
scoped equivalents and the array-terminator compiler refusal; strict exit 0, bool unviable/attempted 0/13
and stmt 1/1. The refusal proves no coverage. All eleven original survivor obligations resolve on the
repaired caller; their original historical S outcomes remain recorded, rather than retrospectively
classified under premises absent from the earlier caller. Exact selected reasons and independent compiled
controls justify current E; selected mode does not establish whole-file stale-E validity.

All 1026 captured originals/current copied regular inputs restore with independent hashes/modes; source,
prior tests, E and native producer stay unchanged. Maintained auditing rejects five altered-evidence arms.
The task leaf records every ID and a reproducible native command. Milestone `.11.4` then passes every
current local integration stage (12 passed, zero failed/unavailable/not-built/quarantined). Its official
stamp matches native Git for the unchanged activation tree; all 1026 originals and retained prior-copy
regular inputs restore. Closure documentation follows with separate checks. This closes the bounded flag
group; remaining normalise edits, full production measurements, review and runner compatibility retain
their owners. No remote CI outcome follows.

## Confirmed defect: an admitted build script executes

The canonical inventory design §3 promises metadata refusal of build scripts and procedural macros before
package code runs. At the observed pre-repair revision, the inventory's own metadata loop collects targets, but does not
apply the catalog metadata helper's corresponding hard checks. Its later default manifest refusal can be admitted.

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
no maintained regression was present at that observation. Independent wrong-manifest-hash, missing-script-unit and false
no-execution records refuse. All captured original/copied regular inputs restore under the maintained census.
Evidence and digests are in the owning task leaf, with raw records in `target/m369-admitted-script/`.

Repair `.11.2.5.1` enforces actual metadata kinds in the shared reached-package manifest preflight, before
root/harness and generated-entrypoint builds. Neither exact manifest admission can override custom-build
or proc-macro refusal. Permanent public parents
`an_admitted_build_script_is_refused_before_any_compilation` and
`an_admitted_procedural_macro_is_refused_before_any_compilation` require attributed refusal, absent owned
sentinels and absent root compilation directories. For example, an exact build-script manifest admission
with `package.build = false` still returns a static checker inventory: the ignored script is no metadata
dynamic target. Enabling that same script refuses before compilation. An admitted macro called by a root
likewise refuses before expansion can write its sentinel.

Both permanent controls fail on the previous production code because package code actually runs, and pass
after repair. Native regression `trust-admitted-dynamic-targets` removes the exact new preflight and finishes
with both named killing parents. Captured original/copied inputs restore; old code/tests/catalogue prefix,
E and reviewed twins are preserved. Evidence, exact commands/exits and digests are in the repair leaf;
ignored orchestration/audits remain supplemental to permanent controls and maintained native/census tools.

`.11.2.5.2` revalidates the delimiter caller premises after repair: root/harness closure preflight precedes
any root build, and generated entry points preflight their closure before compiling. Test targets use no-run;
other targets use build. Configuration remains alias-only, the environment cleared, and RUSTC fixed. Thus
package scripts/macros cannot inject verbose command records into the scoped decoder input. Its later
program filter is not used as the premise: decoding occurs before that filter. The pinned complete-record
framing and exact Unix separator/encoding argument remain necessary; arbitrary command text stays outside.

The maintained compiled actual delimiter controls pass with their independent argv/wider-domain/wrong-oracle
checks on the repaired source. Full local integration passes every current stage, including gate self-tests,
book, target builds/bindings and emulator/spike. Captured original inputs restore; code/tests/catalogue/E
remain exact. The official integration stamp identifies the activation tree, with closure documentation
checked separately. Exact commands, outcomes, identities and qualified supplemental audits are in the leaf.
No new equivalence or remote CI outcome follows. Source/package/standard-library/channel and visited-token
premises still require completion in `.11.2.6` before conditional cap-lints equivalence.

## Confirmed defect: failed Cargo-home reset reuses a wrapper

Observed `2026-10-12` on unchanged `d00c3dee`, independently audited at resume `2026-10-17`, owner
`.11.2.6.1`. Cargo-home deletion errors are ignored before metadata, each root and generated-entrypoint
build. `environment` accepts an existing directory. A failed deletion can therefore retain Cargo
configuration outside the source-tree/ancestor configuration checks. Fixed RUSTC does not disable a wrapper.

The isolated public reproduction first succeeds on ordinary static generator/checker roots, populating the
same owned Cargo home and lock files. It then writes only an untracked Cargo-home build-wrapper configuration
and an owned passthrough wrapper, makes the home directory readable but non-writable for deletion, and calls
the unchanged inventory again. The wrapper writes the owned `WRAPPER_RAN` sentinel. The public result is an
error because verbose commands name the wrapper and no own compiler unit is collected; no inventory is
written. Thus this is actual unsupplied-code execution before an eventual error, not a false written result.
Fixture permissions restore before assertions through a scoped guard. No external system is accessed.

Independent actual wrapper/config/sentinel bytes, fixture Git cleanliness, absent inventory and permissions
agree. False no-execution, written-outcome and inventory-presence evidence refuse. All captured original and
copied inputs restore; production/tests/catalogue/E are unchanged. Ignored diagnostics/audits are qualified
exploratory evidence, not permanent regressions. Exact commands, digests and failed diagnostic setup are in
the leaf and `target/m369-flags-domain-completion/`.

Repair `.11.2.6.2` routes the metadata, root/harness and separate generated-target resets through
`trust::reset_cargo_home`. Only a missing home is harmless. Every other removal error returns an attributed
invocation error before the corresponding environment, metadata or compilation call can reuse the home.

Permanent Unix public parent `a_failed_cargo_home_reset_stops_before_metadata_or_wrapper_execution` first
builds static roots normally, retains that populated home, adds only an owned untracked wrapper configuration,
and denies deletion through directory permissions. Its independent write probe requires actual permission
enforcement. The wrapper sentinel must remain absent, the error must name the reset/home, and metadata/build
directories and the old inventory must be absent. Permissions restore before assertions through a scoped
guard. Making the same home writable again must clear the configuration and produce an ordinary inventory.
Portable parent `cargo_home_reset_accepts_absence_and_clears_data_but_rejects_a_file` holds absent/nested-cache
cleanup and actual non-directory errors, both directly and through public inventory, followed by recovery.

The public wrapper parent fails on the old production because its sentinel is written. Native entry
`trust-home-reset-errors` restores ignored deletion errors and finishes with named failures from the permanent
parents. The bounded sweep also kills the helper's inverted NotFound condition; it is a scoped control,
not a full-file adequacy result. Maintained input census and copied-input checks verify exact restoration;
supplemental orchestration is qualified in the leaf. No existing E record or reviewed twin changes.

`.11.2.6.3` revalidates source/caller and delimiter scope after repair, including offline non-path sources and
standard-library construction, before flag proofs. The earlier integration result remains a dated passing
result; it does not establish absence of this later-discovered defect. No new E classification is justified.

Further exact Cargo sources were retrieved read-only for this assessment. Git fetch requires an existing
cache for offline reuse; cache paths derive from Cargo home. Standard-library unit construction depends on
build-std options. These targeted source observations are premises to complete after the reset repair,
not a complete source-kind/visited-token theorem. Additional retrieved source identities:

| Official source at the same pinned commit | SHA-256 |
| --- | --- |
| [src/cargo/sources/git/source.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/sources/git/source.rs) | `aa8e1bd019c845a62b6754f78d7e9f424f56187f31cdf1d6e26db29a6a8a73ce` |
| [src/cargo/sources/registry/mod.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/sources/registry/mod.rs) | `20715404e59e0fe47f68de6c016c0ebc6e84aebedc740e046f12657147fd148f` |
| [src/cargo/sources/registry/remote.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/sources/registry/remote.rs) | `01b9bb8e68446ad0d4feceeff02b084913d2afd6e067dc34ca56ce9f89a51b65` |
| [src/cargo/sources/config.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/sources/config.rs) | `f3cbe62254b8b4fb512c64abf8d435a8ed0e7e65397c2e9af4be1133884677c8` |
| [src/cargo/core/compiler/standard_lib.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/compiler/standard_lib.rs) | `17cbb514863daad6d6e872750401b1f012a7ca416d401989637a480f5960ee94` |
| [src/cargo/ops/cargo_compile/mod.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/ops/cargo_compile/mod.rs) | `e66a00c5fdd1ff9c27f3f3a3d07326226b26355867535d08659ed4ccb379f04f` |
| [src/cargo/core/compiler/unit_dependencies.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/compiler/unit_dependencies.rs) | `17563082c4c7f47084272cf838400abc3c904de582da570bc473a1bb94d82896` |
| [src/cargo/util/context/mod.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/util/context/mod.rs) | `30310d878bb05ffed2db0c55699619bb1452834d4db5b2166ec2f2aeceebda62` |
| [src/bin/cargo/commands/build.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/bin/cargo/commands/build.rs) | `10435788c80cc418639d2a649acc58efd1d3fb25818fefb9fc78b9d0a9c0bce2` |
| [src/bin/cargo/commands/test.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/bin/cargo/commands/test.rs) | `fe1944aa2e06daadc06fbeb078484b1303e13bb2f31cf2be367a394761e1491d` |

## Revalidation

### Initial inventory-output cleanup

Assessment `.11.2.7` reproduces a separate P2 output-contract gap on `d6791a0`: failed initial removal can
return an ordinary forms refusal while an earlier inventory remains readable. A permanent public control
verifies the retained bytes and their earlier commit against native Git; a directory at the output path
also demonstrates tree/metadata/build work before a late error. Checked `clear_previous_inventory` now
returns an attributed invocation error before forms are classified, allowing only successful deletion or
NotFound. The unremovable old artifact can remain on this exit-2 path; fixing the I/O error and retrying
restores the ordinary no-inventory-on-refusal contract. Directory contents are preserved, with recovery
verified. Native exact ignored-error reversal finishes with both named public parents. Source/caller flag
premises, normalization, previous tests, E and reviewed twins are preserved; the leaf owns verification.

### Gate proposal cleanup

Assessment `.11.2.8`, on `7cdc123`, reproduces ordinary inventory refusal with a new report beside a
readable old proposal whose deletion alone is denied. The permanent macOS control proves actual denial
and sibling writability; the portable directory-at-proposal control also exposes ignored cleanup. Checked
proposal removal now permits only success or NotFound before publishing the refusal report; other I/O
failures end unable to judge (exit 2), naming the path, leaving earlier contents intact. Retry after
repair yields an ordinary refusal with no proposal; a later valid inventory proposes normally. Native
exact ignored-error reversal finishes with both named killing parents. This repairs the gate's output
contract; it changes no inventory source/caller normalization premise or accepted E. The leaf owns the
source reconstruction, original/copy restoration and focused/book checks; earlier full integration is
historical, before this repair.

### Caller domain after checked reset

Revalidated `2026-10-17`, `.11.2.6.3.1`, on committed reset repair `362034e`. The argument concerns the
installed pinned producer and successful inventory compilations under its declared environment/configuration
contract. Local integration checkpoint `.11.2.6.3.2` passes after this assessment; actual-candidate proof
controls and E acceptance remain `.11.2.4`.

**Sources of compiled packages.** `source_id.rs::load` enumerates path, Git, registry, sparse registry,
local registry and directory sources. Source replacement, directory/local-registry selection and alternate
registry settings come from configuration (`sources/config.rs`); alias-only configuration and the cleared
environment provide none. A manifest patch to a path still selects a path source. Admission does not relax
the configuration grammar or the manifest parser's bare-key, single-line, no-string-escape grammar.
`cli.rs::expand_aliases` gives built-in commands precedence over user aliases, so an alias cannot replace
the inventory's metadata, build or test command with a different invocation.

Each successful checked reset removes the old home before metadata or that build; `environment` creates
the empty directory. `GlobalContext` derives Git databases/checkouts and registry index/cache/source paths
from that home. `GitSource::fetch_db` needs a cached revision/database while offline and refuses otherwise.
Both HTTP/sparse and Git registry implementations use those home-derived caches. The HTTP loader returns
NotFound for an uncached offline index entry; registry downloads require an existing nonempty tarball for
cache readiness. An actually needed uncached registry dependency cannot supply a compilation unit offline.
The index resolver may still represent inactive optional/target dependencies without downloaded sources:
this argument concerns compiled units, and does not equate metadata membership with execution.

**Standard library and channel.** `ops/cargo_compile/mod.rs` resolves and generates standard-library roots
only when `cli_unstable().build_std` is present. The inventory passes build or test/no-run arguments, with
neither that option nor trailing compiler arguments; alias-only configuration and the allowlist supply no
unstable configuration or bootstrap override. The installed pin is the stable release recorded in the
ledger; `toolchain_file` requires numeric release syntax. This source argument must be redone for a changed
pin. `Unit::is_local` is path-source and non-stdlib; such units show warnings. Consequently neither branch
of `add_cap_lints` supplies a cap for these compiled units. Installed sysroot libraries are linked, not
rebuilt into standard-library units by these commands.

**Tokens normalization visits.** Follow `prepare_rustc` through base arguments, dependency arguments, cap
selection, extra arguments and unit rustflags; runtime custom flags require the hard-refused dynamic targets.
`TargetInfo::extra_args` obtains rustflags from environment or target/host/build configuration, excluded here.
Admitted profile rustflags still require Cargo's unstable profile feature; the pinned stable-channel gate
refuses it. Manifest lint conversion emits joined `--level=name` or `--level=tool::name` tokens. The diagnostic
formatter emits joined JSON/output-width arguments and no separate color option.

| Candidate family | Why a bare key is not visited in this caller |
| --- | --- |
| Short and long lint-level keys | Manifest conversion uses joined tokens; environment/configuration/profile/trailing-argument routes above supply no extra flags |
| Separate cap-lints key | Compiled path/non-stdlib units are local and show warnings |
| Separate color key | The pinned diagnostic formatter supplies no such argument |

Other argv values do not invalidate that reasoning. Normalization skips the program; source arguments end
in `.rs`, target crate names replace hyphens with underscores, and accepted crate types/editions are their
producer-defined values. Feature cfg values carry their `feature=` encoding. Arbitrary manifest check-cfg
values follow the unchanged consuming check-cfg entry; output/search operands and codegen/extern operands
likewise remain consumed. Thus an operand merely containing or equalling a bare-key string does not show
that the normalization loop visits it. These premises apply to a single table edit with other consumers
unchanged; simultaneous edits and broader compiler invocations require separate assessment.

The existing narrow delimiter premise remains fixed program `rustc`, pinned Unix argument escaping,
complete framed command records, cleared environment and alias-only configurations, with checked fresh
homes and dynamic-target preflight. The program filter occurs after decoding and cannot establish its
input domain. Maintained actual delimiter candidates pass their independent argv, wider-domain and
wrong-oracle controls on the repaired source. Public diagnostic-flag, hard-reset and toolchain-form controls
also pass. The recorded producer's binary identities retain the ledger's stated limitations.

The reset/source integration checkpoint `.11.2.6.3.2` passed on `2026-10-17`, before `.11.2.7`, including gate self-tests, pinned book,
target/browser builds and bindings, emulator and spike. Current originals and the separately scoped retained
copy restore. The official stamp names the activation tree; closure documentation is checked separately.
Exact command, source identities, input populations and stamp binding are in `.11.2.6.3.2`'s leaf.

Additional source bytes retrieved from the same official pinned commit and checked with Python/system
SHA-256; the earlier source identities above are rechecked unchanged:

| Official source | SHA-256 |
| --- | --- |
| [src/cargo/core/source_id.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/source_id.rs) | `fc2c38a770e91f66c45503a3824213c2b0881ffa0a83fef41bb474a1e412d294` |
| [src/cargo/sources/registry/download.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/sources/registry/download.rs) | `5c51eb90daf504013b55af13ea4aa04532ee683ce4c87522a78b4e91eb4609c5` |
| [src/cargo/sources/registry/http_remote.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/sources/registry/http_remote.rs) | `91b958e7137b894ce8594b961fc76e96f7ead76beaf880dc30f86fe65356faad` |
| [src/cargo/sources/registry/index/mod.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/sources/registry/index/mod.rs) | `b5437ca4bdc24a731ff869c1e827412705f1578d4c537639a3a1ba011a45fca7` |
| [src/cargo/core/compiler/build_config.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/compiler/build_config.rs) | `e5f4eeac36687832cd19bd958028bd3811b6757a4d39158b2a1e8bca03586a6f` |
| [src/bin/cargo/cli.rs](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/bin/cargo/cli.rs) | `9648dbe8147e7475ca14b453aba92f0bca9a7741abb9a66d65ee0becc9cae560` |

### Revalidation triggers

Revalidate on toolchain/channel/producer changes, compile-mode/argument/environment/configuration changes,
manifest/admission/metadata policy changes, or changes to token consumption and source attribution. After
repair, maintained compiled controls must distinguish every actual table edit outside its scoped caller
using successful explicit cargo rustc flags and reject wrong oracles before precise E records are considered.
No final whole-production adequacy or remote CI result follows from this assessment.

## Codegen matcher assessment and temporary-path prerequisite

Assessment `.13.2.4.1` (`2026-10-18`) binds the six actual contains/truncated-prefix edits at
`L597.dir.188`, `L597.lit.189`, `L598.dir.192`, `L598.lit.193`, `L599.dir.196` and `L599.lit.197`
to the unchanged native listing and `.13.1` edits. No fresh mutation outcome or E classification follows.

The pinned compiler constructor emits `-C metadata=<hash>` and `-C extra-filename=-<hash>` from
Cargo metadata. When enabled, its incremental constructor emits `-C incremental=<directory>`.
Other codegen operands require their own source domains: profile settings, linker configuration,
build-script linker output, unit rustflags and trailing arguments cannot be dismissed by the sampled
ordinary argv alone. Existing alias-only configuration, stable profile-feature and dynamic-target
restrictions remain relevant. The same official compiler source digest in the table below is rechecked.

A real successful ordinary Cargo producer matches the maintained literal argv assertion. Compiled exact
original and six variants all satisfy independent ordinary normalization expectations, including an
incremental directory containing the other marker strings. For each contains edit, successful pinned rustc
accepts `-C llvm-args=--pass-remarks=metadata=inside` (and the corresponding extra-filename/incremental
values); each produced library is consumed by a separate executable which prints its literal expected
answer. The original helper retains that full option, while its actual contains variant discards it.
These are valid wider-compiler controls, outside the inventory caller; they are not caller-reachability
proofs. Truncated keys also differ for synthetic `<key>X=inside` helper operands, but those operands
have no demonstrated successful compiler producer. Actual bare metadata, extra-filename and incremental
forms are compiler-refused and supply no valid wider control. Wrong retained-option oracles fail.

Caller inspection found an independent defect: the shared `catalog_build::environment` creates local
Cargo/target directories but supplies none of TMPDIR/TMP/TEMP to its cleared children. An exact compiled
API extraction and real cleared child report an effective temporary default outside this repository and
on another volume. Diagnosis only reads OS directory metadata; it creates no off-volume project data
and establishes no historical write or migration census. Existing operator producer fixtures explicitly
seed local TMPDIR and are a separate environment. `PROGRAM.89.1` owns the immediate shared API repair;
`.89.2` owns caller-premise revalidation, fresh native flag/matcher replay and integration before further
matcher classification. Existing E records and historical survivor results are unchanged.
The shared API repair subsequently passes permanent cleared-child/effect/error controls in
`PROGRAM.89.1`; `.89.2` subsequently closes the changed caller-premise checkpoint below before new classifications.

The assessment restores all captured original inputs before closure documentation. Native listing,
compiled modules, literal assertions, pinned producer/artifact consumption and checked source identities
are primary; ignored adapters are supplemental. Permanent per-key evidence follows that prerequisite,
with current gaps explicit. No full caller theorem, copied-input census or full adequacy is claimed here.

## Postrepair temporary-environment checkpoint

`PROGRAM.89.2` (`2026-10-18`) closes the temporary-path prerequisite. Native Git comparison with
`db574de` confirms unchanged trust production, token consumers, preflight/modes and reasons; only the
shared production environment preparation changed. Its added TMPDIR/TMP/TEMP settings supply local
storage and introduce no compiler flags, wrapper or profile/configuration admissions. The compiled-input
allowlist stays exact. Existing actual-edit producer/wider/wrong-oracle controls hold, and 28 recorded
immutable Cargo source identities are rechecked with independent hashes. Toolchain identity metadata
matches the pin; toolchain binary digests are not claimed.

Fresh complete flag replay: three named kills, ten scoped E and one compiler refusal, strict zero.
Fresh matcher replay: all six survive, strict one. No timeout or unjudged result occurs; the native
baselines pass. The survivors remain owned by the per-key M3 leaves with no new E. Refusal establishes
no coverage, and selected E does not validate unrelated whole-file reasons. The maintained auditor
rejects wrong exits, missing/duplicate results, replacement/summary drift and unlisted selected E.
All captured original/current-copy inputs restore after each replay. After staging the activation,
integration originals and the separately retained prior native copy also restore in their own epochs;
copy receipts borrow index metadata and make no copied-index or gitlink-content claim.

All twelve local integration stages pass on activation `22506bfe35d9a0f976c5dd0855109e3e82243d5f`, checked against
native Git and the official stamp. Closure prose follows that activation and receives separate actual
book/staged checks. No remote CI, hardware/assurance result or full-production adequacy follows. The
source/caller checkpoint is complete; permanent per-key matcher evidence and final cohort/full sweeps
remain required. Exact commands, hashes, falsifiers and input populations are in `docs/tasks/PROGRAM.md`.

## Permanent metadata matcher controls

`.13.2.4.2.1` (`2026-10-18`) resolves actual `L597.dir.188` and `L597.lit.189` as scoped E for the
current pinned inventory caller. This relies on source/caller restrictions, not the absence of a flag
in sampled argv. Every literal `-C` constructor in the pinned compiler source is accounted for:

| Operand source | Successful caller domain and effect |
| --- | --- |
| Metadata, extra-filename and incremental constructors | Each uses its literal key followed by `=`; all are already discarded, even if an incremental path contains a metadata marker |
| Prefer-dynamic, debug/overflow assertions, rpath, embed-bitcode and linker-plugin-LTO | Fixed key/boolean forms from the constructor and LTO model |
| Optimization, codegen units, debug/split-debug information and strip | Numeric or named values accepted by the successful compiler; accepted spellings contain no metadata marker |
| Panic and LTO profiles | Cargo validates the named panic/LTO values; its LTO model emits fixed argument forms |
| Explicit linker path | Host/target or matching-cfg configuration is its source; alias-only configuration and the cleared environment exclude that override |
| Linker arguments | Runtime build-script output; hard target preflight excludes it |
| Arbitrary rustflags or trailing compiler arguments | Environment/configuration excluded, admitted profile rustflags still fail the stable feature gate, and compile invokes build/test-no-run without trailing arguments |

The rustdoc metadata constructor belongs to another mode. Codegen backend uses `-Z`, and its profile gate
is separately unstable. Existing visited-token premises still matter: unchanged consumers skip their
operands, source roots end in `.rs`, crate names are normalized, and cfg/edition/type arguments have their
producer encodings. A data operand is not automatically a visited `-C` token.

A qualification found during source review is essential: Cargo's profile `strip` can hold a raw string;
it is not a Cargo enum guarantee. The installed pin's `rustc -C help` names none/debuginfo/symbols, and
actual Rustc and Cargo reject `strip=metadata=inside`. `run_cargo` returns on unsuccessful status before
commands reach normalization. The permanent control holds that refusal, plus a helper-boundary case
showing why arbitrary failed-command argv would invalidate a broader contains claim. This is not valid
successful-producer coverage. The [official rustc guide](https://doc.rust-lang.org/rustc/codegen-options/index.html#strip)
is contextual; the installed pin's native help/refusal is primary. Its version-specific web page was not
accessible to the retrieval tool, and no pinned webpage or toolchain-binary digest is claimed.

The new permanent operator control compiles the exact original and both actual variants. Real ordinary
Cargo argv matches a literal independent contract; libraries return the independently expected value 37
when consumed by another compiler/executable. Successful `cargo rustc -- -C
llvm-args=--pass-remarks=metadata=inside` supplies a valid wider input: original and truncated-prefix
variants retain the full option, while contains drops it. The inventory has no trailing-argument route.
`metadataX=inside` is explicitly only a synthetic helper counterexample for truncation, not a valid
compiler input. An unchanged incremental consumer, wrong positive/negative/helper output oracles and
a wrong forwarded-argv oracle all hold the boundary. The initial broad line selection also found the
already-owned leading-character edit; select only the two exact operator/key pairs. An empty-vector
type annotation was corrected before the successful generated Rust proof; neither failed preparation
supplies mutation coverage.

All prior test bodies and reason bytes are preserved; only the producer fixture's optional source and
refused-strip inputs extend its unchanged defaults. The complete operator suite passes 29 controls.
The two actual native edits now finish with two scoped E results, strict zero, no refusal/timeout/unjudged result, with all
captured original/current-copy inputs restored. False exits, missing/duplicate results, changed edits,
unlisted E and operator-summary drift refuse. Copy receipts borrow original index metadata, without a
copied-index or gitlink-content claim. Closure documentation is checked separately. Changed pin,
constructors, modes, arguments, environment/configuration/admissions/preflight or consumers require
revalidation. Other matcher/helper obligations and full production adequacy remain open.

## Permanent extra-filename matcher controls

`.13.2.4.2.2` (`2026-10-18`) resolves actual `L598.dir.192` and `L598.lit.193` as scoped E. When
present, the pinned Cargo constructor emits the literal `extra-filename=-` prefix and its suffix;
both variants discard it. Metadata and incremental operands are already discarded by unchanged
guards. The complete constructor/visited-token and successful-scalar matrix above applies to this
marker too. Retained successful scalar domains do not contain `extra-filename=`; explicit linker,
build-script and arbitrary flag/trailing-argument routes remain excluded. Failed Cargo status returns
before normalization, including a profile strip string containing this marker.

A new shared proof helper compiles the actual original and both native variants, while every prior
test body remains exact. The key-specific permanent control requires independent literal real Cargo
argv/configuration and consumes each ordinary/wider library with expected value 37. Successful
`cargo rustc -- -C llvm-args=--pass-remarks=extra-filename=inside` is a valid wider compiler case:
the original and truncated-prefix edit retain the full option, and contains discards it. The current
inventory caller has no such trailing-argument route. `extra-filenameX=inside` is only a synthetic
helper counterexample. Refused strip and synthetic cases establish premises, not successful-producer
coverage. An unchanged incremental consumer and wrong argv/positive/negative/helper oracles hold scope.

Only after the compiled proof are two exact reasons appended and bound by the permanent test. The
full operator suite passes 30 controls; native selected replay yields two E at strict zero, with no
refusal, timeout or unjudged outcome. Six false receipts refuse, all captured original/current-copy
inputs restore, and existing test bodies/reasons/production/catalogue/native bytes are preserved.
Copy receipts borrow index metadata without proving a copied index or gitlink contents. No new
Rust-source focused/integration activation or whole-file adequacy follows. Changed pin, constructors,
modes, arguments, environment/configuration/admissions/preflight or consumers require revalidation.
Incremental and basename obligations remain owned. Closure book/staged checks have their own scope.

## Permanent incremental matcher controls

`.13.2.4.2.3` (`2026-10-18`) resolves actual `L599.dir.196` and `L599.lit.197` as scoped E. The
pinned constructor emits `incremental=` followed by the cache directory when enabled. Metadata and
extra-filename guards remain unchanged and already discard their operands. The complete successful
scalar/key and visited-token matrix above excludes a retained operand with an incremental marker;
linker/script/arbitrary flag/trailing-argument routes are still excluded. Failed strip values return
before normalization. No arbitrary-token or every-release equivalence follows.

The permanent control enables a real release profile with `incremental = true`, independently checks
the full actual producer argv and expected `<fixture>/target/release/incremental` operand, and verifies
that the compiler populates that local cache. Both ordinary and wider libraries are consumed with
expected value 37. Exact original/dir/lit variants compile against literal normalized outputs.
Successful `cargo rustc -- -C llvm-args=--pass-remarks=incremental=inside` is the valid wider case:
original/truncated-prefix retain that option, contains discards it. The inventory has no such trailing
route. Synthetic `incrementalX=inside`, a marker under an unchanged metadata consumer and refused
strip are explicitly premise controls rather than valid successful-producer coverage. Wrong forwarded
argv and positive/negative/helper output oracles fail.

After the independent compiled proof, append only these exact reasons and bind both keys. The full
operator suite passes 31 controls; native selected replay yields two E, strict zero, without refusal,
timeout or unjudged results. All prior test bodies/reasons and production/catalogue/native bytes hold.
Captured originals/current copies restore and six altered receipts refuse; copied index/gitlink
contents remain outside the borrowed-metadata census. Helper defaults preserve prior manifest/argv
behavior. Changed pin, constructors, arguments, environment/configuration/admissions/preflight or
consumers require revalidation. All six codegen matcher obligations now close; basename and final
cohort/full production work remain open. Closure book/staged checks are separate from measurement.

## Basename comparison: distinct search positions

`.13.2.5` (`2026-10-18`) resolves actual `L614.cmp.215` as source-context E. The two indices
come from `file.rfind('-')` and `file.rfind('.')` on the same immutable Rust string. If both are
present, their byte positions cannot be equal: one position cannot match both distinct nonempty
ASCII characters. Consequently `dash < dot` and `dash <= dot` take the same branch for every
defined string input here. Other Option cases, branch work, outputs and index progress are unchanged.
This argument requires no Cargo producer or pinned-caller restriction. It is not a theorem about
arbitrary integer comparisons. Revalidate changed patterns, receivers, comparison/context or string
contracts; the permanent test also binds the actual discoverable edit and source search expression.

Before adding the reason, the permanent control compiled the actual original and edited normalise
modules. Independent literal outputs cover empty/plain strings, missing/reversed/adjacent punctuation,
rlib/rmeta basenames, paths, equals signs, repeated punctuation and Unicode. A grid of all words of
length zero through six over `a`, `-`, `.`, `é` checks 5461 helper strings, supplementing rather than
establishing the universal source argument. A same-character search refutes the distinct-position
predicate premise only; it does not claim a wider normalise final-string difference. An equal-index
oracle rejects arbitrary numeric equivalence, and a wrong negative output oracle fails.

After proof, append the exact reason and bind its key. Full operator suite: 32 tests pass; all 31 prior
test bodies, old reasons and production/catalogue/native bytes remain exact. Native selected replay
reports one E, strict zero, with no refusal, timeout or unjudged result. All 1043 captured originals
and current copied regular inputs restore; six altered receipts refuse. Copied index/gitlink contents
are outside the borrowed-metadata copy census. This closes the last initial prefix/helper survivor
obligation; final complete cohort replay and whole-production adequacy remain separate work.

## Complete prefix/helper replay

`.13.3.1` (`2026-10-18`) re-derives the complete `DROPPED_PREFIX`/`normalise` region, binds all
59 actual current edits to their original operator/from/to/relative-span identities, and replays the
unchanged native command with current tests and precise reasons. Native listing remains 1487 across
the production file; only this complete helper region is selected. All 59 edit identities and helper-region bytes
match `.13.1`; its measurement and the initial production outcomes remain historical. Later parent
controls extend the historical test section. The production range and helper-region bytes remain exact.

Final cohort result: 51 named finishing kills, seven scoped E and one compiler refusal, strict zero.
Every one of the twelve survivors in `.13.1` is now resolved: five independent parent controls kill
their exact edits, and the six pinned-caller codegen proofs plus the distinct-search basename proof
justify the other seven. The existing compiler refusal remains a refusal and proves no coverage.
No survivor, timeout or unjudged outcome remains in this selected region. Native per-operator attempted/
refused accounting and every killing test name are retained with exact original/current identities.

Maintained input census confirms all 1043 captured originals restore, with Python/system hashes and
opaque gitlink metadata. The current-copy adapter confirms the same regular-input bytes/modes/sizes,
borrowing original index fields; copied Git index/gitlink contents are not established. Production,
prior controls, reasons, catalogue and native tools are unchanged. Seven altered receipt variants
refuse: exit, missing/duplicate row, replacement, unlisted E, empty reason and operator summary.
These validators bind reason presence/shape/text selection; their mathematical truth rests on the
separately committed source and compiled proofs. Ignored adapters supplement those primary tools.

This is the completed cohort replay only. Caller/premise and full local integration checkpoint
`.13.3.2` is recorded below; closure prose gets a separate pinned book/staged gate. Whole-production final
adequacy, remaining source obligations, independent review and runner compatibility remain open.

## Prefix/helper caller and first integration assessment

Assessment `.13.3.2.1` (`2026-10-18`) binds the complete replay of 59 edits and seven new reasons to
unchanged production, preflight and build inputs from `97189a0`. The local-temp function and compiled
allowlist remain exact. Python and system hashes recheck 28 pinned Cargo source identities. Toolchain
metadata matches; no binary digest is claimed. The 32 operator controls retain exact current source.
The earlier flag receipt binds unchanged source and reasons, with no fresh measurement claimed.
Conditional caller premises remain explicit.

Actual `make integration` exits 2 on held activation `6d82c979f23641a15589b08f6c700a933d96ddd5`:
11 steps pass and one fails, with none unavailable, not built or quarantined. Only the self-test step
fails: the handoff first-sample-only process arm reports eight passed controls and one failed control.
No official integration pass stamp exists for that run. `PROGRAM.92` subsequently verifies its direct
control repair; `.13.3.2.2` subsequently repeats full integration after that committed prerequisite. The first run
remains a failed result.

All 1043 originals restore against the staged epoch. The retained regular-copy population restores
against its prior complete-native epoch; borrowed index fields prove no copied index or gitlink
contents. Production, controls, reasons and gate source bytes remain exact throughout. The caller
assessment alone closes neither checkpoint nor cohort. Its later closure is recorded below;
whole-production adequacy remains open. Closure prose receives separate pinned-book/staged checks.

## Final prefix/helper integration

Final checkpoint `.13.3.2.2` (`2026-10-18`), after committed repair `eb02f9b`, revalidates unchanged
callers, production/build inputs, pinned sources, operator controls and exact complete-helper results/reasons.
The prior flag receipt is source/reason-bound retained evidence, without a fresh flag measurement.
Actual full local integration exits zero: every declared stage passes, with no failed, unavailable,
unbuilt or quarantined result. Native Git and the official stamp identify held activation
`b551ad85dc21174e08c4e167d1a515708d886a3e`. The earlier failed run remains failed historical evidence.

The maintained census restores all current originals (1046) against the staged activation. The retained
native-copy population (1043) restores against its prior complete-replay epoch; borrowed index metadata
proves no copied index/gitlink contents. Actual code, controls, reasons and repaired handoff source stay
exact. Five false activation/result bindings refuse. Later closure prose has separate pinned book and
staged checks. This closes `.13`, the prefix/helper cohort. The following run_cargo preflight `.14.1`
is recorded in the [inventory contract](decision_trust-inventory.md) and book; guarded measurement
is recorded in `.14.2`, with diagnostic assessment `.14.3.1` and the completed framing repair `.14.3.2.1` and expanded binding `.14.3.2.2.1` recorded in the inventory contract. Remaining source obligations, final whole-file sweeps and independent review stay open.

## Retrieved source identities

Every link below names the installed Cargo commit. These are source digests, not toolchain binary digests.

| Official source | SHA-256 |
| --- | --- |
| [src/doc/src/reference/environment-variables.md](https://raw.githubusercontent.com/rust-lang/cargo/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/doc/src/reference/environment-variables.md) | `6f665b89e5a164da0526f22602105ba9dfb16928d76e028bf2fc4a30e0d37323` |
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


## Actual-argv collection caller revalidation

`2026-10-18`, `.15.2.1.3.3.2.1`: the [production collector](compiler-provenance-contract.md#production-unit-collector-and-current-premise-revalidation)
uses Cargo's published wrapper boundary, not diagnostic text, for actual compiler arguments.
The normalization body, base environment9, fixed compiler, configuration/dynamic-target preflight
and all token consumers remain byte-identical or retain their exact construction constraints.
The embedded observer receives immutable native argv and passes its unchanged tail to Command;
its only environment changes remove the wrapper/private settings. A native C receiver independently
holds exact argument bytes, status/stdio and removed fields; actual public inventory retains literal
root/dependency configurations and rejects warning-shaped pseudo-invocations as data.

All16 producer-bound active reasons explicitly carry this revalidated caller. The absolute-prefix
proof source guards additionally cover observer/controller/base environment and reject deliberate
changes to target identity or forwarded argv. All34 maintained operator and compiled proof controls
pass. The retired delimiter test uses a durable historical fixture, requires its four keys absent
from active E and still exercises all five exact helper edits and wider-domain/wrong-oracle cases.
Older native outcomes remain dated; they do not establish every historical caller premise.

Fresh full162-parent native replay includes all22 current trust reason keys, every one scoped E,
strict0. Three iterator counter-name alpha bindings are current. All1077 original/copied regular
inputs restore and copied names agree. Raw listings: trust1433, controller124, observer16; only
reason keys plus four separate protocol smoke edits are selected, not all discovered source edits.
The [book](../../book/src/annex-trust-proofs.md#production-compiler-invocation-collection-repair)
records exact commands/outcomes and the failed-baseline diagnostic-fixture repair. Complete new
source mutation coverage, compatibility and milestone integration remain owned.
Artifact binding `.3.3.2.2` preserves these callers/normalization/producer premises. Its separate
empty-input rustc query adds no Cargo flags; the absolute-cwd guard now binds that production file.
The [artifact record](../../book/src/annex-trust-proofs.md#observed-compiler-artifact-binding) retains revalidation and falsifiers.
