# Annex C: Trust proof records

The [trust gate](trust.md) states the rules. This annex retains detailed observations, commands and
scope limits for the [pinned Rust toolchain](ledger.md#rust-toolchain) and mutation measurements.

## Compiler records and normalization

The pinned Cargo producer's verbose path quoting is exercised by
`cargo_quoted_output_paths_preserve_inventory_units_and_configuration`
(`M3.6.9.6.3.2.2.10.2`, `2026-10-12`): actual public inventory builds with ordinary, space, apostrophe
and literal-backslash output paths and a shared dependency, preserving normalized configuration and shared
items. This is finite pinned-producer evidence, without a general shell grammar guarantee.

The separate `a_root_s_configuration_retains_its_dependency_argument` control
(`M3.6.9.6.3.2.2.10.3.2`) requires the fixture's binary and library units to retain their actual normalized
`common` dependency arguments, `libcommon.rlib` and `libcommon.rmeta` respectively. A final-token reversal
must fail this independent expectation; no universal argv-shape claim follows.

Producer-domain investigation (`M3.6.9.6.3.2.2.10.3.3.1`, `2026-10-12`) read Cargo's
[ProcessBuilder formatter at its installed commit](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/crates/cargo-util/src/process_builder.rs#L45-L68):
the executable is unescaped, while each argument follows one separator and an escape operation.
That commit's [lockfile](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/Cargo.lock)
selects shell-escape 0.1.5; its [Unix escape implementation](https://docs.rs/crate/shell-escape/0.1.5/source/src/lib.rs)
quotes empty arguments. No downloaded source hash was captured. Actual public inventory reproduction with a
spaced toolchain home fails collection after successful Cargo builds; an ordinary-home control passes.
Exploratory check-cfg probes emit an empty argument only in a measured failing build. Their adapters are
untracked and provide no universal absence proof. Revalidate formatting assumptions on a pin/producer change;
the executable-boundary repair and permanent controls are recorded by `.10.3.3.2`.

The repair fixes inventory compilation's compiler selector to `RUSTC=rustc` under the same pinned environment
as its identity command. The [pinned Cargo environment-variable contract](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/doc/src/reference/environment-variables.md#environment-variables-cargo-reads)
defines that selector. `compiler_installation_paths_preserve_inventory_units_and_configuration` is a permanent
Unix public control with isolated ordinary/space/quote/backslash/combined toolchain homes and independent unit
and dependency expectations. Its exact selector reversal gives a named failure. The command-format assumptions
still require revalidation on a producer change; finite controls do not prove universal empty-argument absence.

`cargo_verbose_empty_arguments_keep_their_token_boundaries` (`M3.6.9.6.3.2.2.10.3.3.3.1`, `2026-10-12`)
builds actual successful pinned Cargo rustc fixtures with nonempty and empty `-A`/`-W` operands. Empty lint
names warn; compilation succeeds. Independent expected middle/final tokens hold the helper's format promise,
without admitting extra flags to inventory builds. Exact native and actual systematic loss edits give named
finishing failures. This permanent control refutes a generic absence premise; no universal shell guarantee follows.

Multiline-path diagnosis (`M3.6.9.6.3.2.2.10.3.3.3.2.1`, `2026-10-12`) retains actual successful Cargo
output with line feeds, carriage returns and backticks, plus losslessly decoded dependency bytes. Copied public inventory control exposes
a physical-line command-slice panic. A diagnostic framing intervention exposes empty compiler-read file objects
from the separate first-target reader. Adapters/tests are untracked and qualified, not permanent controls;
then-unmodified production panicked. Dependency, environment and command repairs below precede further proofs.

The private `cargo_dependency_information_retains_actual_source_paths` control
(`M3.6.9.6.3.2.2.10.3.3.3.2.2`, `2026-10-12`) reads actual pinned rustc dependency output with relative and
absolute in-tree inputs, independently requiring their complete paths across ordinary, line-break, escaped-space,
backtick, tab, backslash and hash roots. Known-target/prefix framing repairs file collection; exact prior-reader
reversal gives a named failure. Synthetic colon-space target parsing is separate from Cargo's measured host refusal.
A qualified copied command-framing intervention then exposes escaped environment-value normalization; its repair
precedes public command framing below. This diagnostic establishes no universal producer grammar.

Environment investigation (`M3.6.9.6.3.2.2.10.3.3.3.2.5.1`, `2026-10-12`) verifies rustc commit
`59807616e1fa2540724bfbac14d7976d7e4a3860` against the installed pin and reads its
[dependency encoding](https://github.com/rust-lang/rust/blob/59807616e1fa2540724bfbac14d7976d7e4a3860/compiler/rustc_interface/src/passes.rs#L513-L531)
and [record writer](https://github.com/rust-lang/rust/blob/59807616e1fa2540724bfbac14d7976d7e4a3860/compiler/rustc_interface/src/passes.rs#L653-L686).
Actual Cargo/runtime controls distinguish real line breaks from literal backslash-n and unset values.
Encoding known scratch prefixes preserves the existing record representation. Exploratory adapters are
untracked; no downloaded source hash was captured. Isolated Rust/native getenv controls also trace an
equals-bearing name lookup to this host's boundary, without a universal name or host contract.

`cargo_dependency_environment_normalizes_escaped_scratch_paths`
(`M3.6.9.6.3.2.2.10.3.3.3.2.5.2`, `2026-10-12`) permanently holds encoded-prefix normalization against
actual Cargo dependency output. Independent relative manifest, encoded package metadata and unset expectations
cover line breaks, backslashes, literal backslash-n and a source-marker path, with a separate ordinary target.
Distinct canonical-prefix control passes; exact old replacement gives the producer control's named failure.
The existing record representation remains.

`cargo_multiline_output_paths_preserve_complete_inventory`
(`M3.6.9.6.3.2.2.10.3.3.3.2.3`, `2026-10-12`) permanently exercises actual public builds across
line-break, backtick, fake-Running and combined apostrophe/backslash paths. Independent unit identities,
source/data hashes, environment and dependency arguments hold; configurations/shared items remain stable.
Raw pinned Cargo stderr and JSON artifacts also identify unquoted Compiling package paths as the source of
a fake command under the first framing intervention. Complete quoted records and known raw status-prefix
handling repair both boundaries; synthetic malformed/canonical controls and both exact native reversals hold.
Exploratory raw status audit remains qualified; permanent public controls establish the repair, not full adequacy.

Delimiter E (`M3.6.9.6.3.2.2.10.3.3.3.2.4`, `2026-10-12`) concerns complete pinned Unix records
with fixed program `rustc`. Source construction gives one outside space before each nonempty encoding
(empty operands encode as `''`), without leading/trailing/repeated separators. At every push boundary,
decoded bytes or a visited quote already make the predicate true. Initial true converges at the first
reset; changing/deleting a reset cannot affect the next true predicate, and making either guard true
changes nothing. Reasons live in `xtask/trust.equivalents`, one text key covering both guards. This does
not cover arbitrary command text; compiled controls expose each difference outside the domain:

| Actual edit | Input outside the domain | Original tokens | Changed tokens |
| --- | --- | --- | --- |
| initialize the flag true | empty text | `[]` | `[""]` |
| make the separator guard true | `" rustc"` | `["rustc"]` | `["", "rustc"]` |
| reset the flag true | `"rustc "` | `["rustc"]` | `["rustc", ""]` |
| delete the flag reset | `"rustc '' "` | `["rustc", ""]` | `["rustc", "", ""]` |
| make the final guard true | empty text | `[]` | `[""]` |

`CommandDelimiterTests.test_actual_delimiter_candidates_hold_only_the_fixed_cargo_domain` discovers and
compiles the actual original/replacements. Independent token expectations cover encoded empty middle/final
operands, spaces, quoted line breaks/backticks, apostrophe/exclamation escapes, backslashes and Unicode.
Deliberately wrong positive/negative oracles refuse. The control binds the fixed selector and installed Cargo
commit; a producer/domain change requires revalidating the reasons. Actual producer/public controls above
remain distinct from these synthetic encoded-input controls. Reproduce with local stores:

```sh
CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/tmp" XDG_CACHE_HOME="$PWD/target/cache" \
  python3 -I -B xtask/mutation_sweep_tests.py . CommandDelimiterTests.test_actual_delimiter_candidates_hold_only_the_fixed_cargo_domain
```

The same selected actual edits survive unlisted, then finish equivalent with the recorded reasons under the
parent population, strict exit 0. Both input captures and copied source restore; altered audit evidence refuses.
Selected outcomes establish these bounded judgments.

The final complete current helper replay (`M3.6.9.6.3.2.2.10.3.4`, `2026-10-12`) finishes with nineteen
named kills and five scoped equivalents, strict exit 0, with no survivor, unviable, timeout or unjudged result.
Every actual current helper edit is selected; whole-file adequacy is not inferred. Exact source/operator
bindings, the owned quote repair and guard-body context independently connect the original sixteen survivor
obligations to eleven current named kills and five current equivalents. These resolve the obligations after
caller repairs; they do not retrospectively classify an edit in the earlier buggy caller domain.

All original/copy regular inputs restore with independent hashes, sizes and modes; the opaque submodule
identity remains unchanged. Altered audit evidence refuses. Code, tests, catalogue and equivalent records stay
exact. The original full-sweep counts remain historical. Local helper integration passes below; final
whole-production measurements/review remain open. The complete current identities, witnesses and original
bindings are sealed in `docs/task-history/M3/M3.6.9.6.3.2.2.10.3.md`, reached from
`docs/tasks/M3.md`; re-discover them when production changes.

The local helper integration checkpoint (`M3.6.9.6.3.2.2.10.4`, `2026-10-12`) runs `make integration`
with pinned tools on `PATH` and repository-local stores. All twelve steps pass, with no failed or unavailable
step, including all 52 gate self-tests. The first restricted run failed process-detection controls: the
sandbox denied the required process snapshot. Direct OS probes and targeted passing controls isolate that
boundary; the complete rerun with OS reads available retains the runner's bare-Git self-test environment.
Both results stay recorded. All captured regular inputs restore with independent hashes/modes and the opaque
pin is unchanged. The stamp names the activation/diagnosis tree; subsequent closure documentation has its own
checks. This closes the bounded helper group. Dropped-flag obligations are next; complete production adequacy
and runner-baseline compatibility remain open. No remote CI outcome follows from this local checkpoint.


Initial dropped-flag table measurement `.11.1` (`2026-10-12`) selected all fourteen edits: thirteen
consumption switches and the terminator deletion. Two switches gave named kills, eleven survived;
the unclosed-delimiter compiler refusal proved no coverage. Strict exit 1 left adequacy unmet.
Unchanged table/operator/producer bindings retained every original outcome. All original/copied regular
inputs restored with independent hashes/modes; source, tests, catalogue and E stayed exact. The survivors
were owned for Cargo producer/caller assessment; whole-production adequacy stayed open.


The actual producer assessment (`M3.6.9.6.3.2.2.11.2.1`, `2026-10-12`) accepts ordinary and manifest
allow/warn/deny/forbid cases: separate `--check-cfg` operands `cfg(docsrs,test)`/`cfg(feature, values())`,
and single `--level=lint` tokens. Independent raw/public records agree; altered records refuse and inputs
restore. Its copied diagnostics/adapters are ignored exploratory evidence, not maintained tests or an
absence theorem. The permanent parent `cargo_diagnostic_flags_do_not_enter_unit_configurations`
(`.11.2.2`, same date) requires actual producer operands and independently expected public identities,
source hashes/environment/retained arguments. The actual check-cfg switch changes from survivor to named
finishing kill/strict 0; exact operand-leak failure and restored pass agree. Only that original obligation closes.

Source/caller `.11.2.3` and completion `.11.2.6.3.1` bind package/cache/stdlib/channel and visited-token
premises to pinned sources in `docs/specs/trust/flag-normalization-domain.md`. Metadata repair `.11.2.5.1`
refuses reached scripts/macros despite admissions; `.11.2.5.2` revalidates delimiter scope. Checked cleanup
stops on Cargo-home (`.11.2.6.2`), inventory (`.11.2.7`) and proposal (`.11.2.8`) errors, held by public/native controls.

Lint proof `.11.2.4.1` (`2026-10-17`) covers eight separate lint-level consuming-bit edits. Pinned Cargo
emits joined manifest lint tokens; the scoped caller supplies no separate ones. `DiagnosticFlagTests`
compiles actual edits, requiring independent real argv and normalized outputs for ordinary/allow/warn/
deny/forbid builds. Successful `cargo rustc -- -W unused_variables` and the other seven separate forms
refute broader equivalence: each changed helper leaks the operand. Synthetic consumed-value and wrong
positive/negative controls hold the scope. Reproduce with the local-store environment above and
`python3 -I -B xtask/mutation_sweep_tests.py . DiagnosticFlagTests`. Eight unlisted survivors become eight
scoped E, strict 0; no kill is added. See the final table replay below.

Cap-lints proof `.11.2.4.2` requires compiled local path packages outside the standard-library build,
which show warnings. Those premises make both pinned Cargo cap branches false. Actual local argv and
independent outputs agree; successful `cargo rustc -- --cap-lints allow` makes the variant retain `allow`.
Consumed-value and wrong-oracle controls hold the boundary. Selected survival becomes scoped E, strict 0;
changed caller/pin premises require revalidation.

Color boundary `.11.2.4.3.1` (`2026-10-18`): Cargo's human/short/json modes supply compiler JSON,
which conflicts with separate color. A test-owned `RUSTC` forwarder removes formatting options;
raw requests and actual compiler argv stay distinct, outside the inventory's fixed selector.
Permanent controls `.11.2.4.3.2` use successful override Cargo builds for always/never/auto, checking the
actual edit, library consumers, consumed values and positive/negative/forwarded-argv falsifiers.
Selected survival becomes scoped E, strict 0; changed caller/pin premises require revalidation.

Final table replay `.11.3` (`2026-10-18`) binds all fourteen original edits: three named kills,
ten scoped E and one compiler refusal, strict 0. All eleven initial survivor obligations resolve on the
repaired caller; original classifications remain historical. Every original/current copied input restores.
Milestone integration `.11.4` then passes all twelve local stages, with no failures or absences, after
output cleanup and proof controls. Its official activation stamp matches native Git; original/prior-copy
inputs restore. Closure prose is checked separately. The bounded flag group closes; full production
adequacy, independent review and runner compatibility remain open. No remote CI outcome is claimed.


Prefix/helper `.13.1` (`2026-10-18`): all 59 edits yield 46 named kills, 12 survivors and one compiler
refusal, strict 1. All captured originals/current copies restore; exact original bindings hold. Refusal
proves no coverage. The twelve survivors stay owned; full adequacy remains open.

Path controls `.13.2.2` (`2026-10-18`) close two of those measured survivors: `L590.dir.178` and
`L611.dir.213` now finish with their independently expected source/dependency parent failures.
Both exact catalogue entries and the selected systematic replay pass strict zero; the latter has
two named kills and no refusal, timeout, E or unjudged result. All 1029 captured originals/current
copies restore, and five altered-evidence cases refuse. Prior production/tests/catalogue bodies and E
are preserved. Closure documentation and the focused commit gate are separate from that restoration
capture. Ten other prefix/helper survivors remained after that slice; full adequacy stays open.

Codegen controls `.13.2.3` (`2026-10-18`) then kill actual `L595.int.187`, `L604.stmt.203` and
`L605.int.205` with the independent profile/operand parent. Both native harnesses pass strict zero;
the systematic run has three named kills, int 0/2 and stmt 0/1 unviable/attempted. All 1029 captured
originals/current copies restore and five false receipts refuse. Existing bodies and E are unchanged,
catalogue entries append. Closure docs and focused/staged checks are separate. Seven prefix/helper
survivors remain owned; final cohort replay and full production adequacy stay open.

### Codegen matcher domain and temporary-path prerequisite

Assessment `.13.2.4.1` (`2026-10-18`) keeps the remaining matcher results historical. Exact compiled
variants agree on ordinary Cargo-shaped metadata, filename and incremental operands. Valid wider
compiler invocations such as `-C llvm-args=--pass-remarks=metadata=inside` produce libraries consumed
by a separate executable; the original retains that complete option, while the corresponding contains
edit drops it. Extra-filename and incremental markers have matching wider controls. This does not prove
that the inventory caller can emit those arguments. Truncated-prefix helper counterexamples are
synthetic; compiler-refused bare forms establish no coverage or equivalence. Wrong oracles fail.

The source/caller assessment also finds that the shared cleared build environment omits temporary-path
variables. A passive compiled-API probe confirms a default outside the repository volume. It reads only
OS directory metadata and creates no off-volume project files. `PROGRAM.89` owns immediate local-temp
repair and caller/native/integration revalidation before more matcher classifications. The API repair
`.89.1` passes cleared-child path/file-effect and error/recovery controls; `.89.2` closes the caller
checkpoint below. Producer proof fixtures seed local temp variables. No current E record changes. The canonical source domains,
qualifications and evidence are in `docs/specs/trust/flag-normalization-domain.md` and the owning leaf
in `docs/tasks/M3.md`; full adequacy remains open.

### Postrepair caller checkpoint

`PROGRAM.89.2` (`2026-10-18`) revalidates the changed build environment with unchanged consumers and
compiler-selection/preflight rules. Local temp settings grant no new compiled inputs or compiler flags.
Actual-edit producer, wider-domain and wrong-oracle controls hold, and recorded pinned source identities
are checked again. The complete flag replay retains three named kills, ten scoped equivalents and a
compiler refusal, strict zero. The complete matcher replay still has six survivors, strict one; per-key
evidence remains required, and no matcher E is added. Refusal proves no coverage. Altered result receipts
refuse, and original/current-copy input populations restore.

All twelve steps of local integration pass. Its official stamp binds the activation tree recorded in
`docs/tasks/PROGRAM.md`; later closure prose has separate book/staged validation. No remote CI or full
production adequacy is claimed. The temp-path repair and its caller checkpoint close; M3 resumes the
owned metadata matcher evidence.

### Metadata matcher reasons

Metadata evidence `.13.2.4.2.1` (`2026-10-18`) closes `L597.dir.188` and `L597.lit.189` as scoped E.
Cargo constructs `metadata=<hash>`; other visited codegen keys are distinct. Already-discarded
extra-filename/incremental operands stay discarded. Retained scalar values must pass Cargo/compiler
validation. The current caller excludes linker overrides, build scripts, arbitrary rustflags and trailing compiler
arguments. These source premises, with the unchanged operand consumers, justify the reasons.

For example, successful `cargo rustc` can forward `-C llvm-args=--pass-remarks=metadata=inside`.
The original retains that full option; the contains edit drops it. This valid wider input is outside
the inventory caller, so the reason grants no arbitrary-argument equivalence. The truncated key also
differs for synthetic `metadataX=inside`, which has no successful compiler producer in this control.
Both real libraries are consumed and return the expected value 37; wrong argv/output oracles fail.

Cargo forwards profile `strip` strings, and the compiler rejects unsupported values. The refused
`strip=metadata=inside` case establishes a successful-build premise, not coverage from a failed build.
Exact compiled controls and native replay bind both E reasons, strict zero; original/current-copy
inputs restore and altered receipts refuse. Prior test bodies and reasons are preserved. Per-key
extra-filename/incremental evidence and the basename comparison remain required; full adequacy is open.

### Extra-filename matcher reasons

Extra-filename evidence `.13.2.4.2.2` (`2026-10-18`) closes `L598.dir.192` and `L598.lit.193` as scoped E.
When present, Cargo supplies `extra-filename=-<suffix>`; both edits still discard it. Other discarded
fields stay discarded, successful retained scalar values have disjoint domains, and the caller excludes
arbitrary linker/script/flag routes. Existing operand consumers remain unchanged.

For example, successful `cargo rustc` forwards `-C llvm-args=--pass-remarks=extra-filename=inside`.
The original retains that full option while contains drops it, so the reason is conditional on the
inventory caller. Real libraries are consumed with expected value 37. The truncated edit also differs
on synthetic `extra-filenameX=inside`, which is not a successful compiler input. Compiler-refused strip
and wrong argv/output oracles keep these domains separate. Exact native replay gives two scoped E,
strict zero; original/current-copy inputs restore, altered receipts refuse and prior test bodies/reasons
are preserved. Incremental evidence and the basename comparison remain open; full adequacy is pending.

### Incremental matcher reasons

Incremental evidence `.13.2.4.2.3` (`2026-10-18`) closes `L599.dir.196` and `L599.lit.197` as scoped E.
The permanent producer enables the real release cache:

```toml
[profile.release]
incremental = true
```

Its argv must contain the independently expected cache path, and the compiler must populate that
local directory. Both compiled libraries are consumed with expected value 37. Exact original/edited
normalization still discards the `incremental=<path>` operand. Other discarded fields, successful
scalar domains, excluded flag routes and unchanged consumers justify the narrow caller reason.

Successful `cargo rustc` also forwards `-C llvm-args=--pass-remarks=incremental=inside`; original and
truncated-prefix retain the full option, while contains drops it. This valid wider input is outside
the inventory caller. Synthetic `incrementalX=inside`, refused strip and wrong argv/output oracles
keep the premises explicit. Native selected replay gives two scoped E, strict zero; input populations
restore and altered receipts refuse. All codegen matcher obligations close, with prior bodies/reasons
preserved. The basename comparison and final cohort/full adequacy work remain required.

### Basename comparison reason

Basename evidence `.13.2.5` (`2026-10-18`) closes `L614.cmp.215` as source-context E. Normalization
finds the final dash and final dot in the same filename. Those distinct characters cannot occupy the
same byte position, so changing the guard from `<` to `<=` cannot change the selected branch. This
holds for the function's string inputs without requiring the restricted inventory caller.

The actual original and edited functions compile against independent literal examples:

| Extern operand | Normalized entry |
| --- | --- |
| `common=libcommon-abcd.rlib` | `--extern common=libcommon.rlib` |
| `common=cache=one=two/libcommon-abcd.rmeta` | `--extern common=libcommon.rmeta` |
| `common=a.b-c` | `--extern common=a.b-c` |
| `common=a-foo-123.ext.more` | `--extern common=a-foo.more` |
| `common=é-Ω.λ` | `--extern common=é.λ` |

Empty strings, absent/adjacent punctuation and path components have literal expectations too. A bounded
grid of 5461 strings supplements the source argument; it does not prove an unrestricted universe by
enumeration. Searching for the same character instead refutes the distinct-position predicate premise
only, without claiming a final-string counterexample. Equal numeric indices and a wrong negative output
oracle reject broader or inverted claims. Changed search patterns, receivers or source context require
revalidation.

Only after those controls pass does the exact reason enter the per-file record. Native selected replay
reports one E, strict zero; captured original/current copied inputs restore and altered receipts refuse.
All initial prefix/helper survivor obligations now have separate controls or proofs. Final complete
cohort replay and whole-production adequacy remain required.

### Complete prefix/helper replay

Complete replay `.13.3.1` (`2026-10-18`) binds all 59 actual prefix/helper edits to their original
source/operator/replacement identities and preserves their earlier outcomes. Its current result is
51 named kills, seven scoped E and one compiler refusal, strict zero. All twelve survivors from
`.13.1` resolve: five gain independent parent controls, six have pinned-caller codegen proofs, and
one has the source-context basename proof. The refusal proves no coverage. No survivor, timeout
or unjudged result remains in this helper region.

Captured original and current copied inputs restore; production, controls, reasons and native tools
remain unchanged. Altered result receipts refuse. The copier's borrowed index metadata does not prove
copied index or gitlink contents. Reason validators bind the selected text; the source/compiled proofs
establish each reason's substance. This result covers the complete helper region, with the rest of
the production file still open. A separate caller/premise and full local integration checkpoint follows;
whole-production adequacy, independent review and runner compatibility are still required.

### Caller binding and first checkpoint result

Checkpoint `.13.3.2.1` (`2026-10-18`) binds the completed helper replay and its reasons to unchanged
production callers, preflight/build inputs and pinned source identities. Existing compiled controls
remain source-bound. The earlier flag-table receipt is checked against unchanged source and reasons;
it is explicitly retained evidence, rather than a fresh flag measurement.

Full local integration fails its self-test step; every other declared step passes. The failing control
is the handoff census's first-sample-only process arm. No integration pass stamp is issued. Original
inputs restore, and the separately retained prior mutation copy also restores in its own epoch.
`PROGRAM.92` subsequently verifies the direct repair with acknowledged sampling and large-output
controls. Final checkpoint `.13.3.2.2` follows that committed repair below. The complete helper mutation
result remains valid within its recorded scope; this failed run closes neither cohort nor whole-production adequacy.

### Final prefix/helper checkpoint

Checkpoint `.13.3.2.2` (`2026-10-18`) passes full local integration after the committed handoff-control
repair. Format, lint, contract tests, doctrine checks, gate self-tests, the pinned book, target builds,
toolchain premises, browser binding, emulator and target spike all pass. The actual result is
`12 passed / 0 failed / 0 unavailable / 0 not built / 0 quarantined`. The official stamp and native Git
agree on activation `b551ad85dc21174e08c4e167d1a515708d886a3e`; the earlier failed run remains recorded.

Caller/source/proof bindings remain exact. All captured originals (1046) restore against this staged
epoch. The retained mutation copy (1043 regular inputs) restores against its prior complete-replay
epoch; this check borrows index metadata and does not prove copied Git index or gitlink contents.
Altered activation/result receipts reject, and production, controls, reasons and repaired census source
stay unchanged. Closure prose is checked separately with the pinned book and staged doctrine gates.

The prefix/helper cohort is now closed. The next source group is `run_cargo`: first bind its exact
edits and establish volume-local execution before measuring environment-omission variants. Its old
survivors are obligations, with no current classification inferred from historical results. Final
whole-file adequacy, remaining source groups and independent review remain open.

### run_cargo locality preflight

Preflight `.14.1` (`2026-10-18`) binds the complete current `run_cargo` region. Its eleven initial
edits map uniquely to twelve current edits; the extra statement selects the fixed compiler. Apart
from that three-line insertion, the function is byte-identical. Generated iterator identifiers carry
source line numbers, so only that verified identifier prefix is normalized for matching. Raw edits
remain recorded. The four initial survivors stay historical obligations; no current outcome is inferred.

This helper applies the cleared child environment. Its end-elements iterator edit removes middle
settings, including Cargo home, build target and temporary aliases. Before any altered helper can
launch a build, the measurement needs a write boundary that still holds when those settings disappear.
An exact compiled original and six actual edits are observed using a Rust executable named `cargo`
that only prints selected metadata. It creates no artifacts or caches and runs no Cargo/Rustup code.
The original forwards every local store and clears the parent sentinel. The end-elements edit drops
middle stores; last-only and no-environment edits cannot spawn the controlled child in this setup.
These observations are passive controls, not native kill, equivalence or compiler-refusal results.

On the installed macOS host, the preflight uses the OS `sandbox-exec` command and this exact profile.
The executable and toolchain are necessary read-only OS dependencies. `/dev/null` is the sole explicit
non-project output exception: descriptor metadata verifies it is a character device, not project storage.
The writable path is passed at runtime, so the profile contains no checkout-specific path.

```bash
mkdir -p target/run-cargo-preflight
cat > target/run-cargo-preflight/mutation-volume.sb <<'PROFILE'
(version 1)
(allow default)
(deny file-write*)
(allow file-write* (subpath (param "WRITABLE_ROOT")))
(allow file-write-data (literal "/dev/null"))
PROFILE
```

Profile SHA-256: `2b3fbe199bddb4ba84c453a23334c7025308b06127ce3cc96f74b2a17680eedd`.
Prepare local shell stores as described in [verification](verification.md), before running
commands or redirections. Start with a fresh project-owned control fixture:

```bash
mkdir -p target/run-cargo-preflight/allowed target/run-cargo-preflight/denied
printf 'held\n' > target/run-cargo-preflight/denied/existing
ln -s ../denied target/run-cargo-preflight/allowed/escape
```

Save the following control as `target/run-cargo-preflight/guard-probe.py`. It permits one write,
requires permission errors for the listed forbidden operations and verifies held content. Every
probe target is inside the repository volume, including paths deliberately outside the narrower
writable subtree. No off-volume write is attempted.

```python
from pathlib import Path
import errno
import json
import os
import stat

root = Path.cwd()
fixture = root / "target/run-cargo-preflight"
allowed = fixture / "allowed"
denied = fixture / "denied"
assert allowed.is_dir() and denied.is_dir()

(allowed / "positive").write_text("allowed\n")
assert (allowed / "positive").read_text() == "allowed\n"

operations = [
    ("create", lambda: (denied / "new").write_text("forbidden")),
    ("truncate", lambda: (denied / "existing").write_text("forbidden")),
    ("mkdir", lambda: (denied / "new-dir").mkdir()),
    ("unlink", lambda: (denied / "existing").unlink()),
    ("symlink-target", lambda: (allowed / "escape/existing").write_text("forbidden")),
]
refused = []
for label, action in operations:
    try:
        action()
    except PermissionError as error:
        assert error.errno in (errno.EPERM, errno.EACCES)
        refused.append(label)
    else:
        raise AssertionError("write boundary failed: " + label)

assert (denied / "existing").read_text() == "held\n"
assert not (denied / "new").exists()
assert not (denied / "new-dir").exists()
with open(os.devnull, "wb") as stream:
    assert stat.S_ISCHR(os.fstat(stream.fileno()).st_mode)
    stream.write(b"null-device-control")

report = {
    "qualification": (
        "Installed-host parameterized policy controls only; forbidden targets remain "
        "within project volume. No off-volume write attempt or universal OS security proof."
    ),
    "positive_write": True,
    "refused": refused,
    "denied_fixture_exact": True,
    "null_is_character_device": True,
}
(allowed / "guard-observations.json").write_text(json.dumps(report, indent=2) + "\n")
print(
    "guard: allowed write succeeds; five forbidden operations refuse; "
    "held bytes exact; null is a character device"
)
```

Run the control with only its `allowed` directory writable:

```bash
sandbox-exec -D WRITABLE_ROOT="$PWD/target/run-cargo-preflight/allowed" \
  -f target/run-cargo-preflight/mutation-volume.sb \
  python3 -I -B target/run-cargo-preflight/guard-probe.py
```

The measured control passes. Create, truncate, mkdir, unlink and a write through the symlink all
refuse, with held bytes unchanged. The identical operations succeed without the guard on those same
project-local paths, after which the held fixture is restored; ordinary permissions do not explain
the denials. These are installed-host observations, not a universal OS security proof.

For actual parent tests, pass the current repository as the writable root, keeping the same policy:

```bash
sandbox-exec -D WRITABLE_ROOT="$PWD" \
  -f target/run-cargo-preflight/mutation-volume.sb \
  cargo test -q -p xtask -- trust:: trust_generated:: trust_gate:: --skip route_tests
```

The actual unmutated baseline reports `141 passed; 0 failed; 0 ignored`. This proves compatibility of
that baseline with the guard. The next measurement must retain its exact guarded native command,
profile hash, all candidate outcomes and input restoration. A guard denial is evidence to investigate,
never an equivalence waiver or an invented finishing kill. Changed or unsupported hosts must establish
an equivalent evidenced boundary before running environment-omission build variants. Source, existing
controls, equivalence reasons and production behavior remain unchanged by this preflight.

### Complete guarded run_cargo measurement

Measurement `.14.2` (`2026-10-18`) reconstructs the committed guard and control sources exactly,
rechecks the real write-boundary controls, then runs the unchanged native CLI and its parent selection
inside that guard. The unmutated copy passes. Every edit in the complete current helper region is judged:

```text
9 named finishing kills
3 survivors
0 equivalent, compiler-refused, timed-out or unjudged outcomes
strict native exit: 1
```

| Current edit | Change | Actual result |
| --- | --- | --- |
| `L678.stmt.231` | Drop argument, working-directory and cleared-environment setup | killed |
| `L679.iter.232` | Forward only the first environment entry | killed |
| `L679.iter.233` | Forward only the last environment entry | killed |
| `L679.iter.234` | Forward only the two end entries | killed |
| `L680.stmt.235` | Drop each environment assignment | killed |
| `L684.stmt.236` | Drop the fixed compiler selector | killed |
| `L688.cond.237` | Always take the unsuccessful-command branch | killed |
| `L688.cond.238` | Always take the successful-command branch | killed |
| `L688.not.239` | Reverse the success check | killed |
| `L691.dir.240` | Select a line containing `error` | survived |
| `L691.lit.241` | Select a line starting with `erro` | survived |
| `L691.lit.242` | Select a line starting with `rror` | survived |

The native log retains every finishing test name and the per-operator attempted/refused accounting.
The global listing is a catalogue, not attempted coverage outside this helper. The three survivors
are explicit obligations under `.14.3`; strict adequacy is still unmet. No equivalence is added.

#### Why the environment end-elements edit now fails

The original environment at the initial measurement ended with `CARGO_TARGET_DIR`. Its two-end-entry
edit retained that setting. The current environment ends with `TEMP`, so the same constructor now
omits the target setting as well as other middle entries. Source and raw edit identities remain bound;
the historical result describes its historical context.

The existing shared-package test passes with the original and fails with the current actual edit.
Inspection of the resulting public inventory shows no refusals: the first root has units for `a` and
`common`, while the second has only `b`. The shared dependency was silently absent from that second
root’s compiler records. Successful builds alone do not establish a complete inventory.

An independent public Cargo experiment isolates the target-directory choice. It builds identical
tracked fixture sources in a fresh workspace for each mode, keeping every home/temp setting local
and preserving the OS guard. Both root builds succeed in both modes:

| Target arrangement | Root | Cargo artifact `fresh` for `common` | New compiler command for `common` |
| --- | --- | --- | --- |
| Distinct target per root | `a` | false | present |
| Distinct target per root | `b` | false | present |
| Shared default target | `a` | false | present |
| Shared default target | `b` | true | absent |

The inventory reads verbose compiler invocations. A warm shared target lets Cargo reuse the dependency
without another invocation, so its second-root unit is missing. The actual mutated inventory, native
Cargo JSON/verbose records and target-only intervention establish this cause. The production helper
retains target isolation, and its existing parent test detects this omission. The guard-denial concern
was investigated before accepting this result as coverage.

All captured originals and the current mutation copy restore (1047 regular inputs each), with native
modes, sizes and Python/system hashes checked. The copy check borrows original index fields and does
not prove copied Git index or gitlink contents. Seven altered receipts reject; existing source, tests,
operator catalogue, equivalence reasons and native tooling stay exact. Later closure prose receives
separate pinned-book and staged checks. The next slice assesses the three diagnostic matcher survivors
against exact compiled helpers and supported pinned Cargo output, including independent expected lines.

<a id="diagnostic-selector-assessment-and-current-defect"></a>

### Diagnostic selector assessment

Assessment `.14.3.1` (`2026-10-18`) tests the exact original helper and its three current selector
edits with real pinned Cargo. The caller admits alias configuration only; general configuration-warning
examples fall outside that contract. The witness instead uses multiline scratch prefixes already
supported by public inventory controls and an ordinary Rust type error. All home/temp stores stay
local, and the OS write guard remains active.

Cargo first succeeds on the source, then fails after a normal type mismatch. Structured output names
E0308, and Cargo’s independent stderr diagnostic is:

```text
error: could not compile `a` (bin "a") due to 1 previous error
```

All compiled variants succeed with valid source. On the failure case whose physical path continuation
begins `erroneous-prefix`, the original returns the literal diagnostic. The actual `contains` edit
selects a status/path line, the `erro` edit selects the continuation and the `rror` edit returns an empty
fragment. An oracle accepting any error would pass every variant, so it proves no diagnostic precision.
The literal independent expectation distinguishes them. These observations are supported witnesses;
permanent finishing parent controls and repaired-source native outcomes are still required.

The same assessment exposed the original defect, repaired in the following section. A supported path continuation beginning
`error-path` is selected before Cargo’s genuine message:

```text
expected: error: could not compile `a` (bin "a") due to 1 previous error
reported: error-path/source/crates/common)
```

A standalone-helper control uses `error: decoy` in the workspace prefix while keeping artifacts in a
separate ordinary target. That Cargo build can succeed, but it is not the normal inventory geometry:
Cargo cannot join a Unix library-search path containing a colon in its artifact directory. The earlier
assessment’s inventory-layout implication was too broad. The repair keeps that observation qualified
and adds public operational-failure controls, rather than counting an unsupported positive build.

The cause is scanning physical stderr lines without distinguishing multiline path continuations.
Public dual-config warnings reproduce the colon-looking continuation before Cargo’s genuine setup
failure, so merely tightening the prefix to a colon still loses the actual reason. Repair `.14.3.2.1`
below adds permanent regressions. `.14.3.2.2` rebinds obligations against the repaired source. This assessment
changes no production code, existing controls or equivalence records. Its exact compiled source,
producer JSON/stderr, positive/negative controls and Python/system hashes are retained; the complete
native `9 killed / 3 survived / strict exit 1` remains the preceding measurement, not a new result.

### Cargo failure framing repair

Repair `.14.3.2.1` (`2026-10-18`) preserves Cargo’s genuine failure line by excluding candidates whose
byte positions fall inside known multiline workspace or artifact directory spellings. Ancestors cover
warnings about containing configuration directories; canonical spellings cover output aliases. Raw
status/warning paths and Cargo’s Unix quoted arguments both participate. The existing command decoder
and successful-build processing remain byte-identical.

The permanent public-inventory controls first build both roots successfully, then commit ordinary
type errors in both root kinds. They require the literal Cargo diagnostics, no inventory after the
failure and successful recovery after the sources are repaired. Paths include newline and carriage
return, quoting/backslashes, non-ASCII text and distinct aliased/canonical names. Target-only controls
exercise paths inside verbose command arguments. Admitted legacy/current alias configurations emit
real containing-directory warnings, including repeated path spellings in a message.

```text
accepted old-code baseline: 3 finishing parent failures
final repaired parents:    4 passed, 0 failed, 0 ignored
old-defect catalogue entry: killed by all 4 named finishing parents, source restored
```

The colon boundary is explicit. A colon in the artifact path causes a Unix Cargo library-path setup
failure; it is not successful compiler evidence. Public operational controls preserve that actual Cargo
reason even when a preceding dual-config warning has a diagnostic-looking continuation. The earlier
rejected colon-positive baseline remains retained and excluded from the repair’s failing baseline.
Probe setup and compiler stores are repository-local; targeted Cargo/control runs use the recorded
OS guard. The ordinary focused tier uses the repository-local store environment.

For example, `with\nerror-path` denotes a directory name with a real newline. Its continued path is
not used as the build diagnostic. A type mismatch still reports the pinned Cargo message:

```text
error: could not compile `a` (bin "a") due to 1 previous error
```

The maintained reversal command restores the original physical-line selector and proves the regression
parents detect it:

```console
$ cargo xtask mutate --only trust-cargo-failure-path-framing-removed
```

Every older production/test byte reconstructs after removing only the new helper, call replacement
and new regressions. The catalogue appends its defect entry; earlier operator controls and E reasons
stay exact. Final focused, pinned-book and staged gates bind publication. This is a verified defect
repair, not mutation adequacy: the repaired helper region has 51 current edits within a global listing
of 1526. The old three raw selector rows are retired by the production change. Their semantic
obligations and the new helper still require fresh binding, complete measurement and resolution under
`.14.3.2.2`, before the final cohort checkpoint.

## Mutation measurements and controls

Whether the instrument's tests hold the generated-sources rules — what a program reads, the chain, the blob rule,
each generator's role against each reader — wherever the code judges them, not only at the instances they build, is to
be shown under an accepted design (`M3.6.9`, review closed at round 3): a source-linked census of 32 proposed route/parent mutation pairs and
sweeps of the complete production ranges in `xtask/src/trust.rs`, `xtask/src/trust_generated.rs` and
`xtask/src/trust_gate.rs`. Each exact parent mutation must be killed by finishing parent tests; every sweep survivor
needs an equivalence reason, and timeouts fail both strict modes. The design records actual operator reach and binds
the test copy to its source repository. Implementation begins with strict and attributed mutation outcomes: an
invocation error without a failed test summary and failure-list name is unjudged, never a kill or equivalent.
Those modes are built in `M3.6.9.1`; `M3.6.9.2.1` adds source-span replacements and validated multiline equivalents.
`M3.6.9.2` builds all five additional operators and actual per-operator compiler-refusal counts, with compiled
semantic and CLI restoration controls described in `docs/book/src/verification.md`. `M3.6.9.3.1` adds explicit
configuration-root lists: each outer configuration belongs to its innermost supplied root and must still match
the judged tree's tracked bytes and alias-only content rules. Intermediate target configurations and ancestors
above all supplied roots remain refused. The production gate supplies its repository alone. The tests' seam
(`M3.6.9.3.2`) admits `ARCHOGEN_SWEEP_SOURCE` only after canonicalizing it and the test root, verifying that the
source is its own Git top-level, and requiring the test root to be exactly its `target/mutation-sweep/tree` child.
Canonical aliases are accepted; unrelated roots, enclosing ancestors, deeper copies and redirected physical
locations refuse before inventory. The source hint cannot change production's root list. The sweep now supplies
its canonical own source root for every test command, replacing any inherited hint (`M3.6.9.3.3`). All three parent
populations pass in the actual unmutated copy with an invalid inherited hint; that zero-attempt baseline establishes
copy integration, not mutation adequacy. `M3.6.9.4.1.1.1` adds three source/provenance route controls: a live input
link retains its specific refusal, every live form contributes provenance, and middle generators remain present
beside their inputs. Each exact catalogue mutation compiles and is killed by a named finishing route test.
The input-link mutation changes the diagnostic alone: the original refuses a symbolic link, while the mutant still
refuses it as no blob because the regular-file tree excludes symlinks. It does not show changed link acceptance.
The reader/role controls (`M3.6.9.4.1.2`) hold every role package a generator target's build reaches, including a
foreign package after an allowed one; a harness may use both pair roles but refuses a third role's script or
program target; and a reader handed generated runtime data still judges its generators. Each of the four exact
mutations is killed by a named finishing route test. Build-read controls (`M3.6.9.4.1.3`) hold two generators
reaching a shared dependency through another package: plain reads pass; marked and declared reads refuse with
both generators named. Naming the first alone still refuses but loses attribution; dropping transitive units
admits the marked dependency in the native fixture. Both exact mutations produce named finishing kills.
All nine route additions are built. `M3.6.9.4.2.1` adds the 32 exact parent twins, each retaining the expected
killed criterion. The catalogue contract reads the design's census and frozen finding rows, holding exact
population membership, commands/verdicts and each twin's file/from/to identity. Independent falsifications
and a named compiled guard mutation hold those checks. Run the contract with:

```bash
cargo test -p xtask mutation::adequacy_tests
```

This verifies the catalogue, not semantic adequacy. The full strict route run now has 32 named finishing kills
and exact source restoration (`M3.6.9.4.2.2.1`); the [measurement chapter](trust-adequacy.md) records each actual
killing test and the source hashes. The initial parent baseline passed 91 tests; 2 of the 32 exact mutations had named
kills and 30 survived (`M3.6.9.4.2.2.2`). After the required initial operator measurements, parent controls repaired
those gaps. On `2026-10-11`, the combined final exact run kills all 32 entries with finishing named parent tests, strict exit 0
(`M3.6.9.6.3.1`). These historical measurements remain separate from full production-sweep adequacy.

The first full-range `trust.rs` attempt lists 1,420 candidates, lines 144–2493. Its baseline passes; it stops after 150: 78 named kills, 62 survivors, five compiler refusals, four strict timeouts and one unjudged result, raw exit 2. The incomplete-result auditor refuses; all captured original/copied inputs restore. This is not a full measurement or adequacy pass.

The five non-finishing candidates remove index progress in configuration normalization. A compiler-backed diagnostic
uses each actual edit with a trace that stops before a fourth loop-body evaluation: the baseline visits index 1 once,
while each mutant revisits it; the final mutant also appends the same flag repeatedly. Both deliberately wrong trace
oracles fail. This establishes non-progress, without assigning an OS crash cause to the unjudged command.

```console
bash scripts/trust_adequacy.sh --progress --output target/trust-progress.json
bash scripts/trust_adequacy.sh --self-test
```

The parent tests now execute their unchanged bodies in bounded children selected by the same exact libtest names
(`M3.6.9.6.3.2.1.2`). Each parent requires a successful child exit, one named passed test and a finishing single-test
summary. Zero selected tests, assertions, abrupt exits and a child that exceeds its limit become parent failures.
Child diagnostics are escaped so their failure lists cannot supply names to the outer native result. The test-only
worker in `xtask/src/trust_test_process.rs` reuses the mutation runner's direct system-call group cleanup.

The command's deadline stays active until its child exits and both captured output pipes close. An exited child
whose descendant retains stdout or stderr cannot bypass the deadline: descendants still in its process group
are stopped and the command reports a timeout. The leader stays unreaped while its pipes are open, retaining its
identity for group cleanup. A descendant that creates another group or session leaves this cleanup's scope and
can retain a pipe; independently grouped parent workers must finish under their own limits.

Exact guard reversal bypasses a one-second limit; reapplication times out near one second (`M3.6.9.6.3.2.1.5`, `2026-10-11`). Run the actual runner controls:

```bash
cargo test -p xtask mutation::tests::an_exited_command -- --nocapture
```

All 103 original parents remain apart from guards (`2026-10-11`). Workers have 30s bounds in their own groups. Selected native regressions allow 600s per candidate for successive waves; strict timeout rules remain.

```bash
cargo test -p xtask trust_test_process::tests
ARCHOGEN_PARENT_TEST_TIMINGS=1 cargo test -q -p xtask -- \
  trust:: trust_generated:: trust_gate:: --skip route_tests --nocapture
```

Measured `2026-10-11`: five non-progress mutations gain named finishing kills, strict 0; all 32 entries retain their killing-name sets. Inputs restore, production prefixes stay unchanged and focused gates pass, before the complete measurement below.

Measured `2026-10-11`, source 36154d6: the complete `trust.rs` retry covers 1,420 candidates, all operators, the original 103 parents and no E file. Baseline passes in 17s; results: 802 finishing named kills, 540 survivors, 78 compiler refusals, no timeout/unjudged result. Strict exit 1 records a complete measurement; adequacy remains unmet (`M3.6.9.6.3.2.1.3`).

```bash
bash scripts/mutation_sweep.sh xtask/src/trust.rs --package xtask \
  --timeout 600 --timeouts-fail --filter=trust:: \
  --filter=trust_generated:: --filter=trust_gate:: --filter=--skip --filter=route_tests
```

Measured `2026-10-11`: identities/edits/names/counts agree; altered evidence refuses. The first prefix remains, its five non-finishing cases now named kills. Compiler refusals prove no coverage. All 540 initial survivors stay owned in `docs/tasks/M3.md`, requiring repair or justified E before the final strict sweep.

Initial `.1`–`.7` assessments (`2026-10-11`) repair malformed names/placeholder leaves and reader/role/pair test gaps. Committed leaves hold kills, reversals and source boundaries.

Measured `2026-10-11`, `.8.2`: the ends rewrite preserves order/early returns for two borrowed String references, with no user callback/destructor. Alias/wider-array/wrong-oracle controls hold scoped E: native survivor (strict 1), reviewed E (strict 0); production unchanged, no kill added.

Historical runs restore 988 inputs under Python/system hashes, the new E separately hashed; leaves hold commands/identities. Listing `2026-10-11`: 1,416 candidates, production lines 144–2498. IDs need bound source/fresh discovery; final current-source results stay open.

Measured `2026-10-11`, `.9.1`: four environment survivors gain named kills, strict 0. Public controls hold three compiled values and record boundaries; production/old tests stay unchanged and all 990 captured inputs restore.

Measured `2026-10-11`, `.9.2`: marker-bearing values exposed a configuration-key collision. Keys now use
JSON `[command_tokens, environment_records]`, e.g. `[["--crate-name","common"],["CARGO_PKG_DESCRIPTION=alpha env-dep:omega"]]`,
preserving boundaries/order/empty values and public raw-array shapes. The key-only native reversal gains a
named public kill; all 990 inputs restore and domain/focused controls pass. Changed configuration digests
require Linux proposal compatibility in `M3.6.3.2.1.1`, pending an authorized runner/base checker and review.
The reproducer also changes content; it proves the field defect, not a complete-gate bypass.

The [tooling JSON contracts](annex-repository.md#json-used-by-the-tooling) separate encoding from reader diagnostics. Number grammar, raw controls, Unicode escapes and inventory-byte admission are repaired; `.9.4` local integration passed on `2026-10-12`. These defects establish no artifact-hash bypass.

The [focused-stamp guard](annex-repository.md#a-commit-holds-what-was-run) now holds explicit successful-exit claims and large diffs; `.9.5` records its RED controls and repair.

Timeout/unjudged results fail; full adequacy and independent review remain open. Native killing names
route from original `docs/tasks/M3.md` stubs to sealed task-history. Stubs retain commits, the index holds
digests, and the gate proves reconstruction/provenance.


### Complete repaired Cargo diagnostic measurement

Measurement `.14.3.2.2.1` (`2026-10-18`) binds the exact repaired source and all current edits in
`cargo_error_line` and `run_cargo`. The maintained CLI lists 1526 production candidates globally;
51 belong to this region: 42 in the new diagnostic helper and nine retained invocation edits. The
global listing is a scope census, not 1526 attempted mutations.

The eleven initial obligations bind through the preceding twelve-entry cohort. Eight initial edits
remain textually present, along with the later fixed-compiler deletion; the three original raw
selector edits are retired by the production repair. Retirement is not equivalence. Their current
semantic counterparts are measured: `contains("error")` and `starts_with("rror")` are killed;
`starts_with("erro")` survives and stays open on the repaired path-exclusion domain. Only generated
iterator identifier line numbers are normalized for historical matching; raw replacement text is kept.

The installed-host write guard and its probe are reconstructed from the preceding recipe. One allowed
write succeeds, five forbidden operations refuse, and those same operations succeed without the
policy on the same project-local fixture. Held bytes are restored. The exact profile hash remains
`2b3fbe199bddb4ba84c453a23334c7025308b06127ce3cc96f74b2a17680eedd`.

Every selected edit is attempted through the maintained native command, with the same guard covering
the command and its descendants:

```console
$ sandbox-exec -D WRITABLE_ROOT="$PWD" -f target/cargo-error-measurement/mutation-volume.sb \
    bash scripts/mutation_sweep.sh xtask/src/trust.rs --package xtask --timeout 600 \
    --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: \
    --filter=--skip --filter=route_tests --timeouts-fail \
    --equivalents xtask/trust.equivalents --only <each of the 51 IDs below>
```

The placeholder means 51 separate `--only ID` arguments in one invocation, not a literal argument.
The manifest holds every raw source span/replacement and the actual invocation. The unmutated copy
passes. The complete result is **38 named kills, nine survivors and four compiler refusals**, strict
native exit **1**. There is no equivalent, timeout or unjudged outcome. The criterion remains unmet.
All nine retained invocation edits are killed, with a finishing parent also present in the preceding
measurement. The earlier successful-build target-isolation diagnosis remains applicable: the framing
repair changes only failure-text selection, and the invocation/environment producers remain exact.

The following table covers every current candidate. A killed row shows one actual finishing parent;
the native receipt retains the full set of names. Compiler refusals are not semantic coverage.

| Current candidate | Native outcome | Finishing evidence or open obligation |
| --- | --- | --- |
| `L676.eq.231` | survived | Open obligation; assessment scheduled below |
| `L685.iter.232` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_target_paths` |
| `L685.iter.233` | survived | Open obligation; assessment scheduled below |
| `L685.iter.234` | survived | Open obligation; assessment scheduled below |
| `L687.iter.235` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L687.iter.236` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L687.iter.237` | survived | Open obligation; assessment scheduled below |
| `L688.iter.238` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.iter.239` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.iter.240` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.guard.241` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.guard.242` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.guard.243` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L690.cond.244` | survived | Open obligation; assessment scheduled below |
| `L690.cond.245` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L691.stmt.246` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_output_paths` |
| `L692.stmt.247` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_output_paths` |
| `L706.iter.248` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L706.iter.249` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L706.iter.250` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L708.stmt.251` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L714.dir.252` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L714.lit.253` | survived | Open obligation; assessment scheduled below |
| `L714.lit.254` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L715.logic.255` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L715.not.256` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L717.cmp.257` | survived | Open obligation; assessment scheduled below |
| `L717.cmp.258` | survived | Open obligation; assessment scheduled below |
| `L717.logic.259` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L732.stmt.260` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L733.iter.261` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L733.iter.262` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L733.iter.263` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L734.stmt.264` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L738.stmt.265` | killed | `trust::tests::cargo_diagnostic_flags_do_not_enter_unit_configurations` |
| `L742.cond.266` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L742.cond.267` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L742.not.268` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L676.chain.1145` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_target_paths` |
| `L676.chain.1146` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_target_paths` |
| `L687.chain.1147` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L690.contains.1148` | unviable | E0599: no `first` method on `Cow<str>` |
| `L690.contains.1149` | unviable | E0599: no `first` method on `Cow<str>` |
| `L690.contains.1150` | unviable | E0599: `Cow<str>` is not an iterator |
| `L690.contains.1151` | unviable | E0599: `Cow<str>` is not an iterator |
| `L698.chain.1152` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L698.chain.1153` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L716.chain.1154` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L716.chain.1155` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L717.clogic.1497` | survived | Open obligation; assessment scheduled below |
| `L717.clogic.1498` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |

The four refusals are generated collection operations on a string `Cow`, with actual E0599 diagnostics;
they are not changes to the executable program. No equivalent record was added. Nine survivor
obligations remain owned under `.14.3.2.2.2`: first the two end-elements edits over collections of at
most two entries, then the remaining path-selection, prefix, selector and span-boundary cases. Even
the finite-collection cases require exact source proofs before any E classification.

All 1047 original and current copied regular inputs restore, including modes and bytes, with
independent Python/system hashes. The submodule contributes one opaque Git-index identity; its
implementation is not opened. Seven altered receipts refuse: false pass exit, missing/duplicate
candidate, wrong replacement, wrong operator count, unlisted E and missing finishing kill names.
Production, tests, catalogue, native producer and existing equivalence reasons remain byte-identical
to the committed repair. This is a complete bounded assessment, not whole-file adequacy or remote CI.


### Finite Cargo path iterator proofs

Proof slice `.14.3.2.2.2.1` (`2026-10-18`) resolves only `L685.iter.234` and `L687.iter.237`.
Their end-elements rewrites preserve every element because the current constructors supply one or
two elements. This is a source argument; the finite compiled examples support it rather than define
the universe of inputs.

The outer loop consumes an owned `Vec<PathBuf>`. Its constructor starts with cwd and adds at most
one target, from the last matching environment entry. The exact complete constructor is held by a
permanent source guard. Relative targets are joined to cwd before recording; duplicate values do not
increase the constructor's number of positions.

The inner loop is the concrete chain `std::iter::once(path).chain(canonical)`, where `canonical` was
already computed as `fs::canonicalize(&path).ok()`, an `Option<PathBuf>`. There is always the raw
spelling and optionally its canonical spelling. A missing path supplies no canonical spelling.

For a sequence of length one, its sole element is both first and last. For length two, its positions
are first and last respectively. The rewrite therefore skips nothing and preserves order. Its counter
advances at most twice and cannot overflow. Peeking merely moves concrete owned path values; it calls
no user iterator or filesystem operation. Canonicalization remains before the spelling loop, and the
unchanged ancestor/prefix work receives exactly the same values in the same order. The argument also
holds when raw and canonical spellings coincide or the target equals cwd.

| Actual current edit | Proven constructor | Native replay |
| --- | --- | --- |
| `L685.iter.234` | cwd plus optional target; length one or two | equivalent |
| `L687.iter.237` | raw spelling plus optional canonical spelling; length one or two | equivalent |

The permanent control compiles the actual original helper and each actual native replacement. It
checks literal diagnostics and empty output, with line endings of either newline alone or carriage
return followed by newline. Separate traces hold path order.
Its filesystem cases include aliases, absent paths, relative targets and repeated values. These helper
records are constructed controls; the separate native parent replay exercises real Cargo builds.

Wider constructors mark the boundary concretely. The same actual edit compiles for a vector holding
`[cwd, target, third]` and produces `[cwd, third]`. A chain with a raw path followed by a vector of
two more spellings loses its middle too. First/last-only traces differ on distinct bounded values.
Wrong positive, negative and wider-domain oracles all fail. Independent source-premise controls also
reject insertion of another path or expansion of the canonical iterator before classification.

```console
$ python3 -I -B xtask/mutation_sweep_tests.py "$PWD" \
    IteratorTests.test_actual_cargo_path_end_loops_preserve_the_bounded_collections -v
$ python3 -I -B xtask/mutation_sweep_tests.py "$PWD" -q
```

The exact parent passes and the complete operator suite passes 33 controls. After the source/compiled
proof, the per-file record gains only these precise reasons. The guarded actual native replay attempts
both candidates and reports **two equivalent, strict exit 0**; `iter=0/2` compiler refusals. This
selected replay does not validate unrelated equivalents.

All 1051 original and copied regular inputs restore, with modes and Python/system hashes. Seven
altered receipts refuse. Earlier source, operator controls, catalogue and reason prefixes remain exact.
The production helper is unchanged. The preceding full measurement's nine survivors remain its
historical result; these proofs resolve two obligations, leaving seven for source/domain assessment.
Revalidate if a constructor, concrete type, iterator or loop body changes. No arbitrary vector or chain,
whole-file adequacy or remote CI claim follows.


### Relative gate caller assessment

Assessment `.14.3.2.2.2.2.1` (`2026-10-18`) checks a caller premise before classifying the
seven remaining diagnostic-helper survivors. The inventory and baseline CLIs derive paths from
`repo_root()`; the gate's `--repo` argument is retained as supplied. An absolute-path assumption
therefore cannot be taken across every gate invocation.

The original public CLI is compared on the same current main commit, with separate output
directories and the same installed-host write guard:

```console
$ cargo xtask trust-gate --repo "$PWD" --commit HEAD \
    --out target/cargo-caller-path-assessment/absolute
$ cargo xtask trust-gate --repo . --commit HEAD \
    --out target/cargo-caller-path-assessment/relative
```

The absolute invocation returns 0 and writes an inventory for the generator, scheduling checker,
reference model, implementation and differential harness. Its change comparison is explicitly
skipped because the baseline names the other host. That result establishes inventory production here;
it is not a successful comparison across hosts.

The relative invocation returns 1, writes no inventory and reports this false refusal:

```text
trust-undeclared-input: .cargo/config.toml: catalog-source [(config)] a cargo configuration above the repository, on a build's path (§3)
```

This is an original public defect, not a mutation result. `run_gate` preserves relative repo/output
spellings, while `configurations_on_paths` judges ownership with lexical prefix comparisons. At the
repository ancestor, the own configuration spelling does not match the supplied `.` root. The exact
maintained ownership closure, compiled independently with literal expectations, refuses that relative
pair and accepts the absolute pair. OS device/inode identity and equal bytes prove both name the same
configuration. The absolute inventory binds its configuration hash and current commit.

Repair `.14.3.2.2.2.2.2` owns runtime path resolution, public relative/absolute regression controls,
missing-path/error and output-location behavior, plus verification before survivor classification
resumes. No equivalent reason is added on the unproved caller premise.

The held activation input check restores all 1051 regular inputs with modes and Python/system hashes;
the gitlink remains an opaque index identity. The first post-run check correctly refused an intentional
task finding edit written too early. The exact held task bytes were restored for the full maintained
check, then the saved finding was reapplied byte-exactly. Both outcomes and their separate document
epochs are retained. Source, operator controls, catalogue and previous reasons remain unchanged.


### Repository path caller repair

Repair `.14.3.2.2.2.2.2.1` (`2026-10-18`) resolves the selected repository with the runtime
filesystem before deriving its output path or configuration-root list. Relative, absolute and aliased
repository spellings therefore share the same physical root. Relative output arguments remain relative
to that root; an absolute output argument retains its selected destination. Missing repositories and
non-directory paths remain unable-to-judge errors, exit 2, before output creation.

The public wrapper continues to supply no outer configuration roots. A private command worker shares
parsing and invocation with the regression controls, which use only the existing validated fixture
roots. A permanent public-wrapper control requires the containing test configuration to remain refused;
the explicit authorized fixture control succeeds. This test seam adds no CLI option or environment
route for authorizing a configuration above the selected repository.

The isolated command parents use small committed repositories. Their absolute control succeeds before
the fix; the relative call fails with the false above-repository diagnostic. After resolution, the
relative and absolute spellings target the **same physical output**, preserve every inventory byte and
retain the literal root/configuration identities and both expected fixture programs.

```text
before: 2 passed, 1 failed — relative repository/output command
after:  3 passed, 0 failed, 0 ignored
exact catalogue reversal: killed by the named relative command parent; source restored
```

The controls also require absolute-output behavior, missing/non-directory error status and no created
output on those errors. They run in the existing isolated child protocol, so their current-directory
change cannot affect other tests. The defect reversal is maintained as:

```console
$ cargo xtask mutate --only trust-gate-repository-resolution-removed
```

The real public CLI is then tested on the same frozen source commit as the original assessment.
Absolute, relative and repository-alias arguments all return 0 and write byte-identical inventories
at the same output location. Their inventory SHA-256 is
`73c4e740fc47d32b87b4d1a7a74164fec5daa62aa1933d781a332a36346489c6`.
The baseline's other-host comparison remains explicitly skipped; this is inventory/path evidence,
not successful comparison across hosts. Actual missing and file repository arguments return 2 and
create no output.

An initially overbroad oracle compared different physical output directories. Only the library
artifact hash differed: the retained rlibs have the same archive members and different embedded source
directory paths, verified with `ar`, `strings` and hashes. That rejected oracle is not evidence against
the repair or for reproducibility across arbitrary directories. The corrected unit and public controls
compare equivalent spellings of the same destination. An initial unavailable sandbox setup attempt is
also retained and excluded from the test baseline; the actual targeted runs use the installed guard.

All earlier production/test bytes reconstruct after removing exactly the wrapper/worker adaptation,
resolution block and new controls. The catalogue adds one entry; the diagnostic helper, operator
controls and prior E reasons stay exact. All 1051 public-experiment inputs restore with modes and
Python/system hashes. The final focused tier must pass on the held complete staged tree with its
matching official stamp before this repair is committed. That focused pass is recorded with the repair;
full integration and repaired-caller closure are recorded in the checkpoint below. The seven
diagnostic-helper judgments remain open.


### Repaired caller integration checkpoint

Checkpoint `.14.3.2.2.2.2.2.2.2` (`2026-10-18`) closes the caller repair after its focused
implementation and timing-comment correction. The complete local integration tier passes:

| Step | Result |
| --- | --- |
| format | passed |
| strict lint | passed |
| complete contract suite | passed |
| doctrines | passed |
| script self-tests | passed |
| pinned [mdBook](ledger.md#mdbook) | passed |
| no_std build | passed |
| pinned toolchain premises | passed |
| wasm build | passed |
| wasm binding | passed |
| emulator configuration | passed |
| target spike | passed |

```text
12 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined
activation tree: 180b081792ea1770b6ca8e79efcf11487e97cb21
```

The actual runner log, native Git activation identity and official integration stamp agree. All 1055
held regular inputs restore, including modes and Python/system hashes; the submodule supplies only its
opaque Git-index identity. Five altered bindings refuse: nonzero exit, a missing step, a false summary,
a wrong stamp and missing restoration. Completion prose is a later document epoch, with separate
pinned-book and staged doctrine checks. The integration stamp names the held activation above, rather
than claiming the later completion text itself was the tier's input.

The source/caller binding retains 28 pinned source identities and the 33 operator controls. The
environment constructor, compiled-input allowlist and recorded reasons remain exact. The repaired
command resolves its runtime repository before output/configuration roots, and its production wrapper
supplies no outer roots. The actual frozen-source public absolute/relative/alias and invalid-repository
receipts remain bound; their other-host baseline comparison skip remains explicit.

Historical measurements are rebound by exact source span, operator and raw edit: the prefix/helper
cohort59, flag cohort14, repaired diagnostic region51 and finite-iterator pair2. Their original outcomes
stay historical. No fresh mutation outcome or whole-file equivalent validity follows from this
integration pass. Seven diagnostic-helper obligations still need source/domain assessment and their
finishing controls or precise proofs, followed by the complete current replay.

This checkpoint is local integration evidence. It does not establish remote CI, physical-board
execution or F30 adequacy. The caller defect and stale timing rationale are resolved; the next activity
is the seven outstanding helper judgments.


### Seven diagnostic-helper obligations: domain assessment

Assessment `.14.3.2.2.2.2.3` (`2026-10-18`) compiles the unchanged actual helper and all seven
remaining edits, together with the actual shared environment constructor. It separates real
[Cargo](ledger.md#rust-toolchain) diagnostic witnesses from constructed records that mark proof boundaries.

| Edit | Assessment | Next verification |
| --- | --- | --- |
| `L676.eq.231` | A target alias has a multiline raw spelling and an ordinary physical destination. Selecting the last non-target environment entry loses that raw alias; its command continuation replaces the compiler error. | Permanent alias parent and native replay |
| `L685.iter.233` | A multiline cwd emits a dual-configuration warning; its artifact target has a separate ordinary path. Last-path-only processing loses cwd and selects the warning continuation. | Permanent independent-cwd parent and native replay |
| `L717.clogic.1497` | A missing-target manifest makes Cargo print its leading error before a multiline manifest path. Removing the lower bound suppresses that actual error. | Permanent leading-error parent and native replay |
| `L690.cond.244` | Absolute-path examples agree; relative constructed records differ. | Exact absolute-prefix/caller proof with permanent premise guards |
| `L717.cmp.257` | Absolute-path examples agree; relative constructed records differ. | Exact absolute-prefix/caller proof with permanent premise guards |
| `L714.lit.253` | A constructed non-Cargo `erro:` line distinguishes the shorter literal. | Actual producer evidence or supported witness |
| `L717.cmp.258` | A constructed path ending in newline immediately before an error distinguishes the closed upper bound. | Actual producer/path-boundary evidence or supported witness |

The compiler cases use the actual build arguments and independently retain structured E0308 and
the literal `error: could not compile … due to 1 previous error` diagnostic. OS device/inode identity
confirms the artifact alias and physical directory are the same destination. The configuration-warning
case keeps cwd and artifact prefixes independent, so a shared multiline ancestor cannot mask omission
of cwd. The manifest case exercises the failed-command contract directly; it is not an admitted
inventory or a successful public gate.

The helper can consume arbitrary strings, but constructed records are not automatically possible
outputs of the pinned producer. Conversely, agreement on a finite set cannot prove a supported-domain
equivalence. The next slices therefore add the three real-Cargo parents first, prove the two
absolute-prefix proposals separately, and resolve the final two producer-dependent obligations.

All 1055 held regular inputs restore with modes and Python/system hashes; the submodule remains an
opaque Git-index identity. Four falsified bindings refuse. The installed write guard permits its
positive control and refuses the five forbidden operations; the actual Cargo witnesses use that guard.
Initial scratch-template and stale-binary attempts are excluded from accepted final receipts.
Production, operator controls, catalogue and existing reasons remain unchanged. These are source-bound
witnesses and scheduled proof obligations, with **no fresh native kill or equivalence classification**.
The historical nine-survivor measurement and later two finite-iterator proofs retain their own epochs.


### Real Cargo regression parents and exact replay

Regression slice `.14.3.2.2.2.2.4` (`2026-10-18`) turns the assessment's actual
[Cargo](ledger.md#rust-toolchain) witnesses into permanent isolated parents. Each begins with a
successful build, requires the failure diagnostic, then restores valid source/manifest bytes and
requires successful recovery.

The artifact-alias parent separates the raw multiline alias from its ordinary canonical destination.
The environment's target-local temporary directory is canonical, so selecting its last non-target
entry cannot accidentally preserve the raw alias. The cwd parent moves the workspace to a multiline
path while keeping the target elsewhere; a dual-configuration warning then requires cwd coverage.
Both compiler parents independently inspect structured E0308 and the literal final compiler error.

The leading-error parent temporarily writes a manifest with no target. Cargo prints its genuine
leading error before the multiline manifest path. The parent requires that leading physical line,
then restores the original manifest and builds successfully. This is a direct failed-Cargo control,
not an admitted public inventory.

| Exact edit | Actual finishing parent |
| --- | --- |
| `L676.eq.231` | `trust::tests::cargo_failure_diagnostics_keep_an_independent_target_alias` |
| `L685.iter.233` | `trust::tests::cargo_failure_diagnostics_keep_cwd_beside_a_separate_target` |
| `L717.clogic.1497` | `trust::tests::cargo_failure_diagnostics_keep_an_error_before_a_multiline_path` |

The unmodified three-parent baseline passes. The actual guarded native replay reports **three killed,
strict exit 0**, with precisely the finishing names above and no other outcome. The selected replay
establishes these regression kills; four proof/producer obligations remain before the complete current
region replay. It establishes no whole-file equivalent validity or F30 adequacy.

All 1056 original and current copied regular inputs restore, with modes and Python/system hashes.
The copy check borrows index metadata and makes no copied-index or submodule-content claim. Seven
false receipts refuse. Exact reconstruction removes only the added three-parent/helper test block
to recover every prior source byte. Production behavior, earlier tests, catalogue, operator controls
and existing reasons remain unchanged. Final held focused verification and separate book/staged
checks bind publication.


### Absolute diagnostic-prefix proofs

Proof slice `.14.3.2.2.2.2.5` (`2026-10-18`) resolves `L690.cond.244` and `L717.cmp.257`.
The argument covers every stderr string and environment value with absolute cwd on the supported
Unix callers. It uses path and line boundaries, independently of the producer's diagnostic vocabulary.

The caller chain supplies that absolute cwd. Inventory and baseline start from
[Cargo's](ledger.md#rust-toolchain) absolute manifest directory; the gate canonicalizes its selected
repository before deriving output. Inventory joins output with `tree`, and compile is the sole
production caller of `run_cargo`. The helper retains absolute targets or joins relative targets to cwd.
Raw/canonical spellings and their ancestors remain absolute. Lossy conversion and apostrophe quoting
preserve the leading slash.

The pinned [workspace constructor](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/workspace.rs)
rejects relative manifest paths. [Package roots](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/package.rs)
come from their manifest's parent; the held compiler setup exports that root as
`CARGO_MANIFEST_DIR`. The supported normal workspace build is the premise here. A permanent
control also compiles the actual repository-root function through a relative manifest argument and
directory alias, requiring its absolute value and physical identity.

The unconditional prefix edit adds only prefixes without newline. Such a match cannot straddle a
physical line start: doing so would include the preceding newline. If the match starts exactly at
the candidate, its slash disagrees with the candidate's `error` bytes. Added prefixes therefore
cannot suppress any candidate. For example, `/workspace` cannot begin at the start of
`error: actual diagnostic`, nor cross the newline immediately before it.

The strict-begin edit changes only equality between span begin and candidate start. Every recorded
prefix begins with slash, while a candidate begins with `error`. Equality is impossible. An original
containing span already begins strictly before the candidate, so both predicates select the same
diagnostic and return identical bytes.

| Exact edit | Universal premise | Native selected replay |
| --- | --- | --- |
| `L690.cond.244` | No-newline absolute prefixes cannot cover candidate line starts | equivalent |
| `L717.cmp.257` | A slash-prefixed span cannot begin at an error-prefixed candidate | equivalent |

The permanent operator control holds broad source-premise guards for the complete trust/gate
production slices and repository-root/CLI entry functions. A change there deliberately requires
revalidation. Actual original/two edited helpers compile, with literal diagnostics, aliases,
relative targets, both line endings and 4681 generated strings. Three altered source/caller bindings
refuse. Relative wider records distinguish both edits, and three wrong oracles fail. Finite records
support the source proof; they do not define its universe.

The complete operator suite passes **34 controls**. Only after the proof, two precise reasons are
added. The actual guarded selected replay reports **two equivalent, strict exit 0**, with no other
outcome and a passing original baseline. It does not validate unrelated reasons.

All 1058 original and current copied regular inputs restore with modes and Python/system hashes;
the copy borrows index metadata, and the submodule remains opaque. Six false receipt bindings refuse.
Every earlier source/test/reason byte is preserved; production, catalogue and native operator remain
unchanged. The two additional pinned sources join the earlier 28, all 30 independently hash-checked.
Revalidate after changes to guarded callers/root, constructors, conversion, framing/predicates or
Unix/toolchain contracts. Relative cwd is outside these two reasons. The shorter error literal and
closed span upper bound remain open before the complete current replay.


### Remaining diagnostic-producer assessment

Assessment `.14.3.2.2.2.2.6.1` (`2026-10-18`) tests the final literal and upper-bound edits
with actual [Cargo](ledger.md#rust-toolchain) output. It makes no new native classification.

A quoted unused manifest key can contain an escaped newline. Cargo emits the decoded key in a
warning before the compiler failure. A continuation `erroneous-warning` distinguishes the shortened
`erro` literal: the original retains the compiler error, while the shorter edit selects warning text.
This is a direct failed-command helper witness, like the earlier missing-target manifest control.
It is **outside the inventory's admitted manifest grammar**.

The actual maintained parser refuses both quoted multiline-key fixtures; a bare-key positive parses.
Inventory treats parse refusal separately from admissions and returns before program compilation.
A path-shaped quoted-key continuation also makes the original select warning data, while the
closed-end edit happens to mask it. That wider example establishes neither an admitted/public defect
nor a general repair. The initial suspected defect is narrowed by the verified syntax fence.

An ordinary-manifest case independently distinguishes the upper bound. Its containing configuration
has both file spellings, so Cargo prints a warning ending with the preferred configuration path,
a closing backtick and a newline. A distinct output ancestor is named to match those exact bytes.
Its target layout contains a colon, producing the existing genuine Unix library-path joining refusal
immediately after that warning.

    warning: both ... exist. Using `<repo>/.cargo/config`
    error: failed to join paths from `$…` together

The known ancestor spelling is `<repo>/.cargo/config` followed by a backtick and newline.
The text match ends exactly where the real error starts. The original open upper bound retains that
error; the closed bound suppresses it and returns empty. These paths are distinct filesystem objects:
the collision involves path spelling plus Cargo's framing bytes. This uses ordinary manifest and
configuration text with inventory-shaped output/cwd/target paths; permanent inventory reproduction
is the next parent, so this assessment claims no actual public-gate run.

| Edit | Real producer witness | Next parent |
| --- | --- | --- |
| `L714.lit.253` | Quoted-key continuation distinguishes the direct helper; inventory syntax refuses the manifest | Failed-command helper parent with explicit syntax-refusal boundary |
| `L717.cmp.258` | Configuration-warning terminator matches an ancestor and edited bound suppresses following setup error | Inventory boundary/operational parent with success and recovery |

A lexical-normalization alias hypothesis is refuted. Cargo retains its raw `/./` target spelling;
both error-like path controls agree for original and both edits. These are negative controls.

All 1058 held regular inputs restore with modes and Python/system hashes; five false bindings refuse.
Code, operator, catalogue and existing reasons remain unchanged. The read-only existing-commit source
archive has 4639 entries, including 2906 regular files verified byte-for-byte. All 30 prior source
pins agree with their archive copies and system hashes. No other Git checkout, dependency build
or source amendment is made. The cache binds evidence; no complete producer grammar or whole-file
adequacy is claimed.

Fixture metadata stores repository-relative bases. Current file access derives the root at runtime;
the audit derives historical text prefixes from the retained raw Cargo records. A different root
spelling resolves the same files, and the metadata correction preserves every raw diagnostic byte.


### Final diagnostic regression parents

Regression slice `.14.3.2.2.2.2.6.2` (`2026-10-18`) makes both remaining producer controls
permanent, using actual [Cargo](ledger.md#rust-toolchain) and bounded isolated parents.

The quoted-key parent starts with an ordinary successful inventory. It then commits a Cargo-valid
quoted key whose decoded name contains a newline and `erroneous-warning`. The inventory must refuse
the manifest syntax before creating any program target directory, and write no inventory. A direct
Cargo build succeeds and emits the expected warning. After a real type error is introduced, the
command helper must retain the actual compiler error and structured E0308. Restoring ordinary
source/manifest bytes makes both the direct command and inventory succeed. This guards the direct
failed-command contract without admitting quoted keys to inventory.

The boundary parent begins and ends with successful ordinary inventories. Its alternate output
lies below a directory whose spelling matches the containing configuration warning's final path,
closing backtick and newline. A colon in the target layout produces the genuine operational failure.
The actual inventory must retain the Cargo path-joining error for both roots and write no inventory.
The parent verifies that the legacy configuration is a file and the matching ancestor is a distinct
directory. It explicitly authorizes only the existing fixture's containing configuration root.

| Exact edit | Actual finishing parent |
| --- | --- |
| `L714.lit.253` | `trust::tests::cargo_failure_diagnostics_keep_the_error_after_a_quoted_key_warning` |
| `L717.cmp.258` | `trust::tests::cargo_failure_diagnostics_keep_the_error_at_a_config_warning_boundary` |

Both unmodified parents pass. The actual guarded selected native replay reports **two killed,
strict exit 0**, with precisely those finishing names and no other outcome. All 1060 original and
current copied regular inputs restore with modes and Python/system hashes; seven altered receipts
refuse. The copy borrows index metadata and makes no copied-index/submodule-content claim.

Exact reconstruction removes only the added test functions to recover every prior source byte.
Production behavior, earlier tests, operators, catalogue and existing reasons remain unchanged.
Final held focused verification and book/staged checks bind publication. This resolves the final
individual survivor obligations; the complete current region replay and full integration checkpoint
remain separate committed slices. Historical results are not added together as a substitute for
that new measurement, and no whole-file adequacy follows yet.


### Complete diagnostic-region replay

Replay `.14.4.1` (`2026-10-18`) freshly judges every current `cargo_error_line`/`run_cargo` edit
against the controlling full parent population. This is an actual new run, rather than the sum of
earlier partial outcomes. The unmodified copied tree passes; original and restored copied test
lists select the same **153 tests**, with route tests skipped and no ignored parent attributes.

| Scope/result | Actual observation |
| --- | --- |
| Global production listing | 1526 edits listed |
| Complete diagnostic region | 51 edits attempted |
| Named finishing kills | 43 |
| Proven equivalents | 4 |
| Compiler refusals | 4 |
| Survivors, timeouts, unjudged | 0 each |
| Strict native exit | 0 |

This local native-host run uses the [pinned toolchain](ledger.md#rust-toolchain). Source identity is
`069d8e598fd9d6c12306fbe0f091635b60f942b747e78e6167e92b41e0ca01ba`; the held Git activation tree is `6306e488b8c4e06ce5fc1b39acb53c74586b3028`.
Later completion documents receive separate book/staged checks. These identities do not claim that
the later document tree itself was the native input or that a full integration stamp was produced.

Each row below is freshly observed. One actual finishing parent is shown for each kill; the native
receipt retains every reported failing name. Compiler refusals execute no mutant tests and establish
no coverage. Equivalent rows retain their individual source proofs and revalidation conditions.

| Exact edit | Change | Outcome | One actual finishing parent or precise reason |
| --- | --- | --- | --- |
| `L676.eq.231` | `if let Some((_, target)) = env.iter().rfind(\|(key, _)\| key == "CARGO_TARGET_DIR") {` → `if let Some((_, target)) = env.iter().rfind(\|(key, _)\| key != "CARGO_TARGET_DIR") {` | killed | `trust::tests::cargo_failure_diagnostics_keep_an_independent_target_alias` |
| `L685.iter.232` | Visit first positions of `for path in paths` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_target_paths` |
| `L685.iter.233` | Visit last positions of `for path in paths` | killed | `trust::tests::cargo_failure_diagnostics_keep_cwd_beside_a_separate_target` |
| `L685.iter.234` | Visit first and last positions of `for path in paths` | equivalent | [At-most-two path vector](#finite-cargo-path-iterator-proofs) |
| `L687.iter.235` | Visit first positions of `for spelling in std::iter::once(path).chain(canonical)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L687.iter.236` | Visit last positions of `for spelling in std::iter::once(path).chain(canonical)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L687.iter.237` | Visit first and last positions of `for spelling in std::iter::once(path).chain(canonical)` | equivalent | [At-most-two spelling chain](#finite-cargo-path-iterator-proofs) |
| `L688.iter.238` | Visit first positions of `for prefix in spelling.ancestors()` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.iter.239` | Visit last positions of `for prefix in spelling.ancestors()` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.iter.240` | Visit first and last positions of `for prefix in spelling.ancestors()` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.guard.241` | Take newline-prefix guard only at first ancestor positions in `for prefix in spelling.ancestors()` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.guard.242` | Take newline-prefix guard only at last ancestor positions in `for prefix in spelling.ancestors()` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L688.guard.243` | Take newline-prefix guard only at first and last ancestor positions in `for prefix in spelling.ancestors()` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L690.cond.244` | `if prefix.contains('\n') {` → `if true {` | equivalent | [Absolute no-newline prefix](#absolute-diagnostic-prefix-proofs) |
| `L690.cond.245` | `if prefix.contains('\n') {` → `if false {` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L691.stmt.246` | `prefixes.insert(prefix.replace('\'', "'\\''"));` → `// prefixes.insert(prefix.replace('\'', "'\\''"));` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_output_paths` |
| `L692.stmt.247` | `prefixes.insert(prefix.into_owned());` → `// prefixes.insert(prefix.into_owned());` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_output_paths` |
| `L706.iter.248` | Visit first positions of `for physical in stderr.split_inclusive('\n')` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L706.iter.249` | Visit last positions of `for physical in stderr.split_inclusive('\n')` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L706.iter.250` | Visit first and last positions of `for physical in stderr.split_inclusive('\n')` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L708.stmt.251` | `offset += physical.len();` → `// offset += physical.len();` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L714.dir.252` | `if line.starts_with("error")` → `if line.contains("error")` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L714.lit.253` | `if line.starts_with("error")` → `if line.starts_with("erro")` | killed | `trust::tests::cargo_failure_diagnostics_keep_the_error_after_a_quoted_key_warning` |
| `L714.lit.254` | `if line.starts_with("error")` → `if line.starts_with("rror")` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L715.logic.255` | `&& !path_spans` → `\|\| !path_spans` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L715.not.256` | `&& !path_spans` → `&& path_spans` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L717.cmp.257` | `.any(\|&(begin, end)\| begin <= start && start < end)` → `.any(\|&(begin, end)\| begin < start && start < end)` | equivalent | [Absolute prefix cannot begin at error](#absolute-diagnostic-prefix-proofs) |
| `L717.cmp.258` | `.any(\|&(begin, end)\| begin <= start && start < end)` → `.any(\|&(begin, end)\| begin <= start && start <= end)` | killed | `trust::tests::cargo_failure_diagnostics_keep_the_error_at_a_config_warning_boundary` |
| `L717.logic.259` | `.any(\|&(begin, end)\| begin <= start && start < end)` → `.any(\|&(begin, end)\| begin <= start \|\| start < end)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L732.stmt.260` | `c.args(args).current_dir(cwd).env_clear();` → `// c.args(args).current_dir(cwd).env_clear();` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L733.iter.261` | Visit first positions of `for (k, v) in env` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L733.iter.262` | Visit last positions of `for (k, v) in env` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L733.iter.263` | Visit first and last positions of `for (k, v) in env` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L734.stmt.264` | `c.env(k, v);` → `// c.env(k, v);` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L738.stmt.265` | `c.env("RUSTC", "rustc");` → `// c.env("RUSTC", "rustc");` | killed | `trust::tests::cargo_diagnostic_flags_do_not_enter_unit_configurations` |
| `L742.cond.266` | `if !out.status.success() {` → `if true {` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L742.cond.267` | `if !out.status.success() {` → `if false {` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L742.not.268` | `if !out.status.success() {` → `if out.status.success() {` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L676.chain.1145` | `if let Some((_, target)) = env.iter().rfind(\|(key, _)\| key == "CARGO_TARGET_DIR") {` → `if let Some((_, target)) = env.iter().take(1).rfind(\|(key, _)\| key == "CARGO_TARGET_DIR") {` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_target_paths` |
| `L676.chain.1146` | `if let Some((_, target)) = env.iter().rfind(\|(key, _)\| key == "CARGO_TARGET_DIR") {` → `if let Some((_, target)) = env.iter().rev().take(1).rfind(\|(key, _)\| key == "CARGO_TARGET_DIR") {` | killed | `trust::tests::cargo_failure_diagnostics_ignore_multiline_target_paths` |
| `L687.chain.1147` | `for spelling in std::iter::once(path).chain(canonical) {` → `for spelling in std::iter::once(path)/* chain dropped */ {` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L690.contains.1148` | `if prefix.contains('\n') {` → `if ((prefix).first() == Some('\n')) {` | compiler-refused | E0599: `Cow<'_, str>` has no first method |
| `L690.contains.1149` | `if prefix.contains('\n') {` → `if ((prefix).first().copied() == Some('\n')) {` | compiler-refused | E0599: `Cow<'_, str>` has no first method |
| `L690.contains.1150` | `if prefix.contains('\n') {` → `if ((prefix).last() == Some('\n')) {` | compiler-refused | E0599: `Cow<'_, str>` is not an iterator |
| `L690.contains.1151` | `if prefix.contains('\n') {` → `if ((prefix).last().copied() == Some('\n')) {` | compiler-refused | E0599: `Cow<'_, str>` is not an iterator |
| `L698.chain.1152` | `.iter()` → `.iter().take(1)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L698.chain.1153` | `.iter()` → `.iter().rev().take(1)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L716.chain.1154` | `.iter()` → `.iter().take(1)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L716.chain.1155` | `.iter()` → `.iter().rev().take(1)` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |
| `L717.clogic.1497` | `.any(\|&(begin, end)\| begin <= start && start < end)` → `.any(\|&(begin, end)\|  start < end)` | killed | `trust::tests::cargo_failure_diagnostics_keep_an_error_before_a_multiline_path` |
| `L717.clogic.1498` | `.any(\|&(begin, end)\| begin <= start && start < end)` → `.any(\|&(begin, end)\| begin <= start )` | killed | `trust::tests::cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings` |

The per-operator totals independently agree with every attempted edit:

| Operator | Attempted | Compiler-refused |
| --- | ---: | ---: |
| eq | 1 | 0 |
| iter | 15 | 0 |
| guard | 3 | 0 |
| cond | 4 | 0 |
| stmt | 6 | 0 |
| dir | 1 | 0 |
| lit | 2 | 0 |
| logic | 2 | 0 |
| not | 2 | 0 |
| cmp | 2 | 0 |
| chain | 7 | 0 |
| contains | 4 | 4 |
| clogic | 2 | 0 |

Only the four applicable reason records are judged here. The per-file record retains 26 entries;
this bounded replay does not validate the rest. All five newly added finishing parents remain named
kills in the full population. The source/caller/environment guards stay exact, all 30 pinned source
copies independently hash-check, and Cargo/rustc metadata agrees before and after the run. Metadata
is not a binary-digest attestation.

All 1061 original and current copied regular inputs restore with modes and Python/system hashes.
The restored copied tree is then listed again, proving the same parent population and unchanged
copied inputs. Copied index metadata is borrowed; the submodule supplies only opaque index identity.
Ten altered receipts refuse: wrong exit, missing/duplicate row, foreign ID, wrong replacement,
missing/foreign finishing name, unlisted equivalent, wrong compiler-refusal reason and wrong
operator total.

To repeat this dated selection, prepare the local shell stores and the evidenced installed-host
write guard from the earlier [preflight](#run_cargo-locality-preflight) first. Check the
source identity and re-discover/rebind IDs after source changes; do not read the global listing as
an attempted whole-file sweep. The actual full parent command and complete selection are:

```bash
/usr/bin/sandbox-exec -D WRITABLE_ROOT="$PWD" \
  -f target/run-cargo-preflight/mutation-volume.sb \
  bash scripts/mutation_sweep.sh xtask/src/trust.rs --package xtask \
    --timeout 600 --filter=trust:: --filter=trust_generated:: \
    --filter=trust_gate:: --filter=--skip --filter=route_tests \
    --timeouts-fail --equivalents xtask/trust.equivalents \
  --only L676.eq.231 \
  --only L685.iter.232 \
  --only L685.iter.233 \
  --only L685.iter.234 \
  --only L687.iter.235 \
  --only L687.iter.236 \
  --only L687.iter.237 \
  --only L688.iter.238 \
  --only L688.iter.239 \
  --only L688.iter.240 \
  --only L688.guard.241 \
  --only L688.guard.242 \
  --only L688.guard.243 \
  --only L690.cond.244 \
  --only L690.cond.245 \
  --only L691.stmt.246 \
  --only L692.stmt.247 \
  --only L706.iter.248 \
  --only L706.iter.249 \
  --only L706.iter.250 \
  --only L708.stmt.251 \
  --only L714.dir.252 \
  --only L714.lit.253 \
  --only L714.lit.254 \
  --only L715.logic.255 \
  --only L715.not.256 \
  --only L717.cmp.257 \
  --only L717.cmp.258 \
  --only L717.logic.259 \
  --only L732.stmt.260 \
  --only L733.iter.261 \
  --only L733.iter.262 \
  --only L733.iter.263 \
  --only L734.stmt.264 \
  --only L738.stmt.265 \
  --only L742.cond.266 \
  --only L742.cond.267 \
  --only L742.not.268 \
  --only L676.chain.1145 \
  --only L676.chain.1146 \
  --only L687.chain.1147 \
  --only L690.contains.1148 \
  --only L690.contains.1149 \
  --only L690.contains.1150 \
  --only L690.contains.1151 \
  --only L698.chain.1152 \
  --only L698.chain.1153 \
  --only L716.chain.1154 \
  --only L716.chain.1155 \
  --only L717.clogic.1497 \
  --only L717.clogic.1498
```

The complete `trust.rs` sweep remains separate. The following checkpoint closes full local integration
and the diagnostic cohort; no remote CI, physical-board or whole-file adequacy claim follows here.


### Diagnostic cohort integration checkpoint

Checkpoint `.14.4.2` (`2026-10-18`) closes the diagnostic helper/caller cohort after the
[complete region replay](#complete-diagnostic-region-replay). The actual final source retains that
51-edit result: 43 named kills, four exact supported-domain equivalents and four compiler refusals,
with strict exit 0 and the full 153-test parent population. This integration run does not replace or
add mutation outcomes.

| Local integration step | Result |
| --- | --- |
| format | passed |
| strict lint | passed |
| complete contract suite | passed |
| doctrines | passed |
| gate self-tests | passed |
| pinned [mdBook](ledger.md#mdbook) | passed |
| no_std build | passed |
| pinned toolchain premises | passed |
| wasm build | passed |
| wasm binding | passed |
| emulator configuration | passed |
| target spike | passed |

```text
12 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined
activation tree: 5fc854ec08a322af168a2615934a988a6e9e1b47
```

The actual log and exit, official tier stamp and native Git identity agree. The maintained input
census restores all 1062 held regular inputs, including modes and Python/system hashes; the gitlink
is an opaque index identity. Five altered bindings refuse: nonzero exit, missing step, false summary,
wrong stamp and missing restoration. Final source/operator/catalogue/reason bytes remain exact,
the shared environment constructor and three-name compiled-input allowlist remain unchanged, and all
30 pinned source copies and tool metadata revalidate. The permanent operator suite has 34 controls.
Tool metadata is not executable-digest evidence.

Completion prose is a later document epoch with separate pinned-book and staged doctrine checks.
The integration stamp names the activation above; it does not claim this later prose was itself
the full tier's input. This closes the bounded diagnostic cohort, including its repaired caller and
framing defect, exact reasons and regression gaps. The 26-record equivalent file has only the four
applicable diagnostic reasons judged by this region run. Full-file adequacy, remote CI and physical-board
execution remain separate.

The next action is a survivor-ownership reconciliation, rather than the final whole-file sweep.
The `2026-10-18` task-tree census finds 463 historical rows still assigned to the general assessment
parent. This is an ownership-table observation, not a fresh count of current survivors: later repairs
may have changed or already covered some rows. Task `.15.1` binds every historical row to current
raw edits, retired edits or completed evidence, then schedules the next bounded assessment. The
controlling final-sweep prerequisite remains all required assessments and repairs complete.


### Survivor ownership reconciliation

The `2026-10-18` census (`.15.1`) reconciles every survivor from the initial complete
`trust.rs` measurement at `36154d6`. Native history remains 802 killed, 540 survived and
78 compiler refusals, with strict exit 1. The current production listing contains 1526 candidates.
Mapping old text to current text supplies no new mutation verdict or equivalence.

| Source-identity observation | Initial survivor rows |
| --- | ---: |
| Complete unchanged source span and exact edit retained | 512 |
| No current exact edit | 28 |
| Ambiguous mapping | 0 |
| Bound to a completed actual measurement by current raw edit | 70 |

The fresh-name portion of a generated iterator identifier contains its source line. The constructor
is unchanged and that prefix is absent from both complete sources; only its line number is normalized
when comparing replacements. The original fragments and every other replacement byte remain exact.
All retained rows also lie in complete unchanged source blocks. A missing exact edit means its old
text disappeared, not that its replacement behavior or new candidates pass.

For example, the old inline command framing in `compile` moved to the repaired `cargo_commands`
and `dependency_files` path. Some old edits therefore disappear, while retained ownership/configuration
edits still require assessment. Their replacement helpers have their own current mutation shapes.
The old `run_cargo` selector edits are covered by its separately documented repair and fresh complete
region replay; the initial raw outcomes remain historical.

The census finds 463 rows still assigned to the general parent before this update. They now all have
the concrete lanes below. Of these, 442 retain an exact current edit and 21 lack an exact current edit;
neither category is declared a current survivor or a pass. The completed 77-row families retain their
previous owners; 70 have a retained raw edit and seven lack one.

| Next assessment lane | Source scope | Historical rows assigned | Current candidates in the full lane |
| --- | --- | ---: | ---: |
| `.15.2` — Compiler-input path | `compile` plus `cargo_commands` and `dependency_files` | 48 | 159 |
| `.15.3` — Build artifacts | `build` | 21 | 34 |
| `.15.4` — Source extents | `refused_sites`, `site_extent` | 24 | 90 |
| `.15.5` — Compiled rule names | `inadmissible`, `compiled_names` | 18 | 42 |
| `.15.6` — Tool identity output | `tool_line` | 2 | 5 |
| `.15.7` — Target and program closure | `target_closure`, `classify_programs` | 30 | 90 |
| `.15.8` — Refusal and admission state | `refuse`, `Rules` | 60 | 135 |
| `.15.9` — Inventory orchestration | `inventory_with` | 237 | 688 |
| `.15.10` — Trust command | `run` | 15 | 15 |
| `.15.11` — Harness-relative units | `beside` | 8 | 15 |

The following measurement covers the complete current `compile` function (`.15.2.1.1`):
83 candidates against the full parent command, with strict timeout/unjudged rules and whole-input
restoration. Its command-record and dependency helpers follow as separate bounded children; actual
survivors gain controls, owned repairs or exact proofs before final complete replay. The remaining
lanes follow the same discipline. All required assessments precede the final whole-file sweep.

The maintained historical native auditor and actual current native listing agree with the census.
Independent old/current source-span and complete replacement checks cover every row; six altered
censuses refuse (missing/duplicate row, foreign current ID, wrong replacement, false retirement and
wrong ownership count). All 1062 held regular inputs restore before these later completion documents.
No Rust, operator, catalogue or equivalent reason changes. The closed diagnostic task subtree is
sealed with exact reconstruction and all previous archives preserved; the book remains unbounded.

The following tables expose every initial row, its current raw identity or its absence, the bound
historical verdict when one is available, and its assessment owner. Owner suffixes are under
`M3.6.9.6.3.2.2`; older families keep their previously completed records. “No bound result here”
does not erase earlier evidence or infer a failure: the census only binds the explicitly audited
receipt set. Every final current outcome still requires its actual native run.

#### Initial `read_roots` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L185.cond.2` | `L185.cond.2` | killed (historical) | `.1` |

#### Initial `read_form` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L235.iter.25` | `L238.iter.25` | killed (historical) | `.2` |
| `L235.iter.26` | `L238.iter.26` | killed (historical) | `.2` |
| `L235.guard.28` | `L238.guard.28` | killed (historical) | `.2` |
| `L235.guard.29` | `L238.guard.29` | killed (historical) | `.2` |
| `L235.guard.30` | `L238.guard.30` | killed (historical) | `.2` |
| `L235.guard.31` | `L238.guard.31` | killed (historical) | `.2` |
| `L235.guard.32` | `L238.guard.32` | killed (historical) | `.2` |
| `L264.stmt.42` | `L267.stmt.42` | killed (historical) | `.3` |
| `L317.stmt.47` | `L320.stmt.47` | killed (historical) | `.4` |
| `L263.chain.1020` | `L266.chain.1104` | killed (historical) | `.3` |
| `L312.chain.1029` | `L315.chain.1113` | killed (historical) | `.3` |
| `L312.chain.1030` | `L315.chain.1114` | killed (historical) | `.3` |

#### Initial `check_pairs` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L334.iter.51` | `L340.iter.52` | killed (historical) | `.7` |
| `L334.iter.52` | `L340.iter.53` | killed (historical) | `.7` |
| `L334.guard.53` | `L340.guard.54` | killed (historical) | `.7` |
| `L334.guard.54` | `L340.guard.55` | killed (historical) | `.7` |
| `L334.guard.55` | `L340.guard.56` | killed (historical) | `.7` |
| `L336.iter.58` | `L342.iter.59` | equivalent (historical) | `.8.2` |
| `L348.cond.64` | `L354.cond.65` | killed (historical) | `.7` |
| `L340.clogic.1392` | `L346.clogic.1496` | killed (historical) | `.7` |

#### Initial `unit_configuration` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L376.iter.65` | no current exact edit | no bound result here | `.9.1` |
| `L376.iter.66` | no current exact edit | no bound result here | `.9.1` |
| `L376.iter.67` | no current exact edit | no bound result here | `.9.1` |
| `L377.stmt.68` | no current exact edit | no bound result here | `.9.1` |

#### Initial `split_command` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L388.bool.71` | `L393.bool.67` | equivalent (historical) | `.10.3.4` |
| `L392.not.78` | `L399.not.69` | killed (historical) | `.10.3.4` |
| `L392.stmt.79` | `L399.stmt.70` | killed (historical) | `.10.3.4` |
| `L393.bool.80` | `L400.bool.71` | killed (historical) | `.10.3.4` |
| `L393.stmt.81` | `L400.stmt.72` | killed (historical) | `.10.3.4` |
| `L396.logic.84` | `L403.logic.75` | killed (historical) | `.10.3.4` |
| `L396.cond.86` | `L403.cond.77` | equivalent (historical) | `.10.3.4` |
| `L399.bool.90` | `L406.bool.81` | equivalent (historical) | `.10.3.4` |
| `L399.stmt.91` | `L406.stmt.82` | equivalent (historical) | `.10.3.4` |
| `L404.logic.92` | `L411.logic.83` | killed (historical) | `.10.3.4` |
| `L404.logic.93` | `L411.logic.84` | killed (historical) | `.10.3.4` |
| `L404.logic.94` | `L411.logic.85` | killed (historical) | `.10.3.4` |
| `L404.cond.95` | `L411.cond.86` | equivalent (historical) | `.10.3.4` |
| `L404.cond.96` | `L411.cond.87` | killed (historical) | `.10.3.4` |
| `L404.not.97` | `L411.not.88` | killed (historical) | `.10.3.4` |
| `L405.stmt.98` | `L412.stmt.89` | killed (historical) | `.10.3.4` |

#### Initial `DROPPED` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L416.bool.101` | `L554.bool.158` | killed (historical) | `.11.2.2` |
| `L417.bool.102` | `L555.bool.159` | equivalent (historical) | `.11.2.4.2` |
| `L418.bool.103` | `L556.bool.160` | equivalent (historical) | `.11.2.4.1` |
| `L419.bool.104` | `L557.bool.161` | equivalent (historical) | `.11.2.4.1` |
| `L420.bool.105` | `L558.bool.162` | equivalent (historical) | `.11.2.4.1` |
| `L421.bool.106` | `L559.bool.163` | equivalent (historical) | `.11.2.4.1` |
| `L422.bool.107` | `L560.bool.164` | equivalent (historical) | `.11.2.4.1` |
| `L423.bool.108` | `L561.bool.165` | equivalent (historical) | `.11.2.4.1` |
| `L424.bool.109` | `L562.bool.166` | equivalent (historical) | `.11.2.4.1` |
| `L425.bool.110` | `L563.bool.167` | equivalent (historical) | `.11.2.4.1` |
| `L426.bool.111` | `L564.bool.168` | equivalent (historical) | `.11.2.4.3.2` |

#### Initial `normalise` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L452.cond.120` | `L590.cond.177` | killed (historical) | `.13.1` |
| `L452.dir.121` | `L590.dir.178` | killed (historical) | `.13.2.2` |
| `L453.int.122` | `L591.int.179` | killed (historical) | `.13.1` |
| `L454.stmt.125` | `L592.stmt.182` | killed (historical) | `.13.1` |
| `L457.int.129` | `L595.int.186` | killed (historical) | `.13.1` |
| `L457.int.130` | `L595.int.187` | killed (historical) | `.13.2.3` |
| `L459.dir.131` | `L597.dir.188` | equivalent/survived (historical) | `.13.2.4.2.1` |
| `L459.lit.132` | `L597.lit.189` | equivalent/survived (historical) | `.13.2.4.2.1` |
| `L460.dir.135` | `L598.dir.192` | equivalent/survived (historical) | `.13.2.4.2.2` |
| `L460.lit.136` | `L598.lit.193` | equivalent/survived (historical) | `.13.2.4.2.2` |
| `L461.dir.139` | `L599.dir.196` | equivalent/survived (historical) | `.13.2.4.2.3` |
| `L461.lit.140` | `L599.lit.197` | equivalent/survived (historical) | `.13.2.4.2.3` |
| `L466.stmt.146` | `L604.stmt.203` | killed (historical) | `.13.2.3` |
| `L467.int.148` | `L605.int.205` | killed (historical) | `.13.2.3` |
| `L472.int.155` | `L610.int.212` | killed (historical) | `.13.1` |
| `L473.dir.156` | `L611.dir.213` | killed (historical) | `.13.2.2` |
| `L474.dir.157` | `L612.dir.214` | killed (historical) | `.13.1` |
| `L476.cmp.158` | `L614.cmp.215` | equivalent (historical) | `.13.2.5` |
| `L481.stmt.159` | `L619.stmt.216` | killed (historical) | `.13.1` |
| `L452.chain.1035` | `L590.chain.1129` | killed (historical) | `.13.1` |
| `L452.chain.1036` | `L590.chain.1130` | killed (historical) | `.13.1` |

#### Initial `run_cargo` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L541.iter.177` | `L733.iter.263` | killed (historical) | `.14` |
| `L550.dir.182` | no current exact edit | no bound result here | `.14` |
| `L550.lit.183` | no current exact edit | no bound result here | `.14` |
| `L550.lit.184` | no current exact edit | no bound result here | `.14` |

#### Initial `compile` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L606.guard.189` | no current exact edit | no bound result here | `.15.2` |
| `L606.guard.190` | no current exact edit | no bound result here | `.15.2` |
| `L606.guard.191` | no current exact edit | no bound result here | `.15.2` |
| `L610.int.193` | no current exact edit | no bound result here | `.15.2` |
| `L610.int.194` | no current exact edit | no bound result here | `.15.2` |
| `L612.cond.196` | `L799.cond.277` | no bound result here | `.15.2` |
| `L627.dir.203` | `L814.dir.284` | no bound result here | `.15.2` |
| `L627.lit.204` | `L814.lit.285` | no bound result here | `.15.2` |
| `L655.iter.210` | `L841.iter.290` | no bound result here | `.15.2` |
| `L655.guard.212` | `L841.guard.292` | no bound result here | `.15.2` |
| `L655.guard.213` | `L841.guard.293` | no bound result here | `.15.2` |
| `L655.guard.214` | no current exact edit | no bound result here | `.15.2` |
| `L655.guard.215` | no current exact edit | no bound result here | `.15.2` |
| `L655.guard.216` | no current exact edit | no bound result here | `.15.2` |
| `L659.dir.224` | `L845.dir.301` | no bound result here | `.15.2` |
| `L659.lit.225` | `L845.lit.302` | no bound result here | `.15.2` |
| `L667.stmt.228` | `L857.stmt.304` | no bound result here | `.15.2` |
| `L669.logic.229` | no current exact edit | no bound result here | `.15.2` |
| `L669.logic.230` | no current exact edit | no bound result here | `.15.2` |
| `L669.logic.231` | no current exact edit | no bound result here | `.15.2` |
| `L669.cond.233` | no current exact edit | no bound result here | `.15.2` |
| `L670.stmt.235` | `L871.stmt.314` | no bound result here | `.15.2` |
| `L672.bool.236` | no current exact edit | no bound result here | `.15.2` |
| `L672.stmt.237` | no current exact edit | no bound result here | `.15.2` |
| `L673.dir.238` | no current exact edit | no bound result here | `.15.2` |
| `L676.dir.241` | no current exact edit | no bound result here | `.15.2` |
| `L676.guard.245` | no current exact edit | no bound result here | `.15.2` |
| `L676.guard.246` | no current exact edit | no bound result here | `.15.2` |
| `L676.guard.247` | no current exact edit | no bound result here | `.15.2` |
| `L683.cond.249` | no current exact edit | no bound result here | `.15.2` |
| `L703.iter.253` | `L885.iter.316` | no bound result here | `.15.2` |
| `L703.iter.254` | `L885.iter.317` | no bound result here | `.15.2` |
| `L703.iter.255` | `L885.iter.318` | no bound result here | `.15.2` |
| `L703.guard.256` | `L885.guard.319` | no bound result here | `.15.2` |
| `L703.guard.257` | `L885.guard.320` | no bound result here | `.15.2` |
| `L703.guard.258` | `L885.guard.321` | no bound result here | `.15.2` |
| `L704.logic.259` | `L886.logic.322` | no bound result here | `.15.2` |
| `L704.logic.260` | `L886.logic.323` | no bound result here | `.15.2` |
| `L704.logic.261` | `L886.logic.324` | no bound result here | `.15.2` |
| `L704.cond.263` | `L886.cond.326` | no bound result here | `.15.2` |
| `L729.cond.269` | `L911.cond.332` | no bound result here | `.15.2` |
| `L639.chain.1057` | `L826.chain.1162` | no bound result here | `.15.2` |
| `L639.chain.1058` | `L826.chain.1163` | no bound result here | `.15.2` |
| `L659.contains.1059` | `L845.contains.1164` | no bound result here | `.15.2` |
| `L659.contains.1060` | `L845.contains.1165` | no bound result here | `.15.2` |
| `L659.contains.1061` | `L845.contains.1166` | no bound result here | `.15.2` |
| `L659.contains.1062` | `L845.contains.1167` | no bound result here | `.15.2` |
| `L676.chain.1065` | no current exact edit | no bound result here | `.15.2` |

#### Initial `build` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L768.guard.274` | `L950.guard.337` | no bound result here | `.15.3` |
| `L768.guard.275` | `L950.guard.338` | no bound result here | `.15.3` |
| `L768.guard.276` | `L950.guard.339` | no bound result here | `.15.3` |
| `L768.guard.277` | `L950.guard.340` | no bound result here | `.15.3` |
| `L768.guard.278` | `L950.guard.341` | no bound result here | `.15.3` |
| `L768.guard.279` | `L950.guard.342` | no bound result here | `.15.3` |
| `L770.cond.282` | `L952.cond.345` | no bound result here | `.15.3` |
| `L771.stmt.283` | `L953.stmt.346` | no bound result here | `.15.3` |
| `L777.cond.286` | `L959.cond.349` | no bound result here | `.15.3` |
| `L778.stmt.287` | `L960.stmt.350` | no bound result here | `.15.3` |
| `L795.logic.290` | `L977.logic.353` | no bound result here | `.15.3` |
| `L798.logic.293` | `L980.logic.356` | no bound result here | `.15.3` |
| `L789.chain.1072` | `L971.chain.1174` | no bound result here | `.15.3` |
| `L789.chain.1073` | `L971.chain.1175` | no bound result here | `.15.3` |
| `L795.chain.1074` | `L977.chain.1176` | no bound result here | `.15.3` |
| `L795.chain.1075` | `L977.chain.1177` | no bound result here | `.15.3` |
| `L798.chain.1076` | `L980.chain.1178` | no bound result here | `.15.3` |
| `L798.chain.1077` | `L980.chain.1179` | no bound result here | `.15.3` |
| `L801.chain.1078` | `L983.chain.1180` | no bound result here | `.15.3` |
| `L801.chain.1079` | `L983.chain.1181` | no bound result here | `.15.3` |
| `L805.chain.1080` | `L987.chain.1182` | no bound result here | `.15.3` |

#### Initial `refused_sites` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L835.guard.298` | `L1017.guard.361` | no bound result here | `.15.4` |
| `L835.guard.299` | `L1017.guard.362` | no bound result here | `.15.4` |
| `L835.guard.300` | `L1017.guard.363` | no bound result here | `.15.4` |
| `L848.int.305` | `L1030.int.368` | no bound result here | `.15.4` |
| `L852.cmp.307` | `L1034.cmp.370` | no bound result here | `.15.4` |
| `L853.eq.308` | `L1035.eq.371` | no bound result here | `.15.4` |
| `L855.int.311` | `L1037.int.374` | no bound result here | `.15.4` |
| `L857.cond.316` | `L1039.cond.379` | no bound result here | `.15.4` |
| `L860.iter.317` | `L1042.iter.380` | no bound result here | `.15.4` |
| `L860.iter.318` | `L1042.iter.381` | no bound result here | `.15.4` |
| `L860.iter.319` | `L1042.iter.382` | no bound result here | `.15.4` |

#### Initial `site_extent` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L911.cond.340` | `L1093.cond.403` | no bound result here | `.15.4` |
| `L924.int.345` | `L1106.int.408` | no bound result here | `.15.4` |
| `L931.cmp.347` | `L1113.cmp.410` | no bound result here | `.15.4` |
| `L933.logic.351` | `L1115.logic.414` | no bound result here | `.15.4` |
| `L951.cmp.361` | `L1133.cmp.424` | no bound result here | `.15.4` |
| `L954.stmt.363` | `L1136.stmt.426` | no bound result here | `.15.4` |
| `L955.int.364` | `L1137.int.427` | no bound result here | `.15.4` |
| `L955.int.365` | `L1137.int.428` | no bound result here | `.15.4` |
| `L962.stmt.369` | `L1144.stmt.432` | no bound result here | `.15.4` |
| `L963.int.371` | `L1145.int.434` | no bound result here | `.15.4` |
| `L970.stmt.375` | `L1152.stmt.438` | no bound result here | `.15.4` |
| `L917.chain.1086` | `L1099.chain.1188` | no bound result here | `.15.4` |
| `L917.chain.1087` | `L1099.chain.1189` | no bound result here | `.15.4` |

#### Initial `inadmissible` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L996.bool.382` | `L1178.bool.445` | no bound result here | `.15.5` |
| `L999.eq.383` | `L1181.eq.446` | no bound result here | `.15.5` |
| `L999.logic.385` | `L1181.logic.448` | no bound result here | `.15.5` |
| `L994.chain.1088` | `L1176.chain.1190` | no bound result here | `.15.5` |
| `L998.chain.1091` | `L1180.chain.1193` | no bound result here | `.15.5` |
| `L989.clogic.1393` | `L1171.clogic.1499` | no bound result here | `.15.5` |
| `L995.clogic.1396` | `L1177.clogic.1502` | no bound result here | `.15.5` |
| `L999.clogic.1397` | `L1181.clogic.1503` | no bound result here | `.15.5` |
| `L999.clogic.1398` | `L1181.clogic.1504` | no bound result here | `.15.5` |

#### Initial `compiled_names` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1014.cmp.387` | `L1196.cmp.450` | no bound result here | `.15.5` |
| `L1019.guard.393` | `L1201.guard.456` | no bound result here | `.15.5` |
| `L1019.guard.394` | `L1201.guard.457` | no bound result here | `.15.5` |
| `L1019.guard.395` | `L1201.guard.458` | no bound result here | `.15.5` |
| `L1020.cond.398` | `L1202.cond.461` | no bound result here | `.15.5` |
| `L1021.stmt.400` | `L1203.stmt.463` | no bound result here | `.15.5` |
| `L1026.eq.401` | `L1208.eq.464` | no bound result here | `.15.5` |
| `L1028.int.402` | `L1210.int.465` | no bound result here | `.15.5` |
| `L1028.int.403` | `L1210.int.466` | no bound result here | `.15.5` |

#### Initial `tool_line` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1073.iter.410` | `L1255.iter.473` | no bound result here | `.15.6` |
| `L1073.iter.412` | `L1255.iter.475` | no bound result here | `.15.6` |

#### Initial `target_closure` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1098.stmt.417` | `L1280.stmt.480` | no bound result here | `.15.7` |
| `L1100.iter.419` | `L1282.iter.482` | no bound result here | `.15.7` |
| `L1100.iter.420` | `L1282.iter.483` | no bound result here | `.15.7` |
| `L1100.guard.422` | `L1282.guard.485` | no bound result here | `.15.7` |
| `L1100.guard.423` | `L1282.guard.486` | no bound result here | `.15.7` |
| `L1103.logic.435` | `L1285.logic.498` | no bound result here | `.15.7` |
| `L1103.logic.437` | `L1285.logic.500` | no bound result here | `.15.7` |
| `L1103.contains.1098` | `L1285.contains.1200` | no bound result here | `.15.7` |
| `L1103.contains.1100` | `L1285.contains.1202` | no bound result here | `.15.7` |

#### Initial `classify_programs` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1128.logic.453` | `L1310.logic.516` | no bound result here | `.15.7` |
| `L1129.logic.455` | `L1311.logic.518` | no bound result here | `.15.7` |
| `L1139.logic.460` | `L1321.logic.523` | no bound result here | `.15.7` |
| `L1118.chain.1102` | `L1300.chain.1204` | no bound result here | `.15.7` |
| `L1119.chain.1104` | `L1301.chain.1206` | no bound result here | `.15.7` |
| `L1119.chain.1106` | `L1301.chain.1208` | no bound result here | `.15.7` |
| `L1120.chain.1107` | `L1302.chain.1209` | no bound result here | `.15.7` |
| `L1120.chain.1108` | `L1302.chain.1210` | no bound result here | `.15.7` |
| `L1125.chain.1109` | `L1307.chain.1211` | no bound result here | `.15.7` |
| `L1133.chain.1111` | `L1315.chain.1213` | no bound result here | `.15.7` |
| `L1134.chain.1113` | `L1316.chain.1215` | no bound result here | `.15.7` |
| `L1134.chain.1114` | `L1316.chain.1216` | no bound result here | `.15.7` |
| `L1134.contains.1117` | `L1316.contains.1219` | no bound result here | `.15.7` |
| `L1151.chain.1122` | `L1333.chain.1224` | no bound result here | `.15.7` |
| `L1151.chain.1123` | `L1333.chain.1225` | no bound result here | `.15.7` |
| `L1169.chain.1125` | `L1351.chain.1227` | no bound result here | `.15.7` |
| `L1170.chain.1126` | `L1352.chain.1228` | no bound result here | `.15.7` |
| `L1170.chain.1127` | `L1352.chain.1229` | no bound result here | `.15.7` |
| `L1139.clogic.1399` | `L1321.clogic.1505` | no bound result here | `.15.7` |
| `L1139.clogic.1400` | `L1321.clogic.1506` | no bound result here | `.15.7` |
| `L1139.clogic.1401` | `L1321.clogic.1507` | no bound result here | `.15.7` |

#### Initial `refuse` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1192.stmt.466` | `L1374.stmt.529` | no bound result here | `.15.8` |
| `L1193.stmt.467` | `L1375.stmt.530` | no bound result here | `.15.8` |

#### Initial `Rules` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1224.int.468` | `L1408.int.531` | no bound result here | `.15.8` |
| `L1224.int.469` | `L1408.int.532` | no bound result here | `.15.8` |
| `L1224.stmt.470` | `L1408.stmt.533` | no bound result here | `.15.8` |
| `L1230.cond.472` | `L1414.cond.535` | no bound result here | `.15.8` |
| `L1231.int.473` | `L1415.int.536` | no bound result here | `.15.8` |
| `L1231.int.474` | `L1415.int.537` | no bound result here | `.15.8` |
| `L1231.stmt.475` | `L1415.stmt.538` | no bound result here | `.15.8` |
| `L1249.guard.480` | `L1433.guard.543` | no bound result here | `.15.8` |
| `L1249.guard.481` | `L1433.guard.544` | no bound result here | `.15.8` |
| `L1249.guard.482` | `L1433.guard.545` | no bound result here | `.15.8` |
| `L1253.iter.486` | `L1437.iter.549` | no bound result here | `.15.8` |
| `L1253.iter.487` | `L1437.iter.550` | no bound result here | `.15.8` |
| `L1253.iter.488` | `L1437.iter.551` | no bound result here | `.15.8` |
| `L1261.cond.492` | `L1445.cond.555` | no bound result here | `.15.8` |
| `L1262.stmt.494` | `L1446.stmt.557` | no bound result here | `.15.8` |
| `L1278.int.495` | `L1472.int.574` | no bound result here | `.15.8` |
| `L1278.int.496` | `L1472.int.575` | no bound result here | `.15.8` |
| `L1278.stmt.497` | `L1472.stmt.576` | no bound result here | `.15.8` |
| `L1286.int.500` | `L1480.int.579` | no bound result here | `.15.8` |
| `L1308.guard.508` | `L1502.guard.587` | no bound result here | `.15.8` |
| `L1308.guard.509` | `L1502.guard.588` | no bound result here | `.15.8` |
| `L1308.guard.510` | `L1502.guard.589` | no bound result here | `.15.8` |
| `L1308.guard.511` | `L1502.guard.590` | no bound result here | `.15.8` |
| `L1308.guard.512` | `L1502.guard.591` | no bound result here | `.15.8` |
| `L1308.guard.513` | `L1502.guard.592` | no bound result here | `.15.8` |
| `L1308.guard.514` | `L1502.guard.593` | no bound result here | `.15.8` |
| `L1308.guard.515` | `L1502.guard.594` | no bound result here | `.15.8` |
| `L1313.cond.520` | `L1507.cond.599` | no bound result here | `.15.8` |
| `L1317.stmt.521` | `L1511.stmt.600` | no bound result here | `.15.8` |
| `L1325.cond.525` | `L1519.cond.604` | no bound result here | `.15.8` |
| `L1338.iter.529` | `L1532.iter.608` | no bound result here | `.15.8` |
| `L1363.guard.536` | `L1557.guard.615` | no bound result here | `.15.8` |
| `L1363.guard.537` | `L1557.guard.616` | no bound result here | `.15.8` |
| `L1363.guard.538` | `L1557.guard.617` | no bound result here | `.15.8` |
| `L1367.iter.542` | `L1561.iter.621` | no bound result here | `.15.8` |
| `L1367.iter.543` | `L1561.iter.622` | no bound result here | `.15.8` |
| `L1367.iter.544` | `L1561.iter.623` | no bound result here | `.15.8` |
| `L1373.eq.546` | `L1567.eq.625` | no bound result here | `.15.8` |
| `L1373.logic.549` | `L1567.logic.628` | no bound result here | `.15.8` |
| `L1373.cond.551` | `L1567.cond.630` | no bound result here | `.15.8` |
| `L1374.iter.553` | `L1568.iter.632` | no bound result here | `.15.8` |
| `L1374.iter.554` | `L1568.iter.633` | no bound result here | `.15.8` |
| `L1374.iter.555` | `L1568.iter.634` | no bound result here | `.15.8` |
| `L1375.iter.556` | `L1569.iter.635` | no bound result here | `.15.8` |
| `L1375.iter.557` | `L1569.iter.636` | no bound result here | `.15.8` |
| `L1375.iter.558` | `L1569.iter.637` | no bound result here | `.15.8` |
| `L1382.iter.564` | `L1576.iter.643` | no bound result here | `.15.8` |
| `L1382.iter.565` | `L1576.iter.644` | no bound result here | `.15.8` |
| `L1382.guard.566` | `L1576.guard.645` | no bound result here | `.15.8` |
| `L1382.guard.567` | `L1576.guard.646` | no bound result here | `.15.8` |
| `L1382.guard.568` | `L1576.guard.647` | no bound result here | `.15.8` |
| `L1383.cond.570` | `L1577.cond.649` | no bound result here | `.15.8` |
| `L1253.chain.1133` | `L1437.chain.1235` | no bound result here | `.15.8` |
| `L1253.chain.1134` | `L1437.chain.1236` | no bound result here | `.15.8` |
| `L1312.chain.1135` | `L1506.chain.1239` | no bound result here | `.15.8` |
| `L1312.chain.1136` | `L1506.chain.1240` | no bound result here | `.15.8` |
| `L1313.contains.1137` | `L1507.contains.1241` | no bound result here | `.15.8` |
| `L1313.contains.1139` | `L1507.contains.1243` | no bound result here | `.15.8` |

#### Initial `inventory_with` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L1435.eq.577` | `L1629.eq.657` | no bound result here | `.15.9` |
| `L1459.iter.584` | `L1653.iter.664` | no bound result here | `.15.9` |
| `L1459.iter.586` | `L1653.iter.666` | no bound result here | `.15.9` |
| `L1460.iter.587` | `L1654.iter.667` | no bound result here | `.15.9` |
| `L1460.iter.588` | `L1654.iter.668` | no bound result here | `.15.9` |
| `L1460.iter.589` | `L1654.iter.669` | no bound result here | `.15.9` |
| `L1460.guard.590` | `L1654.guard.670` | no bound result here | `.15.9` |
| `L1460.guard.591` | `L1654.guard.671` | no bound result here | `.15.9` |
| `L1460.guard.592` | `L1654.guard.672` | no bound result here | `.15.9` |
| `L1461.logic.594` | `L1655.logic.674` | no bound result here | `.15.9` |
| `L1494.cond.600` | `L1688.cond.680` | no bound result here | `.15.9` |
| `L1598.iter.613` | `L1792.iter.693` | no bound result here | `.15.9` |
| `L1598.iter.614` | `L1792.iter.694` | no bound result here | `.15.9` |
| `L1599.iter.617` | `L1793.iter.697` | no bound result here | `.15.9` |
| `L1599.guard.620` | `L1793.guard.700` | no bound result here | `.15.9` |
| `L1607.iter.624` | `L1801.iter.704` | no bound result here | `.15.9` |
| `L1607.iter.626` | `L1801.iter.706` | no bound result here | `.15.9` |
| `L1607.guard.627` | `L1801.guard.707` | no bound result here | `.15.9` |
| `L1607.guard.629` | `L1801.guard.709` | no bound result here | `.15.9` |
| `L1619.eq.632` | `L1813.eq.712` | no bound result here | `.15.9` |
| `L1619.logic.634` | `L1813.logic.714` | no bound result here | `.15.9` |
| `L1647.iter.645` | `L1841.iter.725` | no bound result here | `.15.9` |
| `L1652.iter.646` | `L1846.iter.726` | no bound result here | `.15.9` |
| `L1652.iter.647` | `L1846.iter.727` | no bound result here | `.15.9` |
| `L1652.iter.648` | `L1846.iter.728` | no bound result here | `.15.9` |
| `L1682.iter.654` | `L1877.iter.734` | no bound result here | `.15.9` |
| `L1687.stmt.658` | `L1882.stmt.738` | no bound result here | `.15.9` |
| `L1689.iter.659` | `L1884.iter.739` | no bound result here | `.15.9` |
| `L1689.iter.661` | `L1884.iter.741` | no bound result here | `.15.9` |
| `L1689.guard.662` | `L1884.guard.742` | no bound result here | `.15.9` |
| `L1689.guard.664` | `L1884.guard.744` | no bound result here | `.15.9` |
| `L1690.eq.666` | `L1885.eq.746` | no bound result here | `.15.9` |
| `L1690.logic.669` | `L1885.logic.749` | no bound result here | `.15.9` |
| `L1691.logic.671` | `L1886.logic.751` | no bound result here | `.15.9` |
| `L1691.logic.672` | `L1886.logic.752` | no bound result here | `.15.9` |
| `L1691.logic.673` | `L1886.logic.753` | no bound result here | `.15.9` |
| `L1691.logic.674` | `L1886.logic.754` | no bound result here | `.15.9` |
| `L1691.logic.675` | `L1886.logic.755` | no bound result here | `.15.9` |
| `L1691.cond.676` | `L1886.cond.756` | no bound result here | `.15.9` |
| `L1731.iter.693` | `L1926.iter.774` | no bound result here | `.15.9` |
| `L1732.iter.696` | `L1927.iter.777` | no bound result here | `.15.9` |
| `L1759.iter.710` | `L1954.iter.791` | no bound result here | `.15.9` |
| `L1759.iter.711` | `L1954.iter.792` | no bound result here | `.15.9` |
| `L1759.iter.712` | `L1954.iter.793` | no bound result here | `.15.9` |
| `L1759.guard.713` | `L1954.guard.794` | no bound result here | `.15.9` |
| `L1759.guard.714` | `L1954.guard.795` | no bound result here | `.15.9` |
| `L1759.guard.715` | `L1954.guard.796` | no bound result here | `.15.9` |
| `L1826.iter.729` | `L2021.iter.810` | no bound result here | `.15.9` |
| `L1827.iter.730` | `L2022.iter.811` | no bound result here | `.15.9` |
| `L1827.iter.731` | `L2022.iter.812` | no bound result here | `.15.9` |
| `L1827.iter.732` | `L2022.iter.813` | no bound result here | `.15.9` |
| `L1845.iter.739` | `L2040.iter.820` | no bound result here | `.15.9` |
| `L1857.iter.750` | `L2052.iter.831` | no bound result here | `.15.9` |
| `L1857.iter.751` | `L2052.iter.832` | no bound result here | `.15.9` |
| `L1858.iter.752` | `L2053.iter.833` | no bound result here | `.15.9` |
| `L1858.iter.753` | `L2053.iter.834` | no bound result here | `.15.9` |
| `L1858.iter.754` | `L2053.iter.835` | no bound result here | `.15.9` |
| `L1858.guard.755` | `L2053.guard.836` | no bound result here | `.15.9` |
| `L1858.guard.756` | `L2053.guard.837` | no bound result here | `.15.9` |
| `L1858.guard.757` | `L2053.guard.838` | no bound result here | `.15.9` |
| `L1890.iter.769` | `L2085.iter.850` | no bound result here | `.15.9` |
| `L1890.iter.770` | `L2085.iter.851` | no bound result here | `.15.9` |
| `L1890.iter.771` | `L2085.iter.852` | no bound result here | `.15.9` |
| `L1890.guard.772` | `L2085.guard.853` | no bound result here | `.15.9` |
| `L1890.guard.773` | `L2085.guard.854` | no bound result here | `.15.9` |
| `L1890.guard.774` | `L2085.guard.855` | no bound result here | `.15.9` |
| `L1890.guard.775` | `L2085.guard.856` | no bound result here | `.15.9` |
| `L1890.guard.776` | `L2085.guard.857` | no bound result here | `.15.9` |
| `L1890.guard.777` | `L2085.guard.858` | no bound result here | `.15.9` |
| `L1896.logic.781` | `L2091.logic.862` | no bound result here | `.15.9` |
| `L1897.eq.782` | `L2092.eq.863` | no bound result here | `.15.9` |
| `L1897.eq.783` | `L2092.eq.864` | no bound result here | `.15.9` |
| `L1897.eq.784` | `L2092.eq.865` | no bound result here | `.15.9` |
| `L1897.logic.786` | `L2092.logic.867` | no bound result here | `.15.9` |
| `L1897.logic.787` | `L2092.logic.868` | no bound result here | `.15.9` |
| `L1900.iter.788` | `L2095.iter.869` | no bound result here | `.15.9` |
| `L1900.iter.789` | `L2095.iter.870` | no bound result here | `.15.9` |
| `L1900.iter.790` | `L2095.iter.871` | no bound result here | `.15.9` |
| `L1901.iter.791` | `L2096.iter.872` | no bound result here | `.15.9` |
| `L1901.iter.793` | `L2096.iter.874` | no bound result here | `.15.9` |
| `L1910.cond.796` | `L2105.cond.877` | no bound result here | `.15.9` |
| `L1911.stmt.798` | `L2106.stmt.879` | no bound result here | `.15.9` |
| `L1941.iter.806` | `L2136.iter.888` | no bound result here | `.15.9` |
| `L1941.iter.808` | `L2136.iter.890` | no bound result here | `.15.9` |
| `L2021.iter.834` | `L2216.iter.916` | no bound result here | `.15.9` |
| `L2022.iter.837` | `L2217.iter.919` | no bound result here | `.15.9` |
| `L2022.guard.840` | `L2217.guard.922` | no bound result here | `.15.9` |
| `L2045.lit.853` | `L2240.lit.935` | no bound result here | `.15.9` |
| `L2062.iter.856` | `L2257.iter.938` | no bound result here | `.15.9` |
| `L2062.iter.857` | `L2257.iter.939` | no bound result here | `.15.9` |
| `L2062.guard.859` | `L2257.guard.941` | no bound result here | `.15.9` |
| `L2062.guard.860` | `L2257.guard.942` | no bound result here | `.15.9` |
| `L2065.bool.865` | `L2260.bool.947` | no bound result here | `.15.9` |
| `L2067.dir.867` | `L2262.dir.949` | no bound result here | `.15.9` |
| `L2079.stmt.876` | `L2274.stmt.958` | no bound result here | `.15.9` |
| `L2091.iter.880` | `L2286.iter.962` | no bound result here | `.15.9` |
| `L2108.logic.882` | `L2303.logic.964` | no bound result here | `.15.9` |
| `L2109.eq.884` | `L2304.eq.966` | no bound result here | `.15.9` |
| `L2109.eq.885` | `L2304.eq.967` | no bound result here | `.15.9` |
| `L2109.logic.886` | `L2304.logic.968` | no bound result here | `.15.9` |
| `L2109.logic.888` | `L2304.logic.970` | no bound result here | `.15.9` |
| `L2182.iter.897` | `L2377.iter.979` | no bound result here | `.15.9` |
| `L2182.guard.899` | `L2377.guard.981` | no bound result here | `.15.9` |
| `L2182.guard.900` | `L2377.guard.982` | no bound result here | `.15.9` |
| `L2190.logic.905` | `L2385.logic.987` | no bound result here | `.15.9` |
| `L2204.eq.912` | `L2399.eq.994` | no bound result here | `.15.9` |
| `L2204.eq.913` | `L2399.eq.995` | no bound result here | `.15.9` |
| `L2204.logic.914` | `L2399.logic.996` | no bound result here | `.15.9` |
| `L2204.logic.916` | `L2399.logic.998` | no bound result here | `.15.9` |
| `L2212.iter.920` | `L2407.iter.1002` | no bound result here | `.15.9` |
| `L2218.logic.922` | `L2413.logic.1004` | no bound result here | `.15.9` |
| `L2220.eq.925` | `L2415.eq.1007` | no bound result here | `.15.9` |
| `L2220.eq.926` | `L2415.eq.1008` | no bound result here | `.15.9` |
| `L2220.logic.928` | `L2415.logic.1010` | no bound result here | `.15.9` |
| `L2296.iter.936` | `L2491.iter.1018` | no bound result here | `.15.9` |
| `L2296.iter.937` | `L2491.iter.1019` | no bound result here | `.15.9` |
| `L2296.guard.939` | `L2491.guard.1021` | no bound result here | `.15.9` |
| `L2296.guard.940` | `L2491.guard.1022` | no bound result here | `.15.9` |
| `L2308.iter.947` | `L2503.iter.1029` | no bound result here | `.15.9` |
| `L2308.iter.948` | `L2503.iter.1030` | no bound result here | `.15.9` |
| `L2308.guard.950` | `L2503.guard.1032` | no bound result here | `.15.9` |
| `L2308.guard.951` | `L2503.guard.1033` | no bound result here | `.15.9` |
| `L2309.cond.952` | `L2504.cond.1034` | no bound result here | `.15.9` |
| `L2314.cond.962` | `L2509.cond.1044` | no bound result here | `.15.9` |
| `L2375.iter.986` | `L2570.iter.1068` | no bound result here | `.15.9` |
| `L2375.iter.987` | `L2570.iter.1069` | no bound result here | `.15.9` |
| `L2439.stmt.992` | `L2634.stmt.1074` | no bound result here | `.15.9` |
| `L1447.chain.1145` | `L1641.chain.1249` | no bound result here | `.15.9` |
| `L1447.chain.1146` | `L1641.chain.1250` | no bound result here | `.15.9` |
| `L1461.contains.1148` | `L1655.contains.1252` | no bound result here | `.15.9` |
| `L1461.contains.1150` | `L1655.contains.1254` | no bound result here | `.15.9` |
| `L1491.chain.1152` | `L1685.chain.1256` | no bound result here | `.15.9` |
| `L1553.chain.1154` | `L1747.chain.1258` | no bound result here | `.15.9` |
| `L1553.chain.1155` | `L1747.chain.1259` | no bound result here | `.15.9` |
| `L1557.contains.1158` | `L1751.contains.1262` | no bound result here | `.15.9` |
| `L1557.contains.1159` | `L1751.contains.1263` | no bound result here | `.15.9` |
| `L1557.contains.1160` | `L1751.contains.1264` | no bound result here | `.15.9` |
| `L1557.contains.1161` | `L1751.contains.1265` | no bound result here | `.15.9` |
| `L1566.chain.1162` | `L1760.chain.1266` | no bound result here | `.15.9` |
| `L1566.chain.1163` | `L1760.chain.1267` | no bound result here | `.15.9` |
| `L1578.chain.1165` | `L1772.chain.1269` | no bound result here | `.15.9` |
| `L1583.chain.1168` | `L1777.chain.1272` | no bound result here | `.15.9` |
| `L1587.chain.1170` | `L1781.chain.1274` | no bound result here | `.15.9` |
| `L1587.chain.1171` | `L1781.chain.1275` | no bound result here | `.15.9` |
| `L1591.chain.1172` | `L1785.chain.1276` | no bound result here | `.15.9` |
| `L1591.chain.1173` | `L1785.chain.1277` | no bound result here | `.15.9` |
| `L1591.chain.1174` | `L1785.chain.1278` | no bound result here | `.15.9` |
| `L1618.chain.1177` | `L1812.chain.1281` | no bound result here | `.15.9` |
| `L1618.chain.1178` | `L1812.chain.1282` | no bound result here | `.15.9` |
| `L1691.contains.1179` | `L1886.contains.1283` | no bound result here | `.15.9` |
| `L1691.contains.1181` | `L1886.contains.1285` | no bound result here | `.15.9` |
| `L1826.chain.1211` | `L2021.chain.1315` | no bound result here | `.15.9` |
| `L1858.chain.1226` | `L2053.chain.1330` | no bound result here | `.15.9` |
| `L1858.chain.1227` | `L2053.chain.1331` | no bound result here | `.15.9` |
| `L1875.chain.1235` | `L2070.chain.1339` | no bound result here | `.15.9` |
| `L1877.chain.1237` | `L2072.chain.1341` | no bound result here | `.15.9` |
| `L1877.chain.1238` | `L2072.chain.1342` | no bound result here | `.15.9` |
| `L1893.chain.1243` | `L2088.chain.1347` | no bound result here | `.15.9` |
| `L1981.chain.1247` | `L2176.chain.1351` | no bound result here | `.15.9` |
| `L1981.chain.1248` | `L2176.chain.1352` | no bound result here | `.15.9` |
| `L2023.contains.1256` | `L2218.contains.1360` | no bound result here | `.15.9` |
| `L2023.contains.1258` | `L2218.contains.1362` | no bound result here | `.15.9` |
| `L2054.chain.1259` | `L2249.chain.1363` | no bound result here | `.15.9` |
| `L2054.chain.1260` | `L2249.chain.1364` | no bound result here | `.15.9` |
| `L2068.chain.1262` | `L2263.chain.1366` | no bound result here | `.15.9` |
| `L2097.chain.1266` | `L2292.chain.1370` | no bound result here | `.15.9` |
| `L2105.chain.1267` | `L2300.chain.1371` | no bound result here | `.15.9` |
| `L2105.chain.1268` | `L2300.chain.1372` | no bound result here | `.15.9` |
| `L2106.chain.1269` | `L2301.chain.1373` | no bound result here | `.15.9` |
| `L2106.chain.1270` | `L2301.chain.1374` | no bound result here | `.15.9` |
| `L2106.chain.1271` | `L2301.chain.1375` | no bound result here | `.15.9` |
| `L2108.contains.1272` | `L2303.contains.1376` | no bound result here | `.15.9` |
| `L2127.chain.1276` | `L2322.chain.1380` | no bound result here | `.15.9` |
| `L2127.chain.1277` | `L2322.chain.1381` | no bound result here | `.15.9` |
| `L2145.chain.1278` | `L2340.chain.1382` | no bound result here | `.15.9` |
| `L2145.chain.1279` | `L2340.chain.1383` | no bound result here | `.15.9` |
| `L2154.chain.1280` | `L2349.chain.1384` | no bound result here | `.15.9` |
| `L2154.chain.1281` | `L2349.chain.1385` | no bound result here | `.15.9` |
| `L2165.chain.1282` | `L2360.chain.1386` | no bound result here | `.15.9` |
| `L2165.chain.1283` | `L2360.chain.1387` | no bound result here | `.15.9` |
| `L2202.chain.1293` | `L2397.chain.1397` | no bound result here | `.15.9` |
| `L2216.chain.1296` | `L2411.chain.1400` | no bound result here | `.15.9` |
| `L2218.contains.1299` | `L2413.contains.1403` | no bound result here | `.15.9` |
| `L2229.chain.1305` | `L2424.chain.1409` | no bound result here | `.15.9` |
| `L2229.chain.1306` | `L2424.chain.1410` | no bound result here | `.15.9` |
| `L2230.chain.1309` | `L2425.chain.1413` | no bound result here | `.15.9` |
| `L2233.chain.1310` | `L2428.chain.1414` | no bound result here | `.15.9` |
| `L2236.chain.1313` | `L2431.chain.1417` | no bound result here | `.15.9` |
| `L2237.chain.1314` | `L2432.chain.1418` | no bound result here | `.15.9` |
| `L2237.chain.1315` | `L2432.chain.1419` | no bound result here | `.15.9` |
| `L2237.contains.1319` | `L2432.contains.1423` | no bound result here | `.15.9` |
| `L2237.contains.1320` | `L2432.contains.1424` | no bound result here | `.15.9` |
| `L2249.chain.1321` | `L2444.chain.1425` | no bound result here | `.15.9` |
| `L2249.chain.1322` | `L2444.chain.1426` | no bound result here | `.15.9` |
| `L2250.chain.1323` | `L2445.chain.1427` | no bound result here | `.15.9` |
| `L2250.chain.1324` | `L2445.chain.1428` | no bound result here | `.15.9` |
| `L2250.chain.1325` | `L2445.chain.1429` | no bound result here | `.15.9` |
| `L2251.chain.1326` | `L2446.chain.1430` | no bound result here | `.15.9` |
| `L2251.chain.1327` | `L2446.chain.1431` | no bound result here | `.15.9` |
| `L2252.chain.1329` | `L2447.chain.1433` | no bound result here | `.15.9` |
| `L2252.chain.1330` | `L2447.chain.1434` | no bound result here | `.15.9` |
| `L2257.chain.1333` | `L2452.chain.1437` | no bound result here | `.15.9` |
| `L2257.chain.1334` | `L2452.chain.1438` | no bound result here | `.15.9` |
| `L2283.chain.1340` | `L2478.chain.1444` | no bound result here | `.15.9` |
| `L2283.chain.1341` | `L2478.chain.1445` | no bound result here | `.15.9` |
| `L2284.chain.1342` | `L2479.chain.1446` | no bound result here | `.15.9` |
| `L2284.chain.1343` | `L2479.chain.1447` | no bound result here | `.15.9` |
| `L2285.chain.1346` | `L2480.chain.1450` | no bound result here | `.15.9` |
| `L2291.chain.1347` | `L2486.chain.1451` | no bound result here | `.15.9` |
| `L2291.chain.1348` | `L2486.chain.1452` | no bound result here | `.15.9` |
| `L2292.chain.1349` | `L2487.chain.1453` | no bound result here | `.15.9` |
| `L2292.chain.1350` | `L2487.chain.1454` | no bound result here | `.15.9` |
| `L2292.chain.1351` | `L2487.chain.1455` | no bound result here | `.15.9` |
| `L2293.chain.1354` | `L2488.chain.1458` | no bound result here | `.15.9` |
| `L2294.chain.1359` | `L2489.chain.1463` | no bound result here | `.15.9` |
| `L2294.chain.1360` | `L2489.chain.1464` | no bound result here | `.15.9` |
| `L2344.chain.1365` | `L2539.chain.1469` | no bound result here | `.15.9` |
| `L2345.chain.1367` | `L2540.chain.1471` | no bound result here | `.15.9` |
| `L2345.chain.1368` | `L2540.chain.1472` | no bound result here | `.15.9` |
| `L2353.chain.1369` | `L2548.chain.1473` | no bound result here | `.15.9` |
| `L2353.chain.1370` | `L2548.chain.1474` | no bound result here | `.15.9` |
| `L2373.chain.1371` | `L2568.chain.1475` | no bound result here | `.15.9` |
| `L2373.chain.1372` | `L2568.chain.1476` | no bound result here | `.15.9` |
| `L2373.chain.1374` | `L2568.chain.1478` | no bound result here | `.15.9` |
| `L2374.chain.1375` | `L2569.chain.1479` | no bound result here | `.15.9` |
| `L2374.chain.1376` | `L2569.chain.1480` | no bound result here | `.15.9` |
| `L2374.chain.1377` | `L2569.chain.1481` | no bound result here | `.15.9` |
| `L2374.chain.1378` | `L2569.chain.1482` | no bound result here | `.15.9` |
| `L2374.chain.1379` | `L2569.chain.1483` | no bound result here | `.15.9` |
| `L2388.chain.1380` | `L2583.chain.1484` | no bound result here | `.15.9` |
| `L2388.chain.1381` | `L2583.chain.1485` | no bound result here | `.15.9` |
| `L2430.chain.1383` | `L2625.chain.1487` | no bound result here | `.15.9` |
| `L2431.chain.1384` | `L2626.chain.1488` | no bound result here | `.15.9` |
| `L2431.chain.1385` | `L2626.chain.1489` | no bound result here | `.15.9` |
| `L1435.clogic.1403` | `L1629.clogic.1509` | no bound result here | `.15.9` |
| `L1619.clogic.1405` | `L1813.clogic.1511` | no bound result here | `.15.9` |
| `L1750.clogic.1406` | `L1945.clogic.1512` | no bound result here | `.15.9` |

#### Initial `run` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L2449.cmp.994` | `L2644.cmp.1076` | no bound result here | `.15.10` |
| `L2449.cond.995` | `L2644.cond.1077` | no bound result here | `.15.10` |
| `L2450.int.996` | `L2645.int.1078` | no bound result here | `.15.10` |
| `L2450.int.997` | `L2645.int.1079` | no bound result here | `.15.10` |
| `L2454.stmt.998` | `L2649.stmt.1080` | no bound result here | `.15.10` |
| `L2458.int.999` | `L2653.int.1081` | no bound result here | `.15.10` |
| `L2458.int.1000` | `L2653.int.1082` | no bound result here | `.15.10` |
| `L2458.stmt.1001` | `L2653.stmt.1083` | no bound result here | `.15.10` |
| `L2478.iter.1002` | `L2673.iter.1084` | no bound result here | `.15.10` |
| `L2478.iter.1003` | `L2673.iter.1085` | no bound result here | `.15.10` |
| `L2478.iter.1004` | `L2673.iter.1086` | no bound result here | `.15.10` |
| `L2479.stmt.1005` | `L2674.stmt.1087` | no bound result here | `.15.10` |
| `L2488.stmt.1006` | `L2683.stmt.1088` | no bound result here | `.15.10` |
| `L2464.chain.1389` | `L2659.chain.1493` | no bound result here | `.15.10` |
| `L2464.chain.1390` | `L2659.chain.1494` | no bound result here | `.15.10` |

#### Initial `beside` rows

| Initial ID | Current exact ID | Bound result | Owner |
| --- | --- | --- | --- |
| `L526.chain.1037` | `L664.chain.1131` | no bound result here | `.15.11` |
| `L526.chain.1038` | `L664.chain.1132` | no bound result here | `.15.11` |
| `L526.chain.1039` | `L664.chain.1133` | no bound result here | `.15.11` |
| `L526.chain.1040` | `L664.chain.1134` | no bound result here | `.15.11` |
| `L526.chain.1041` | `L664.chain.1135` | no bound result here | `.15.11` |
| `L528.chain.1042` | `L666.chain.1136` | no bound result here | `.15.11` |
| `L529.chain.1044` | `L667.chain.1138` | no bound result here | `.15.11` |
| `L529.contains.1049` | `L667.contains.1143` | no bound result here | `.15.11` |


### Complete compiler-function measurement

Measurement `.15.2.1.1` (`2026-10-18`) actually attempts every current `compile`
candidate, using the complete controlling parent population. This function associates compiled
units with package targets, records dependency/environment inputs, normalizes command configuration,
and refuses unheld paths or a build that produced nothing of its own package.

| Observation | Actual result |
| --- | --- |
| Global production listing | 1526 candidates listed |
| Complete compiler-function selection | 83 attempted |
| Full parent population, original and restored copy | 153 tests, none ignored |
| Named finishing kills | 49 |
| Survivors requiring assessment | 28 |
| Compiler refusals | 6 |
| Equivalents / timeouts / unjudged | 0 / 0 / 0 |
| Native strict exit | 1 — criterion unmet |
| Unmutated copied baseline | passed |

The controller's exit 0 means the complete native result was audited. Native exit 1 remains an
unmet adequacy criterion. No survivor is classified as equivalent, and the six typed refusals
establish no coverage. No equivalent record applies to this selection; the file still contains
26 records whose unrelated validity is outside this run.

Every actual ID/raw replacement and ordered operator total binds the source
`069d8e598fd9d6c12306fbe0f091635b60f942b747e78e6167e92b41e0ca01ba`.
The installed same-volume write guard passes its positive control and refuses all five forbidden
write operations before the run; the unmodified copied parent command passes. All 1063 held
regular inputs restore in the original and copied tree, with modes and Python/system hashes.
The restored copy retains the same full parent names; its borrowed index metadata does not prove
a copied Git index or submodule contents. The gitlink remains an opaque identity.

The native held activation is `21450e7ebb479e57f83ea79f013c4ab867397a9a`. This is not an
integration stamp or the later completion-document tree. Final source/operator/catalogue/reason
bytes, all 30 pinned source copies and Cargo/rustc metadata remain exact. Seven altered receipts
refuse: missing/duplicate row, foreign ID, wrong replacement or strict exit, and missing/foreign
finishing test name. The completion book/staged checks belong to a later document epoch.

| Operator | Attempted | Compiler-refused |
| --- | ---: | ---: |
| `stmt` | 5 | 2 |
| `iter` | 12 | 0 |
| `guard` | 12 | 0 |
| `cond` | 10 | 0 |
| `not` | 3 | 0 |
| `eq` | 3 | 0 |
| `int` | 2 | 0 |
| `logic` | 5 | 0 |
| `dir` | 5 | 0 |
| `lit` | 8 | 0 |
| `chain` | 10 | 0 |
| `contains` | 8 | 4 |

Each killed row below gives one actual finishing parent name; the native receipt retains the
complete name set. Survivor rows name their owned follow-up. Compiler-refused rows retain their
actual type diagnostic. Every selected candidate appears exactly once.

| Current ID | Operator | Actual outcome | Finishing parent, owned assessment or compiler diagnostic |
| --- | --- | --- | --- |
| `L783.stmt.269` | `stmt` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L797.iter.270` | `iter` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L797.iter.271` | `iter` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L797.iter.272` | `iter` | killed | `trust::tests::a_harness_or_a_library_root_compiling_another_role_s_package_is_refused` |
| `L797.guard.273` | `guard` | survived | `.15.2.1.3` — finishing parent verified; original parser defect remains owned |
| `L797.guard.274` | `guard` | survived | `.15.2.1.3` — finishing parent verified; original parser defect remains owned |
| `L797.guard.275` | `guard` | survived | `.15.2.1.3` — finishing parent verified; original parser defect remains owned |
| `L799.cond.276` | `cond` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L799.cond.277` | `cond` | survived | `.15.2.1.3` — finishing parent verified; original parser defect remains owned |
| `L799.not.278` | `not` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L805.eq.279` | `eq` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L806.int.280` | `int` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L806.int.281` | `int` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L814.eq.282` | `eq` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L814.logic.283` | `logic` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L814.dir.284` | `dir` | survived | `.15.2.1.4` — pending assessment |
| `L814.lit.285` | `lit` | survived | `.15.2.1.4` — pending assessment |
| `L814.lit.286` | `lit` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L834.stmt.287` | `stmt` | unviable | error[E0308]: match arms have incompatible types |
| `L841.iter.288` | `iter` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L841.iter.289` | `iter` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L841.iter.290` | `iter` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L841.guard.291` | `guard` | killed | `trust::tests::a_shared_package_s_environment_is_in_its_item_and_a_foreign_variable_is_refused` |
| `L841.guard.292` | `guard` | survived | `.15.2.1.2` — resolved by the environment parents below |
| `L841.guard.293` | `guard` | survived | `.15.2.1.2` — resolved by the environment parents below |
| `L842.lit.294` | `lit` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L842.lit.295` | `lit` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L844.dir.296` | `dir` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L845.logic.297` | `logic` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L845.cond.298` | `cond` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L845.cond.299` | `cond` | killed | `trust::tests::a_shared_package_s_environment_is_in_its_item_and_a_foreign_variable_is_refused` |
| `L845.not.300` | `not` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L845.dir.301` | `dir` | survived | `.15.2.1.2` — resolved by the environment parents below |
| `L845.lit.302` | `lit` | survived | `.15.2.1.2` — resolved by the environment parents below |
| `L845.lit.303` | `lit` | killed | `trust::tests::a_separator_inside_an_environment_value_cannot_hide_a_configuration_change` |
| `L857.stmt.304` | `stmt` | survived | `.15.2.1.5` — pending assessment |
| `L860.iter.305` | `iter` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L860.iter.306` | `iter` | killed | `trust::tests::a_macro_assembling_a_path_attribute_is_refused` |
| `L860.iter.307` | `iter` | killed | `trust_generated::generator_tests::a_generator_build_failing_leaves_step_4_unable_to_judge_with_its_refusals_beside_it` |
| `L860.guard.308` | `guard` | survived | `.15.2.1.6` — pending assessment |
| `L860.guard.309` | `guard` | survived | `.15.2.1.6` — pending assessment |
| `L860.guard.310` | `guard` | survived | `.15.2.1.6` — pending assessment |
| `L866.cond.311` | `cond` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L866.cond.312` | `cond` | survived | `.15.2.1.6` — pending assessment |
| `L866.dir.313` | `dir` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L871.stmt.314` | `stmt` | unviable | error[E0308]: if and else have incompatible types |
| `L877.stmt.315` | `stmt` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L885.iter.316` | `iter` | survived | `.15.2.1.7` — pending assessment |
| `L885.iter.317` | `iter` | survived | `.15.2.1.7` — pending assessment |
| `L885.iter.318` | `iter` | survived | `.15.2.1.7` — pending assessment |
| `L885.guard.319` | `guard` | survived | `.15.2.1.7` — pending assessment |
| `L885.guard.320` | `guard` | survived | `.15.2.1.7` — pending assessment |
| `L885.guard.321` | `guard` | survived | `.15.2.1.7` — pending assessment |
| `L886.logic.322` | `logic` | survived | `.15.2.1.7` — pending assessment |
| `L886.logic.323` | `logic` | survived | `.15.2.1.7` — pending assessment |
| `L886.logic.324` | `logic` | survived | `.15.2.1.7` — pending assessment |
| `L886.cond.325` | `cond` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L886.cond.326` | `cond` | survived | `.15.2.1.7` — pending assessment |
| `L886.dir.327` | `dir` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L886.lit.328` | `lit` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L886.lit.329` | `lit` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L911.eq.330` | `eq` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L911.cond.331` | `cond` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L911.cond.332` | `cond` | survived | `.15.2.1.8` — pending assessment |
| `L911.not.333` | `not` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L784.chain.1156` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L784.chain.1157` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L804.chain.1158` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L804.chain.1159` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L822.chain.1160` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L822.chain.1161` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L826.chain.1162` | `chain` | survived | `.15.2.1.9` — pending assessment |
| `L826.chain.1163` | `chain` | survived | `.15.2.1.9` — pending assessment |
| `L845.contains.1164` | `contains` | killed | `trust::tests::cargo_multiline_output_paths_preserve_complete_inventory` |
| `L845.contains.1165` | `contains` | killed | `trust::tests::cargo_multiline_output_paths_preserve_complete_inventory` |
| `L845.contains.1166` | `contains` | killed | `trust::tests::cargo_multiline_output_paths_preserve_complete_inventory` |
| `L845.contains.1167` | `contains` | killed | `trust::tests::cargo_multiline_output_paths_preserve_complete_inventory` |
| `L886.contains.1168` | `contains` | unviable | error[E0599]: no method named first found for reference &String in the current scope |
| `L886.contains.1169` | `contains` | unviable | error[E0599]: no method named first found for reference &String in the current scope |
| `L886.contains.1170` | `contains` | unviable | error[E0599]: &String is not an iterator |
| `L886.contains.1171` | `contains` | unviable | error[E0599]: &String is not an iterator |
| `L911.chain.1172` | `chain` | killed | `trust::adequacy_tests::build_read_tests::every_generator_names_a_shared_transitive_marked_or_declared_build_read` |
| `L911.chain.1173` | `chain` | killed | `trust::adequacy_tests::reader_role_tests::every_compiled_reader_is_judged_for_scripts_and_program_targets` |


All surviving obligations are scheduled, without a presumed cause:

| Assessment | Exact current survivors | What it must establish |
| --- | --- | --- |
| `.15.2.1.2` — Environment admission | `L841.guard.292`, `L841.guard.293`, `L845.dir.301`, `L845.lit.302` | foreign environment record positions and exact CARGO_PKG_ namespace |
| `.15.2.1.3` — Compiler-command filtering | `L797.guard.273`, `L797.guard.274`, `L797.guard.275`, `L799.cond.277` | non-rustc command reachability, actual producer/caller domain and command positions |
| `.15.2.1.4` — Artifact suffix extraction | `L814.dir.284`, `L814.lit.285` | exact extra-filename option and prefix/substring boundaries |
| `.15.2.1.5` — Environment-loop tail | `L857.stmt.304` | final continue control flow and complete remaining loop body/drop order |
| `.15.2.1.6` — Dependency absolute paths | `L860.guard.308`, `L860.guard.309`, `L860.guard.310`, `L866.cond.312` | absolute dependency refusals at every position and supported raw/canonical path geometry |
| `.15.2.1.7` — Configuration absolute paths | `L885.iter.316`, `L885.iter.317`, `L885.iter.318`, `L885.guard.319`, `L885.guard.320`, `L885.guard.321`, `L886.logic.322`, `L886.logic.323`, `L886.logic.324`, `L886.cond.326` | retained absolute configuration reachability, both rejection forms and every position |
| `.15.2.1.8` — Own-package completion | `L911.cond.332` | successful-build own-package completion and actual target/caller reachability |
| `.15.2.1.9` — Unit owner cardinality | `L826.chain.1162`, `L826.chain.1163` | target metadata lookup, duplicate owners and exact reachable collection cardinality |

The environment parents below resolve those four admission obligations. Command filtering is
the next assessment. Each follow-up uses actual supported callers and independent expectations,
then add finishing parent controls, repair an original defect if found, or establish a precise
source/domain proof. Finite examples alone do not establish equivalence. Each resulting slice is
committed separately before complete current-region replay; other helper/file scopes stay separate.

Reproduce the full parent population with:

```bash
cargo test -q -p xtask -- trust:: trust_generated:: trust_gate:: --skip route_tests
```

The exact selected native command is:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust.rs --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail --equivalents xtask/trust.equivalents \
  --only L783.stmt.269 \
  --only L797.iter.270 \
  --only L797.iter.271 \
  --only L797.iter.272 \
  --only L797.guard.273 \
  --only L797.guard.274 \
  --only L797.guard.275 \
  --only L799.cond.276 \
  --only L799.cond.277 \
  --only L799.not.278 \
  --only L805.eq.279 \
  --only L806.int.280 \
  --only L806.int.281 \
  --only L814.eq.282 \
  --only L814.logic.283 \
  --only L814.dir.284 \
  --only L814.lit.285 \
  --only L814.lit.286 \
  --only L834.stmt.287 \
  --only L841.iter.288 \
  --only L841.iter.289 \
  --only L841.iter.290 \
  --only L841.guard.291 \
  --only L841.guard.292 \
  --only L841.guard.293 \
  --only L842.lit.294 \
  --only L842.lit.295 \
  --only L844.dir.296 \
  --only L845.logic.297 \
  --only L845.cond.298 \
  --only L845.cond.299 \
  --only L845.not.300 \
  --only L845.dir.301 \
  --only L845.lit.302 \
  --only L845.lit.303 \
  --only L857.stmt.304 \
  --only L860.iter.305 \
  --only L860.iter.306 \
  --only L860.iter.307 \
  --only L860.guard.308 \
  --only L860.guard.309 \
  --only L860.guard.310 \
  --only L866.cond.311 \
  --only L866.cond.312 \
  --only L866.dir.313 \
  --only L871.stmt.314 \
  --only L877.stmt.315 \
  --only L885.iter.316 \
  --only L885.iter.317 \
  --only L885.iter.318 \
  --only L885.guard.319 \
  --only L885.guard.320 \
  --only L885.guard.321 \
  --only L886.logic.322 \
  --only L886.logic.323 \
  --only L886.logic.324 \
  --only L886.cond.325 \
  --only L886.cond.326 \
  --only L886.dir.327 \
  --only L886.lit.328 \
  --only L886.lit.329 \
  --only L911.eq.330 \
  --only L911.cond.331 \
  --only L911.cond.332 \
  --only L911.not.333 \
  --only L784.chain.1156 \
  --only L784.chain.1157 \
  --only L804.chain.1158 \
  --only L804.chain.1159 \
  --only L822.chain.1160 \
  --only L822.chain.1161 \
  --only L826.chain.1162 \
  --only L826.chain.1163 \
  --only L845.contains.1164 \
  --only L845.contains.1165 \
  --only L845.contains.1166 \
  --only L845.contains.1167 \
  --only L886.contains.1168 \
  --only L886.contains.1169 \
  --only L886.contains.1170 \
  --only L886.contains.1171 \
  --only L911.chain.1172 \
  --only L911.chain.1173
```

The guard and source activation described above remain part of the recorded execution. These
results establish neither final regional adequacy nor whole-file adequacy, remote CI or physical-board
execution. The complete measured result is committed before any subsequent assessment changes.


### Compiler environment admission regressions

The `2026-10-18` regression slice (`.15.2.1.2.1`) resolves the four measured environment
survivors with two permanent parents. Production already refuses these inputs; the correction is
to the test coverage, with production, earlier tests, mutation operators and equivalent reasons
unchanged.

A compiled environment record must use the exact `CARGO_PKG_` prefix or one of the explicit
allowed names, and a foreign record is refused at any position. The old single-`HOME` fixture's
foreign record is the final physical dependency line. A check narrowed to the last line or the two
ends therefore still passes that fixture. The new position parent holds a real foreign record
strictly inside the emitted dependency file, before an allowed package record.

| Measured survivor | Required finishing parent | Actual new verdict |
| --- | --- | --- |
| `L841.guard.292` | `trust::tests::compiled_environment_refusals_cover_nonterminal_records` | killed |
| `L841.guard.293` | `trust::tests::compiled_environment_refusals_cover_nonterminal_records` | killed |
| `L845.dir.301` | `trust::tests::compiled_environment_refuses_package_namespace_lookalikes` | killed |
| `L845.lit.302` | `trust::tests::compiled_environment_refuses_package_namespace_lookalikes` | killed |

The namespace parent uses two foreign names: `CARGO_PKGNAME`, which lacks the required underscore,
and `PREFIX_CARGO_PKG_NAME`, which merely contains the prefix. The expected refusal names the
literal foreign variable. Proper package variables remain accepted, including the recorded absence
of `CARGO_PKG_NOT_DEFINED`. For example, these are valid Rust reads but remain foreign to the
inventory's allowed namespace:

```rust
pub const MISSING_SEPARATOR: Option<&str> = option_env!("CARGO_PKGNAME");
pub const EMBEDDED_PREFIX: Option<&str> = option_env!("PREFIX_CARGO_PKG_NAME");
```

Both parents use actual inventory builds. Allowed controls write an inventory; foreign reads
produce the intended refusal and leave no `trust-dependencies.json`; restoring the allowed source
writes successfully again. The position control independently reads Cargo/rustc's actual dependency
file and asserts the literal records' physical ordering. The namespace control checks the recorded
allowed value and absence, rather than only accepting a successful command.

The selected native command uses the full **155 tests**, not just the new parents. Its unmodified
copied baseline passes, and all four exact edits have their required finishing names with strict
exit 0. Every original 153 parent remains present. All 1063 held original/copied regular inputs
restore, including modes and Python/system hashes; restored copied names agree. Seven false
bindings refuse, including substitution of another valid but unintended parent name.

Only the two Rust test functions are added. Removing them reconstructs every previous Rust byte;
the complete production listing remains 1526 with identical raw edits. The equivalent file still
has 26 records, none applicable here; all 30 pinned source copies and tool metadata remain exact.
The held native activation is `b423bf43fe5b28c48c01ea3101260108acd07df5`; later book/staged
completion checks and the final focused verification belong to their own epoch. Copied index
metadata is borrowed, the gitlink is opaque, and no executable-digest or new integration claim follows.

This resolves four particular obligations from the earlier 83-edit measurement. The other 24
survivors remain owned at that earlier measurement's epoch; no fresh whole-region count is inferred.
Command-filtering assessment `.15.2.1.3` is next, before complete current compiler replay and
the remaining helpers/files.


### Compiler command provenance assessment

Assessment `.15.2.1.3.1` (`2026-10-18`) finds a supported source of non-compiler
records and an original phantom-unit refusal. The four command-filter survivors remain
unclassified; JSON-message mode does not establish their equivalence.

The current callers use `build` or `test --no-run --test`, the fixed
`--message-format=json` and single `-v`, a cleared allowlisted environment, and prebuild
target-kind admission. A first real deprecated-note probe puts the warning's forged
`Running` line on stdout in JSON mode. The wider plain-mode control puts it on stderr,
where the compiled current parser reads it as an extra command. That first result cannot
be generalized to every compiler diagnostic.

Actual multiline assembler warnings provide the counterexample. The pinned compiler can emit
raw diagnostic text on stderr even in the fixed JSON mode. Cargo's pinned
`on_stderr_line_inner` forwards a non-JSON line unchanged; the current `cargo_commands`
then reads an embedded `Running` line as a command. The source census records ten matching
call sites across seven Cargo production modules for its two exact patterns. Those real
command writers do not account for every parsed record: diagnostic data is another route.
Seven newly copied source inputs verify against the official pinned archive and existing cache;
all 37 source identities and independent system hashes agree.

| Actual producer probe | Build/parser outcome |
| --- | --- |
| Deprecated note, fixed JSON mode | success; warning on stdout, only the real rustc record |
| Same note, wider plain mode | success; an extra non-compiler record parsed from stderr |
| Assembler warning with literal backslash-n | success; no extra parsed command |
| Assembler print directive | success; its output is on stdout |
| Assembler warning with a physical line break | success; two extra non-compiler records |
| Same physical warning, text naming rustc | success; two extra records naming rustc |

All these rows describe the installed pinned toolchain. The inventory boundary is checked
separately with a maintained two-root fixture whose roots compile a common dependency.
Ordinary source writes an inventory. The warning macro is refused by the catalog unless its
exact source extent has a reviewed admission; the following comparison includes that admission.

| Admitted warning payload | Actual inventory |
| --- | --- |
| `Running not-a-compiler`, with the command delimiters | written |
| `Running rustc`, with the same delimiters | refused for empty-name phantom units in both gen and chk |

The latter is an **owned P2 defect**, not an unadmitted assembly-site refusal. Real compilation
succeeds and the exact macro extent is admitted, yet warning text is interpreted as a compilation
unit. The false diagnostic says that each root compiles an empty-named crate with no target owner.
No inventory is left after refusal. The admitted non-compiler case is also a concrete supported
witness for testing the filter; it is not evidence for equivalence.

The compiler-warning source used by the second inventory case is:

```rust
core::arch::global_asm!(r#".warning "note
Running `rustc`""#);
pub fn f() {}
```

Its exact macro-extent admission is:

```lisp
(defadmit (file "crates/common/src/lib.rs") (rule "global_asm")
  (sha256 "8fe9c284e0ccf464eee42de4f7d905be99f720fff309738ff5e35117168164ae") (reason "a fixture"))
```

Changing the payload to `not-a-compiler` also requires updating the extent hash. Both cases
use the same roots, package graph, allowed configuration and JSON-mode build path. The observation
parent temporarily extends only the maintained restored sweep copy and uses its normal exact
outer-root configuration seam; no original Rust source changes. The copied Rust file restores
byte-for-byte, and all 1063 original regular inputs restore with modes and Python/system hashes.
Six altered observations refuse, including a false successful inventory and a wrong refusal reason.

The following slice adds permanent finishing controls for the four measured filter edits
(`.15.2.1.3.2`). The phantom-unit defect remains scheduled immediately afterward under
`.15.2.1.3.3`: define and verify compiler-invocation provenance that keeps warning data separate,
preserve admitted-source behavior, implement the repair, then rebind mutation/proof scopes and
run the full milestone gate. The command-filter cohort remains open until both tracks finish.
No new parent or equivalent entry is added by this assessment, and no fresh whole-region result,
remote CI or universal-host behavior is claimed.


### Admitted nonterminal warning regression parent

The `2026-10-18` slice `.15.2.1.3.2` adds one permanent parent:
`trust::tests::compiler_command_filter_ignores_admitted_nonterminal_warning_records`.
Two ordinary roots compile a common dependency. An ordinary control writes an inventory;
then the dependency gains a reviewed exact-extent assembly macro with two explicit warning
records, naming `not-a-compiler-first` and `not-a-compiler-last`. The unchanged production
filter ignores this warning data, and the inventory still writes. Restoring ordinary source
and its roots removes the admission and writes again.

The exact warning-source macro is:

```rust
core::arch::global_asm!(r#".warning "first
Running `not-a-compiler-first`"
.warning "last
Running `not-a-compiler-last`""#);
```

The producer control uses an actual cold Cargo build with the current environment constructor
and the fixed JSON/single-verbose arguments. It independently observes a genuine common-crate
compilation before the first warning and a later warning after it. One warning is therefore
nonterminal without depending on when the consumer crate starts compiling. The initial stricter
fixture required the dependency warning before that consumer's command. Actual output put the
consumer command first: pipelined dependency metadata can enable consumer work before final
assembler diagnostics. That experimental assertion failed and was corrected before verification;
the original failed receipt remains retained.

| Exact edit | Required actual finishing parent | New native outcome |
| --- | --- | --- |
| `L797.guard.273` | `trust::tests::compiler_command_filter_ignores_admitted_nonterminal_warning_records` | killed |
| `L797.guard.274` | `trust::tests::compiler_command_filter_ignores_admitted_nonterminal_warning_records` | killed |
| `L797.guard.275` | `trust::tests::compiler_command_filter_ignores_admitted_nonterminal_warning_records` | killed |
| `L799.cond.277` | `trust::tests::compiler_command_filter_ignores_admitted_nonterminal_warning_records` | killed |

The native command retains the full **156 tests**, including all earlier 155 parent names,
and freshly kills all four exact edits with the intended new parent and strict exit 0.
All 1066 held regular inputs restore in the original and copied tree, with modes and
Python/system hashes; restored copied names agree. Seven falsified receipts refuse, including
another valid but unintended finishing name.

Removing only this new Rust function reconstructs every earlier Rust byte. Production,
the 1526-candidate listing/raw edits, mutation operators, catalogue and 26 equivalent records
remain unchanged. All 37 pinned source copies and tool metadata revalidate. Native held
activation `ff8c42813c93968b07c03684f11f6713a3bf0f79` is distinct from later completion
documents and final staged focused verification; copied index metadata is borrowed and the
gitlink remains opaque.

These four particular filter obligations are resolved. Together with the earlier four environment
controls, eight individual obligations from the initial compiler measurement now have finishing
parents. The other 20 initial survivors remain owned at that measurement's epoch; this is not
a newly measured full-region result.

At this parent slice's close, the P2 phantom-unit defect remained open.
Its next action was the concrete producer/consumer contract and prototype, `.15.2.1.3.3.1`,
followed by bounded repair and complete current source/premise/mutation/integration verification.
The command-filter cohort stays open until that defect is fixed.


### Compiler provenance contract and prototype

The `2026-10-18` slice `.15.2.1.3.3.1` verifies a repair mechanism for the admitted-warning
false refusal above. **At this prototype epoch, production inventory was unchanged.** The prototype records compiler
invocations separately, through Cargo's published
[`RUSTC_WRAPPER` interface](https://doc.rust-lang.org/cargo/reference/environment-variables.html),
whose first argument is the compiler followed by its arguments. The global wrapper observes dependency
compilations too. The workspace-only wrapper is not used. Both interfaces and the installed public
compiler documentation are tied to the [pinned toolchain](ledger.md#rust-toolchain).

A compiler warning can contain any of the following data without becoming a compilation unit:

```text
Running `not-a-compiler`
Running `rustc`
Running `rustc --crate-name phantom phantom.rs --out-dir bogus`
```

The observer receives arguments from Cargo's process invocation, independently of those strings.
It forwards the arguments to the real compiler, with stdout and stderr inherited, and writes its own
completed record into the build's private directory. The consumer reads that directory only after
Cargo succeeds. It does not recover units from warning text if provenance is missing or malformed.

The proposed production boundary is:

| Step | Contract |
| --- | --- |
| Prepare the instrument | Compile the tracked embedded std-only observer with the pinned compiler once for this inventory invocation; verify its installed bytes in each role's target directory. |
| Begin a role build | Create a fresh same-volume session for the root, harness or generator. Preserve the existing cleared base environment and fixed compiler; add explicit instrument settings only. |
| Run the compiler | Reserve a partial record with exclusive creation before execution. Forward native argv and working directory, remove the wrapper and private settings from the compiler environment, and preserve diagnostic streams. |
| Complete the record | Retain session, role, invocation identity, working directory, complete argv bytes and exit/signal outcome. Publish without replacing another record. A record failure fails the build. |
| Read provenance | Require the exact schema and current identity, regular non-symlink files, complete byte framing and successful status. Empty, stale, partial, foreign-role or unknown records refuse. |
| Separate queries | Recognize the pinned version and target-information queries; they yield no units. Unknown shapes refuse. Each real compilation remains subject to metadata ownership, dependency files, source admission, environment and configuration checks. |
| Select the artifact | Bind the role target to an observed compilation. Ask pinned rustc for link-output names using empty input and observed naming options, then check basenames, target containment, regular output files and hashes. Arbitrary Cargo stdout does not select the artifact. |
| Finish or fail | Sort units as before. Keep instrument/session paths out of configuration and shared-item identities. On failure leave no inventory, retain useful diagnostics and partial evidence, wait for children, and use a new session next time. |

The [canonical inventory record](../../../docs/specs/trust/decision_trust-inventory.md#compiler-provenance-repair-contract-and-prototype)
keeps the same contract. Implementation remains divided into producer/unit collection, artifact
binding, compatibility/refusal controls and complete current mutation/premise/integration verification.
The first implementation slice must include a real public inventory red/green case for the P2 defect.
Independent review and final F30 adequacy remain separate obligations.

#### Replaying the prototype

The instrument and both native source fixtures are tracked:

```bash
python3 -I -B scripts/trust_compiler_provenance_probe.py \
  --out target/compiler-provenance-probe
```

Choose a new output directory on each run. It must be relative to the repository under `target/` and
resolve on the repository volume. The instrument derives the root from its own location, prepares
project-local temporary/cache directories, compiles the actual current environment constructor and
uses the pinned toolchain. The OS C compiler and installed toolchain/public documentation are read-only
external dependencies. No submodule implementation, other checkout or old evaluation case is used.
All build workspaces, Cargo homes, traces, binaries and receipts stay under the chosen output.

The prototype format is `archogen-compiler-provenance-prototype/1`. It encodes native argument and
working-directory bytes as hexadecimal; a record includes the session, role, PID, compiler exit code
and signal. Raw observations can therefore retain the actual runtime spelling without becoming
project lookup paths. Production configuration remains repository-relative after normalization.
This is an internal prototype format, not a new public inventory schema or a production repair.

#### Actual role and payload matrix

The dated run completes **20 actual Cargo builds**, all with release/locked/offline/no-default-features,
JSON messages and single verbosity, using the actual base environment plus explicit instrument settings.
Each cell gives the independently expected real compilation units; every build succeeds.

| Common-dependency source | Binary root | Library root | Comparison harness | Generator |
| --- | --- | --- | --- | --- |
| Ordinary | common, app | common, api | common, api, adapter | common, generator |
| Warning naming not-a-compiler | common, app | common, api | common, api, adapter | common, generator |
| Warning naming rustc | common, app | common, api | common, api, adapter | common, generator |
| Full command-shaped warning | common, app | common, api | common, api, adapter | common, generator |
| Escaped JSON-looking stdout | common, app | common, api | common, api, adapter | common, generator |

Each build also observes the pinned producer's three query invocations: version twice and target
information once. They produce no units. Every real unit agrees with a Cargo artifact report.
These direct Cargo fixtures exercise the mechanism; they do not replace the separately reproduced
public inventory admission/refusal case above or establish repaired production behavior.

The stdout probe uses an assembler print directive. Its output retains backslashes before quotes,
so it is JSON-looking text rather than a valid artifact object. Attempts using adjacent or single
quotes fail in the assembler. No artifact-report exploit was demonstrated. The contract binds
artifacts to real observed outputs so their selection does not depend on guessing which diagnostic
text can resemble a Cargo record.

For every observed compilation, a separate empty-input naming query selects only observed crate
name/type, test mode, target if present and extra-filename. All **45 naming queries** agree with actual
Cargo artifact bytes. A complete before/after file-hash census of each build directory is unchanged.
This uses rustc's documented
[`file-names` print option](https://doc.rust-lang.org/rustc/command-line-arguments/print-options.html#file-names),
checked against the installed pinned documentation and actual outputs. No source macro is evaluated
by these empty-input queries. The future artifact slice must preserve containment, role ownership and
existing supported path geometry, including cases beyond this finite fixture matrix.

#### Transport, failure and concurrency controls

An independently compiled C receiver writes its actual native argv as NUL-separated bytes. The
observer's decoded record matches it exactly for empty arguments, spaces, apostrophes, double quotes,
LF/CR, backslashes, Unicode and non-UTF8 Unix bytes. The receiver also checks the private fields and
wrapper setting are absent. Its exit7 and exact stdout/stderr are preserved. A signal15 case exits143
and records the signal; a compiler spawn failure exits2 and leaves a refused partial record. A missing
trace-directory setup exits2 before the requested receiver can execute.

Eight concurrent native writers retain distinct, complete records with exact independent argv
receipts. No record is replaced or left partial in that successful concurrency case. The consumer's
following controls each alter a separate copy of valid primary records and all refuse:

| Alteration | Required rejection |
| --- | --- |
| Missing record population | missing provenance |
| Partial file | incomplete/unexpected record |
| Old session | session identity |
| Other role | role identity |
| Other schema | schema identity |
| Extra field | record schema |
| Duplicate JSON field | duplicate JSON field |
| Invalid hexadecimal | byte framing/alphabet |
| Filename/PID disagreement | invocation identity |
| Failed compiler status | compiler status |
| Other compiler | compiler selection |
| Other working directory | compiler cwd |
| Unknown query | unknown query shape |
| Symbolic-link record | incomplete/unexpected record |

These **14 rejecting controls** and the native error cases exercise this prototype, not all future
production failure branches. A trusted host/toolchain and preflight's prohibition on executing build
scripts/procedural macros remain premises; the record is not a security boundary against a host that
can rewrite both instrument and evidence. Source admissions and all input/file/environment rules
remain in force.

#### Evidence and remaining work

The verified receipt is `target/compiler-provenance-prototype/verified/prototype-results.json`,
SHA-256 `d247612c4726520bed465797a60d3eb2728a9538025ad5934f9aa552eca2937f`.
The producer is durable: the replay command above builds the tracked instrument and fixtures.
The receipt binds their hashes, current production sources, native tool identities and installed
public documentation. The tracked supplemental auditor rechecks raw records, literal role sets and
actual artifact hashes, and rejects altered summaries:

```bash
python3 -I -B scripts/fixtures/compiler_provenance_audit.py \
  --input target/compiler-provenance-probe/prototype-results.json
```

The private runtime paths in raw argv are observations, not relocation inputs.

All 1067 original regular inputs restore at the activation epoch, including modes and independent
Python/system hashes. Later completion documents are a separate epoch. The original trust production
and tests, current mutation operators/catalogue and equivalent-reason file remain unchanged.
Exploratory failures are retained: an overly narrow emit-mode expectation omitted binary compilation;
an assembler template needed brace escaping; escaped stdout was initially mistaken for valid JSON.
The durable run uses the corrected expectations and records the narrower actual conclusions.

Next, `.15.2.1.3.3.2.1` implements actual invocation collection with an admitted-warning public
red/green regression. `.2.2` binds role artifacts to observed output; `.2.3` holds compatible paths,
roles and refusal/recovery behavior. Affected old helpers and equivalent premises must be explicitly
retired or revalidated as production changes, with their historical outcomes preserved.
`.3.3.3` then rebinds all affected sources, current raw mutations, callers and producer premises,
resolves new survivors and runs full milestone integration. The command-filter cohort and F30 stay
open until the repair and those gates finish.


### Production compiler invocation collection repair

The `2026-10-18` slice `.15.2.1.3.3.2.1` replaces production command collection with the
separate observer protocol verified above. The admitted warning that previously invented empty-name
units now produces a valid inventory. Root and dependency units come from actual compiler process
arguments. Diagnostic text cannot add a unit to this collection.

The regression uses two roots, each depending on `common`. An ordinary inventory establishes the
configuration. The common source then receives an exact-extent admission for each warning payload:

```text
Running `not-a-compiler`
Running `rustc`
Running `rustc --crate-name phantom phantom.rs --out-dir bogus`
```

Against the original production code, the bare `rustc` payload fails with empty-name phantom-unit
refusals for both roles. After the repair, every payload writes an inventory. Each root has exactly
its own package and the fixture package value `"crates/common"`, and its complete unit configurations agree with the ordinary
control. Restoring ordinary source and removing the admission succeeds again. Source-content hashes
still reflect the source that was actually compiled; the configuration comparison does not erase them.

#### What the production observer does

The inventory builds its tracked, embedded, std-only observer with the pinned compiler once per
inventory operation. For each root, comparison harness or generator compilation, it installs and
byte-verifies a copy under that role's Cargo target directory and creates a fresh records directory.
The base cleared environment is preserved. Only these explicit instrument settings are added:

```text
RUSTC_WRAPPER
ARCHOGEN_COMPILER_RECORDS
ARCHOGEN_COMPILER_SESSION
ARCHOGEN_COMPILER_ROLE
```

Cargo's published wrapper interface passes the compiler and its actual native arguments to this
observer. The compiler selection remains the pinned literal `rustc`. The observer reserves an
exclusive partial file, forwards the immutable argument tail to the real compiler, inherits its
working directory and diagnostic streams, and removes those private settings from the compiler's
environment. After the compiler finishes, it syncs the record, publishes it without replacement,
and removes the partial name. An observer failure makes the build fail.

The internal record schema is `archogen-compiler-provenance/1`. The consumer requires canonical JSON,
which also rejects duplicate fields, and validates every field:

| Field | Meaning and check |
| --- | --- |
| `schema` | Exact protocol version. |
| `session_hex` | The fresh current session identity; historical records cannot substitute. |
| `role_hex` | The current root, harness or generator build identity. |
| `cwd_hex` | The expected canonical working directory. |
| `pid` | A positive invocation identity agreeing with the record filename. |
| `argv_hex` | Complete native argument bytes, including the fixed compiler program. |
| `exit` | Exactly successful compiler status for an accepted record. |
| `signal` | No signal for an accepted record. |

The hexadecimal wire representation preserves bytes. The current JSON inventory requires interpreted
arguments and paths to be valid UTF-8; unsupported bytes produce an explicit inability to judge,
without lossy argument conversion. Raw runtime paths in trace evidence are observations, not project
lookup paths or persisted configuration. Session and observer paths do not enter unit configuration
or shared-item identity.

The consumer rejects missing, partial, linked, stale, foreign-role, malformed, failed or unknown
records. Version and target-information queries from the pinned producer yield no units. Real
compilations retain complete arguments and still pass the existing metadata ownership, dependency
file, source admission, environment and configuration checks. There is no diagnostic-parser fallback.
The unit sequence retains its deterministic sort.

This slice repairs **unit collection**. Role artifacts still use Cargo's artifact report; direct
observed-output binding is `.3.3.2.2`. The complete compatibility/failure matrix and whole-source
mutation/integration checkpoint remain separately owned. No full F30 adequacy or independent
review closure follows from this bounded repair.

#### Independent forwarding and failure controls

The production observer itself is checked with an independently compiled C receiver. That receiver
writes NUL-separated native argv. Empty arguments, spaces, quotes, LF/CR, backslashes, Unicode,
non-UTF8 bytes and flag-shaped arguments all arrive byte-for-byte, and the record agrees with the
receiver. Exit7 and exact stdout/stderr are preserved. The receiver verifies that the private fields
and wrapper setting are absent.

A real compiler source also asserts that every private setting is absent. Its successful compile
has the exact expected argument vector. A genuine compiler failure retains its diagnostic and is
rejected by the record consumer; a missing compiler leaves a refused partial record; a fresh session
recovers. A standalone observer test checks literal byte encodings and runs through the production
parent. Separate actual query records exercise missing/partial/session/role/cwd/extra-field/duplicate/
status/identity/hex/compiler/query/link/type/object refusals. Every valid copied record is accepted
before its individual fault is applied.

An initial identity test used PID zero while expecting a filename-mismatch diagnostic. Zero is
already an invalid identity, so the fixture was corrected to a different positive PID. The initial
failure remains in the evidence. This corrects the test's branch selection, not production behavior.

#### Bounded diagnostic fixtures and baseline evidence

Observer compilation increased the cost of each inventory operation. The old output-path diagnostic
parent ran its entire matrix under one 30-second bound and failed in the unmodified sweep copy.
An exact copied replay reproduced the timeout: the overall suite ended with 158 passes and one
failure. That is a failed baseline, not a mutation kill or an adequacy result.

The fixture now has three independently bounded parents, retaining the original first name:

| Parent suffix | Path cases |
| --- | --- |
| `cargo_failure_diagnostics_ignore_multiline_output_paths` | ordinary; newline plus erroneous-path; newline plus error-path |
| `_messages` | newline error decoy; newline Cargo-looking error; carriage-return/newline decoy |
| `_quoted_and_unicode` | apostrophe/backslash/newline path; Unicode/newline path |

All original path bytes and their order are retained. Each group uses the same success,
compiler-failure/no-inventory and recovery phases. An audit checks the original assertion body,
allowing only the fixture-name parameter and its exact formatting reflow. The bound stays at
30 seconds. The direct split run passes with observed parent times about 6.6, 9.7 and 9.7 seconds;
these timings describe that run, not every host or future load.

The failed sweep also exposed an owned tool-evidence gap: its baseline diagnostic omitted the raw
streams and finishing name. The exact copied test command supplied the diagnosis here.
`.3.3.3.0.1` now closes that gap; [failed mutation baseline evidence](verification.md#failed-mutation-baseline-evidence)
gives the implemented receipt contract and permanent controls. No automatic retry or changed passing
classification is used to hide the failed attempt.

#### Source, catalogue and reason continuity

The old command splitter and framer now exist only in test fixtures that observe historical verbose
output. Production never calls them. Five catalogue mutations targeting those removed production
paths are explicitly retired. The fixed-compiler mutation remains, with its reason updated for the
new program-identity check. The catalogue's exact-source uniqueness control passes.

Four delimiter reason records move to `xtask/trust.retired-equivalents`, a historical test fixture.
Compiled controls still exercise their exact five old helper edits and wider-domain counterexamples,
and require those keys to be absent from the active file. Historical native outcomes remain dated
records; they are not relabelled as new kills or equivalents.

The existing normalization and Cargo diagnostic helper bodies remain byte-for-byte unchanged.
Three iterator replacement counter names are rebound to current source lines. The producer-dependent
normalization premises are explicitly revalidated: the observer forwards actual immutable argv,
adds no compiler flags, and removes only instrument settings. The existing constructor, fixed
compiler, configuration/preflight and argument consumers retain their contracts. New source guards
include the observer, controller and base environment, with altered-premise rejections. All 34
operator and compiled proof controls pass on this source.

The fresh native epoch holds **162 tests**, including every previous parent plus the new
protocol and split-fixture parents. Its complete source listings contain 1433 trust, 124 controller
and 16 observer candidates; the bounded commands select only the following edits. Source listings
are discovery counts, not attempted coverage. Each selected protocol edit is killed by the intended
`trust::compiler::tests::actual_query_records_reject_missing_stale_foreign_and_malformed_inputs`
parent; other finishing names are retained in the raw results. All active trust reason records are
included in their separate replay.

| Source | Exact current edit | Native result |
| --- | --- | --- |
| `trust_compiler.rs` | `L216.cond.64` | killed |
| `trust_compiler.rs` | `L273.cond.97` | killed |
| `trust_compiler_observer.rs` | `L60.stmt.7` | killed |
| `trust_compiler_observer.rs` | `L64.stmt.11` | killed |
| `trust.rs` | `L345.iter.59` | equivalent |
| `trust.rs` | `L455.bool.83` | equivalent |
| `trust.rs` | `L456.bool.84` | equivalent |
| `trust.rs` | `L457.bool.85` | equivalent |
| `trust.rs` | `L458.bool.86` | equivalent |
| `trust.rs` | `L459.bool.87` | equivalent |
| `trust.rs` | `L460.bool.88` | equivalent |
| `trust.rs` | `L461.bool.89` | equivalent |
| `trust.rs` | `L462.bool.90` | equivalent |
| `trust.rs` | `L463.bool.91` | equivalent |
| `trust.rs` | `L464.bool.92` | equivalent |
| `trust.rs` | `L497.dir.112` | equivalent |
| `trust.rs` | `L497.lit.113` | equivalent |
| `trust.rs` | `L498.dir.116` | equivalent |
| `trust.rs` | `L498.lit.117` | equivalent |
| `trust.rs` | `L499.dir.120` | equivalent |
| `trust.rs` | `L499.lit.121` | equivalent |
| `trust.rs` | `L514.cmp.139` | equivalent |
| `trust.rs` | `L585.iter.158` | equivalent |
| `trust.rs` | `L587.iter.161` | equivalent |
| `trust.rs` | `L590.cond.168` | equivalent |
| `trust.rs` | `L617.cmp.181` | equivalent |

The protocol groups each finish with two kills and strict exit 0. The reason group finishes with
22 scoped equivalents and strict exit 0. There are no compiler refusals, timeouts or unjudged
results in these selected runs. Every group uses the complete parent command, not a finishing-only
subset, and restores all 1077 held regular inputs in the original and copied tree with modes and
independent Python/system hashes. Copied parent names agree exactly. The gitlink is metadata only.
All 37 retained pinned source identities and tool metadata remain bound.

The maintained compiler-selection catalogue entry is also replayed in the restored copy. It is killed
by `compiler_installation_paths_preserve_inventory_units_and_configuration`, as expected, and both
original/copy inputs restore again. A separate raw-receipt audit rejects altered scope, missing result,
replacement, finishing name, count, exit, filename and outcome summaries. The summary adapters are
ignored supplementary evidence; the native CLI, compiled controls and exact commands below are the
tracked/re-runnable producers.

All commands use this complete filter/strict tail:

```bash
--package xtask --timeout 600 \
  --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: \
  --filter=--skip --filter=route_tests --timeouts-fail
```

The consumer command is `bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs` with
`--only L216.cond.64 --only L273.cond.97`. The observer command is
`bash scripts/mutation_sweep.sh xtask/src/trust_compiler_observer.rs` with
`--only L60.stmt.7 --only L64.stmt.11`. The trust command is
`bash scripts/mutation_sweep.sh xtask/src/trust.rs` with
`--equivalents xtask/trust.equivalents` and one `--only` for every trust edit in the table.
Run these commands sequentially: they share the maintained sweep workspace.

Held native tree `9abd2a4f22ae17b3eba7cac0d030d787427c7aa6` names this measured epoch.
The receipts are under `target/compiler-provenance-repair/current/`; the supplementary audit is
SHA-256 `8fa90c2092f898b465e942601d5221c56d2529b6bacea595ddbe57211dfbd51e`.


The held native epoch and final completion-document/focused epoch are separate. New judgment sources
`xtask/src/trust_compiler.rs` and `trust_compiler_observer.rs` join the complete later mutation scope.
Artifact binding, remaining compatibility controls, the baseline-evidence repair and full current
source/premise/integration verification still precede F30 signoff.

### Observed compiler artifact binding

The `2026-10-18` slice `.15.2.1.3.3.2.2` completes production role-artifact binding.
The inventory identifies an executable, a library's rlib or a comparison harness by the
compilation actually observed for that role. Cargo's stdout messages no longer select that file.
The earlier escaped JSON-looking warning experiment did not establish an artifact-report exploit;
this change implements the specified provenance contract without claiming one.

`xtask/src/trust_artifact.rs` owns selection and validation. The metadata pass retains each
target's source by package directory, kind and original target name, separately from the existing
unit-owner and target-classification maps. A binary or test selects that precise metadata target;
a library selects its unique library source. Selection requires exactly one recorded compilation
owned by the package and naming that source, whether relative to the written tree or under its
ordinary/canonical prefix. Missing or ambiguous metadata and invocations stop the inventory.
The generator-entrypoint caller still receives all verified units and ignores the additional
invocation return value; its compiled-input checks remain in force.

Source identity matters for test harnesses. A package may have a binary and integration test
with the same target name. A custom `[[test]]` with `harness = false` also compiles without
the normal `--test` switch. The permanent public-inventory regression builds that combination
and runs the selected artifact: it prints `custom-harness`, while its companion binary would
print `companion-bin`. The standard test-harness phase independently lists `same: test`.
The selector therefore uses metadata ownership and source, rather than treating the presence of
`--test` as target identity.

#### A naming query, with no source evaluation

For the selected invocation the instrument runs the pinned compiler in the same written tree,
with the unchanged base environment and null stdin:

```text
rustc --print=file-names - --crate-name <observed-name> \
  [--crate-type <observed-type>]... [--target <observed-target>] \
  [--test] [-C extra-filename=<observed-suffix>]
```

Only observed naming options are forwarded. The actual source path, code generation options,
feature cfgs, extern libraries and search paths are excluded. The observed `--out-dir` is retained
for locating outputs, not passed to the query. Repeated crate types are preserved; duplicate
singleton options, missing values and unsupported joined naming spellings refuse. An absent
extra-filename option is not invented. The query uses the fixed `rustc` entrypoint and pinned
toolchain environment, without the private observer fields. It creates no additional provenance
unit or session record.

A permanent actual-compiler control first compiles a real executable, then replaces its source
with `compile_error!`. Selection still succeeds and the executable prints `query-artifact`.
The source bytes, output file population and executable bytes stay identical across the query.
This is an observable check that naming does not re-evaluate that source; the separate prototype's
45 queries remain dated corroborating evidence, not a projection of all possible inputs.

#### File validation and inventory paths

The complete query output must be UTF-8, newline terminated and a nonempty sequence of unique
basenames. Empty names, dot/dot-dot, separators, CR, NUL and duplicate names refuse. The canonical
observed output directory must be within the canonical target directory for this role. Every
reported output must exist as a regular non-symlink file and is read. The selected artifact is
hashed: an executable requires exactly one output, and a library exactly one rlib among the
validated outputs.
Missing, foreign, linked, directory-valued and ambiguous outputs refuse rather than choosing an
arbitrary last report.

The recorded path is relative to the role's target directory and now names the compiler's direct
output. For example, an actual binary fixture records
`release/deps/a-ea31e1b4fa6d99dc`, while its library records
`release/deps/libb-20572edba3a82d76.rlib`. The suffixes are compiler/build identities, not names
a package consumer should hardcode. Previous Cargo root aliases such as `release/a` or
`release/libb.rlib` are no longer selected. The inventory schema still carries `artifact.path`
and `artifact.sha256`; packaging and verification use those fields and actual bytes.
A clean rebuild under another output directory must retain the established build/configuration
identity contracts; the broad trust parents continue to check that behavior.

The permanent binary/library parent runs the selected binary, checks the library archive header,
requires direct-output paths and compares both recorded hashes with the independent system
SHA-256 implementation. The harness parent does the same hash/path validation before running
both standard and custom harnesses. The output-refusal parent uses a literal independent digest
and checks valid behavior before each malformed-output case. The actual-query parent also tests
missing/wrong/ambiguous metadata, foreign/missing/duplicate invocations and an actual failing
target-information query, followed by fresh successful selection.

#### Replay commands and scope

All shell scratch/cache settings are those in the toolbox. The focused artifact command is:

```bash
cargo test -p xtask artifact -- --nocapture
python3 -I -B xtask/mutation_sweep_tests.py "$PWD" \
  IteratorTests.test_cargo_absolute_prefix_edits_preserve_line_selection -v
```

The proof-premise guard initially rejects the changed production source. Revalidation confirms
the unchanged normalization and diagnostic-selection bodies, base environment, observer,
controller, CLI/gate, catalogue and active/historical reason files. The new artifact query uses
the same absolute tree and no Cargo producer flags. The guard now also binds artifact production
and rejects a deliberately changed query cwd. An initial isolated Python module-style command
could not import the test harness; the recorded replay uses the harness's documented file/root
arguments. That invocation error was not a proof result.

The native copied-tree command is:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 \
  --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: \
  --filter=--skip --filter=route_tests --timeouts-fail \
  --only L84.cond.58 --only L156.logic.98 --only L154.chain.132
```

It selects three actual discovery edits: omit the containment refusal, weaken package/source
conjunction to disjunction, and truncate the matching invocation population before uniqueness
checking. This is a bounded contract smoke measurement. The full new source and all affected
current sources/reasons remain owned by `.3.3.3`, after compatibility and the separately owned
baseline-evidence repair. No whole-source adequacy or F30 closure follows from these three edits.

The dated native result is 3 named kills, strict exit 0, with no timeout, compiler refusal or
unjudged result. Original and restored-copy parent CLI lists agree at 167 names; all 1079
held regular inputs restore with modes and independent Python/system hashes. The gitlink is
metadata only. All 37 retained producer-source pins remain bound. The receipt audit rejects nine
false summaries: scope, omitted result, replacement, intended name, total, exit, filename, source
identity and outcome. It is supplemental to the maintained native producer and tracked controls.
Held native tree `0e213978519fa101cc75e9f6eac1ed30249fd408` precedes completion-doc/focused staging.
Receipts are under `target/compiler-artifact-binding/`; supplemental audit SHA-256 is
`74fcaf11ad73522da648ecd14031568738f8893861dbe07302dc4721a7e0910a`.

The prior completed collector leaf is sealed without rewriting its bytes. Independent committed-core,
index and full pre-seal tree reconstruction agree, and all earlier archives remain identical.
Final focused/book/doctrine checks close this bounded slice. Compatibility, the owned baseline
evidence repair and complete current-source/integration review remain ahead; F30 remains open.

### Compiler warning data across harness and generator roles

The `2026-10-18` slice `.15.2.1.3.3.2.3.1` adds permanent actual-inventory controls for
the two remaining caller roles. The earlier root/library parent stays unchanged. Production
collector, observer, artifact selection, normalization, base environment, gate/CLI, catalogue,
proof controls and reason bytes stay identical to the preceding artifact-binding commit.

Both new parents use a committed fixture and real Cargo/compiler builds under the pinned toolchain.
The warning is an exact reviewed `global_asm!` site emitting a multiline `.warning` whose text
contains a `Running` line. The complete command-shaped payload is
`rustc --crate-name phantom phantom.rs --out-dir bogus`. Those tokens are warning data, never
a compiler invocation.

| Phase | Comparison harness | Separate generator entrypoint |
| --- | --- | --- |
| Ordinary source | Exact literal unit names, owners and source files; complete configurations retained as the baseline; selected test artifact hashed independently and run | Declared generator/input provenance paths, roles and hashes; separate generator target build required |
| Admitted `not-a-compiler` warning | Same literal units/files and complete configurations; artifact still lists the actual test | Exact source/input hashes and roles; source admission is used |
| Admitted `rustc` warning | Same checks | Same checks |
| Admitted full command-shaped warning | Same checks; no phantom unit | Same checks; no diagnostic-driven build refusal |
| Identical warning without its admission | Source-rule refusal, no inventory | Source-rule refusal, no inventory |
| Genuine `compile_error!` | Public Cargo failure summary and actual recorded compiler exit 1, no inventory | Public Cargo failure summary and actual recorded compiler exit 1, no inventory |
| Restored ordinary source | Original literal expectations/configurations and actual test executable recover | Original declared provenance/hash expectations recover |

Package/path strings below belong to the scratch fixtures. The harness compiles exactly `d` and `i` from `"crates/i"`, and `r` from `"crates/r"`.
Its test unit must read both `"crates/i/tests/d.rs"` and
`"crates/i/tests/support/mod.rs"`; the other units read their literal library roots.
The selected executable lists `same: test` and its inventoried hash agrees with the system
SHA-256 implementation. Complete configuration comparisons use the same fixture/output root,
while source-file population expectations are literal and independent of the collector.

The generator is a separate `"crates/g"` binary that reads `src/helper.rs`.
Its live form explicitly declares that helper as an input. The checker's provenance must
therefore contain precisely `"crates/g/src/main.rs"` as `generator` and
`"crates/g/src/helper.rs"` as `input`, with the exact committed source bytes hashed.
Every admitted warning must consume its source admission; no unused admission can hide a
skipped generator-file check.

An initial oracle expected an undeclared helper to appear automatically in provenance.
The [canonical generator limitation](trust.md#what-it-is-not) and
`docs/specs/trust/decision_trust-generated-sources.md` §8 explicitly distinguish a
generator's compiled-input checks from its declared provenance. Native dep-info did contain
the helper, while the serialized provenance correctly held only the declared entrypoint.
The fixture now declares the helper input rather than changing that contract. A separate
initial failure-message oracle expected the source diagnostic text; Cargo emits that on
its JSON stdout channel, while the public error selects Cargo's stderr summary. The corrected
control independently requires one failed compiler record with exit 1, null signal and the
expected crate-name argument. These were test-oracle corrections, not production repairs.

The tracked parents are:

```bash
cargo test -p xtask -- \
  compiler_provenance_preserves_admitted \
  compiler_provenance_harness_warning_admission_and_failure_recover --nocapture
```

The original names are
`trust::tests::compiler_provenance_preserves_admitted_harness_warning_records_and_recovery`
and
`trust_generated::generator_tests::compiler_provenance_preserves_admitted_generator_warning_records_and_recovery`.
The current replay above also includes the later bounded admission/failure parent described under
[writer lifecycle](#bounded-harness-warning-workloads).
The unchanged child-process protocol retains its 30-second limit and requires a finishing
named result. The local dated run passes both parents, with about 17.5 seconds for the harness
and 13.9 for the generator. These timings describe that run; they are not a new timing bound.

The native copied-tree smoke selects actual file-population edits, retaining only the first
or only the last dependency-file entry:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust.rs \
  --package xtask --timeout 600 \
  --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: \
  --filter=--skip --filter=route_tests --timeouts-fail \
  --only L775.iter.222 --only L775.iter.223
```

The scope is file-consumer integration across the real caller routes, not a reconstruction of
the retired warning parser. No whole-source adequacy result follows. Remaining path/feature/
copied-execution and lifecycle/concurrency controls stay separately owned before the complete
source/premise/integration milestone and F30 review.

The dated native result is 2 named kills, strict exit 0, with no timeout, compiler refusal or
unjudged result. Both new parents kill the first-file truncation; the harness also kills the
last-file truncation. Full original/restored-copy parent CLI lists agree at 169 names. All 1079
regular inputs restore with modes and Python/system hashes; the gitlink is metadata only. All 37
retained producer pins bind. Nine deliberately false receipt summaries reject; the supplementary
audit is SHA-256 `c88ab259e9a0c5336a3e2a4bcf9b32f9e293bf51eb1f36c48fce38ae0172e85e`.
Receipts are under `target/compiler-role-warnings/`; held native tree
`ade5532820ac0f42eeaeed5ad2e60db124b69b49` precedes completion-doc/focused staging.

The source diff is insertion-only inside test regions, with production/caller/reason identity
checks retained in the receipt. The final staged focused/book/doctrine workflow is required before
commit. Path/feature/copy and lifecycle/concurrent-writer slices remain active obligations;
complete source/premise/integration verification still precedes F30 signoff.

The previous committed artifact-binding leaf is sealed during completion bookkeeping. Its
committed core, archive/index bytes and full raw pre-seal reconstruction agree; all 275 earlier
archives remain identical. This archival epoch follows the native input-restoration epoch.

### Compiler artifact path and feature compatibility

The `2026-10-18` slice `.15.2.1.3.3.2.3.2` holds the observed-artifact contract across
actual harness and separate generator builds, including admitted target features. Earlier
quoted/multiline/source-environment/canonical path and feature-isolation parents remain in place.
Production and previous controls remain unchanged; the maintained mutation catalogue gains one
default-feature omission control.

The new parent is
`trust::tests::observed_role_artifacts_preserve_paths_and_disable_target_default_features`.
It builds a comparison harness that reads a declared generated support module, plus a separate
generator binary that reads its declared helper input. The library target and generator each
declare this reviewed manifest feature table:

```toml
[features]
default = ["tripwire"]
tripwire = []
```

Both source files contain a compile-error guard under `cfg(feature = "tripwire")`.
The manifests need exact whole-file admissions for the feature rule. Successful inventory builds
therefore exercise the fixed `--no-default-features` argument for the actual library, test and
generator target routes. A separate real Cargo build of the generator with defaults enabled must
fail with the generator tripwire diagnostic, establishing that its guard is operative.

The output cases are these fixture-relative path strings:

```text
plain
space'quote\slash\nRunning `rustc`\rµ
alias/output -> physical'\nµ/output
```

Here `\n` and `\r` describe actual LF and CR bytes in the fixture names; the mixed path also
contains a space, apostrophe, backslash, backticks and Unicode. The alias is a real symbolic link
in the scratch fixture. It resolves to a physical parent whose name itself contains a quote,
line break and Unicode. The artifact files themselves must still be regular non-symlink files
within the canonical target directory.

For every output, the parent requires:

- Complete package, unit and provenance records, and shared items, equal to the plain baseline.
- Literal unit counts and source-file/hash maps for every root and the comparison harness,
  including the harness's generated support module.
- Direct `release/deps/` artifact paths, existing regular files and actual bytes whose hashes
  agree with the independent system SHA-256 implementation.
- A selected harness executable that lists `same: test`.
- Precisely the declared generator/helper provenance paths and source hashes, plus the separate
  generator build directory.

The independent hash command receives artifact bytes on stdin. Its filename-output format
escapes paths containing line breaks or backslashes, so parsing that format as an unescaped digest
would be an incorrect oracle. No equality of rlib bytes across different output directories is
assumed: the inventory records each actual artifact and its digest at that output.

The direct command is:

```bash
cargo test -p xtask \
  observed_role_artifacts_preserve_paths_and_disable_target_default_features -- --nocapture
```

The dated run passes within the unchanged 30-second parent limit, at about 7.5 seconds.
The copied whole-trust baseline includes this parent. A native file-population smoke also
requires its finishing name:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust.rs \
  --package xtask --timeout 600 \
  --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: \
  --filter=--skip --filter=route_tests --timeouts-fail --only L775.iter.222
```

The maintained catalogue control `trust-cargo-target-default-features-enabled` removes only
the observed compile argument `--no-default-features`. Its exact source block must occur once,
and `cargo xtask mutate` must report a finishing kill by the new parent, then restore the
original source. It is run in the already verified scratch copy:

```bash
# Working directory: target/mutation-sweep/tree
cargo xtask mutate --timeouts-fail --only trust-cargo-target-default-features-enabled
```

This omission reaches the library's default-feature tripwire before generator checking. The
separate direct-Cargo failure above independently holds the generator guard. The control is a
regression against a missing argument, not a claim that current production enabled defaults.
The native smoke and catalogue replay are bounded evidence, not a whole-source adequacy result.

#### Owned native selection defect

Preparing the copied run uncovered a separate P2 tool defect. The native sweep's `--only`
filter does not validate requested IDs. Through its existing explicit Git/Cargo CLI fixture,
requesting `L999999.not-a-current-id.0` runs one baseline, attempts zero mutations, and exits 0
with `none attempted`. A contrasting valid control attempts its one mutation and reports an
equivalent. The actual script is unchanged in this reproduction; the fixture tests tool argument
selection and makes no claim about a Rust compiler.

Source review identifies the skip loop and empty-result success path as the root cause.
Task `M3.6.9.6.3.2.2.15.2.1.3.3.3.0.2` owns rejecting unknown, mixed-valid/unknown and stale
selections before commands, with permanent red/green tests and unchanged valid-selection/
restoration semantics. The repair now passes permanent refusal, valid-selection and real compiler
counter controls; [selected mutation IDs](verification.md#selected-mutation-ids) gives the implemented
contract and its source-binding limit. Current measurements additionally bind exact selected/result
populations; receipt audits reject missing results and incorrect scopes. A successful empty
exploration is not mutation evidence.

The dated native result is one named file-population kill and one maintained default-feature
catalogue kill, both strict exit 0. The full original/restored-copy CLI lists agree at 170 parent
names. All 1081 held regular inputs restore after both runs, with modes and Python/system hashes;
the gitlink is metadata only. All 37 producer-source pins remain bound. Eleven false receipt
summaries reject. The catalogue preserves its entire prior prefix and appends only the new control.
Held native tree `2d57037b0d18086c94dcd668d65776f62f51180a` precedes completion-doc/focused staging.
Receipts are under `target/compiler-path-feature/`; supplemental audit SHA-256 is
`7ba3713e1e98903138857d1e19642b581242a631ca0c9dcb1fd920457748e14a`.

The prior committed warning-role leaf is sealed byte-exact. Its committed core, archive/index
and full saved raw reconstruction agree; all 276 older archives remain identical. The final
staged focused/book/doctrine workflow closes this bounded slice. Lifecycle/concurrent-writer
controls, both native-tool hardening children and full current-source/integration review remain
before F30 signoff.


### Compiler record boundary controls

The `2026-10-18` slice `.15.2.1.3.3.2.3.3.1` fills the remaining record-consumer
boundary controls. Production, existing tests, observer, artifact selector, constructors, CLI,
catalogue and reasons remain unchanged. This is one bounded part of the lifecycle matrix;
writer/setup/publication/signal and concurrent-writer closure remain separate children.

The new parent is
`trust::compiler::tests::actual_records_refuse_remaining_field_range_byte_and_shape_boundaries`.
It compiles the embedded observer with the pinned compiler, then uses that observer to run a
real successful Rust compilation with actual source, dep-info/link output and complete argv.
The consumer must return the literal original argument vector. Its completed record becomes
the seed for isolated corruption cases, each first checked as valid with that same result.

| Boundary | Explicit fault cases |
| --- | --- |
| Schema and decoded identity field types | `schema-version`, `schema-type`, `session-type`, `role-type`, `cwd-type`, `missing-key` |
| PID representation/range | `pid-zero`, `pid-negative`, `pid-overflow`, `pid-fraction`, `pid-type` |
| Exit and signal types | `exit-type`, `signal-number` |
| Argument vector | `argv-type`, `argv-empty` |
| Argument byte encoding | `hex-odd`, `hex-uppercase`, `hex-nul`, `hex-non-utf8` |
| Compilation/query shape | `query-tail`, `compilation-no-output`, `compilation-no-name`, `compilation-no-emit` |
| Canonical record framing | `no-newline`, `extra-newline` |
| Raw JSON/text | `invalid-json`, `invalid-text` |
| Record kind/extension | `record-directory`, `record-extension` |

The dated source-derived census contains 29 cases, each with a valid-before-fault check.
The earlier parent independently retains its missing/partial/stale-session/wrong-role/cwd,
extra/duplicate fields, failed status, PID/filename mismatch, invalid hex/compiler/query,
linked record, non-string argument and non-object controls. The actual compilation/forwarding
and native byte receiver parents remain intact.

Numeric PID faults use matching record filenames. In particular, PID 0 is paired with
`0.json`, so the positive-PID guard must reject it even though the filename agrees.
Byte faults append an extra encoded argument to an otherwise complete compilation record.
Omitting the NUL guard would therefore admit the damaged compilation, rather than merely
changing an unknown-query diagnostic. Odd-length and uppercase hex, embedded NUL and invalid
UTF-8 remain explicit refusal boundaries.

Missing/non-string identity fields, invalid PID type/range, malformed argv and status values,
partial invocation shapes, missing/extra record newlines, invalid JSON/text and directory/
extension substitutions must all refuse. Expected instrument-owned error categories are held;
raw JSON/UTF-8 reader failures must produce a nonempty error without overconstraining the
standard-library diagnostic. A fresh independently created session then runs a real
`rustc -vV` query successfully and yields no compilation units.

The direct command is:

```bash
cargo test -p xtask   actual_records_refuse_remaining_field_range_byte_and_shape_boundaries -- --nocapture
```

The final dated parent run passes within the existing 30-second bound, at about 0.65 seconds.
An earlier successful query-seed version was strengthened before measurement to require
otherwise valid compilations for byte faults and matching filenames for numeric PID faults.
That strengthens the acceptance oracle without changing production.

The native copied-tree command is:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs   --package xtask --timeout 600   --filter=trust:: --filter=trust_generated:: --filter=trust_gate::   --filter=--skip --filter=route_tests --timeouts-fail   --only L168.cond.31 --only L233.cmp.72
```

These actual discovery edits omit the NUL refusal and change the PID predicate from
`> 0` to `>= 0`. The new parent must finish and kill each. Complete current-source
assessment, all new judgment-source obligations and integration still belong to `.3`;
neither these samples nor the corruption matrix establishes whole-source mutation adequacy.
The separately owned baseline-evidence and unknown-ID selection repairs precede those full sweeps.

The dated native result is 2 named kills, strict exit 0, with no timeout, compiler refusal or
unjudged result. Both kills carry the new parent name. Original/restored-copy complete CLI
lists agree at 171 parents, and all 1082 held regular inputs restore with modes and independent
Python/system hashes. The gitlink is metadata only; all 37 retained producer-source pins bind.
Nine deliberately false receipt summaries reject. Held native tree
`1d6fca0e3c4861f50ce0bd33e241eff6c8184123` precedes completion-doc/focused staging.
Receipts are under `target/compiler-record-boundaries/`; supplemental audit SHA-256 is
`4c28247a53873e7d2ed3983bc3f334a08f3ec2b33addcab9ff9f6ee414c470e6`.

The prior committed path/feature leaf seals with exact committed core, archive/index and full
saved raw reconstruction; all 277 older archives remain identical. Final staged focused/book/
doctrine checks close this bounded consumer slice; writer/publication/signal and concurrency
children remain open before the compatibility checkpoint and native-tool/source milestone.


### Compiler writer lifecycle controls

The `2026-10-18` slice `.15.2.1.3.3.2.3.3.2` adds actual setup/installation,
publication and native signal controls. Production remains unchanged. A full-suite timeout also
requires the bounded harness-warning split below; its original predicates and phases are retained.
The observer source, base environment, caller interfaces and existing receiver fixture stay exact.
A new independent native publication fixture and one maintained signal-status mutation are tracked.

The direct command runs the bounded parents:

```bash
cargo test -p xtask actual_observer_ -- --nocapture
```

Their names and responsibilities are:

| Parent in `trust::compiler::tests` | Actual boundary held |
| --- | --- |
| `actual_observer_setup_and_installation_fail_before_child_execution` | Controller storage/target/install failures; real relative target resolution and installed bytes/mode/cwd; observer missing argument/settings and non-directory records; no requested child execution; fresh pinned-query recovery |
| `actual_observer_publication_collision_preserves_completed_and_partial_evidence` | A native child creates the completed filename before publication; wrapper fails without replacing it, preserves a complete partial record, consumer refuses, both evidence files remain exact; fresh recovery |
| `actual_observer_records_native_signal_and_recovers` | Real native receiver terminates by SIGTERM, exact argv sink/status/record outcome, removed private environment, failed-record refusal and fresh recovery |

The final local run passes within the unchanged 30-second parent bounds: about 1.20 seconds for
setup/installation and 0.86 seconds each for publication and signal. These are dated observations.

#### Setup and installation

The controller refuses an observer-output parent that is a regular file, a missing
`CARGO_TARGET_DIR`, a target path blocked by a regular file and a missing original observer
whose metadata is needed for installation. The test restores the original executable before
asserting that last refusal. A valid relative target resolves under the supplied cwd; installed
bytes equal the prepared observer, execute permission bits match and cwd is canonical.

The actual observer then receives no compiler argument, or lacks each required records/session/
role setting in turn, or receives a records path that is a regular file. It must return 2 with
its own diagnostic, leave the real session's record directory empty and produce no native-child
completion marker. The held blocking file's bytes remain unchanged. Test setup first builds the
observer and native witness; these refusal cases specifically prevent the requested wrapped child
from executing. An independent new session subsequently runs the pinned `rustc -vV` query.

#### A deterministic publication collision

`scripts/fixtures/compiler_provenance_collision.c` is a controlled native test child. It derives
its parent's PID through the OS and exclusively creates that PID's completed filename in the
explicitly supplied test record directory. It writes `retained native evidence` and prints
`native compiler completed` before exiting 0. This arranges the collision before the observer
publishes, without a timing race or a replacement implementation of the publisher.

The observer must return 2. The directory contains the child's unchanged completed evidence and
the observer's fully written partial record. The partial filename and its recorded PID must agree;
its child exit is 0 and its signal is null. Consumer refusal must not alter either file.
A fresh session recovers successfully. The native witness is a lower-level producer fault control;
ordinary inventory builds still use fixed pinned rustc.

The actual native mutation omits `fs::hard_link(&partial, &completed)?`. The publication parent
must finish and kill it, rather than accepting a wrapper that silently skips its publication step:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_compiler_observer.rs   --package xtask --timeout 600   --filter=trust:: --filter=trust_generated:: --filter=trust_gate::   --filter=--skip --filter=route_tests --timeouts-fail --only L63.stmt.10
```

#### Native signal outcome

The unchanged independent receiver fixture writes its complete native argv and checks that the
wrapper/private fields were removed, then raises SIGTERM. On the measured host that is signal 15.
The observer must publish a completed record with `exit: null` and `signal: 15`, return 143,
and preserve the expected empty diagnostic streams. The consumer rejects the unsuccessful
invocation before interpreting its compiler identity. A new real pinned query then recovers.

The maintained catalogue entry `trust-compiler-observer-signal-status-not-forwarded` replaces
the observer's `128 + signal` outcome with generic exit 1. Its new native signal parent must
kill that exact source edit. The replay runs in the verified scratch copy:

```bash
# Working directory: target/mutation-sweep/tree
cargo xtask mutate --timeouts-fail   --only trust-compiler-observer-signal-status-not-forwarded
```

#### I/O limits and caller failure propagation

These tests inject deterministic storage/metadata/setup and destination-collision failures.
They do not inject every kernel write, sync or post-publication cleanup errno. Those operations
use the observer's error-return path, whose main returns 2; the source retains each `?`.
The standard command-status operation waits for the requested child before publication.
The contract matrix keeps those source premises distinct from exercised failures.

In `xtask/src/trust.rs`, session/setup errors propagate before Cargo; observer exit failure makes
Cargo fail, and `compile` propagates that failure. Existing root/harness/generator public failure
controls hold no-inventory and recovery behavior. The new lower-level producer controls do not
claim to inject every private-record error through the complete public inventory entrypoint.
Concurrent writers and final matrix accounting remain under the next child; full source judgments
and native-tool hardening remain before the milestone.

The dated native result is one named publication kill and one maintained signal-status catalogue
kill, both strict exit 0. Complete original/restored-copy CLI lists agree at 174 parents. All 1084
regular inputs restore after both runs, including modes and independent Python/system hashes;
the gitlink is metadata only. All 37 producer-source pins bind. Eleven false receipt summaries
reject. The catalogue preserves its old prefix and appends only the signal control.
Held tree `88889135d33dcb003386dc90fdcddc8e28399d4d` precedes completion-doc/focused staging; receipts are
under `target/compiler-writer-lifecycle/`, with supplemental audit SHA-256
`3810c5092e82093e392f6c656d45fe3093e7f5cd9a046b5d7d1cfa9633dbaf08`.

The prior committed record-boundary leaf seals with exact core, archive/index and saved raw
reconstruction; all 278 older archives remain identical. Final staged focused/book/doctrine
checks are required before commit. Concurrency and matrix closure remain next.


#### Bounded harness warning workloads

The first focused pass for the writer code succeeded. After the glossary correction, a full
focused run on held tree `c9a61cae4f6a89e05fe3c603a8fa631919fdad8e` failed because the
harness-warning parent exceeded its existing 30-second limit. Exact source comparison with
the earlier passing `c22eee1fe38a5f1bc1c554e8293de14ba6d1f73f` confirms identical measuring/
producer code. The fixture's current source was ordinary recovery and its preceding commit
contained the intentional compiler error: the timeout occurred in the final recovery phase
of the bundled workload, not in a production assertion. The native census found no stragglers.

The old parent bundled ordinary, all admitted warning payloads, source refusal, actual compiler
failure and recovery in one process. It is now split into independent fixtures under the same
bound:

| Parent | Inventory phases |
| --- | --- |
| `compiler_provenance_preserves_admitted_harness_warning_records_and_recovery` | Ordinary; each admitted warning payload; ordinary recovery |
| `compiler_provenance_harness_warning_admission_and_failure_recover` | Admitted complete-command warning; remove admission/refuse/no inventory; actual compiler failure/no inventory; ordinary recovery |

Their shared configuration checker retains the complete old assertion body: literal units/
owners/files, full configurations, independent artifact hash and execution. The entire old
refusal/compiler-failure/failed-child/recovery tail is retained. The source literal and payload
loop remain exact apart from helper/variable spelling. A separate retention audit checks these
after formatting and rejects a deliberately lost failed-child check. Each fixture owns its
state and recovery; no workload depends on another test's output.

The direct split run passes at about 11.0 and 8.7 seconds, respectively. These timings are dated
observations, not replacement limits. The unchanged parent bound remains 30 seconds.

```bash
cargo test -p xtask --   compiler_provenance_preserves_admitted_harness_warning_records_and_recovery   compiler_provenance_harness_warning_admission_and_failure_recover --nocapture
```

The timeout, source/phase diagnosis and retention receipts remain under
`target/compiler-writer-lifecycle/`. The original native writer receipts above retain their
dated population. A fresh copied/native epoch below binds the split parents before final
focused validation; there is no blind retry of the unchanged timed-out workload.


After that split, fresh native revalidation passes the complete copied baseline at 175 parents.
The publication edit again has its intended named kill; the signal catalogue control again has
its native signal parent's kill. Both strict exits are 0, all 1084 regular inputs/modes restore
in both trees after each run, all 37 producer pins bind and 11 false receipt summaries reject.
The source-retention audit separately rejects a lost failed-child assertion. The latest held
tree is `df6dc8e496dfaec26cb4f8b7566b6f9f83ab8023`; receipts are under
`target/compiler-writer-lifecycle/after-latency/`, supplemental audit SHA-256
`b89e0f83847331ddb6179657f0238025f03fd229e164d28df9e584a72fa51859`. Earlier receipts and the failed focused/diagnostic epoch
are preserved. The final staged focused gate must pass this repaired workload before commit.

### Compiler provenance compatibility checkpoint

The `2026-10-18` slice `.15.2.1.3.3.2.3.3.3` completes the bounded compatibility
control matrix. Production and all previous control bodies stay unchanged. The final new parent is
`trust::compiler::tests::actual_concurrent_observer_writers_publish_independent_complete_records`.

#### Actual overlapping writers

Measured `2026-10-18`: the main session publishes 8 records from overlapping observer processes.

The parent prepares the production observer, then launches actual pinned rustc compilations through
it. Each compilation receives source on standard input and writes into its own output directory.
The parent holds all input pipes open until every observer's empty partial reservation exists and
every observer is still running. No compiler can finish its input before that release. This establishes
overlapping live writers through real process and filesystem observations, without a timing-based
guess, a mock compiler or a replacement publisher.

After all reservations are present, the parent supplies source and closes every input. The source
contains compile-time assertions that the wrapper/private fields are absent. Every native compiler
must finish successfully. The record consumer must return exactly the expected complete argument
vectors, independent of file iteration order. Completed record names and recorded PIDs agree with
the operating system's observer-child identifiers; current role/session/cwd, successful status and unique
record population are checked. These PIDs identify the observers, each of which waits for its own
real rustc child. No partial record remains.

Each actual compiled executable is run and must print its own worker marker. A fresh session with
a different role label then runs a real pinned query, sees only its own query record and returns no compilation
units; the preceding session retains its completed population. This is a producer/record-consumer
concurrency control. Normal inventory builds still apply metadata/source ownership to every unit.

The parent routes spawn, reservation and input-write errors through a common close-and-wait path
before assertions: all input handles close and every started observer is waited on. The existing
bounded outer worker guards a hang and its process group. The direct dated run passes in about
2.16 seconds under the unchanged 30-second limit.

```bash
cargo test -p xtask \
  actual_concurrent_observer_writers_publish_independent_complete_records -- --nocapture
```

The native smoke omits the record write and then omits partial-file cleanup:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_compiler_observer.rs \
  --package xtask --timeout 600 \
  --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: \
  --filter=--skip --filter=route_tests --timeouts-fail \
  --only L60.stmt.7 --only L64.stmt.11
```

The new parent must finish and kill each actual edit. This measures publication-data/cleanup
behavior under overlapping real writers, not complete mutation coverage.

#### Complete contract accounting

The [canonical provenance contract](../../../docs/specs/trust/compiler-provenance-contract.md)
keeps the required behaviors. This table accounts for each row through the current production,
permanent controls and explicit source/domain premises. Test names below are in
`trust::compiler::tests`, `trust::artifact::tests`, `trust::tests` or the named
`trust_generated` modules; the measuring implementation is in `xtask/src/trust.rs`,
`xtask/src/trust_compiler.rs`, `xtask/src/trust_compiler_observer.rs` and
`xtask/src/trust_artifact.rs`.

| Contract row | Current evidence and practical limits |
| --- | --- |
| Producer | `Compiler::prepare` builds the tracked std-only observer with pinned rustc before the role loop; `session` installs held bytes. Actual compiler/environment, native argv and setup/install parents hold executable bytes/mode and failure. The once-per-inventory call placement is a source premise. |
| Invocation | `actual_compiler_removes_private_environment_and_records_failures` and `actual_native_receiver_observes_every_forwarded_argument_byte` hold complete argv, private-field removal, status/stdio and unsupported reader encodings. The native signal parent holds the measured signal outcome. The unchanged base constructor/fixed compiler and preflight remain source-bound. |
| Lifecycle | Fresh exclusive session and partial paths, completed no-overwrite publication and actual error/recovery controls; the native collision witness preserves both destination and partial evidence. The overlapping-writer parent holds reservation/publication/cleanup populations. The later sync control injects one calibrated libc-boundary error and holds pre-publication refusal/recovery. Arbitrary kernel write/post-link errno injection is not claimed; those source operations propagate errors. |
| Consumer | The original malformed-record parent plus `actual_records_refuse_remaining_field_range_byte_and_shape_boundaries` hold schema/session/role/cwd/PID/filename/status, exact canonical framing, byte/type/range and record-file refusals, valid-before-fault and recovery. Native NUL and PID 0 acceptance edits are killed. No diagnostic fallback exists. |
| Queries | Actual version queries return no units; actual cold Cargo root/harness/generator builds exercise the pinned target-information query. Unknown/partial invocation shapes refuse. Actual successful compilations retain their full arguments. The accepted query population is pinned, not a promise for future compiler versions. |
| Artifact | `actual_empty_input_query_preserves_build_and_ignores_now_invalid_source`, naming/output refusal controls and actual binary/library/standard/custom same-named harness parents bind metadata source and observed naming/output. Missing/foreign/linked/ambiguous outputs refuse; selected bytes agree with independent system hashes. |
| Determinism | Existing same-output repeat, different-directory configuration, quoted/multiline/canonical path, dependency-environment and configuration-key parents remain. The combined path/feature parent holds complete records/shared items and each actual output hash; cross-directory rlib-byte equality is not assumed. Private settings stay outside compiled configuration; normalization and unit ordering remain source-bound. |
| Scope | Actual root binary/library, standard/custom test harness and separate generator callers share the collector. Admitted warning success, source refusal, compiler failure/recovery and target-default-feature tripwires run through those roles. Existing manifest/configuration/dynamic-target/source/role ownership and copied-execution controls remain. |
| Failure | Public root/harness/generator compiler failures leave no inventory and recover; observer setup/spawn/publication/signal and strict consumer errors refuse. Source error propagation precedes inventory writing, and command status waits for the compiler. The constructor/consumer/public-caller composition is distinguished from injecting every private fault through the whole entrypoint. |

The matrix keeps the trusted-host, pinned-toolchain, source-admission and supported path/encoding
domain explicit. It establishes the implemented contract's bounded compatibility controls.
Complete current-source/premise/mutation/integration verification and independent review remain
before F30 signoff. Native source-boundary, failed-baseline evidence and selected-ID repairs now
close under `.3.3.3.0`; the qualification checkpoint below precedes full measurements. No global adequacy result is
inferred from compatibility closure.

The dated native result is 2 named kills, strict exit 0, with no timeout, compiler refusal or
unjudged result; each carries the new concurrency parent. Complete original/restored-copy
CLI lists agree at 176 parents. All 1085 held regular inputs restore with modes and independent
Python/system hashes; the gitlink is metadata only. All 37 producer-source pins bind, and nine
false receipt summaries reject. Held native tree `163f93e9fcc9725c87f12f5aa5fe7c360a766d3e` precedes
completion-doc/focused staging. Receipts are under `target/compiler-concurrent-writers/`;
supplemental audit SHA-256 is `55d0ceaff9bb0b2eef4ec110ea65f36639d9e68200722a1eb202d7db3671dcdf`.

The previous committed writer leaf seals with exact core, archive/index and saved raw
reconstruction; all 279 earlier archives remain identical. Final staged focused/book/doctrine
checks close the compatibility implementation subtree. The native prerequisite repairs now pass;
the current source/integration checkpoint follows below.

### Current compiler measurement qualification

Checkpoint `.15.2.1.3.3.3.1.1` (`2026-10-19`) qualifies the measurement inputs after the
source-boundary, failed-baseline evidence and selected-ID repairs. The four changed production
files below are discovered with all native operators; these are raw planning populations, with
no killed/survived/equivalent classification and no global adequacy inference.

The actual CLI output agrees exactly with `scripts/trust_adequacy.py::discover` and its complete
listing verifier. For each file, deliberately missing a candidate, changing the range or naming a
different source makes that verifier refuse. Source bytes and production prefixes are bound;
the latter also agree with the independently captured implementation prefixes from the bounded
compatibility work and with system SHA-256 over stdin. The full input census checks complete
source hashes independently through Python and the system tool.

Earlier active routing labels described the initial `069d8e59` compiler lane as current and named
its historical 159/83 scopes. Those labels now say historical: the removed diagnostic-text
collector and the new provenance/artifact modules cannot inherit those old populations. Historical
outcomes remain retained under their original source bindings.

| Source | Raw candidates | Complete source SHA-256 |
| --- | --- | --- |
| `xtask/src/trust.rs` | 1404 | `fdc28b267ceb173da133e6b0d287758df703ecba6e7b3ca4453964b1c7aa567c` |
| `xtask/src/trust_compiler.rs` | 124 | `470e941f7d85d1e737d9942099173fa4060de9b9e41457abca760c1776785b01` |
| `xtask/src/trust_compiler_observer.rs` | 16 | `64b8376bf404a32f3097071f7737a721e789133f716d16e05f8d0ceeb3e38475` |
| `xtask/src/trust_artifact.rs` | 141 | `3c42078e46da0424e4c47aa467cd5e4b6edf851af1a0d50c811f595979aaa5c5` |

Operator populations for this same source epoch:

| Operator | `trust.rs` | Controller | Observer | Artifact |
| --- | --- | --- | --- | --- |
| `alt` | 4 | 0 | 0 | 2 |
| `bool` | 18 | 2 | 2 | 0 |
| `chain` | 286 | 13 | 2 | 23 |
| `clogic` | 31 | 2 | 0 | 2 |
| `cmp` | 12 | 2 | 0 | 0 |
| `cond` | 132 | 22 | 0 | 20 |
| `contains` | 100 | 8 | 0 | 8 |
| `dir` | 19 | 1 | 0 | 5 |
| `eq` | 98 | 18 | 0 | 12 |
| `guard` | 117 | 15 | 0 | 21 |
| `int` | 60 | 4 | 0 | 4 |
| `iter` | 234 | 6 | 3 | 6 |
| `lit` | 18 | 2 | 0 | 4 |
| `logic` | 93 | 15 | 0 | 19 |
| `not` | 42 | 5 | 0 | 11 |
| `stmt` | 140 | 9 | 9 | 4 |

The production range starts at the first supported function declaration and ends before the
first standalone test-module marker. Test bodies do not join these raw production populations.
Caller, fixture, operator/control, catalogue, configuration and contract bindings accompany the
census; the full indexed-input snapshot also holds their bytes and modes. The retained prior
copied tree has its own earlier snapshot and is verified separately, without treating it as the
current source. Gitlinks contribute metadata only; their implementation is never read.

The published compiler-provenance contract and complete compatibility matrix remain the behavior
contract. These bindings qualify measurements; all affected current-source mutations, reason
premises and independent review still need their finishing verification before F30 signoff.

Reproduce the raw discovery/listing agreement from the committed source:

```sh
python3 -I -B - <<'PY'
from pathlib import Path
import importlib.util, subprocess
spec = importlib.util.spec_from_file_location('adequacy', 'scripts/trust_adequacy.py')
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
driver = Path(m.SWEEP).read_text()
for name in ['trust.rs', 'trust_compiler.rs', 'trust_compiler_observer.rs', 'trust_artifact.rs']:
    filename = 'xtask/src/' + name
    candidates, listing, bounds = m.discover(driver, Path(filename).read_text())
    actual = subprocess.check_output(['bash', m.SWEEP, filename, '--package', 'xtask', '--list'], text=True)
    m.verify_listing(actual, listing, filename, bounds)
    print(filename, len(candidates), 'raw candidates; listing agrees')
PY
```

The full local integration command uses the repository-owned stores from `TOOLBOX.md` and the
pinned book tool on `PATH`:

```sh
PATH="$PWD/target/ci/tools/bin:$PATH" make integration
```

Held tree `086cca8d05162af74a1ed0343ed40efbd4503f69` passes all 12 steps, with zero failures,
unavailable steps, unbuilt steps or quarantines: format, lint, complete tests, doctrines,
self-tests, book, no-std, pinned premises, browser build/binding, emulator configuration and actual
target execution. The test and self-test phases observe about 147 and 408 seconds on this run;
these are measurements, not timing guarantees. The current full trust parent population remains
176 names. All 37 pinned producer sources match; direct source/caller/control/configuration
bindings cover 28 files, including the four discovered source records.

The held indexed snapshot contains 1090 regular inputs and one gitlink identity. All regular
bytes/modes restore with independent Python/system hashes, and gitlink checks read metadata only.
The prior native copy separately retains its earlier 1085-input epoch exactly. It is not relabeled
as the current source. The prerequisite subtree seals with two exact committed bodies and all
283 older archives/index rows unchanged.

Receipts are under `target/compiler-source-qualification/`: raw CLI lists, complete candidate
records and source/producer bindings, the held-tree/input snapshot, full integration log and
original/prior-copy restoration reports. The regeneration example above passes unmodified.
Completion-document edits follow the held integration epoch; production, caller, fixture,
control, catalogue and contract bindings remain unchanged. Final book/doctrine checks cover those
document edits. The first next complete native population is the observer source; each actual
outcome must be bound and owned before the other current files can close.

### Compiler provenance commit ledger

These are exact completed task-table records for ownership reconciliation and the compiler
qualification, repair and verification sequence, with every original note retained. The record date is the source
epoch described in those entries. Words such as current and next retain their meaning at that
epoch; the latest measurement record and resume pointer govern current work. Private task rows
retain their original leaf/work-unit identities and route to this full ledger.

| Recorded | Leaf | Work unit | Original record |
| --- | --- | --- | --- |
| 2026-10-18 | `M3.6.9.6.3.2.2.14.3.2.2.2.2.4` | `ARCHOGEN-M3-0751 (leaf M3.6.9.6.3.2.2.14.3.2.2.2.2.4)` | **Three real-Cargo regression gaps closed**: bounded alias/separate-cwd/leading-error parents pass with independent compiler diagnostics and recovery; actual native3 K/strict0, each named new finishing parent; all1056 originals/copies restore, seven falsified receipts refuse, prior code/reasons/operators exact; final held focused and book/staged gates; four proof/producer obligations remain |
| 2026-10-18 | `M3.6.9.6.3.2.2.14.3.2.2.2.2.5` | `ARCHOGEN-M3-0752 (leaf M3.6.9.6.3.2.2.14.3.2.2.2.2.5)` | **Two absolute-prefix edits proven**: universal leading-slash/line-boundary arguments, guarded production/callers/root and actual Cargo relative-manifest/alias producer; actual compiled helpers, literals/wider/wrong-oracle controls and34 operator tests; only2 precise reasons, native2E/strict0, all1058 originals/copies restore, six false bindings refuse,30 source pins agree; final two producer judgments next, no complete adequacy claim |
| 2026-10-18 | `M3.6.9.6.3.2.2.14.3.2.2.2.2.6.1` | `ARCHOGEN-M3-0753 (leaf M3.6.9.6.3.2.2.14.3.2.2.2.2.6.1)` | **Final producer boundaries assessed**: quoted-key helper differences are inventory-syntax-refused, not admitted defects; ordinary config/path boundary makes closed-end edit suppress genuine operational error while original retains it; lexical hypothesis refuted; all1058 inputs restore,30 pins/full cache agree and five false bindings refuse; direct-helper/inventory parents next, no new native K/E or repair |
| 2026-10-18 | `M3.6.9.6.3.2.2.14.3.2.2.2.2.6.2` | `ARCHOGEN-M3-0754 (leaf M3.6.9.6.3.2.2.14.3.2.2.2.2.6.2)` | **Final two diagnostic regression gaps closed**: direct-Cargo quoted-key parent enforces inventory syntax refusal/prebuild state and recovery; actual inventory warning-terminator/ancestor collision retains genuine setup errors, no inventory and recovery; native2K/strict0 with intended names, all1060 originals/copies restored, seven false bindings refuse, prior code/reasons exact; final focused/book gates, full current replay then integration owned separately |
| 2026-10-18 | `M3.6.9.6.3.2.2.14.4.1` | `ARCHOGEN-M3-0755 (leaf M3.6.9.6.3.2.2.14.4.1)` | **Complete current diagnostic region passes**: actual full153-parent native51 replay43K/4E/4U, strict0, no S/timeout/unjudged; exact current raw edits/4 reasons and all intended new names bind, E0599 refusals explicit, full operator totals; all1061 originals/copies restore and copied names agree, ten false receipts refuse,30 pins/tool metadata stable; every row published, no full-file/remote CI claim; integration checkpoint next |
| 2026-10-18 | `M3.6.9.6.3.2.2.14.4.2` | `ARCHOGEN-M3-0756 (leaf M3.6.9.6.3.2.2.14.4.2)` | **Diagnostic cohort full checkpoint passes**: final native51 result43K/4E/4U/full153 parents and exact source/proofs bind; actual local integration12/0 with official/native held5fc854 identity, all1062 inputs restore, five false bindings refuse,30 pins/34 controls stable; bounded containers close, completion book/staged epoch separate; own/fix premature final-sweep resume and schedule complete remaining ownership census before final adequacy |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.1` | `ARCHOGEN-M3-0757 (leaf M3.6.9.6.3.2.2.15.1)` | **All initial survivor ownership reconciled**: native old1420/current1526 bind, all540 initial rows exact/order complete;512 retained raw edits/28 absent/no ambiguity,70 exact historical result bindings without relabelling; all463 general rows gain coherent .15.2–.11 owners, complete compiler83 next;1062 held inputs restore, six false censuses refuse; closed .14 sealed with nine committed bodies/all267 older archives exact and full reconstruction, book exposes every mapping, no code/E/new-outcome claim |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.2.1.1` | `ARCHOGEN-M3-0758 (leaf M3.6.9.6.3.2.2.15.2.1.1)` | **Complete compiler region measured, criterion unmet**: actual83/full153-parent guarded run49K/28S/6U strict1, zero E/timeout/unjudged; exact raw IDs/names/operator totals/source30 pins bind, two E0308/four E0599 refusals explicit; all1063 originals/copies restore and copied153 names agree, seven false receipts reject; own every S in .2–.9, environment admission4 next, all83 outcomes/command in book; no Rust/E/full-file claim |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.2.1.2.1` | `ARCHOGEN-M3-0759 (leaf M3.6.9.6.3.2.2.15.2.1.2.1)` | **Four environment gaps closed**: two real-inventory parents hold nonterminal foreign records and exact namespace boundaries, allowed value/absence plus refusal/no inventory/recovery; full155-parent native4K/strict0 with intended names, all1063 originals/copies restore and seven false receipts reject, prior Rust exact except two added functions/production operators reasons unchanged; final focused/book/staged gates, no projected83 outcome; command-filter assessment next |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.2.1.3.1` | `ARCHOGEN-M3-0760 (leaf M3.6.9.6.3.2.2.15.2.1.3.1)` | **Command producer boundary/root defect established**: actual current helper/Cargo deprecated JSON/plain and multiline assembler probes refute JSON-only E; exact-admitted noncompiler warning writes, rustc-looking warning invents empty units and refuses gen/chk, P2 .3.3 owned immediately after .3.2 finishing controls; all1063 originals/copied Rust restore, six false observations reject,37 source identities/ten literal sites checked; three committed units/four leaves sealed with268 older archives exact; no new E/parent/full-region claim |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.2.1.3.2` | `ARCHOGEN-M3-0761 (leaf M3.6.9.6.3.2.2.15.2.1.3.2)` | **Four command-filter gaps closed**: one real-inventory parent holds exact-admitted nonterminal warning records, ordinary/recovery writes and independent cold producer ordering; correct initial overstrict pipelining assertion; full156-parent native4K/strict0 with intended name, all1066 originals/copies restore and seven false bindings reject, prior Rust exact except one added function/production operators reasons unchanged; final focused/book gates, P2 original phantom-unit defect open and provenance contract/prototype next, no projected83 result |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.2.1.3.3.1` | `ARCHOGEN-M3-0762 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.1)` | **Compiler provenance prototype verified**: tracked published-wrapper/C-receiver instrument passes20 real Cargo builds/four roles,45 empty-input artifact queries with unchanged build files,14 false-record controls,8 concurrent writers and byte/exit/stdio/environment/signal/failure cases; all1067 activation inputs restore, original production/tests/operators/reasons exact; canonical/book contract and three bounded repair children, final focused/book gates; original P2 remains open, producer/unit repair next |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.2.1.3.3.2.1` | `ARCHOGEN-M3-0763 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.2.1)` | **Compiler unit provenance repaired**: private embedded observer/typed consumer replaces diagnostic parser for both callers; actual P2 red/green and native C byte oracle, malformed/failure/recovery controls; split slow diagnostic matrix with all cases/assertions/bounds preserved; fresh full162-parent4K/22E strict0,1077 original/copied inputs restore, intended names/37 pins and eight false-receipt refusals; retire5 catalogue/4 E, alpha-bind3/revalidate16 and preserve historical controls; seal committed prototype with full reconstruction/273 older archives exact; final focused/book gates, artifact binding next and baseline-evidence P2 owned |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.2.1.3.3.2.2` | `ARCHOGEN-M3-0764 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.2.2)` | **Artifacts bind to observed compiler outputs**: metadata/source-owned selection, empty-input pinned query, direct paths/hash, real bin/lib/standard/custom same-named harness controls; artifact7/0, native3K strict0 full167,1079 original/copy restore,37 pins/nine false receipts; producer/normalization/reasons exact, guard red/green; collector sealed/reconstructed, complete book/live sync and final staged focused/doctrine required; compatibility next |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.2.1.3.3.2.3.1` | `ARCHOGEN-M3-0765 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.2.3.1)` | **Harness/generator warning routes held**: exact admitted payload matrix, literal units/files/configurations or declared provenance/hashes, actual artifact/exit evidence, source refusal and fresh recovery; two initial oracle contracts corrected from native/canonical evidence, no production change; native2K strict0 full169,1079 original/copy restore,37 pins/nine false receipts; book/live sync and final staged focused required; path/feature/copy next |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.2.1.3.3.2.3.2` | `ARCHOGEN-M3-0766 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.2.3.2)` | **Role path/features held**: mixed/canonical outputs and operative defaults; native1K/catalogue1K strict0, full170/1081 restore/37 pins/11 falsifiers; prior leaf sealed, unknown selection owned .0.2; book/live/final focused, lifecycle next |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.2.1.3.3.2.3.3.1` | `ARCHOGEN-M3-0767 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.2.3.3.1)` | **Record boundaries held**: real compilation seed/29 corruption cases/recovery, meaningful NUL/PID0 native2K strict0 full171;1082 inputs restore/37 pins/nine falsifiers, production/prior tests/reasons exact; prior leaf seal verified, full book/live/final focused; writer lifecycle next |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.2.1.3.3.2.3.3.2` | `ARCHOGEN-M3-0768 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.2.3.3.2)` | **Writer lifecycle/harness bounds held**: actual setup/install/no-overwrite/signal, observed recovery timeout fixed by exact-assertion split/30s; fresh native1K/catalogue1K full175/1084 restore/37 pins/11 falsifiers; seal/book/glossary/live/final focused, concurrency next |
| 2026-10-18 | `M3.6.9.6.3.2.2.15.2.1.3.3.2.3.3.3` | `ARCHOGEN-M3-0769 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.2.3.3.3)` | **Bounded compatibility closes**: actual8 overlapping writers/records/argv/executables/recovery; native2K full176/1085 restores/37 pins/nine falsifiers; complete9-row matrix/explicit limits, production/prior controls exact; seal/book/live/final focused, native baseline evidence next |

| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.0.0` | `ARCHOGEN-M3-0770 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.0.0)` | repair P1 original-source mutation with normalized indexed regular relative selection; actual red/green fixtures and real Git, retained copy sentinel, unchanged operators and exact archive proof; complete tool/focused/book gates; native baseline evidence next |
| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.0.1` | `ARCHOGEN-M3-0771 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.0.1)` | retain raw failed-baseline evidence/status/names and source/command identity, unique relative receipts/no retry; actual red/green/native Rust and complete41 tool controls, exact classification/archive audits; focused/book gates; selected-ID refusal next |
| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.0.2` | `ARCHOGEN-M3-0772 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.0.2)` | reject unknown/mixed/stale/excluded selected IDs before copy/commands; actual red/green,45 tool and real compiler controls, unchanged functions/catalogue and exact archive audit; focused/book gates; native prerequisites close, source qualification next |

| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.1.1` | `ARCHOGEN-M3-0773 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.1.1)` | current raw/source/producer/caller bindings, historical labels corrected, full integration12 pass and exact original/prior-copy restoration; prerequisites seal exactly, book/live sync; observer native population next |
| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.1` | `ARCHOGEN-M3-0774 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.1)` | full observer16 native14K2S/exit1; all1091 originals/copies restore,28 bindings/37 pins/11 false receipts bind; exact qualification seal and seven historical rows in book; P1 sync then P2 close owned next |
| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.2` | `ARCHOGEN-M3-0775 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.2)` | calibrated actual sync fault/pinned query/partial evidence/recovery; native1K/full177 and catalogue2K with intended name,1093 original/copy restoration/29 bindings/37 pins/11 falsifiers; production/prior controls exact, seal/book/live/focused gates; close assessment next |
| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.3` | `ARCHOGEN-M3-0776 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.3)` | source/API/caller close proof, actual omission/error protocol and deliberate timing difference; guarded per-file1E/full177,1096 restored/31 bindings/37 pins/11 falsifiers;47 tools, prior bytes/seal/book/live gates; complete observer replay next |
| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.4` | `ARCHOGEN-M3-0777 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.4)` | complete observer16/full177:15 named K/1 scoped E, strict0;1097 original/copy restoration,31 bindings/37 pins/12 false receipts, production/control/reason bytes exact; close leaf/287 older archives exact, book/live sync; controller next |
| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.5` | `ARCHOGEN-M3-0779 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.5)` | complete controller124/full177:89K/24S/11U strict1;24 concrete owners,1099 restored/31 bindings/37 pins/12 false receipts; code/reasons exact, prior seal/289 archives exact, full book/live sync; P1 preparation next |
| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.1` | `ARCHOGEN-M3-0780 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.1)` | actual controller/base-env/native callee calibration and pinned recovery; new parent/catalogue3K/native3K full178,1102 restoration/33 bindings/37 pins/12 falsifiers; production/prior bodies/reasons exact, measurement seal/290 archives exact, book/live/focused gates; record positions next |
| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.2` | `ARCHOGEN-M3-0781 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.2)` | real multi-record original/literal calibration and every-file refusals/recovery; native14K/full179,1103 restored/33 bindings/37 pins/12 false receipts; stale index epoch reconciled to held tree, prior controller except new test/relative fixture exact;291 archives/new seal and six ledger notes preserved, focused/book/live gates; internal NUL next |
| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.3` | `ARCHOGEN-M3-0782 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.3)` | actual compilation-record internal-NUL original/literal calibration, permanent refusal/evidence/recovery; native2K/full180,1104 restored/33 bindings/37 pins/12 falsifiers; prior source/control/catalogue/reason bytes exact, record-position seal/292 archives exact; focused/book/live gates; exact emit marker next |
| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.4` | `ARCHOGEN-M3-0783 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.4)` | real compilation-record exact-prefix/equals original/literal calibration and permanent refusal/evidence/recovery; native2K/full181,1105 restored/33 bindings/37 pins/12 falsifiers; prior source/control/catalogue/reason bytes exact, internal-NUL seal/293 archives exact; focused/book/live gates; installed-byte verification next |
| 2026-10-19 | `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.5` | `ARCHOGEN-M3-0784 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.5)` | actual controller/native installed-byte original/literal corruption and pinned recovery; permanent macOS parent/native1K full182,1108 restored/35 bindings/37 pins/12 falsifiers; prior source/control/catalogue/reason bytes exact, emit-marker seal/294 archives exact; focused/book/live gates; installation close next |

### Complete native compiler observer measurement

Measurement `.15.2.1.3.3.3.2.1` (`2026-10-19`) executes the complete current observer population
listed by the qualification checkpoint. The unmodified full-parent baseline passes in 53 seconds.
Every candidate finishes: 14 named kills and two survivors, with no timeout, unjudged result,
compiler refusal or listed equivalent. The native command exits 1 because of the survivors;
this is a complete measurement, not an adequacy pass. The production source remains unchanged.

The controlling command runs inside the same repository write-boundary profile as the earlier
native measurements, with the repository-owned stores from `TOOLBOX.md`:

```sh
bash scripts/mutation_sweep.sh xtask/src/trust_compiler_observer.rs --package xtask \
  --timeout 600 --filter=trust:: --filter=trust_generated:: --filter=trust_gate:: \
  --filter=--skip --filter=route_tests --timeouts-fail
```

The following table gives a direct finishing control for each kill. Control names have the
common prefix `trust::compiler::tests::`; the receipt retains every finishing name, including
broader public inventory tests. The two survivor rows identify their owned next assessments.

| Current ID | Actual edit | Result | Direct control or scheduled owner |
| --- | --- | --- | --- |
| `L16.iter.0` | hex loop first | killed | `actual_native_receiver_observes_every_forwarded_argument_byte` |
| `L16.iter.1` | hex loop last | killed | `actual_native_receiver_observes_every_forwarded_argument_byte` |
| `L16.iter.2` | hex loop ends | killed | `actual_native_receiver_observes_every_forwarded_argument_byte` |
| `L17.stmt.3` | omit high hex nibble | killed | `actual_native_receiver_observes_every_forwarded_argument_byte` |
| `L18.stmt.4` | omit low hex nibble | killed | `actual_native_receiver_observes_every_forwarded_argument_byte` |
| `L34.bool.5` | disable write access | killed | `actual_native_receiver_observes_every_forwarded_argument_byte` |
| `L35.bool.6` | disable exclusive creation | killed | `actual_native_receiver_observes_every_forwarded_argument_byte` |
| `L60.stmt.7` | omit record write | killed | `actual_concurrent_observer_writers_publish_independent_complete_records` |
| `L61.stmt.8` | omit record sync | survived | P1 `.2.2` sync assessment, next |
| `L62.stmt.9` | omit explicit file close | survived | P2 `.2.3` close assessment, after sync |
| `L63.stmt.10` | omit no-overwrite publication | killed | `actual_observer_publication_collision_preserves_completed_and_partial_evidence` |
| `L64.stmt.11` | omit partial cleanup | killed | `actual_concurrent_observer_writers_publish_independent_complete_records` |
| `L74.stmt.12` | omit error diagnostic | killed | `actual_observer_publication_collision_preserves_completed_and_partial_evidence` |
| `L75.stmt.13` | omit failure exit | killed | `actual_observer_setup_and_installation_fail_before_child_execution` |
| `L45.chain.14` | record first argv only | killed | `actual_native_receiver_observes_every_forwarded_argument_byte` |
| `L45.chain.15` | record last argv only | killed | `actual_native_receiver_observes_every_forwarded_argument_byte` |

The sync omission survived despite the pre-publication sync action and error-propagation
contract. Its P1 assessment now adds the independent [failure control](#observer-synchronization-failure-control)
described below. The explicit-close omission has a separate P2 assessment of File lifetime,
consumer timing and the publication contract. Neither is called equivalent from survival; both
are scheduled before measurements of the other production files. The sync obligation now closes through the bounded native proof below; explicit-close
assessment remains open.

All 1091 held original and copied inputs restore by bytes/modes and independent Python/system
hashes; the gitlink contributes metadata only. The original and copied full parent lists agree on
176 names. All 28 direct source/caller/control bindings and 37 producer pins remain exact.
The receipt checker rejects changed scope, missing or duplicate results, wrong replacement,
wrong direct-control attribution, terminal counts, exit, source identity, outcome and an invented
equivalent reason. These are plumbing controls, not substitute proofs of equivalence.

Held native tree `93b487968f63bf4694a0aa143066cd98ea80aa4c` precedes completion-document staging.
Receipts are under `target/compiler-observer-full/`; the supplemental audit SHA-256 is
`df0418ff47bc286fe3b7b55a75acf26a5ca86380fbfa93fcb849c44b2b5188cc`.
The prior qualification subtree seals byte for byte. The historical provenance ledger above
preserves the original completed task notes verbatim while private routes retain their identities.
Final book/doctrine checks cover these documentation changes. Sync assessment is next; full
source/premise verification and independent review remain open before F30 signoff.

### Observer synchronization failure control

Assessment `.15.2.1.3.3.3.2.2` (`2026-10-19`) resolves the sync survivor as a missing regression
control. The production observer already propagates synchronization errors. A calibrated native
fault makes that observer exit 2 with partial evidence, while the exact omission exits 0 and
publishes a completed record. That is a behavior difference under the required I/O-failure contract,
so survival cannot justify an equivalent entry.

The [File synchronization API](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all)
describes explicit synchronization and error handling. Runtime conclusions here come from the
pinned executable, not a claim that current online documentation proves its implementation.
The probe first verified library loading. A `fsync`-only hook did not observe this macOS build:
the binary imports `fcntl`, and the actual sync path uses `F_FULLFSYNC`. A strict forwarding
tripwire exposed required metadata operations, including `F_GETPATH` and file-descriptor flags.
A pointer-identity probe then showed that `RTLD_NEXT` resolved the interposed symbol back to the
fixture itself. The corrected Mach-O fixture calls original symbols directly, following the
[public interposition interface](https://raw.githubusercontent.com/apple-oss-distributions/dyld/main/include/mach-o/dyld-interposing.h).
Those unsuccessful calibrations remain recorded; none is treated as a passing fault control.

The permanent control is
`trust::compiler::tests::actual_observer_sync_failure_retains_partial_evidence_and_recovers`.
It builds the tracked C fixture `scripts/fixtures/compiler_provenance_io_fault.c` with warnings as
errors, and runs a real pinned `rustc -vV` query through the actual observer. The fault library
clears its loader variables inside that observer, so the compiler child runs without the library.
The test compares compiler stdout with an independent direct query and requires successful child
status in the completed or partial record.

The control checks all of these boundaries:

- Ordinary operation without a library succeeds, publishes only the completed record, and creates no trace.
- The loaded library must report its constructor and exactly the observed parent sync call. The trace carries the actual observer PID and a valid descriptor.
- A programmed I/O error must produce observer exit 2 and useful diagnostics, with only a fully written partial record. All protocol fields match independent expected values; consumer refusal preserves the bytes.
- Loaded pass-through still succeeds and produces a valid query record. A new session then runs an ordinary real query successfully, leaving the failed evidence unchanged.

The preload path is relative to the canonical working directory, avoiding delimiter characters
in a relocated repository root. The macOS fixture forwards documented `fcntl` argument shapes
and refuses unexpected operations; it does not guess unknown variadic arguments. The ELF branch
uses the host preload mechanism. Runtime validation reported here is macOS; this record does not
claim local Linux execution. These controls inject a libc-boundary return error, not a physical
disk failure, power loss, or every possible kernel errno.

The direct parent passes in about 1.25 seconds on this run. Both maintained regressions finish
with its name: `trust-compiler-observer-sync-skipped` removes the call, and
`trust-compiler-observer-sync-error-ignored` retains the call but discards its error. The second
case ensures the control holds failure behavior as well as observing an actual synchronization.

```sh
cargo test -p xtask trust::compiler::tests::actual_observer_sync_failure_retains_partial_evidence_and_recovers -- --exact
cargo xtask mutate --timeouts-fail --only trust-compiler-observer-sync-skipped trust-compiler-observer-sync-error-ignored
```

The final native omission replay uses the complete current parent population of 177 names.
Its unmodified baseline passes in 56 seconds, and the exact `L61.stmt.8` omission finishes with
only the new control as its killing name, strict exit 0. All 1093 original/copied regular inputs
restore by bytes/modes and independent Python/system hashes; gitlinks remain metadata only.
All 29 direct bindings and 37 producer pins bind. Eleven false receipt controls reject wrong
scope, identity, result, name, counts, exit or invented equivalence.

The observer production bytes and the controller production prefix are unchanged. Removing the
new parent from the controller recovers its prior file exactly; the old catalogue is an exact
prefix with only the two new regressions appended. The prior complete observer result remains
the dated 14-kill/two-survivor measurement. This bounded replay resolves the sync obligation;
the explicit-close survivor still needs its owned assessment, followed by complete current
source and premise verification before F30 signoff.

Held native tree `97f50a2d181ff6fd3f213e9e60d7cd5478ddec76` precedes completion-document/focused
staging. Receipts are under `target/compiler-observer-sync/`, including failed calibrations and
the initial stand-in native epoch. The final supplemental audit SHA-256 is
`185c0854c51a77269b79b4f10203f862f9c4454dc43720c438ec0d2292ef7013`.
The preceding measurement leaf seals exactly with all 285 older archives unchanged. Final
staged focused/book/doctrine checks close this control slice; explicit-close assessment is next.

### Observer explicit-close assessment

`2026-10-19`, leaf `.15.2.1.3.3.3.2.3`: the actual `L62.stmt.9` edit replaces
`drop(file);` with its inert comment. The complete earlier measurement found it survived.
The current assessment supplies a source/API/caller proof and permanent controls before
recording a scoped equivalent in `xtask/trust_compiler_observer.equivalents`.
The observer production source remains SHA-256
`64b8376bf404a32f3097071f7737a721e789133f716d16e05f8d0ceeb3e38475`.

#### What the protocol observes

The concrete writer is an exclusively created, unbuffered `std::fs::File`. It has no clones,
locks or ownership transfer. The complete record is written and `sync_all` must succeed before
publication. After explicit close, the only filesystem operations are path-based hard-link
publication and removal of the partial name; neither opens another descriptor. There are no
later record writes. The only other work computes the compiler's already available status.

The [File API](https://doc.rust-lang.org/std/fs/struct.File.html) specifies automatic close on
scope exit, ignored destructor close errors and unbuffered reads/writes. The
[Rust drop-scope rules](https://doc.rust-lang.org/reference/destructors.html#drop-scopes)
put the local writer in `observe`'s scope. Removing the explicit call therefore moves its
destructor to that function's return, including either publication error propagated with `?`.
`main` calls `process::exit` only after `observe` has returned. These public documents explain
the semantics; the current online standard-library page is not the pinned compiler's source.
Actual pinned executions below independently check the relevant behavior.

The public [hard-link interface](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/link.2.html)
creates another directory name for the same object; removal of the partial name leaves the
completed name. The [unlink interface](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/unlink.2.html)
removes a directory entry, with final object reclamation depending on remaining links and open
references. Here the successful hard link precedes partial-name removal. The proof uses ordinary
Unix regular-file semantics. It does not establish behavior for an arbitrary filesystem with
additional close-sensitive rules, injected resource schedules or crash timing.

The production caller census finds the private controller embedded by `trust.rs`.
Its `compile` function creates the session, calls `run_cargo`, and only then reads
`Session::invocations`. `run_cargo` uses synchronous `Command::output` and refuses unsuccessful
Cargo status. Consequently the supported reader sees records after the observer has returned
and closed its writer in either version. No production streaming reader or descriptor inspection
occurs between publication operations. Earlier setup, child, write and synchronization error
paths precede the edited statement and retain the same destructor behavior.

This gives the source proof: under those concrete premises, moving this ignored-error
unbuffered destructor across these path operations changes none of the required record bytes,
no-replacement publication, partial evidence, diagnostics or exit/signal results. The native
samples validate the premises and controls; surviving tests alone are not that proof.

#### Actual native controls and their limits

`ObserverCloseTests` in `xtask/mutation_sweep_tests.py` discovers and applies the real omission,
compiles the complete original and changed observer, and compares actual pinned `rustc -vV`
stdout with an independent direct query. Each record must match an independently constructed
complete protocol object, actual process identity and exact expected filenames. Only process
identity is removed for cross-run record comparison. Diagnostics must also agree.

| Mode, applied to both executables | Required result |
| --- | --- |
| No fault library | Success, only a completed query record, no trace |
| Loaded pass-through | Same protocol success; parent-only trace confirms synchronization and publication |
| Link return error | Exit 2 with useful diagnostics and only the complete partial record |
| Unlink return error | Exit 2 with diagnostics and both partial and completed names |
| Close return error after real close | Success in both; the destructor ignores that return error |

The macOS C fixture `scripts/fixtures/compiler_provenance_close.c` uses the public Mach-O
interposition interface. Its constructor clears loader variables before the compiler child.
It tracks the synchronized record descriptor, forwards documented `fcntl` argument shapes,
and refuses unknown operations. Normal close, link and unlink call the real libc functions.
Fault modes return an I/O error at a selected boundary. The close-error mode really closes first,
then reports the programmed error; it does not model every possible failed-close kernel state.
The trace itself preserves errno and must report only the observer's actual PID.

The successful original trace is `loaded → full-sync → closed → link → unlink`.
The omission trace is `loaded → full-sync → link → unlink → closed`.
Link/unlink errors also show the deferred destructor before process termination.
Publication sees the live record descriptor only in the omission. This is a deliberately
observed wider-domain difference: a claim that descriptor lifetime or syscall timing is equal
must fail. Wrong negative protocol expectations also fail. These controls validate a specific
libc boundary on macOS; they make no claim of local Linux execution or physical-disk fault coverage.

Separate premise controls hash the complete production prefixes of observer, controller and
caller, bind the pinned toolchain and require the unique actual omission and per-file reason.
Changed-byte falsifiers reject each source binding. These controls live outside the semantic
parent mutation population, so a hash mismatch cannot manufacture a killing test for a mutant.
Changes to writer type, buffering, locks, operations after synchronization, platform semantics,
producer or reader ordering require revalidation. The initial prefix-check attempt mixed two
trailing-newline conventions; retained diagnostics and Python/system hashes identify and correct
that boundary without changing production code.

```sh
python3 -I -B xtask/mutation_sweep_tests.py "$PWD" ObserverCloseTests
bash scripts/mutation_sweep.sh xtask/src/trust_compiler_observer.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L62.stmt.9 --equivalents xtask/trust_compiler_observer.equivalents
```

The permanent close controls pass in about 1.10 seconds; the complete tool suite passes all
47 tests in 151.443 seconds. The selected native replay uses all 177 current parent names:
baseline 59 seconds, one scoped equivalent, strict exit 0. All 1096 original/current-copy
regular inputs restore by bytes/modes and Python/system hashes. All 31 direct bindings and
37 producer pins bind; eleven false receipt arms refuse altered scope, source, replacement,
reason, outcome, totals, exit or missing/duplicate results. Gitlinks remain metadata only.

The prior tool file is recovered byte for byte by removing the new test class. All production
sources, prior equivalents, catalogue and sync fixture remain exact. The committed sync leaf
seals exactly with all 286 older archives unchanged; the extra inter-record separator newline
is restored during reconstruction. Receipts are under `target/compiler-observer-close/`.
Held native tree `826d5ded634f4fe55261a1e3555b5286bddef7b2` precedes final documentation staging;
supplemental audit SHA-256 is
`309adf235fb12ec720f957cc4f7e42f2dbbbca204621730c4bcd4232c9417e65`.
Completed prerequisite/measurement commit notes are retained verbatim in the book ledger;
private rows keep their identities and links. Reconstruction checks every other prior commit
row unchanged and rejects missing notes, duplicates or changed identities. The book remains
unbounded. Complete current observer replay is next, before other-source and premise/integration review.

### Complete current observer replay

`2026-10-19`, leaf `.15.2.1.3.3.3.2.4`: every actual observer candidate now has a
finishing disposition on the current parent population. The complete native result is fifteen
named kills and one scoped equivalent, strict exit 0; there are no timeouts, unjudged results,
compiler refusals or unlisted survivors. The earlier fourteen-kill/two-survivor measurement
remains a historical record. The synchronization failure control resolves its first obligation;
the explicit-close proof resolves the other within its stated protocol domain.

The production file remains SHA-256
`64b8376bf404a32f3097071f7737a721e789133f716d16e05f8d0ceeb3e38475`.
Actual discovery agrees exactly with the complete CLI listing, from the first production
function through the first standalone test boundary. Every actual replacement and identity
is checked, including the complete population rather than a selected subset. A listing
preflight initially omitted the required package option and refused before any copy or
baseline work; the corrected command below agrees with maintained discovery.

```sh
bash scripts/mutation_sweep.sh xtask/src/trust_compiler_observer.rs --package xtask --list
python3 -I -B xtask/mutation_sweep_tests.py "$PWD" \
  ObserverCloseTests.test_close_reason_requires_current_production_and_caller_premises
bash scripts/mutation_sweep.sh xtask/src/trust_compiler_observer.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --equivalents xtask/trust_compiler_observer.equivalents
```

The unmodified copied baseline passes in 54 seconds with the complete current population of
177 parent names. Every kill reports its finishing test names, all belonging to that population.
The table gives the number of names and one actual direct observer control for each kill;
it does not present that example as the only killing test. The collision control is
`trust::compiler::tests::actual_observer_publication_collision_preserves_completed_and_partial_evidence`;
the sync control is
`trust::compiler::tests::actual_observer_sync_failure_retains_partial_evidence_and_recovers`.

| Actual ID | Disposition | Finishing names | Direct control or reason |
| --- | --- | --- | --- |
| `L16.iter.0` | killed | 119 | [collision](#a-deterministic-publication-collision) |
| `L16.iter.1` | killed | 119 | [collision](#a-deterministic-publication-collision) |
| `L16.iter.2` | killed | 119 | [collision](#a-deterministic-publication-collision) |
| `L17.stmt.3` | killed | 119 | [collision](#a-deterministic-publication-collision) |
| `L18.stmt.4` | killed | 119 | [collision](#a-deterministic-publication-collision) |
| `L34.bool.5` | killed | 119 | [collision](#a-deterministic-publication-collision) |
| `L35.bool.6` | killed | 119 | [collision](#a-deterministic-publication-collision) |
| `L60.stmt.7` | killed | 119 | [collision](#a-deterministic-publication-collision) |
| `L61.stmt.8` | killed | 1 | [sync](#observer-synchronization-failure-control) |
| `L62.stmt.9` | equivalent | 0 | [close proof](#observer-explicit-close-assessment) |
| `L63.stmt.10` | killed | 119 | [collision](#a-deterministic-publication-collision) |
| `L64.stmt.11` | killed | 118 | [collision](#a-deterministic-publication-collision) |
| `L74.stmt.12` | killed | 3 | [collision](#a-deterministic-publication-collision) |
| `L75.stmt.13` | killed | 4 | [collision](#a-deterministic-publication-collision) |
| `L45.chain.14` | killed | 119 | [collision](#a-deterministic-publication-collision) |
| `L45.chain.15` | killed | 119 | [collision](#a-deterministic-publication-collision) |

The sync omission has exactly its intended new finishing name. The close omission survives
and matches only the exact maintained per-file reason. Its source/caller/toolchain premise
control passes again. No source or test changes were made for this final replay: observer,
controller, caller, artifact binder, mutation catalogue, tool controls, old reasons and sync/close
fixtures are byte for byte the committed versions. The preceding code-qualified commit also
passed formatting, strict lint and the complete focused test suite in 154.81 seconds, staged
tree `1aed62fad517f44f67b7af3e755f2736c1f53eaa`.

All 1097 original and current copied regular inputs restore by bytes/modes and independent
Python/system hashes. The original and copied parent lists agree exactly. All 31 direct
bindings and 37 pinned public producer inputs bind, with gitlinks recorded only as metadata.
The supplemental audit re-discovers the entire candidate list and checks every actual
replacement, outcome, named kill, reason and terminal count. It rejects twelve altered receipts:
wrong scope, missing result, changed replacement, invented name, wrong total, wrong exit,
wrong file, wrong source hash, changed outcome, duplicate result, invented reason and reduced
population. The committed close assessment seals exactly and all 287 older archives remain
unchanged; reconstruction retains the structural separator newline.

Held native tree `43acf850fb92f7c4c5249e3578c8afffffb1b6e1` precedes completion-document staging.
Receipts are under `target/compiler-observer-final/`, including the listing, plan, full raw
native output, per-edit finishing names, original/copy restoration and source audit.
The supplemental audit SHA-256 is
`12c3d62f20fdcc57ba073250d48dafbc6188855db172dd0e45f01b60b40cb2a6`.

This closes the current observer replay. The remaining controller, artifact and caller
populations, their producer/caller premises, broader integration and independent review remain
owned before F30. Next is the current controller's complete native measurement; this observer
result makes no adequacy projection to those other files.

### Complete native compiler controller measurement

`2026-10-19`, leaf `.15.2.1.3.3.3.2.5`: every actual edit in the controller's complete
production range is measured against all current parents. The result is 89 named kills,
24 survivors and 11 compiler refusals, strict exit 1. There are no timeouts, unjudged results
or equivalent classifications. This completes the measurement; the surviving edits remain
owned test-adequacy obligations before the controller can meet the source criterion.

The controller `xtask/src/trust_compiler.rs` is SHA-256
`cfb16b7b6c97e52a3b702429a6400db9a196bff07ddc5f1fef1ac5b224f9c1e5`.
The range begins at `fresh` and ends before the standalone test module: source lines 31 through
307. Maintained discovery agrees exactly with the complete CLI listing and every replacement.
The observer, artifact binder, caller, controls, catalogue and reasons remain committed bytes.
The prior observer close reason is specific to its own file and was not applied to this one.

```sh
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs --package xtask --list
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail
```

These commands reproduce this record only with the matching source/caller/producer epoch;
later changes require a new measurement. The unmodified copied baseline passes in 54 seconds,
and original/restored-copy lists agree on all 177 parent names. The directory-retry loop's
first/last/endpoint edits deliberately make retries spin; bounded parent workers finish and
supply named kills. They are not native timeout classifications.

#### Owned survivor resolution

All survivor ownership below shares prefix `M3.6.9.6.3.2.2.15.2.1.3.3.3.2`.
Container `.6` owns their resolution and final full replay. Each child commits separately;
preparation is first, followed by the remaining P1 controls and then the P2 assessments.
Source repairs or scoped proofs follow diagnosis, rather than being inferred from survival.

| Child | Priority | Boundary | Exact original survivor IDs |
| --- | --- | --- | --- |
| `.6.1` | P1 | Preparation status, output and selected environment | `L72.cond.7`, `L80.cond.11`, `L68.chain.101` |
| `.6.2` | P1 | Record validation at every position | `L201.guard.36`, `L201.guard.38`, `L201.guard.39`, `L201.guard.40`, `L201.guard.41`, `L201.guard.42`, `L201.guard.43`, `L201.guard.44`, `L201.guard.45`, `L201.guard.46`, `L201.guard.47`, `L201.guard.48`, `L201.guard.49`, `L201.guard.50` |
| `.6.3` | P1 | Internal NUL argument bytes | `L168.contains.109`, `L168.contains.111` |
| `.6.4` | P1 | Exact compilation emit marker | `L265.dir.85`, `L265.lit.86` |
| `.6.5` | P1 | Installed observer byte verification | `L120.cond.18` |
| `.6.6` | P2 | Installation explicit-close assessment | `L119.stmt.15` |
| `.6.7` | P2 | Record-presence state | `L271.int.92` |

The preparation cases disable failure-status and empty-output checks or forward only the
first selected environment entry. Installation cases remove explicit close or installed-byte
comparison. The close assessment must account for the later file read, which allocates a
descriptor; it cannot borrow the observer's path-only argument.

The position mutations still visit every entry. They restrict only one validation check to
the first entry, last entry, or the two endpoints. Their exact predicate families are:

| Original check | Surviving positions |
| --- | --- |
| Regular file and `.json` metadata | First; endpoints |
| Exact field count and canonical JSON framing | First; last; endpoints |
| Filename equals the record's PID identity | First; last; endpoints |
| Compiler argument names `rustc` | First; last; endpoints |
| Non-compilation invocation is a known query | First; last; endpoints |

The remaining scalar cases broaden `--emit=` prefix matching to substring matching or a
prefix without the equals sign; restrict NUL detection to a byte-string endpoint; or change
`count += 1` to `count += 2` when the value is used only to test presence. Each has its own
owner. The presence assessment may simplify that state directly; no platform limit or
arithmetic equivalence has been adopted from these observations.

Controlled preparation processes, multi-record corruption, internal argument bytes, marker
boundaries, installation readback and descriptor lifetime are planned diagnostics, not claimed
fixes. Full current controller replay follows the resolutions, before another source pivot.

#### Every actual outcome

The table retains every current ID and its method. For a kill, the final column shows one
actual finishing control and the number counts every reported name; the example is not claimed
to be the only killing test. Full raw names and replacement spans remain in the retained
receipt, and maintained discovery regenerates the exact edit from this source epoch.
Compiler refusals are not semantic coverage. The owning child appears for each survivor.

| Actual ID | Method | Outcome | Finishing names | Control label or owner |
| --- | --- | --- | --- | --- |
| `L32.stmt.0` | `fresh` | killed | 119 | `C1` |
| `L33.iter.1` | `fresh` | killed | 48 | `C1` |
| `L33.iter.2` | `fresh` | killed | 119 | `C1` |
| `L33.iter.3` | `fresh` | killed | 48 | `C1` |
| `L37.eq.4` | `fresh` | killed | 48 | `C1` |
| `L55.stmt.5` | `prepare` | killed | 119 | `C1` |
| `L72.cond.6` | `prepare` | killed | 119 | `C1` |
| `L72.cond.7` | `prepare` | survived | 0 | `.6.1` |
| `L72.not.8` | `prepare` | killed | 119 | `C1` |
| `L79.stmt.9` | `prepare` | unviable | 0 | Compiler refusal |
| `L80.cond.10` | `prepare` | killed | 119 | `C1` |
| `L80.cond.11` | `prepare` | survived | 0 | `.6.1` |
| `L95.eq.12` | `session` | killed | 3 | `C2` |
| `L106.bool.13` | `session` | killed | 119 | `C1` |
| `L107.bool.14` | `session` | killed | 119 | `C1` |
| `L119.stmt.15` | `session` | survived | 0 | `.6.6` |
| `L120.eq.16` | `session` | killed | 119 | `C1` |
| `L120.cond.17` | `session` | killed | 119 | `C1` |
| `L120.cond.18` | `session` | survived | 0 | `.6.5` |
| `L124.stmt.19` | `session` | killed | 119 | `C1` |
| `L126.stmt.20` | `session` | unviable | 0 | Compiler refusal |
| `L147.not.21` | `decode_hex` | killed | 118 | `C1` |
| `L148.logic.22` | `decode_hex` | killed | 2 | `C3` |
| `L148.not.23` | `decode_hex` | killed | 118 | `C1` |
| `L150.logic.24` | `decode_hex` | killed | 118 | `C1` |
| `L159.cmp.25` | `decode_hex` | killed | 118 | `C1` |
| `L159.cond.26` | `decode_hex` | killed | 118 | `C1` |
| `L159.cond.27` | `decode_hex` | killed | 118 | `C1` |
| `L162.int.28` | `decode_hex` | killed | 118 | `C1` |
| `L162.int.29` | `decode_hex` | killed | 118 | `C1` |
| `L168.cond.30` | `decode_hex` | killed | 118 | `C1` |
| `L168.cond.31` | `decode_hex` | killed | 1 | `C4` |
| `L192.stmt.32` | `environment` | unviable | 0 | Compiler refusal |
| `L201.iter.33` | `invocations` | killed | 111 | `C5` |
| `L201.iter.34` | `invocations` | killed | 111 | `C5` |
| `L201.iter.35` | `invocations` | killed | 102 | `C5` |
| `L201.guard.36` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.37` | `invocations` | killed | 1 | `C3` |
| `L201.guard.38` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.39` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.40` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.41` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.42` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.43` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.44` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.45` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.46` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.47` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.48` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.49` | `invocations` | survived | 0 | `.6.2` |
| `L201.guard.50` | `invocations` | survived | 0 | `.6.2` |
| `L205.eq.51` | `invocations` | killed | 118 | `C1` |
| `L205.logic.52` | `invocations` | killed | 3 | `C1` |
| `L205.logic.53` | `invocations` | killed | 2 | `C3` |
| `L205.logic.54` | `invocations` | killed | 3 | `C1` |
| `L205.cond.55` | `invocations` | killed | 118 | `C1` |
| `L205.cond.56` | `invocations` | killed | 3 | `C1` |
| `L205.not.57` | `invocations` | killed | 118 | `C1` |
| `L216.eq.58` | `invocations` | killed | 118 | `C1` |
| `L216.eq.59` | `invocations` | killed | 118 | `C1` |
| `L216.logic.60` | `invocations` | killed | 2 | `C3` |
| `L216.logic.61` | `invocations` | killed | 2 | `C3` |
| `L216.logic.62` | `invocations` | killed | 2 | `C3` |
| `L216.cond.63` | `invocations` | killed | 118 | `C1` |
| `L216.cond.64` | `invocations` | killed | 2 | `C3` |
| `L222.eq.65` | `invocations` | killed | 118 | `C1` |
| `L223.eq.66` | `invocations` | killed | 118 | `C1` |
| `L223.logic.67` | `invocations` | killed | 2 | `C3` |
| `L224.eq.68` | `invocations` | killed | 118 | `C1` |
| `L224.logic.69` | `invocations` | killed | 2 | `C3` |
| `L225.eq.70` | `invocations` | killed | 118 | `C1` |
| `L225.logic.71` | `invocations` | killed | 2 | `C3` |
| `L233.cmp.72` | `invocations` | killed | 1 | `C4` |
| `L237.eq.73` | `invocations` | killed | 118 | `C1` |
| `L237.cond.74` | `invocations` | killed | 118 | `C1` |
| `L237.cond.75` | `invocations` | killed | 1 | `C3` |
| `L242.eq.76` | `invocations` | killed | 118 | `C1` |
| `L243.eq.77` | `invocations` | killed | 118 | `C1` |
| `L243.logic.78` | `invocations` | killed | 3 | `C1` |
| `L259.eq.79` | `invocations` | killed | 118 | `C1` |
| `L259.cond.80` | `invocations` | killed | 118 | `C1` |
| `L259.cond.81` | `invocations` | killed | 2 | `C3` |
| `L262.eq.82` | `invocations` | killed | 1 | `C4` |
| `L264.logic.83` | `invocations` | killed | 1 | `C4` |
| `L265.logic.84` | `invocations` | killed | 1 | `C4` |
| `L265.dir.85` | `invocations` | survived | 0 | `.6.4` |
| `L265.lit.86` | `invocations` | survived | 0 | `.6.4` |
| `L265.lit.87` | `invocations` | killed | 113 | `C1` |
| `L267.stmt.88` | `invocations` | killed | 113 | `C1` |
| `L268.cond.89` | `invocations` | killed | 118 | `C1` |
| `L268.cond.90` | `invocations` | killed | 2 | `C3` |
| `L268.not.91` | `invocations` | killed | 118 | `C1` |
| `L271.int.92` | `invocations` | survived | 0 | `.6.7` |
| `L271.int.93` | `invocations` | killed | 118 | `C1` |
| `L271.stmt.94` | `invocations` | killed | 118 | `C1` |
| `L273.eq.95` | `invocations` | killed | 118 | `C1` |
| `L273.cond.96` | `invocations` | killed | 118 | `C1` |
| `L273.cond.97` | `invocations` | killed | 1 | `C3` |
| `L281.eq.98` | `query` | killed | 118 | `C1` |
| `L282.logic.99` | `query` | killed | 118 | `C1` |
| `L283.eq.100` | `query` | killed | 112 | `C3` |
| `L68.chain.101` | `prepare` | survived | 0 | `.6.1` |
| `L68.chain.102` | `prepare` | killed | 119 | `C1` |
| `L94.chain.103` | `session` | killed | 119 | `C1` |
| `L94.chain.104` | `session` | killed | 119 | `C1` |
| `L150.contains.105` | `decode_hex` | unviable | 0 | Compiler refusal |
| `L150.contains.106` | `decode_hex` | unviable | 0 | Compiler refusal |
| `L150.contains.107` | `decode_hex` | unviable | 0 | Compiler refusal |
| `L150.contains.108` | `decode_hex` | unviable | 0 | Compiler refusal |
| `L168.contains.109` | `decode_hex` | survived | 0 | `.6.3` |
| `L168.contains.110` | `decode_hex` | unviable | 0 | Compiler refusal |
| `L168.contains.111` | `decode_hex` | survived | 0 | `.6.3` |
| `L168.contains.112` | `decode_hex` | unviable | 0 | Compiler refusal |
| `L233.chain.113` | `invocations` | unviable | 0 | Compiler refusal |
| `L233.chain.114` | `invocations` | unviable | 0 | Compiler refusal |
| `L233.chain.115` | `invocations` | killed | 1 | `C4` |
| `L251.chain.116` | `invocations` | killed | 118 | `C1` |
| `L251.chain.117` | `invocations` | killed | 118 | `C1` |
| `L262.chain.118` | `invocations` | killed | 113 | `C1` |
| `L262.chain.119` | `invocations` | killed | 113 | `C1` |
| `L265.chain.120` | `invocations` | killed | 113 | `C1` |
| `L265.chain.121` | `invocations` | killed | 113 | `C1` |
| `L150.clogic.122` | `decode_hex` | killed | 118 | `C1` |
| `L150.clogic.123` | `decode_hex` | killed | 118 | `C1` |

The illustrated controls are:

| Label | Exact finishing test |
| --- | --- |
| `C1` | `trust::compiler::tests::actual_compiler_removes_private_environment_and_records_failures` |
| `C2` | `trust::compiler::tests::actual_observer_setup_and_installation_fail_before_child_execution` |
| `C3` | `trust::compiler::tests::actual_query_records_reject_missing_stale_foreign_and_malformed_inputs` |
| `C4` | `trust::compiler::tests::actual_records_refuse_remaining_field_range_byte_and_shape_boundaries` |
| `C5` | `trust::compiler::tests::actual_concurrent_observer_writers_publish_independent_complete_records` |

All 1099 original/current-copy regular inputs restore by bytes, modes and independent
Python/system hashes. All 31 direct bindings and 37 public producer pins agree; gitlinks
remain metadata only. The audit re-discovers the entire population and rejects twelve altered
receipts: wrong scope, missing result, changed replacement, invented name, wrong totals,
wrong exit, wrong file/hash, changed outcome, duplicate result, invented equivalence and
reduced population. The committed observer replay seals exactly with all 289 older archives
unchanged, including its structural separator newline.

Held native tree `5f2358cac0bb909f92b1e7e41d9794da24e877b1` precedes completion-document
staging. Receipts are under `target/compiler-controller-full/`, with the source/listing plan,
full raw run, per-edit finishing names, operator refusals, original/copied restoration and
supplemental source audit. Audit SHA-256:
`1e752e3d046b474156d1f94baf58dc42ddd210fc9a3656a5be00c0d6592b60cc`.
Six earlier completed task notes move verbatim into the book ledger; private rows keep their
identities and links. Reconstruction preserves every other prior commit row and rejects lost
notes, duplicate records or changed identities.
The code and tests are unchanged for this measurement; the unmutated parent baseline is the
core check, followed by pinned book and final doctrine checks for completion documentation.
The artifact/caller populations and wider source/premise/integration/review obligations remain
open before F30. Next is `.6.1` preparation failure/output/environment control.

### Compiler preparation boundary controls

`2026-10-19`, leaf `.15.2.1.3.3.3.2.6.1`: the preparation status, empty-output and
selected-environment obligations now have finishing controls. Production preparation already
contains the intended checks and forwarding. This slice adds a native component witness,
permanent parent and regressions while preserving the production prefix exactly.

The control compiles the complete actual controller with this project's actual JSON and
embedded observer sources. It retains the real `catalog_build::environment` allowlist constructor.
A test-only wrapper prepends a relative directory containing a controlled native callee to
`PATH`; all other selected values remain unchanged. The helper captures the selected vector,
and the independent C callee records the environment it actually receives as zero-delimited
`KEY=VALUE` bytes. Comparison requires every key/value and rejects duplicates or inherited
marker leakage. The selected keys are `PATH`, `HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN`,
`CARGO_HOME`, `CARGO_TARGET_DIR`, `TMPDIR`, `TMP` and `TEMP`.

The relative `PATH` component introduces no checkout-root delimiter into the test callee lookup.
The constructor still creates repository-owned stores and canonical temporary paths. The
standalone helper runs in an isolated child, so the test never changes the parent process's
global environment. Controller source is embedded from the compiled test's context, including
an actual native mutation; the pristine source-root hint is not substituted for it.

The C callee checks the real preparation argument shape and nonempty source file. It then
records its received environment and performs the selected output/status operation:

| Controlled case | Native behavior | Required preparation result |
| --- | --- | --- |
| Failed | Copy a real pinned observer image, then return 7 with a deliberate diagnostic | Refuse and retain the useful failure diagnostic despite nonempty output |
| Empty | Create an empty output and return 0 | Refuse the empty observer |
| Success | Copy the real image and return 0 | Accept; install and run a fresh real pinned query |

The image is first compiled by the ordinary pinned compiler. Nonempty outputs must match
those bytes exactly and have executable permissions. In the successful case, the helper
installs the prepared observer using the real base environment, compares a real `rustc -vV`
query with an independent direct query, and requires the typed consumer to accept a query-only
record without producing units. This is a controlled component callee, with real process,
filesystem, status and environment effects; it does not claim to induce a genuine rustc
internal failure or a physical I/O fault.

Initial actual-source probes compile original plus each literal edit separately. Disabling
the status check accepts the failed-but-nonempty case. Disabling the byte guard accepts the
empty case. Forwarding only the first environment entry preserves those output cases but
loses the rest of the selected environment. The original passes every independent expectation,
and each actual variant fails its intended boundary. The permanent control is
`trust::compiler::tests::actual_preparation_holds_status_output_and_selected_environment`;
it passes directly in 1.08 seconds. The C and Rust helper fixtures are tracked under
`scripts/fixtures/compiler_preparation_callee.c` and
`scripts/fixtures/compiler_preparation_probe.rs`.

```sh
cargo test -p xtask trust::compiler::tests::actual_preparation_holds_status_output_and_selected_environment -- --exact
cargo xtask mutate --timeouts-fail --only \
  trust-compiler-prepare-failed-status-accepted \
  trust-compiler-prepare-empty-output-accepted \
  trust-compiler-prepare-selected-environment-truncated
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L72.cond.7 --only L80.cond.11 --only L68.chain.101
```

All maintained catalogue anchors pass. The catalogue regressions finish with the intended
new parent in 1.7, 1.8 and 2.0 seconds. The bounded native replay uses all 178 current parent
names: baseline 50 seconds, all three actual edits killed, only that new finishing name,
strict exit 0 and no timeout, unjudged result or compiler refusal. The complete controller
listing remains the original production population; this replay selects only these obligations.

All 1102 original/current-copy regular inputs restore by bytes/modes and independent
Python/system hashes. All 33 direct bindings and 37 public producer pins bind; twelve altered
scope/source/replacement/name/outcome/count/exit/population receipts reject. Gitlinks remain
metadata only. Removing the new parent recovers the entire previous controller exactly;
the old catalogue remains an exact prefix, with three regressions appended. Other production,
controls and reasons remain byte for byte unchanged. The committed measurement seals exactly
with all 290 older archives unchanged.

Held native tree `d79d96e319108baf2b68da46d89b432de2ef48d6` precedes completion-document and
focused staging. Receipts, direct original/literal calibration and retained component outputs
are under `target/compiler-prepare-controls/`. Supplemental audit SHA-256:
`2bb6af43d44b09aa5bcc2d78b09a717f46af95e20e823eabe474f68d2c8a0699`.
Final staged focused/book/doctrine checks close this slice. The original controller result
remains its dated measurement; remaining ownership groups require their own controls and
final complete replay. Next is record validation across real multi-record inputs.

### Compiler record validation at every position

`2026-10-19`, leaf `.15.2.1.3.3.3.2.6.2`: record validation now has a permanent
multi-record control. The original production consumer already checks each record; the
remaining position variants weakened those checks while earlier controls stayed green.
The production prefix is unchanged.

The control starts a real observer session, runs pinned version queries, and compiles
distinct programs through it. It executes the resulting programs and independently compares
the consumer's complete compilation vectors with the original arguments. Query records
must remain excluded from those vectors. Returned vector order is normalized only for this
comparison; directory iteration order is never assumed.

`2026-10-19`: the seed contains three real query records and two real compilation records.
Each seeded filename is temporarily replaced with a relative symbolic link to its original
bytes in a sibling directory. The test reads the link and target to prove it is relative and
valid, adds a fresh real query record, and requires the regular-file refusal. Both link and
held bytes must remain intact. Restoring the regular file must restore the exact expected
compilation vectors.

`2026-10-19`: after those five link cases, the directory contains ten real records.
Every filename then receives each of the following corruptions, one at a time:

| Corruption | Independent refusal expectation | Position variants |
| --- | --- | --- |
| Valid relative link to original record bytes | Incomplete or unexpected record | `L201.guard.36`, `L201.guard.38` |
| Additional field in otherwise canonical JSON | Unexpected fields | `L201.guard.39`, `L201.guard.40`, `L201.guard.41` |
| Leading whitespace before otherwise valid JSON | Noncanonical framing | Same canonical-record group |
| Positive PID changed while filename stays fixed | Filename differs from invocation identity | `L201.guard.42`, `L201.guard.43`, `L201.guard.44` |
| Compiler name changed while other arguments remain | Another compiler | `L201.guard.45`, `L201.guard.46`, `L201.guard.47` |
| Unknown query vector in a real producer record | Unknown invocation shape | `L201.guard.48`, `L201.guard.49`, `L201.guard.50` |

Each refusal must retain the corrupted bytes. The test restores the original file, checks
successful reading again, and compares all regular record bytes with the full snapshot.
Both compilation and query records receive every content fault. This distinguishes
validity of the whole directory from merely validating an endpoint.

The older single-record symbolic-link fixture also stored an absolute target. Reading its
actual persisted link reproduced that portability-policy defect while confirming the target
was valid. Its replacement computes a relative target and explicitly verifies the same
bytes; the metadata refusal still has a real, readable target. No production behavior changes.

The permanent parent is
`trust::compiler::tests::actual_record_validation_applies_to_every_real_record`.
The corrected older parent is
`trust::compiler::tests::actual_query_records_reject_missing_stale_foreign_and_malformed_inputs`.

```sh
cargo test -p xtask trust::compiler::tests::actual_record_validation_applies_to_every_real_record -- --exact
cargo test -p xtask trust::compiler::tests::actual_query_records_reject_missing_stale_foreign_and_malformed_inputs -- --exact
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L201.guard.36 --only L201.guard.38 --only L201.guard.39 \
  --only L201.guard.40 --only L201.guard.41 --only L201.guard.42 \
  --only L201.guard.43 --only L201.guard.44 --only L201.guard.45 \
  --only L201.guard.46 --only L201.guard.47 --only L201.guard.48 \
  --only L201.guard.49 --only L201.guard.50
```

Original/literal calibration runs the original and each actual production edit separately
against the new parent. The original succeeds and every edited controller produces a
finishing failed-test summary. Direct permanent tests pass. The native replay uses all current trust parents.

`2026-10-19`: baseline 50 seconds; all fourteen selected edits finish killed by only the new
parent, strict exit 0. All 179 current parent names bind. There is no timeout, unjudged result,
compiler refusal or equivalence judgment in this selected replay. The full production listing
remains unchanged; final complete controller measurement remains a separate owned leaf.

The first restoration check correctly refused its stale index epoch: the input census had
been captured before staging. A full difference census found unchanged working bytes/modes,
three updated index entries and the newly staged preparation archive. The original census and
refusal remain retained. Reconciliation checks every regular blob against the immutable held
Git tree, then checks originals and the restored copy with independent Python/system hashes.
The reconciliation's initial permission assertion was also too strict: Git regular-file mode
does not encode write bits, and an existing read-only policy file legitimately retains them.
The corrected check preserves observed permissions and compares the executable bit with Git.

`2026-10-19`: all 1103 activation inputs restore; all 33 direct bindings and 37 public pins
agree. Twelve altered source/scope/edit/name/outcome/count/exit/population receipts reject.
Removing the new parent and reversing only the explicit relative-link fixture correction
reconstructs the entire previous controller exactly. Other code, prior controls, catalogue and
reasons are unchanged. The committed preparation body seals exactly, and all 291 prior
archives retain their bytes. Six earlier completed private notes move verbatim to the
compiler-provenance ledger, with their identities and every other private commit row preserved;
lost-note, duplicate-record and changed-identity controls reject.

Held native tree `0e7f8ca8b85c97b4447ff5d2fc4bd4fb0f5c93ff` precedes completion documentation
and final focused staging. Raw calibration, restoration and reconciliation evidence is under
`target/compiler-record-positions/`. Supplemental audit SHA-256:
`ff272af1f01c64017746825e63511db8f3e148e40c07381fe71540c78e48a600`.
Final focused/book/doctrine checks close this slice. Internal-NUL argument validation is next;
the dated original controller result retains its original meaning until full replay.

### Internal NUL argument validation

`2026-10-19`, leaf `.15.2.1.3.3.3.2.6.3`: the consumer's complete NUL-byte check now
has a control distinguishing it from checks of only the first or last decoded byte.
The original production check is retained exactly.

The permanent control creates a real observer session and compiles a program with the pinned
compiler. It checks successful status and empty diagnostics, executes the produced program,
and independently compares the consumer's full argument vector with the arguments supplied.
A snapshot retains the actual producer record.

`2026-10-19`: the test separately inserts an internal NUL into the crate-name value,
output-directory value and source-path value. Insertion uses a UTF-8 character boundary and
leaves both endpoint bytes nonzero. The compiler name and compilation markers remain intact.
The consumer must refuse with `NUL in argument`, preserve the corrupted record, and accept the
exact original vector after byte-for-byte restoration. Each case starts and ends valid.

These are corrupted-record controls at the consumer boundary. Native operating-system process
arguments cannot carry NUL bytes; the test does not claim the compiler produced such arguments.
The complete byte-validation requirement also protects the reader from damaged or foreign
record data. Leaving required compilation markers intact prevents unrelated shape checks
from masking the endpoint-only validation defect.

The permanent parent is
`trust::compiler::tests::actual_compilation_records_refuse_internal_nul_in_argument_values`.

```sh
cargo test -p xtask trust::compiler::tests::actual_compilation_records_refuse_internal_nul_in_argument_values -- --exact
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L168.contains.109 --only L168.contains.111
```

Actual-source calibration runs the original and each literal endpoint edit separately.
The original succeeds; each endpoint-only controller compiles and produces the intended
finishing failed-test summary. The permanent parent passes directly in 0.43 seconds.

`2026-10-19`: native baseline 54 seconds; both actual endpoint edits finish killed by only
the new parent against all 180 current trust parent names, strict exit 0. There is no timeout,
unjudged result, compiler refusal or equivalence judgment in this selected replay.

`2026-10-19`: all 1104 original/current-copy inputs restore by bytes, modes and independent
Python/system hashes. All 33 direct bindings and 37 public source pins agree. Twelve altered
source/scope/replacement/name/outcome/count/exit/population receipts reject. Removing the new
parent reconstructs the entire previous controller exactly; other production, prior tests,
catalogue and reasons are unchanged. The committed record-position body seals exactly and
all 292 earlier archives retain their bytes.

Held native tree `1881a975636dda2540509433923760d7cb6f0fe2` precedes completion documentation
and final focused staging. Raw calibration and receipts are under `target/compiler-internal-nul/`.
Supplemental audit SHA-256:
`7db90d127e54867b18d3bc202e2c82b2aba6688071be34a9a2d1eb6de4545839`.
Final focused/book/doctrine checks close this slice. Exact compilation emit-marker validation
is next; the final complete controller replay remains separately owned.

### Exact compilation emit-marker validation

`2026-10-19`, leaf `.15.2.1.3.3.3.2.6.4`: the compilation-record shape check now has
a permanent control for the exact `--emit=` prefix. The original production check remains
unchanged.

The control creates a real observer session, compiles a program with the pinned compiler,
requires successful status and empty diagnostics, and executes the resulting program.
It independently compares the complete compilation vector returned by the consumer with
the supplied arguments, then retains the exact producer-record bytes.

`2026-10-19`: each case changes only the emit argument in that real record:

| Changed argument | Boundary held |
| --- | --- |
| `prefix--emit=dep-info,link` | The marker must start the argument |
| `--emit-extra=dep-info,link` | Extra name text before the equals sign is refused |
| `--emit` | The equals sign is required |

The compiler name, crate-name marker, output-directory marker and their values remain intact.
Each case requires the explicit unknown-invocation-shape refusal and unchanged corrupted bytes.
Restoring the original record must restore the exact accepted compilation vector and bytes.
These are consumer corruption controls, not claims that Cargo emitted the changed argument
shapes or that the underlying compiler executed them. The control holds prefix recognition;
it does not claim to replace the compiler's option-value parser or downstream artifact checks.

The permanent parent is
`trust::compiler::tests::actual_compilation_records_require_the_exact_emit_marker`.

```sh
cargo test -p xtask trust::compiler::tests::actual_compilation_records_require_the_exact_emit_marker -- --exact
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L265.dir.85 --only L265.lit.86
```

Actual-source calibration runs the original and each literal edit separately. The original
succeeds. Replacing the prefix test with a substring test accepts the prefixed decoy; shortening
the tested literal to `--emit` accepts the extra-name form. Each variant compiles and finishes
with the intended failed-test summary. The permanent parent passes directly in 0.77 seconds.

`2026-10-19`: native baseline 56 seconds; both actual edits finish killed by only the new
parent against all 181 current trust parent names, strict exit 0. There is no timeout, unjudged
result, compiler refusal or equivalence judgment in this selected replay.

`2026-10-19`: all 1105 original/current-copy inputs restore by bytes, modes and independent
Python/system hashes. All 33 direct bindings and 37 public source pins agree. Twelve altered
source/scope/replacement/name/outcome/count/exit/population receipts reject. Removing the new
parent reconstructs the entire previous controller exactly. Other production, prior tests,
catalogue and reasons are unchanged. The committed internal-NUL body seals exactly, and all
293 earlier archives retain their bytes.

Held native tree `a596c2190204f09b9a72399f0c7f23d8e84f41a6` precedes completion documentation
and final focused staging. Raw calibration and receipts are under `target/compiler-emit-marker/`.
Supplemental audit SHA-256:
`45f7cb2ad33b21ff8772b69212bb3fc1cee1a728eef3e6b5e95f1104465db95f`.
Final focused/book/doctrine checks close this slice. Installed-observer byte verification is
next; final complete controller replay remains separately owned.

### Installed observer byte verification

`2026-10-19`, leaf `.15.2.1.3.3.3.2.6.5`: the installed-image comparison now has a
native corruption control. The original production comparison already refuses changed bytes;
its production prefix is unchanged. This measurement and retained native fixture are macOS-specific.

The control compiles the actual controller with its JSON and embedded observer sources in an
isolated helper. It includes the real base-environment constructor without changing its
selection or forwarding. Controller source comes from the compiled test context, including
an actual native mutation. The pristine source-root hint is not substituted for it.

The tracked C fixture uses the existing public Mach-O interposition boundary. After the real
permission operation succeeds on an installed observer, it changes the first file byte with
an exclusive-or mask of one. The write occurs before the controller closes its writer and
before its readback. The fixture targets only observer files in the helper's own session root.
It does not hook close or change the bytes returned by read. Consequently, both the original
and guard-omitted controllers receive a genuinely changed installed file, even when the latter
omits readback altogether.

Trace records identify loaded-library, successful permission, corruption and readback events
by actual file inode. The parent independently reads the prepared and installed images and
requires exactly the intended changed byte, unchanged remaining bytes and preserved permissions.
This separates a real filesystem effect from a fixture that only reports that it injected one.

| Case | Required behavior |
| --- | --- |
| Baseline, no library | Accept an exact installed image and complete a real pinned query |
| Loaded library, pass mode | Preserve exact image bytes and complete the same query |
| Corruption | Refuse with `installed compiler provenance observer differs`, retain the changed image without creating its records directory, then complete a fresh valid session and pinned-query recovery |

The helper compares observer-query stdout with an independent direct pinned query and requires
the typed consumer to accept the query record without producing compilation units. The fixture
removes its loader variables before compiler or observer children start. All environment changes
remain within the isolated helper process. The relative preload path avoids a checkout-root
delimiter in the loader list.

`2026-10-19`: original/literal calibration retains all six combinations of controller and
baseline/pass/corrupt mode. Both controllers receive the same precise corruption effect.
The original refuses and recovers; the omitted comparison accepts the changed image.
The deliberately unexpected-acceptance branch returns before attempting to execute that image.
Baseline and loaded-pass cases succeed for both. Independent byte, permission, inode-trace,
record-state, stdout and stderr checks agree.

This is controlled filesystem corruption at a component boundary. It does not claim to induce
physical storage failure, establish crash consistency or measure arbitrary close-sensitive
filesystems. Other hosts require their own supported boundary witness. The host-neutral
catalogue remains unchanged.

The permanent parent is
`trust::compiler::tests::actual_installed_observer_corruption_is_refused_before_use_and_recovers`.
Its fixtures are `scripts/fixtures/compiler_install_readback.c` and
`scripts/fixtures/compiler_install_readback_probe.rs`.

```sh
cargo test -p xtask trust::compiler::tests::actual_installed_observer_corruption_is_refused_before_use_and_recovers -- --exact
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L120.cond.18
```

The permanent control passes directly in 2.08 seconds. The native replay confirms the guard-omission regression.

`2026-10-19`: native baseline 54 seconds; the actual guard omission finishes killed by only
the new parent against all 182 current macOS trust parents, strict exit 0. There is no timeout,
unjudged result, compiler refusal or equivalence judgment in this selected replay.

`2026-10-19`: all 1108 original/current-copy inputs restore by bytes, modes and independent
Python/system hashes. All 35 direct bindings and 37 public source pins agree. Twelve altered
source/scope/replacement/name/outcome/count/exit/population receipts reject. The independent
native calibration audit verifies every retained original/literal case and its actual files.
Removing the new parent reconstructs the entire previous controller exactly. Other production,
prior tests, catalogue and reasons are unchanged. The committed emit-marker body seals exactly,
and all 294 earlier archives retain their bytes.

Held native tree `5354a8d5fe8a9cf7e3a9dfffce9d803d39341783` precedes completion documentation
and final focused staging. Raw calibration and receipts are under `target/compiler-install-readback/`.
Supplemental audit SHA-256:
`80af8168e21a568b165d78eb695a49a0e66006b65543355a69db18f983d8cee1`.
Final focused/book/doctrine checks close this slice. Installation's explicit close is assessed
next; the observer's path-only close proof does not transfer to this later readback allocation.

### Installation writer lifetime and readback allocation

`2026-10-19`, leaf `.15.2.1.3.3.3.2.6.6`: installation's explicit `drop(file)`
is observably needed by a supported native resource case. It is a killed lifetime obligation.
The observer's earlier path-only close equivalence does not transfer: installation subsequently
reads the installed image, which needs another file descriptor.

The [File contract](https://doc.rust-lang.org/std/fs/struct.File.html) describes closure at
scope exit, and [the read convenience function](https://doc.rust-lang.org/std/fs/fn.read.html)
opens and reads a file. [The resource-limit API](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getrlimit.2.html)
controls a process's open-file budget, with a soft value adjustable within its hard value.
The online Rust pages are newer than the pinned toolchain; they supply general API context,
not pinned implementation evidence. The actual source variants and queries execute the
[recorded pinned compiler](ledger.md#rust-toolchain).

The control compiles the actual controller, JSON, embedded observer and real environment
constructor in an isolated helper. A macOS native fixture runs after successful installation
permissions, while the writer is still open. It identifies that actual writer descriptor,
temporarily lowers only the helper's soft open-file allowance, and allocates its own
close-on-exec filler descriptors until the kernel returns `EMFILE`. A separate descriptor
census requires the available table to be full. A trace descriptor was reserved before
scarcity, so logging requires no new open operation at this boundary.

The read-only operating-system dependency `/dev/null` supplies real native filler descriptors.
It is needed to exercise the kernel allocation boundary; it stores no project data.
All project files and traces remain on the repository volume. The fixture never closes the
installation writer and does not interpose close.

The helper returns from the actual session attempt, then changes permissions on a prepared
fixture release marker. That test-only callback closes exactly its recorded fillers,
checks that those descriptors are closed, and restores the precise previous soft and hard
resource values before any query or fresh-session recovery. The hard value is never lowered.
The helper's compiler and observer children do not inherit the native fixture loader or
filler descriptors. Other processes' resource values remain untouched.

| Case | Required behavior |
| --- | --- |
| Baseline, no library | Install exact bytes, complete a pinned query and a fresh recovery query |
| Loaded pass mode | Preserve the same image, query and recovery behavior |
| Scarcity | Explicit close frees the writer slot; real readback reuses it, installation succeeds, cleanup restores the budget, and both query checks succeed |

The parent independently checks every installed image against the prepared image, regular-file
state and permissions, precise fixture trace fields, complete resource-table occupancy,
owned-filler cleanup, restored values and exact query stdout. The scarcity trace must order
exhaustion, readback on the original writer's descriptor, release and fresh-session readback.
Both successful sessions must retain their real query records.

`2026-10-19`: final actual-source calibration retains all six controller/mode combinations.
The measured scarcity case has soft value 64, prior soft value 4096, and 59 owned fillers.
The original closes its writer and completes readback on that released descriptor. The
literal omission keeps the writer live, returns the kernel's too-many-open-files error,
and leaves that first session without a records directory. Both variants preserve every
installed byte, restore the prior resource values, and complete valid fresh-session recovery.
Baseline and loaded-pass cases succeed for both variants.

This is a controlled per-process descriptor-allocation witness on macOS. It establishes a
concrete observable difference, not universal timing behavior or an equivalence judgment.
Production already has the required explicit close. The host-neutral catalogue and observer
close reason remain unchanged.

The permanent parent is
`trust::compiler::tests::actual_installation_closes_its_writer_before_readback_allocation`.
Its fixtures are `scripts/fixtures/compiler_install_close.c` and
`scripts/fixtures/compiler_install_close_probe.rs`.

```sh
cargo test -p xtask trust::compiler::tests::actual_installation_closes_its_writer_before_readback_allocation -- --exact
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L119.stmt.15
```

The permanent control passes directly in 2.52 seconds. The native replay confirms the explicit-close regression.

`2026-10-19`: native baseline 54 seconds; the actual drop omission finishes killed by only
the new parent against all 183 current macOS trust parents, strict exit 0. There is no timeout,
unjudged result, compiler refusal or equivalence judgment in this selected replay.

`2026-10-19`: all 1111 original/current-copy inputs restore by bytes, modes and independent
Python/system hashes. All 37 direct bindings and 37 public source pins agree. Twelve altered
source/scope/replacement/name/outcome/count/exit/population receipts reject. The independent
native calibration audit checks each retained controller/mode case against its actual bytes,
inode trace, descriptor fields, records and query output. Removing the new parent reconstructs
the entire previous controller exactly. Other production, prior tests, catalogue and reasons
are unchanged. The committed installed-byte body seals exactly, and all 295 earlier archives
retain their bytes.

Held native tree `415072dd0e305c2b54f46f8e0b1f30ca63d51ee5` precedes completion documentation
and final focused staging. Raw calibration, API observations and receipts are under
`target/compiler-install-close/`. Supplemental audit SHA-256:
`b193d51e65b3cf922a3f0d390bde46332916010d0b7b3edf2fa9ea872261d9f0`.
Eight completed private notes also move verbatim to the compiler-provenance ledger; original
identities and every other private commit row are preserved, with lost-note, duplicate-record
and changed-identity controls.
`2026-10-19`: four staging/check requests and app inspection failed automatic review
before execution, reporting capacity or a deadline rather than an unsafe-action determination.
The director confirmed that **Ask for approval** was selected, while requests still selected
automatic review. After the prior run ended, the active execution policy changed to manual
review and explicit staging of the intended files succeeded. The delayed uptake has no
established root cause; the observed recovery is not a general guarantee. Final focused
verification and staged doctrine gates precede commit. Record-presence state follows closure
and the director restoring the default permission, before the full controller replay.

`2026-10-19`: the director requires an immediate, prominent approval-blocker alert carrying
the project name, blocked operation, exact cause, verified user steps and the next action.
The canonical agent instruction is in `TOOLBOX.md`. For this automatic-review failure, the
documented manual-review setting is **Ask for approval** in the permissions control; resume
so the command can receive human review. This changes the reviewer while retaining the
workspace sandbox. See [the official permission controls](https://learn.chatgpt.com/docs/permission-modes).
Official [troubleshooting](https://learn.chatgpt.com/docs/reference/troubleshooting) also
recommends a new focused chat for stuck states; if needed, use this same working directory
and read the saved resume pointer and dirty owning leaf before any change. Do not create a
worktree from `HEAD` and lose the uncommitted slice.


`2026-10-19`: the director sets **Approve for me** as the default. Whenever a permission
change is needed, issue the prominent `ARCHOGEN` alert and block until the director reacts.
After finishing the work that needs the temporary permission, pause again so the director
can restore **Approve for me**; await that reaction before continuing the next-task loop. Record both
waiting states durably. This instruction governs the current approval recovery too.

### Record-presence state assessment

`2026-10-19`, leaf `.15.2.1.3.3.3.2.6.7`: the actual old `L271.int.92` changes
`count += 1` to `count += 2`. The consumer uses this quantity only for its final zero test.
The implementation now stores `saw_record`: initially false, set true only after a record
passes every validation, then checked after the complete directory traversal. Empty directories
still refuse; valid query records establish presence without becoming inventory units; real
compilation records retain their complete argument vectors. A valid earlier record does not
skip checks on later entries. This replaces unnecessary numeric state; the old edit is removed
from the current source, without declaring it equivalent or claiming a new platform restriction.

The only controller changes are the initialization, update and final presence condition.
Existing actual missing/query/compilation/malformed and later-record controls remain unchanged.
Observer-close and absolute-working-directory proof guards both rejected the changed controller
before their stored hashes were updated. Source comparison confirms that session target/cwd
construction, environment propagation, synchronous successful Cargo-before-reader ordering and
observer post-synchronization publication operations are unchanged. Actual protocol and cwd
counterexample executions must revalidate those premises before this leaf can close.

The current discovery includes false/true initialization, false/omitted update and both constant
or inverted final-condition edits. Selected native controls and the final complete controller
replay judge the current source; historical edit identities and outcomes remain historical.
`xtask/src/trust_compiler.rs`, `xtask/mutation_sweep_tests.py` and the
[compiler provenance contract](../../specs/trust/compiler-provenance-contract.md) own these
interfaces and checks. Raw assessment evidence is under `target/compiler-record-presence/`.

`2026-10-19`: automatic review accepted tracking-file staging after default permissions were
restored, but refused the proof-control execution request because its reviewer model was at
capacity. The command never executed. The director responded to the prominent alert by changing
to **Ask for approval**; the retry still selected automatic review and failed capacity before
execution. The active run still declares automatic review. End the run, then resume in a fresh
turn with manual review retained, as in the preceding recovery; this is an observed recovery
procedure, not an established explanation of the setting mismatch. Native/proof/focused
results were pending at that stopping point. A fresh turn picked up manual approval and
launched the actual controls. After commit and the native handoff census, pause again for
the director to restore **Approve for me** and react before continuing.

#### Presence contract examples

The state records whether any complete valid provenance exists, independently of whether
those records contribute compilation units:

| Input in the private record directory | Required consumer result |
| --- | --- |
| Empty directory | Explicit missing-record refusal |
| Valid pinned compiler queries | Success with no compilation units |
| Valid observed compilations | Success retaining their complete argument vectors |
| Valid records together with a malformed, partial or foreign record | Refusal; earlier valid records cannot hide the bad entry |

The existing actual query-refusal, compilation and every-record-position parents exercise
these boundaries. They were preserved verbatim. Neither the scope of supported records nor
the order in which validation visits directory entries changes.

#### Revalidation and actual native results

`2026-10-19`: all 47 controls in `xtask/mutation_sweep_tests.py` pass in 138.525 s. This includes
the actual pinned observer original/omitted-close protocol and its deliberately different
syscall timing, plus compiled absolute-cwd/target-path counterexamples and all existing
changed-premise/wrong-oracle falsifiers. Both stale controller guards first failed on purpose.
Only their controller prefix digests change after the complete source/caller comparison:

| Guard | Current controller prefix SHA-256 |
| --- | --- |
| Observer-close proof, excluding the separator newline | `1c5e2739a3ebcf1d1b1d5c5a34bb1e1fde189e7b7e54d1faa023e0d5f9d6d973` |
| Absolute-cwd proof, including that newline | `634034fc7f85d67cb843251b15b2065ba229aee6692c6b68987c717383915b70` |

`2026-10-19`: the actual CLI and discovery agree on all 124 current edits, with production lines 31 through 307 and whole controller SHA-256
`28565b71d0a9471a855335b4037685a8a0a11449a8fe955a10c031e502e6d766`.
`2026-10-19`: the selected native run uses all 183 current macOS trust parents; baseline passes in 56 s.
Every presence edit has a finishing kill from
`actual_query_records_reject_missing_stale_foreign_and_malformed_inputs`:

| Actual current ID | Change | Required boundary that rejects it |
| --- | --- | --- |
| `L199.bool.33` | Initialize presence true | Empty record directory must refuse |
| `L271.bool.93` | Set presence false after a valid record | A valid query must succeed |
| `L271.stmt.94` | Omit setting presence after a valid record | A valid query must succeed |
| `L273.cond.95` | Always take missing-record refusal | A valid query must succeed |
| `L273.cond.96` | Never take missing-record refusal | Empty record directory must refuse |
| `L273.not.97` | Invert final presence condition | A valid query must succeed |

`2026-10-19`: the selected result is six finishing kills, strict exit 0, with no timeout, unjudged, unviable or equivalent result. Complete current controller replay remains the next leaf; these selected
results do not establish the outcome of the other current edits or any global adequacy rate.
`2026-10-19`: all 1,112 original/current-copy regular inputs restore by bytes/modes and Python/system hashes; Gitlinks remain metadata only. All 37 direct bindings and 37 public producer pins bind, and 12 false receipt arms reject altered scope, missing/duplicate results, replacement, killing name,
totals, exit, source file/hash, outcome, invented equivalence or reduced population. Every
reported killing name belongs to the complete current parent list, which the restored copy
also reproduces. The proof controls run outside that semantic parent population.

Reconstructing the prior controller by reversing the three state edits recovers its complete
committed bytes, including every prior test. Reversing only the two digest replacements
recovers the complete prior Python tool controls. Other callers, observer, environment,
artifact code, catalogue and reasons remain exact.
`2026-10-19`: the committed installation-close leaf seals exactly and all 296 older archives are unchanged. Held native tree:
`48c97b148dea511d9cbb3c39d6d23b32d1c75eb0`; supplemental raw-audit SHA-256:
`2a456733f50dc3fa68800e4a6edb65f7900e1615eaa7f0078825260519d66ae0`.
Final focused/book/staged doctrine gates close this slice before permission handback.

```sh
python3 -I -B xtask/mutation_sweep_tests.py "$PWD"
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L199.bool.33 --only L271.bool.93 --only L271.stmt.94 \
  --only L273.cond.95 --only L273.cond.96 --only L273.not.97
```

`2026-10-19`: the director asked why apparently safe commands still need explicit approval.
Approval is also an execution-boundary decision, rather than a declaration that the requested
operation is dangerous. This session requested broader execution for protected Git writes and
native verification. Under the active request policy, commands containing environment setup,
redirection or other shell wrappers are not automatically covered by saved command-prefix
approvals. Grouping routine work into such requests can therefore add approval friction.
The engineering practice is to use ordinary workspace sandbox execution where possible,
separate routine steps from operations needing an exception, and make each required approval
request narrow and explicit. This does not override a denial or authorize bypassing a review.
The [official permission model](https://learn.chatgpt.com/docs/permission-modes) distinguishes
the sandbox boundary from the reviewer: **Approve for me** sends eligible requests to the
automatic reviewer; **Ask for approval** sends them to the user. The observed reviewer capacity
failure prompted the temporary switch. No permission configuration is changed by this explanation.

### Complete current controller replay

`2026-10-19`, leaf `.15.2.1.3.3.3.2.6.8`: the complete current controller population finishes with 113 named kills and 11 compiler refusals, strict exit 0. There are no survivors, timeouts, unjudged results or equivalence classifications.
This closes the controller's current source criterion after its owned survivor resolutions.
It does not project adequacy to the artifact binder, caller or other judgment sources.

The measured controller `xtask/src/trust_compiler.rs` is SHA-256
`28565b71d0a9471a855335b4037685a8a0a11449a8fe955a10c031e502e6d766`.
The production range starts at `fresh` and ends before the standalone test module, source
lines 31 through 307. Fresh maintained discovery agrees exactly with the complete actual
CLI listing, source spans, operators and replacements. There is no selected-ID filter or
controller equivalence file in this measurement.

`2026-10-19`: all 124 current edits run against the complete current macOS cohort of 183 semantic trust parents; original and restored-copy lists agree exactly. The unmodified copied baseline passes in 56 seconds.
The directory-retry loop's non-progress edits finish through bounded parent failures with
actual libtest names. These are named kills, not outer native timeout classifications.

```sh
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs --package xtask --list
bash scripts/mutation_sweep.sh xtask/src/trust_compiler.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail
```

Reproduction requires these matching source, caller and producer identities. The same-volume
native profile derives the writable repository root at runtime; its SHA-256 remains
`2b3fbe199bddb4ba84c453a23334c7025308b06127ce3cc96f74b2a17680eedd`.
It permits writes only in this repository and to the null device. The held native tree is
`239f541d37dd06b49ee9bbfa14bb3e9fbb15dfb2`; the raw execution, plans, receipts, restoration
proofs and independent audit are under `target/compiler-controller-final/`.

#### Complete operator census

The terminal operator summary and independent audit agree on every attempt and refusal.
The viable edits in each row finish as named kills; compiler refusals remain separate.

| Operator | Attempted | Killed | Compiler refusals |
| --- | --- | --- | --- |
| `bool` | 4 | 4 | 0 |
| `chain` | 13 | 11 | 2 |
| `clogic` | 2 | 2 | 0 |
| `cmp` | 2 | 2 | 0 |
| `cond` | 22 | 22 | 0 |
| `contains` | 8 | 2 | 6 |
| `dir` | 1 | 1 | 0 |
| `eq` | 17 | 17 | 0 |
| `guard` | 15 | 15 | 0 |
| `int` | 2 | 2 | 0 |
| `iter` | 6 | 6 | 0 |
| `lit` | 2 | 2 | 0 |
| `logic` | 15 | 15 | 0 |
| `not` | 6 | 6 | 0 |
| `stmt` | 9 | 6 | 3 |

#### Every current controller outcome

This table records the complete current population. For a kill, the name count includes every
finishing name in its receipt; the label identifies an actual control, not the only possible
killer. Compiler refusals carry their own diagnostics below and do not count as semantic coverage.
Exact replacements regenerate from the source-bound listing; the retained receipt has all names.

| Actual ID | Method | Outcome | Finishing names | Control |
| --- | --- | --- | --- | --- |
| `L32.stmt.0` | `fresh` | killed | 125 | `F1` |
| `L33.iter.1` | `fresh` | killed | 50 | `F2` |
| `L33.iter.2` | `fresh` | killed | 125 | `F1` |
| `L33.iter.3` | `fresh` | killed | 50 | `F2` |
| `L37.eq.4` | `fresh` | killed | 50 | `F2` |
| `L55.stmt.5` | `Compiler::prepare` | killed | 125 | `F1` |
| `L72.cond.6` | `Compiler::prepare` | killed | 125 | `F1` |
| `L72.cond.7` | `Compiler::prepare` | killed | 1 | `F1` |
| `L72.not.8` | `Compiler::prepare` | killed | 125 | `F1` |
| `L79.stmt.9` | `Compiler::prepare` | unviable | — | compiler refusal |
| `L80.cond.10` | `Compiler::prepare` | killed | 125 | `F1` |
| `L80.cond.11` | `Compiler::prepare` | killed | 1 | `F1` |
| `L95.eq.12` | `Compiler::session` | killed | 5 | `F2` |
| `L106.bool.13` | `Compiler::session` | killed | 125 | `F1` |
| `L107.bool.14` | `Compiler::session` | killed | 125 | `F1` |
| `L119.stmt.15` | `Compiler::session` | killed | 1 | `F3` |
| `L120.eq.16` | `Compiler::session` | killed | 125 | `F1` |
| `L120.cond.17` | `Compiler::session` | killed | 125 | `F1` |
| `L120.cond.18` | `Compiler::session` | killed | 2 | `F2` |
| `L124.stmt.19` | `Compiler::session` | killed | 125 | `F1` |
| `L126.stmt.20` | `Compiler::session` | unviable | — | compiler refusal |
| `L147.not.21` | `decode/decode_hex` | killed | 124 | `F1` |
| `L148.logic.22` | `decode/decode_hex` | killed | 2 | `F4` |
| `L148.not.23` | `decode/decode_hex` | killed | 124 | `F1` |
| `L150.logic.24` | `decode/decode_hex` | killed | 124 | `F1` |
| `L159.cmp.25` | `decode/decode_hex` | killed | 124 | `F1` |
| `L159.cond.26` | `decode/decode_hex` | killed | 124 | `F1` |
| `L159.cond.27` | `decode/decode_hex` | killed | 124 | `F1` |
| `L162.int.28` | `decode/decode_hex` | killed | 124 | `F1` |
| `L162.int.29` | `decode/decode_hex` | killed | 124 | `F1` |
| `L168.cond.30` | `decode/decode_hex` | killed | 124 | `F1` |
| `L168.cond.31` | `decode/decode_hex` | killed | 2 | `F5` |
| `L192.stmt.32` | `Session::environment` | unviable | — | compiler refusal |
| `L199.bool.33` | `Session::invocations` | killed | 1 | `F4` |
| `L201.iter.34` | `Session::invocations` | killed | 109 | `F6` |
| `L201.iter.35` | `Session::invocations` | killed | 111 | `F6` |
| `L201.iter.36` | `Session::invocations` | killed | 104 | `F6` |
| `L201.guard.37` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.38` | `Session::invocations` | killed | 2 | `F6` |
| `L201.guard.39` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.40` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.41` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.42` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.43` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.44` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.45` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.46` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.47` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.48` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.49` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.50` | `Session::invocations` | killed | 1 | `F6` |
| `L201.guard.51` | `Session::invocations` | killed | 1 | `F6` |
| `L205.eq.52` | `Session::invocations` | killed | 124 | `F1` |
| `L205.logic.53` | `Session::invocations` | killed | 4 | `F6` |
| `L205.logic.54` | `Session::invocations` | killed | 3 | `F6` |
| `L205.logic.55` | `Session::invocations` | killed | 3 | `F4` |
| `L205.cond.56` | `Session::invocations` | killed | 124 | `F1` |
| `L205.cond.57` | `Session::invocations` | killed | 4 | `F6` |
| `L205.not.58` | `Session::invocations` | killed | 124 | `F1` |
| `L216.eq.59` | `Session::invocations` | killed | 124 | `F1` |
| `L216.eq.60` | `Session::invocations` | killed | 124 | `F1` |
| `L216.logic.61` | `Session::invocations` | killed | 3 | `F6` |
| `L216.logic.62` | `Session::invocations` | killed | 3 | `F6` |
| `L216.logic.63` | `Session::invocations` | killed | 3 | `F6` |
| `L216.cond.64` | `Session::invocations` | killed | 124 | `F1` |
| `L216.cond.65` | `Session::invocations` | killed | 3 | `F6` |
| `L222.eq.66` | `Session::invocations` | killed | 124 | `F1` |
| `L223.eq.67` | `Session::invocations` | killed | 124 | `F1` |
| `L223.logic.68` | `Session::invocations` | killed | 2 | `F4` |
| `L224.eq.69` | `Session::invocations` | killed | 124 | `F1` |
| `L224.logic.70` | `Session::invocations` | killed | 2 | `F4` |
| `L225.eq.71` | `Session::invocations` | killed | 124 | `F1` |
| `L225.logic.72` | `Session::invocations` | killed | 2 | `F4` |
| `L233.cmp.73` | `Session::invocations` | killed | 1 | `F7` |
| `L237.eq.74` | `Session::invocations` | killed | 124 | `F1` |
| `L237.cond.75` | `Session::invocations` | killed | 124 | `F1` |
| `L237.cond.76` | `Session::invocations` | killed | 2 | `F6` |
| `L242.eq.77` | `Session::invocations` | killed | 124 | `F1` |
| `L243.eq.78` | `Session::invocations` | killed | 124 | `F1` |
| `L243.logic.79` | `Session::invocations` | killed | 3 | `F4` |
| `L259.eq.80` | `Session::invocations` | killed | 124 | `F1` |
| `L259.cond.81` | `Session::invocations` | killed | 124 | `F1` |
| `L259.cond.82` | `Session::invocations` | killed | 3 | `F6` |
| `L262.eq.83` | `Session::invocations` | killed | 1 | `F7` |
| `L264.logic.84` | `Session::invocations` | killed | 2 | `F8` |
| `L265.logic.85` | `Session::invocations` | killed | 2 | `F8` |
| `L265.dir.86` | `Session::invocations` | killed | 1 | `F8` |
| `L265.lit.87` | `Session::invocations` | killed | 1 | `F8` |
| `L265.lit.88` | `Session::invocations` | killed | 116 | `F6` |
| `L267.stmt.89` | `Session::invocations` | killed | 116 | `F6` |
| `L268.cond.90` | `Session::invocations` | killed | 122 | `F1` |
| `L268.cond.91` | `Session::invocations` | killed | 4 | `F6` |
| `L268.not.92` | `Session::invocations` | killed | 123 | `F1` |
| `L271.bool.93` | `Session::invocations` | killed | 124 | `F1` |
| `L271.stmt.94` | `Session::invocations` | killed | 124 | `F1` |
| `L273.cond.95` | `Session::invocations` | killed | 124 | `F1` |
| `L273.cond.96` | `Session::invocations` | killed | 1 | `F4` |
| `L273.not.97` | `Session::invocations` | killed | 124 | `F1` |
| `L281.eq.98` | `query` | killed | 123 | `F1` |
| `L282.logic.99` | `query` | killed | 122 | `F1` |
| `L283.eq.100` | `query` | killed | 114 | `F6` |
| `L68.chain.101` | `Compiler::prepare` | killed | 1 | `F1` |
| `L68.chain.102` | `Compiler::prepare` | killed | 125 | `F1` |
| `L94.chain.103` | `Compiler::session` | killed | 125 | `F1` |
| `L94.chain.104` | `Compiler::session` | killed | 125 | `F1` |
| `L150.contains.105` | `decode/decode_hex` | unviable | — | compiler refusal |
| `L150.contains.106` | `decode/decode_hex` | unviable | — | compiler refusal |
| `L150.contains.107` | `decode/decode_hex` | unviable | — | compiler refusal |
| `L150.contains.108` | `decode/decode_hex` | unviable | — | compiler refusal |
| `L168.contains.109` | `decode/decode_hex` | killed | 1 | `F5` |
| `L168.contains.110` | `decode/decode_hex` | unviable | — | compiler refusal |
| `L168.contains.111` | `decode/decode_hex` | killed | 1 | `F5` |
| `L168.contains.112` | `decode/decode_hex` | unviable | — | compiler refusal |
| `L233.chain.113` | `Session::invocations` | unviable | — | compiler refusal |
| `L233.chain.114` | `Session::invocations` | unviable | — | compiler refusal |
| `L233.chain.115` | `Session::invocations` | killed | 1 | `F7` |
| `L251.chain.116` | `Session::invocations` | killed | 124 | `F1` |
| `L251.chain.117` | `Session::invocations` | killed | 124 | `F1` |
| `L262.chain.118` | `Session::invocations` | killed | 116 | `F6` |
| `L262.chain.119` | `Session::invocations` | killed | 116 | `F6` |
| `L265.chain.120` | `Session::invocations` | killed | 116 | `F6` |
| `L265.chain.121` | `Session::invocations` | killed | 116 | `F6` |
| `L150.clogic.122` | `decode/decode_hex` | killed | 124 | `F1` |
| `L150.clogic.123` | `decode/decode_hex` | killed | 124 | `F1` |

Control labels name actual finishing libtest parents:

| Label | Exact parent name |
| --- | --- |
| `F1` | `trust::compiler::tests::actual_preparation_holds_status_output_and_selected_environment` |
| `F2` | `trust::compiler::tests::actual_installed_observer_corruption_is_refused_before_use_and_recovers` |
| `F3` | `trust::compiler::tests::actual_installation_closes_its_writer_before_readback_allocation` |
| `F4` | `trust::compiler::tests::actual_query_records_reject_missing_stale_foreign_and_malformed_inputs` |
| `F5` | `trust::compiler::tests::actual_compilation_records_refuse_internal_nul_in_argument_values` |
| `F6` | `trust::compiler::tests::actual_record_validation_applies_to_every_real_record` |
| `F7` | `trust::compiler::tests::actual_records_refuse_remaining_field_range_byte_and_shape_boundaries` |
| `F8` | `trust::compiler::tests::actual_compilation_records_require_the_exact_emit_marker` |

#### Actual compiler refusals

These edits do not compile with the pinned producer. Their exact source-edit signatures and
first compiler diagnostics match the earlier complete controller measurement. They remain
compiler refusals; none is promoted to a kill or equivalence. Invented and removed refusal
population controls are both rejected by the independent comparison.

| Actual ID | Diagnostic |
| --- | --- |
| `L79.stmt.9` | `` error: expected `;`, found `Ok` `` |
| `L126.stmt.20` | `` error: expected expression, found `let` statement `` |
| `L192.stmt.32` | `` error: mismatched closing delimiter: `}` `` |
| `L150.contains.105` | `` error[E0599]: no method named `first` found for struct `std::ops::RangeInclusive<Idx>` in the current scope `` |
| `L150.contains.106` | `` error[E0599]: no method named `first` found for struct `std::ops::RangeInclusive<Idx>` in the current scope `` |
| `L150.contains.107` | `error[E0308]: mismatched types` |
| `L150.contains.108` | `` error[E0599]: no method named `copied` found for enum `Option<u8>` in the current scope `` |
| `L168.contains.110` | `error[E0308]: mismatched types` |
| `L168.contains.112` | `error[E0308]: mismatched types` |
| `L233.chain.113` | `error[E0061]: this method takes 0 arguments but 1 argument was supplied` |
| `L233.chain.114` | `` error[E0599]: no method named `rev` found for enum `Option<u32>` in the current scope `` |

The statement omissions leave an incomplete assignment or an unmatched delimiter. The range
membership rewrites request sequence methods or incompatible option types on a range receiver.
The copied byte endpoints compare a value option with a reference option. The PID rewrites
request iterator-style operations from an option. None reaches a semantic test judgment.

#### Resolution of the earlier surviving edits

The mapping uses exact span/operator/original/replacement signatures, not IDs alone. The
removed quantity update has no current signature after the presence-state repair and receives
no equivalence classification. Every retained former survivor has its own measured current
disposition here.

| Earlier ID | Current ID | Current disposition |
| --- | --- | --- |
| `L72.cond.7` | `L72.cond.7` | killed |
| `L80.cond.11` | `L80.cond.11` | killed |
| `L119.stmt.15` | `L119.stmt.15` | killed |
| `L120.cond.18` | `L120.cond.18` | killed |
| `L201.guard.36` | `L201.guard.37` | killed |
| `L201.guard.38` | `L201.guard.39` | killed |
| `L201.guard.39` | `L201.guard.40` | killed |
| `L201.guard.40` | `L201.guard.41` | killed |
| `L201.guard.41` | `L201.guard.42` | killed |
| `L201.guard.42` | `L201.guard.43` | killed |
| `L201.guard.43` | `L201.guard.44` | killed |
| `L201.guard.44` | `L201.guard.45` | killed |
| `L201.guard.45` | `L201.guard.46` | killed |
| `L201.guard.46` | `L201.guard.47` | killed |
| `L201.guard.47` | `L201.guard.48` | killed |
| `L201.guard.48` | `L201.guard.49` | killed |
| `L201.guard.49` | `L201.guard.50` | killed |
| `L201.guard.50` | `L201.guard.51` | killed |
| `L265.dir.85` | `L265.dir.86` | killed |
| `L265.lit.86` | `L265.lit.87` | killed |
| `L271.int.92` | — | removed numeric state; no equivalence claim |
| `L68.chain.101` | `L68.chain.101` | killed |
| `L168.contains.109` | `L168.contains.109` | killed |
| `L168.contains.111` | `L168.contains.111` | killed |

#### Restoration and independent audit

`2026-10-19`: all 1,113 indexed regular inputs restore byte-for-byte and with their original modes in the original tree and current copy. Python and independent system SHA-256 agree; the sole Gitlink is compared as metadata only.
No submodule implementation is inspected. The copied parent list and all direct source,
caller, proof, fixture and published-producer bindings are checked again after the run.

`2026-10-19`: all 37 direct bindings and 37 pinned public producer sources remain exact. The independent auditor rejects 12 altered transcript/selection receipts and both invented-refusal and missing-refusal populations.
The controls falsify changed source/scope/replacement/population, missing or duplicate rows,
invented names or equivalence, outcome/summary/exit disagreement and altered refusal membership.
Every retained earlier survivor maps by its exact source-edit signature to a measured current
kill. The old numeric quantity update is absent after the presence-state repair, with no
equivalence asserted for the old edit.

`2026-10-19`: the committed record-presence leaf is sealed by exact reconstruction, and all 297 older task-history files remain unchanged. Production source, prior tests and proof controls, catalogues and per-file reasons remain committed bytes.
The audit script is SHA-256
`06f7dcd531e4ead01cb205e797f84fc4c1bb6a3de798a5e0a704971ffd1e6e61`.
Its complete result and separate refusal assessment both pass.

`2026-10-19`: `make integration` passes all 12 steps with no failure, unavailable step, missing implementation or quarantine. Workspace tests take 149.37 seconds and rejecting gate self-tests 387.98 seconds.
The passing integration stamp names the same held tree as the native replay. Formatting,
strict lint, doctrines, book, bare-metal/browser builds, pinned premises and target execution
all pass. Final documentation edits receive the staged doctrine and book checks before commit.
F30 and other source populations remain open under their own owners.

The next native source population is the artifact binder, followed by the current caller and
other judgment sources. Their complete source/premise revalidation, restoration, warranted
broader gates and independent review remain owned before F30 or any global adequacy claim.

### Historical trust helper and controller commit records

`2026-10-19`: these verbatim completed task-table notes retain their original epoch and
meaning. Private rows keep every leaf and work-unit identity and route here. This historical
ledger does not project earlier measurements to the current source or replace the current frontier.

| Leaf | Work unit | Original record |
| --- | --- | --- |
| `M3.6.9.6.3.2.2.9.4` | `ARCHOGEN-M3-0680 (leaf M3.6.9.6.3.2.2.9.4)` | **configuration checkpoint integration passed**: after committed key/API/JSON/effects repairs, complete local tier 12 passed/zero failures or absences, exact committed-tree stamp, captured inputs restored with auxiliary durability limitation explicit; JSON repair container closes, book/live frontier advances to maintained census/current native replay; no current full mutation or runner-baseline claim |
| `M3.6.9.6.3.2.2.9.7.1` | `ARCHOGEN-M3-0681 (leaf M3.6.9.6.3.2.2.9.7.1)` | **restoration producer is maintained**: indexed working bytes/modes and gitlink metadata snapshots/checks, independent system hashes, strict on-volume/boundary/schema/receipt controls; actual byte RED/restore and guard/truncation reversals fail correctly, restored 23 self-tests/native contracts/effects/planner pass; book/tool/live sync and version-register scope qualification, current five native replays next |
| `M3.6.9.6.3.2.2.9.7.2` | `ARCHOGEN-M3-0682 (leaf M3.6.9.6.3.2.2.9.7.2)` | **current boundary repairs replayed with durable census**: committed producer/catalogue/code, all selected native entries yield strict named finishing kills and all 1003 captured inputs restore after each; historical auxiliary limits retained, environment/JSON/durability group closes, book/live state advances to split_command assessment; complete current mutation adequacy and runner compatibility remain open |
| `M3.6.9.6.3.2.2.10.1` | `ARCHOGEN-M3-0683 (leaf M3.6.9.6.3.2.2.10.1)` | **bounded helper measurement and actual Cargo quote defect**: all 29 current edits measured, 13 named kills/16 survivors/strict 1, exact initial bindings and complete original/copy restoration; ordinary/space public Cargo controls versus apostrophe path failure, untracked probe limitation explicit and permanent repair owned next; book/live sync, no survivor closure or full adequacy claim |
| `M3.6.9.6.3.2.2.10.2` | `ARCHOGEN-M3-0684 (leaf M3.6.9.6.3.2.2.10.2)` | **Cargo apostrophe paths preserve inventory**: actual public inventory RED/green over shared dependencies and six output-path cases; outside escapes decoded, quoted backslashes retained, exact old-helper native named kill and maintained 1003-input restoration; old bodies/catalogue prefix exact, format failure corrected and final focused required; canonical/book/pinned-producer/live sync, survivor assessment next |
| `M3.6.9.6.3.2.2.10.3.1` | `ARCHOGEN-M3-0685 (leaf M3.6.9.6.3.2.2.10.3.1)` | **post-repair helper measured**: all 24 current edits, 13 named kills/11 survivors/strict 1, five prior survivor edits now killed by the quote control, exact source-region/edit bindings; all 1003 originals and copied source restored, auxiliary producer limits explicit; final-token coverage gap owned next, book/live sync, no equivalence or full adequacy claim |
| `M3.6.9.6.3.2.2.10.3.2` | `ARCHOGEN-M3-0686 (leaf M3.6.9.6.3.2.2.10.3.2)` | **retained dependency argument controlled independently**: real binary/library fixture expects rlib/rmeta arguments, exact deleted-push reversal and both actual final-token edits give named kills/strict 0, all 1003 originals/copied source restored; production/prior bodies/catalogue prefix preserved, focused passes and final stamp required; book/pinned-source/live sync, remaining domain judgments next |
| `M3.6.9.6.3.2.2.10.3.3.1` | `ARCHOGEN-M3-0688 (leaf M3.6.9.6.3.2.2.10.3.3.1)` | **producer boundary investigated**: pinned formatter distinguishes raw executable from escaped arguments; actual spaced toolchain builds succeed but public inventory collection fails with own-unit guards, ordinary-home control passes; empty check-cfg emitted only in measured failing build, exploratory producer limits explicit; production/tests/catalogue/equivalents unchanged, high-priority repair owned next, no survivor closure |
| `M3.6.9.6.3.2.2.10.3.3.2` | `ARCHOGEN-M3-0689 (leaf M3.6.9.6.3.2.2.10.3.3.2)` | **compiler selector fixes collection**: same pinned rustc command makes Cargo print a plain program name; permanent isolated public RED/green requires exact units/dependencies across five compiler-home controls; exact native reversal and actual statement deletion yield named kills/strict 0, all 1003 captured inputs and copied source restore, prior bodies/catalogue prefix exact; canonical/book/ledger/live sync, exact staged focused required, remaining helper judgments next |
| `M3.6.9.6.3.2.2.10.3.3.3.1` | `ARCHOGEN-M3-0690 (leaf M3.6.9.6.3.2.2.10.3.3.3.1)` | **real empty operands held**: successful pinned Cargo nonempty/empty lint controls preserve middle/final boundaries; four actual surviving loss edits now give named finishing kills/strict 0, exact catalogue reversal also killed; all 1009 captured inputs restore after each run, copied source matches, prior source bodies/catalogue prefix exact; test-only, book/ledger/live sync and published history rollover, final staged focused required; initial obligations and delimiter judgments remain separately owned |
| `M3.6.9.6.3.2.2.10.3.3.3.2.1` | `ARCHOGEN-M3-0691 (leaf M3.6.9.6.3.2.2.10.3.3.3.2.1)` | **multiline framing defects owned**: real Cargo succeeds with line-break output paths; copied public ordinary/LF control pinpoints current compile slice panic; diagnostic framing intervention exposes separate empty compiler-read file objects, qualified apart from unchanged production; originals unchanged and copy restored, immediate dependency-first/command-second repair children, canonical/book/ledger/live lockstep; no production or equivalent change |
| `M3.6.9.6.3.2.2.10.3.3.3.2.2` | `ARCHOGEN-M3-0692 (leaf M3.6.9.6.3.2.2.10.3.3.3.2.2)` | **dependency files retain complete paths**: match known first target and whole scratch prefixes before delimiters, real relative/absolute producer controls and malformed-target refusals pass; exact prior-reader native reversal yields named finishing kill, all 1010 captured inputs restore, older bodies outside owned compile region preserved; copied diagnostic owns escaped environment normalization next before public command framing; canonical/book/ledger/live sync and final staged focused required |
| `M3.6.9.6.3.2.2.10.3.3.3.2.5.1` | `ARCHOGEN-M3-0693 (leaf M3.6.9.6.3.2.2.10.3.3.3.2.5.1)` | **environment producer domain pinned**: actual encoded metadata/runtime controls and pinned rustc source bind LF/CR/backslash encoding; source-marker control and output-colon refusal distinguish framing hypotheses; isolated native getenv oracle root-causes equals-name host surprise without a universal or project-defect claim; qualify exploratory producers, preserve tracked code, own encoded-prefix repair next; canonical/book/ledger/live sync |
| `M3.6.9.6.3.2.2.10.3.3.3.2.5.2` | `ARCHOGEN-M3-0694 (leaf M3.6.9.6.3.2.2.10.3.3.3.2.5.2)` | **encoded environment prefixes normalize**: permanent actual Cargo controls require exact package-relative manifest path, encoded description/unset records across six source roots; distinct canonical control passes, exact literal-prefix reversal gives named finishing kill/strict 0 and all 1010 captured inputs restore; all prior bodies outside new helper/call/tests and catalogue prefix exact; book/canonical/ledger/live sync, final exact staged focused required; public complete-command framing next |
| `M3.6.9.6.3.2.2.10.3.3.3.2.3` | `ARCHOGEN-M3-0695 (leaf M3.6.9.6.3.2.2.10.3.3.3.2.3)` | **complete Cargo records preserve inventory**: public newline panic and raw status-path fake invocation independently reproduced/root-caused; quote-aware full records plus known raw status prefixes repaired, malformed framing refuses; permanent exact unit/file-hash/env/dependency/configuration controls pass, both native reversals give named finishing kills/strict 0, all 1010 inputs restore and prior bodies/catalogue prefix exact; canonical/book/ledger/live sync, exact staged focused required; delimiter proofs next, final adequacy open |
| `M3.6.9.6.3.2.2.10.3.3.3.2.6` | `ARCHOGEN-M3-0696 (leaf M3.6.9.6.3.2.2.10.3.3.3.2.6)` | **verification prose formatting repaired**: exact HEAD/current comparison replaces only literal newline escapes in the completed framing record; all other evidence and code/test/catalogue/equivalent bytes unchanged; book clarifies existing raw-status behavior, documentation gates pass and live frontier remains delimiter proofs |
| `M3.6.9.6.3.2.2.10.3.3.3.2.4` | `ARCHOGEN-M3-0697 (leaf M3.6.9.6.3.2.2.10.3.3.3.2.4)` | **delimiter equivalents precisely bound**: same five actual sites survive before records and finish equivalent afterward/strict 0; fixed-program/pinned Unix formatter invariant includes encoded empty operands, compiled actual argv and wider-domain/wrong-oracle controls pass, all 1010 inputs/copy restore and older bodies unchanged; four per-file keys cover both guards and state edits, operator suite passes, canonical/book/ledger/live sync; exact staged focused required, final helper replay/initial closure next |
| `M3.6.9.6.3.2.2.10.3.4` | `ARCHOGEN-M3-0698 (leaf M3.6.9.6.3.2.2.10.3.4)` | **complete bounded helper replay passes**: all 24 actual current edits finish 19 named kills/5 scoped E, strict 0; all sixteen original obligations independently bind to eleven named kills/five E through exact source/operator/context and owned repair, all 1010 original/copied regular inputs restore under maintained hashes/modes and opaque pin preserved; altered evidence refuses, code/tests/catalogue/E unchanged, book/canonical/live sync; integration checkpoint next, whole-production adequacy remains open |
| `M3.6.9.6.3.2.2.10.4` | `ARCHOGEN-M3-0699 (leaf M3.6.9.6.3.2.2.10.4)` | **helper integration checkpoint passes**: first restricted tier fails process-detection controls, direct denied/readable OS probes and targeted RED arms isolate permissions; full bare-Git rerun passes all twelve steps/52 self-tests, exact activation-tree stamp and all 1010 captured inputs restore under maintained hashes/modes; code/tests unchanged, book/canonical/live sync, helper group closes and dropped-flag measurement is owned next; no remote CI or whole-production adequacy claim |
| `M3.6.9.6.3.2.2.11.1` | `ARCHOGEN-M3-0700 (leaf M3.6.9.6.3.2.2.11.1)` | **complete dropped-flag table measurement**: all fourteen actual edits finish two named kills/eleven survivors/one compiler refusal, strict 1 and criterion unmet; unchanged table/producer bind every original entry and retain all eleven P1 obligations, no guessed offsets or E classifications; all 1011 original/copied regular inputs restore under maintained hashes/modes, altered evidence refuses, code/tests/catalogue/E unchanged, book/live sync and actual producer/caller assessment owned next |
| `M3.6.9.6.3.2.2.11.2.1` | `ARCHOGEN-M3-0701 (leaf M3.6.9.6.3.2.2.11.2.1)` | **finite actual Cargo flag assessment**: successful ordinary/manifest-lint producer and unchanged public inventory records independently agree on raw argv, units/roles/packages/source hashes/env/configurations; altered evidence refuses, all 1011 original/copied inputs restore, source/tests/catalogue/E unchanged; ignored diagnostic/adapters explicitly qualified, all eleven obligations remain open, permanent check-cfg regression then source-bound remaining-flag evidence/conditional proofs owned; book/live sync |
| `M3.6.9.6.3.2.2.11.2.2` | `ARCHOGEN-M3-0702 (leaf M3.6.9.6.3.2.2.11.2.2)` | **check-cfg operand exclusion controlled**: permanent successful actual-Cargo/public parent independently requires expected producer operands and exact inventory identity/source hashes/env/retained arguments across manifest lint levels; actual current switch changes prior S to named finishing K/strict 0, exact candidate fails for operand leak then restored GREEN, all 1011 original/copied inputs restore and altered evidence refuses; production/older tests/catalogue/E unchanged, one original obligation closes, ten remain, book/live sync and final exact-staged focused required; source-bound flag assessment next |
| `M3.6.9.6.3.2.2.11.2.3` | `ARCHOGEN-M3-0703 (leaf M3.6.9.6.3.2.2.11.2.3)` | **initial flag premises assessed; execution defect owned**: installed Cargo pin/primary source and explicit-admission stable-profile refusals independently verified, finite evidence/ignored diagnostics qualified; actual public admitted build script writes owned sentinel and returns inventory, independent manifest/source/unit/sentinel audits reject altered evidence, all 1011 original/copied inputs restore and code/tests/catalogue/E unchanged; canonical/book/live disclose contract gap, immediate P1 hard metadata repair and premise/integration/source completion precede E work; no remaining-flag equivalence or final adequacy claim |
| `M3.6.9.6.3.2.2.11.2.5.1` | `ARCHOGEN-M3-0704 (leaf M3.6.9.6.3.2.2.11.2.5.1)` | **hard metadata execution boundary repaired**: both admitted dynamic targets reproduce owned sentinel execution before repair, permanent public refusal/static positive controls pass after; actual native reversal killed by both named finishing parents, all 1012 original/copied regular inputs restore and independent altered-evidence controls refuse; older code/tests/catalogue prefix/E/twins preserved, book/canonical/live sync and exact-staged focused required; caller/E premise and full integration checkpoint next |
| `M3.6.9.6.3.2.2.11.2.5.2` | `ARCHOGEN-M3-0705 (leaf M3.6.9.6.3.2.2.11.2.5.2)` | **repaired caller/integration checkpoint**: root/harness/generated preflight, fixed compiler/modes/environment/config and unchanged narrow delimiter reasons revalidated; actual compiled variants and wider-domain/wrong-oracle controls pass, every full local integration stage passes with official activation-tree stamp; all 1012 original regular inputs and opaque pin restore, independent altered-evidence audit refuses; close repair parent, sync book/canonical/live, source/stdlib/channel flag-domain completion next; no remote CI or full-production adequacy claim |
| `M3.6.9.6.3.2.2.11.2.6.1` | `ARCHOGEN-M3-0706 (leaf M3.6.9.6.3.2.2.11.2.6.1)` | **failed Cargo-home reset execution gap owned**: unchanged public static fixture executes untracked owned wrapper after ignored removal failure, returns error/no inventory; independent bytes/permissions/altered-evidence audit agrees, all 1012 original/copied inputs restore and code/tests/catalogue/E unchanged; pinned source identities retained, source-domain completion unclaimed, current book/canonical/live disclose P1 and immediate hard reset repair .6.2; cleanup due at clean boundary |
| `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.6` | `ARCHOGEN-M3-0785 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.6)` | actual controller/native descriptor-scarcity original/literal lifetime calibration and pinned recovery; permanent macOS parent/native1K full183,1111 restored/37 bindings/37 pins/12 falsifiers; prior source/control/catalogue/reason bytes exact, installed-byte seal/295 archives exact; focused/book/live gates; record presence next |
| `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.7` | `ARCHOGEN-M3-0786 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.7)` | direct record presence; only three state lines/two revalidated guards; actual47 proof/tool controls, native6K/full183,1112 restored/37 bindings/37 pins/12 falsifiers; prior tests/catalogue/reasons/installation-close seal/296 older archives exact; book/live/focused gates, permission handback then complete controller replay |
| `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.8` | `ARCHOGEN-M3-0787 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.6.8)` | complete current controller124/full183:113K/11U strict0, historical23S now K/quantity removed; 1113 restored/37 bindings/37 pins/12 receipt+2 refusal falsifiers, presence seal/297 archives exact; integration12/book/live gates, artifact source next |
| `M3.6.9.6.3.2.2.11.2.6.2` | `ARCHOGEN-M3-0708 (leaf M3.6.9.6.3.2.2.11.2.6.2)` | **hard fresh Cargo-home reset**: checked helper propagates actual errors before metadata and all root/harness/generated-target builds; permanent public wrapper/permission and file-state controls hold absent/populated/recovery cases; old ignored-error reversal and NotFound inversion finish named kills; all original/copied inputs restore, earlier production/tests reconstruct exactly, catalogue append only, E/twins unchanged; focused/book and staged gates precede commit, source/caller revalidation next |
| `M3.6.9.6.3.2.2.11.2.6.3.1` | `ARCHOGEN-M3-0709 (leaf M3.6.9.6.3.2.2.11.2.6.3.1)` | **pinned source/caller premises after hard reset**: source-kind/cache/std/channel/alias/visited-token branches revalidated with immutable official identities; inactive metadata dependencies distinguished from compiled units; permanent public and compiled delimiter/wider-domain/wrong-oracle controls pass, production/tests/catalogue/E exact; canonical/book/ledger/live synced and integration child next |
| `M3.6.9.6.3.2.2.11.2.6.3.2` | `ARCHOGEN-M3-0710 (leaf M3.6.9.6.3.2.2.11.2.6.3.2)` | **full postreset local integration**: every current stage passes with pinned local tools/stores and required OS reads; official activation stamp matches native Git and complete originals/scoped prior-copy inputs restore; code/tests/catalogue/E exact, source/caller and reset groups close; closure book/canonical/live synced, initial output cleanup assessment owned next before flag controls; no remote CI or final adequacy claim |
| `M3.6.9.6.3.2.2.11.2.7` | `ARCHOGEN-M3-0711 (leaf M3.6.9.6.3.2.2.11.2.7)` | **hard prior-inventory cleanup**: public P2 reproduction ties retained readable JSON to the preceding commit after ordinary refusal; directory output triggers late work; checked remove_file now stops before forms on actual errors, preserves contents and supports recovery; permanent public controls/native reversal and complete restoration hold the boundary, previous production/tests/E exact, skipped-cleanup anchor migrated and repair entry appended; canonical/book/live sync, focused 3/3 and book checks pass, exact final stamp/staged gates bind commit; proposal cleanup assessment owned next |
| `M3.6.9.6.3.2.2.11.2.8` | `ARCHOGEN-M3-0712 (leaf M3.6.9.6.3.2.2.11.2.8)` | **hard proposal cleanup**: ordinary refusal/new report beside protected readable old proposal reproduced with writable sibling output; checked remove_file now returns attributed Err before report replacement, preserves old files and supports refusal/valid retries; public controls/native finishing kills and complete restoration hold the boundary; gate/tests reconstructed, catalogue append-only, trust.rs/E unchanged; canonical/book/live sync, historical .9.4 status corrected, fixed ceilings retained; focused 3/3 and book pass, exact stamp/staged gates bind commit; actual-candidate flag proofs next |
| `M3.6.9.6.3.2.2.11.2.4.1` | `ARCHOGEN-M3-0713 (leaf M3.6.9.6.3.2.2.11.2.4.1)` | **lint-level actual-edit proofs**: eight consuming-bit switches compile beside original helper; actual pinned Cargo argv and literal normalized outputs, eight valid separate-argument counterexamples, consumed-value and wrong-oracle controls precede per-key E; native 8 S strict1 to 8 scoped E strict0 with altered-evidence refusal; 26 operator tests pass, original/current copy restore, older bodies/E prefix exact and production/catalogue unchanged; canonical/book/live sync, tight book ceilings trigger owned annex containment before cap/color |
| `M3.6.9.6.3.2.2.11.2.4.4` | `ARCHOGEN-M3-0714 (leaf M3.6.9.6.3.2.2.11.2.4.4)` | **book proof annex**: compiler/normalization and closed mutation-measurement blocks move byte-exact from ledger/trust into Annex C; old entry anchors link onward, all source entries/rules/counts/examples/limits retained; shorter accurate sidebar caption keeps generated index under its unchanged ceiling; pinned rendering and maintained book gates plus native Git/Python-system hash/HTML checks pass; code/tests/catalogue/E unchanged, live selects cap-lints proof |
| `M3.6.9.6.3.2.2.11.2.4.2` | `ARCHOGEN-M3-0715 (leaf M3.6.9.6.3.2.2.11.2.4.2)` | **conditional cap-lints proof**: compile actual consuming-bit edit beside original helper, independent real local argv/outputs and successful explicit-cap difference plus consumed-value/wrong-oracle controls precede precise E; native 1 S / strict 1 to 1 scoped E / strict 0 with audit falsifiers; 27 operator tests and all 1021 original/current copy restoration pass; old bodies/E prefix preserved with bounded operand helper extension, Rust/catalogue unchanged; canonical/Annex C/live sync, focused/book/staged gates bind commit, color next |
| `M3.6.9.6.3.2.2.11.2.4.3.1` | `ARCHOGEN-M3-0716 (leaf M3.6.9.6.3.2.2.11.2.4.3.1)` | **color compatibility boundary**: exact before key survives; pinned Cargo always emits compiler JSON, so ordinary separate-color probes refuse and cannot count as successful falsifiers; direct compiler and published test-owned RUSTC forwarding override succeed with real consumed artifact, raw/forwarded args distinct; source/artifact hashes and all 1026 original/current copy restoration agree; exploratory adapters qualified, no E/code/test/catalogue change; canonical/Annex C/live sync, permanent controls next |
| `M3.6.9.6.3.2.2.11.2.4.3.2` | `ARCHOGEN-M3-0717 (leaf M3.6.9.6.3.2.2.11.2.4.3.2)` | **permanent color proof**: actual consuming-bit variant compiled against independent ordinary/raw/physical argv, valid public-override builds for every supported color, real library consumers and JSON/consumed-value/three wrong-oracle controls before precise E; native 1 S / strict 1 to 1 scoped E / strict 0 with audit falsifiers; 28 operator tests and all 1026 original/current copy restoration pass, old bodies/E prefix exact, Rust/catalogue unchanged; canonical/Annex C/live sync, proof/producer parents close; focused/staged gates bind commit, final table replay and integration owned next |
| `M3.6.9.6.3.2.2.11.3` | `ARCHOGEN-M3-0718 (leaf M3.6.9.6.3.2.2.11.3)` | **final bounded flag table**: all 14 actual edits bind original outcomes, 3 named kills/10 scoped E/1 compiler refusal and strict 0; all 11 original survivors resolve on repaired caller, no retrospective E or refusal coverage; all 1026 originals/current copy restore, five altered-evidence arms refuse and production/tests/E/native stay exact; canonical/book/live sync, integration checkpoint owned next |
| `M3.6.9.6.3.2.2.11.4` | `ARCHOGEN-M3-0719 (leaf M3.6.9.6.3.2.2.11.4)` | **flag milestone checkpoint**: every current local integration stage passes after caller/output repairs and actual-edit proofs, exact unchanged activation stamp matches native Git, all 1026 originals/prior-copy inputs restore and false receipts refuse; code/tests/E unchanged, book/canonical/live sync closes bounded flag group; task-record repair .12 next, full production/review/runner obligations stay open |
| `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.8.1` | `ARCHOGEN-M3-0789 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.8.1)` | one existing test target becomes relative; actual old relocation fails/repaired copy+move preserves payload/hash; original assertions/production exact, native1K/full183/1115 restores/37 pins/four false receipts; measurement seal/299 archives exact; book/live/final staged focused, --test controls next |
| `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.8.2` | `ARCHOGEN-M3-0790 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.8.2)` | single-test flag forwarding and early/middle/final duplicate refusal held by one literal-query/error parent; exact historical5S become native5K/full184 strict0,1116 originals/copies restore/37 pins/five false receipts; prior repair seal/300 archives exact, production/old tests/reasons unchanged; book/live/final staged focused, option values next |
| `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.8.3` | `ARCHOGEN-M3-0791 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.8.3)` | nonempty/nonflaglike values for all naming options held with literal queries/exact errors; exact historical4S become native4K/full185 strict0,1118 restored/37 pins/five false receipts; preceding test-flag seal/301 archives exact, production/old tests/reasons unchanged; book/live/final staged focused, exact suffix prefix next |

### Complete native artifact-binder measurement

`2026-10-19`, leaf `.15.2.1.3.3.3.2.7`: the complete current artifact binder has **99 killed,
31 survived and 11 compiler-rejected edits**, strict exit `1`. The source adequacy criterion
is **unmet**. There are no equivalent, timeout or unjudged outcomes. All survivors have exact
owned resolution leaves below; they are not silently exempted. The first next repair fixes
an actual absolute symlink in an existing test fixture.

The whole measured file is `xtask/src/trust_artifact.rs`, SHA-256
`3c42078e46da0424e4c47aa467cd5e4b6edf851af1a0d50c811f595979aaa5c5`.
The discovery range is zero-based `[11,198]`; the production prefix including its separator
has SHA-256 `f51d00153db5f9be189efbe73ddb3ebd468cd1e48de93882d4c6e90406a2faa2`.
Fresh actual CLI output agrees exactly with maintained discovery; this command has no
selected-ID filter or equivalent reason file:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail
```

`2026-10-19`: all 141 edits run against all 183 current trust parents after a passing baseline.
Held native tree `a17071fc8835ba0af612ee1048a04f61fca7eb21` includes the measurement activation
and exact committed-controller seal. Native sandbox profile SHA-256
`2b3fbe199bddb4ba84c453a23334c7025308b06127ce3cc96f74b2a17680eedd`
permits repository-volume writes and the required null device. This records the actual local
execution, not remote CI or a caller result.

#### Every native result

`2026-10-19`: the table preserves all 141 current identities in native order. `K` means a
finishing named parent failed; `S` means the current full parent population did not distinguish
the edit; `U` means compilation rejected the candidate. A killed row names an actual witness
and gives the total distinct named killing parents; the raw receipt retains every such name.
The witness aliases expand to exact fresh parent names below. Survivor destinations are
suffixes under `M3.6.9.6.3.2.2.15.2.1.3.3.3.2`; each remains open.

| Alias | Actual named parent |
| --- | --- |
| A1 | `trust::artifact::tests::naming_query_excludes_source_codegen_and_environment_inputs` |
| A2 | `trust::artifact::tests::actual_empty_input_query_preserves_build_and_ignores_now_invalid_source` |
| A3 | `trust::tests::a_second_run_writes_the_same_bytes` |
| A4 | `trust::artifact::tests::artifact_names_refuse_missing_foreign_linked_and_ambiguous_outputs` |
| A5 | `trust::tests::every_program_target_is_a_root_or_classified_and_the_rest_is_reported` |
| A6 | `trust_generated::generator_tests::the_harness_may_use_its_pair_s_roles_and_no_other` |
| A7 | `trust::tests::naked_asm_is_refused` |

| Current ID | Outcome | Actual witness / refusal / owner |
| --- | --- | --- |
| `L16.iter.0` | K | A1; 112 named killing parents |
| `L16.iter.1` | K | A1; 112 named killing parents |
| `L16.iter.2` | K | A1; 112 named killing parents |
| `L16.guard.3` | K | A1; 1 named killing parent |
| `L16.guard.4` | K | A1; 1 named killing parent |
| `L16.guard.5` | K | A1; 1 named killing parent |
| `L16.guard.6` | K | A1; 112 named killing parents |
| `L16.guard.7` | K | A1; 112 named killing parents |
| `L16.guard.8` | K | A1; 112 named killing parents |
| `L16.guard.9` | S | owned `.8.2` |
| `L16.guard.10` | S | owned `.8.2` |
| `L16.guard.11` | S | owned `.8.2` |
| `L16.guard.12` | K | A1; 112 named killing parents |
| `L16.guard.13` | K | A1; 112 named killing parents |
| `L16.guard.14` | K | A1; 112 named killing parents |
| `L16.guard.15` | K | A1; 1 named killing parent |
| `L16.guard.16` | K | A1; 1 named killing parent |
| `L16.guard.17` | K | A1; 1 named killing parent |
| `L20.int.18` | K | A1; 112 named killing parents |
| `L20.int.19` | K | A1; 112 named killing parents |
| `L21.logic.20` | S | owned `.8.3` |
| `L21.not.21` | K | A1; 112 named killing parents |
| `L21.not.22` | K | A1; 112 named killing parents |
| `L21.dir.23` | K | A2; 111 named killing parents |
| `L23.eq.24` | K | A1; 112 named killing parents |
| `L23.logic.25` | K | A1; 112 named killing parents |
| `L23.logic.26` | K | A1; 1 named killing parent |
| `L23.logic.27` | K | A1; 112 named killing parents |
| `L23.cond.28` | K | A1; 112 named killing parents |
| `L23.cond.29` | K | A1; 112 named killing parents |
| `L23.not.30` | K | A1; 112 named killing parents |
| `L26.eq.31` | K | A1; 112 named killing parents |
| `L26.cond.32` | K | A1; 112 named killing parents |
| `L26.cond.33` | K | A1; 112 named killing parents |
| `L27.stmt.34` | K | A1; 112 named killing parents |
| `L29.stmt.35` | K | A1; 112 named killing parents |
| `L33.cond.36` | K | A3; 13 named killing parents |
| `L33.cond.37` | S | owned `.8.2` |
| `L33.not.38` | K | A3; 13 named killing parents |
| `L36.stmt.39` | S | owned `.8.2` |
| `L40.int.40` | K | A1; 112 named killing parents |
| `L40.int.41` | K | A1; 112 named killing parents |
| `L42.cond.42` | K | A1; 111 named killing parents |
| `L42.cond.43` | K | A1; 112 named killing parents |
| `L42.dir.44` | S | owned `.8.4` |
| `L42.lit.45` | S | owned `.8.4` |
| `L42.lit.46` | K | A1; 112 named killing parents |
| `L43.cond.47` | K | A1; 112 named killing parents |
| `L43.cond.48` | K | A1; 1 named killing parent |
| `L43.not.49` | K | A1; 112 named killing parents |
| `L46.stmt.50` | K | A1; 112 named killing parents |
| `L58.dir.51` | S | owned `.8.5` |
| `L65.cond.52` | K | A1; 112 named killing parents |
| `L65.cond.53` | K | A1; 1 named killing parent |
| `L65.not.54` | K | A1; 112 named killing parents |
| `L65.lit.55` | K | A1; 112 named killing parents |
| `L65.lit.56` | K | A1; 112 named killing parents |
| `L84.cond.57` | K | A4; 112 named killing parents |
| `L84.cond.58` | K | A4; 1 named killing parent |
| `L84.not.59` | K | A4; 112 named killing parents |
| `L84.dir.60` | U | compiler refusal assessed below |
| `L89.dir.61` | S | owned `.8.12` |
| `L89.iter.62` | K | A4; 1 named killing parent |
| `L89.iter.63` | K | A4; 1 named killing parent |
| `L89.iter.64` | S | owned `.8.6` |
| `L89.guard.65` | S | owned `.8.7` |
| `L89.guard.66` | S | owned `.8.7` |
| `L89.guard.67` | S | owned `.8.7` |
| `L89.guard.68` | K | A4; 1 named killing parent |
| `L89.guard.69` | K | A4; 1 named killing parent |
| `L89.guard.70` | S | owned `.8.6` |
| `L91.logic.71` | S | owned `.8.8` |
| `L91.alt.72` | U | compiler refusal assessed below |
| `L91.alt.73` | U | compiler refusal assessed below |
| `L92.logic.74` | S | owned `.8.8` |
| `L93.logic.75` | S | owned `.8.8` |
| `L93.not.76` | K | A4; 112 named killing parents |
| `L99.logic.77` | S | owned `.8.7` |
| `L99.logic.78` | S | owned `.8.7` |
| `L99.logic.79` | S | owned `.8.13` |
| `L99.cond.80` | K | A4; 112 named killing parents |
| `L99.cond.81` | K | A4; 1 named killing parent |
| `L99.not.82` | K | A4; 112 named killing parents |
| `L105.logic.83` | K | A4; 112 named killing parents |
| `L105.logic.84` | K | A4; 111 named killing parents |
| `L105.logic.85` | K | A4; 111 named killing parents |
| `L105.cond.86` | K | A4; 1 named killing parent |
| `L105.cond.87` | K | A4; 112 named killing parents |
| `L105.not.88` | K | A4; 111 named killing parents |
| `L135.eq.89` | K | A2; 111 named killing parents |
| `L136.logic.90` | K | A3; 49 named killing parents |
| `L137.eq.91` | K | A2; 95 named killing parents |
| `L137.eq.92` | K | A2; 97 named killing parents |
| `L137.logic.93` | K | A5; 4 named killing parents |
| `L138.eq.94` | K | A3; 13 named killing parents |
| `L138.eq.95` | K | A3; 13 named killing parents |
| `L138.logic.96` | K | A6; 4 named killing parents |
| `L155.eq.97` | K | A2; 111 named killing parents |
| `L156.logic.98` | K | A2; 17 named killing parents |
| `L158.eq.99` | K | A2; 17 named killing parents |
| `L159.eq.100` | K | A2; 17 named killing parents |
| `L159.logic.101` | K | A2; 111 named killing parents |
| `L160.eq.102` | K | A2; 17 named killing parents |
| `L160.logic.103` | S | owned `.8.9` |
| `L179.cond.104` | K | A2; 111 named killing parents |
| `L179.cond.105` | K | A2; 1 named killing parent |
| `L179.not.106` | K | A2; 111 named killing parents |
| `L195.eq.107` | K | A2; 110 named killing parents |
| `L16.chain.108` | K | A1; 112 named killing parents |
| `L16.chain.109` | K | A1; 112 named killing parents |
| `L21.chain.110` | U | compiler refusal assessed below |
| `L21.chain.111` | U | compiler refusal assessed below |
| `L21.chain.112` | S | owned `.8.3` |
| `L57.chain.113` | S | owned `.8.5` |
| `L57.chain.114` | K | A1; 1 named killing parent |
| `L65.contains.115` | U | compiler refusal assessed below |
| `L65.contains.116` | S | owned `.8.14` |
| `L65.contains.117` | U | compiler refusal assessed below |
| `L65.contains.118` | K | A1; 112 named killing parents |
| `L92.contains.119` | U | compiler refusal assessed below |
| `L92.contains.120` | U | compiler refusal assessed below |
| `L92.contains.121` | U | compiler refusal assessed below |
| `L92.contains.122` | U | compiler refusal assessed below |
| `L133.chain.123` | K | A7; 110 named killing parents |
| `L133.chain.124` | K | A7; 109 named killing parents |
| `L134.chain.125` | S | owned `.8.11` |
| `L134.chain.126` | S | owned `.8.11` |
| `L134.chain.127` | K | A7; 110 named killing parents |
| `L144.chain.128` | S | owned `.8.11` |
| `L144.chain.129` | K | A2; 1 named killing parent |
| `L153.chain.130` | K | A2; 39 named killing parents |
| `L153.chain.131` | K | A2; 42 named killing parents |
| `L154.chain.132` | K | A2; 1 named killing parent |
| `L154.chain.133` | K | A2; 1 named killing parent |
| `L154.chain.134` | K | A2; 49 named killing parents |
| `L156.chain.135` | K | A2; 111 named killing parents |
| `L156.chain.136` | K | A2; 111 named killing parents |
| `L175.chain.137` | S | owned `.8.10` |
| `L175.chain.138` | K | A2; 111 named killing parents |
| `L21.clogic.139` | S | owned `.8.3` |
| `L21.clogic.140` | S | owned `.8.3` |

| Operator | Attempted | Killed | Survived | Compiler refused |
| --- | --- | --- | --- | --- |
| `iter` | 6 | 5 | 1 | 0 |
| `guard` | 21 | 14 | 7 | 0 |
| `int` | 4 | 4 | 0 | 0 |
| `logic` | 19 | 11 | 8 | 0 |
| `not` | 11 | 11 | 0 | 0 |
| `dir` | 5 | 1 | 3 | 1 |
| `eq` | 12 | 12 | 0 | 0 |
| `cond` | 20 | 19 | 1 | 0 |
| `stmt` | 4 | 3 | 1 | 0 |
| `lit` | 4 | 3 | 1 | 0 |
| `alt` | 2 | 0 | 0 | 2 |
| `chain` | 23 | 15 | 6 | 2 |
| `contains` | 8 | 1 | 1 | 6 |
| `clogic` | 2 | 0 | 2 | 0 |

#### Compiler refusals are not coverage

`2026-10-19`: all 11 `U` candidates are explicitly compiler-rejected. They contribute no
semantic killing parent or equivalence. These diagnostics match the actual attempted edit:

| ID | First actual diagnostic | Source/API assessment |
| --- | --- | --- |
| `L84.dir.60` | ``error[E0599]: no method named `contains` found for struct `PathBuf` in the current scope`` | PathBuf has starts_with, not the substituted contains method. |
| `L91.alt.72` | ``error: unexpected closing delimiter: `}``` | Alternative mutation of matches! breaks Rust delimiter structure; no executable candidate. |
| `L91.alt.73` | ``error: mismatched closing delimiter: `}``` | Alternative mutation of matches! breaks Rust delimiter structure; no executable candidate. |
| `L21.chain.110` | ``error[E0061]: this method takes 0 arguments but 1 argument was supplied`` | Option iterator-style chain supplies an argument to a zero-argument method. |
| `L21.chain.111` | ``error[E0599]: no method named `rev` found for enum `Option<&String>` in the current scope`` | Option<&String> is not the substituted reversible iterator. |
| `L65.contains.115` | ``error[E0308]: mismatched types`` | BTreeSet<&str> first/last returns Option<&&str>; direct Some(&str) comparison has incompatible type. |
| `L65.contains.117` | ``error[E0308]: mismatched types`` | BTreeSet<&str> first/last returns Option<&&str>; direct Some(&str) comparison has incompatible type. |
| `L92.contains.119` | ``error[E0599]: no method named `first` found for reference `&str` in the current scope`` | A str slice has no first/last collection method; these membership rewrites are outside its API. |
| `L92.contains.120` | ``error[E0599]: no method named `first` found for reference `&str` in the current scope`` | A str slice has no first/last collection method; these membership rewrites are outside its API. |
| `L92.contains.121` | ``error[E0599]: `&str` is not an iterator`` | A str slice has no first/last collection method; these membership rewrites are outside its API. |
| `L92.contains.122` | ``error[E0599]: `&str` is not an iterator`` | A str slice has no first/last collection method; these membership rewrites are outside its API. |

#### Assessed survivors and repair order

All probes below compile the actual helper or whole artifact component with the exact literal
edit. Whole-component probes retain exact caller type declarations, the actual environment
constructor and actual pinned compiler artifacts; file hashes have independent Python checks.
They establish the stated helper behavior, with limits made explicit. Production is unchanged;
these are adequacy gaps and proof obligations, except for the separately identified fixture link.
No finite agreement or native survival is classified as equivalent.

| Resolution leaf | Reproduction, impact and required next action | Actual survivor identities |
| --- | --- | --- |
| `.8.1` — relative artifact fixture link; P1, first | Existing output-refusal test stores an absolute link target; relocation loses its payload. Change only that target to relative root, preserve every assertion, independently copy/relocate/read it and replay the actual refusal parent. This repairs fixture locality, not production selection. | separate actual defect / final checkpoint |
| `.8.2` — --test forwarding and duplicate refusal; P1 | Single --test must be forwarded; repeats at early, middle and final positions must refuse. Literal edits accept a repeated middle flag or omit the valid query flag. Add one bounded naming-helper control with independent exact query/refusal expectations. | `L16.guard.9`, `L16.guard.10`, `L16.guard.11`, `L33.cond.37`, `L36.stmt.39` |
| `.8.3` — nonempty naming-option values; P1 | Present empty and flaglike values currently lack controls. Exact helper edits accept invalid --out-dir values while valid outputs agrees. Hold each naming-option family and both invalid shapes, exact error attribution and valid query controls. | `L21.logic.20`, `L21.chain.112`, `L21.clogic.139`, `L21.clogic.140` |
| `.8.4` — exact extra-filename prefix; P1 | Substring and missing-equals edits forward metadata=extra-filename=-bait or bare extra-filename. Require only exact extra-filename= forwarding and valid suffix controls. Reconcile old .15.2.1.4 identities by exact source signature before closing that historical obligation. | `L42.dir.44`, `L42.lit.45` |
| `.8.5` — joined option spellings and lookalikes; P1 | Truncation accepts unsupported joined families after crate-name; contains falsely refuses an ordinary source filename containing --target=. Hold every joined family and supported ordinary lookalikes with exact errors and valid controls. | `L58.dir.51`, `L57.chain.113` |
| `.8.6` — complete output visitation and selection positions; P1 | A three-name library response loses middle validation or middle rlib selection. Use actual pinned three-output order, sole rlib in each position and a missing middle nonselected file. Hold independent selected path/hash plus valid recovery. | `L89.iter.64`, `L89.guard.70` |
| `.8.7` — regular non-symlink outputs at every position; P1 | Position guards accept a middle symlink to owned foreign bytes. Lost regular-file tests select bytes from an owned named pipe. Hold every position and nonregular kind with valid file controls, bounded parent failures, no stranded writer/read or outer-timeout verdict. | `L89.guard.65`, `L89.guard.66`, `L89.guard.67`, `L99.logic.77`, `L99.logic.78` |
| `.8.8` — basename grammar and all-output uniqueness; P1 | Existing nested/parent-relative files defeat lost syntax checks; duplicate nonselected library output defeats lost uniqueness. Dot/empty mutants retain refusal but lose the precise invalid-basename category. Hold exact diagnostics before filesystem access, valid controls and nonselected duplicate cases. | `L91.logic.71`, `L92.logic.74`, `L93.logic.75` |
| `.8.9` — ordinary and canonical source prefixes under a tree alias; P1 | Whole actual component with actual environment/compiler and relative tree alias accepts ordinary/canonical absolute source argv; conjunction loses both. Hold both geometries and physical control, invalidated source, unchanged artifact bytes/hash and real invocation evidence. | `L160.logic.103` |
| `.8.10` — complete pinned naming-query environment; P1 | Exact environment/whole component with nonexistent explicit pin refuses originally; take(1) keeps PATH and succeeds through ambient default. Establish a bounded meaningful pin/environment control, preserving original source. A nonexistent-pin helper falsifier alone is not a supported Cargo build claim. | `L175.chain.137` |
| `.8.11` — complete metadata roots: producer and defensive contract; P1 assessment | Constructed two matching library keys/distinct roots refuse originally but truncation selects real compiled output. Bind exact Cargo metadata producer and every caller: library kinds may share one source. Add justified direct defensive controls or exact guarded all-domain proof; no equivalence from synthetic reachability or survival. | `L134.chain.125`, `L134.chain.126`, `L144.chain.128` |
| `.8.12` — reported-output diagnostic ordering; P2 before caller | Reversed split preserves valid selection but attributes first failure to the later missing name. Review accepted diagnostic contract; hold a meaningful deterministic first-failure oracle or precise source proof. Never invent an arbitrary kill or claim acceptance changed. | `L89.dir.61` |
| `.8.13` — explicit symlink term after regular-file predicate; P2 before caller | Finite regular/named-pipe controls agree after deleting only is_symlink. Bind symlink_metadata snapshot and published FileType/Metadata exclusivity/purity, source/callers and falsified axioms. Choose exact guarded proof or justified verified simplification; no E from the finite grid. | `L99.logic.79` |
| `.8.14` — required crate-name minimum-member predicate; P2 before caller | Exact helper/mutant agree over exercised optional subsets; crate-name appears lexical minimum of closed inserted-key set. Prove all insertions, BTreeSet/string ordering and source/caller closure; compile actual original/edit, reject wider smaller-key premise and guard it before any exact reason/native E. | `L65.contains.116` |
| `.8.15` — complete final artifact replay and checkpoint; after .1-.14 | Fresh complete CLI/discovery/source/caller/producer/reason census and all current native parents, no selected projection. Reconcile every initial survivor/removed edit, compiler refusal, exact proof reason and historical suffix obligation. Restore original/copied inputs/modes/names; reject false receipts; integration at milestone; book/live sync before caller pivot. | separate actual defect / final checkpoint |

The actual library-output probes use a response containing dylib, rlib and staticlib names.
The metadata position edits accept a middle relative symlink and hash its owned foreign target;
the regular-file edits accept writer-supplied named-pipe bytes. All named-pipe probes have finite
writer/reader deadlines and join their workers. Valid regular files and fresh recovery are held.
Missing-middle and sole-middle-rlib probes distinguish visitation from artifact selection.
Syntax witnesses create the nested and parent-relative files first, preventing an unrelated
missing-file error from masking the removed basename judgment. The uniqueness witness repeats
a nonselected output while keeping exactly one valid rlib. Dot/empty-name and output-order
changes currently affect diagnostic category/order, not successful selection; their owners
must justify a meaningful diagnostic contract rather than assert an acceptance defect.

The last metadata truncations are calibrated against explicitly constructed differing roots.
Cargo library kinds normally share the library's source; the producer/caller reachability proof
is still required. Likewise, dropping the query environment ignores an explicitly nonexistent
toolchain pin and succeeds through an ambient default in the helper probe. That probe proves
the pin is lost, not that such a pin belongs to a successful Cargo build. These scopes remain
visible in `.8.10` and `.8.11`.

The required-crate-name membership edit agrees over the exercised optional-key subsets because
the observed insertion keys place crate-name first lexically. The regular-file-only predicate
also agrees on the exercised file kinds. Both need exact source/API premises, rejected wider
domains and source/caller guards before any equivalence reason can be considered.

#### Real fixture-locality defect: owned repair first

The existing `artifact_names_refuse_missing_foreign_linked_and_ambiguous_outputs` parent creates
its link with `symlink(out.join("root"), out.join("link"))`; `out` is runtime absolute, so the
stored link target is absolute. The actual retained fixture confirms this with `readlink`.
An owned copy/move calibration reproduces loss of the target after relocating the source tree;
a relative `root` link preserves the same payload and independently checked hash. The original
fixture was untouched during the frozen replay. `.8.1` owns the immediate next safe repair after
this measurement commit, including preservation of every existing assertion and actual
relocation/refusal checks. This is a real portability defect in a test fixture, not a claim
that production selected a wrong artifact.

#### Restoration, receipt controls and preservation

`2026-10-19`: all 1114 captured regular original/current-copy inputs restore with independent
Python/system hashes and modes; the gitlink identity is checked as metadata only. The copied
parent listing equals all 183 fresh names. All 37 direct source/caller/tool bindings and
37 pinned public producer sources agree. Production, previous tests, mutation catalogues and
active reason records remain byte-exact to `bab3a537598dd611515fd13f60c9b94ca3a93849`.

`2026-10-19`: all 12 falsified receipts refuse: changed scope, missing result, changed edit,
invented parent, changed total, changed exit, wrong file, wrong source hash, changed outcome,
duplicate result, invented equivalent and reduced population. The exact audit program has
SHA-256 `1e351c6170886c2ae13618b04d4823e28a33131d50c5e1e4b2fde32b2445a77f`.
The committed controller container and final-replay body seal exactly, with all 298 older
archive files unchanged. Historical task-table routing reconstructs all 36 original notes
verbatim and rejects omitted/changed notes; no book-size ceiling is introduced.

Raw plan, actual CLI/listing, execution/native log/exit, complete named results, original/copy
restoration, audit, seal, source-bound calibrations and routing audit remain under
`target/compiler-artifact-full/`. The executable audit and detailed results are retained evidence,
not a dependency of normal project builds. An initial malformed snapshot CLI invocation refused
before native work; the corrected flag-only invocation generated the input snapshot. No result
is inferred from that refused invocation.

The preceding controller checkpoint's full integration passed on unchanged production; this
ordinary measurement commit runs the documentation gates and book build. Resolution controls
run focused checks when Rust changes, and the complete artifact checkpoint requires full
integration. Caller measurement and final source/premise/reason replay plus independent review
remain ahead; neither this measured population nor the bounded helper probes closes F30.

### Relative artifact-output fixture link repaired

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.1`: the existing output-refusal test now creates
its link with `symlink("root", out.join("link"))`. Only the stored target literal changes.
Every existing assertion and every production byte remain exact. This fixes the fixture's
relocation defect; it changes no production artifact selection or adequacy classification.
The preceding measurement remains the historical source epoch it records.

The actual old parent passes but stores an absolute target. An independent check copies its
real fixture with links preserved and moves the source directory: neither copied nor moved
link resolves. It restores the source in a finalizer and verifies all regular payloads/modes.
After the repair the same actual parent passes; copied and moved links both resolve to the
same payload. The target returned by `readlink` is exactly `root`, including in the native
mutation copy. No global cache, toolchain or other repository is written.

| Actual fixture | Stored link | After copy and source move | Payload SHA-256 |
| --- | --- | --- | --- |
| Before | absolute runtime path | does not resolve | `064d7f281f05977e9f8b0f34f693e69bac17a13f1640d4decac712ea3128329f` |
| Repaired | `root` | both links resolve | `064d7f281f05977e9f8b0f34f693e69bac17a13f1640d4decac712ea3128329f` |

The digest is independently equal under Python and the system SHA-256 command. Byte-level
reconstruction replaces the relative target with its old expression and recovers the complete
committed source exactly. The source-free naming production prefix is unchanged; the whole
file SHA-256 after this test-only repair is
`c45cb81b2e1bcc202eadfbf9a15bb74952efacedb7fe8b12bc5ba72f3517c62c`.

```bash
cargo test -q -p xtask \
  trust::artifact::tests::artifact_names_refuse_missing_foreign_linked_and_ambiguous_outputs \
  -- --exact
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L99.cond.81
```

`2026-10-19`: the selected native edit removes file-kind validation and is killed by the
unchanged named output-refusal parent, with all 183 current parents available and strict exit
`0`. This is a selected regression check, not a new complete artifact replay. All 1115 original
and copied regular inputs restore; direct bindings and public producer pins remain at 37 each.
Altered invented-parent, missing-result, outcome and exit receipts refuse. The measured leaf
seals byte-exact, with all 299 previous archives unchanged.
Held native tree `99eac01cf0ad66f22a7239054c0223a4d5b6b03e` predates completion documentation.
Audit SHA-256 `17d862b3fa7d49b142ae23a13f41270c259767216f64755458cb8c7179773783`
binds the actual old/repaired fixture, exact source reconstruction and native/restoration checks.
Raw evidence remains in `target/artifact-fixture-locality/`; final staged focused, book and
commit doctrine gates complete this repair. The next owned leaf holds `--test` forwarding
and duplicate refusal. The measured survivors and complete final artifact checkpoint remain open.

### Test-flag naming query controls

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.2`: a permanent direct naming-helper parent now
requires exact forwarding of a single `--test` flag and exact refusal of duplicates.
Production and every previous test remain byte-exact; the relative-link fixture repair stays
in place. These controls close the measured forwarding/duplicate gaps, not a production defect.

A single flag is exercised before, between and after the required naming options. Literal
query expectations retain observed option order and the output directory `outputs`. For example:

```text
observed: rustc --crate-name probe --out-dir outputs --test
query:    --print=file-names - --crate-name probe --test
directory: outputs
```

Repeated flags at early, middle and final positions must return exactly
`duplicate observed --test`. A middle duplicate is followed by ignored `-C opt-level=3`
arguments, preventing endpoint-only checking from appearing sufficient. A single early flag
has its own literal query order; the test does not compute an expectation through the helper.
This parent is pure and finite and launches no compiler, worker or fixture process.

The source-bound initial full measurement classified every edit below as survived; actual
literal-helper calibration independently demonstrates accepted repeated flags or loss of
valid query forwarding. After the new parent, the same exact production source edits have
these native outcomes:

| Current ID | Lost judgment | Actual native outcome |
| --- | --- | --- |
| `L16.guard.9` | duplicate check only at first argument | K, new intended parent |
| `L16.guard.10` | duplicate check only at last argument | K, new intended parent |
| `L16.guard.11` | duplicate check only at argument endpoints | K, new intended parent |
| `L33.cond.37` | duplicate guard replaced with false | K, new intended parent |
| `L36.stmt.39` | valid --test query argument omitted | K, new intended parent |

The exact new parent is
`trust::artifact::tests::naming_query_preserves_test_and_refuses_duplicate_test_options`.
Every old parent name remains; only this name is added.

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L16.guard.9 --only L16.guard.10 --only L16.guard.11 \
  --only L33.cond.37 --only L36.stmt.39
```

`2026-10-19`: all 5 selected edits finish killed against all 184 current trust parents, strict
exit `0`, with the new intended name in every actual failure list. There are no selected
survivors, compiler refusals, equivalents, timeouts or unjudged outcomes. All 1116 regular
original/copied inputs restore, with independent Python/system hashes and modes. All 37 direct
bindings and 37 pinned public producer sources agree. Invented-parent, missing-result,
changed-outcome, changed-exit and duplicate-result receipts refuse. The committed fixture
repair seals exactly and all 300 older archive files remain unchanged.

Held native tree `f929ac7974353f043378e810e5085e030f9e5877` precedes completion documentation.
Whole test file SHA-256 `9264bfe668221706c3404c5559c6ea33c721e77f2e8f52086272e764da0738cf`;
the production prefix is unchanged. Audit SHA-256
`6728f64af252265b3a31e5fee3ce3d02745d885b7d539a537041f8034e292863`
binds historical/current edit signatures, exact parent/source preservation, native outcomes,
restoration and false receipts. Raw evidence remains in `target/artifact-test-flags/`.
An ad hoc progress display initially assumed an unpadded native column; the actual raw log
showed completed kills. Correcting it to accept whitespace runs fixed the display; the
unchanged maintained parser audits the final results. No replay retry or result was changed.
Final staged focused/book/commit gates complete this leaf; nonempty naming-option values are
next. The complete current artifact replay remains separately owned after all resolution
leaves, so these selected kills do not project a new full-file or caller adequacy result.

`2026-10-19`: the required routine changelog rollover seals its oldest 20 entries into
`docs/history/changelog/0031.md`. Independent concatenation reconstructs the complete
pre-rollover staged changelog byte-for-byte; previous history segments remain exact.

### Naming-option values: nonempty and nonflaglike

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.3`: a permanent direct naming-helper parent now
holds value validation for `--crate-name`, `--crate-type`, `--target` and `--out-dir`.
Each family has a literal valid-query expectation and malformed controls for an empty value
and `-flaglike`. Every malformed input must return exactly `missing observed <option> value`,
including the option's own spelling. Existing absent-value and duplicate controls remain.
Production and every earlier test are byte-exact; this closes defensive-parser coverage gaps
without asserting that malformed argv belongs to a successful supported Cargo compilation.

| Observed option | Valid value | Valid naming-query suffix | Returned output directory |
| --- | --- | --- | --- |
| `--crate-name` | `probe` | `--crate-name probe` | `outputs` |
| `--crate-type` | `rlib` | `--crate-name probe --crate-type rlib` | `outputs` |
| `--target` | `host` | `--crate-name probe --target host` | `outputs` |
| `--out-dir` | `outputs` | `--crate-name probe` | `outputs` |

Valid in this table means accepted by the helper value grammar; `host` is illustrative
and does not name an installed compiler target here. No compiler is launched by this parent.
Each expected query starts with the literal `--print=file-names -`. The output directory is
retained for locating artifacts and is never forwarded to the query. The expected vectors are
literal test data rather than computed by the helper. This parent is pure, finite and creates
no compiler process or filesystem fixture.

The initial complete measurement records these exact source edits as survived. Actual compiled
helper calibrations distinguish loss of the nonempty and nonflaglike terms separately and
together; the permanent parent now gives these actual native results:

| Current ID | Actual source edit | Native result |
| --- | --- | --- |
| `L21.logic.20` | value conjunction becomes disjunction | K, new intended parent |
| `L21.chain.112` | complete value filter omitted | K, new intended parent |
| `L21.clogic.139` | nonempty term omitted | K, new intended parent |
| `L21.clogic.140` | nonflaglike term omitted | K, new intended parent |

The exact new parent is
`trust::artifact::tests::naming_query_requires_nonempty_nonflaglike_option_values`.
It is the only addition to the parent population; every previous name remains.

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L21.logic.20 --only L21.chain.112 \
  --only L21.clogic.139 --only L21.clogic.140
```

`2026-10-19`: all 4 selected edits finish killed against all 185 current trust parents, strict
exit `0`, with the intended new parent in every failure list. No selected survivor, compiler
refusal, equivalent, timeout or unjudged result occurs. All 1118 regular original/copied
inputs restore with independent Python/system hashes and modes. All 37 direct bindings and
37 pinned public producer sources agree. Invented-parent, missing-result, changed-outcome,
changed-exit and duplicate-result receipts refuse. The committed test-flag leaf seals exactly;
all 301 prior task archive files remain unchanged.

Held native tree `976733a91232da6d1cdfa297dfc6c3cc5f38e070` precedes completion documentation.
Whole file SHA-256 `50e3629323a989734b0a8ed46ffd893f61ee491edbdfbb0982c4abbfaa11c42c`;
the production prefix remains exact. Audit SHA-256
`f49175270f8287cac13d3ebc8011377621eee96c861e279e026151c7409c470f`
binds actual source/parent preservation, historical edit identity, native results, restoration
and false receipts. Raw evidence remains in `target/artifact-option-values/`. Final staged
focused/book/commit gates complete this leaf. Exact `extra-filename=` prefix controls are next;
the complete current artifact replay and caller adequacy remain separately owned and open.

### Exact extra-filename naming-query boundary

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.4`: a permanent pure naming-helper parent now
holds the exact `extra-filename=` prefix. It forwards the original `-C` and value unchanged
for an exact match, including an empty suffix, and excludes nonmatching codegen arguments.
Production and every previous test remain byte-exact.

| Observed value following `-C` | Required query tail |
| --- | --- |
| `extra-filename=-hash` | `-C extra-filename=-hash` |
| `extra-filename=` | `-C extra-filename=` |
| `metadata=extra-filename=-bait` | omitted |
| `extra-filename` | omitted |
| `extra-filename-other=-hash` | omitted |

Each literal expected query begins `--print=file-names - --crate-name probe`; its returned
output directory remains `outputs`. These are helper-boundary controls, including malformed
and nonnaming lookalikes. The test launches no compiler and does not assert that every negative
example is an argument of a successful supported Cargo compilation.

| Exact helper edit | Initial full measurement | Current native result |
| --- | --- | --- |
| `L42.dir.44`, prefix test becomes substring test | S | K, new intended parent |
| `L42.lit.45`, required equals sign omitted | S | K, new intended parent |

The actual new parent is
`trust::artifact::tests::naming_query_forwards_only_the_exact_extra_filename_option`.
It is the only added parent name. Every earlier name, source body and assertion is preserved.

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L42.dir.44 --only L42.lit.45
```

`2026-10-19`: both selected edits finish killed against all 186 current trust parents, strict
exit `0`, and the new intended name occurs in each failure list. No selected survivor,
compiler refusal, equivalent, timeout or unjudged result occurs. All 1119 original/copied
regular inputs restore with independent Python/system hashes and modes; all 37 direct bindings
and 37 pinned public producer sources agree. Invented-parent, missing-result, changed-outcome,
changed-exit and duplicate-result receipts refuse. The committed option-value leaf seals
exactly and all 302 prior task archive files remain unchanged.

#### Historical caller suffix extraction remains separately owned

The older suffix obligations are **not** the helper edits above. The caller still extracts a
dep-info filename suffix from adjacent observed arguments with this expression:

```rust
(w[0] == "-C" && w[1].starts_with("extra-filename="))
    .then(|| w[1]["extra-filename=".len()..].to_owned())
```

Exact original/replacement/operator identity, independently rediscovered in the recorded old
source and current caller, gives this unique mapping:

| Historical caller ID | Current caller ID | Disposition |
| --- | --- | --- |
| `L814.dir.284` | `L729.dir.201` | source-bound producer/domain assessment pending |
| `L814.lit.285` | `L729.lit.202` | source-bound producer/domain assessment pending |

The historical commit is `541ec182f44c1e6013e95ad30f736ae88d3e42e1`, after the permitted
source boundary, with whole-source SHA-256
`069d8e598fd9d6c12306fbe0f091635b60f942b747e78e6167e92b41e0ca01ba`.
Current `xtask/src/trust.rs` remains SHA-256
`fdc28b267ceb173da133e6b0d287758df703ecba6e7b3ca4453964b1c7aa567c`.
A missing mapping row and an invented helper-as-caller mapping both refuse. No current caller
native result, equivalence or retirement is inferred from this identity reconciliation or the
helper kills. `M3.6.9.6.3.2.2.15.2.1.4.1` owns actual successful-Cargo producer/domain assessment
after the complete artifact checkpoint and before caller closure, with bounded committed
controls, repair or proof as its evidence requires.

`2026-10-19`: six retained compiler-queue descriptions still called the historical 83-edit
source epoch current. Their wording now identifies source069d8e59 as historical; all 20
queued survivor identities retain their original recorded outcomes and ownership. Independent
reverse reconstruction recovers the prior text exactly, and identity order/status lines are
unchanged. Current suffix replay is routed explicitly to its current child. The artifact
resolution container likewise calls its original cohort measured rather than implying a fresh
full result after every selected control. This repairs resume/documentation drift without
relabeling any measurement.

`2026-10-19`: eight additional completed task notes move verbatim into the existing unbounded
historical book ledger. Reconstructing the private rows is byte-exact and omitted/altered notes
refuse; an independent comparison retains every previous ledger byte. Every original leaf and
work-unit identity remains in its private task table. No book-size ceiling is introduced.

Held native tree `4c9ee9eaef77c553d127f834516d2936dc53a524` precedes completion documentation.
Whole artifact file SHA-256 `b068f48eaf224dd74c37b576fd55275ca69526b91d7c3595d3f1394815d2740b`;
its production prefix remains exact. Audit SHA-256
`7d86abe7c1a8a559f32b7fabb43aa0a52e51b81779fbc92cae2673dedf437113`
binds actual source/parent preservation, historical caller identity, native results, restoration,
false receipts and note routing. Raw evidence remains in `target/artifact-extra-filename/`,
including the separate epoch-routing reconstruction. Final staged focused/book/commit gates
complete this leaf. Joined naming-option spellings and lookalikes are next; the complete
current artifact replay, retained caller obligations and global adequacy remain open.

### Complete joined-option refusal and ordinary lookalikes

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.5`: a permanent pure naming-helper parent now
holds every unsupported joined naming spelling and preserves ordinary source strings containing
those spellings later in their path. Production and every previous test remain byte-exact.

| Joined argument: must refuse | Ordinary path lookalike: must retain the base query |
| --- | --- |
| `--crate-name=joined` | `src/look--crate-name=joined.rs` |
| `--crate-type=rlib` | `src/look--crate-type=rlib.rs` |
| `--target=host` | `src/look--target=host.rs` |
| `--out-dir=outputs` | `src/look--out-dir=outputs.rs` |
| `-Cextra-filename=-hash` | `src/look-Cextra-filename=-hash.rs` |

Each joined form must return exactly `unsupported observed naming option <argument>`, retaining
its complete spelling. Each lookalike, plus an ordinary `source.rs` control, must produce the
literal query `--print=file-names - --crate-name probe` and output directory `outputs`.
The parent is finite and launches no compiler or fixture process. It holds the helper's lexical
boundary, not successful Cargo builds for every path. The earlier source-bound pinned compiler
filename control already accepted a file containing `--target=`; that separate experiment
remains under `target/compiler-artifact-full/joined-option-calibration/` with its explicit scope.

| Exact current edit | Initial full measurement | Current native result |
| --- | --- | --- |
| `L58.dir.51`, joined prefix check becomes substring check | S | K, new intended parent |
| `L57.chain.113`, joined-family population truncated to its first element | S | K, new intended parent |

The exact new parent is
`trust::artifact::tests::naming_query_refuses_each_joined_option_but_allows_source_lookalikes`.
It is the only added parent name; every earlier name remains. Literal expected queries and
messages do not call the helper to calculate their oracle.

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L58.dir.51 --only L57.chain.113
```

`2026-10-19`: both selected edits finish killed against all 187 current trust parents, strict
exit `0`, with the new intended parent in each failure list. No selected survivor, compiler
refusal, equivalent, timeout or unjudged result occurs. All 1120 original/copied regular
inputs restore with independent Python/system hashes and modes. All 37 direct bindings and
37 pinned public producer sources agree. Invented-parent, missing-result, changed-outcome,
changed-exit and duplicate-result receipts refuse. The committed exact-prefix leaf seals
exactly; all 303 previous task archive files remain unchanged.

Held native tree `50ad1f847af3be30b840b74dfb5187c881fd7543` precedes completion documentation.
Whole artifact file SHA-256 `c3285ddddad646b5b0f89b6dc2234f86953b533f37963165cea116e9d6348af4`;
the production prefix remains exact. Audit SHA-256
`3946b27a676e50b72435335371ba1b064ea4cb3a404c4300f0f3a15de074a78d`
binds historical/current edit identity, source/parent preservation, native results, restoration
and false receipts. Raw evidence remains in `target/artifact-joined-options/`. Final staged
focused/book/commit gates complete this leaf; complete output visitation and selection positions
are next. A final complete current artifact replay remains separately owned, so these selected
kills establish no new full-file, caller or global adequacy result.

### Complete output visitation and rlib selection positions

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.6`: a permanent bounded parent obtains the pinned
compiler's actual three-type filename report, validates the selector on that report and holds
selection of the sole rlib in every position. Each nonselected output must also exist: removing
it must refuse with that filename attributed, followed by exact recovery. Production and every
previous test remain byte-exact.

The parent uses the actual base-environment constructor, pinned toolchain and null stdin:

```text
rustc --print=file-names - --crate-name probe \
  --crate-type dylib --crate-type rlib --crate-type staticlib
```

It requires exactly the expected shared-library, rlib and static-library name set. On the measured
macOS host the fresh response is `libprobe.dylib`, `libprobe.rlib`, `libprobe.a`; the supported
Linux fixture expects `libprobe.so` for the shared library. It feeds the actual reported order
to the selector, without requiring the producer to promise one order. Explicit protocol
permutations then place the rlib first, middle and last:

| Sole rlib position | Protocol response order |
| --- | --- |
| First | rlib, shared library, static library |
| Middle | shared library, rlib, static library |
| Last | shared library, static library, rlib |

The named files contain controlled regular-file payloads. The selected payload is the literal
`selected library bytes` followed by LF; the others contain `nonselected output bytes` followed
by LF. Every valid call must select `release/deps/libprobe.rlib` with SHA-256
`26fff0e84dc91af757184906354cddbb54702d2078c781ce0ad5eb1ff693de8d`.
The expected digest agrees independently between Python, the system SHA-256 command and the
actual selector's Rust digest. These fixtures exercise selection/hash/recovery; existing public
artifact parents retain actual compiled binary and library checks.

For each selected position, each nonselected file is removed in turn. The parent calls the
selector and restores the file before asserting the attributed refusal, then immediately checks
the valid path/hash again. In particular, a valid first rlib cannot hide a missing middle output.
The original and native-copy fixtures retain all payloads after the current selected replay.
The actual report remains in the fixture's `reported-names.txt`; the bounded parent leaves no
compiler or worker running.

| Exact current edit | Initial full measurement | Current native result |
| --- | --- | --- |
| `L89.iter.64`, output loop restricted to endpoints | S | K, new intended parent |
| `L89.guard.70`, artifact selection condition restricted to endpoints | S | K, new intended parent |

The exact new parent is
`trust::artifact::tests::every_reported_output_is_read_and_the_rlib_can_be_in_each_position`.
It is the only added name, and every earlier parent remains.

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L89.iter.64 --only L89.guard.70
```

`2026-10-19`: both selected edits finish killed against all 188 current trust parents, strict
exit `0`, with the new intended name in each failure list. No selected survivor, compiler refusal,
equivalent, timeout or unjudged result occurs. All 1121 original/copied regular inputs
restore with independent Python/system hashes and modes. All 37 direct bindings and 37 pinned
public producer sources agree. Invented-parent, missing-result, changed-outcome, changed-exit
and duplicate-result receipts refuse. The committed joined-option leaf seals exactly and all
304 prior task archive files remain unchanged. Original/copy fixture audits independently
verify the actual filename sets, regular file kinds and each complete payload/hash.

Held native tree `dbb687bdb69dfe640b399d55a9a9dcc9a8ba319a` precedes completion documentation.
Whole artifact file SHA-256 `1cb191cbc4645e8c41650ef605177534586b770bbaf18f143c7e557d52e67d7e`;
its production prefix remains exact. Audit SHA-256
`417aa5fc0f086a69ab9ad6aa5f795d4ddb73dba09fb7e5591a5125fa938a346c`
binds source/parent preservation, actual fixture recovery, historical/native edit identities,
restoration and false receipts. Raw evidence remains in `target/artifact-output-positions/`.
Final staged focused/book/commit gates complete this leaf; every-position regular non-symlink
file controls are next. Complete artifact/caller/global adequacy remains separately owned and open.

### Regular files and symlinks at every output position

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.7`: permanent bounded parents now require regular
non-symlink files at every reported position, including nonselected outputs. One parent holds
real relative symlinks; another holds named pipes with an independently checked finite writer.
Production and every previous test remain byte-exact. Existing directory refusal remains; the
separate all-domain FileType/predicate assessment stays owned by `.8.13`.

Both fixtures start with shared-library, rlib and static-library names and controlled regular
payloads. Their valid selection is `release/deps/libprobe.rlib`, with the previously independently
verified SHA-256 `26fff0e84dc91af757184906354cddbb54702d2078c781ce0ad5eb1ff693de8d`.
Every negative case restores the regular file before asserting the result and checks valid
selection/hash again after recovery.

#### Relative symlinks, including nonselected outputs

The symlink parent replaces each reported file in turn with a real relative link to an owned
`foreign.rlib` outside the role target. Its stored target is `../../../foreign.rlib`, so moving
the complete fixture preserves its meaning. The parent independently reads the link to confirm
it resolves to the foreign payload, then requires exactly
`compiler artifact <name> is not a regular non-symlink file` from the selector. It removes the
link and restores the regular file before checking the refusal; foreign bytes remain unchanged.
Thus a nonselected link must refuse even when the selected rlib and its hash would stay valid.

#### Named pipes: actual byte flow and no-reader refusal

The test-only `scripts/fixtures/artifact_output_pipe.py` provides a finite resource fixture.
Its paths derive from its own repository root and stay beneath the pipe fixture directory on
the same device. It refuses non-pipe and linked endpoints; the writer verifies the opened file
type too. Its private protocol is:

| Operation | Required outcome |
| --- | --- |
| `create --pipe <owned-path>` | create a mode-restricted named pipe and report `created` |
| `write --pipe <owned-path> --stop <owned-marker>` | try a nonblocking write-open; report `wrote` after a complete payload or `stopped` after a regular stop marker |
| No reader or marker before its finite deadline | refuse explicitly, with nonzero exit |

A positive control first reads a separate real named pipe and requires the exact bytes
`stream artifact bytes` followed by LF and writer result `wrote`. This checks the writer can
actually exchange bytes before it becomes a refusal witness. Each output position is then
replaced with a named pipe, starting with the selected rlib. The selector must return the exact
regular-file refusal without opening a reader. The parent signals stop, joins the writer,
removes the pipe and restores the file before checking result `stopped` and the refusal.
A lost guard allows the writer to report `wrote` and supplies stream bytes to the selector;
that is a finishing named failure rather than an outer mutation timeout.

The stream payload's independently equal Python/system SHA-256 is
`4c27fec8360fdfc27464d0ddf2c92953614100af57399c0f351aa0f8ae9f06c3`, distinct
from the permitted regular rlib payload. Every writer process is joined; the ordinary test
uses the existing bounded-parent contract as an additional execution limit.

`2026-10-19`: seven standalone fixture controls hold regular-stop handling, the finite
no-reader deadline, regular/linked endpoint refusals, outside endpoint/stop refusals and
nonregular stop-marker refusal. All control children are joined and their endpoints removed.
The writer uses its own monotonic deadline of three seconds; the measured no-reader refusal took
about three seconds. The positive and refusal parents pass together on the original source.

#### Actual native kills and preservation

| Exact current edit | Required actual killing parent |
| --- | --- |
| `L89.guard.65`, metadata judged only at first position | linked-output parent |
| `L89.guard.66`, metadata judged only at last position | linked-output parent |
| `L89.guard.67`, metadata judged only at endpoints | linked-output parent |
| `L99.logic.77`, file-kind disjunction becomes conjunction | named-pipe parent |
| `L99.logic.78`, regular-file term omitted | named-pipe parent |

The exact parent names are:

```text
trust::artifact::tests::linked_outputs_are_refused_at_every_reported_position
trust::artifact::tests::named_pipe_outputs_are_refused_without_reading_at_every_position
```

Every old parent name remains; only these names are added. The named-pipe parent also kills
the positional metadata edits; the complete raw failure lists retain those additional names.

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L89.guard.65 --only L89.guard.66 --only L89.guard.67 \
  --only L99.logic.77 --only L99.logic.78
```

`2026-10-19`: all 5 selected edits finish killed against all 190 current trust parents, strict
exit `0`, with the required new names in their actual failure lists. No selected survivor,
compiler refusal, equivalent, timeout or unjudged result occurs. All 1123 regular original/copied
inputs restore with independent Python/system hashes and modes. All 38 direct bindings,
including the new writer source, and 37 pinned public producer sources agree. Invented-parent,
missing-result, changed-outcome, changed-exit and duplicate-result receipts refuse. The committed
output-position leaf seals exactly; all 305 previous task archive files remain unchanged.
Original/copy fixture audits verify complete payload recovery and the absence of remaining
pipes, links and stop markers.

Held native tree `608626871555d3ae58b6f7e214125cff8f0b57cc` precedes completion documentation.
Whole artifact file SHA-256 `60f0ff3fdc31255926bb3f927372bd27c6a4c60d92de65bfe097264a022cbc49`;
its production prefix remains exact. Writer source SHA-256
`45b3cbfe9115136cc999e832e0117420a0d3bd4c7ae2c2a441485c4c3f9cca36`. Audit SHA-256
`5fad04c7e558326e0374b4dd47044185e5805ac1d84c371a2603fb1f84f9da10`
binds actual source/parent preservation, writer controls, both complete fixture recoveries,
historical/native edit identities, restoration and false receipts. Raw evidence remains in
`target/artifact-output-kinds/`. An initial Cargo invocation put the second filter before its
argument separator and refused before testing; passing both exact filters to libtest corrected
that invocation. No native result is inferred from the refusal. Final staged focused/book/commit
gates complete this leaf; basename grammar and all-output uniqueness are next.
The explicit symlink-term assessment, complete artifact replay and caller/global adequacy remain open.

### Basename grammar and all-output uniqueness

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.8`: permanent bounded parents now hold malformed
basename validation and uniqueness of every reported output, including nonselected files.
Production, the finite pipe fixture and every previous test remain byte-exact. Valid selection
and recovery retain the literal rlib path and independently verified payload hash.

#### Existing malformed files and precise protocol diagnostics

The grammar parent creates readable controlled witnesses before exercising malformed reports:
a nested file beneath the output directory, a parent-relative file outside the role target but
inside the fixture, and Unix filenames containing a backslash or carriage return. These files
exist, so a missing-file error cannot mask loss of the grammar check. Malformed path cases run
before the diagnostic-only cases, preserving a semantic acceptance witness for the path edit.

| Reported name | Required behavior |
| --- | --- |
| `nested/inside` | refuse despite an existing readable nested file |
| `"../../../foreign.rlib"` | refuse despite an existing readable fixture file outside the role target |
| `"back\\slash"` | refuse despite an existing readable Unix filename containing backslash |
| `"carriage\rname"` | refuse despite an existing readable Unix filename containing carriage return |
| `"nul\0name"` | refuse as an invalid basename |
| empty, `.` or `..` | refuse as an invalid basename |

Every case must return exactly `invalid or duplicate compiler artifact basename`. The target
and output directories are valid and canonicalized first; the held diagnostic then identifies
the malformed protocol name before its per-output metadata lookup. The dot/empty edit already
retains refusal through later directory checks, but changes this observable reason to a file-kind
error. The parent preserves the precise protocol diagnosis rather than treating unrelated
filesystem failure as proof of validation. All witness bytes remain unchanged, and valid query
selection is checked before and after each case.

#### Duplicates that do not create artifact ambiguity

The uniqueness parent repeats a nonselected shared-library name while keeping exactly one rlib.
It exercises a final repeat, a first/last repeat and an early repeat, then a repeated selected
rlib as an additional diagnostic control. Every reported file exists and is regular.

| Response shape | Required outcome |
| --- | --- |
| rlib, shared, shared | invalid-or-duplicate basename refusal |
| shared, rlib, shared | the same refusal |
| shared, shared, rlib | the same refusal |
| rlib, rlib, shared | the same refusal before later artifact ambiguity |

The required exact message is `invalid or duplicate compiler artifact basename`. Without the
uniqueness judgment the nonselected-repeat cases can still select one rlib and its unchanged
hash, so they distinguish the rule directly. Unique valid reports pass before and after each
case with `release/deps/libprobe.rlib` and SHA-256
`26fff0e84dc91af757184906354cddbb54702d2078c781ce0ad5eb1ff693de8d`.

#### Actual native results

| Exact current edit | Initial full measurement | Required actual killing parent |
| --- | --- | --- |
| `L91.logic.71`, dot/empty disjunction becomes conjunction | S | grammar parent |
| `L92.logic.74`, forbidden-character disjunction becomes conjunction | S | grammar parent |
| `L93.logic.75`, uniqueness disjunction becomes conjunction | S | nonselected-duplicate parent |

The exact added names are:

```text
trust::artifact::tests::malformed_basenames_are_refused_before_their_output_lookup
trust::artifact::tests::every_output_name_is_unique_including_nonselected_outputs
```

Every earlier parent remains. The grammar parent also kills the uniqueness edit because that
edit weakens the combined condition; the raw receipt retains every additional actual name.

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L91.logic.71 --only L92.logic.74 --only L93.logic.75
```

`2026-10-19`: all 3 selected edits finish killed against all 192 current trust parents, strict
exit `0`, with their required new names in the actual failure lists. No selected survivor,
compiler refusal, equivalent, timeout or unjudged result occurs. All 1124 original/copied
regular inputs restore with independent Python/system hashes and modes. All 38 direct bindings
and 37 pinned public producer sources agree. Invented-parent, missing-result, changed-outcome,
changed-exit and duplicate-result receipts refuse. The committed file-kind leaf seals exactly;
all 306 previous task archive files remain unchanged. Original/copy fixture audits verify
all normal and malformed witness payloads with independent content hashes; stdin hashing keeps
control characters in filenames from affecting checksum-output parsing.

Held native tree `51c082478d60c284b1812dce53b0d2e210eb489c` precedes completion documentation.
Whole artifact file SHA-256 `95ad9e9262a1b848f99273ee0f5c16ac823e52276ae1e9e55e364c9e63bb8b71`;
its production prefix remains exact. Audit SHA-256
`d988dac8a42bdd0d2b8bb1579001d23c455fdc3493d579218f88ecd63d665bed`
binds source/parent preservation, historical/native edit identities, both fixture recoveries,
restoration and false receipts. Raw evidence remains in `target/artifact-basename-controls/`.
Final staged focused/book/commit gates complete this leaf; ordinary/canonical source prefixes
under a tree alias are next. Complete artifact/caller/global adequacy remains separately owned and open.

#### Audit population correction

`2026-10-19`: the first completed audit compared every older archive byte but reused its
archive-list variable for a later fixture listing. The report therefore emitted the final
fixture size as the archive count. This was caught before committing, owned in the current
leaf, and corrected with distinct archive/fixture variables and independent population checks:

| Independent population | Correct measured count |
| --- | --- |
| Prior Git tree archive paths | 306 |
| Committed task-history index rows | 306 |
| Current filesystem archive files | 307 |
| Newly sealed archive | 1 |

The corrected audit rejects reported counts of `3`, `305` and `307`, including the original
shadowed value. The native results, source bindings, payload verification and actual archive
byte comparisons are unchanged. The incorrect report/program remain under
`target/artifact-basename-controls/audit-counter-before-fix.*` for inspection; the corrected
`audit.py` reproduces rejection of the stale count and writes the verified `audit.json`. The earlier mismatched parent-log filename
also refused before publication and was corrected to the actual `parents.log` evidence.

### Source prefixes under a tree alias

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.9`: a bounded permanent parent holds source
matching through a physical tree and its relative symbolic alias. The source selector must
match the observed compiler argument against either the ordinary tree prefix or its canonical
prefix. Replacing the final disjunction with conjunction wrongly requires both prefixes at once.

#### Real compilation and source-free selection

The parent uses the actual catalog environment constructor, pinned toolchain and compiler
([Rust toolchain source](ledger.md#rust-toolchain)). It creates a repository-local physical tree,
a relative `tree-alias -> physical-tree` link, a metadata source `source.rs` and a real binary.
The recorded source argument is passed to the actual successful compilation in each case.

| Case | Selector tree | Actual compiler source argument | Required original behavior |
| --- | --- | --- | --- |
| physical control | physical tree | physical tree / source.rs | select the compiled binary |
| ordinary alias | alias tree | alias tree / source.rs | select the compiled binary |
| canonical alias | alias tree | physical tree / source.rs | select the compiled binary |

For example, the ordinary alias supplies the alias spelling of the source while metadata
continues to identify `source.rs`. The canonical alias supplies its physical spelling under
the same aliased selector tree. Neither case depends on reading the source to make a naming
decision. After each successful compilation, the parent records its actual arguments and
independently hashes the produced executable by feeding its bytes to the system checksum tool.

Before calling the actual selector it replaces the source with:

```rust
compile_error!("selection must not recompile the source");
```

Selection must still return `release/deps/probe-alias` with the independently measured hash.
The executable bytes, invalid source bytes and complete output-directory population must
remain unchanged. Running the selected executable must succeed and print `alias-artifact`
followed by a newline. The parent checks each artifact against its own before-image; it does
not require independent compilations to produce identical bytes.

These controls exercise the complete actual artifact component and environment with real
compiler invocations. They establish the helper's accepted source geometries; they do not
claim every geometry was independently produced by a complete Cargo inventory build.

#### Exact mutation and restored evidence

| Exact edit | Earlier complete measurement | New native result |
| --- | --- | --- |
| `L160.logic.103`, final source-prefix disjunction becomes conjunction | S | K, required alias parent |

The retained whole-component calibration runs the cases independently: the original accepts
every geometry, while the edit accepts the physical control and refuses each alias geometry
as missing or ambiguous observed compilations. The permanent parent orders the physical
control first, then the ordinary and canonical cases. Its actual mutant execution fails at
ordinary selection, after completing the physical control; it never reaches the canonical
case. The copy audit verifies exactly that partial fixture, while the original fixture
contains all completed cases. This prevents later-case evidence from being inferred from
an earlier failure.

```text
trust::artifact::tests::source_selection_preserves_ordinary_and_canonical_prefixes_under_a_tree_alias
```

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L160.logic.103
```

`2026-10-19`: the selected edit finishes killed against all 193 current trust parents, strict
exit `0`, with the new parent in its actual failure list. No survivor, equivalent, compiler
refusal, timeout or unjudged result occurs. All 1125 original/copied regular inputs
restore with independent Python/system hashes and modes; all 38 direct bindings and 37 pinned
public producer sources agree. Invented-parent, missing-result, changed-outcome, changed-exit
and duplicate-result receipts refuse. The basename leaf seals exactly; all 307 older task
archives remain byte-exact. Git, committed index and filesystem populations agree, and
incorrect archive counts refuse. Production, all older tests and the pipe fixture are unchanged.

Held native tree `a6a361ad834d3ea054dae257fcb9ede3ccf135a9` precedes completion documentation.
Whole artifact file SHA-256 `fcd59faf5248bdd8c016f973e7ff9974747b9bc9f1a7e0b3315085a733d8e781`;
its production prefix remains exact. Local supplemental audit SHA-256
`f878f365377db270ab8512a4eefbceddf256bdff1c80f311024119755652c524`
binds the input, parent, source, fixture, archive and false-receipt checks. Supplemental raw
programs and observations are untracked in `target/artifact-source-aliases/`; the tracked
parent and native command above reproduce the behavior and mutation result.
Final staged focused/book/commit checks complete this leaf. The complete pinned naming-query
environment is next; complete artifact/caller/global adequacy remains separately owned and open.

### Explicit toolchain environment for naming queries

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.10`: a permanent bounded parent now holds the
naming query's explicit toolchain pin. The query must receive the supplied allowlisted
environment after clearing inherited variables; losing its tail can silently select an
ambient compiler instead of honoring an unavailable explicit pin.

#### Actual pinned build and independent refusal

The parent obtains the actual catalog environment, asks the real compiler for its version
and requires the configured pinned version. It then uses that compiler and the recorded
arguments to build a real executable
([Rust toolchain source](ledger.md#rust-toolchain)). Before selection it independently
hashes the executable bytes through the system checksum tool and invalidates the source:

```rust
compile_error!("naming must preserve the compiled artifact");
```

Valid selection must still return `release/deps/probe-pinned` and the independent before
hash. The binary must execute successfully and print `pinned-artifact` followed by a newline.
The invalid source, artifact bytes and full output-directory population must remain unchanged.

The control then clones that same environment and changes only `RUSTUP_TOOLCHAIN` to the
deliberately unavailable custom name `archogen-artifact-query-missing-toolchain`. A direct
real `rustc --version` command under that environment must fail and identify that exact pin.
This independently establishes the refusal before asking the actual selector to perform its
source-free naming query.

| Supplied environment | Independent compiler control | Required selector behavior |
| --- | --- | --- |
| actual pinned environment | version succeeds; actual compilation succeeds | select existing artifact and its hash |
| same environment, unavailable explicit custom pin | version refuses and identifies that pin | naming-query failure identifying that pin |
| valid environment again | previously compiled bytes remain valid | selection and unchanged bytes still pass |

The selector result is recorded before the assertion. Valid recovery runs even if the bad-pin
result is unexpectedly successful, so the mutation's completed fixture retains the before
artifact and invalid source checks. The unavailable-pin result must begin with
`compiler artifact naming query failed:` and contain the explicit custom pin.

Both runtime environment records preserve the actual constructor order and values. Their
key sets are `PATH`, `HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN`, `CARGO_HOME`,
`CARGO_TARGET_DIR`, `TMPDIR`, `TMP` and `TEMP`. The Cargo home, target and temporary
stores derive from the fixture root on the repository volume. The installed toolchain is
the existing read-only toolchain dependency. An operating-system compiler cache observed
beneath the fixture's target-local temporary directory remains project-local.

This is a defensive helper-boundary control. It does not assert that an unavailable pin can
produce a successful Cargo build, nor that this single pin falsifier independently tests
every environment key's effect.

#### Exact environment-tail mutation

| Exact edit | Earlier complete measurement | Actual new result |
| --- | --- | --- |
| `L175.chain.137`, truncate the supplied environment iterator | S | K, required explicit-pin parent |

The actual edit adds `take(1)` before the key/value mapping. The constructor supplies `PATH`
first, so the modified query loses the explicit pin. In the copied native execution the
independent compiler command still refuses the missing pin, but the modified selector returns
the existing artifact and its correct hash. That observed acceptance is precisely the
behavior the new parent rejects; the original selector reports the expected pin failure.

```text
trust::artifact::tests::naming_query_honors_the_explicit_toolchain_environment
```

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L175.chain.137
```

`2026-10-19`: the selected edit finishes killed against all 194 current trust parents, strict
exit `0`, with the new parent in its actual failure list. No selected survivor, equivalent,
compiler refusal, timeout or unjudged outcome occurs. All 1126 original/copied regular
inputs restore with independent Python/system hashes and modes; all 38 direct bindings and
37 pinned public producer sources agree. Invented-parent, missing-result, changed-outcome,
changed-exit and duplicate-result receipts refuse. The alias leaf seals exactly, and all
308 older task archives remain byte-exact with independent Git/index/filesystem counts.
Production and every earlier test remain unchanged.

Held native tree `7271a5c64ddb7aaee566318ed5306a4a7d6b652b` precedes completion documentation.
Whole artifact file SHA-256 `576635928cb5d67afd460503de8417b250e4e20c19cb469341e2c89f678531e9`;
its production prefix remains exact. Local supplemental audit SHA-256
`20eca7edf8614d6a00ef3c4acd7bc9568f8ecf42e89edafc98d5fa1bebcbfe9c`
binds both complete fixtures, environment records, actual original refusal and mutant
acceptance, restoration, archives and false receipts. Supplemental programs/observations are
untracked in `target/artifact-query-environment/`; the tracked parent and native command
above reproduce the behavior and mutation result. Final staged focused/book/commit checks
complete this leaf. Metadata producer/root contracts are next; complete artifact/caller/global
adequacy remains separately owned and open.

### Complete metadata source roots

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.11`: a permanent bounded parent now holds
shared-source library metadata and the selector's defensive refusal of distinct metadata
sources. The producer domain and the wider helper-input domain remain explicit.

#### Published producer, pinned implementation and actual caller

Cargo permits a package's library target to report several crate kinds while retaining its
source path. A package has one library target. These published contracts are documented in
[the metadata format](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html) and
[Cargo targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html);
the pinned producer is also bound through the [toolchain ledger](ledger.md#rust-toolchain).

The retained pinned target serializer emits a target's kind list and its source path from
the same target object. Its library-kind serializer emits the configured crate kinds.
The actual inventory caller obtains successful locked, offline metadata, then adds that
source under each reported kind in its source map. Library selection unions these matching
roots before requiring exactly one distinct source.

| Source-bound production census | Result |
| --- | --- |
| Metadata source-map construction site | inventory metadata loop |
| Artifact selector call site | `build`, after observed compilation |
| Library kind keys with the same source | deduplicated to that source |
| Missing or distinct matching sources | explicit metadata-source refusal |

The exact caller and producer hashes are retained with the supplemental contract record.
The maintained production caller, source type and map construction remain unchanged.

#### Real shared-source metadata and actual compiled artifacts

The new parent creates an isolated repository-local workspace with a library configured as:

```toml
[lib]
path = "a.rs"
crate-type = ["rlib", "dylib"]
```

It generates the fixture's lock file offline, then runs the same metadata command shape as
the caller: `cargo metadata --offline --locked --format-version 1`. The real metadata must
identify the fixture package, its library, both kinds and the actual source path. The parent
maps these actual kind/source fields to the selector's source-map interface:

| Metadata key | Repository source |
| --- | --- |
| package `p`, kind `dylib`, name `probe` | `p/a.rs` |
| package `p`, kind `rlib`, name `probe` | `p/a.rs` |

Using the actual pinned environment/compiler, it then compiles that source to the requested
library forms and compiles a different source as a second real rlib. The actual arguments and
before-images of every output are retained. The selector must accept the real shared-source
map and choose `release/deps/libprobe-a.rlib` with its independent system-measured hash.

Before every selection, both source files contain an unconditional compile error. Every
valid and defensive selection must preserve those invalid source bytes and every compiled
output byte, with no added or removed output names. Valid selection/recovery runs around
each conflicting case. The compiler-produced libraries, not arbitrary nonexistent paths,
therefore witness what a truncating selector could wrongly select.

#### Constructed conflicting roots and defensive contract

These negative maps are deliberately constructed helper inputs, not successful metadata
outputs claimed reachable from Cargo's library model:

| Defensive input | Required selector result |
| --- | --- |
| matching kind keys point to different compiled sources | missing-or-ambiguous metadata-source refusal |
| one matching key contains both compiled sources | the same refusal |

The parent holds the existing complete error, including the role name, rather than accepting
a later missing-compilation or filesystem failure as evidence that ambiguity was checked.
Both possible sources have real observed compilations and existing artifacts, so taking a
first or last root can select a genuine output. This protects the helper's broader defensive
contract. It does not turn the constructed conflict into an end-to-end Cargo defect or an
all-domain reachability claim. No equivalent classification is inferred.

| Exact edit | Earlier complete measurement | Required native result |
| --- | --- | --- |
| `L134.chain.125`, first matching metadata key only | S | K, new metadata-roots parent |
| `L134.chain.126`, last matching metadata key only | S | K, new metadata-roots parent |
| `L144.chain.128`, first distinct unioned root only | S | K, new metadata-roots parent |

```text
trust::artifact::tests::metadata_roots_share_one_source_and_refuse_distinct_compiled_sources
```

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L134.chain.125 --only L134.chain.126 --only L144.chain.128
```

`2026-10-19`: all 3 selected edits finish killed against all 195 current trust parents, strict
exit `0`, with the required new parent in their actual failure lists. No selected survivor,
equivalent, compiler refusal, timeout or unjudged outcome occurs. All 1127 original/copied
regular inputs restore with independent Python/system hashes and modes; all 38 direct
bindings and 37 pinned public producer sources agree. Invented-parent, missing-result,
changed-outcome, changed-exit and duplicate-result receipts refuse. The environment leaf
seals exactly; all 309 older task archives remain byte-exact with independent population
checks. Production and every older test remain unchanged.

The original fixture completes both conflict checks. The final native copy is the
union-truncation edit: it accepts the first conflicting-key source, then fails the parent's
refusal assertion before the single-key conflict case. The audit verifies that actual
failure point, both invalidated sources, every before-image and the full actual metadata.
It never infers that the copied test completed later cases. Cargo/compiler caches created
inside these fixtures remain under their project-local Cargo home and target stores.

Held native tree `30be6cc0f7b7b9e0330f290b5f02086c0aac8e66` precedes completion documentation.
Whole artifact file SHA-256 `86a90f2767a257b500214086193efa30910de2937f3eb5dd359e94911a730d40`;
its production prefix remains exact. Local supplemental audit SHA-256
`7315f5c50a93e947edb98f73503127a76034d5d516e4c39655332d45c25490c6`
binds source/caller/producer preservation, actual metadata/arguments/artifacts, both fixture
states, restoration, archive counts and false receipts. Supplemental programs/observations
are untracked in `target/artifact-metadata-roots/`; the tracked parent and native command
above reproduce the behavior and mutation result. Final staged focused/book/commit checks
complete this leaf. Reported-output diagnostic ordering is next; complete artifact/caller/global
adequacy remains separately owned and open.

### First reported output failure

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.12`: a permanent bounded parent now holds
the first failing basename in a supplied compiler-output response. Artifact validation reports
that basename through the build result and inventory diagnostic. This lets the user start
with the earliest reported output problem for that response.

This is a diagnostic contract. It neither promises that the compiler emits the same output
order across runs nor claims that reversing validation changes valid artifact selection.
It holds the existing forward traversal and first-error attribution rather than creating a
successful-selection defect from a different error name.

#### Literal order controls with real filesystem witnesses

The parent starts with the regular library-output fixture: a shared-library file, a sole
selected rlib and a static-library file. It confirms valid selection and its independently
verified payload hash, then removes both nonselected files. Separate metadata lookups must
confirm that both are missing before the selector sees the response.

| Supplied response order | Required first failing basename |
| --- | --- |
| shared, rlib, static | shared |
| static, rlib, shared | static |
| rlib, shared, static | shared |
| rlib, static, shared | static |
| shared, static, rlib | shared |
| static, shared, rlib | static |

Each expected first name is literal in the parent. The selected rlib remains valid, so
missing nonselected outputs test the full validation path. The error must begin with the
expected basename and its attribution separator; the operating system's remaining error
wording is not fixed.

After the query returns, the parent restores both missing files and confirms valid
selection before asserting the diagnostic. Every resulting fixture therefore recovers even
when the mutation makes that assertion fail. Both normal and mutated runs retain the
unchanged selected payload and the complete regular output population.

#### Actual reversal and scoped native result

| Exact edit | Earlier complete measurement | New result |
| --- | --- | --- |
| `L89.dir.61`, response `split` becomes `rsplit` | S | K, required first-failure parent |

The copied mutant reports the static-library name for the initial response, while the
original reports the shared-library name. Its existing rlib selection remains valid after
fixture recovery. The original completes every literal response order; the copied parent
fails its initial diagnostic assertion after restoring all payloads. No later copied case is
inferred. The unchanged production caller propagates the helper's error through `build`
into the inventory failure text.

```text
trust::artifact::tests::output_diagnostics_name_the_first_failure_in_the_reported_order
```

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L89.dir.61
```

`2026-10-19`: the selected edit finishes killed against all 196 current trust parents, strict
exit `0`, with the new parent in the actual failure list. No selected survivor, equivalent,
compiler refusal, timeout or unjudged outcome occurs. All 1128 original/copied regular
inputs restore with independent Python/system hashes and modes; all 38 direct bindings and
37 pinned public producer sources agree. Invented-parent, missing-result, changed-outcome,
changed-exit and duplicate-result receipts refuse. The metadata leaf seals exactly; all
310 prior task archives remain byte-exact with independent population checks.
Production, all previous tests and the finite pipe fixture remain unchanged.

Held native tree `2a0cfbff07e83b4bc312a600d18f87f3be885786` precedes completion documentation.
Whole artifact file SHA-256 `7bb0d87d15f8f48356a39ef4bad371e8d4a28955176a9980704059cea7bc3001`;
its production prefix remains exact. Local supplemental audit SHA-256
`9550dabad17d661c06ced0c32534d8558a0df85dcc7b671b31ad17dac80f2dc8`
binds the original order cases, actual copied diagnostic/failure point, both recovered payload
fixtures, caller/source preservation, restoration, archives and false receipts. Supplemental
programs/observations are untracked in `target/artifact-output-order/`; the tracked parent and
native command above reproduce the behavior and mutation result. Final staged focused/book/commit
checks complete this leaf. The explicit symlink term is next; complete artifact/caller/global
adequacy remains separately owned and open.

### Redundant symlink predicate on one metadata snapshot

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.13`: the explicit symlink term is removed from
the regular-file refusal after an API/source proof. Symlinks, directories, pipes and other
nonregular kinds still refuse. The same error text and every semantic parent remain intact. The external absolute-cwd proof binding is revalidated below.

```diff
-if !metadata.is_file() || metadata.file_type().is_symlink() {
+if !metadata.is_file() {
```

This simplification relies on the actual `std::fs::Metadata` value returned by the unchanged
`symlink_metadata` call. It is not a general identity for unrelated booleans or arbitrary
getter functions, and finite filesystem examples alone do not establish it.

#### Pinned API and pure stored-value predicates

The [FileType contract](https://doc.rust-lang.org/std/fs/struct.FileType.html#method.is_file)
makes regular-file and symbolic-link results mutually exclusive.
[Metadata::is_file](https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.is_file)
also excludes symlink metadata, and
[symlink_metadata](https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html) obtains the
link's own metadata. The proof uses the documentation and published source installed with
the [pinned toolchain](ledger.md#rust-toolchain), rather than assuming the current online
documentation has that pinned version.

In that pinned public implementation, `Metadata::is_file` delegates to its stored file
type's regular-file predicate. `Metadata::file_type` obtains that same stored type.
The relevant Unix implementation copies the saved mode from its metadata record; file-kind
predicates compare masked mode bits. These getters neither query the path again nor mutate
the record, perform fallible work or introduce observable side effects. The selector binds
one immutable metadata value and uses these exact methods on that same value.

The installed toolchain is a necessary read-only external dependency. Documentation/source
paths derive at runtime from `rustc --print sysroot`; local evidence copies live beneath
`target/artifact-filetype-simplification/` on the repository volume. Project configuration
does not depend on a persisted absolute toolchain path.

| Pinned installed documentation/source, relative to its HTML root | SHA-256 |
| --- | --- |
| `std/fs/struct.FileType.html` | `ae3988afb7bb18c46b29cdaa7e4446ffc7de6d6842f34553f59d75bc1f41640e` |
| `std/fs/struct.Metadata.html` | `e21b302f98e64ce35a50d50ab38dcb2b4fef27caa4049d38a02a337acd1f6c9e` |
| `std/fs/fn.symlink_metadata.html` | `5a6682b0fc52456b68ad9b867e34f2e4d6012373064fee376bdecd398c6fb80f` |
| `src/std/fs.rs.html` | `e61ac152430012a90b36cba590ca32c50f92088095079993a848ab88dc24f734` |
| `src/std/sys/fs/unix.rs.html` | `ae881a5195b1c9756f3d336c16aeaa1d970cc50bc6c4885717573ea09ca78004` |

The HTML root is `share/doc/rust/html` beneath the runtime sysroot. This complete command
re-derives the source identities without an absolute installation path:

```bash
python3 -I -B - <<'PY'
from pathlib import Path
import hashlib, subprocess
root = Path(subprocess.check_output(
    ["rustc", "--print", "sysroot"], text=True).strip()) / "share/doc/rust/html"
for path in [
    "std/fs/struct.FileType.html",
    "std/fs/struct.Metadata.html",
    "std/fs/fn.symlink_metadata.html",
    "src/std/fs.rs.html",
    "src/std/sys/fs/unix.rs.html",
]:
    print(path, hashlib.sha256((root / path).read_bytes()).hexdigest())
PY
```

#### Complete predicate argument and falsified premises

Let `R` be `metadata.is_file()` and `S` be
`metadata.file_type().is_symlink()`. The API supplies `R => !S` on this same value.

| Legal metadata state | Original refusal | Simplified refusal |
| --- | --- | --- |
| not regular, not symlink | true | true |
| not regular, symlink | true | true |
| regular, not symlink | false | false |

If the record is nonregular, both conditions refuse. If it is regular, the API excludes the
symlink state and both conditions allow the next unchanged validation step. The removed
getter is pure, so its omission also preserves observable behavior. This covers every metadata
value admitted by the pinned API contract; filesystem examples are supplementary controls.

The following executable abstract-domain check includes the counterexample excluded by the
API, rather than silently dropping it:

```python
from itertools import product
for regular, symlink in product([False, True], repeat=2):
    original = (not regular) or symlink
    simplified = not regular
    if regular and symlink:
        assert original != simplified  # excluded by the FileType contract
    else:
        assert original == simplified
```

The other premises are independently falsified:

| False premise | Counterexample held |
| --- | --- |
| Arbitrary simultaneous file/link flags are allowed | regular and symlink makes the conditions disagree |
| The omitted getter may have side effects | a regular-file case with a recording getter has equal booleans but a different call trace |
| Predicates may use different metadata records | saved regular metadata and fresh symlink metadata make the mixed-record expression disagree |

For the purity control, a getter that appends to a trace and returns false is evaluated by
the original regular-file expression but omitted by the simplified expression. The actual
pinned getter source rules out that model. For the snapshot control, the real compiled probe
saves regular metadata, replaces the path with a relative symlink and obtains fresh metadata.
The saved value stays regular; the fresh value describes the symlink. The expression using
both records differs, while the expression on the saved record remains equal. Exact
production-source checks retain the single `symlink_metadata` binding and the same value in
both original predicates.

#### Actual before/after whole-component controls

The original and simplified complete artifact components are compiled separately with the
exact unchanged caller types and environment constructor. Each uses a real pinned compiler
to build an executable, then invalidates its source before invoking selection. Independent
Python/system hashes verify the retained before-image and final executable bytes.

| Actual fixture | Required behavior in both components |
| --- | --- |
| regular compiled executable | select the same relative path and independently measured hash; execution succeeds |
| relative link to owned foreign bytes | refuse as nonregular |
| dangling relative link | refuse as nonregular |
| directory | refuse as nonregular |
| named pipe | refuse without opening it |
| local Unix-domain socket | refuse as nonregular |
| saved and fresh metadata around path replacement | stable saved record; mixed-record premise counterexample |

Every negative fixture recovers to its original compiled regular file and valid selection.
No pipe or socket remains after completion. The whole-component probes supplement the
all-domain API argument; they are not an equivalent verdict derived from a finite kind grid.

#### Source mutation retirement and fresh protection control

The only maintained artifact-source change is the condition above. Existing semantic test bodies,
the caller, producer/controller code, finite pipe fixture and equivalence catalogs remain exact.
The external absolute-cwd proof changes only its artifact-prefix binding after revalidation.

| Historical predicate edit | Current disposition |
| --- | --- |
| `L99.logic.77`, change predicate disjunction to conjunction | old predicate edit retired by the simplification |
| `L99.logic.78`, keep only explicit symlink term | old predicate edit retired |
| `L99.logic.79`, keep only the regular-file refusal | now the proved production simplification; old edit retired |

No retired identity is counted as a current kill or equivalent. Source reconciliation uses
span, operator, full original and replacement text, rebasing the removed term in enclosing
source contexts. This maps current edit syntax, not outcomes: no old verdict transfers merely
because its edit maps to a new identity.

`2026-10-19`: discovery changes from 141 raw mutations to 138; every current
identity maps exactly once, while the retired predicate edits have no current candidate.
The unchanged crate-name membership edit maps from `L65.contains.116` to `L65.contains.113`
and remains separately owned by the next leaf, with no inherited K/E classification.

The fresh current protection control deliberately skips the remaining regular-file judgment:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L99.cond.78
```

`2026-10-19`: that current edit finishes killed against all 196 unchanged semantic trust parents,
strict exit `0`. Both existing every-position relative-link and finite pipe parents are actual
killing witnesses. All 1129 original/copied regular inputs restore with independent
Python/system hashes and modes; all 38 direct bindings, 37 public producer pins and
the pinned toolchain bindings agree. Invented-parent, missing-result, changed-outcome,
changed-exit and duplicate-result receipts refuse. The diagnostic leaf seals exactly;
all 311 prior task archives remain byte-exact with independent population checks.
This is a current protection-control result, not a complete artifact adequacy checkpoint.

#### Calibration observations

Identity calibration: an initial text-only reconciliation key refused its uniqueness assertion because equal edits occur at different spans. The corrected key includes source span/operator/full text; every current edit maps once and only the removed predicate edits retire.

Calibration fix: initial whole-component socket witness exceeded the OS socket-address path bound before selection; retained failure in `target/artifact-filetype-simplification/socket-path-before-fix.*`. Bind its relative basename from the same owned directory, restoring the previous working directory before asserting; no verdict from the failed probe. The default sandbox then refused local socket bind with `EPERM`; the approved same probe succeeds. Installed toolchain documentation/source reads remain read-only.

The failed calibration attempts produce no selector or equivalence classification. The
corrected whole-component probes and identity reconciliation pass before publication.

Held native tree `be55e84050d599d3ebdb4424b50a7accde3df8b5` precedes completion documentation.
Whole artifact file SHA-256 `f3c8b408ec1733cdf5828123ad5e1f6db1d205995fffe30f0e9bc1acfd30d673`; production-prefix SHA-256
`9365cd07c990d1ef4295b8e905ac1f3baa4754272498904f6b88e1c982b8ef8c` replaces the previous prefix
`f51d00153db5f9be189efbe73ddb3ebd468cd1e48de93882d4c6e90406a2faa2`. Supplemental premise script SHA-256
`db266bee6b99f42c1dea4afeafa1244db4c6d320fcc71bd5e811baca53a529fb` and audit SHA-256
`b26fc049ad7de7d2c88c5851da4c719f7fbcc46482f6139a96e27a02a5ceef1d` bind the pinned sources, false premises,
before/after components, source mapping, restored fixtures, inputs and archive census.
Supplemental programs/observations are untracked in `target/artifact-filetype-simplification/`;
the proof and reproduction commands above are retained here, and the existing tracked
parents/native command reproduce the operational protections. Final staged focused/book/commit
checks complete this leaf. Required crate-name minimum membership is next; the complete final
artifact replay, caller work and global adequacy remain separately owned and open.

#### Absolute-cwd proof revalidated after the source change

The initial final-focused run caught an existing source binding that had not been revalidated.
The absolute-cwd proof guard correctly refused the changed artifact production prefix. Its
failed run and earlier regularity evidence remain under
`target/artifact-filetype-simplification/before-cwd-revalidation/`. That earlier snapshot is
inspection evidence; the completed current replays below replace its final-tree qualification.

Revalidation reread the [absolute-prefix argument](#absolute-diagnostic-prefix-proofs), pinned
producer identities and actual CLI/gate/inventory/compile/query chain. The only artifact
behavioral edit removes a pure redundant predicate. It changes no root construction, output
join, query cwd, environment, compiler controller/observer or diagnostic-prefix framing.
The supported absolute-cwd premise and universal slash/physical-line-boundary arguments
therefore still hold. Relative cwd remains outside these existing reasons.

Only the existing operator control's artifact-prefix digest changes. All of its case bodies,
altered-premise arms, compiled helpers, real Cargo-root/relative-manifest/alias control, wider
witnesses and wrong oracles remain byte-exact. The complete targeted control passes before
fresh native replay. The precise equivalent reason records also remain byte-exact.

| Revalidated guarded slice | SHA-256 |
| --- | --- |
| trust production | `036f650dc082d9d43956865ef67fb16f761c4b3ede54c0567f8936733322bef4` |
| compiler controller | `634034fc7f85d67cb843251b15b2065ba229aee6692c6b68987c717383915b70` |
| compiler observer | `c625e10018af3a1b4fd8c96ece4b21b5afdba53cd9b77bd8c619ab6a2037743f` |
| artifact binding | `9365cd07c990d1ef4295b8e905ac1f3baa4754272498904f6b88e1c982b8ef8c` |
| base environment | `8b6fa81a2d715c2e42f8ab75a02b6e0c037c97d2f55e71c44c21e4a5516ee74c` |
| gate production | `8ad36a27c5e585660af256b59050f4393b1fa8601f591bd59de9ba2ffbfb7961` |
| repository root | `84ec4298740c835ae19ed9f2cff11f0f985cd59a4f3c5c205e9abd47805b6d77` |
| CLI entry | `3bf4d646e31f96b58fe18697db897614d1e1aac4e11b585e0d02cca641b4ab62` |

`2026-10-19`: the permanent targeted proof control passes with all 8 guarded source
bindings and 7 altered-source rejection arms. The source/caller review precedes the
single digest update; it is not a blind refresh to silence the guard. The ordinary final
focused tier also runs the complete operator suite before commit.

| Earlier absolute-prefix identity | Current exact identity | Fresh result |
| --- | --- | --- |
| `L690.cond.244` | `L590.cond.168` | existing precise E retained |
| `L717.cmp.257` | `L617.cmp.181` | existing precise E retained |

```bash
python3 -I -B xtask/mutation_sweep_tests.py "$PWD" \
  IteratorTests.test_cargo_absolute_prefix_edits_preserve_line_selection -v
bash scripts/mutation_sweep.sh xtask/src/trust.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --equivalents xtask/trust.equivalents \
  --only L590.cond.168 --only L617.cmp.181
```

`2026-10-19`: the refreshed current regularity replay finishes 1K and the existing absolute-prefix
replay finishes 2E, each against all 196 current semantic parents with strict exit `0`.
Both use exactly the same held tree `be55e84050d599d3ebdb4424b50a7accde3df8b5` and identical
indexed input snapshots. All 1129 original/copied regular inputs restore after each replay;
all direct, producer and pinned toolchain bindings agree. The regularity audit rejects its
invented-parent/missing/outcome/exit/duplicate controls. The equivalence audit separately
rejects changed exit, missing result, duplicate result, changed replacement, unlisted E and
invented detail. No earlier or unrelated equivalent verdict is projected from this selection.

Final audit SHA-256 `b26fc049ad7de7d2c88c5851da4c719f7fbcc46482f6139a96e27a02a5ceef1d` binds both current native
receipts, exact one-digest proof-test change, source/premise preservation and restored fixtures.
No new FileType equivalent classification is assigned. The formal simplification, retired
predicate identities and independently revalidated existing prefix reasons remain distinct.
Final staged focused/book/commit checks complete this leaf; complete artifact/caller/global
adequacy remains separately owned and open.

### Required crate-name minimum membership

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.14`: the remaining required-name membership
edit is assessed on the constructor's closed source domain. The exact current edit is
`L65.contains.113`, mapped from the earlier `L65.contains.116`:

```diff
-if !seen.contains("--crate-name") {
+if !((seen).first().copied() == Some("--crate-name")) {
```

The native replacement deliberately parenthesizes the receiver; the permanent control binds
that literal edit rather than an abbreviated spelling.

#### Closed local state and ordering contracts

The source starts with an empty local `std::collections::BTreeSet<&str>`. Every possible
insertion is accounted for:

| Insertion site | Possible inserted values |
| --- | --- |
| exact naming-option arm, with `--crate-type` excluded | `--crate-name`, `--out-dir`, `--target` |
| exact test flag | `--test` |
| qualifying code-generation option | literal `extra-filename` |

Unknown arguments do not insert a key. Unsupported joined forms and malformed or duplicate
options can return earlier, identically in the original and edit. Processing a value as
another token still cannot widen these insertion cases. The set never escapes for a caller
to modify; its borrowed string keys remain immutable during the call. The actual typed
constructor and its production caller/imports are bound by the source guard.

Let `KEYS` be this complete key set. Induction over the loop starts with the empty subset
and preserves `seen` as a subset of `KEYS` after every insertion. This statement covers
arbitrary argument-vector lengths and values; a finite selection of argument examples is
not its basis.

The [ordered-set API](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#method.first)
gives `first` as the minimum present key.
[Membership](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#method.contains)
tests equality with the borrowed key under matching ordering. The actual keys are ordinary
immutable `str` values with built-in comparisons. The pinned string implementation compares
their byte sequences for ordering and equality
([toolchain identity](ledger.md#rust-toolchain)). Every other possible key sorts after
`--crate-name`: the other option names follow it, while `extra-filename` follows names
beginning with dashes. The compiled control independently checks those literal comparisons.

#### All-domain predicate argument

If `--crate-name` is present, it is the minimum of the closed set, so `first().copied()`
is `Some("--crate-name")`. If it is absent, the first key is another present member or the set
is empty; neither is that value. Thus membership and this exact first-element comparison
have the same boolean on every reachable local state, including the empty state.

The lookups use the same immutable set with primitive string comparisons and introduce no
user comparison callbacks or changed state. All earlier parsing and returns, and all code
after the required-name judgment, are identical. Therefore the complete constructor returns
the same query/output pair or the same error for every input in its typed, source-guarded
domain. This proves only the exact first/copied edit under the closed-key contract.

#### Complete abstract state and wider-source falsifiers

The permanent operator control compiles the actual constructor and the exact discovered
edit. It checks the complete powerset of the closed keys, not merely reached example states,
and compares original/edited helper results over alternative option-group orders. A literal
valid query supplies an independent output oracle.

The control also inserts `--aaa` immediately before the required-name judgment in separate
copies of each actual helper. With a valid observed crate name, the original still returns
the literal query while the first-element edit falsely reports a missing crate name.
A matching wider set contains the required key while its minimum is `--aaa`. This is the
concrete counterexample to dropping the closed-key premise.

Source guards reject a smaller inserted key, a changed collection type and a changed
toolchain pin. Deliberately wrong equal/different result oracles also fail. Those controls
run before the precise-reason requirement; the initial run reaches only that final missing
record guard after the source and compiled proof controls succeed.

The proof gate lives in `ContainsTests` within the existing operator suite. Ordinary focused
checks run it. The native semantic trust filters exclude that suite, so its source hashes
cannot become artificial semantic killing witnesses. The semantic parent cohort stays
unchanged; operational protection and formal reason validity are measured separately.

The source production prefix is
`9365cd07c990d1ef4295b8e905ac1f3baa4754272498904f6b88e1c982b8ef8c`.
The helper body SHA-256 is
`37d4ebcdc401547ff50834df1c11f4fe17046b5a15dc8ec15d49afa7cb7a0c1c`.
The exact pinned toolchain-file SHA-256 is
`4706b7b407427c38a3b63860b2e131cacc9eb57569a6755a39e81756cca676cd`.
Changes to guarded source/types/callers, insertion closure, ordering contracts or the pin
require revalidation before retaining a reason.

`2026-10-19`: the compiled proof holds all 32 closed key subsets, 96 paired actual-helper
comparisons, literal query behavior and the wider inserted-key difference. This is pre-reason
proof evidence; native equivalence is completed below; final operator/focused checks remain required before commit.
Supplemental pinned documentation/source copies and calibration records live under
`target/artifact-crate-name-membership/`. The tracked operator control and this argument
preserve the proof independently of those untracked observations.

#### Concrete constructor results

For this observed argument vector, both actual helpers return the same literal query and
separate output directory:

```text
observed:
  rustc source.rs --crate-name probe --out-dir outputs
query:
  ["--print=file-names", "-", "--crate-name", "probe"]
output directory:
  outputs
```

| Local membership state | Contains required name | First is required name |
| --- | --- | --- |
| empty | false | false |
| `--out-dir` only | false | false |
| `--crate-name` only | true | true |
| `--crate-name` and `--test` | true | true |
| wider `--aaa` and `--crate-name` | true | false; outside the guarded source domain |

The actual inserted-key helper control uses valid crate-name/output arguments. Its original
returns the query above, while its edited version returns `missing observed --crate-name`.
The normal source guard rejects that wider source before it can qualify a reason.

#### Pinned source identities and permanent reproduction

The source/API review uses the documentation and public implementation installed with the
pinned toolchain. These read-only external dependency paths derive at runtime from
`rustc --print sysroot`. The local evidence copies are repository-relative; builds do not
depend on a persisted absolute toolchain installation path.

| Installed document/source relative to `share/doc/rust/html` | SHA-256 |
| --- | --- |
| `std/collections/struct.BTreeSet.html` | `29479dab12759067e827b8aecb41cf599a67a7e5c8b4798506c8366b8ccd1a2a` |
| `std/primitive.str.html` | `0a08f21dd7dffe6cad11fffe94061ade54560f1f1a8bd781011496686206afb4` |
| `std/cmp/trait.Ord.html` | `64a2217080830334a3139f756906b2fb87faf49c39b10db053a37c4d3b9b6aac` |
| `src/alloc/collections/btree/set.rs.html` | `816ce6b51e99a05652ace5eb49a2cf0ed03cf8a5e1847c553a2f9cfbc78c4969` |
| `src/core/str/traits.rs.html` | `a787be8d697865a7f30d98ac01ccac82cac4fff66a17535c93784dfbb10c2dd7` |

The same toolchain-root derivation shown in the preceding file-kind proof reproduces these
identities using the relative paths above. The permanent control is independently runnable:

```bash
python3 -I -B xtask/mutation_sweep_tests.py "$PWD" \
  ContainsTests.test_required_crate_name_first_membership_has_a_closed_key_domain -v
```

Its exact source guards preserve the constructor, standard collection/string types and
production caller. A pin change requires source/API revalidation. Earlier operator cases,
all production source and every semantic trust parent remain byte-exact.

#### Precise reason and actual native result

Only the exact first/copied pair gains a record in `xtask/trust_artifact.equivalents`.
The original constructor retains its clear membership expression. No last-element or
arbitrary-set reason is added.

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --equivalents xtask/trust_artifact.equivalents --only L65.contains.113
```

`2026-10-19`: the selected edit finishes as 1E against all 196 unchanged semantic parents,
strict exit `0`, after the guarded proof and precise reason. The original copied baseline
passes. No selected kill, compiler refusal, timeout or unjudged result occurs; source-premise
hash guards are not in this cohort. All 1131 original/copied regular inputs restore with
independent Python/system hashes and modes; all 39 direct bindings, 37 public producer
pins and 5 pinned toolchain source bindings agree. Changed-exit, missing-result,
duplicate-result, changed-replacement, unlisted-E and invented-detail receipts refuse.
The file-kind leaf seals exactly; all 312 prior task archives remain byte-exact, with
independent Git/index/filesystem counts and false-count controls.

#### Calibration corrections and scope

Calibration: the pinned str equality source spells `impl const PartialEq`, and the exact native predicate parenthesizes its receiver `(seen)`. Narrower initial literal lookups refused before proof/result publication; source-domain notes and `predicate-shape-before-fix.*` retain the corrections.

The proof-book append is verified against the complete committed chapter prefix byte for
byte. An overbroad temporary section-marker expansion was corrected before reason/result
publication by reconstructing that prefix exactly; only this new proof section remains.
These calibration refusals/repairs are not semantic or equivalence evidence.

Held native tree `6217ab18486beced79780fe73bd78b878b11f1a4` precedes completion documentation.
Artifact file SHA-256 `f3c8b408ec1733cdf5828123ad5e1f6db1d205995fffe30f0e9bc1acfd30d673` is unchanged;
precise reason-file SHA-256 `591981f27cd23cb1c40f63c71554ef01208ed1c07a191d63e66ed82d66678435` and supplemental
audit SHA-256 `91b6a03c3a700d385beacb797a0bd98e1f06c971d1397a61fdf8467169eb2156` bind the exact relation,
preserved source/parents/book prefix, proof controls, complete input restoration and rejected
receipts. Supplemental programs/observations remain untracked under
`target/artifact-crate-name-membership/`; the tracked permanent control, precise reason,
source proof and native command preserve re-runnability.

Final staged focused/book/commit checks run the complete operator suite before this leaf
commits. The complete final artifact checkpoint is next: fresh full discovery and reasons,
all current semantic parents, compiler-refusal assessment, initial/retired identity reconciliation,
restoration and integration. This selected result does not close artifact/caller/global adequacy.

The final whitespace check required removing the new catalog's extra end separator.
The native reader returns identical records for both endings; the permanent proof control
passes on the normalized file. The initial layout/receipt is retained under
`target/artifact-crate-name-membership/before-eof-normalization/`. The fresh native replay
binds the final catalog bytes, again finishing as one equivalent with complete restoration.
Final focused/commit checks remain required.

### Complete final artifact checkpoint

`2026-10-19`, leaf `.15.2.1.3.3.3.2.8.15`: the fresh complete artifact run finishes with
**126 named kills, 1 precisely guarded equivalent edit and 11 compiler
refusals**, strict exit `0`. There are no survivors, timeouts or unjudged edits.
This closes the bounded artifact resolution population after the committed controls and
source proof. Caller suffix/remaining caller populations, final source/reason revalidation and
independent review remain open before F30 or global adequacy.

#### Exact current source, reason and full cohort

The complete current artifact file is `xtask/src/trust_artifact.rs`, SHA-256
`f3c8b408ec1733cdf5828123ad5e1f6db1d205995fffe30f0e9bc1acfd30d673`. Its production prefix has SHA-256
`9365cd07c990d1ef4295b8e905ac1f3baa4754272498904f6b88e1c982b8ef8c`.
Fresh actual CLI discovery and maintained discovery agree on the entire zero-based
production range `[11,198]`, every span/operator/original/replacement and native order.

```bash
bash scripts/mutation_sweep.sh xtask/src/trust_artifact.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --equivalents xtask/trust_artifact.equivalents
```

`2026-10-19`: this command runs all 138 current edits against all 196 current semantic
parents after a passing unmodified baseline. It has no selected-ID filter. The copied parent
names and order agree exactly with the fresh original population. The normal operator proof
suite remains outside this semantic cohort, so source-hash guards cannot supply semantic
kills. Its permanent membership proof is required separately by normal verification.

The precise reason file SHA-256 is
`591981f27cd23cb1c40f63c71554ef01208ed1c07a191d63e66ed82d66678435`. It contains only the exact required-crate-name first/copied
predicate proved in the preceding section. The native reader binds the same original/edit
pair. The pure same-snapshot file-kind simplification adds no reason record.

The held native Git tree is `3d6e5613db5cc40d623dfba44ffd9bf79b8dc54e`. The unchanged
repository-volume sandbox profile has SHA-256
`2b3fbe199bddb4ba84c453a23334c7025308b06127ce3cc96f74b2a17680eedd`.
The held tree includes the owned checkpoint activation and membership-leaf seal; no source,
test, producer or reason changes occur during the replay.

#### Every final native result

`2026-10-19`: every current edit appears below in actual native order. Initial IDs/outcomes
refer only to the earlier source `3c42078e46da0424e4c47aa467cd5e4b6edf851af1a0d50c811f595979aaa5c5`
and its earlier parent cohort. Identity is reconciled by exact span, operator and edit text
under the single proved term removal; final outcomes are all freshly measured.
`K` is an actual finishing named failure, `E` the precise source-proved reason, and `U` a
compiler refusal. Each K row gives an actual witness alias and the total distinct failing
names; the full native receipt retains all names. Required regression witnesses are
independently checked below.

| Alias | Actual current semantic parent |
| --- | --- |
| F1 | `trust::artifact::tests::naming_query_excludes_source_codegen_and_environment_inputs` |
| F2 | `trust::artifact::tests::naming_query_preserves_test_and_refuses_duplicate_test_options` |
| F3 | `trust::artifact::tests::naming_query_requires_nonempty_nonflaglike_option_values` |
| F4 | `trust::artifact::tests::actual_empty_input_query_preserves_build_and_ignores_now_invalid_source` |
| F5 | `trust::tests::a_second_run_writes_the_same_bytes` |
| F6 | `trust::artifact::tests::naming_query_forwards_only_the_exact_extra_filename_option` |
| F7 | `trust::artifact::tests::naming_query_refuses_each_joined_option_but_allows_source_lookalikes` |
| F8 | `trust::artifact::tests::artifact_names_refuse_missing_foreign_linked_and_ambiguous_outputs` |
| F9 | `trust::artifact::tests::output_diagnostics_name_the_first_failure_in_the_reported_order` |
| F10 | `trust::artifact::tests::every_reported_output_is_read_and_the_rlib_can_be_in_each_position` |
| F11 | `trust::artifact::tests::linked_outputs_are_refused_at_every_reported_position` |
| F12 | `trust::artifact::tests::malformed_basenames_are_refused_before_their_output_lookup` |
| F13 | `trust::artifact::tests::every_output_name_is_unique_including_nonselected_outputs` |
| F14 | `trust::adequacy_tests::every_program_target_position_is_refused_before_a_foreign_role_build` |
| F15 | `trust::adequacy_tests::reader_role_tests::the_harness_allows_both_pair_roles_and_refuses_each_foreign_generator_kind` |
| F16 | `trust::artifact::tests::source_selection_preserves_ordinary_and_canonical_prefixes_under_a_tree_alias` |
| F17 | `trust::artifact::tests::metadata_roots_share_one_source_and_refuse_distinct_compiled_sources` |
| F18 | `trust::artifact::tests::naming_query_honors_the_explicit_toolchain_environment` |

| Current ID | Earlier ID / outcome | Final | Actual witness or refusal |
| --- | --- | --- | --- |
| `L16.iter.0` | `L16.iter.0` / K | K | F1; 119 distinct named killing parents |
| `L16.iter.1` | `L16.iter.1` / K | K | F1; 119 distinct named killing parents |
| `L16.iter.2` | `L16.iter.2` / K | K | F1; 119 distinct named killing parents |
| `L16.guard.3` | `L16.guard.3` / K | K | F1; 1 distinct named killing parent |
| `L16.guard.4` | `L16.guard.4` / K | K | F1; 1 distinct named killing parent |
| `L16.guard.5` | `L16.guard.5` / K | K | F1; 1 distinct named killing parent |
| `L16.guard.6` | `L16.guard.6` / K | K | F1; 119 distinct named killing parents |
| `L16.guard.7` | `L16.guard.7` / K | K | F1; 119 distinct named killing parents |
| `L16.guard.8` | `L16.guard.8` / K | K | F1; 119 distinct named killing parents |
| `L16.guard.9` | `L16.guard.9` / S | K | F2; 1 distinct named killing parent |
| `L16.guard.10` | `L16.guard.10` / S | K | F2; 1 distinct named killing parent |
| `L16.guard.11` | `L16.guard.11` / S | K | F2; 1 distinct named killing parent |
| `L16.guard.12` | `L16.guard.12` / K | K | F1; 116 distinct named killing parents |
| `L16.guard.13` | `L16.guard.13` / K | K | F1; 116 distinct named killing parents |
| `L16.guard.14` | `L16.guard.14` / K | K | F1; 116 distinct named killing parents |
| `L16.guard.15` | `L16.guard.15` / K | K | F1; 1 distinct named killing parent |
| `L16.guard.16` | `L16.guard.16` / K | K | F1; 1 distinct named killing parent |
| `L16.guard.17` | `L16.guard.17` / K | K | F1; 1 distinct named killing parent |
| `L20.int.18` | `L20.int.18` / K | K | F1; 119 distinct named killing parents |
| `L20.int.19` | `L20.int.19` / K | K | F1; 119 distinct named killing parents |
| `L21.logic.20` | `L21.logic.20` / S | K | F3; 1 distinct named killing parent |
| `L21.not.21` | `L21.not.21` / K | K | F1; 119 distinct named killing parents |
| `L21.not.22` | `L21.not.22` / K | K | F1; 119 distinct named killing parents |
| `L21.dir.23` | `L21.dir.23` / K | K | F4; 114 distinct named killing parents |
| `L23.eq.24` | `L23.eq.24` / K | K | F1; 119 distinct named killing parents |
| `L23.logic.25` | `L23.logic.25` / K | K | F1; 119 distinct named killing parents |
| `L23.logic.26` | `L23.logic.26` / K | K | F1; 2 distinct named killing parents |
| `L23.logic.27` | `L23.logic.27` / K | K | F1; 119 distinct named killing parents |
| `L23.cond.28` | `L23.cond.28` / K | K | F1; 119 distinct named killing parents |
| `L23.cond.29` | `L23.cond.29` / K | K | F1; 119 distinct named killing parents |
| `L23.not.30` | `L23.not.30` / K | K | F1; 119 distinct named killing parents |
| `L26.eq.31` | `L26.eq.31` / K | K | F1; 119 distinct named killing parents |
| `L26.cond.32` | `L26.cond.32` / K | K | F1; 119 distinct named killing parents |
| `L26.cond.33` | `L26.cond.33` / K | K | F1; 119 distinct named killing parents |
| `L27.stmt.34` | `L27.stmt.34` / K | K | F1; 119 distinct named killing parents |
| `L29.stmt.35` | `L29.stmt.35` / K | K | F1; 119 distinct named killing parents |
| `L33.cond.36` | `L33.cond.36` / K | K | F5; 14 distinct named killing parents |
| `L33.cond.37` | `L33.cond.37` / S | K | F2; 1 distinct named killing parent |
| `L33.not.38` | `L33.not.38` / K | K | F5; 14 distinct named killing parents |
| `L36.stmt.39` | `L36.stmt.39` / S | K | F2; 1 distinct named killing parent |
| `L40.int.40` | `L40.int.40` / K | K | F1; 116 distinct named killing parents |
| `L40.int.41` | `L40.int.41` / K | K | F1; 116 distinct named killing parents |
| `L42.cond.42` | `L42.cond.42` / K | K | F1; 112 distinct named killing parents |
| `L42.cond.43` | `L42.cond.43` / K | K | F1; 116 distinct named killing parents |
| `L42.dir.44` | `L42.dir.44` / S | K | F6; 1 distinct named killing parent |
| `L42.lit.45` | `L42.lit.45` / S | K | F6; 1 distinct named killing parent |
| `L42.lit.46` | `L42.lit.46` / K | K | F1; 116 distinct named killing parents |
| `L43.cond.47` | `L43.cond.47` / K | K | F1; 116 distinct named killing parents |
| `L43.cond.48` | `L43.cond.48` / K | K | F1; 1 distinct named killing parent |
| `L43.not.49` | `L43.not.49` / K | K | F1; 116 distinct named killing parents |
| `L46.stmt.50` | `L46.stmt.50` / K | K | F1; 116 distinct named killing parents |
| `L58.dir.51` | `L58.dir.51` / S | K | F7; 1 distinct named killing parent |
| `L65.cond.52` | `L65.cond.52` / K | K | F1; 119 distinct named killing parents |
| `L65.cond.53` | `L65.cond.53` / K | K | F1; 1 distinct named killing parent |
| `L65.not.54` | `L65.not.54` / K | K | F1; 119 distinct named killing parents |
| `L65.lit.55` | `L65.lit.55` / K | K | F1; 119 distinct named killing parents |
| `L65.lit.56` | `L65.lit.56` / K | K | F1; 119 distinct named killing parents |
| `L84.cond.57` | `L84.cond.57` / K | K | F4; 121 distinct named killing parents |
| `L84.cond.58` | `L84.cond.58` / K | K | F8; 1 distinct named killing parent |
| `L84.not.59` | `L84.not.59` / K | K | F4; 121 distinct named killing parents |
| `L84.dir.60` | `L84.dir.60` / U | U | error[E0599]: no method named contains found for struct PathBuf in the current scope |
| `L89.dir.61` | `L89.dir.61` / S | K | F9; 1 distinct named killing parent |
| `L89.iter.62` | `L89.iter.62` / K | K | F8; 7 distinct named killing parents |
| `L89.iter.63` | `L89.iter.63` / K | K | F8; 8 distinct named killing parents |
| `L89.iter.64` | `L89.iter.64` / S | K | F10; 6 distinct named killing parents |
| `L89.guard.65` | `L89.guard.65` / S | K | F11; 2 distinct named killing parents |
| `L89.guard.66` | `L89.guard.66` / S | K | F11; 2 distinct named killing parents |
| `L89.guard.67` | `L89.guard.67` / S | K | F11; 2 distinct named killing parents |
| `L89.guard.68` | `L89.guard.68` / K | K | F8; 7 distinct named killing parents |
| `L89.guard.69` | `L89.guard.69` / K | K | F8; 8 distinct named killing parents |
| `L89.guard.70` | `L89.guard.70` / S | K | F10; 6 distinct named killing parents |
| `L91.logic.71` | `L91.logic.71` / S | K | F12; 1 distinct named killing parent |
| `L91.alt.72` | `L91.alt.72` / U | U | error: unexpected closing delimiter: } |
| `L91.alt.73` | `L91.alt.73` / U | U | error: mismatched closing delimiter: } |
| `L92.logic.74` | `L92.logic.74` / S | K | F12; 1 distinct named killing parent |
| `L93.logic.75` | `L93.logic.75` / S | K | F13; 2 distinct named killing parents |
| `L93.not.76` | `L93.not.76` / K | K | F4; 121 distinct named killing parents |
| `L99.cond.77` | `L99.cond.80` / K | K | F4; 121 distinct named killing parents |
| `L99.cond.78` | `L99.cond.81` / K | K | F8; 3 distinct named killing parents |
| `L99.not.79` | `L99.not.82` / K | K | F4; 121 distinct named killing parents |
| `L105.logic.80` | `L105.logic.83` / K | K | F4; 121 distinct named killing parents |
| `L105.logic.81` | `L105.logic.84` / K | K | F4; 113 distinct named killing parents |
| `L105.logic.82` | `L105.logic.85` / K | K | F8; 118 distinct named killing parents |
| `L105.cond.83` | `L105.cond.86` / K | K | F8; 8 distinct named killing parents |
| `L105.cond.84` | `L105.cond.87` / K | K | F4; 121 distinct named killing parents |
| `L105.not.85` | `L105.not.88` / K | K | F4; 120 distinct named killing parents |
| `L135.eq.86` | `L135.eq.89` / K | K | F4; 114 distinct named killing parents |
| `L136.logic.87` | `L136.logic.90` / K | K | F5; 49 distinct named killing parents |
| `L137.eq.88` | `L137.eq.91` / K | K | F4; 97 distinct named killing parents |
| `L137.eq.89` | `L137.eq.92` / K | K | F4; 99 distinct named killing parents |
| `L137.logic.90` | `L137.logic.93` / K | K | F14; 4 distinct named killing parents |
| `L138.eq.91` | `L138.eq.94` / K | K | F5; 13 distinct named killing parents |
| `L138.eq.92` | `L138.eq.95` / K | K | F5; 13 distinct named killing parents |
| `L138.logic.93` | `L138.logic.96` / K | K | F15; 4 distinct named killing parents |
| `L155.eq.94` | `L155.eq.97` / K | K | F4; 114 distinct named killing parents |
| `L156.logic.95` | `L156.logic.98` / K | K | F4; 18 distinct named killing parents |
| `L158.eq.96` | `L158.eq.99` / K | K | F4; 18 distinct named killing parents |
| `L159.eq.97` | `L159.eq.100` / K | K | F4; 18 distinct named killing parents |
| `L159.logic.98` | `L159.logic.101` / K | K | F4; 114 distinct named killing parents |
| `L160.eq.99` | `L160.eq.102` / K | K | F4; 18 distinct named killing parents |
| `L160.logic.100` | `L160.logic.103` / S | K | F16; 1 distinct named killing parent |
| `L179.cond.101` | `L179.cond.104` / K | K | F4; 114 distinct named killing parents |
| `L179.cond.102` | `L179.cond.105` / K | K | F4; 2 distinct named killing parents |
| `L179.not.103` | `L179.not.106` / K | K | F4; 114 distinct named killing parents |
| `L195.eq.104` | `L195.eq.107` / K | K | F4; 113 distinct named killing parents |
| `L16.chain.105` | `L16.chain.108` / K | K | F1; 119 distinct named killing parents |
| `L16.chain.106` | `L16.chain.109` / K | K | F1; 119 distinct named killing parents |
| `L21.chain.107` | `L21.chain.110` / U | U | error[E0061]: this method takes 0 arguments but 1 argument was supplied |
| `L21.chain.108` | `L21.chain.111` / U | U | error[E0599]: no method named rev found for enum Option<&String> in the current scope |
| `L21.chain.109` | `L21.chain.112` / S | K | F3; 1 distinct named killing parent |
| `L57.chain.110` | `L57.chain.113` / S | K | F7; 1 distinct named killing parent |
| `L57.chain.111` | `L57.chain.114` / K | K | F1; 2 distinct named killing parents |
| `L65.contains.112` | `L65.contains.115` / U | U | error[E0308]: mismatched types |
| `L65.contains.113` | `L65.contains.116` / S | E | exact closed-key membership proof and reason above |
| `L65.contains.114` | `L65.contains.117` / U | U | error[E0308]: mismatched types |
| `L65.contains.115` | `L65.contains.118` / K | K | F1; 119 distinct named killing parents |
| `L92.contains.116` | `L92.contains.119` / U | U | error[E0599]: no method named first found for reference &str in the current scope |
| `L92.contains.117` | `L92.contains.120` / U | U | error[E0599]: no method named first found for reference &str in the current scope |
| `L92.contains.118` | `L92.contains.121` / U | U | error[E0599]: &str is not an iterator |
| `L92.contains.119` | `L92.contains.122` / U | U | error[E0599]: &str is not an iterator |
| `L133.chain.120` | `L133.chain.123` / K | K | F5; 111 distinct named killing parents |
| `L133.chain.121` | `L133.chain.124` / K | K | F5; 110 distinct named killing parents |
| `L134.chain.122` | `L134.chain.125` / S | K | F17; 1 distinct named killing parent |
| `L134.chain.123` | `L134.chain.126` / S | K | F17; 1 distinct named killing parent |
| `L134.chain.124` | `L134.chain.127` / K | K | F5; 110 distinct named killing parents |
| `L144.chain.125` | `L144.chain.128` / S | K | F17; 1 distinct named killing parent |
| `L144.chain.126` | `L144.chain.129` / K | K | F4; 2 distinct named killing parents |
| `L153.chain.127` | `L153.chain.130` / K | K | F4; 41 distinct named killing parents |
| `L153.chain.128` | `L153.chain.131` / K | K | F4; 44 distinct named killing parents |
| `L154.chain.129` | `L154.chain.132` / K | K | F4; 1 distinct named killing parent |
| `L154.chain.130` | `L154.chain.133` / K | K | F4; 1 distinct named killing parent |
| `L154.chain.131` | `L154.chain.134` / K | K | F4; 50 distinct named killing parents |
| `L156.chain.132` | `L156.chain.135` / K | K | F4; 114 distinct named killing parents |
| `L156.chain.133` | `L156.chain.136` / K | K | F4; 114 distinct named killing parents |
| `L175.chain.134` | `L175.chain.137` / S | K | F18; 1 distinct named killing parent |
| `L175.chain.135` | `L175.chain.138` / K | K | F4; 114 distinct named killing parents |
| `L21.clogic.136` | `L21.clogic.139` / S | K | F3; 1 distinct named killing parent |
| `L21.clogic.137` | `L21.clogic.140` / S | K | F3; 1 distinct named killing parent |


#### Initial survivors, removed predicates and intended witnesses

`2026-10-19`: every earlier survivor is reconciled to a freshly measured current outcome
or an explicitly source-proved removed predicate. No old selected result is projected onto
this full run. The current regularity control also preserves the legacy relative-link parent
and both every-position link/finite-pipe parents; it was already a kill after the fixture
repair and remains a separately checked regression.

| Earlier survivor | Exact current ID or retired source | Fresh result and intended evidence |
| --- | --- | --- |
| `L16.guard.9` | `L16.guard.9` | K; `trust::artifact::tests::naming_query_preserves_test_and_refuses_duplicate_test_options` |
| `L16.guard.10` | `L16.guard.10` | K; `trust::artifact::tests::naming_query_preserves_test_and_refuses_duplicate_test_options` |
| `L16.guard.11` | `L16.guard.11` | K; `trust::artifact::tests::naming_query_preserves_test_and_refuses_duplicate_test_options` |
| `L21.logic.20` | `L21.logic.20` | K; `trust::artifact::tests::naming_query_requires_nonempty_nonflaglike_option_values` |
| `L33.cond.37` | `L33.cond.37` | K; `trust::artifact::tests::naming_query_preserves_test_and_refuses_duplicate_test_options` |
| `L36.stmt.39` | `L36.stmt.39` | K; `trust::artifact::tests::naming_query_preserves_test_and_refuses_duplicate_test_options` |
| `L42.dir.44` | `L42.dir.44` | K; `trust::artifact::tests::naming_query_forwards_only_the_exact_extra_filename_option` |
| `L42.lit.45` | `L42.lit.45` | K; `trust::artifact::tests::naming_query_forwards_only_the_exact_extra_filename_option` |
| `L58.dir.51` | `L58.dir.51` | K; `trust::artifact::tests::naming_query_refuses_each_joined_option_but_allows_source_lookalikes` |
| `L89.dir.61` | `L89.dir.61` | K; `trust::artifact::tests::output_diagnostics_name_the_first_failure_in_the_reported_order` |
| `L89.iter.64` | `L89.iter.64` | K; `trust::artifact::tests::every_reported_output_is_read_and_the_rlib_can_be_in_each_position` |
| `L89.guard.65` | `L89.guard.65` | K; `trust::artifact::tests::linked_outputs_are_refused_at_every_reported_position` |
| `L89.guard.66` | `L89.guard.66` | K; `trust::artifact::tests::linked_outputs_are_refused_at_every_reported_position` |
| `L89.guard.67` | `L89.guard.67` | K; `trust::artifact::tests::linked_outputs_are_refused_at_every_reported_position` |
| `L89.guard.70` | `L89.guard.70` | K; `trust::artifact::tests::every_reported_output_is_read_and_the_rlib_can_be_in_each_position` |
| `L91.logic.71` | `L91.logic.71` | K; `trust::artifact::tests::malformed_basenames_are_refused_before_their_output_lookup` |
| `L92.logic.74` | `L92.logic.74` | K; `trust::artifact::tests::malformed_basenames_are_refused_before_their_output_lookup` |
| `L93.logic.75` | `L93.logic.75` | K; `trust::artifact::tests::every_output_name_is_unique_including_nonselected_outputs` |
| `L99.logic.77` | retired with the redundant same-metadata symlink term | source proof and one-term production diff; no K/E inherited |
| `L99.logic.78` | retired with the redundant same-metadata symlink term | source proof and one-term production diff; no K/E inherited |
| `L99.logic.79` | retired with the redundant same-metadata symlink term | source proof and one-term production diff; no K/E inherited |
| `L160.logic.103` | `L160.logic.100` | K; `trust::artifact::tests::source_selection_preserves_ordinary_and_canonical_prefixes_under_a_tree_alias` |
| `L21.chain.112` | `L21.chain.109` | K; `trust::artifact::tests::naming_query_requires_nonempty_nonflaglike_option_values` |
| `L57.chain.113` | `L57.chain.110` | K; `trust::artifact::tests::naming_query_refuses_each_joined_option_but_allows_source_lookalikes` |
| `L65.contains.116` | `L65.contains.113` | E; closed-key all-domain proof; exact native E |
| `L134.chain.125` | `L134.chain.122` | K; `trust::artifact::tests::metadata_roots_share_one_source_and_refuse_distinct_compiled_sources` |
| `L134.chain.126` | `L134.chain.123` | K; `trust::artifact::tests::metadata_roots_share_one_source_and_refuse_distinct_compiled_sources` |
| `L144.chain.128` | `L144.chain.125` | K; `trust::artifact::tests::metadata_roots_share_one_source_and_refuse_distinct_compiled_sources` |
| `L175.chain.137` | `L175.chain.134` | K; `trust::artifact::tests::naming_query_honors_the_explicit_toolchain_environment` |
| `L21.clogic.139` | `L21.clogic.136` | K; `trust::artifact::tests::naming_query_requires_nonempty_nonflaglike_option_values` |
| `L21.clogic.140` | `L21.clogic.137` | K; `trust::artifact::tests::naming_query_requires_nonempty_nonflaglike_option_values` |

The original production prefix becomes the current prefix after removing exactly
` || metadata.file_type().is_symlink()`, with no other production-byte change. The earlier
file-kind section proves the same immutable metadata snapshot, exclusivity and pure stored
getters, and includes falsified mixed snapshots. Thus the removed boolean-term edits have
no current candidate. Their earlier S outcomes remain historical; they are never counted as
new kills or equivalents.

`2026-10-19`: all 28 mapped earlier survivors meet their fresh required verdict.
The witness audit holds all 29 required rows, including the separate regularity regression,
all 30 intended named-witness obligations and the single precise E. Wrong required
outcome, removed required witness, changed refusal diagnostic, U-as-E and omitted-refusal
receipts all fail. The independent full-receipt audit also prevents invented parent names.

#### Every compiler refusal remains separate from coverage

The exact whole current component, with its exact caller `Target`, `Program`,
`TargetSources` and `Compilation` definitions plus the actual `Digest` dependency, compiles
in normal configuration before any edit. Each exact current compiler-rejected edit is then
compiled in its own component copy with `rustc --emit=metadata`. Every compiler rejection
matches the final native first diagnostic. This confirms production compilation failure,
rather than a broken test-only branch or a runtime behavioral witness.

| Current U | Actual type or syntax reason |
| --- | --- |
| `L84.dir.60` | PathBuf receiver; no contains method |
| `L91.alt.72` | Token-level alternative removal leaves an unexpected closing delimiter |
| `L91.alt.73` | Token-level alternative removal leaves a mismatched closing delimiter |
| `L21.chain.107` | Option<&String>::take takes no numeric argument; this is not Iterator::take |
| `L21.chain.108` | Option<&String> has no rev method; this is not an iterator chain |
| `L65.contains.112` | BTreeSet<&str>::first returns Option<&&str>, versus Some(&str) |
| `L65.contains.114` | BTreeSet<&str>::last returns Option<&&str>, versus Some(&str) |
| `L92.contains.116` | &str receiver has no first method |
| `L92.contains.117` | &str receiver has no first method; copied cannot repair the missing method |
| `L92.contains.118` | &str receiver has no last method or Iterator implementation |
| `L92.contains.119` | &str receiver has no last method or Iterator implementation; copied cannot repair it |


The set's `first`/`last` result is `Option<&&str>`; the two rejected comparisons try to build
`Some(&str)`. The separately proved first/copied edit removes the extra reference and is a
different, compiling candidate. The option-chain receiver remains `Option<&String>`, so an
iterator-shaped `take(1)` or `rev()` does not acquire iterator semantics. The basename
receiver is `&str`, and the directory receiver is `PathBuf`; their rejected methods are
bound to those actual source types. The alternative-token rewrites are malformed syntax.
These are tool/operator outcomes, never coverage, equivalence or proof of a missing test.

Exact caller-type text has SHA-256
`1aabfad19aab65116a32d4ee6c044d14911233dc21174d991f5590e5f16c3df4`.
The installed compiler identity is the pinned toolchain
([Rust toolchain ledger](ledger.md#rust-toolchain)). Component copies, complete diagnostics,
commands and dependency/source hashes remain under
`target/compiler-artifact-final/refusal-assessment/`. Normal builds do not depend on these
observations. Reproduction uses the actual current file, source types and dependency, rather
than a shortened hand-written substitute.

#### Restored inputs, pinned premises and falsified receipts

`2026-10-19`: all 1132 captured regular original/copied inputs restore with independent
Python/system hashes and modes; the opaque gitlink is checked as metadata only. All
39 direct bindings, 37 pinned public producer sources and 10 installed public API/source
bindings agree. The installed sources derive at runtime from `rustc --print sysroot`; their
repository-relative evidence copies and identities are preserved in the preceding proof
sections. No submodule implementation is inspected and no pin changes.

`2026-10-19`: all 13 full-receipt falsifiers reject: scope, omitted result, changed
replacement, invented parent, changed total/exit/file/source hash/outcome, duplicate result,
invented equivalent, reduced population and missing reason. All 313 earlier archive files
remain byte-exact. The membership leaf seals exactly; independent Git/index/filesystem
populations agree, and false small/off-by-one archive counts refuse.

Full-receipt audit SHA-256 `52d8e48f87a7dad42a24b9b9f1992f2ce6d1c8a1e6dc8dbd937766b77c635244` and
witness/refusal audit SHA-256 `e7ae1256e4928a65e2dcfad18d7a1dab7f0162b0c847a5d1656e84003199cc71` bind the native results,
source identities, required witnesses, unchanged reasons/parents and restoration. Raw
discovery, listings, plans, commands, native output/exit, complete named results and audit
programs remain untracked under `target/compiler-artifact-final/`.

#### Integration checkpoint and next caller obligation

`2026-10-19`: the complete local integration tier must pass on the final still staged tree
before this checkpoint commits. It includes formatting, strict lints, the full contract suite
and operator proof controls, doctrine gates and their negative self-tests, the pinned book
build, bare-metal and browser-target builds and the emulator check. A failed, incomplete,
unavailable or quarantined result prevents closure. The official integration stamp must
name exactly the committed tree. This is a local checkpoint, with no remote CI claim.

The next owned caller suffix assessment is `.15.2.1.4.1`: historical
`L814.dir.284`/`L814.lit.285` map uniquely to current caller
`L729.dir.201`/`L729.lit.202` in `xtask/src/trust.rs`. These are distinct from the artifact
constructor forwarding edits now killed above. No artifact K/E or retired predicate
resolves a caller obligation. The current caller source SHA-256 is
`fdc28b267ceb173da133e6b0d287758df703ecba6e7b3ca4453964b1c7aa567c`.
Public producer/caller-domain assessment and its own full-parent evidence precede later
compiler/global closure and independent review.

### Caller suffix producer assessment

`2026-10-19`, leaf `.15.2.1.4.1`: both exact current caller edits still survive the complete
current semantic parent cohort. A newly assessed successful public fixture distinguishes
the substring edit. Its permanent regression is owned next; no equivalence reason is added
and the caller adequacy criterion remains unmet.

#### Exact caller identities and preliminary native result

The unchanged file `xtask/src/trust.rs` has SHA-256
`fdc28b267ceb173da133e6b0d287758df703ecba6e7b3ca4453964b1c7aa567c`. Fresh CLI and maintained discovery agree on the complete
zero-based production range `[146,2587]` and every raw candidate. The assessment selects only
the adjacent-option suffix extraction pair, distinct from the artifact naming constructor.

| Earlier caller | Current caller | Exact change | Current result |
| --- | --- | --- | --- |
| `L814.dir.284` | `L729.dir.201` | `starts_with("extra-filename=")` becomes `contains("extra-filename=")` | S |
| `L814.lit.285` | `L729.lit.202` | `starts_with("extra-filename=")` becomes `starts_with("extra-filename")` | S |

The source-bound historical mapping is exact; the earlier source069d8e59 measurement is
historical evidence only. Current outcomes are measured independently.

```bash
bash scripts/mutation_sweep.sh xtask/src/trust.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L729.dir.201 --only L729.lit.202
```

`2026-10-19`: fresh complete discovery lists 1404 raw candidates; the two selected edits run
against all 196 current semantic parents after a passing copied baseline. Both finish S,
strict native exit `1`. No selected K, E, compiler refusal, timeout or unjudged result occurs.
The copied names/order match the original cohort. Held native tree
`ffe17ea0b13282d0916e73796e68cef90dcd3184` includes only the owned assessment activation and prior artifact seal.
No selected result is projected onto the other caller candidates.

#### Successful public producer controls

A standalone driver imports the actual unchanged production modules and calls the public
`inventory_with` API on committed project-local fixtures. The initial driver omitted
`trust_generated` and `generated_header`; compilation refused before any fixture ran.
Adding those unchanged modules makes the whole actual API compile. The refused driver and
diagnostics remain supplemental calibration evidence, not a producer result.

Ordinary binary/library roots succeed, including release incremental builds. An explicit
mixed `rlib`/`dylib` library is refused without its manifest admission. With the exact
manifest hash/reason admitted and its dynamic output explicitly classified through
`defprogram`, the same public input succeeds. These are ordinary documented input forms;
no production bypass or ignored refusal converts an unsupported probe into evidence.

Cargo's pinned producer makes the filename suffix optional. Its published
[compilation-file source](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/compiler/build_runner/compilation_files.rs)
returns no extra filename for the local dynamic-library case exercised here. The source is
matched to the installed Cargo commit; its repository-relative evidence copy is
`target/caller-suffix-assessment/compilation_files.rs.txt`, SHA-256
`5ab7f5e394d69a042c13229d59689c3b037eef966bfb0bad8098a713bc82d910`. The existing pinned compiler-command producer emits
incremental options after the optional naming suffix
([toolchain ledger](ledger.md#rust-toolchain)). Those source observations guide the probes;
the successful actual builds below establish the exercised host cases.

| Actual public input | Unchanged API | Observed library dep-info |
| --- | --- | --- |
| ordinary roots | inventory written | `b-<sixteen hexadecimal digits>.d` |
| ordinary roots with release incremental enabled | inventory written | same hashed-name shape |
| mixed library without admission, with ordinary or bait path | refused before recorded compilation | none; negative controls only |
| admitted/classified mixed library | inventory written | literal `b.d` |
| admitted/classified mixed library, release incremental, ordinary path | inventory written | literal `b.d` |
| same admitted incremental library under `extra-filename=inside-admitted` | inventory written | literal `b.d` |

The independent observation reader decodes the actual observer's native argument records,
checks successful compiler status and ordinary input hashes, and finds the actual dep-info
file from the fresh output directory. It holds the literal unhashed basename in the mixed
case. This expectation is not derived from the suffix matcher under assessment.

#### Whole actual edit and root cause

The actual caller uses the adjacent pair predicate and then slices the operand at the
length of the exact key:

```rust
(w[0] == "-C" && w[1].starts_with("extra-filename="))
    .then(|| w[1]["extra-filename=".len()..].to_owned())
```

With no extra-filename option, the correct suffix is empty and the actual file is `b.d`.
Release incremental compilation still supplies a codegen operand beginning `incremental=`.
When its real output path contains `extra-filename=`, the contains edit accepts that different
operand and slices from the fixed offset for an extra-filename key. The resulting
`b<incorrect-fragment>.d` names a missing nested path. The genuine compiler has already
succeeded and written `b.d`; the error comes from the edited unit collector's filename.

For the exact whole-caller copies, only the two child-module path attributes point back to
their unchanged production files. Reversing those adapters and the single selected edit
reconstructs the original source byte for byte. A relocated unchanged copy succeeds in
every admitted case, so path relocation is independently controlled.

| Admitted actual fixture | Unchanged / relocated | Contains edit | No-equals-prefix edit |
| --- | --- | --- | --- |
| mixed library, no incremental option | written / written | written | written |
| mixed library, incremental, ordinary path | written / written | written | written |
| mixed library, incremental, path containing `extra-filename=` | written / written | error; no inventory | written |

The same bait case's binary root retains its real extra-filename operand before the later
incremental option and still compiles. The no-suffix library is what exposes the broader
substring judgment. The unrelated no-equals edit agrees on this finite matrix; that grants
no all-domain equivalence.

#### Independent source, artifact and execution oracles

`2026-10-19`: the observations hold 16 public contexts, 28 actual compilations,
13 written inventories, two unadmitted negative controls and one exact supported
contains-edit failure. Every written unit names the literal source file and its independent
Python/system hash. Actual artifact bytes match inventory hashes; the binary artifacts run.
The changed collector's failure still leaves a genuine compiled library. All actual-library
consumers below execute and return the independent expected value.

| Actual caller variant on the bait case | Library SHA-256 | Consumer stdout |
| --- | --- | --- |
| `original` | `1a9105566b5502a2785d7b9798865d130fbda9e64f19b914112360420b1feeee` | `37` |
| `relocated` | `dbc4751b1c2e4573bba926608ef4cb560ae273c0d58833f7da63c828af959058` | `37` |
| `L729.dir.201` | `fe3f7fe436356fff1bf3616f876c743626484fed75816c3d03d181742e8a597d` | `37` |
| `L729.lit.202` | `bfc55807ee227ec1f36ba014c49741d16227f2eed9c04468dc94391f91680a31` | `37` |

The consumer is compiled against the actual emitted rlib, including the case where the
contains-edited collector cannot write an inventory. Wrong value, omitted consumer and
wrong-library-hash receipts all reject. This distinguishes a wrong dep-info lookup from a
genuine failed compiler invocation.

#### Preservation, falsified receipts and scheduled resolution

`2026-10-19`: all 1133 captured regular original/copied inputs restore with independent
hashes/modes, and the opaque gitlink is checked as metadata only. All 39 direct bindings,
37 existing public producer pins and the additional immutable Cargo source agree.
Maintained production, previous tests, catalogues and every reason file remain byte-exact.

`2026-10-19`: all ten native-receipt falsifiers reject missing/changed/duplicate results,
changed exit/file/source/count/replacement/scope and invented equivalence. Five public
falsifiers reject missing/invented cases, false success/inventory and invented diagnostics;
the separate consumer controls also reject altered evidence. The artifact-resolution seal
contains exactly its two newly closed bodies. All fourteen earlier stubs remain separate
and all 314 older archive files remain byte-exact. Git/index/filesystem populations
agree; false small/off-by-one archive counts refuse.

The supplemental checker initially expected a contiguous container including those already
sealed stubs. Its assertion refused before publication. Reviewing the canonical sealer and
exact reconstruction corrected the independent checker to compare the actual newly sealed
bodies, preserving every older archive and stub. The original checker and correction record
remain under `target/caller-suffix-assessment/`. This changes no canonical history semantics.

Audit SHA-256 `6ae1f5aa675fbcd0d4fb976c773bb674c1e2b992d2dead4dd55c0ea76444c9bf` and
observation-reader SHA-256 `30566f184acf7810e76efe6039ede9c48756bf2f5f8a8b5917759d0be51499df` bind exact native/whole-source
observations, typed argument records, independent dep-info/source/artifact oracles, consumers,
false receipts and restoration. Raw commands, sources, records and diagnostics are retained
under `target/caller-suffix-assessment/` as supplemental reproducible observations; normal
builds do not depend on them.

> Finding owned: the complete current semantic cohort survives the contains edit while
> this accepted public fixture distinguishes it. P1 child `.15.2.1.4.2` owns one bounded
> permanent semantic regression and its intended fresh native kill next. This is a
> test-adequacy obligation, with no original production failure claimed.

Child `.15.2.1.4.3` separately assesses/proves the exact no-equals-prefix edit, including all
producer routes, optional suffix states, actual compiled copies and rejected wider premises
before any precise reason. Child `.15.2.1.4.4` owns the complete bounded suffix checkpoint.
Later whole-caller/source/reason replays and independent review remain open before F30.

This assessment changes no maintained source, tests or reasons. The preceding artifact
checkpoint's full local integration passed on that unchanged code; this documentation/history
commit runs the maintained book and doctrine checks. No new full-integration, remote CI or
caller-adequacy pass is claimed for the assessment.

#### Caller assessment commit-note routing

`2026-10-19`: the following completed commit rows are retained verbatim from the committed
private task index. Their statements describe their recorded source/parent epochs; they do
not become current measurements here. Exact reconstruction recovers every original private
row and preserves the whole preceding book. This contains the private resume index while
the public book remains unbounded.

| Leaf | Recorded commit | Verbatim historical note |
| --- | --- | --- |
| `M3.6.9.6.3.2.2.14.1` | `ARCHOGEN-M3-0740 (leaf M3.6.9.6.3.2.2.14.1)` | **run_cargo measurement preflight verified**: full native listing/initial source/result and exact11-to12 binding, fixed-RUSTC insertion/new obligation explicit; compiled passive original/six actual edits reveal omitted stores without writers; parameterized installed-host OS guard proves positive/five denied operations and unprotected reverse controls on local targets,141 real parents pass; production/controls/reasons exact, book recipe/live guarded measurement next, no current mutant outcome or E |
| `M3.6.9.6.3.2.2.14.2` | `ARCHOGEN-M3-0741 (leaf M3.6.9.6.3.2.2.14.2)` | **Complete guarded run_cargo assessment**: exact12 edits yield9 named kills/3 owned diagnostic survivors, strict1; all1047 originals/current copies restore, seven altered receipts refuse and code/control/reason bytes exact; end-elements shared-package kill traced through actual successful inventory and independent all-success Cargo target isolation/Fresh/Running intervention, guard confounding investigated; book/live advance to diagnostic matcher assessment, no adequacy/E claim |
| `M3.6.9.6.3.2.2.14.3.1` | `ARCHOGEN-M3-0742 (leaf M3.6.9.6.3.2.2.14.3.1)` | **Cargo selector assessment exposes original defect**: supported multiline prefix and real type error distinguish all three source-bound edits from independently literal Cargo diagnostic; four successful controls and loose-oracle rejection; original helper misreports path continuations including error: decoy, root physical-line matching confirmed, immediate P2 public framing repair/regression owned; code/operator/reasons unchanged, book/live disclose defect and next repair, no E/native-kill claim |
| `M3.6.9.6.3.2.2.14.3.2.1` | `ARCHOGEN-M3-0743 (leaf M3.6.9.6.3.2.2.14.3.2.1)` | **Cargo failure diagnostics exclude path continuations**: public compiler/operational/target/canonical-ancestor regressions before3fail to final4pass and recovery, native old-defect reversal killed by all four finishing parents/restored; new byte-span helper, other source/tests exact, catalogue append-only, prior E unchanged; colon-layout/sandbox-scope overclaims corrected transparently; exact-stage focused/book/staged closure, current51 scope and retired raw selector rows owned for fresh measurement, no adequacy claim |
| `M3.6.9.6.3.2.2.14.3.2.2.1` | `ARCHOGEN-M3-0744 (leaf M3.6.9.6.3.2.2.14.3.2.2.1)` | **Complete repaired helper assessment**: all51 exact current candidates,38 named kills/9 owned survivors/4 compiler refusals strict1; initial11/prior12 retained/new/retired bindings and guard controls revalidated, all1047 original/current copied inputs restore, seven false receipts reject; code/tests/reasons exact, book/canonical/live aligned, finite-iterator proofs next, no E or adequacy pass |
| `M3.6.9.6.3.2.2.14.3.2.2.2.1` | `ARCHOGEN-M3-0745 (leaf M3.6.9.6.3.2.2.14.3.2.2.2.1)` | **Finite Cargo path loops proven**: exact one-or-two constructors and concrete path types preserve all ends/order; permanent original/actual helper and traces, successful wider constructors and wrong-oracle/source-premise refusals before precise reasons; full33 operator controls and native2 E strict0, all1051 originals/current copies restored, seven false receipts refuse; prior code/control/reason prefix exact, canonical/book/live sync, seven remaining source/domain obligations next |
| `M3.6.9.6.3.2.2.14.3.2.2.2.2.1` | `ARCHOGEN-M3-0746 (leaf M3.6.9.6.3.2.2.14.3.2.2.2.2.1)` | **Relative public gate defect confirmed**: actual same-commit absolute0/written versus relative1/own-config false refusal; exact compiled lexical closure and OS same-file/bytes isolate root, baseline host skip qualified; held1051 input census and intentional early document-edit epochs recorded, code/control/reason bytes exact; book/canonical/live expose limitation and immediate P2 repair owner, seven survivor judgments wait |
| `M3.6.9.6.3.2.2.14.3.2.2.2.2.2.1` | `ARCHOGEN-M3-0747 (leaf M3.6.9.6.3.2.2.14.3.2.2.2.2.2.1)` | **Repository command paths resolve before authorization/output**: isolated command before2pass/1fail to final3pass, exact defect reversal killed/restored; real frozen-source absolute/relative/alias calls produce identical same-output inventory bytes, invalid repo paths2/no output and public authorization unchanged; overbroad different-output oracle rooted in rlib embedded paths/excluded, all1051 public inputs restore, prior bodies/reasons exact; canonical/book/live and final exact-stage focused required, integration checkpoint next |
| `M3.6.9.6.3.2.2.14.3.2.2.2.2.2.2.1` | `ARCHOGEN-M3-0748 (leaf M3.6.9.6.3.2.2.14.3.2.2.2.2.2.2.1)` | **Tier timing rationale corrected**: maintained scoped census/current focused receipt show obsolete source/book warm estimates; replace with actual coverage and per-run timing contract, seven source-comment lines/positions and all executable/Tier bytes preserved, native tier output identical, helper/caller/operator/E exact; final focused matching-stage gate/book/live sync required, full integration checkpoint next |
| `M3.6.9.6.3.2.2.14.3.2.2.2.2.2.2.2` | `ARCHOGEN-M3-0749 (leaf M3.6.9.6.3.2.2.14.3.2.2.2.2.2.2.2)` | **Repaired caller full checkpoint passes**: exact current raw historical59/14/51/2,28 pinned sources/33 controls and unchanged environment/allowlist/reasons bind; actual local integration12/0 on official/native held180b activation, all1055 originals restore and five false bindings refuse; code exact, completion book/staged epoch separate, caller repair and timing checkpoint close, seven remaining helper judgments resume, no fresh adequacy/remote CI/board claim |
| `M3.6.9.6.3.2.2.14.3.2.2.2.2.3` | `ARCHOGEN-M3-0750 (leaf M3.6.9.6.3.2.2.14.3.2.2.2.2.3)` | **Seven helper obligations assessed**: exact current edits/compiled environment bind; real Cargo target alias, independent cwd warning and leading manifest error distinguish three edits; two absolute-path proof proposals and two producer-dependent boundaries remain; wider/false-binding controls, all1055 restored; no new native result/E; separately own immediate regressions, proofs and producer assessment |
| `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.8.6` | `ARCHOGEN-M3-0794 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.8.6)` | actual three-output query, each rlib position and each missing nonselected file held with literal path/independent digest/recovery; exact historical2S become native2K/full188,1121 restored/37 pins/five false receipts; original/copy payloads/joined seal/304 archives exact, production/old tests unchanged; book/live/final staged focused, file kinds next |

### Artifact hash checks under escaped paths

`2026-10-19`, leaf `.15.2.1.4.2.0`: the test-only `observed_artifact` checker now sends
the actual artifact bytes to the independent system hash command through stdin. The new
path regression reproduces a false failure in the old helper and passes after the repair.
This protects the existing artifact oracles before the supported caller-suffix regression.

#### Actual formatter defect and unchanged-helper reproduction

The old helper runs `shasum -a 256 <path>` and compares its first output token with the
inventory's plain digest. Installed `shasum 6.02` documents a leading backslash when its
filename output escapes a newline or backslash. That marker belongs to the filename-bearing
output format, not to the SHA-256 value.

The known regular-file payload is `held artifact bytes` followed by LF. Python and the
system stdin form independently give this digest:

```text
ea40e457619b75b94b1cd15dfbaaf73b8cf1f6b5d039a77548178711af4e7629
```

With an escaped filename, the filename form's first token instead begins:

```text
\ea40e457619b75b94b1cd15dfbaaf73b8cf1f6b5d039a77548178711af4e7629
```

The permanent parent calls the actual unchanged helper with a minimal inventory artifact
record and this known payload. The ordinary case passes; the newline case triggers the
helper's digest equality assertion. The bounded child and Cargo both report exit `101`.
The expected token is the plain hash while the parsed token includes the extra backslash.
The existing helper body was byte-exact to the committed source during that reproduction.

The first supplemental output guard expected the child's failure as flattened text. The
bounded runner retains it as a debug-escaped stdout string, so that guard refused. Decoding
the nested string and both digest values exactly confirms the actual helper failure;
`red.json` retains that qualification. No compile/setup failure is counted as the defect.

#### Byte hashing and child lifetime

The helper reads the verified regular file's bytes before spawning the independent hash
process, pipes those bytes to `shasum -a 256` with no filename argument, and waits for its
output. The write result is checked only after waiting, so the child is reaped before an
input-write error is reported:

```rust
let sent = child.stdin.take().unwrap().write_all(&bytes);
let digest = child.wait_with_output().unwrap();
sent.unwrap();
```

The path, regular-file and inventory-digest checks retain their existing contracts. The
new regression supplies ordinary, newline, backslash and combined path names:

| Fixture path component | Filename form has an escape prefix | Repaired actual helper |
| --- | --- | --- |
| `ordinary` | no | passes |
| `line\nbreak` | yes | passes |
| `back\slash` | yes | passes |
| `both\nand\slash` | yes | passes |

The backslash sequences in this table describe actual newline/backslash filename bytes;
the test creates those bytes, rather than paths containing a printed escape notation.
Each stored file contains the exact known payload. Independent Python/system stdin hashes
agree in every case, while the filename form reproduces its documented marker. Wrong
payload and wrong-hash oracles reject. All files remain on the repository volume.

`2026-10-19`: the exact maintained parent finishes `1 passed; 0 failed` after the repair.
Its full name is
`trust::tests::observed_artifact_hashing_preserves_newlines_and_backslashes_in_paths`.

```bash
cargo test -p xtask \
  trust::tests::observed_artifact_hashing_preserves_newlines_and_backslashes_in_paths \
  -- --exact
```

The code ordering handles child lifetime on a write error; the runtime regression exercises
successful writes and the filename cases, without claiming a separate injected write-error test.

#### Exact change and source scope

Reversing only the helper repair and removing only the new parent reconstructs the complete
previous `xtask/src/trust.rs` byte for byte. Every earlier test body and production byte is
preserved. Current whole-file SHA-256 is
`b39ca1f3e2b31d2fdd57a25e0e3e4fa6edd7729fb5ea302f2a1976268737d28b`; production-prefix SHA-256 remains
`036f650dc082d9d43956865ef67fb16f761c4b3ede54c0567f8936733322bef4`.

`2026-10-19`: fresh CLI/maintained discovery retains all 1404 raw production edits exactly.
The current semantic listing is 197 parents, with only the new path control added to the
earlier cohort. That is a fresh listing, not a new caller mutation verdict. The earlier
suffix assessment's source/cohort remains historical evidence; the next regression must
bind the current source and require its intended finishing native kill.

Helper SHA-256 `5e5061a995c93a838ea18eec26e363f621752c7a54ef9893f328beec326fba5c` and
new-parent SHA-256 `932d3ff3a8479f28047d55785c7a4976da400242071740bd1d22279643a1a3b5` identify the exact repair/control.
The independent audit checks 38 other source/test/tool bindings and every catalogue/reason
against the committed bytes. No production predicate or equivalence record changes.

The prior caller-assessment body seals exactly, and all 315 older archive files remain
byte-exact. Independent Git/index/filesystem populations agree and false small/off-by-one
counts reject. The new assessment seal retains the committed body’s terminal blank
separator exactly: the canonical body has three final LF bytes and its sealed form has two.
The ordinary whitespace command flags only that historical ending; new-content whitespace
is checked separately and the sealed bytes remain unchanged. Exact-source reconstruction
verifies that retained separator. Audit SHA-256
`99a4997dd9551b39429c84428edc16418e4edf0947ccb6cc73740111ad96b436` binds the original helper reproduction, repaired control,
known payload/hash oracles, exact source reconstruction, raw discovery and preservation.
Supplemental logs, formatter/help output and audit programs remain under
`target/artifact-hash-filenames/`. The permanent maintained test preserves the regression
independently of those untracked observations.

The final still-tree focused tier must pass before this leaf commits, including strict
format/lint, the full contract suite and existing operator proof controls. The next owned
leaf `.15.2.1.4.2.1` adds the actual admitted no-suffix/incremental-path collector regression.
Caller/global adequacy and the separate no-equals-prefix assessment remain open.

### Supported no-suffix collector regression

`2026-10-19`, leaf `.15.2.1.4.2.1`: the exact caller substring edit `L729.dir.201` now
finishes as a named kill against the complete current semantic cohort. The permanent public
input succeeds before and after its incremental phase, retains genuine compiler products,
and runs the actual library consumer before judging the captured result. Production code
remains unchanged; this closes the owned substring test-adequacy obligation.

#### Complete admitted input and independent expectations

The fixture has an ordinary binary root `a` and library root `b`. All paths in this input
and the source table are relative to the generated fixture root. The library manifest
explicitly requests these outputs:

```toml
[lib]
crate-type = ["rlib", "dylib"]
```

Its full manifest SHA-256 is `900157f2b6cfaa42f7526b2b12e936ece4fab7c48daccbfb1341b6caa65d4ae4`. The actual roots/admission/classification
input is retained here, with the exact hash and rule rather than an inferred bypass:

<pre><code class="language-lisp">(defroot gen (role generator) (package "crates/a") (target bin a) (role-packages "crates/a"))
(defroot chk (role scheduling-checker) (package "crates/b") (target lib) (role-packages "crates/b"))
(defadmit (file "crates/b/Cargo.toml") (rule "&#96;crates/b/Cargo.toml&#96;: &#96;lib.crate-type&#96; holds &#96;crate-type&#96;, which can make the package another kind of crate") (sha256 "900157f2b6cfaa42f7526b2b12e936ece4fab7c48daccbfb1341b6caa65d4ae4") (reason "a fixture"))
(defprogram dynamic (package "crates/b") (target dylib b) (reason "explicit alternate output") (role-packages "crates/b"))</code></pre>

The ordinary workspace has the standard explicit member set. Only the captured phase adds:

```toml
[profile.release]
incremental = true
```

The fixture path contains literal `extra-filename=inside`. The pinned producer's admitted
mixed-library case has no extra-filename operand and emits literal `b.d` and `libb.rlib`.
The [producer assessment](#caller-suffix-producer-assessment) binds that optional branch to
published source and actual accepted inputs
([toolchain ledger](ledger.md#rust-toolchain)). The test requires the literal dep-info
basename independently of the matcher. It also requires the exact unit/package/source set
and these known source hashes:

| Unit | Literal source | Independent expected SHA-256 |
| --- | --- | --- |
| `a / crates/a` | `"crates/a/src/main.rs"` | `536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4` |
| `b / crates/b` | `"crates/b/src/lib.rs"` | `a746195f0d21200b88bfa6608912329fec0d6b4b19c2f09a0e75628a1eada50b` |

#### Successful phases and a finishing failure

| Phase | Original collector | Exact contains edit |
| --- | --- | --- |
| admitted mixed library, ordinary release profile | inventory written | inventory written |
| same input with release incremental enabled | captured inventory written | captured wrong dep-info error |
| ordinary profile restored in a fresh commit | inventory written | inventory written |
| consumer linked to retained incremental rlib | prints `37` | prints `37` |
| final captured incremental-result judgment | passes | intended named failure |

The permanent parent captures both an API error and an unexpected panic before retrying;
either still fails its final success requirement. It saves the actual incremental `b.d`
and rlib before the clean retry replaces the role outputs. The retained rlib has a valid
archive header and an independent system stdin digest. The actual consumer is compiled
against that retained library and must run successfully with the literal output above.

Only after recovery and consumer assertions pass does the test write
`incremental-recovery.txt` with `recovered` and `consumer=37`. Then it unwraps the captured
incremental result and checks its unit/source and artifact hash against the retained bytes.
Thus a compiler failure, incomplete retry or failed consumer cannot masquerade as the
finishing suffix judgment.

On this host the edited collector records the expected wrong nested dep-info path, with
an OS not-found error. The real literal `b.d` exists; the compiler succeeded. The recovery
marker and re-executed actual consumer are independently checked in both original and
native-copy fixtures.

| Audited fixture | Captured incremental result | Retained rlib SHA-256 | Consumer |
| --- | --- | --- | --- |
| original | written | `9038c55a0e8514369e2d1269d8488d9c18e181b362207d49321ce8d5bf0c2e48` | `37` |
| native-copy | expected wrong dep-info error | `834a98939eb78fd796206edb52fe66a20d18b9d9121216114e16766c30ccef2b` | `37` |

The retained rlib hashes differ between the physical original and copied build locations.
This was investigated before signoff. Their machine-code object member payloads are byte
identical, while archive member names and compiler metadata contain different build paths.
The matching real invocation records retain the actual incremental arguments in each case.

A separate diagnostic replay removes incremental-cache naming from both recorded commands;
all non-location flags agree. Raw path builds still differ. Mapping each compilation
directory to the same virtual relative path makes the two resulting rlibs byte-identical,
with independent Python/system hashes agreeing. The argument census also identifies the
distinct role dependency-search directories; an initial check that normalized only the
output directory refused before this qualification.

This matches the [inventory consumption contract](../../specs/trust/decision_trust-inventory.md),
section 6, case 4: the package ships its actual trust-build artifacts, and two-build byte
reproducibility is not assumed. The table records those actual per-build identities.
The diagnostic adds no production/test build flag. Commands/member hashes/results remain
under `target/caller-suffix-regression/library-variance/`.

Both fixture repositories finish clean with the ordinary workspace restored; their prior
commits retain the actual incremental setting. Every recovery inventory's source and
artifact hashes agree with independent Python/system hashes. The retained compiler products
and result/recovery traces remain available under each bounded fixture.

#### Fresh source-bound native kill

The source file is `xtask/src/trust.rs`, SHA-256
`90c1078daba457e76f6564dcaf19557013b30e2275921a2fd25ca000c16d10ef`. Its production prefix remains SHA-256
`036f650dc082d9d43956865ef67fb16f761c4b3ede54c0567f8936733322bef4`. Removing only the new parent reconstructs the complete
preceding file, including the repaired hash helper and every earlier parent, byte for byte.

The historical caller `L814.dir.284` maps to current `L729.dir.201`. Its earlier current-source
assessment measured S on sourcefdc28b26/full196. That is before evidence only. This native
run binds the actual current file, exact unchanged raw edit and expanded current cohort.

```bash
bash scripts/mutation_sweep.sh xtask/src/trust.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --only L729.dir.201
```

`2026-10-19`: complete CLI/maintained discovery still agrees on all 1404 raw candidates.
The selected edit runs against all 198 current semantic parents after a passing copied
baseline and finishes **1K**, strict exit `0`. Its sole failing name is the intended parent:

```text
trust::tests::dep_info_suffix_does_not_match_an_incremental_output_path
```

There are no selected survivors, equivalent, compiler-refused, timeout or unjudged results.
The copied names/order match the fresh original population. Held native tree
`1e3121db1090f4439648bd01f30dba5dc5a006dd` binds the activation/source state before completion documentation.
The new parent uses actual input/output/hash oracles; it adds no production-source hash guard
to the semantic cohort. No result is projected onto the separate no-equals edit or the rest
of the caller production range.

#### Restoration and checkpoint scope

`2026-10-19`: all 1134 captured regular original/copied inputs restore with independent
Python/system hashes and modes; the gitlink is metadata only. All 39 direct bindings,
37 earlier public producer pins and the additional immutable optional-suffix source agree.
All 316 task archives and their index remain byte-exact; independent
Git/index/filesystem populations agree and false small/off-by-one counts refuse.

Nine false native receipts reject missing/changed/duplicate results, changed exit, invented
parent/equivalence, wrong source/replacement/scope. Empty or wrong-value recovery markers
reject. Independent retained-library execution and source/artifact hashes bind the actual
failure point, after the successful recovery marker. The original probe and permanent
parent both pass; the actual edit fails that parent's final captured-result requirement.

Audit SHA-256 `31156a17eedaa83a3da3452448cad4856db3898e7c81da0923324a57301e4cdb` and
new-parent SHA-256 `61cde5bea3a9cbe62e27b1576e0bc25b9e1d960b7bfcbd94df99017d45445592` identify the exact result and permanent
control. Raw plans, commands, native output/exit, complete original/copy restoration,
fixture observations and audits remain under `target/caller-suffix-regression/`. The
maintained parent and commands preserve reproducibility independently of those observations.

Final still-tree focused/book/doctrine checks must pass before this leaf commits. The
bounded actual-regression group `.15.2.1.4.2` then closes with its hash-helper prerequisite.
The next owned leaf `.15.2.1.4.3` assesses the distinct no-equals-prefix edit; its finite
agreement still grants no E. The bounded pair checkpoint, later full caller/source/reason
replays and independent review remain open before F30/global adequacy.

#### Supported regression commit-note routing

`2026-10-19`: the following completed commit rows are retained verbatim from the committed
private task index. Their statements describe their recorded source/parent epochs; they do
not become current measurements here. Exact reconstruction recovers every original private
row and preserves the whole preceding book. This contains the private resume index while
the public book remains unbounded.

| Leaf | Recorded commit | Verbatim historical note |
| --- | --- | --- |
| `M3.6.9.6.3.2.2.12` | `ARCHOGEN-M3-0720 (leaf M3.6.9.6.3.2.2.12)` | **task-log layout repaired**: all 23 outside-section and 51 separator-broken current row versions traced, stable merge preserves all 241 prior commit records and both source sequences; 59 verification separators repaired with all 159 evidence rows exact; rendered rows 167 to 242 / 100 to 160 including new records, separator/wrong-location/changed/missing controls refuse; unchanged public presence gate boundary explicit, source/archives untouched, book/live sync and next prefix/helper cohort owned |
| `M3.6.9.6.3.2.2.13.1` | `ARCHOGEN-M3-0721 (leaf M3.6.9.6.3.2.2.13.1)` | **complete prefix/helper measurement**: all 59 actual edits bind unchanged initial region, 46 finishing named kills/12 survivors/1 compiler refusal and strict 1; reconcile 14 labelled plus 7 generic original obligations, nine now K/twelve routed; all 1026 originals/current copy restore, altered evidence refuses, production/prior Rust tests/native/E exact; canonical/book/live sync, director's unbounded-book ruling removes all book ceilings and cancels containment, route regression controls/history sealing verified; concrete path controls next, full adequacy open |
| `M3.6.9.6.3.2.2.13.2.2` | `ARCHOGEN-M3-0722 (leaf M3.6.9.6.3.2.2.13.2.2)` | **public path boundaries held**: valid diagnostic-substring source targets and ordinary/one-/two-equals dependency paths retain independent literal arguments in actual successful inventory builds; exact catalogue and current native edits finish named K strict0, all 1029 original/current copied inputs restore, altered evidence refuses; prior production/tests/E exact and catalogue append-only; full book examples/canonical/live sync, codegen arguments next, focused/stamp/staged gates bind commit; full adequacy open |
| `M3.6.9.6.3.2.2.13.2.3` | `ARCHOGEN-M3-0723 (leaf M3.6.9.6.3.2.2.13.2.3)` | **codegen operands independently held**: real release/package-override inventories retain four literal complete options once without raw-operand duplicates; current-token selection, retained-output deletion and one-token advancement exact catalogue/systematic edits all finish named K strict0, every original/current copied input restores and false receipts refuse; prior production/tests/E exact, catalogue append-only; book examples/canonical/live sync, six matcher assessments next, focused/stamp/staged gates bind commit; full adequacy open |
| `M3.6.9.6.3.2.2.13.2.4.2.1` | `ARCHOGEN-M3-0730 (leaf M3.6.9.6.3.2.2.13.2.4.2.1)` | **Metadata matcher obligations resolve narrowly**: complete pinned constructor/visited-token matrix, successful scalar validation and raw-strip refusal qualified; permanent actual edits with independent Cargo argv, consumed library values and valid wider/wrong-oracle controls; after proof append two exact reasons, actual native2 E strict0, operator29 and original/current-copy1036 restoration, six false receipts refuse; old bodies/reasons/code preserved, book/live extra-filename evidence next |
| `M3.6.9.6.3.2.2.13.2.4.2.2` | `ARCHOGEN-M3-0731 (leaf M3.6.9.6.3.2.2.13.2.4.2.2)` | **Extra-filename matcher obligations resolve narrowly**: permanent actual edits with independent Cargo argv/configuration and consumed library values, valid wider LLVM and qualified refused/synthetic/wrong-oracle controls; after proof append two exact reasons, native2 E strict0, operator30, source preservation and original/current-copy1037 restoration, six false receipts refuse; canonical/book/live incremental next, exact committed archival as needed, full adequacy open |
| `M3.6.9.6.3.2.2.13.2.4.2.3` | `ARCHOGEN-M3-0732 (leaf M3.6.9.6.3.2.2.13.2.4.2.3)` | **Incremental matcher obligations resolve narrowly**: real enabled release profile, independent full Cargo argv/cache path/population and consumed library values, actual compiled edits, valid wider LLVM and qualified premises/wrong oracles; after proof append two reasons, native2 E strict0, operator31, prior body/reason/code preservation and original/current-copy1043 restoration, six false receipts refuse; canonical/book/live close codegen pair parents and resume basename assessment, full adequacy open |
| `M3.6.9.6.3.2.2.13.2.5` | `ARCHOGEN-M3-0733 (leaf M3.6.9.6.3.2.2.13.2.5)` | **Basename comparison proven from distinct positions**: actual compiled original/edit, independent literal examples, bounded helper grid and qualified predicate/equal-index/wrong-negative controls precede exact reason; native1 E strict0, operator32, prior source/body/reason preservation and original/current-copy1043 restoration, six false receipts refuse; canonical/book/live close evidence-directed parent and resume final complete cohort replay |
| `M3.6.9.6.3.2.2.13.3.1` | `ARCHOGEN-M3-0734 (leaf M3.6.9.6.3.2.2.13.3.1)` | **Complete prefix/helper replay passes strict criterion**: all59 actual edits exactly bound to original identities/outcomes, 51 named kills/seven scoped proofs/one compiler refusal strict0; all12 prior cohort survivors resolved, originals/current copies1043 restored, prior source/controls/reasons/native exact, seven altered receipts refuse; canonical/book/live sync, caller/full integration checkpoint next, whole-production adequacy remains open |
| `M3.6.9.6.3.2.2.13.3.2.1` | `ARCHOGEN-M3-0737 (leaf M3.6.9.6.3.2.2.13.3.2.1)` | **Caller bound; failed integration preserved**: exact production/build/pin/source/32-control and complete59 reason/result bindings; full local tier11 passed/1 self-test failed on held native activation, no pass stamp; all1043 originals/prior-copy restore, epochs explicit; book/live record failure, PROGRAM.92 owns handoff-arm root/repair before final integration .2 |
| `M3.6.9.6.3.2.2.13.3.2.2` | `ARCHOGEN-M3-0739 (leaf M3.6.9.6.3.2.2.13.3.2.2)` | **Prefix/helper checkpoint passes**: unchanged caller/28-source/32-control and complete59 reason/result bindings; actual full local tier12/0 on held native/official activation, prior11/1 remains historical; all1046 originals and prior-copy1043 restore in separate epochs, five false bindings refuse, code/controls/reasons/handoff exact; book/live close .13 and own run_cargo binding/locality preflight, full adequacy remains open |
| `M3.6.9.6.3.2.2.15.2.1.3.3.3.2.8.4` | `ARCHOGEN-M3-0792 (leaf M3.6.9.6.3.2.2.15.2.1.3.3.3.2.8.4)` | exact extra-filename= helper boundary held; historical2S become native2K/full186,1119 restored/37 pins/five false receipts; caller old/current pair separately mapped/owned, six historical queue labels corrected with exact reversal;8 notes verbatim/prior ledger/value seal/302 archives exact; book/live/final staged focused, joined options next |

### Repeated generator preflight refusals

`2026-10-19`, leaf `.15.2.1.4.3.0`: a repeated generator package could run code after
preflight had already refused it. The repair in `xtask/src/trust.rs` caches successful
package judgments. A failed visit remains a failure for every later target that reaches
that package. Separate bounded parents hold script execution and manifest-only compilation.

#### Trigger and observed defect

Two generator binaries belong to one package. A role reads a file declared with both:

```lisp
(defgenerated "crates/a/src/main.rs"
  (generator "crates/g/src/bin/one.rs" "crates/g/src/bin/two.rs")
  (command "fixture")
  (reason "fixture"))
```

These paths are relative to the generated fixture repository. Both binaries have explicit
program classifications in the successful controls. The package's build script writes a
literal marker in its own written-tree directory:

```rust
fn main() {
    std::fs::write("RAN", "build script ran").unwrap();
}
```

Its manifest has the exact reviewed build-script admission. That admission permits the
reviewed manifest bytes; unconditional metadata preflight still refuses a live `custom-build`
target before that package is compiled. Its admission rule is
<code>&#96;crates/g/build.rs&#96;: a package has no build script</code>.

Previously the first generator visit inserted the package into the manifest cache before
recording its dynamic-target refusal. Step 4 skips a build when that visit adds refusals.
The second target found the package cached, skipped checks, added no new refusal and
compiled. The script wrote `RAN`. Final refusal prevented an inventory, but could not undo
the execution.

The same defect affected an ordinary unadmitted feature manifest, without a build script:

```toml
[features]
default = ["extra"]
extra = []
```

The first target recorded the manifest refusal and the second still compiled. The defect
concerned caching failed judgments, rather than only the dynamic check's location.

Eight fresh public API replays compare both implementations, cases and generator orders.
All inputs have two classified generator binaries; fixture source bytes remain exact.

| Case | Original, each order | Repaired, each order |
| --- | --- | --- |
| admitted manifest, active script | first target refused; second compiles and writes `RAN` | both refused; no generator compilation or script marker |
| unadmitted feature manifest | first target refused; second compiles | both refused; no generator compilation |
| final inventory in either refused case | absent | absent |

Role builds finish before live generator preflight; the no-build assertions cover the
refused generator package. This does not promise that no earlier role compilation ran.

#### Successful caching and regression controls

The function records the refusal-vector length before each package's checks. Only a visit
that adds no refusal enters the successful set. Immutable accepted packages still reuse
their judgment. Failed metadata, manifest parsing, missing manifests or unadmitted content
cannot acquire a passed entry.

The two parents use the existing bounded-child limit independently:

```text
trust::tests::repeated_generator_build_script_refusals_prevent_compilation
trust::tests::repeated_generator_manifest_refusals_prevent_compilation
```

Each checks both orders. An ordinary inventory builds both binaries; the bad manifest is
committed; a third ordinary commit recovers before the captured bad result is judged.
The dynamic case uses an ignored `build = false` script for successful phases. The other
case introduces and removes the unadmitted feature table.

Literal expectations hold both classification statuses, no unused classifications or
admissions, and one accepted manifest site for the ignored script versus zero for the
plain case. Shared items match before and after recovery. Only that fixture's prior
generator outputs are cleared before the bad invocation, so stale products cannot count
as a new attempt. Script-marker, target-directory and inventory presence are captured
before recovery cleans or rebuilds them.

Both parents fail on the original cache at the intended script/compilation assertion,
after successful recovery; repaired code passes both. Early fixture-oracle calibrations
are retained separately: generator directories include the full package path, prior
products must be cleared, and the inventory site count is a string. These early refusals
are not counted as the defect's RED evidence.

#### Source and native revalidation

Whole-file SHA-256: `c02f30e133a9ef8b54f237dd5aa2cb0c8b55041fbdd304290d5109d65277aa49`. Production-prefix SHA-256: `51af305a7198e25bb793f1d24a24628082f6b0479704b6e65a488497d9c4502d`.
Removing only the helper/two new parents and reversing the small cache change reconstructs
the committed predecessor. Earlier test bodies, all reasons and catalogues remain exact.

The absolute-Cargo-path proof guards the production prefix. Its helper, both compile
routes, root/cwd/output constructors and environment/observer are unchanged. Actual
original/edited helpers, generated records, wider relative paths, altered premises and
wrong oracles pass before the maintained binding changes.

The observer-close proof’s guarded caller hash also changes. Its writer/controller,
post-sync/publication operations and synchronous reader are exact; actual close omission,
protocol/error/timing and altered-premise controls pass before that binding changes.
These two reviewed hash bindings are the only operator-suite edits; all 48 controls pass.

Fresh CLI/maintained discovery agrees on 1414 raw candidates. This repair selects the
cache insertion guard `L1360.cond.479` against all 200 semantic parents. All 22
current reasons remain byte-exact and map to current candidates; their native outcomes are
not remeasured by this repair. The targeted invocation is:

```bash
bash scripts/mutation_sweep.sh xtask/src/trust.rs \
  --package xtask --timeout 600 --filter=trust:: --filter=trust_generated:: \
  --filter=trust_gate:: --filter=--skip --filter=route_tests --timeouts-fail \
  --equivalents xtask/trust.equivalents --only L1360.cond.479
```

The owned receipt reports **1K**, strict exit `0`. The cache edit finishes with exactly
the two intended new parent names. No selected survivor, timeout, compiler refusal or
unjudged result remains. Copied baseline and all current parent names agree.

All 1135 captured regular original/copied inputs restore with Python/system hashes and
modes; the gitlink is metadata-only. The 39 direct bindings and 37 pinned
published producer sources agree with the native input epoch. During completion, the
canonical repair detail and completed collector history route verbatim below. Exact
reconstruction recovers that document’s measured bytes; the compact contract retains its
protocol table, cache invariant and links to the complete records. Ten false native receipts and three false public-case
receipts reject. The prior regression subtree seals exactly; all 316 older archives
stay exact, with independent Git/index/filesystem populations and false-count controls.

Held tree `79a9ca87afb538b38dc69b3d904ae8ad1252fdb6` and audit SHA-256 `31aa4f0d1dc08eddcf95ca5196be45e4b3ae14fa3b4097feb42d76911632e46f` identify this checkpoint. Commands,
original/repaired public observations, RED/GREEN and proof-binding logs, source copies,
native output and restoration audits remain under `target/generator-preflight-cache/`.
The initial combined replay omitted its explicit reason-file argument and was interrupted;
its partial survivors are not the planned evidence. Original/copied inputs restored before
the corrected targeted run. Its failed receipt remains available. Final still-tree focused,
book and doctrine checks precede the commit.

#### The separate prefix proof remains open

The defect surfaced while checking the no-equals suffix proof's preflight premise. Its
source90c1078d/full198 provisional E receipt and operator49 result do not establish that
proof while the premise is false. The proposed new reason/control were removed from active
files and retained verbatim as deferred evidence; the 22 earlier reasons remain.

Next, `.15.2.1.4.3.3` repairs figure registration for literal fences. Leaf `.4.3.1` then
rebinds repaired source and remeasures without E before completing
the source/constructor/adjacency proof. Leaf `.4.3.2` measures the complete changed-cache
cohort and all current trust reasons. The suffix-pair checkpoint, complete caller/global replay and independent review
remain open before F30/M3 adequacy.

The [canonical contract](../../specs/trust/compiler-provenance-contract.md#repeated-generator-package-preflight)
records the successful-cache rule and provenance.

#### Detailed preflight repair record

This record is retained verbatim from the repair’s canonical draft. The canonical contract
keeps the cache invariant and points here; complete detail remains available in the book.


`M3.6.9.6.3.2.2.15.2.1.4.3.0`, `2026-10-19`: unconditional metadata and manifest
preflight must block every target whose build reaches a refused package. A package already
visited for another generator is not thereby accepted.

The previous `Rules::manifests` inserted the package into `manifests_checked` before its
dynamic-target and manifest-content checks. Generated-source step 4 refuses each target
when that visit adds refusals. If two distinct generator targets reach the same failing
package, the first visit records the refusal, but the second used to skip the cached
checks, add no new refusal and proceed to compilation. A public API fixture actually ran
the package's build script on that second visit. The final inventory was still refused;
the prebuild no-package-code contract had already been violated. A separate unadmitted
feature manifest also allowed its second target to compile.

The repair caches only successful complete package judgments. Record the refusal count
before that package's checks and insert it into the successful set only if the count has
not grown. Repeated failed visits therefore issue their own contextual refusal and cannot
pass the caller's new-refusal gate. An already successful immutable package still reuses
its manifest judgment, with accepted sites/admissions counted once. Source-extension
checks remain per visit; no configuration, environment, manifest admission or supported
target kind is broadened.

Two bounded maintained parents in `xtask/src/trust.rs` cover actual build-script and
unadmitted-feature refusals, each in both generator orders. Their ordinary and recovered
inventories have two explicitly classified generator binaries; literal accepted site counts
and unchanged shared items hold. Before the bad invocation, only the fixture's prior
generator build outputs are cleared so stale successful products cannot masquerade as a
new compilation. The parents capture the bad-phase script marker, target-directory creation
and inventory presence before running the successful recovery, then judge those captured
values and require both target-specific refusals.

The owned original source fails both parents after successful recovery; the repaired source
must pass both. The [book evidence](../../book/src/annex-trust-proofs.md#repeated-generator-preflight-refusals)
records current native/reason preservation and final focused checks. The complete current
cache-edit cohort has its own checkpoint before this assessment container closes.

This defect was found while assessing a caller suffix proof. Its provisional source90c1078d/
full198 reason receipt and operator49 result do not establish that proof: the supposed
unconditional dynamic-target premise had not held for repeated generator packages. The new
prefix reason and proof control were removed from the active files and preserved as deferred
evidence. The owned follow-up rebinds the repaired source, remeasures without E and completes
the proof before adding any precise reason. Existing prefix/helper proofs are revalidated
against the repaired preflight; no finite native receipt substitutes for their source arguments.

#### Collector revalidation history

This completed implementation record is retained verbatim from the canonical contract.
Its dates, controls and open verification statements describe its recorded source epoch.


`2026-10-18`, `.15.2.1.3.3.2.1`: the controller/embedded observer implement the collection
rows above for both compile callers. Base environment/preflight and argv remain. Records require
current schema/session/role/cwd, matching PID/filename and successful status. Unknown, malformed,
partial, stale or unsuccessful provenance stops the inventory. UTF-8 interpretation is explicit.
Artifact/compatibility controls pass; source/integration gates follow.

The production record retains the verified P2/native controls, bounded diagnostic matrix,
legacy catalogue/reason retirements, historical controls and current premise revalidation.
Baseline evidence repair is .3.3.3.0; bounded results do not establish full source adequacy.

Exact commands, controls, outcomes, failed attempts and scope are in the
[production record](../../book/src/annex-trust-proofs.md#production-compiler-invocation-collection-repair).
All new judgment files join .3's scope. The
[artifact record](../../book/src/annex-trust-proofs.md#observed-compiler-artifact-binding) owns metadata/source
selection, empty-input naming, direct relative paths, output refusals and actual/copy controls.
The [complete compatibility matrix](../../book/src/annex-trust-proofs.md#compiler-provenance-compatibility-checkpoint)
binds every contract row to controls and explicit source/domain premises.
Full source/premise/integration verification remains .3.

#### Literal fence gate follow-up

The publication gate read tilde-fenced draft text as prose even though mdBook rendered it
as code. This publication uses standard backtick fences with the archived bytes unchanged;
`.4.3.3` owns the isolated reproduction and gate repair. It does not grant any draft claim.

#### Withdrawn prefix draft preserved for continuation

The following three draft additions are retained verbatim against base commit
§05c7ddd1a2abfc5a7d6e0d4b847e2f97cc96eecb§. They were withdrawn when the repeated-generator preflight
counterexample invalidated a premise. Their old source guards, provisional reason and
unconditional preflight statements are historical draft material; they are not an active
or accepted equivalence. The follow-up must rebind repaired source and finish its proof.

**Python proof control**, SHA-256 §277facb417041eb9a94315e62eadd563ec41f22759d7f55608343883fb61426b§:

```python
    @unittest.skipUnless(os.name == "posix", "the pinned producer/caller proof is Unix")
    def test_caller_suffix_prefix_requires_the_complete_pinned_constructor(self):
        """Exact extraction edit: source proof is conditional, wider argv is deliberately different."""
        paths = ["xtask/src/trust.rs", "xtask/src/trust_compiler.rs",
                 "xtask/src/trust_compiler_observer.rs", "xtask/src/catalog_build.rs",
                 "rust-toolchain.toml", ".cargo/config.toml"]
        inputs = {path: (ROOT / path).read_bytes() for path in paths}
        expected = {'xtask/src/trust.rs': '036f650dc082d9d43956865ef67fb16f761c4b3ede54c0567f8936733322bef4', 'xtask/src/trust_compiler.rs': '634034fc7f85d67cb843251b15b2065ba229aee6692c6b68987c717383915b70', 'xtask/src/trust_compiler_observer.rs': 'c625e10018af3a1b4fd8c96ece4b21b5afdba53cd9b77bd8c619ab6a2037743f', 'xtask/src/catalog_build.rs': 'a3ec83b4f8ae870ea7a301dd8acc61d0eb42bb20a5b561c673fa8bb2a23d4864', 'rust-toolchain.toml': '4706b7b407427c38a3b63860b2e131cacc9eb57569a6755a39e81756cca676cd', '.cargo/config.toml': '48296b18a2b27cc7268cf489fc37bdcf24e04d79b914c2237c248e4db1f6fa79'}
        version = self.bounded(["cargo", "--version", "--verbose"])
        commit = "commit-hash: f2d3ce0bd7f24a49f8f72d9000448f8838c4e850"

        def premises(values, producer):
            actual = {path: hashlib.sha256(data.split(b"#[cfg(test)]", 1)[0]
                       if path.endswith(".rs") else data).hexdigest()
                      for path, data in values.items()}
            self.assertEqual(actual, expected, "caller suffix source premise requires revalidation")
            self.assertIn(commit, producer, "caller suffix producer premise requires revalidation")

        premises(inputs, version)
        for path in paths:
            altered = dict(inputs)
            altered[path] = b"// changed premise\n" + altered[path]
            with self.assertRaisesRegex(AssertionError, "source premise requires revalidation"):
                premises(altered, version)
        with self.assertRaisesRegex(AssertionError, "producer premise requires revalidation"):
            premises(inputs, version.replace(commit, "commit-hash: changed", 1))

        source = inputs["xtask/src/trust.rs"].decode()
        namespace, literals = self.discovery(source, "lit")
        key = ('(w[0] == "-C" && w[1].starts_with("extra-filename="))',
               '(w[0] == "-C" && w[1].starts_with("extra-filename"))')
        family = [candidate for candidate in literals if namespace["mutation_key"](candidate) == key]
        self.assertEqual(len(family), 1, "one exact actual caller edit")

        def module(text, name):
            start = text.index("        let extra = tokens\n")
            end = text.index("        // The unit's package", start)
            body = text[start:end]
            self.assertEqual(body.count(".windows(2)"), 1)
            return f"mod {name} {{ pub fn collect(tokens: &[String]) -> String {{\n{body}extra\n}} }}\n"

        modules = module(source, "baseline") + module(namespace["apply_mutant"](family[0]), "variant")
        ordinary = self.producer_argv("caller-suffix-ordinary", library_source="pub fn c() -> u32 { 37 }\n")
        incremental = self.producer_argv("caller-suffix-incremental",
                                       library_source="pub fn c() -> u32 { 37 }\n",
                                       release_incremental=True)

        # This distinct, real producer exercises Cargo's optional no-suffix branch.
        project = self.base / "producer-caller-extra-filename=inside"
        (project / "src").mkdir(parents=True)
        (project / "Cargo.toml").write_text(
            '[workspace]\n[package]\nname = "flagproof"\nversion = "0.1.0"\nedition = "2021"\n'
            '[lib]\ncrate-type = ["rlib", "dylib"]\n[profile.release]\nincremental = true\n')
        (project / "src/lib.rs").write_text("pub fn c() -> u32 { 37 }\n")
        target = project / "target"
        env = {name: os.environ[name] for name in ("PATH", "HOME")}
        env.update(RUSTUP_HOME=os.environ.get("RUSTUP_HOME", str(Path(env["HOME"]) / ".rustup")),
                   RUSTUP_TOOLCHAIN="1.95.0", RUSTC="rustc", CARGO_HOME=str(project / "cargo-home"),
                   CARGO_TARGET_DIR=str(target), TMPDIR=str(self.base), TMP=str(self.base), TEMP=str(self.base))

        def cargo(arguments):
            process = subprocess.Popen(["cargo"] + arguments, cwd=project, env=env,
                                       stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                       text=True, start_new_session=True)
            try:
                stdout, stderr = process.communicate(timeout=60)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.communicate()
                self.fail("actual optional-suffix Cargo producer timed out")
            self.assertEqual(process.returncode, 0, stdout + stderr)
            return stderr

        cargo(["generate-lockfile", "--offline"])
        stderr = cargo(["build", "--lib", "--release", "--locked", "--offline", "--no-default-features",
                        "--message-format=json", "-v", "-p", "flagproof"])
        commands = [line.strip()[len("Running `"):-1] for line in stderr.splitlines()
                    if line.strip().startswith("Running `rustc ") and line.strip().endswith("`")]
        self.assertEqual(len(commands), 1, stderr)
        mixed = shlex.split(commands[0])
        metadata = [arg for arg in mixed if re.fullmatch(r"metadata=[0-9a-f]{16}", arg)]
        self.assertEqual(len(metadata), 1)
        expected_mixed = ["rustc", "--crate-name", "flagproof", "--edition=2021", "src/lib.rs",
                          "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat",
                          "--crate-type", "rlib", "--crate-type", "dylib", "--emit=dep-info,link",
                          "-C", "opt-level=3", "-C", "embed-bitcode=no", "--check-cfg", "cfg(docsrs,test)", "--check-cfg",
                          "cfg(feature, values())", "-C", metadata[0], "--out-dir", str(target / "release/deps"),
                          "-C", "incremental=" + str(target / "release/incremental"),
                          "-C", "strip=debuginfo", "-L", "dependency=" + str(target / "release/deps")]
        self.assertEqual(mixed, expected_mixed, "independent complete optional-suffix producer argv")
        self.assertEqual(sorted(path.name for path in (target / "release/deps").glob("*.d")), ["flagproof.d"])
        self.assertIn("src/lib.rs", (target / "release/deps/flagproof.d").read_text())

        actual = []
        for label, argv, base in [("ordinary", ordinary, self.base / "producer-caller-suffix-ordinary"),
                                 ("incremental", incremental, self.base / "producer-caller-suffix-incremental"),
                                 ("mixed", mixed, project)]:
            operands = [value for left, value in zip(argv, argv[1:]) if left == "-C"
                        and re.fullmatch(r"extra-filename=-[0-9a-f]{16}", value)]
            self.assertEqual(len(operands), 0 if label == "mixed" else 1)
            suffix = operands[0].split("=", 1)[1] if operands else ""
            library = base / "target/release/deps" / ("libflagproof" + suffix + ".rlib")
            self.assertTrue(library.read_bytes().startswith(b"!<arch>\n"))
            consumer, binary = base / "consumer.rs", base / "consumer"
            consumer.write_text('fn main() { println!("{}", flagproof::c()); }\n')
            self.bounded(["rustc", "--edition=2021", "--crate-name", "caller_suffix_consumer", str(consumer),
                          "--extern", "flagproof=" + str(library), "-o", str(binary)])
            self.assertEqual(self.bounded([str(binary)]), "37\n")
            actual.append((label, argv, suffix))

        vector = lambda values: ("vec![" + ", ".join(json.dumps(value, ensure_ascii=False) + ".to_owned()"
                                                    for value in values) + "]"
                                 if values else "Vec::<String>::new()")
        checks = []
        for label, argv, suffix in actual:
            for name in ["baseline", "variant"]:
                checks.append(f'assert_eq!({name}::collect(&{vector(argv)}), {json.dumps(suffix)}, "{label}");')
        literal = [([], ""), (["-C"], ""), (["rustc", "extra-filenameX=tail"], ""),
                   (["-C", "extra-filename="], ""),
                   (["-C", "extra-filename=-0123456789abcdef"], "-0123456789abcdef"),
                   (["-C", "extra-filename=-é"], "-é"),
                   (["-C", "extra-filename=-first", "-C", "extra-filename=-second"], "-first"),
                   (["-C", "incremental=cache/extra-filenameX=tail"], ""),
                   (["-C", "metadata=extra-filenameX=tail"], ""),
                   (["--out-dir", "-C", "--error-format=json"], "")]
        # These are constructor/adjacency controls, not additional successful Cargo inputs.
        for operand in ["prefer-dynamic", "rpath", "lto", "linker-plugin-lto"] + [
                name + "=extra-filenameX=tail" for name in [
                    "opt-level", "panic", "lto", "embed-bitcode", "codegen-units", "debuginfo",
                    "split-debuginfo", "debug-assertions", "overflow-checks", "metadata", "linker",
                    "incremental", "strip", "link-arg"]]:
            literal.append((["-C", operand], ""))
        literal.append((["--check-cfg", "-C", "-C", "extra-filename=-held"], "-held"))
        for argv, suffix in literal:
            for name in ["baseline", "variant"]:
                checks.append(f'assert_eq!({name}::collect(&{vector(argv)}), '
                              f'{json.dumps(suffix, ensure_ascii=False)}, "literal suffix");')
        wider = ["-C", "extra-filenameX=tail"]
        incidental = ["--out-dir", "-C", "extra-filenameX=tail"]
        for argv in [wider, incidental]:
            checks += [f'assert_eq!(baseline::collect(&{vector(argv)}), "");',
                       f'assert_eq!(variant::collect(&{vector(argv)}), "=tail");']
        for value in ["extra-filename", "extra-filenameé=tail"]:
            argv = vector(["-C", value])
            checks += [f'assert_eq!(baseline::collect(&{argv}), "");',
                       f'assert!(std::panic::catch_unwind(|| variant::collect(&{argv})).is_err());']
        main = ("fn main() {\n" + "\n".join(checks)
                + '\nmatch std::env::args().nth(1).as_deref() {\n'
                + f' Some("wrong-positive") => assert_eq!(baseline::collect(&{vector(wider)}), '
                + f'variant::collect(&{vector(wider)}), "wrong wider equality"),\n'
                + f' Some("wrong-negative") => assert_ne!(baseline::collect(&{vector(ordinary)}), '
                + f'variant::collect(&{vector(ordinary)}), "wrong supported inequality"),\n'
                + ' Some("wrong-literal") => assert_eq!(baseline::collect(&vec!["-C".into(), '
                + '"extra-filename=-held".into()]), "-changed", "wrong literal suffix"),\n'
                + ' _ => {}\n}\nprintln!("caller suffix controls: literal, actual, wider and panic domains held");\n}\n')
        program, binary = self.base / "caller-suffix.rs", self.base / "caller-suffix"
        program.write_text(modules + main)
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.assertIn("literal, actual, wider and panic domains held", self.bounded([str(binary)]))
        for argument, message in [("wrong-positive", "wrong wider equality"),
                                  ("wrong-negative", "wrong supported inequality"),
                                  ("wrong-literal", "wrong literal suffix")]:
            with self.assertRaisesRegex(AssertionError, message):
                self.bounded([str(binary), argument])
        print("caller suffix compiled controls complete; checking the precise reason", flush=True)
        records = namespace["load_equivalents"](ROOT / "xtask/trust.equivalents")
        self.assertTrue(key in records, "the completed caller proof requires its exact per-file reason")


```

**Provisional reason record**, SHA-256 §d60d3fec0ab5ef4c96aa80560ecf699be09bb5a4f377283f8b75178a038bc185§:

```text

from (w[0] == "-C" && w[1].starts_with("extra-filename="))
to (w[0] == "-C" && w[1].starts_with("extra-filename"))
why For the current Unix inventory caller with pinned stable Cargo f2d3ce0bd7f24a49f8f72d9000448f8838c4e850 / Rust 1.95.0, every adjacent pair whose first token is -C either has no extra-filename prefix or has the complete ASCII extra-filename= constructor. Cargo emits the optional suffix as extra-filename=- plus a sixteen-hex UnitHash, or omits it. All other codegen constructors retain distinct initial keys for any value. Both role/test and generated-entry compile routes use fixed build or test/no-run commands without trailing compiler args; cleared allowlisted environment, reset Cargo homes and alias-only ancestor-checked config exclude arbitrary flags/wrappers, hard metadata preflight excludes dynamic package code independently of manifest admissions, and the pinned stable gate excludes profile rustflags. Source/name/type, lint/check-cfg, dependency and directory constructors cannot introduce a distinguishing incidental adjacent pair; in particular arbitrary custom check-cfg values are followed by fixed option tokens, not arbitrary suffix operands. The observer forwards immutable native argv and the consumer selects only successful current compilation records after Cargo succeeds. Thus both predicates choose the same first pair or the same empty default; the unchanged fifteen-byte slice starts after the same complete ASCII prefix at a valid UTF-8 boundary and returns identical bytes, preserving dep-info selection and subsequent behavior. Permanent exact production/controller/observer/base/pin/config guards outside semantic tests, actual compiled original/edit, complete real ordinary/incremental/mixed argv and three consumed libraries, literal/constructor/adjacency controls, wider changed-output and short/Unicode panic witnesses, seven altered-premise controls and three wrong-oracle controls delimit the proof. Wider argv is explicitly not equivalent; finite producer agreement is not the proof. Revalidate producer/pin, both callers, environment/config/admissions/preflight, constructors/adjacency, record filter or extraction/slicing changes. Canonical proof: docs/specs/trust/compiler-provenance-contract.md, Caller dep-info suffix prefix proof (leaf M3.6.9.6.3.2.2.15.2.1.4.3).
```

**Provisional source argument**, SHA-256 §c26dc7ff1576aebea2c8fcb69d12a9238c50db148d778da923af5ca2452bf240§:

```markdown

## Caller dep-info suffix prefix proof

`M3.6.9.6.3.2.2.15.2.1.4.3`, `2026-10-19`: this proof concerns only the actual
`compile` extraction edit from `starts_with("extra-filename=")` to
`starts_with("extra-filename")`. It does not justify the separate substring matcher,
which the supported no-suffix/incremental-path regression kills.

The domain is the current Unix inventory caller with the repository's pinned stable
Cargo `f2d3ce0bd7f24a49f8f72d9000448f8838c4e850` / Rust `1.95.0`, its unchanged preflight,
base environment and compiler observer. Finite API/helper agreement is not the proof.
Changes to these premises require source/producer revalidation before retaining the reason.

### Producer and every compile route

There are two production call sites. Role builds select binary/library or comparison-test
targets; generated-source entry points select binary/library/example targets after the
same manifest/dynamic-target preflight. Every route uses fixed `build` or `test --no-run`
commands with release/locked/offline/no-default-features/JSON/package arguments. None uses
`cargo rustc` or supplies trailing compiler arguments.

The Cargo environment is cleared and rebuilt from the allowlist. Cargo homes are reset and
configuration is alias-only, including ancestor checks. User flags/compiler/workspace wrappers
are excluded. Exact manifest admissions do not bypass unconditional build-script/procedural-macro
metadata preflight; the default manifest rule is not itself used as an unconditional premise.
The stable Cargo feature gate prevents admitted profile rustflags from becoming arbitrary
compiler arguments. Cargo's `extra_args_for` supplies trailing `rustc`/`rustdoc` arguments,
absent in both guarded callers. The observer forwards the immutable native argument tail,
without adding compiler flags. Only successful, current-session compilation records with
crate name/out-dir/emit are passed to extraction; recognized queries are excluded.

The published producer source is pinned in
[the flag-domain record](flag-normalization-domain.md#retrieved-source-identities).
Relevant constructors in
[compiler/mod.rs](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/compiler/mod.rs)
are `prepare_rustc`, `build_base_args`, `lto_args`, `build_deps_args`, `extern_args`,
`add_cap_lints` and the runtime native/custom-flag appenders. The optional suffix comes from
[compilation_files.rs](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/compiler/build_runner/compilation_files.rs),
SHA-256 `5ab7f5e394d69a042c13229d59689c3b037eef966bfb0bad8098a713bc82d910`:
`Option<UnitHash>` emits either no operand or the complete
`extra-filename=-` followed by a sixteen-digit hexadecimal `UnitHash`.

| Construction | Relevant adjacent tokens and boundary |
| --- | --- |
| Scalar/boolean/codegen profiles | After literal `-C`, keys are `prefer-dynamic`, `opt-level`, `panic`, `lto`, `linker-plugin-lto`, `embed-bitcode`, `codegen-units`, `debuginfo`, `split-debuginfo`, `debug-assertions`, `overflow-checks`, `metadata`, `rpath` or `strip`. Dynamic values follow a fixed key and cannot change its initial bytes. |
| Optional suffix | Only the complete `extra-filename=` constructor has the shortened prefix; the optional absent branch introduces no matching operand. |
| Directories/link configuration | Cargo constructs `--out-dir` with its output directory, `-L dependency=…` and optionally fixed `linker=…` / `incremental=…` codegen keys. Directory text containing the prefix does not change any initial key. |
| Crate identity/source/type | Names/types have their Cargo constructors; the source argument is followed by fixed diagnostic arguments. Unconditional preflight requires source roots with the `rs` extension. No source/path/name argument can create an unguarded matching adjacent pair. |
| Manifest lints/features | Joined lint flags begin with `--`. Feature and generated check-cfg operands retain their fixed prefixes. Custom check-cfg strings are paired with `--check-cfg`: even if an operand were literal `-C`, its successor is another fixed option, not an arbitrary suffix operand. This accounts for incidental data tokens rather than assuming every `-C` is a codegen flag. |
| Dependencies/lint caps | Extern values contain their name/path delimiter or fixed `proc_macro`; search paths have `dependency=`; cap values are fixed levels. These introduce no matching incidental pair. |
| Build-script/native runtime appenders | Hard dynamic-target preflight excludes these routes. Their fixed `link-arg=` codegen key is also distinct; arbitrary raw custom cfg/native flags are not silently assumed absent without that preflight. |
| Arbitrary extra/profile/unit flags | Trailing compiler args are absent; alias-only/fresh-home/cleared environment excludes config/environment rustflags; the pinned stable feature gate excludes profile rustflags. These are required premises, not conclusions inferred from sample builds. |

### Consequence for the actual extraction

For every adjacent pair satisfying `w[0] == "-C"`, either both prefix predicates are false,
or the same operand begins with the complete ASCII `extra-filename=` prefix. The source
construction and boundary argument above establish this implication for all values in the
stated caller domain, independently of the finite controls.

Both predicates therefore choose the same first pair. For a chosen pair, the unchanged
fifteen-byte slice starts after that complete ASCII prefix, at a valid UTF-8 boundary;
the extracted string is identical. With no chosen pair, both use the same empty default.
Subsequent dep-info paths, errors, units and source hashes are unchanged. No arbitrary-argv
equivalence or compiler-refusal coverage is claimed.

### Permanent controls and falsification

The maintained control in `xtask/mutation_sweep_tests.py` guards complete production prefixes
of collector/controller/observer/base build code, the toolchain file and alias configuration,
plus actual Cargo commit identity. Altered bytes in each source/pin/config premise and a
changed producer identity refuse. These proof gates remain outside the semantic native cohort.

It compiles the exact actual original/edited extraction, not a reimplementation. Independently
specified literals cover the first match, empty/missing operands, complete prefix, Unicode
suffixes, all other codegen key families and incidental check-cfg adjacency. Actual ordinary,
incremental and mixed-rlib/dylib Cargo commands have independently expected complete argv;
the mixed case emits literal `flagproof.d` without a suffix. All three real library consumers
print `37`.

Wider synthetic inputs explicitly disprove arbitrary-domain equivalence: after `-C`,
`extra-filenameX=tail` changes the returned suffix, bare `extra-filename` panics only in
the edit, and `extra-filenameé=tail` makes its byte slice cross a UTF-8 character boundary.
A synthetic incidental-data pair also distinguishes the edit. These are helper-boundary
counterexamples, not successful Cargo inputs. Wrong supported/wider/literal output oracles
refuse independently.

The first new direct-Cargo expectation omitted `embed-bitcode=no` and refused before a reason
existed. Pinned
[lto.rs](https://github.com/rust-lang/cargo/blob/f2d3ce0bd7f24a49f8f72d9000448f8838c4e850/src/cargo/core/compiler/lto.rs),
SHA-256 `97652b448f525af54f1baee199722cd1534768927a2d4deeae924504cade64d6`,
selects `OnlyObject` for the default false profile; `lto_args` supplies that operand.
Fresh current inventory records independently confirm it. The corrected new control requires
the complete actual argv. Historical diagnostic command shapes are not inherited as current
producer expectations. Completed compiled/false controls precede adding the exact reason;
final reason/native replay and checkpoint evidence are recorded in the task tree and book.
```

#### Figure registration repair task

The complete next-task record is retained verbatim here; its private resume entry links to it.

```markdown
- ID: `M3.6.9.6.3.2.2.15.2.1.4.3.3`
  Status: `pending` — figure registration must respect literal Markdown fences
  Priority: P1 immediately after .0, before .1; owner .4.3.
  Defect: mdBook renders the withdrawn draft's four-tilde fences as literal code, while
    scripts/check_figure_register.sh's figures() recognizes only line-start triple backticks.
    Its public gate falsely calls the fenced historical phrase "two production" new prose.
    Root cause is the fence scanner, not a carried current figure.
  Reproduce: .0's archived draft with four-tilde Python/text/Markdown fences builds in mdBook
    but the staged figure gate exits1 on "two production"; standard backtick fences pass
    with exact archived bytes. Original build/refusal output retained in .0's tool records.
  Impact: valid public book literals can block publication or be wrongly registered as
    current prose. .0 uses supported fences; the defect remains owned until verified repair.
  Scope: reproduce the public gate independently in same-volume scratch; preserve source
    ownership boundaries (this project owns the gate per the director's section10 ruling).
    Repair open/close fence handling for supported Markdown backtick/tilde lengths and
    reject mismatched/short closing fences without hiding subsequent real prose. Add
    meaningful gate self-test arms, retain all earlier controls, sync book/live and commit.
  Acceptance: code literals skip in both styles; prose after correct closing fences still
    rejects unclassified counts; existing gate self-tests and real staged book pass.
  Verification: `pending`
  Commit: `pending`
```


### Figure registration for literal fences

Measured `2026-10-19`, leaf `M3.6.9.6.3.2.2.15.2.1.4.3.3`,
`ARCHOGEN-M3-0808`. The preceding archived draft's tilde fences exposed a real publication
defect: the gate toggled a Boolean at every line beginning with triple backticks. It counted
tilde-fenced content as prose and could end a block on short or invalid closing markers.
The actual pinned mdBook renderer treated that historical content as code. Registering the
phrase as a current figure would have described the wrong surface.

The repaired gate remembers the opening marker and length. Only an appropriately indented
closing run of the same kind, of sufficient length and with permitted trailing whitespace,
ends it. It validates backtick info strings before entering code. Figure patterns, inline-code
handling, dated records, register validation and the staged per-document ratchet are unchanged.
Reversing exactly the fence-state replacement and new self-test arms reconstructs the complete
previous script byte for byte. The director's section10 ownership ruling permits repair of
this project-created gate; no held template file or external repository is changed.

Measured `2026-10-19`: the original 14-arm self-test passes; after adding the maintained
regressions the original scanner reports20 pass/13 fail across33 arms, while the repaired
scanner reports33 pass/0 fail. An independent staged scratch-repository replay compares
the original and repaired public commands on19 explicit source fixtures and confirms the same
13 original disagreements. A separate actual mdBook0.5.2 build independently identifies prose
and code in all19 HTML pages. Those expectations come from rendered text, not from calling
the repaired lexer. Every inverted output oracle is refused.

The permanent cases cover both marker kinds, shorter/longer/mismatched closers, a closer with
text, opening/closing indentation, overindentation, unclosed blocks, valid/invalid info strings,
short opening runs, LF/CRLF inputs and the pinned closing-tab behavior. These are standalone
fences; no complete Markdown-container correctness claim follows from this finite matrix.

#### Renderer compatibility finding

The independent renderer check also found inputs on which the pinned renderer differs from
the published grammar. The following JSON strings preserve their exact control bytes:

```json
[
  {
    "source": "~~~text\rseven tests\r~~~\r",
    "intended": "seven tests is literal"
  },
  {
    "source": "```text\rseven tests\r```\rnine tests\r",
    "intended": "seven tests literal; nine tests prose"
  },
  {
    "source": "~~~text\nseven tests\n~~~ \t\nnine tests\n",
    "intended": "seven tests literal; nine tests prose"
  }
]
```

The bare-CR tilde input becomes an empty code block whose language attribute contains the
payload. The bare-CR backtick input exposes `seven tests` as prose and places following
text in code metadata. The real tab after the last fixture's closing markers keeps
`nine tests` in code. Source bytes were checked directly, including09 for the tab.
Disabling default preprocessors reproduces the bare-CR result, while the Markdown renderer
retains the original bytes, narrowing the discrepancy to the HTML parse/render path.

The pinned release's
[Cargo.lock](https://github.com/rust-lang/mdBook/blob/v0.5.2/Cargo.lock)
selects pulldown-cmark0.13.0. Its checksum-verified
[scanner source](https://docs.rs/crate/pulldown-cmark/0.13.0/source/src/scanners.rs)
shows `scan_nextline` searching only LF and `scan_closing_code_fence` consuming only spaces
before checking the line ending. Its
[first-pass parser](https://docs.rs/crate/pulldown-cmark/0.13.0/source/src/firstpass.rs)
uses that line scan to delimit the opening info string and subsequent code lines. These
mechanisms explain both observations. The crate archive SHA-256 is
`1e8bbe1a966bd2f362681a44f6edce3c2310ac21e4d5067a6e7ec396297a6ea0`;
the actual local native mdBook executable reports0.5.2 and hashes to
`8085338f2694c9614c48f80430b4ac7cc8ecf8503974f061070602f3c08b5c1a`.
The executable hash is a local observation, not the release-archive checksum.

The current gate matches the actual pinned closing-tab outcome. Bare-CR probes grant no
positive gate coverage. Leaf `M3.6.9.6.3.2.2.15.2.1.4.3.4` owns compatibility for both
deviations before the deferred prefix proof. It must preserve literal evidence, qualify
the public transformation and check the gate against independently rendered prose/code.
Current committed book chapters are unaffected by these control-byte inputs. Nothing here
limits the book's size, chapter count or detail.

#### Verification and continuity

Measured `2026-10-19`: all317 earlier task archives retain their exact SHA-256.
The newly sealed completed preflight leaf is the original committed span minus its separating
LF, independently reconstructed; filesystem, Git-plus-new-path and digest-index sets agree
on318 archives. Truncated/modified seals and false previous/total-count oracles all refuse.
Every byte of the preceding committed trust annex remains an exact prefix of this chapter.

Supplementary commands and receipts live under `target/figure-fences/`:

```console
$ bash -n scripts/check_figure_register.sh
$ bash scripts/check_figure_register.sh --self-test
$ python3 -I -B target/figure-fences/render-controls.py
$ python3 -I -B target/figure-fences/audit.py
```

The self-test and public gate remain maintained entrypoints; ignored renderer/audit scratch
is supplementary. Its public input cases and root cause are durable above and in the script.
The audit binds original/repaired sources, public exits, actual HTML, archive reconstruction,
and falsifiers; its SHA-256 is
`18d912b0716b33255278026b94ce473af89caaca5dcd0173beb86087bb1e856a`.
Bash syntax and the targeted self-tests pass. This Bash-only slice changes no Rust source,
mutation result, reason or native adequacy claim. Publication/doctrine gates and the pinned
book build are required before its commit; the preflight-cache checkpoint, precise prefix
proof and whole-source adequacy remain separately owned.

#### Completed figure-registration task record

The completed private record is preserved verbatim below, measured `2026-10-19`. Its compact
private entry retains the acceptance checks and links here; the book has no size limit.

~~~~markdown
- ID: `M3.6.9.6.3.2.2.15.2.1.4.3.3`
  Status: `done` — figure registration respects the pinned literal fence boundary
  Priority: P1 immediately after .0, before .1; owner .4.3.
  Defect: mdBook renders the withdrawn draft's four-tilde fences as literal code, while
    scripts/check_figure_register.sh's figures() recognizes only line-start triple backticks.
    Its public gate falsely calls the fenced historical phrase "two production" new prose.
    Root cause is the fence scanner, not a carried current figure.
  Reproduce: .0's archived draft with four-tilde Python/text/Markdown fences builds in mdBook
    but the staged figure gate exits1 on "two production"; standard backtick fences pass
    with exact archived bytes. Original build/refusal output retained in .0's tool records.
  Impact: valid public book literals can block publication or be wrongly registered as
    current prose. .0 uses supported fences; the defect remains owned until verified repair.
  Scope: reproduce the public gate independently in same-volume scratch; preserve source
    ownership boundaries (this project owns the gate per the director's section10 ruling).
    Repair open/close fence handling for supported Markdown backtick/tilde lengths and
    reject mismatched/short closing fences without hiding subsequent real prose. Add
    meaningful gate self-test arms, retain all earlier controls, sync book/live and commit.
  Acceptance: code literals skip in both styles; prose after correct closing fences still
    rejects unclassified counts; existing gate self-tests and real staged book pass.
  Ownership: director ruling section10, current gate origin PROGRAM.20.3, outside the17
    untouched template files; no external repository is changed.
  Additional scope: seal prior completed preflight leaf exactly for private continuity;
    compare public gate outcomes with actual mdBook code/prose rendering, preserve previous
    scanner/register/count behavior outside fence handling and every earlier self-test.
  Acceptance:
  - [x] ROOT CAUSE: `python3 -I -B target/figure-fences/audit.py`, exit code 0:
    original public gate disagrees on13/19 renderer-qualified cases; initial14 passes,
    expanded original20pass/13fail; source toggle mechanism and ownership verified.
  - [x] ADDRESSED: `bash scripts/check_figure_register.sh --self-test`, exit code 0:
    33pass/0fail; actual mdBook19 independent literal/prose checks and public19 replay
    agree, including valid-close prose and pinned tab behavior; inverted oracles refuse.
  - [x] NO REGRESSION: `bash -n scripts/check_figure_register.sh` and
    `python3 -I -B target/figure-fences/audit.py`, exit code 0:
    reverse only state/new arms gives exact prior script,317 old archives exact,
    new318th seal reconstructs exactly;4 history falsifiers refuse, book prefix preserved.
  Verification: audit18d912b0716b33255278026b94ce473af89caaca5dcd0173beb86087bb1e856a.
    Public record: book annex-trust-proofs, figure-registration-for-literal-fences.
    Bare-CR and closing-tab standard deviations root-caused and owned in .4 before .1.
  Commit: `ARCHOGEN-M3-0808`

~~~~

Measured `2026-10-19`: the verbatim task fragment is3135 bytes, SHA-256
`c36e268f6e0afb8c8eae78ac4443da4aeea812291a75693b6325f0b8a999ef06`;
its private entry is1159 bytes, saving1976 bytes under the private task-directory rule.
Substituting the preserved fragment back into that entry reconstructs the entire previous
task file exactly. Truncated, modified and extra-newline fragments are refused independently.
The previous book prefix is exact; no public detail is discarded.


### Publication guard for renderer control bytes

Measured `2026-10-19`, leaf `M3.6.9.6.3.2.2.15.2.1.4.3.4.0`,
`ARCHOGEN-M3-0809`. The preceding finding identifies real differences between source and
rendered code/prose boundaries. The project's supported book build now refuses the affected
source forms before invoking the pinned renderer. Its diagnostics name the relative source
path and source line and give an immediate correction:

```text
book-sources: docs/book/src/nested/control.md:3: bare CR line ending; use LF or CRLF so the pinned renderer preserves code/prose boundaries
book-sources: docs/book/src/nested/control.md:5: tab after closing markers; replace closing-line tabs with spaces
```

`scripts/check_book_sources.py` reads Markdown files recursively beneath `docs/book/src`;
it imposes no chapter, file-size or detail ceiling. It rejects bare CR line endings, invalid
UTF-8 input, unreadable/absent sources and tabs after matching standalone closing markers.
The fence check tracks marker kind and length, valid info strings and indentation. Shorter,
mismatched or overindented markers inside a literal remain data. The check is conservative
source validation rather than a complete Markdown parser. Source bytes are never rewritten.

Authors can use LF or CRLF line endings and spaces after closing markers. A longer outer
fence or a different marker kind keeps an inner marker line literal. JSON escapes retain exact
control-byte evidence without depending on those bytes as Markdown syntax. The examples in
the preceding finding preserve their actual source bytes when decoded.

`scripts/build_book.sh` keeps its version check first, then runs the source check before
`mdbook build`. Source-check failures, including an unexpected checker exit, return build
failure without reaching the renderer. Its existing unavailable/version behavior is preserved:
the unavailable tool result remains `20`, ordinary failure `1` and success `0`. The
discovered build-script self-test now runs the checker and actual wrapper controls, so those
controls also run in the integration tier. The supported publication entrypoint remains this
wrapper; the external renderer's own CLI retains its upstream behavior.

#### Public execution and source preservation

Measured `2026-10-19`: the original wrapper reports `22 pass / 4 fail (26 arms)` against
the expanded controls. It reaches the renderer for each defective input and even when the
checker would fail. The repaired checker controls report `26 pass / 0 fail`, and the
build-script suite reports `5 pass / 0 fail`, preserving its earlier version controls.
A deliberately failing checker is normalized to build failure and cannot fall through.

An independent public replay executes the original and repaired wrappers with the actual
pinned mdBook0.5.2 executable. Instrumentation records each build invocation and passes the
complete command to that native executable; it supplies no HTML. The phases are ordinary,
bare-CR tilde, bare-CR backtick, closing-tab and corrected CRLF/space-only input, for each
wrapper. Original runs reproduce the actual misrenderings. Repaired defective runs refuse
before any renderer build invocation and retain the preceding complete output inventory.
Ordinary and corrected runs render `seven tests` as code, with `nine tests` after the
corrected closing fence as prose. Source SHA-256 remains exact through every phase.

The replay binds the build wrapper, checker, unchanged figure gate, tool pins and native
executable. Its receipt SHA-256 is
`f621347b91fd4890f13d8ee0fc5cb5830ac73b874086f049314defec7d9ac878`.
The first independent diagnostic expectation omitted the fixture's heading and blank line;
the correct reported source line includes them. That oracle calibration is retained separately
and supplies no defect-repair or passing-coverage claim.

The source/archive audit reverses only the new wrapper guard and self-test integration to
recover the complete previous wrapper byte for byte. It verifies Python/Bash syntax, unchanged
figure-gate bytes, the final public receipt, unchanged read-only book sources and the existing
release pin. False helper binding, renderer-call, source-rewrite, output-rewrite and recovered
literal assertions are independently refused.

Measured `2026-10-19`: all318 prior task archives remain exact; the completed figure leaf
forms the319th archive. Filesystem, Git-plus-new-path and indexed digest sets agree, and the
new body plus its separating LF reconstructs the original committed span exactly.
Truncated/modified seals and a false total are refused. Every preceding byte of this trust
annex remains an exact prefix. The combined audit SHA-256 is
`6c47da1b9e64f6c52e17a546337161c0d83a1d4e324de94aa2031754c5653b30`.

Supplementary reproduction commands and receipts are in `target/book-fence-compat/`:

```console
$ bash scripts/build_book.sh --self-test
$ python3 -I -B target/book-fence-compat/replay-public.py
$ python3 -I -B target/book-fence-compat/audit.py
```

The maintained checker/build self-tests are the durable executable controls. The public
fixtures, parser mechanisms, invocation boundary and expected outcomes are described here;
ignored scratch is supplementary. Publication/doctrine checks and the pinned full book build
must pass before this commit. This slice changes no Rust source, reason or mutation result.

#### Corrected upstream renderer remains owned

The reviewed stable release is
[mdBook0.5.4](https://github.com/rust-lang/mdBook/releases/tag/v0.5.4),
published `2026-07-06T15:28:05Z`. Its
[locked dependencies](https://github.com/rust-lang/mdBook/blob/v0.5.4/Cargo.lock)
select pulldown-cmark0.13.4. The tagged
[scanner source](https://github.com/pulldown-cmark/pulldown-cmark/blob/v0.13.4/pulldown-cmark/src/scanners.rs)
is byte-identical to the reviewed0.13.0 scanner:
`2cdcb3cc76bc3d6947eec066eefe4582b660dd52da71d048f6dc515665291840`.
Both identified mechanisms therefore remain in the stable source; upgrading that release
alone supplies neither fix. This is a source-level conclusion; actual replay executions
above use the project's pinned0.5.2 binary.

Leaf `M3.6.9.6.3.2.2.15.2.1.4.3.4.1` owns adoption of a stable corrected renderer at
the next release review, before this guard is retired. It requires qualified source/assets,
local and CI pins, actual control-byte renders, old anchors/literals, figure-gate alignment
and complete integration. The explicit publication refusal is in place while that upstream
prerequisite remains. Parent compatibility stays open; the deferred prefix proof resumes
after the committed guard. The book remains unbounded.
